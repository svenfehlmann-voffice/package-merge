use package_merge::parser::replace_conflict_markers;
use std::path::Path;

#[test]
fn test_empty_file() {
    let empty_file = Path::new("tests/files/empty.json");

    let output = replace_conflict_markers(&empty_file);
    assert!(output.is_err());
}

#[test]
fn test_no_conflict() {
    let no_conflict_file = Path::new("tests/files/no_conflict.json");

    let output = replace_conflict_markers(&no_conflict_file);
    assert!(output.is_ok());
}

#[test]
fn unclosed_marker() {
    let no_conflict_file = Path::new("tests/files/unclosed_marker.json");

    let output = replace_conflict_markers(&no_conflict_file);
    assert!(output.is_err());
}
