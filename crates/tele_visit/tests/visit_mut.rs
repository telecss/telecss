use insta::assert_debug_snapshot;
use tele_parser::{DeclarationNode, Parser};
use tele_tokenizer::Tokenizer;
use tele_visit::{transform, VisitMut};

struct Renamer;
impl<'s> VisitMut<'s> for Renamer {
  fn visit_decl_node(&mut self, mut decl_node: DeclarationNode<'s>) -> DeclarationNode<'s> {
    decl_node.name = format!("-moz-{}", decl_node.name);
    decl_node
  }
}

#[test]
fn test_visit_decl_node() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red; }  ".into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(transform(parser.parse().unwrap(), Renamer));
}
