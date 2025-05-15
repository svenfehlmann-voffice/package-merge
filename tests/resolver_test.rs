use serde_json::{Map, Value};
use std::path::Path;

use package_merge::{parser::replace_conflict_markers, resolver::resolve_versions};

#[test]
fn no_dev_dependencies() {
    let mixed_conflict_file = Path::new("tests/files/package.json");

    let mut output = replace_conflict_markers(&mixed_conflict_file).unwrap();

    resolve_versions(&mut output);

    // output should not contain a devDependencies key
    if let Some(output_object) = output.as_object() {
        // Extract keys as vector
        let keys: Vec<String> = output_object.keys().cloned().collect();

        // Check if "devDependencies" is present
        assert!(!keys.contains(&String::from("devDependencies")));
    }
}

fn assert_version(dependencies: &Map<String, Value>, key: &str, expected_version: &str) {
    assert_eq!(
        dependencies[key].as_str().unwrap(),
        expected_version,
        "Version mismatch for {}",
        key
    );
}

#[test]
fn latest_version() {
    let mixed_conflict_file = Path::new("tests/files/package.json");

    let mut output = replace_conflict_markers(&mixed_conflict_file).unwrap();

    resolve_versions(&mut output);

    if let Some(output_object) = output.as_object() {
        // Check all test dependency versions
        let dependencies = output_object["dependencies"].as_object().unwrap();

        assert_version(dependencies, "package-1", "1.0.0");
        assert_version(dependencies, "package-2", "1.1.0");
        assert_version(dependencies, "package-3", "1.0.0");
        assert_version(dependencies, "package-4", "1.4.0");
        assert_version(dependencies, "package-5", "1.1.0");
        assert_version(dependencies, "package-6", "1.0.0");
        assert_version(dependencies, "package-7", "1.0.0");
        assert_version(dependencies, "package-8", "1.0.0");
        assert_version(dependencies, "package-9", "1.0.0");
    }
}
