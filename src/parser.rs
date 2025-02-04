use std::fs::File;

use js_sys::JSON;
use regex::Regex;
use serde_json::Value;

pub struct LogMessageParser {
    pub text_field: String,
}

impl LogMessageParser {
    pub fn new(text_field: String) -> Self {
        let string =
            std::fs::read_to_string("config.json").expect("Could not read the config file");
        let config: Value = serde_json::from_str(&string).expect("Could not read JSON");

        Self { text_field }
    }

    pub fn format(&mut self) {
        self.json_format();
        self.highlight_elements();
    }

    pub fn get_text(&self) -> String {
        self.text_field.clone()
    }

    fn highlight_elements(&mut self) {

        
        // regex to capture all ips.

        let ip_regex = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();

        // skal i capturegroup index 0, da vi kun har 1 capture group i ovenstående regex string.
        // Skal måske ændres hvis der laves nogle flere regex strings, som skal formatteres.
        ip_regex.replace_all(&self.text_field, |captures: &regex::Captures| {
            format!(
                r#"<span style="color:blue;font-weight:bold;">{}</span>"#,
                &captures[0]
            )
        });
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
