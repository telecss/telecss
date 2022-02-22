use insta::assert_debug_snapshot;
use std::{cell::RefCell, rc::Rc};
use tele_parser::{DeclarationNode, Parser};
use tele_tokenizer::Tokenizer;
use tele_visit::{MutVisitor, VisitMut};

struct Renamer;
impl<'s> MutVisitor<'s> for Renamer {
  fn visit_decl_node(&self, decl_node: Rc<RefCell<DeclarationNode<'s>>>) {
    let mut decl_node = decl_node.borrow_mut();
    decl_node.name = format!("-moz-{}", decl_node.name);
  }
}

struct ValueConverter;
impl<'s> MutVisitor<'s> for ValueConverter {
  fn visit_decl_node(&self, decl_node: Rc<RefCell<DeclarationNode<'s>>>) {
    let mut decl_node = decl_node.borrow_mut();
    if decl_node.value == "red" {
      decl_node.value = "green".to_owned();
    }
  }
}

#[test]
fn test_visit_decl_node() {
  // tokenizing
  let mut tokenizer: Tokenizer = r"  .foo { color: red; }  ".into();
  let tokens = tokenizer.tokenize().unwrap();
  // parsing
  let parser = Parser::from(tokens);
  let ast = parser.parse().unwrap();
  // transforming
  let transformer = VisitMut::new(Rc::clone(&ast), vec![Box::new(Renamer)]);
  transformer.transform();

  assert_debug_snapshot!(ast);
}

#[test]
fn test_multi_visitors() {
  // tokenizing
  let mut tokenizer: Tokenizer = r"  .foo { color: red; }  ".into();
  let tokens = tokenizer.tokenize().unwrap();
  // parsing
  let parser = Parser::from(tokens);
  let ast = parser.parse().unwrap();
  // transforming
  let transformer = VisitMut::new(
    Rc::clone(&ast),
    vec![Box::new(Renamer), Box::new(ValueConverter)],
  );
  transformer.transform();

  assert_debug_snapshot!(ast);
}
