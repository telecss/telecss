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

#[test]
fn test_visit_decl_node() {
  // tokenizing
  let mut tokenizer: Tokenizer = r"  .foo { color: red; }  ".into();
  let tokens = tokenizer.tokenize().unwrap();
  // parsing
  let parser = Parser::from(tokens);
  let ast = Rc::new(RefCell::new(parser.parse().unwrap()));
  // transforming
  let transformer = VisitMut::new(Rc::clone(&ast), vec![Renamer]);
  transformer.transform();

  assert_debug_snapshot!(ast);
}
