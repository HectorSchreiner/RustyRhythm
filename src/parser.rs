use regex::Regex;
use serde_json::Value;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::HtmlElement;
use std::marker::PhantomData;

use crate::{config::Config, util::get_document};

// Holds the state for the logmessage parser, to ensure json format comes before the highlight rules!
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
        let re_whitespace = Regex::new(r"\s").unwrap(); // Matches any whitespace (spaces, newlines, tabs)
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
        self.text_field = self.text_field.replace("<", "&lt;").replace(">", "&gt;"); 
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
            let empty = ", ";
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

#[wasm_bindgen]
pub fn parse_text() -> Result<(), JsValue> {
    let document = get_document()?;
    
    let selector = ".detailsActionsScroll.details-log-message.ng-binding";
    
    if let Some(element) = document.query_selector(selector).ok().flatten() {
        log!("{:?}", element);
        log!("Processing log...");

        let current_text = element.text_content();
        if let Some(current_text) = element.text_content() {

            // Check if already formatted, and don't reformat
            if let Some(prev_text) = element.get_attribute("data-original-text") {
                if prev_text == current_text {
                    log!("Skipping formatting: Log content unchanged.");
                    return Ok(());
                }
            }
        
            // Parse and format log message
            let log_message_parser = LogMessageParser::new(current_text.clone()).json_format().format_config_rules();
            let formatted_text = log_message_parser.get_text();
            log!("Cleaned text{}", formatted_text);

            // Try to find the target element with either class
            if let Some(target) = document.query_selector(".detailsActionsScroll.ii-outer").ok().flatten()
                .or_else(|| document.query_selector(".detailsActionsScroll.hectorsswaggycustomclass").ok().flatten()) {
                
                if let Ok(target) = target.dyn_into::<HtmlElement>() {
                    // Remove the .ii-outer class and add the custom class
                    target.class_list().remove_1("ii-outer").unwrap();
                    target.class_list().add_1("hectorsswaggycustomclass").unwrap();
                    log!("Updated class to customclass");

                    // Change the content inside the div
                    target.set_inner_html(&format!("{}", &formatted_text));
                    log!("Changed content inside the div");
                }
            } else {
                log!("Target for formatted log not found");
            }
        } else {
            log!("Log message div not found");
        }
    }
    Ok(())
}
