use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use midir::{MidiOutput, MidiOutputConnection};

use crate::ir::{IrScore, IrEventKind};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Debug)]
pub enum PlaybackCommand {
    Play,
    Pause,
    Stop,
    Seek(f64), // Seek to beat position
    SetTempo(u32),
    Shutdown,
}

pub struct PlaybackEngine {
    command_tx: Sender<PlaybackCommand>,
    state: Arc<Mutex<PlaybackState>>,
    position_beats: Arc<Mutex<f64>>,
    available_ports: Vec<String>,
}

impl PlaybackEngine {
    pub fn new(ir: IrScore, tempo: u32) -> Result<Self, String> {
        let midi_out = MidiOutput::new("Melos Playback")
            .map_err(|e| format!("Failed to create MIDI output: {}", e))?;

        let available_ports: Vec<String> = midi_out
            .ports()
            .iter()
            .filter_map(|p| midi_out.port_name(p).ok())
            .collect();

        let state = Arc::new(Mutex::new(PlaybackState::Stopped));
        let position_beats = Arc::new(Mutex::new(0.0));

        let (command_tx, command_rx) = mpsc::channel();

        // Clone for the playback thread
        let state_clone = Arc::clone(&state);
        let position_clone = Arc::clone(&position_beats);

        // Try to connect to first available port
        let connection = if !midi_out.ports().is_empty() {
            let port = &midi_out.ports()[0];
            midi_out.connect(port, "melos-playback").ok()
        } else {
            None
        };

        // Spawn playback thread
        thread::spawn(move || {
            Self::playback_thread(
                connection,
                ir,
                tempo,
                command_rx,
                state_clone,
                position_clone,
            );
        });

        Ok(Self {
            command_tx,
            state,
            position_beats,
            available_ports,
        })
    }

