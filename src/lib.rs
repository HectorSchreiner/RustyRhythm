use regex::Regex;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, DomTokenList};
use js_sys::Math::log;

#[macro_use]
mod util;
mod config;
mod parser;
mod ui;

use ui::*;
use parser::*;
use crate::util::get_document;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log!("Starting - Registering Keypress Listener");

    if let Err(e) = register_keypress_listener() {
        log!("Error registering keypress listener: {:?}", e);
    }
}

#[wasm_bindgen]
pub fn register_keypress_listener() -> Result<(), JsValue> {
    let document = get_document()?;
    
    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        if event.ctrl_key() && event.shift_key() && event.key() == "F" {
            log!("Ctrl + Shift + F Pressed");
            if let Err(e) = parse_text() {
                log!("Error formatting logs: {:?}", e);
            }
        }
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}




