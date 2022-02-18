use insta::assert_debug_snapshot;
use tele_folder::Folder;
use tele_parser::{DeclarationNode, Parser, StyleSheetNode};
use tele_tokenizer::Tokenizer;

struct Renamer;
impl Folder for Renamer {
  fn fold_decl_node(&mut self, mut decl_node: DeclarationNode) -> DeclarationNode {
    decl_node.name = format!("-moz-{}", decl_node.name);
    decl_node
  }
}

fn transform<T: Folder>(ast: StyleSheetNode, mut folder: T) -> StyleSheetNode {
  folder.fold_ss_node(ast)
}

#[test]
fn test_fold_decl_node() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red; }  ".into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(transform(parser.parse().unwrap(), Renamer));
}
