use crate::ir::{IrScore, IrEventKind};
use anyhow::Result;
use midly::{Smf, Header, Format, Timing, TrackEvent, TrackEventKind, MidiMessage, MetaMessage};
use midly::num::{u4, u7, u15, u24, u28};

pub fn generate(score: &IrScore) -> Result<Smf<'static>> {
    let header = Header {
        format: Format::Parallel, // Type 1
        timing: Timing::Metrical(u15::new(score.ppq as u16)),
    };

    let mut tracks = Vec::new();

    for ir_track in &score.tracks {
        let mut events = Vec::new();

        // 1. Expand IR events into absolute MIDI events
        for event in &ir_track.events {
            match &event.kind {
                IrEventKind::Note { pitch, velocity, duration } => {
                    // Note On
                    events.push(AbsEvent {
                        time: event.time,
                        kind: TrackEventKind::Midi {
                            channel: u4::new(ir_track.channel),
                            message: MidiMessage::NoteOn {
                                key: u7::new(*pitch),
                                vel: u7::new(*velocity),
                            },
                        },
                    });
                    // Note Off
                    events.push(AbsEvent {
                        time: event.time + duration,
                        kind: TrackEventKind::Midi {
                            channel: u4::new(ir_track.channel),
                            message: MidiMessage::NoteOff {
                                key: u7::new(*pitch),
                                vel: u7::new(0),
                            },
                        },
                    });
                }
                IrEventKind::Tempo(bpm) => {
                     let mpq = 60_000_000 / bpm;
                     events.push(AbsEvent {
                         time: event.time,
                         kind: TrackEventKind::Meta(MetaMessage::Tempo(u24::new(mpq))),
                     });
                }
                IrEventKind::ProgramChange(program) => {
                    events.push(AbsEvent {
                        time: event.time,
                        kind: TrackEventKind::Midi {
                            channel: u4::new(ir_track.channel),
                            message: MidiMessage::ProgramChange {
                                program: u7::new(*program),
                            },
                        },
                    });
                }
                IrEventKind::TimeSignature(num, den) => {
                    // MIDI Time Signature:
                    // nn: numerator
                    // dd: denominator (negative power of 2, e.g. 2 means 2^2=4)
                    // cc: clocks per metronome click (usually 24)
                    // bb: 32nd notes per quarter note (usually 8)
                    
                    // den is the actual denominator (e.g. 4), we need log2(den)
                    let dd = (*den as f64).log2() as u8;
                    
                    events.push(AbsEvent {
                        time: event.time,
                        kind: TrackEventKind::Meta(MetaMessage::TimeSignature(
                            *num as u8,
                            dd,
                            24,
                            8,
                        )),
                    });
                }
                _ => {}
            }
        }

        // 2. Sort events by time
        events.sort_by_key(|e| e.time);

        // 3. Convert to Delta
        let mut track = Vec::new();
        let mut last_time = 0;

        for event in events {
            let delta = event.time - last_time;
            last_time = event.time;
            track.push(TrackEvent {
                delta: u28::new(delta),
                kind: event.kind,
            });
        }

        // Add EndOfTrack
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });

        tracks.push(track);
    }

    Ok(Smf { header, tracks })
}

struct AbsEvent<'a> {
    time: u32,
    kind: TrackEventKind<'a>,
}
