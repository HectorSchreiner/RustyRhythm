use parser::*;
use wasm_bindgen::prelude::*;
#[macro_use]
mod util;
mod config;
mod parser;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    parse_text(".test");
}

#[wasm_bindgen]
pub fn parse_text(selector: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    // debug log
    log!("{}", document.body().unwrap().inner_html());
    if let Some(element) = document.query_selector(selector).unwrap() {
        let log_message_parser = LogMessageParser::new(element.inner_html().to_string());
        let mut log_message_parser = log_message_parser.format_config_rules();
        log_message_parser.json_format();
        element.set_inner_html(&log_message_parser.get_text());
    }
}
