use melos::parser::parse;
use melos::walker::walk;
use melos::ir::*;

#[test]
fn test_walk_chord() {
    let input = r#"
    Title: "Chord Test"
    Part: Piano Instrument: Piano {
        | [C4 E4 G4]q |
    }
    "#;
    let score = parse(input).expect("Failed to parse");
    let ir = walk(&score).expect("Failed to walk");
    let track = ir.tracks.iter().find(|t| t.name == "Piano").expect("Piano track not found");
    
    // Should have 5 events (1 ProgramChange + 1 TimeSignature + 3 Notes)
    assert_eq!(track.events.len(), 5);
    
    // First event should be ProgramChange
    if let IrEventKind::ProgramChange(prog) = track.events[0].kind {
        assert_eq!(prog, 0); // Piano
    } else {
        panic!("Expected ProgramChange event");
    }

    // Second event should be TimeSignature
    if let IrEventKind::TimeSignature(num, den) = track.events[1].kind {
        assert_eq!(num, 4);
        assert_eq!(den, 4);
    } else {
        panic!("Expected TimeSignature event");
    }

    for event in &track.events[2..] {
        assert_eq!(event.time, 0);
        if let IrEventKind::Note { duration, .. } = event.kind {
            assert_eq!(duration, 480); // Quarter note at 480 PPQ
        } else {
            panic!("Expected Note event");
        }
    }
}
