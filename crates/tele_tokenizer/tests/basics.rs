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

#[test]
fn dimension_with_unit_and_percentage_token() {
  let mut tokenizer: Tokenizer = "height: 30px; width: 30%;".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn hash_token() {
  let mut tokenizer: Tokenizer = "#foo {}".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn plus_sign_as_delim() {
  let mut tokenizer: Tokenizer = "div + p".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn plus_sign_as_numeric() {
  let mut tokenizer: Tokenizer = "+100".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn hyphen_minus_sign_as_numeric() {
  let mut tokenizer: Tokenizer = "-200".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn hyphen_minus_sign_as_cdc() {
  let mut tokenizer: Tokenizer = "->".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn hyphen_minus_sign_as_ident() {
  let mut tokenizer: Tokenizer = "--theme-color".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn hyphen_minus_sign_as_delim() {
  let mut tokenizer: Tokenizer = " - ".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}
