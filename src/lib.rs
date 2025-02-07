use parser::*;
use wasm_bindgen::prelude::*;
#[macro_use]
mod util;
mod config;
mod parser;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    parse_text(".test").unwrap();
}

#[wasm_bindgen]
pub fn parse_text(selector: &str) -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or("Failed to get window")?
        .document()
        .ok_or("Failed to get document")?;

    // debug log
    if let Some(element) = document.query_selector(selector).ok().flatten() {
        log!("{:?}", element.inner_html());
        let log_message_parser = LogMessageParser::new(element.inner_html());
        let log_message_parser = log_message_parser.json_format().format_config_rules();
        let formatted_text = format!("<pre>{}</pre>", log_message_parser.get_text());
        log!("{:?}", formatted_text);
        element.set_inner_html(&formatted_text);
    }

    Ok(())
}
