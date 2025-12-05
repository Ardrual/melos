use crate::ast::*;
use crate::ir::*;
use crate::instruments::get_instrument_program;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

const PPQ: u32 = 480;

pub fn walk(score: &Score) -> Result<IrScore> {
    let mut tracks: Vec<IrTrack> = Vec::new();
    let mut track_map: HashMap<String, (usize, u32)> = HashMap::new(); // Name -> (index, end_time)
    
    // Create a conductor track for global events like Tempo and Time Signature
    let mut conductor_events = Vec::new();

    // Default Time Signature if not present
    let mut has_time_signature = false;

    for header in &score.headers {
        match header {
            Header::Tempo(bpm) => {
                conductor_events.push(IrEvent {
                    time: 0,
                    kind: IrEventKind::Tempo(*bpm),
                });
            }
            Header::TimeSignature(num, den) => {
                has_time_signature = true;
                conductor_events.push(IrEvent {
                    time: 0,
                    kind: IrEventKind::TimeSignature(*num, *den),
                });
            }
            _ => {}
        }
    }

    if !has_time_signature {
        conductor_events.push(IrEvent {
            time: 0,
            kind: IrEventKind::TimeSignature(4, 4),
        });
    }

    let mut global_time_signature = (4, 4);
    for header in &score.headers {
        if let Header::TimeSignature(num, den) = header {
            global_time_signature = (*num, *den);
        }
    }

    let mut next_channel = 0;

    for part in &score.parts {
        if let Some(&(index, current_end_time)) = track_map.get(&part.name) {
            // Merge with existing track
            let channel = tracks[index].channel;
            let (mut new_track, duration) = walk_part(part, channel, global_time_signature)?;

            // Shift events
            for event in &mut new_track.events {
                event.time += current_end_time;
            }

            tracks[index].events.extend(new_track.events);
            
            // Update map
            track_map.insert(part.name.clone(), (index, current_end_time + duration));
        } else {
            // New track
            let channel = next_channel;
            next_channel = (next_channel + 1) % 16; // Wrap around 0-15

            let (new_track, duration) = walk_part(part, channel, global_time_signature)?;
            tracks.push(new_track);
            track_map.insert(part.name.clone(), (tracks.len() - 1, duration));
        }
    }

    // Sort conductor events
    conductor_events.sort_by(|a, b| a.time.cmp(&b.time));
    conductor_events.dedup();

    tracks.insert(0, IrTrack {
        name: "Conductor".to_string(),
        channel: 0, // Channel doesn't matter for Meta events, but 0 is fine
        events: conductor_events,
    });

    Ok(IrScore { tracks, ppq: PPQ })
}

fn walk_part(part: &Part, channel: u8, initial_time_signature: (u32, u32)) -> Result<(IrTrack, u32)> {
    let mut events = Vec::new();
    let mut current_time = 0;
    let mut current_velocity = 100; // Default velocity (mf)
    let mut current_time_signature = initial_time_signature;
    let mut measure_index = 0;

    // Add Program Change event if instrument is found
    if let Some(program) = get_instrument_program(&part.instrument) {
        events.push(IrEvent {
            time: 0,
            kind: IrEventKind::ProgramChange(program),
        });
    } else {
        // Default to Piano (0) if not found
        events.push(IrEvent {
            time: 0,
            kind: IrEventKind::ProgramChange(0),
        });
    }

    // Emit initial Time Signature
    events.push(IrEvent {
        time: 0,
        kind: IrEventKind::TimeSignature(initial_time_signature.0, initial_time_signature.1),
    });

    for block in &part.content {
        match block {
            MeasureBlock::Measure(measure) => {
                measure_index += 1;
                
                // Verify measure duration
                let expected_ticks = (current_time_signature.0 as u64 * PPQ as u64 * 4 / current_time_signature.1 as u64) as u32;
                if let Ok(actual_ticks) = calculate_measure_duration(measure, PPQ) {
                    if actual_ticks != expected_ticks {
                        eprintln!("Warning: Measure {} in part '{}' has incorrect duration. Expected {} ticks, got {}.", 
                            measure_index, part.name, expected_ticks, actual_ticks);
                    }
                }

                for event in &measure.events {
                    process_event(event, &mut current_time, &mut events, 1.0, &mut current_velocity)?;
                }
            }
            MeasureBlock::ContextChange(cc) => {
                match cc {
                    ContextChange::TimeSignature(num, den) => {
                        current_time_signature = (*num, *den);
                        events.push(IrEvent {
                            time: current_time,
                            kind: IrEventKind::TimeSignature(*num, *den),
                        });
                    }
                    ContextChange::KeySignature(root, scale) => {
                        events.push(IrEvent {
                            time: current_time,
                            kind: IrEventKind::KeySignature {
                                root: root.clone(),
                                scale: scale.clone(),
                            },
                        });
                    }
                    ContextChange::Tempo(bpm) => {
                        events.push(IrEvent {
                            time: current_time,
                            kind: IrEventKind::Tempo(*bpm),
                        });
                    }
                }
            }
        }
    }


    
    // Dedup to remove redundant TimeSignatures (e.g. global default + explicit context change to same value)
    events.dedup();

    // Special cleanup for t=0 TimeSignatures: keep only the last one
    let mut final_events = Vec::new();
    let mut t0_ts_indices = Vec::new();
    
    for (i, event) in events.iter().enumerate() {
        if event.time == 0 {
            if let IrEventKind::TimeSignature(_, _) = event.kind {
                t0_ts_indices.push(i);
            }
        }
    }

    if t0_ts_indices.len() > 1 {
        // Keep only the last one
        let last_index = *t0_ts_indices.last().unwrap();
        for (i, event) in events.into_iter().enumerate() {
            if event.time == 0 && matches!(event.kind, IrEventKind::TimeSignature(_, _)) {
                if i == last_index {
                    final_events.push(event);
                }
            } else {
                final_events.push(event);
            }
        }
        events = final_events;
    }

    Ok((IrTrack {
        name: part.name.clone(),
        channel,
        events,
    }, current_time))
}

