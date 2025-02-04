use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct HighlightRule {
    pub rule_type: String,
    pub pattern: String,
    pub style: Option<String>,
}
impl HighlightRule {
    pub fn new(rule_type: String, pattern: String, style: Option<String>) -> Self {
        Self { rule_type, pattern, style }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletionRule {
    pub rule_type: String,
    pub pattern: String,
} 

impl DeletionRule {
    pub fn new(rule_type: String, pattern: String,) -> Self {
        Self { rule_type, pattern }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeRule {
    pub rule_type: String,
    pub pattern: String,
    pub replacement: String,
}

impl ChangeRule {
    fn new(rule_type: String, pattern: String, replacement: String) -> Self {
        Self { rule_type, pattern, replacement }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub highlight_rules: Vec<HighlightRule>,
    pub deletion_rules: Vec<DeletionRule>,
    pub change_rules: Vec<ChangeRule>,
} 

impl Config {
    pub fn load_config(path: &str) -> std::io::Result<Self> {
        let config_data =
            std::fs::read_to_string(path).expect("Could not read the config file");

        // Parse the config data into the Config struct
        let config: Config = serde_json::from_str(&config_data)?;

        Ok(config)
    }
}

