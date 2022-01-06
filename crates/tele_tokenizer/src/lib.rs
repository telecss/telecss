#![warn(missing_docs)]

//! CSS Tokenizer

mod token;

#[derive(Debug)]
pub struct Tokenizer;

impl Tokenizer {
  pub fn tokenize(css: &str) -> Vec<token::Token> {
    vec![]
  }
}
