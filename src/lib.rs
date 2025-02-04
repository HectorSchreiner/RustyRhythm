use parser::*;
use regex::Regex;
use serde_json::Value;
use util::get_document;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CssStyleDeclaration, Document, Element, HtmlElement};
#[macro_use]
mod util;
mod parser;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log!("Init");

    let mut parser = LogMessageParser::init();
    parser.format();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
}

#[wasm_bindgen]
pub fn parse_text(selector: &str) {
    let document = get_document();

    if let Some(element) = document.query_selector(selector).unwrap() {
        let mut log_message_parser = LogMessageParser::init();
        log_message_parser.format();
        element.set_inner_html(&log_message_parser.get_text());
    }
}
