use melos::ast::*;
use melos::ir::*;
use melos::walker::walk;
use pretty_assertions::assert_eq;

#[test]
fn test_global_header_time_signature() {
    let ast = Score {
        headers: vec![Header::TimeSignature(3, 4)],
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

    let result = walk(&ast).expect("Failed to walk");
    
    // Check for Conductor track with TimeSignature
    let conductor = &result.tracks[0];
    assert_eq!(conductor.name, "Conductor");
    assert_eq!(conductor.events.len(), 1);
    assert_eq!(conductor.events[0].kind, IrEventKind::TimeSignature(3, 4));

    // Check that Piano track ALSO has the global TimeSignature
    let piano = &result.tracks[1];
    assert_eq!(piano.name, "Piano");
    // ProgramChange(0) + TimeSignature(3,4) + Note
    assert_eq!(piano.events.len(), 3); 
    assert_eq!(piano.events[1].kind, IrEventKind::TimeSignature(3, 4));
}

#[test]
fn test_inline_time_signature_single_part() {
    let ast = Score {
        headers: vec![],
        parts: vec![Part {
            name: "Piano".to_string(),
            instrument: "Piano".to_string(),
            content: vec![
                MeasureBlock::ContextChange(ContextChange::TimeSignature(5, 8)),
                MeasureBlock::Measure(Measure {
                    events: vec![Event::Rest(None)],
                }),
            ],
        }],
    };

    let result = walk(&ast).expect("Failed to walk");
    
    let conductor = &result.tracks[0];
    assert_eq!(conductor.name, "Conductor");
    assert_eq!(conductor.events.len(), 1);
    assert_eq!(conductor.events[0].kind, IrEventKind::TimeSignature(4, 4));

    let piano = &result.tracks[1];
    // ProgramChange + TS(5,8)
    assert_eq!(piano.events.len(), 2);
    assert_eq!(piano.events[1].kind, IrEventKind::TimeSignature(5, 8));
}

#[test]
fn test_inline_time_signature_multiple_parts_merge() {
    let ast = Score {
        headers: vec![],
        parts: vec![
            Part {
                name: "Piano".to_string(),
                instrument: "Piano".to_string(),
                content: vec![
                    MeasureBlock::ContextChange(ContextChange::TimeSignature(4, 4)),
                    MeasureBlock::Measure(Measure { events: vec![Event::Rest(None)] }),
                    MeasureBlock::ContextChange(ContextChange::TimeSignature(3, 4)),
                ],
            },
            Part {
                name: "Violin".to_string(),
                instrument: "Violin".to_string(),
                content: vec![
                    MeasureBlock::ContextChange(ContextChange::TimeSignature(4, 4)),
                    MeasureBlock::Measure(Measure { events: vec![Event::Rest(None)] }),
                    MeasureBlock::ContextChange(ContextChange::TimeSignature(3, 4)),
                ],
            },
        ],
    };

    let result = walk(&ast).expect("Failed to walk");
    
    let conductor = &result.tracks[0];
    assert_eq!(conductor.name, "Conductor");
    // Should be deduped
    assert_eq!(conductor.events.len(), 1);
    assert_eq!(conductor.events[0].kind, IrEventKind::TimeSignature(4, 4));

    // Check Piano
    let piano = &result.tracks[1];
    assert_eq!(piano.events.len(), 3); // PC + TS(4/4) + TS(3/4)
    assert_eq!(piano.events[1].kind, IrEventKind::TimeSignature(4, 4));
    assert_eq!(piano.events[2].kind, IrEventKind::TimeSignature(3, 4));
}

#[test]
fn test_complex_time_signatures() {
    let ast = Score {
        headers: vec![],
        parts: vec![Part {
            name: "Piano".to_string(),
            instrument: "Piano".to_string(),
            content: vec![
                MeasureBlock::ContextChange(ContextChange::TimeSignature(11, 8)),
                MeasureBlock::Measure(Measure { events: vec![Event::Rest(None)] }),
                MeasureBlock::ContextChange(ContextChange::TimeSignature(7, 16)),
            ],
        }],
    };

    let result = walk(&ast).expect("Failed to walk");
    
    let conductor = &result.tracks[0];
    assert_eq!(conductor.events.len(), 1);
    assert_eq!(conductor.events[0].kind, IrEventKind::TimeSignature(4, 4));

    let piano = &result.tracks[1];
    assert_eq!(piano.events[1].kind, IrEventKind::TimeSignature(11, 8));
    assert_eq!(piano.events[2].kind, IrEventKind::TimeSignature(7, 16));
}
