#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

//! CSS Parser

use tele_tokenizer::*;

pub struct Parser<'s> {
  tokens: &'s Vec<Token<'s>>,
}

impl<'s> From<&'s Vec<Token<'s>>> for Parser<'s> {
  fn from(tokens: &'s Vec<Token<'s>>) -> Self {
    Self { tokens }
  }
}

impl<'s> Parser<'s> {
  pub fn parse(&self) {
    println!("{:#?}", self.tokens);
  }
}
