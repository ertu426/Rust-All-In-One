use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum MyError {
    SomeError(String),
}

impl From<String> for MyError {
    fn from(s: String) -> Self {
        MyError::SomeError(s)
    }
}

impl From<wasm_bindgen::JsValue> for MyError {
    fn from(js_value: wasm_bindgen::JsValue) -> Self {
        MyError::SomeError(js_value.as_string().unwrap())
    }
}