fn process_event(
    event: &Event,
    current_time: &mut u32,
    events: &mut Vec<IrEvent>,
    time_scale: f64,
    current_velocity: &mut u8,
) -> Result<()> {
    match event {
        Event::Note(note) => {
            let duration = calculate_duration(&note.duration, PPQ)?;
            let scaled_duration = (duration as f64 * time_scale) as u32;
            let pitch = calculate_pitch(&note.pitch)?;
            
            // Update velocity if dynamic is present
            if let Some(dyn_str) = &note.dynamic {
                *current_velocity = dynamic_to_velocity(dyn_str);
            }

            events.push(IrEvent {
                time: *current_time,
                kind: IrEventKind::Note {
                    pitch,
                    velocity: *current_velocity,
                    duration: scaled_duration,
                },
            });
            *current_time += scaled_duration;
        }
        Event::Chord(pitches, duration_opt, dynamic_opt, _articulation) => {
            let duration = calculate_duration(duration_opt, PPQ)?;
            let scaled_duration = (duration as f64 * time_scale) as u32;
            
            if let Some(dyn_str) = dynamic_opt {
                *current_velocity = dynamic_to_velocity(dyn_str);
            }

            for pitch in pitches {
                let midi_pitch = calculate_pitch(pitch)?;
                events.push(IrEvent {
                    time: *current_time,
                    kind: IrEventKind::Note {
                        pitch: midi_pitch,
                        velocity: *current_velocity, 
                        duration: scaled_duration,
                    },
                });
            }
            *current_time += scaled_duration;
        }
        Event::Rest(duration_opt) => {
            let duration = calculate_duration(duration_opt, PPQ)?;
            let scaled_duration = (duration as f64 * time_scale) as u32;
            *current_time += scaled_duration;
        }
        Event::Tuplet(tuplet) => {
            let new_scale = time_scale * (tuplet.q as f64 / tuplet.p as f64);
            for sub_event in &tuplet.events {
                process_event(sub_event, current_time, events, new_scale, current_velocity)?;
            }
        }
        Event::Dynamic(dyn_str) => {
            *current_velocity = dynamic_to_velocity(dyn_str);
        }
        _ => {} // Handle other events if necessary
    }
    Ok(())
}

fn dynamic_to_velocity(dynamic: &str) -> u8 {
    match dynamic {
        "fff" => 127,
        "ff" => 112,
        "f" => 96,
        "mf" => 80,
        "mp" => 64,
        "p" => 48,
        "pp" => 32,
        "ppp" => 16,
        _ => 80, // Default to mf
    }
}

fn calculate_duration(duration_opt: &Option<Duration>, ppq: u32) -> Result<u32> {
    match duration_opt {
        Some(d) => match d {
            Duration::Base(base, dots) => {
                let mut ticks = match base {
                    BaseDuration::Whole => ppq * 4,
                    BaseDuration::Half => ppq * 2,
                    BaseDuration::Quarter => ppq,
                    BaseDuration::Eighth => ppq / 2,
                    BaseDuration::Sixteenth => ppq / 4,
                };
                let mut add = ticks / 2;
                for _ in 0..*dots {
                    ticks += add;
                    add /= 2;
                }
                Ok(ticks)
            }
            _ => Err(anyhow!("Unsupported duration type")),
        },
        None => Ok(ppq), // Default to quarter
    }
}

fn calculate_pitch(pitch: &Pitch) -> Result<u8> {
    let base = match pitch.step {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return Err(anyhow!("Invalid pitch step")),
    };
    
    let accidental = match &pitch.accidental {
        Some(Accidental::Sharp) => 1,
        Some(Accidental::Flat) => -1,
        None => 0,
    };

    let midi = (pitch.octave + 1) * 12 + base + accidental;
    if midi < 0 || midi > 127 {
        return Err(anyhow!("Pitch out of MIDI range"));
    }
    Ok(midi as u8)
}

fn calculate_measure_duration(measure: &Measure, ppq: u32) -> Result<u32> {
    let mut total_ticks = 0;
    for event in &measure.events {
        total_ticks += calculate_event_duration(event, ppq)?;
    }
    Ok(total_ticks)
}

fn calculate_event_duration(event: &Event, ppq: u32) -> Result<u32> {
    match event {
        Event::Note(note) => calculate_duration(&note.duration, ppq),
        Event::Chord(_, duration_opt, _, _) => calculate_duration(duration_opt, ppq),
        Event::Rest(duration_opt) => calculate_duration(duration_opt, ppq),
        Event::Tuplet(tuplet) => {
            let mut content_ticks = 0;
            for sub_event in &tuplet.events {
                content_ticks += calculate_event_duration(sub_event, ppq)?;
            }
            // Scale duration: content_ticks * (q / p)
            // Use u64 to prevent overflow before division
            Ok((content_ticks as u64 * tuplet.q as u64 / tuplet.p as u64) as u32)
        }
        Event::Dynamic(_) => Ok(0),
        _ => Ok(0),
    }
}


