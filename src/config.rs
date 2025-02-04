use std::str::Chars;

use regex::*;
use serde_json::*;

pub enum RuleType {
    EXACT,
    REGEX,
}

pub struct HighlightRule {
    pub rule_type: RuleType,
    pub pattern: String,
    pub style: Option<String>,
}
impl HighlightRule {
    pub fn new(rule_type: RuleType, pattern: String, style: Option<String>) -> Self {
        Self { rule_type, pattern, style }
    }
}

pub struct DeletionRule {
    pub rule_type: RuleType,
    pub pattern: String,
} 

impl DeletionRule {
    pub fn new(rule_type: RuleType, pattern: String,) -> Self {
        Self { rule_type, pattern }
    }
}

pub struct ChangeRule {
    pub rule_type: RuleType,
    pub pattern: String,
    pub replacement: String,
}

impl ChangeRule {
    fn new(rule_type: RuleType, pattern: String, replacement: String) -> Self {
        Self { rule_type, pattern, replacement }
    }
}

pub struct Config {
    pub highlight_rules: Vec<HighlightRule>,
    pub deletion_rules: Vec<DeletionRule>,
    pub change_rules: Vec<ChangeRule>,
}

impl Config {
    // pub fn load_config(path: &str) -> Result<Config> {
    //     let string = std::fs::read_to_string(path).expect("Could not read the config file");
    //     let value: Value = serde_json::from_str(&string).expect("Could not read JSON");

    //     if let Some(highlight_rules) = value.get("highlight_rules").and_then(|v| v.as_array()) {
    //         for rule in highlight_rules {
    //             if let (Some(rule_type), Some(pattern)) = (rule.get("rule_type"), rule.get("pattern")) {
    //                 if let (Some(rule_type_str), Some(pattern_str)) = (rule_type.as_str(), pattern.as_str()) {
    //                     let rule_type = match rule_type_str {
    //                         "exact" => RuleType::EXACT,
    //                         "regex" => RuleType::REGEX,
    //                         _ => continue, // Skip invalid rule types
    //                     };

    //                     let style = rule.get("style").and_then(|s| s.as_str()).map(String::from);

    //                     let highlight_rule = HighlightRule {
    //                         rule_type,
    //                         pattern: pattern_str.to_string(),
    //                         style,
    //                     };
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }

}

