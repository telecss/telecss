use tele_tokenizer::Tokenizer;

#[test]
fn it_works() {
  let mut tokenizer: Tokenizer = ".foo { color: red; }".into();

  assert_eq!(tokenizer.tokenize(), vec![]);
}
