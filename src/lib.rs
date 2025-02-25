use std::rc::Rc;

use parser::*;
use wasm_bindgen::prelude::*;
use web_sys::{console::log, window, Document, MutationObserver, MutationObserverInit};
#[macro_use]
mod util;
mod config;
mod parser;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log!("start");
    let selector = ".detailsActionsScroll.details-log-message.ng-binding";
    dom_observer().unwrap(); // perchance handle errors! 

    // loop {
    //     log!("hello dom");
    //     // Wait for a short period before checking again
    //     wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
    //         web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 2000).unwrap();
    //     })).await.unwrap();
    // }
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
pub fn dom_observer() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();

    let callback = Closure::wrap(Box::new(move |mutations: js_sys::Array, _: web_sys::MutationObserver| {
        log!("callback called!");
        if let Ok(Some(target_div)) = document.query_selector(".detailsActionsScroll.details-log-message.ng-binding") {
            log!("{:?}", target_div.inner_html());
        }
    }) as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);

    let observer = MutationObserver::new(callback.as_ref().unchecked_ref())?;

    let options = MutationObserverInit::new();
    options.set_child_list(true);
    options.set_subtree(true);
    
    let target = web_sys::window()
       .ok_or("Failed to get window")?
       .document()
       .ok_or("Failed to get document")?;

    observer.observe_with_options(&target.as_ref(), &options)?;

    callback.forget();

    Ok(())
}
