use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element};
#[macro_use]
mod util;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log!("Hello from Rust");
    make_the_window_small();
}

#[wasm_bindgen]
pub fn change_text(new_text: &str) {
    let window = web_sys::window().unwrap_or_else(|| {
        log!("no window found");
        panic!()
    });
    let document: web_sys::Document = window.document().unwrap_or_else(|| {
        log!("no document found");
        panic!()
    });

    if let Some(element) = document.query_selector(".detailsActionScroll .details-log-message .ng-binding").unwrap() {
        element.set_inner_html(new_text);
    }
}


#[wasm_bindgen]
pub fn make_the_window_small() {
    // Resize the window to 500px by 500px.
    let window = web_sys::window().unwrap();
    window.resize_to(500, 500)
        .expect("could not resize the window");
    log!("window didnt get smaller?");
}