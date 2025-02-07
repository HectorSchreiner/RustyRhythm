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
        Self {
            rule_type,
            pattern,
            style,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletionRule {
    pub rule_type: String,
    pub pattern: String,
}

impl DeletionRule {
    pub fn new(rule_type: String, pattern: String) -> Self {
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
        Self {
            rule_type,
            pattern,
            replacement,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub highlight_rules: Vec<HighlightRule>,
    pub deletion_rules: Vec<DeletionRule>,
    pub change_rules: Vec<ChangeRule>,
}

impl Config {
    const DATA: &[u8] = include_bytes!("../config.json");
    pub fn load_config() -> std::io::Result<Self> {
        Ok(serde_json::from_slice(Self::DATA)?)
    }
}
