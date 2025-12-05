use melos::ast::*;
use melos::parser::parse;
use pretty_assertions::assert_eq;

#[test]
fn test_parse_simple_score() {
    let input = r#"
    Title: "Test Score"
    Part: Piano Instrument: Piano {
        | C4 q |
    }
    "#;

    let expected = Score {
        headers: vec![Header::Title("Test Score".to_string())],
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

    let result = parse(input).expect("Failed to parse");
    assert_eq!(result, expected);
}

#[test]
fn test_parse_advanced_features() {
    let input = r#"
    Title: "Advanced"
    Tempo: 120
    Time: 4/4
    Part: Flute Instrument: Flute {
        | C5 q r q |
        Time: 3/4
        Key: G "Major"
        | Tuplet(3:2) { D5 q E5 q F#5 q } |
    }
    "#;

    let expected = Score {
        headers: vec![
            Header::Title("Advanced".to_string()),
            Header::Tempo(120),
            Header::TimeSignature(4, 4),
        ],
        parts: vec![Part {
            name: "Flute".to_string(),
            instrument: "Flute".to_string(),
            content: vec![
                MeasureBlock::Measure(Measure {
                    events: vec![
                        Event::Note(Note {
                            pitch: Pitch { step: 'C', accidental: None, octave: 5 },
                            duration: Some(Duration::Base(BaseDuration::Quarter, 0)),
                            dynamic: None,
                            articulation: None,
                        }),
                        Event::Rest(Some(Duration::Base(BaseDuration::Quarter, 0))),
                    ],
                }),
                MeasureBlock::ContextChange(ContextChange::TimeSignature(3, 4)),
                MeasureBlock::ContextChange(ContextChange::KeySignature("G".to_string(), "Major".to_string())),
                MeasureBlock::Measure(Measure {
                    events: vec![
                        Event::Tuplet(Tuplet {
                            p: 3,
                            q: 2,
                            events: vec![
                                Event::Note(Note {
                                    pitch: Pitch { step: 'D', accidental: None, octave: 5 },
                                    duration: Some(Duration::Base(BaseDuration::Quarter, 0)),
                                    dynamic: None,
                                    articulation: None,
                                }),
                                Event::Note(Note {
                                    pitch: Pitch { step: 'E', accidental: None, octave: 5 },
                                    duration: Some(Duration::Base(BaseDuration::Quarter, 0)),
                                    dynamic: None,
                                    articulation: None,
                                }),
                                Event::Note(Note {
                                    pitch: Pitch { step: 'F', accidental: Some(Accidental::Sharp), octave: 5 },
                                    duration: Some(Duration::Base(BaseDuration::Quarter, 0)),
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

    let result = parse(input).expect("Failed to parse");
    assert_eq!(result, expected);
}
