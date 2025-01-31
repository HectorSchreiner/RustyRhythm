use js_sys::Promise;
use std::time::Duration;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Document, Element, Window};

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

pub fn get_document() -> Document {
    let window = window().unwrap_or_else(|| {
        log!("no window found");
        panic!()
    });
    let document: web_sys::Document = window.document().unwrap_or_else(|| {
        log!("no document found");
        panic!()
    });

    return document;
}
