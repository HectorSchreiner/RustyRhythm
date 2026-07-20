use regex::Regex;
use wasm_bindgen::prelude::*;
use web_sys::{Document, DomTokenList, Element, HtmlElement, MutationObserver, MutationObserverInit, MutationRecord, Node};
use js_sys::{Function, Math::log};

#[macro_use]
mod util;
mod config;
mod parser;
mod ui;

use ui::replace_ugly_name;
use parser::*;
use crate::util::get_document;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log!("Starting - Registering MutationObserver");

    if let Err(e) = register_dom_listener() {
        log!("Error registering DOM listener: {:?}", e);
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

#[wasm_bindgen]
pub fn register_dom_listener() -> Result<(), JsValue> {
    let document = get_document()?;
    let selector = ".detailsActionsScroll.details-log-message.ng-binding";

    if let Some(target) = document.query_selector(selector).ok().flatten() {
        let target_node = target.dyn_into::<Node>().unwrap();

        let callback = Closure::wrap(Box::new(move |_records: Vec<MutationRecord>, _observer: MutationObserver| {
            log!("Log content changed - Parsing...");
            if let Err(e) = parse_text() {
                log!("Error formatting logs: {:?}", e);
            }

            if let Err(e) = replace_ugly_name() {
                log!("Error replacing the name: {:?}", e);
            }
        }) as Box<dyn FnMut(Vec<MutationRecord>, MutationObserver)>);

        let observer = MutationObserver::new(callback.as_ref().unchecked_ref::<Function>())?;

        let mut config = MutationObserverInit::new();
        
        config.set_child_list(true);
        config.set_character_data(true);
        config.set_subtree(true);

        observer.observe_with_options(&target_node, &config)?;

        // Prevent Rust from dropping the callback
        callback.forget();
    } else {
        log!("Log message element not found");
    }

    Ok(())
}


