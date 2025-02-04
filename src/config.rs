use std::str::Chars;

use regex::*;

pub enum RuleType {
    EXACT,
    REGEX,
}

pub struct HighlightRule {
    pub rule_type: RuleType,
    pub pattern: String,
    pub style: Option<String>,
}

pub struct DeletionRule {
    pub rule_type: RuleType,
    pub pattern: String,
}

pub struct ChangeRule {
    pub rule_type: RuleType,
    pub pattern: String,
    pub replacement: String,
}

pub struct Config {
    pub highlight_rules: Vec<HighlightRule>,
    pub deletion_rules: Vec<DeletionRule>,
    pub change_rules: Vec<ChangeRule>,
}

pub fn load_config() -> Config {
    let string = std::fs::read_to_string("config.json").expect("Could not read the config file");
    let config = serde_json::from_str(&string).expect("Could not read JSON");
}
