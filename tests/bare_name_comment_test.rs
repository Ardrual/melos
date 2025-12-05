use melos::parser::parse;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_bare_name_comment() {
    let mut dsl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dsl_path.push("tests/bare_name_comment.mel");

    let input = fs::read_to_string(dsl_path).expect("Failed to read DSL file");
    let result = parse(&input).expect("Failed to parse");

    let part = &result.parts[0];
    println!("Part name: '{}'", part.name);
    
    // The name should be "Piano " (with trailing space) or "Piano", but definitely NOT contain "//"
    assert!(!part.name.contains("//"), "Part name contains comment!");
    assert_eq!(part.name.trim(), "Piano", "Part name is incorrect");
}
