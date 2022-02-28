use tele_parser::*;
use tele_tokenizer::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokenize(s: &str) -> JsValue {
  let mut tokenizer: Tokenizer = s.into();
  JsValue::from_serde(tokenizer.tokenize().unwrap()).unwrap()
}

#[wasm_bindgen]
pub fn parse(s: &str) -> JsValue {
  let mut tokenizer: Tokenizer = s.into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  let ast = parser.parse().unwrap();
  JsValue::from_serde(&*ast).unwrap()
}
