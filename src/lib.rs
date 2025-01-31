use format::*;
use regex::Regex;
use serde_json::Value;
use util::get_document;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CssStyleDeclaration, Document, Element, HtmlElement};
#[macro_use]
mod util;
mod format;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log!("Init");

    let selector = ".detailsActionScroll .details-log-message .ng-binding";
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
}

#[wasm_bindgen]
pub fn parse_text(selector: &str, new_text: &str) {
    let document = get_document();

    if let Some(element) = document.query_selector(selector).unwrap() {
        element.set_inner_html(new_text);
    }
}
/*
#[wasm_bindgen]
pub fn popup() {
    let window = window().expect("No global `window` exists");
    let document = window.document().expect("No global `document` exists");

    // Ensure we are inside the popup
    if document.get_element_by_id("popup-body").is_none() {
        return; // Do nothing if the popup body is not found
    }

    // Create button inside the popup
    let button = document.create_element("button").unwrap();
    button.set_inner_html("Toggle Rust Content");
    button.set_attribute("id", "toggle-btn").unwrap();

    let popup_body = document.get_element_by_id("popup-body").unwrap();
    popup_body.append_child(&button).unwrap();

    // Create hidden div inside the popup
    let div = document.create_element("div").unwrap();
    div.set_inner_html("<h1>Hello from Rust!</h1>");
    div.set_attribute("id", "rust-content").unwrap();
    div.set_attribute("style", "display: none;").unwrap();
    popup_body.append_child(&div).unwrap();

    // Add event listener to button
    let closure = Closure::wrap(Box::new(move || {
        let div = document.get_element_by_id("rust-content").unwrap();
        let style = div.dyn_into::<HtmlElement>().unwrap().style();
        if style.display() == "none" {
            style.set_property("display", "block").unwrap();
        } else {
            style.set_property("display", "none").unwrap();
        }
    }) as Box<dyn Fn()>);

    button
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(closure.as_ref().unchecked_ref()));

    closure.forget(); // Prevent Rust from dropping the closure
}
*/