    fn playback_thread(
        mut connection: Option<MidiOutputConnection>,
        ir: IrScore,
        mut tempo: u32,
        command_rx: Receiver<PlaybackCommand>,
        state: Arc<Mutex<PlaybackState>>,
        position_beats: Arc<Mutex<f64>>,
    ) {
        let ppq = ir.ppq as f64;
        let mut current_tick: f64 = 0.0;
        let mut last_update = Instant::now();
        let mut active_notes: Vec<(u8, u8)> = Vec::new(); // (channel, pitch)

        // Collect all note events with their timing
        let mut all_events: Vec<(u32, u8, IrEventKind)> = Vec::new();
        for track in &ir.tracks {
            for event in &track.events {
                all_events.push((event.time, track.channel, event.kind.clone()));
            }
        }
        all_events.sort_by_key(|(time, _, _)| *time);

        // Find max tick for looping bounds
        let max_tick: u32 = all_events
            .iter()
            .filter_map(|(time, _, kind)| {
                if let IrEventKind::Note { duration, .. } = kind {
                    Some(time + duration)
                } else {
                    Some(*time)
                }
            })
            .max()
            .unwrap_or(0);

        loop {
            // Check for commands (non-blocking)
            while let Ok(cmd) = command_rx.try_recv() {
                match cmd {
                    PlaybackCommand::Play => {
                        *state.lock().unwrap() = PlaybackState::Playing;
                        last_update = Instant::now();
                    }
                    PlaybackCommand::Pause => {
                        *state.lock().unwrap() = PlaybackState::Paused;
                        // Send note offs for all active notes
                        if let Some(ref mut conn) = connection {
                            for (channel, pitch) in &active_notes {
                                let _ = conn.send(&[0x80 | channel, *pitch, 0]);
                            }
                        }
                        active_notes.clear();
                    }
                    PlaybackCommand::Stop => {
                        *state.lock().unwrap() = PlaybackState::Stopped;
                        current_tick = 0.0;
                        *position_beats.lock().unwrap() = 0.0;
                        // Send note offs for all active notes
                        if let Some(ref mut conn) = connection {
                            for (channel, pitch) in &active_notes {
                                let _ = conn.send(&[0x80 | channel, *pitch, 0]);
                            }
                        }
                        active_notes.clear();
                    }
                    PlaybackCommand::Seek(beat) => {
                        current_tick = beat * ppq;
                        *position_beats.lock().unwrap() = beat;
                        // Send note offs for all active notes
                        if let Some(ref mut conn) = connection {
                            for (channel, pitch) in &active_notes {
                                let _ = conn.send(&[0x80 | channel, *pitch, 0]);
                            }
                        }
                        active_notes.clear();
                    }
                    PlaybackCommand::SetTempo(new_tempo) => {
                        tempo = new_tempo;
                    }
                    PlaybackCommand::Shutdown => {
                        // Send note offs for all active notes
                        if let Some(ref mut conn) = connection {
                            for (channel, pitch) in &active_notes {
                                let _ = conn.send(&[0x80 | channel, *pitch, 0]);
                            }
                        }
                        return;
                    }
                }
            }

            let current_state = *state.lock().unwrap();

            if current_state == PlaybackState::Playing {
                let now = Instant::now();
                let elapsed = now.duration_since(last_update);
                last_update = now;

                // Calculate ticks elapsed
                // tempo is in BPM, ppq is ticks per quarter note
                // ticks per second = (tempo / 60) * ppq
                let ticks_per_second = (tempo as f64 / 60.0) * ppq;
                let ticks_elapsed = elapsed.as_secs_f64() * ticks_per_second;

                let prev_tick = current_tick;
                current_tick += ticks_elapsed;

                // Update position
                *position_beats.lock().unwrap() = current_tick / ppq;

                // Process events in this time window
                if let Some(ref mut conn) = connection {
                    for (time, channel, kind) in &all_events {
                        let time_f = *time as f64;

                        // Note on events
                        if let IrEventKind::Note { pitch, velocity, duration } = kind {
                            // Check if note starts in this window
                            if time_f >= prev_tick && time_f < current_tick {
                                let _ = conn.send(&[0x90 | channel, *pitch, *velocity]);
                                active_notes.push((*channel, *pitch));
                            }

                            // Check if note ends in this window
                            let end_time = time_f + *duration as f64;
                            if end_time >= prev_tick && end_time < current_tick {
                                let _ = conn.send(&[0x80 | channel, *pitch, 0]);
                                active_notes.retain(|(c, p)| !(*c == *channel && *p == *pitch));
                            }
                        }

                        // Program change events
                        if let IrEventKind::ProgramChange(program) = kind {
                            if time_f >= prev_tick && time_f < current_tick {
                                let _ = conn.send(&[0xC0 | channel, *program]);
                            }
                        }
                    }
                }

                // Check for end of playback
                if current_tick >= max_tick as f64 {
                    *state.lock().unwrap() = PlaybackState::Stopped;
                    current_tick = 0.0;
                    *position_beats.lock().unwrap() = 0.0;
                    active_notes.clear();
                }
            }

            // Sleep a bit to avoid busy waiting
            thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn play(&self) {
        let _ = self.command_tx.send(PlaybackCommand::Play);
    }

    pub fn pause(&self) {
        let _ = self.command_tx.send(PlaybackCommand::Pause);
    }

    pub fn stop(&self) {
        let _ = self.command_tx.send(PlaybackCommand::Stop);
    }

    pub fn seek(&self, beat: f64) {
        let _ = self.command_tx.send(PlaybackCommand::Seek(beat));
    }

    pub fn set_tempo(&self, tempo: u32) {
        let _ = self.command_tx.send(PlaybackCommand::SetTempo(tempo));
    }

    pub fn state(&self) -> PlaybackState {
        *self.state.lock().unwrap()
    }

    pub fn position_beats(&self) -> f64 {
        *self.position_beats.lock().unwrap()
    }

    pub fn available_ports(&self) -> &[String] {
        &self.available_ports
    }
}

impl Drop for PlaybackEngine {
    fn drop(&mut self) {
        let _ = self.command_tx.send(PlaybackCommand::Shutdown);
    }
}
