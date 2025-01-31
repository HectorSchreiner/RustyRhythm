use util::get_document;
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element};
#[macro_use]
mod util;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let selector = ".detailsActionScroll .details-log-message .ng-binding";
    log!("Hello from Rust");
    make_the_window_small();
}

#[wasm_bindgen]
pub fn reformat_div(selector: &str, new_text: &str) {
    let document = get_document();

    if let Some(element) = document.query_selector(selector).unwrap() {
        element.set_inner_html(new_text);
    }
}

#[wasm_bindgen]
pub fn make_the_window_small() {
    // Resize the window to 500px by 500px.
    let window = web_sys::window().unwrap();
    window
        .resize_to(500, 500)
        .expect("could not resize the window");
    log!("window didnt get smaller?");
}
