use melos::ast::*;
use melos::ir::*;
use melos::walker::walk;
use pretty_assertions::assert_eq;

#[test]
fn test_walk_simple_score() {
    let ast = Score {
        headers: vec![],
        parts: vec![Part {
            name: "Piano".to_string(),
            instrument: "Piano".to_string(),
            content: vec![MeasureBlock::Measure(Measure {
                events: vec![Event::Note(Note {
                    pitch: Pitch { step: 'C', accidental: None, octave: 4 },
                    duration: Some(Duration::Base(BaseDuration::Quarter, 0)),
                    dynamic: None,
                    articulation: None,
                })],
            })],
        }],
    };

    // Assume 480 PPQ.
    // Quarter note = 480 ticks.
    // C4 MIDI = 60.
    let expected = IrScore {
        ppq: 480,
        tracks: vec![
            IrTrack {
                name: "Conductor".to_string(),
                channel: 0,
                events: vec![
                    IrEvent {
                        time: 0,
                        kind: IrEventKind::TimeSignature(4, 4),
                    },
                ],
            },
            IrTrack {
            name: "Piano".to_string(),
            channel: 0,
            events: vec![
                IrEvent {
                    time: 0,
                    kind: IrEventKind::ProgramChange(0), // Piano
                },
                IrEvent {
                    time: 0,
                    kind: IrEventKind::TimeSignature(4, 4),
                },
                IrEvent {
                time: 0,
                kind: IrEventKind::Note {
                    pitch: 60,
                    velocity: 100, // Default
                    duration: 480,
                },
            }],
        }],
    };

    let result = walk(&ast).expect("Failed to walk");
    assert_eq!(result, expected);
}

#[test]
fn test_walk_tuplet_and_context() {
    let ast = Score {
        headers: vec![],
        parts: vec![Part {
            name: "Violin".to_string(),
            instrument: "Violin".to_string(),
            content: vec![
                MeasureBlock::ContextChange(ContextChange::TimeSignature(3, 4)),
                MeasureBlock::Measure(Measure {
                    events: vec![
                        Event::Tuplet(Tuplet {
                            p: 3,
                            q: 2,
                            events: vec![
                                Event::Note(Note {
                                    pitch: Pitch { step: 'C', accidental: None, octave: 5 }, // 72
                                    duration: Some(Duration::Base(BaseDuration::Quarter, 0)), // 480
                                    dynamic: None,
                                    articulation: None,
                                }),
                                Event::Note(Note {
                                    pitch: Pitch { step: 'D', accidental: None, octave: 5 }, // 74
                                    duration: Some(Duration::Base(BaseDuration::Quarter, 0)), // 480
                                    dynamic: None,
                                    articulation: None,
                                }),
                                Event::Note(Note {
                                    pitch: Pitch { step: 'E', accidental: None, octave: 5 }, // 76
                                    duration: Some(Duration::Base(BaseDuration::Quarter, 0)), // 480
                                    dynamic: None,
                                    articulation: None,
                                }),
                            ],
                        }),
                    ],
                }),
            ],
        }],
    };

    // PPQ = 480
    // Tuplet 3:2 means 3 notes in time of 2.
    // 2 quarter notes = 2 * 480 = 960 ticks.
    // Each note in tuplet takes 960 / 3 = 320 ticks.
    
    let expected = IrScore {
        ppq: 480,
        tracks: vec![
            IrTrack {
                name: "Conductor".to_string(),
                channel: 0,
                events: vec![
                    IrEvent {
                        time: 0,
                        kind: IrEventKind::TimeSignature(4, 4),
                    },
                ],
            },
            IrTrack {
            name: "Violin".to_string(),
            channel: 0,
            events: vec![
                IrEvent {
                    time: 0,
                    kind: IrEventKind::ProgramChange(40), // Violin
                },
                IrEvent {
                    time: 0,
                    kind: IrEventKind::TimeSignature(3, 4),
                },
                IrEvent {
                    time: 0,
                    kind: IrEventKind::Note {
                        pitch: 72,
                        velocity: 100,
                        duration: 320,
                    },
                },
                IrEvent {
                    time: 320,
                    kind: IrEventKind::Note {
                        pitch: 74,
                        velocity: 100,
                        duration: 320,
                    },
                },
                IrEvent {
                    time: 640,
                    kind: IrEventKind::Note {
                        pitch: 76,
                        velocity: 100,
                        duration: 320,
                    },
                },
            ],
        }],
    };

    let result = walk(&ast).expect("Failed to walk");
    assert_eq!(result, expected);
}
