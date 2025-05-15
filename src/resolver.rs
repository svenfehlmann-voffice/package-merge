use serde_json::Value;
use std::collections::HashMap;

use serde_json::Map;

fn resolve_dependencies(dependencies: &mut Map<String, Value>) {
    let keys: Vec<String> = dependencies.keys().cloned().collect();

    let mut resolved_dependencies: HashMap<String, String> = HashMap::new();

    for key in keys {
        if key.starts_with("HEAD") {
            let index = key.split('_').nth(1).unwrap();

            let head_key = format!("HEAD_{}", index);
            let origin_key = format!("ORIGIN_{}", index);

            // Iterate over head dependencies
            let head_deps = dependencies[&head_key].as_object().unwrap();
            let origin_deps = dependencies[&origin_key].as_object().unwrap();

            for (dep_key, dep_value) in head_deps {
                let value = dep_value.as_str().unwrap();
                resolved_dependencies.insert(String::from(dep_key), String::from(value));
            }

            for (dep_key, dep_value) in origin_deps {
                let value = dep_value.as_str().unwrap();

                // check if the dependency is already in the resolved dependencies
                if resolved_dependencies.contains_key(dep_key) {
                    // If it is, compare the versions and keep the one with the highest version
                    let existing_value = resolved_dependencies.get(dep_key).unwrap();
                    if value > existing_value {
                        resolved_dependencies.insert(String::from(dep_key), String::from(value));
                    }
                } else {
                    // If it is not, add it to the resolved dependencies
                    resolved_dependencies.insert(String::from(dep_key), String::from(value));
                }
            }

            // Remove the HEAD and ORIGIN keys from the dependencies map
            dependencies.remove(&head_key);
            dependencies.remove(&origin_key);
        }
    }

    // Add the resolved dependencies to the dependencies map
    for (dep_key, dep_value) in resolved_dependencies {
        dependencies.insert(dep_key, Value::String(dep_value));
    }
}

pub fn resolve_versions(json: &mut Value) {
    if let Some(dependencies) = json["dependencies"].as_object_mut() {
        resolve_dependencies(dependencies);
    } else {
        println!("No dependencies found in the JSON.");
        // Delete the dependencies key if it exists
        json.as_object_mut().unwrap().remove("dependencies");
    }

    if let Some(dev_dependencies) = json["devDependencies"].as_object_mut() {
        resolve_dependencies(dev_dependencies);
    } else {
        println!("No devDependencies found in the JSON.");
        // Delete the devDependencies key if it exists
        json.as_object_mut().unwrap().remove("devDependencies");
    }
}
