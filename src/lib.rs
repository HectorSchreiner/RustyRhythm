use parser::*;
use wasm_bindgen::prelude::*;
use web_sys::{Document, MutationObserver, MutationObserverInit};
#[macro_use]
mod util;
mod config;
mod parser;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    parse_text(".detailsActionsScroll.details-log-message.ng-binding").unwrap();
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
        let formatted_text = format!("hector was here{:?}", log_message_parser.get_text());
        log!("{:?}", formatted_text);
        element.set_text_content(Some(&formatted_text));
    }

    Ok(())
}

#[wasm_bindgen]
pub fn observe_dom_changes(selector: &str) -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or("Failed to get window")?
        .document()
        .ok_or("Failed to get document")?;

    //let callback = ||
    //let mutation_observer = MutationObserver::new();

    Ok(())
}
