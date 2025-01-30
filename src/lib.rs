use wasm_bindgen::prelude::*;
use web_sys::*;
#[macro_use]
mod util;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log!("Hello from Rust");
    make_the_window_small();
}

#[wasm_bindgen]
pub fn change_text(new_text: &str) {
    let window = window().expect("no window found");
}


#[wasm_bindgen]
pub fn make_the_window_small() {
    // Resize the window to 500px by 500px.
    let window = web_sys::window().unwrap();
    window.resize_to(500, 500)
        .expect("could not resize the window");
    log!("window didnt get smaller?");
}