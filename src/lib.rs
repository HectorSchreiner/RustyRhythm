use std::{fmt::format, rc::Rc};

use js_sys::Math::log;
use parser::*;
use regex::Regex;
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, MutationObserver, MutationObserverInit, NodeList, Node, Element, XmlHttpRequest};
#[macro_use]
mod util;
mod config;
mod parser;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log!("start - registering keypress listener");
    
    if let Err(e) = register_keypress_listener() {
        
        log!("Error registering keypress listener: {:?}", e);
    }
}

#[wasm_bindgen]
pub fn replace_ugly_name() -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or("Failed to get window")?
        .document()
        .ok_or("Failed to get document")?;

    let selector = ".tab-label.ng-scope";

    if let Ok(target) =  document.query_selector(&selector) {
        match target {
            Some(target) => {
                log!("{:?}", target.inner_html());
                target.set_text_content(Some("Swaggy Gangster Name"));
            }
            _ => {
                log!("Target not found");
            }
        }
    }
    Ok(())

}


#[wasm_bindgen]
pub fn parse_text(selector: &str) -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or("Failed to get window")?
        .document()
        .ok_or("Failed to get document")?;

    let elements = document.query_selector_all(selector).map_err(|_| "Failed to select logs")?;

    for i in 0..elements.length() {
        if let Some(node) = elements.item(i) {
            if let Ok(element) = node.dyn_into::<Element>() {
                let log_message_parser: LogMessageParser<Unformatted> =
                    LogMessageParser::new(element.inner_html());
                let formatted_text = log_message_parser.get_text();
                let cleaned_text_with_newlines = formatted_text.replace("\n", "<br>");
                log!("{:?}", cleaned_text_with_newlines);

                if let Ok(target) = document.query_selector(".detailsActionsScroll.ii-outer") {
                    match target {
                        Some(target) => {
                            log!("{:?}", target.text_content());
                            log!("{:?}", &cleaned_text_with_newlines);
                            target.set_inner_html(&format!("<pre>{:?}</pre>", formatted_text));
                        }
                        _ => {
                            log!("Target not found");
                        }
                    }
                }
                element.set_attribute("data-formatted", "true")?;
            }
        }
    }

    Ok(())
}

#[wasm_bindgen]
pub fn register_keypress_listener() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("Failed to get window")?;
    let document = window.document().ok_or("Failed to get document")?;

    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        if event.ctrl_key() && event.shift_key() && event.key() == "F" {
            log!("Ctrl + Shift + F Pressed");
            if let Err(e) = parse_text(".detailsActionsScroll.details-log-message.ng-binding") {
                log!("Error formatting logs: {:?}", e);
            }
        }
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    closure.forget(); 

    Ok(())
}

fn remove_quotes(input: &str) -> String {
    let re = Regex::new(r#"^"|"$"#).unwrap();
    re.replace_all(input, "").into_owned()
}
