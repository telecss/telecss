#![warn(missing_docs)]

//! CSS Tokenizer

mod token;

use std::{iter::Peekable, str::CharIndices};

use token::*;

#[derive(Debug)]
pub struct Tokenizer<'s> {
  source: &'s str,
  iter: Peekable<CharIndices<'s>>,
  start_pos: Pos,
  end_pos: Pos,
  current_state: State,
}

impl<'s> From<&'s str> for Tokenizer<'s> {
  fn from(source: &'s str) -> Self {
    Tokenizer {
      source,
      iter: source.char_indices().peekable(),
      start_pos: Pos::default(),
      end_pos: Pos::default(),
      current_state: State::default(),
    }
  }
}

impl<'a> Tokenizer<'a> {
  pub fn tokenize(&mut self) -> Vec<token::Token> {
    while let Some((offset, c)) = self.iter.peek() {
      self.current_state = match self.current_state {
        State::Initial => match c {
          '.' => State::FullStop,
          _ => State::QuotationMark,
        },
        State::FullStop => State::Initial,
        State::QuotationMark => State::Apostrophe,
        State::Apostrophe => State::EOF,
        State::EOF => break,
      };
      self.iter.next();
    }

    vec![]
  }
}

#[derive(Debug)]
enum State {
  Initial,
  FullStop,
  QuotationMark,
  Apostrophe,
  EOF,
}

impl Default for State {
  fn default() -> Self {
    Self::Initial
  }
}
