use insta::assert_json_snapshot;
use tele_tokenizer::*;

#[test]
fn it_works() {
  let mut tokenizer: Tokenizer = ".foo { color: red; }".into();

  assert_json_snapshot!(tokenizer.tokenize());
}
