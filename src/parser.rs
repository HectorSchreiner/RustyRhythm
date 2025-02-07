use regex::Regex;
use serde_json::Value;
use std::marker::PhantomData;

use crate::config::Config;

pub struct Formatted;
pub struct Unformatted;

pub struct LogMessageParser<State> {
    pub text_field: String,
    config: Config,
    state: PhantomData<State>,
}

impl<State> LogMessageParser<State> {
    pub fn new(text_field: String) -> Self {
        let config = Config::load_config().unwrap();
        Self {
            text_field,
            config,
            state: Default::default(),
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text_field
    }
}

impl LogMessageParser<Unformatted> {
    pub fn json_format(mut self) -> LogMessageParser<Formatted> {
        let re_whitespace = Regex::new(r"\s+").unwrap(); // Matches any whitespace (spaces, newlines, tabs)
        self.text_field = re_whitespace
            .replace_all(&self.text_field.trim(), " ")
            .into_owned();

        let re = Regex::new(r"\{.*?\}").unwrap(); // Matches JSON-like content within {}

        self.text_field = re
            .replace_all(&self.text_field, |caps: &regex::Captures| {
                serde_json::from_str::<Value>(&caps[0])
                    .and_then(|json| serde_json::to_string_pretty(&json))
                    .unwrap_or_else(|_| caps[0].to_string())
            })
            .into_owned();

        LogMessageParser {
            text_field: self.text_field,
            config: self.config,
            state: PhantomData::default(),
        }
    }
}

impl LogMessageParser<Formatted> {
    pub fn format_config_rules(mut self) -> Self {
        self.deletetion_format();
        self.change_format();
        self.highlight_format();
        self
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
            let replacement = format!(r#"<span style="{}">{}</span>"#, style, "$0");

            match rule.rule_type.as_str() {
                // Handle exact match replacement
                "exact" => {
                    *text_field = text_field
                        .replace(&rule.pattern, &replacement.replace("$0", &rule.pattern));
                }
                // Handle regex match replacement
                "regex" => {
                    if let Ok(regex) = Regex::new(&rule.pattern) {
                        *text_field = regex
                            .replace_all(text_field, replacement.as_str())
                            .to_string();
                    }
                }
                _ => continue, // Ignore invalid rule types
            }
        }
    }

    fn deletetion_format(&mut self) {
        let text_field = &mut self.text_field;

        for rule in &self.config.deletion_rules {
            let empty = "";
            match rule.rule_type.as_str() {
                "exact" => {
                    *text_field = text_field.replace(&rule.pattern, empty);
                }
                "regex" => {
                    if let Ok(regex) = Regex::new(&rule.pattern) {
                        *text_field = regex.replace_all(text_field, empty).to_string();
                    }
                }
                _ => continue,
            }
        }
    }

    fn change_format(&mut self) {
        let text_field = &mut self.text_field;

        for rule in &self.config.change_rules {
            let replacement = rule.replacement.as_str();
            match rule.rule_type.as_str() {
                "exact" => {
                    *text_field = text_field.replace(&rule.pattern, replacement);
                }
                "regex" => {
                    if let Ok(regex) = Regex::new(&rule.pattern) {
                        *text_field = regex.replace_all(text_field, replacement).to_string();
                    }
                }
                _ => continue,
            }
        }
    }
}
