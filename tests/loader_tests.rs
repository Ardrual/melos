use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use melos::loader::{load_source, LoadedSource};

fn create_test_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp dir")
}

#[test]
fn test_load_single_file() {
    let dir = create_test_dir();
    let file_path = dir.path().join("test.mel");

    let content = r#"Title: "Test"
Part: Piano Instrument: Piano {
    | C4 q |
}"#;
    fs::write(&file_path, content).unwrap();

    let result = load_source(&file_path).expect("Failed to load source");

    assert_eq!(result.source, content);
    assert_eq!(result.base_path, file_path);
}

#[test]
fn test_load_directory_single_file() {
    let dir = create_test_dir();
    let score_dir = dir.path().join("my_score");
    fs::create_dir(&score_dir).unwrap();

    let content = r#"Title: "Test"
Part: Piano Instrument: Piano {
    | C4 q |
}"#;
    fs::write(score_dir.join("score.mel"), content).unwrap();

    let result = load_source(&score_dir).expect("Failed to load source");

    assert_eq!(result.source, content);
    assert_eq!(result.base_path, score_dir);
}

#[test]
fn test_load_directory_multiple_files_score_first() {
    let dir = create_test_dir();
    let score_dir = dir.path().join("my_score");
    fs::create_dir(&score_dir).unwrap();

    // score.mel should come first
    let score_content = r#"Title: "Multi-File Test"
Tempo: 120
Time: 4/4
"#;
    fs::write(score_dir.join("score.mel"), score_content).unwrap();

    // piano.mel
    let piano_content = r#"Part: Piano Instrument: Piano {
    | C4 q D4 q E4 q F4 q |
}
"#;
    fs::write(score_dir.join("piano.mel"), piano_content).unwrap();

    // violin.mel
    let violin_content = r#"Part: Violin Instrument: Violin {
    | G4 q A4 q B4 q C5 q |
}
"#;
    fs::write(score_dir.join("violin.mel"), violin_content).unwrap();

    let result = load_source(&score_dir).expect("Failed to load source");

    // score.mel should be first, then others alphabetically
    assert!(result.source.starts_with("Title: \"Multi-File Test\""));
    assert!(result.source.contains("Part: Piano"));
    assert!(result.source.contains("Part: Violin"));

    // Verify order: score.mel content appears before piano.mel content
    let score_pos = result.source.find("Title:").unwrap();
    let piano_pos = result.source.find("Part: Piano").unwrap();
    let violin_pos = result.source.find("Part: Violin").unwrap();

    assert!(score_pos < piano_pos);
    assert!(piano_pos < violin_pos); // alphabetical: piano before violin
}

#[test]
fn test_load_directory_alphabetical_order_without_score() {
    let dir = create_test_dir();
    let score_dir = dir.path().join("my_score");
    fs::create_dir(&score_dir).unwrap();

    // No score.mel, files should be in alphabetical order
    let cello_content = r#"Title: "Test"
Part: Cello Instrument: Cello {
    | C3 w |
}
"#;
    fs::write(score_dir.join("cello.mel"), cello_content).unwrap();

    let violin_content = r#"Part: Violin Instrument: Violin {
    | G4 w |
}
"#;
    fs::write(score_dir.join("violin.mel"), violin_content).unwrap();

    let result = load_source(&score_dir).expect("Failed to load source");

    // cello.mel comes before violin.mel alphabetically
    let cello_pos = result.source.find("Part: Cello").unwrap();
    let violin_pos = result.source.find("Part: Violin").unwrap();

    assert!(cello_pos < violin_pos);
}

#[test]
fn test_load_directory_ignores_non_mel_files() {
    let dir = create_test_dir();
    let score_dir = dir.path().join("my_score");
    fs::create_dir(&score_dir).unwrap();

    let score_content = r#"Title: "Test"
Part: Piano Instrument: Piano {
    | C4 q |
}
"#;
    fs::write(score_dir.join("score.mel"), score_content).unwrap();

    // These should be ignored
    fs::write(score_dir.join("README.md"), "# My Score").unwrap();
    fs::write(score_dir.join("notes.txt"), "Some notes").unwrap();
    fs::write(score_dir.join(".DS_Store"), "garbage").unwrap();

    let result = load_source(&score_dir).expect("Failed to load source");

    assert!(!result.source.contains("README"));
    assert!(!result.source.contains("Some notes"));
    assert!(!result.source.contains("garbage"));
}

#[test]
fn test_load_directory_empty_errors() {
    let dir = create_test_dir();
    let score_dir = dir.path().join("empty_score");
    fs::create_dir(&score_dir).unwrap();

    let result = load_source(&score_dir);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("No .mel files"));
}

#[test]
fn test_load_nonexistent_path_errors() {
    let path = PathBuf::from("/nonexistent/path/to/file.mel");

    let result = load_source(&path);

    assert!(result.is_err());
}

#[test]
fn test_load_directory_with_newline_separator() {
    let dir = create_test_dir();
    let score_dir = dir.path().join("my_score");
    fs::create_dir(&score_dir).unwrap();

    // Files without trailing newlines
    fs::write(score_dir.join("score.mel"), "Title: \"Test\"").unwrap();
    fs::write(score_dir.join("piano.mel"), "Part: Piano Instrument: Piano { | C4 q | }").unwrap();

    let result = load_source(&score_dir).expect("Failed to load source");

    // Should have newline between concatenated files
    assert!(result.source.contains("Title: \"Test\"\n"));
}
