use std::{fs::File};

use js_sys::JSON;
use regex::Regex;
use serde_json::Value;

use crate::config::Config;

pub struct LogMessageParser {
    pub text_field: String,
    config: Config
}

impl LogMessageParser {
    pub fn new(text_field: String) -> Self {
        let config = Config::load_config("config.json").unwrap();

        Self { text_field, config }
    }

    pub fn format(&mut self) {
        self.json_format();
        self.highlight_format();
    }

    pub fn get_text(&self) -> String {
        self.text_field.clone()
    }

    fn highlight_format(&mut self) {
        let text_field = &mut self.text_field;

        for rule in &self.config.highlight_rules {
            let style = rule
                .style
                .as_deref()
                // default styling
                .unwrap_or("color:white;font-weight:normal;");

            // capture group 0, entire match
            let replacement = format!(
                r#"<span style="{}">{}</span>"#,
                style, "$0"
            );

            match rule.rule_type.as_str() {
                // Handle exact match replacement
                "exact" => {
                    text_field.replace(&rule.pattern, &replacement.replace("$0", &rule.pattern));
                }
                // Handle regex match replacement
                "regex" => {
                    if let Ok(regex) = Regex::new(&rule.pattern) {
                        *text_field = regex.replace_all(text_field, replacement.as_str()).to_string();
                    }
                }
                _ => continue, // Ignore invalid rule types
            }
        }
    }

    fn deletetion_format(&mut self) {
        let text_field = &mut self.text_field;

        for rule in &self.config.deletion_rules {
            match rule.rule_type.as_str() {
                "exact" => {
                    text_field.replace(&rule.pattern, "to");
                }
                "regex" => {
                    if let Some(regex) = Regex::new(&rule.pattern) {
                        *text_field = regex.replace_all(text_field, )
                    }
                }
                _ => continue,
            }
        }
    } 

    fn json_format(&mut self) {
        let re = Regex::new(r"\{.*?\}").unwrap(); // Matches JSON-like content within {}

        self.text_field = re
            .replace_all(&self.text_field, |caps: &regex::Captures| {
                serde_json::from_str::<Value>(&caps[0])
                    .and_then(|json| serde_json::to_string_pretty(&json))
                    .unwrap_or_else(|_| caps[0].to_string())
            })
            .to_string();
    }
}
