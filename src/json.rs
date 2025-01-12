use std::{fs, path::PathBuf};

use serde_json::Value;

#[cfg(test)]
const RELATIVE_JSON_PATH: &str = "test/data/rss2webhook_data.json";

#[cfg(not(test))]
const RELATIVE_JSON_PATH: &str = "data/rss2webhook_data.json";

fn get_json_file_location() -> PathBuf {
    match std::env::current_dir() {
        Ok(dir) => dir.join(RELATIVE_JSON_PATH),
        Err(_) => {
            eprintln!("Failed to get current directory, using default path");
            PathBuf::from(RELATIVE_JSON_PATH)
        }
    }
}

// use dyn because maybe return std::io::Error or serde_json::Error
fn read_json() -> Result<Value, Box<dyn std::error::Error>> {
    let json_file_location = get_json_file_location();

    // trying to get file
    let file_content = fs::read(&json_file_location)
        .map_err(|e| format!("Failed to read the JSON file: {:?}", e))?;

    // trying to parse JSON
    let parsed_json = serde_json::from_slice(&file_content)
        .map_err(|e| format!("Failed to parse the JSON file: {:?}", e))?;

    Ok(parsed_json)
}

// under develop, remove when finish
#[allow(dead_code)]
fn main() {
    let _ = read_json();
}

#[cfg(test)]
mod tests {
    use super::*;

    // test for read json file
    #[test]
    fn test_read_json() {
        match read_json() {
            Ok(json) => {
                assert!(json.is_array(), "Expected a JSON object");
                println!("{:?}", json);
            }
            Err(e) => panic!("Error reading JSON: {}", e),
        }
    }
}
