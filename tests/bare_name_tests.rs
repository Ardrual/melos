use melos::ast::*;
use melos::parser::parse;
use pretty_assertions::assert_eq;

#[test]
fn test_parse_bare_part_name() {
    let input = r#"
    Title: "Bare Name Test"
    Part: Acoustic Guitar Instrument: "Acoustic Guitar (Steel)" {
        | C4 q |
    }
    "#;

    let expected = Score {
        headers: vec![Header::Title("Bare Name Test".to_string())],
        parts: vec![Part {
            name: "Acoustic Guitar".to_string(),
            instrument: "Acoustic Guitar (Steel)".to_string(),
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

    let result = parse(input).expect("Failed to parse");
    assert_eq!(result, expected);
}

#[test]
fn test_parse_bare_part_name_with_special_chars() {
    let input = r#"
    Title: "Special Chars Test"
    Part: First_Violin (Solo) Instrument: Violin {
        | C4 q |
    }
    "#;

    let expected = Score {
        headers: vec![Header::Title("Special Chars Test".to_string())],
        parts: vec![Part {
            name: "First_Violin (Solo)".to_string(),
            instrument: "Violin".to_string(),
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

    let result = parse(input).expect("Failed to parse");
    assert_eq!(result, expected);
}
