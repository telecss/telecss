use insta::assert_debug_snapshot;
use tele_tokenizer::*;

#[test]
fn it_works() {
  let mut tokenizer: Tokenizer = ".foo { color: red; }".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn comments() {
  let mut tokenizer: Tokenizer = " /** I'm Comment * */ ".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn comma() {
  let mut tokenizer: Tokenizer = ".foo, .bar {}".into();
  assert_debug_snapshot!(tokenizer.tokenize());
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

#[test]
fn url_function_quotation_mark() {
  let mut tokenizer: Tokenizer = "url(\"foo.svg\");".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn url_function_apostrophe() {
  let mut tokenizer: Tokenizer = "url('foo.svg');".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn url_function_with_whitespace() {
  let mut tokenizer: Tokenizer = "url(   'foo.svg'  );".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn url_token() {
  let mut tokenizer: Tokenizer = "url(   foo.svg  );".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn bad_url_token() {
  let mut tokenizer: Tokenizer = "url(   foo.svg  ".into();
  assert_eq!(tokenizer.tokenize(), Err(Error::from(ErrorKind::BadURL)));

  let mut tokenizer: Tokenizer = "url(   foo.svg  .bar".into();
  assert_eq!(tokenizer.tokenize(), Err(Error::from(ErrorKind::BadURL)));
}

#[test]
fn number_token() {
  let mut tokenizer: Tokenizer = "2 .3 4.4 +5 -6 7E7 8e8 9E+9 9e-9".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}
