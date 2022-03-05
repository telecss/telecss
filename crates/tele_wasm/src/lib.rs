use std::{cell::RefCell, rc::Rc};
use tele_codegen::*;
use tele_parser::*;
use tele_tokenizer::*;
use tele_visit::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokenize(s: &str) -> JsValue {
  let mut tokenizer: Tokenizer = s.into();
  JsValue::from_serde(tokenizer.tokenize().unwrap()).unwrap()
}

struct VBind;
impl<'s> MutVisitor<'s> for VBind {
  fn visit_fn_node(&self, fn_node: Rc<RefCell<FunctionNode>>) {
    let mut fn_node = fn_node.borrow_mut();

    if fn_node.name == "v-bind" {
      fn_node.name = "var".to_owned();

      if let Some(Value::Ident(ident)) = fn_node.children.iter().next() {
        let mut ident = ident.borrow_mut();
        ident.name = format!("--calchash-{}", ident.name);
      }
    }
  }
}

#[wasm_bindgen]
pub fn parse(s: &str) -> String {
  let mut tokenizer: Tokenizer = s.into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  let ast = parser.parse().unwrap();

  let transformer = VisitMut::new(Rc::clone(&ast), vec![Box::new(VBind)]);
  transformer.transform();

  // codegen
  let mut css = String::new();
  let mut codegen = Codegen::new(&mut css);
  codegen.generate(ast);

  css
}
