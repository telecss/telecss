#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

//! CSS Parser

use std::{
  cell::RefCell,
  iter::{Enumerate, Peekable},
  slice::Iter,
};

use tele_tokenizer::*;

pub struct Parser<'s> {
  tokens: &'s Vec<Token<'s>>,
  iter: RefCell<Peekable<Enumerate<Iter<'s, Token<'s>>>>>,
}

impl<'s> From<&'s Vec<Token<'s>>> for Parser<'s> {
  fn from(tokens: &'s Vec<Token<'s>>) -> Self {
    Self {
      tokens,
      iter: RefCell::new(tokens.iter().enumerate().peekable()),
    }
  }
}

impl<'s> Parser<'s> {
  pub fn parse(&self) {
    let mut iter = self.iter.borrow_mut();
    while let Some(&(offset, _token)) = iter.peek() {
      println!("offset: {}", offset);
      println!("Token: {:#?}", self.tokens[offset]);

      iter.next();
    }
  }
}
