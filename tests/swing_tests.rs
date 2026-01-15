use melos::ast::*;
use melos::parser::parse;
use melos::walker::walk;
use melos::ir::*;
use pretty_assertions::assert_eq;

#[test]
fn test_parse_swing_header() {
    let input = r#"
    Title: "Swing Test"
    Swing: e 0.66
    Part: Piano Instrument: Piano {
        | C4 e D4 e |
    }
    "#;

    let result = parse(input).expect("Failed to parse");
    assert!(result.headers.iter().any(|h| matches!(h, Header::Swing(Some((BaseDuration::Eighth, _))))));
}

#[test]
fn test_parse_swing_context_change() {
    let input = r#"
    Part: Piano Instrument: Piano {
        | C4 q |
        Swing: e 0.66
        | C4 e D4 e |
        Swing: off
        | C4 q |
    }
    "#;

    let result = parse(input).expect("Failed to parse");
    let part = &result.parts[0];
    assert!(matches!(part.content[1], MeasureBlock::ContextChange(ContextChange::Swing(Some((BaseDuration::Eighth, 0.66))))));
    assert!(matches!(part.content[3], MeasureBlock::ContextChange(ContextChange::Swing(None))));
}

#[test]
fn test_walk_swing_timing() {
    let input = r#"
    Title: "Swing Timing Test"
    Part: Piano Instrument: Piano {
        Swing: e 0.66
        | C4 e D4 e |
    }
    "#;

    let score = parse(input).expect("Failed to parse");
    let ir = walk(&score).expect("Failed to walk");
    
    // Track 0 is Conductor, Track 1 is Piano
    let track = &ir.tracks[1];
    let events: Vec<_> = track.events.iter().filter(|e| matches!(e.kind, IrEventKind::Note { .. })).collect();
    
    assert_eq!(events.len(), 2);
    
    if let IrEventKind::Note { duration: d1, .. } = events[0].kind {
        if let IrEventKind::Note { duration: d2, .. } = events[1].kind {
            // Total duration should be 480 (one quarter beat)
            // PPQ is 480. Eighth is 240.
            // Swung: 240 * 0.66 * 2 = 316.8 (approx 316-317)
            // Unswung: 240 * (1-0.66) * 2 = 163.2 (approx 163-164)
            // 316 + 164 = 480.
            
            assert!(d1 > 240);
            assert!(d2 < 240);
            assert_eq!(d1 + d2, 480);
        }
    }
}
