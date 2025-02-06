use config::Config;
use parser::*;
use regex::Regex;
use serde_json::Value;
use util::get_document;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CssStyleDeclaration, Document, Element, HtmlElement};
#[macro_use]
mod util;
mod config;
mod parser;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log!("Init");

    let selector = ".test";
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    parse_text(selector);
}

#[wasm_bindgen]
pub fn parse_text(selector: &str) {
    log!("parse text has been called");

    let document = web_sys::window().unwrap().document().unwrap();
    log!("get_document");
    log!("{}", document.body().unwrap().inner_html());

    if let Some(element) = document.query_selector(selector).unwrap() {
        log!("has element");
        let mut log_message_parser = LogMessageParser::new(element.inner_html().to_string());
        log!("created parser");

        log_message_parser.format();
        log!("format");

        element.set_inner_html(&log_message_parser.get_text());
        log!("set html");
    }
    log!("after has element");
}
