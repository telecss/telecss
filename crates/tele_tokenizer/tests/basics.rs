use insta::assert_debug_snapshot;
use tele_tokenizer::*;

#[test]
fn it_works() {
  let mut tokenizer: Tokenizer = r".foo { color: red; }".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn comments() {
  let mut tokenizer: Tokenizer = r" /** I'm Comment * */ ".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn comma() {
  let mut tokenizer: Tokenizer = r".foo, .bar {}".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn unexpected_eof_comments() {
  let mut tokenizer: Tokenizer = r" /** I'm Comment *".into();
  let res = tokenizer.tokenize();
  assert_eq!(res, Err(Error::from(ErrorKind::UnexpectedEOF)));

  let mut tokenizer: Tokenizer = r" /** I'm Comment".into();
  let res = tokenizer.tokenize();
  assert_eq!(res, Err(Error::from(ErrorKind::UnexpectedEOF)));
}

#[test]
fn url_function_quotation_mark() {
  let mut tokenizer: Tokenizer = r#"url("foo.svg");"#.into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn url_function_apostrophe() {
  let mut tokenizer: Tokenizer = r"url('foo.svg');".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn url_function_with_whitespace() {
  let mut tokenizer: Tokenizer = r"url(   'foo.svg'  );".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn url_token() {
  let mut tokenizer: Tokenizer = r"url(   foo.svg  );".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn bad_url_token() {
  let mut tokenizer: Tokenizer = r"url(   foo.svg  ".into();
  assert_eq!(tokenizer.tokenize(), Err(Error::from(ErrorKind::BadURL)));

  let mut tokenizer: Tokenizer = r"url(   foo.svg  .bar".into();
  assert_eq!(tokenizer.tokenize(), Err(Error::from(ErrorKind::BadURL)));
}

#[test]
fn string_case() {
  let mut tokenizer: Tokenizer = r"color: '\''".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn number_token() {
  let mut tokenizer: Tokenizer = r"2 .3 4.4 +5 -6 7E7 8e8 9E+9 9e-9".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn dimension_with_unit_and_percentage_token() {
  let mut tokenizer: Tokenizer = r"height: 30px; width: 30%;".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn hash_token() {
  let mut tokenizer: Tokenizer = r"#foo {}".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn plus_sign_as_delim() {
  let mut tokenizer: Tokenizer = r"div + p".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn plus_sign_as_numeric() {
  let mut tokenizer: Tokenizer = r"+100".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn hyphen_minus_sign_as_numeric() {
  let mut tokenizer: Tokenizer = r"-200".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn hyphen_minus_sign_as_cdc() {
  let mut tokenizer: Tokenizer = r"-->".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn hyphen_minus_sign_as_ident() {
  let mut tokenizer: Tokenizer = r"--theme-color".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn hyphen_minus_sign_as_delim() {
  let mut tokenizer: Tokenizer = r" - ".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn less_than_sign_as_cdo() {
  let mut tokenizer: Tokenizer = r"<!--".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn less_than_sign_as_delim() {
  let mut tokenizer: Tokenizer = r" < ".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn at_keyword() {
  let mut tokenizer: Tokenizer = r"@media screen".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}

#[test]
fn at_keyword_as_delim() {
  let mut tokenizer: Tokenizer = r" @ ".into();
  assert_debug_snapshot!(tokenizer.tokenize());
}
