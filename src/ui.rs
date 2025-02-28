use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use crate::util::get_document;

// replace the title of the box
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
