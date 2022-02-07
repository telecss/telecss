use insta::assert_debug_snapshot;
use tele_parser::Parser;
use tele_tokenizer::Tokenizer;

#[test]
fn ruleset_with_a_single_decl() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red; }  ".into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(parser.parse());
}

#[test]
fn ruleset_with_a_single_decl_no_trailing_semicolon() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red }  ".into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(parser.parse());
}

#[test]
fn ruleset_with_multi_decls() {
  let mut tokenizer: Tokenizer = r"
    .foo { color: red;
    background: url('bar.png')
  }  "
    .into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(parser.parse());
}

#[test]
fn decl_with_important() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red !important; }  ".into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(parser.parse());
}

#[test]
fn skip_comments() {
  let mut tokenizer: Tokenizer = r"
    /* comment 1 */ .foo /* comment 2 */ {
    /* comment 3 */ color /* comment 4 */ :
    /* comment 5 */ red /* comment 6 */ !important;
    /* comment 7 */ } /* comment 8 */
  "
  .into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(parser.parse());
}

#[test]
fn percentage_token() {
  let mut tokenizer: Tokenizer = r"
    .foo {
      height: 100%;
    }
  "
  .into();
  let parser = Parser::from(tokenizer.tokenize().unwrap());
  assert_debug_snapshot!(parser.parse());
}
