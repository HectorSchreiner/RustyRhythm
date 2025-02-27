use regex::Regex;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, DomTokenList};
use js_sys::Math::log;

#[macro_use]
mod util;
mod config;
mod parser;
use parser::*;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log!("Starting - Registering Keypress Listener");

    if let Err(e) = register_keypress_listener() {
        log!("Error registering keypress listener: {:?}", e);
    }
}

// replace the title for the page! 
#[wasm_bindgen]
pub fn replace_ugly_name() -> Result<(), JsValue> {
    let document = get_document()?;
    let selector = ".tab-label.ng-scope";

    if let Some(target) = document.query_selector(selector).ok().flatten() {
        log!("Replacing tab name: {:?}", target.inner_html());
        target.set_text_content(Some("Swaggy Gangster Name"));
    } else {
        log!("Target not found");
    }

    Ok(())
}

#[wasm_bindgen]
pub fn parse_text() -> Result<(), JsValue> {
    let document = get_document()?;
    
    let selector = ".detailsActionsScroll.details-log-message.ng-binding";
    
    if let Some(element) = document.query_selector(selector).ok().flatten() {
        log!("{:?}", element);
        log!("Processing log...");

        let current_text = element.text_content();
        if let Some(current_text) = element.text_content() {

            // Check if already formatted, and don't reformat
            if let Some(prev_text) = element.get_attribute("data-original-text") {
                if prev_text == current_text {
                    log!("Skipping formatting: Log content unchanged.");
                    return Ok(());
                }
            }
        
            // Parse and format log message
            let log_message_parser = LogMessageParser::new(current_text.clone()).json_format().format_config_rules();
            let formatted_text = log_message_parser.get_text();
            log!("Cleaned text{}", formatted_text);

            // Try to find the target element with either class
            if let Some(target) = document.query_selector(".detailsActionsScroll.ii-outer").ok().flatten()
                .or_else(|| document.query_selector(".detailsActionsScroll.customclass").ok().flatten()) {
                
                if let Ok(target) = target.dyn_into::<HtmlElement>() {
                    // Remove the .ii-outer class and add the custom class
                    target.class_list().remove_1("ii-outer").unwrap();
                    target.class_list().add_1("customclass").unwrap();
                    log!("Updated class to customclass");

                    // Change the content inside the div
                    target.set_inner_html(&format!("{}", &formatted_text));
                    log!("Changed content inside the div");
                }
            } else {
                log!("Target for formatted log not found");
            }
        } else {
            log!("Log message div not found");
        }
    }
    Ok(())
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

fn get_document() -> Result<Document, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Failed to get window"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("Failed to get document"))?;
    Ok(document)
}


