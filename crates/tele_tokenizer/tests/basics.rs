use insta::assert_json_snapshot;
use tele_tokenizer::*;

#[test]
fn it_works() {
  let mut tokenizer: Tokenizer = ".foo { color: red; }".into();
  assert_json_snapshot!(tokenizer.tokenize());
}

#[test]
fn comments() {
  let mut tokenizer: Tokenizer = " /** I'm Comment * */ ".into();
  assert_json_snapshot!(tokenizer.tokenize());
}

#[test]
fn comma() {
  let mut tokenizer: Tokenizer = ".foo, .bar {}".into();
  assert_json_snapshot!(tokenizer.tokenize());
}

#[test]
fn unexpected_eof_comments() {
  let mut tokenizer: Tokenizer = " /** I'm Comment *".into();
  let res = tokenizer.tokenize();
  assert_eq!(res, Err(Error::from(ErrorKind::UnexpectedEOF)));

  let mut tokenizer: Tokenizer = " /** I'm Comment".into();
  let res = tokenizer.tokenize();
  assert_eq!(res, Err(Error::from(ErrorKind::UnexpectedEOF)));
}
