use js_sys::Promise;
use regex::Regex;
use wasm_bindgen::JsValue;
use std::time::Duration;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Document};

#[allow(dead_code)]
pub async fn sleep(duration: Duration) {
    JsFuture::from(Promise::new(&mut |yes, _| {
        window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &yes,
                duration.as_millis() as i32,
            )
            .unwrap();
    }))
    .await
    .unwrap();
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn get_document() -> Result<Document, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Failed to get window"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("Failed to get document"))?;
    Ok(document)
}

pub fn remove_quotes(input: &str) -> String {
    Regex::new(r#"^"|"$"#).unwrap().replace_all(input, "").into_owned()
}
