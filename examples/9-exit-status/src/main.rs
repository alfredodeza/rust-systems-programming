use serde::Deserialize;
use serde_json;
use std::{fs, f32::consts::E, vec};
use glob::glob;
use std::os::unix::fs::PermissionsExt;

// load the regex-fules.json file to provide configs
const JSON: &str = include_str!("../rules.json");


#[derive(Deserialize, Debug)]
struct ComplianceRule {
    path_regex: String,
    file_permissions: u32,
    required_files: Vec<String>,
}

impl ComplianceRule {
    fn new(path_regex: String, file_permissions: u32, required_files: Vec<String>) -> Self {
        Self {
            path_regex,
            file_permissions,
            required_files,
        }
    }
}

// Load the rules from a configuration file (JSON)
fn load_rules() -> Vec<ComplianceRule> {
    let loaded_json: Vec<ComplianceRule> = serde_json::from_str(JSON).unwrap();

    let mut rules: Vec<ComplianceRule> = Vec::new();
    for rule in loaded_json {
        rules.push(ComplianceRule::new(
            rule.path_regex,
            rule.file_permissions,
            rule.required_files,
        ));
    }
    rules
}

fn main() {
    let rules = load_rules(); 
    let mut failed: bool = false;
    for rule in rules {
        let mut seen_files: Vec<String> = Vec::new();
        for entry in glob(&rule.path_regex).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    if path.is_dir() {
                        continue;
                    }
                    seen_files.push(path.to_str().unwrap().to_string());
                    let metadata = fs::metadata(&path).unwrap();
                    if metadata.permissions().mode() != rule.file_permissions {
                        failed = true;
                        println!("[FAIL] incorrect permissions: {:?}", path);
                    }


                }
                Err(e) => println!("{:?}", e),
            }
        }
        for file in &rule.required_files {
            if !seen_files.contains(&file) {
                failed = true;
                println!("[FAIL] required file {file} not found in {}", rule.path_regex);
            }
        }
        
    }
    if failed {
        std::process::exit(2);
    }
}
