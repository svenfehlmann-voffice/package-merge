use std::fmt::Write;
use std::fs;
use std::io::BufRead;
use std::path::Path;

use serde_json::Value;

#[derive(Debug, PartialEq)]
enum ConflictState {
    OutsideDependencies,
    InDependenciesWithoutConflict,
    InHead,
    InOrigin,
}

pub fn replace_conflict_markers(input_file: &Path) -> Result<Value, std::fmt::Error> {
    let file = fs::File::open(input_file).expect("Unable to open file");
    let reader = std::io::BufReader::new(file);
    let mut state = ConflictState::OutsideDependencies;

    let mut output = String::new();
    let mut head_buffer = String::new();
    let mut origin_buffer = String::new();

    let mut conflict_number: u32 = 0;

    for line_result in reader.lines() {
        let line = line_result.expect("Unable to read line");
        let trimmed_line = line.trim();

        match state {
            ConflictState::OutsideDependencies => {
                if trimmed_line.starts_with("\"dependencies\": {")
                    || trimmed_line.starts_with("\"devDependencies\": {")
                {
                    state = ConflictState::InDependenciesWithoutConflict;
                }
                writeln!(output, "{}", line)?;
            }

            ConflictState::InDependenciesWithoutConflict => {
                if trimmed_line.starts_with("<<<<<<<") {
                    state = ConflictState::InHead;
                    head_buffer.clear();
                    origin_buffer.clear();

                    // Start JSON object
                    writeln!(head_buffer, "{}", "{")?;
                } else if trimmed_line.starts_with("}") {
                    state = ConflictState::OutsideDependencies;
                    writeln!(output, "{}", line)?;
                } else {
                    writeln!(output, "{}", line)?;
                }
            }

            ConflictState::InHead => {
                if trimmed_line.starts_with("=======") {
                    state = ConflictState::InOrigin;
                    writeln!(head_buffer, "{}", "}")?;
                    writeln!(origin_buffer, "{}", "{")?;
                } else {
                    writeln!(head_buffer, "{}", line)?;
                }
            }

            ConflictState::InOrigin => {
                if trimmed_line.starts_with(">>>>>>>") {
                    // Write head and origin buffers to output
                    writeln!(origin_buffer, "{}", "}")?;

                    writeln!(output, "\"HEAD_{}\": {},", conflict_number, head_buffer)?;
                    writeln!(output, "\"ORIGIN_{}\": {},", conflict_number, origin_buffer)?;

                    // println!("Head buffer:\n{}\n", head_buffer);
                    // println!("Origin buffer:\n{}\n", origin_buffer);

                    state = ConflictState::InDependenciesWithoutConflict;

                    conflict_number += 1;
                } else {
                    writeln!(origin_buffer, "{}", line)?;
                }
            }
        }
    }

    if state != ConflictState::OutsideDependencies {
        // Throw error
        return Err(std::fmt::Error);
    }

    // Serialize and deserialize the output to ensure valid JSON with serde
    match json5::from_str(&output) {
        Ok(json) => return Ok(json),
        Err(_) => return Err(std::fmt::Error),
    }
}
