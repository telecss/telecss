use insta::assert_debug_snapshot;
use tele_folder::{transform, Folder};
use tele_parser::{DeclarationNode, Parser};
use tele_tokenizer::Tokenizer;

struct Renamer;
impl Folder for Renamer {
  fn fold_decl_node(&mut self, mut decl_node: DeclarationNode) -> DeclarationNode {
    decl_node.name = format!("-moz-{}", decl_node.name);
    decl_node
  }
}

#[test]
fn test_fold_decl_node() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red; }  ".into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(transform(parser.parse().unwrap(), Renamer));
}
