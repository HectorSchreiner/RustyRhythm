use regex::Regex;
use serde_json::Value;

pub struct LogMessageParser {
    pub text_field: String,
}

impl LogMessageParser {
    pub fn new(text_field: String) -> Self {
        Self { text_field }
    }

    pub fn format(&self) {
        self.json_format();
        self.highlight_elements();
    }

    pub fn get_text(&self) -> String {
        self.text_field.clone()
    }

    fn highlight_elements(&self) {
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

    fn json_format(&self) {
        if let Ok(json) = serde_json::from_str::<Value>(&self.text_field) {
            // hvis json parsningen fejler, returnerer den bare inputtet som string.
            serde_json::to_string_pretty(&json).unwrap_or(self.text_field.to_string());
        }
    }
}
