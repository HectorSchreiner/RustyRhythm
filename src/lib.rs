use parser::*;
use wasm_bindgen::prelude::*;
use web_sys::{console::log, Document, MutationObserver, MutationObserverInit};
#[macro_use]
mod util;
mod config;
mod parser;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let selector = ".detailsActionsScroll.details-log-message.ng-binding";
    loop {
        observe_dom_changes(selector.to_string()).unwrap();
        log!("hello dom");
        // Wait for a short period before checking again
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
            web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 100).unwrap();
        })).await.unwrap();
    }
}

#[wasm_bindgen]
pub fn parse_text(selector: &str) -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or("Failed to get window")?
        .document()
        .ok_or("Failed to get document")?;
    // debug log
    if let Some(element) = document.query_selector(selector).ok().flatten() {
        log!("Here is the html: {:?}", element.inner_html());
        let log_message_parser: LogMessageParser<Formatted> =
            LogMessageParser::new(element.inner_html());
        //let log_message_parser = log_message_parser.json_format().format_config_rules();
        let formatted_text = format!("hector was here: {:?}", log_message_parser.get_text());
        log!("{:?}", formatted_text);
        element.set_text_content(Some(&formatted_text));
    }

    Ok(())
}

#[wasm_bindgen]
pub fn observe_dom_changes(selector: String) -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or("Failed to get window")?
        .document()
        .ok_or("Failed to get document")?;

    let element = document
        .query_selector(&selector)
        .ok()
        .flatten()
        .ok_or("failed to find element")?;

    let selector = selector.to_string();

    let callback = Closure::wrap(Box::new(
        move |_mutation_list: js_sys::Array, _observer: web_sys::MutationObserver| {
            let selector_clone = selector.clone();
            if let Some(element) = document.query_selector(&selector_clone).ok().flatten() { 
                if element.inner_html().is_empty() {
                    // debug log
                    log!("Element is empty, doing nothing.");
                } else {
                    log!("Text was found");
                    if let Err(err) = parse_text(&selector) {
                        log!("Error reformatting text: {:?}", err);
                    }
                } 
            } 
            log!("didnt find anything");
        },
    )
        as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);

    let mutation_observer = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref())?;

    let mutation_config = web_sys::MutationObserverInit::new();
    mutation_config.set_child_list(true);
    mutation_config.set_subtree(true);
    

    mutation_observer
        .observe_with_options(&element, &mutation_config)
        .unwrap();
    callback.forget();
   
    Ok(())
}
