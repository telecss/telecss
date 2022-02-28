#![allow(unused_must_use)]

use insta::assert_snapshot;
use std::{cell::RefCell, rc::Rc};
use tele_codegen::{CodeGenerator, Codegen};
use tele_parser::*;
use tele_tokenizer::*;
use tele_visit::*;

struct Renamer;
impl<'s> MutVisitor<'s> for Renamer {
  fn visit_decl_node(&self, decl_node: Rc<RefCell<DeclarationNode<'s>>>) {
    let mut decl_node = decl_node.borrow_mut();
    decl_node.name = format!("-moz-{}", decl_node.name);
  }
}

#[test]
fn codegen_for_ruleset() {
  // tokenizing
  let mut tokenizer: Tokenizer = r"  .foo { color: red; }  ".into();
  let tokens = tokenizer.tokenize().unwrap();
  // parsing
  let parser = Parser::from(tokens);
  let ast = parser.parse().unwrap();
  // transforming
  let transformer = VisitMut::new(Rc::clone(&ast), vec![Box::new(Renamer)]);
  transformer.transform();

  // codegen
  let mut css = String::new();
  let mut codegen = Codegen::new(&mut css);
  codegen.generate(ast);

  assert_snapshot!(&css);
}

#[test]
fn codegen_for_at_rule() {
  // tokenizing
  let mut tokenizer: Tokenizer = r"
  @media screen and (min-width: 900px) { article { padding: 1rem 3rem; } }"
    .into();
  let tokens = tokenizer.tokenize().unwrap();
  // parsing
  let parser = Parser::from(tokens);
  let ast = parser.parse().unwrap();
  // transforming
  let transformer = VisitMut::new(Rc::clone(&ast), vec![Box::new(Renamer)]);
  transformer.transform();

  // codegen
  let mut css = String::new();
  let mut codegen = Codegen::new(&mut css);
  codegen.generate(ast);

  assert_snapshot!(&css);
}
