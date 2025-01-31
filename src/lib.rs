use util::{get_document, replace_text};
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element};
#[macro_use]
mod util;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log!("Init");
    let selector = ".detailsActionScroll .details-log-message .ng-binding";
}

#[wasm_bindgen]
pub fn replace_text(selector: &str, new_text: &str) {
    let document = get_document();

    if let Some(element) = document.query_selector(selector).unwrap() {
        element.set_inner_html(new_text);
    }
}
