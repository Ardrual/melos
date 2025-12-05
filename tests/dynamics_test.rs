use melos::parser::parse;
use melos::walker::walk;
use melos::codegen::generate;
use midly::{TrackEventKind, MidiMessage};

#[test]
fn test_dynamics_propagation() {
    let source = r#"
    Title: "Dynamics Test"
    Tempo: 120
    Time: 4/4

    Part: Piano Instrument: Piano {
        | C4 q fff D4 q pp E4 q mf |
    }
    "#;

    let ast = parse(source).expect("Failed to parse");
    let ir = walk(&ast).expect("Failed to walk");
    let smf = generate(&ir).expect("Failed to generate MIDI");

    // Track 0 is Conductor (Tempo/TimeSig), Track 1 is Piano
    let track = &smf.tracks[1];
    let mut note_on_events = Vec::new();

    for event in track {
        if let TrackEventKind::Midi { message, .. } = event.kind {
            if let MidiMessage::NoteOn { vel, .. } = message {
                if vel.as_int() > 0 {
                    note_on_events.push(vel.as_int());
                }
            }
        }
    }

    // Expected velocities: fff=127, pp=32, mf=80
    assert_eq!(note_on_events, vec![127, 32, 80]);
}
