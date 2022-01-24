#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

//! CSS Parser

use tele_tokenizer::*;

pub struct Parser<'s> {
  tokens: &'s Vec<Token<'s>>,
}

impl<'s> Parser<'s> {
  fn parse(tokens: &'s Vec<Token<'s>>) {
    println!("{:#?}", tokens);
  }
}
