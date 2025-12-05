use melos::parser::parse;
use melos::ast::*;

#[test]
fn test_parse_chord() {
    let input = r#"
    Title: "Chord Test"
    Part: Piano Instrument: Piano {
        | [C4 E4 G4]q |
    }
    "#;
    let score = parse(input).expect("Failed to parse");
    let part = &score.parts[0];
    if let MeasureBlock::Measure(m) = &part.content[0] {
        if let Event::Chord(pitches, duration, _, _) = &m.events[0] {
            assert_eq!(pitches.len(), 3);
            assert_eq!(pitches[0].step, 'C');
            assert_eq!(pitches[1].step, 'E');
            assert_eq!(pitches[2].step, 'G');
            
            if let Some(Duration::Base(BaseDuration::Quarter, 0)) = duration {
                // OK
            } else {
                panic!("Expected Quarter duration");
            }
        } else {
            panic!("Expected Chord");
        }
    } else {
        panic!("Expected Measure");
    }
}
