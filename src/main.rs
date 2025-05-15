use std::path::Path;

mod parser;
use parser::replace_conflict_markers;

mod resolver;

fn main() {
    // Check if "package.json" exists in the working directory
    let package_json_path = Path::new("package.json");
    if package_json_path.exists() {
        let mut output = replace_conflict_markers(package_json_path).unwrap();
        resolver::resolve_versions(&mut output);

        // Overwrite the original file with the resolved JSON
        std::fs::write(
            package_json_path,
            serde_json::to_string_pretty(&output).unwrap(),
        )
        .expect("Unable to write file");

        // Run 'npm i' to sync the lockfile
        std::process::Command::new("npm")
            .arg("i")
            .status()
            .expect("Failed to execute npm install");
    } else {
        // If it doesn't exist, print a message
        println!("package.json not found in the current directory.");
    }
}
