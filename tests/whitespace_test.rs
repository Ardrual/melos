use melos::parser::parse;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_whitespace_handling() {
    let mut dsl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dsl_path.push("tests/whitespace_repro.mel");

    let input = fs::read_to_string(dsl_path).expect("Failed to read DSL file");
    let result = parse(&input);

    match result {
        Ok(_) => println!("Successfully parsed whitespace_repro.mel"),
        Err(e) => panic!("Failed to parse whitespace_repro.mel: {:?}", e),
    }
}
