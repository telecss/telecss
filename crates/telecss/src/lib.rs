use tele_tokenizer::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_tokens(s: &str) -> JsValue {
  let mut tokenizer: Tokenizer = s.into();
  JsValue::from_serde(tokenizer.tokenize().unwrap()).unwrap()
}
