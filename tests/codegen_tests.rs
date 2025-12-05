use melos::ir::*;
use melos::codegen::generate;
use midly::{TrackEventKind, MidiMessage};

#[test]
fn test_codegen_simple_score() {
    let ir = IrScore {
        ppq: 480,
        tracks: vec![IrTrack {
            name: "Piano".to_string(),
            channel: 0,
            events: vec![IrEvent {
                time: 0,
                kind: IrEventKind::Note {
                    pitch: 60,
                    velocity: 100,
                    duration: 480,
                },
            }],
        }],
    };

    let smf = generate(&ir).expect("Failed to generate MIDI");

    assert_eq!(smf.header.timing, midly::Timing::Metrical(midly::num::u15::new(480)));
    assert_eq!(smf.tracks.len(), 1);

    let track = &smf.tracks[0];
    // Expect NoteOn at 0, NoteOff at 480.
    
    let e1 = &track[0];
    assert_eq!(e1.delta.as_int(), 0);
    match e1.kind {
        TrackEventKind::Midi { channel, message } => {
            assert_eq!(channel.as_int(), 0);
            match message {
                MidiMessage::NoteOn { key, vel } => {
                    assert_eq!(key.as_int(), 60);
                    assert_eq!(vel.as_int(), 100);
                }
                _ => panic!("Expected NoteOn"),
            }
        }
        _ => panic!("Expected Midi event"),
    }

    let e2 = &track[1];
    assert_eq!(e2.delta.as_int(), 480);
    match e2.kind {
        TrackEventKind::Midi { channel, message } => {
            assert_eq!(channel.as_int(), 0);
            match message {
                MidiMessage::NoteOff { key, .. } => {
                    assert_eq!(key.as_int(), 60);
                }
                MidiMessage::NoteOn { key, vel } => {
                    assert_eq!(key.as_int(), 60);
                    assert_eq!(vel.as_int(), 0);
                }
                _ => panic!("Expected NoteOff"),
            }
        }
        _ => panic!("Expected Midi event"),
    }
}

#[test]
fn test_codegen_program_change() {
    let ir = IrScore {
        ppq: 480,
        tracks: vec![IrTrack {
            name: "Violin".to_string(),
            channel: 0,
            events: vec![
                IrEvent {
                    time: 0,
                    kind: IrEventKind::ProgramChange(40),
                },
                IrEvent {
                    time: 0,
                    kind: IrEventKind::Note {
                        pitch: 60,
                        velocity: 100,
                        duration: 480,
                    },
                },
            ],
        }],
    };

    let smf = generate(&ir).expect("Failed to generate MIDI");
    let track = &smf.tracks[0];

    // First event should be ProgramChange
    let e1 = &track[0];
    assert_eq!(e1.delta.as_int(), 0);
    match e1.kind {
        TrackEventKind::Midi { channel, message } => {
            assert_eq!(channel.as_int(), 0);
            match message {
                MidiMessage::ProgramChange { program } => {
                    assert_eq!(program.as_int(), 40);
                }
                _ => panic!("Expected ProgramChange"),
            }
        }
        _ => panic!("Expected Midi event"),
    }
}
