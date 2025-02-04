use serde_json::Value;
use wasm_bindgen_test::*;

// On a target other then `wasm32-unknown-unknown`, the `#[test]` attribute
// will be used instead, allowing this test to run on any target.
#[wasm_bindgen_test(unsupported = test)]
fn pass() {
    assert_eq!(1, 1);
}

#[wasm_bindgen_test(unsupported = test)]
fn json() {
    let data = r#"
        {
            elements [
                {
                    "word": "error1",
                    "style": "color:red;font-weight:bold;"
                },
                {
                  "word": "error2",
                  "style": "color:red;font-weight:bold;"
                }
            ]


        "#;
    let json: Value = serde_json::from_str(data).unwrap();
    assert_eq!(json["word"], "error1");
}
