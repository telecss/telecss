#![warn(missing_docs)]

//! CSS Tokenizer

use std::{iter::Peekable, str::CharIndices};

mod inner_state;
mod token;

use inner_state::*;
use tele_utils::*;
use token::{Pos, Token, TokenType};

#[derive(Debug)]
pub struct Tokenizer<'s> {
  source: &'s str,
  iter: Peekable<CharIndices<'s>>,
  line: usize,
  colnmu: usize,
  offset: usize,
  pre_cursor: Pos,
  cursor: Pos,
  current_state: State,
  buffer: Vec<char>,
  tokens: Vec<Token>,
}

impl<'s> From<&'s str> for Tokenizer<'s> {
  fn from(source: &'s str) -> Self {
    Tokenizer {
      source,
      iter: source.char_indices().peekable(),
      // position related
      line: 1,
      colnmu: 1,
      offset: 0,
      pre_cursor: Pos::default(),
      cursor: Pos::default(),
      // state machine related
      current_state: State::Initial,
      buffer: Vec::with_capacity(100),
      tokens: Vec::new(),
    }
  }
}

impl<'s> Tokenizer<'s> {
  pub fn tokenize(&mut self) -> &Vec<Token> {
    while let Some(&(offset, c)) = self.iter.peek() {
      self.current_state = match self.current_state {
        State::Initial => match c {
          '\n' => State::NewLine,
          '.' => State::MayBeNumber,
          _ => State::Initial,
        },
        State::NewLine => {
          self.line += 1;
          self.colnmu = 1;
          State::Initial
        }
        State::MayBeNumber => {
          if is_digit(c) {
            State::EOF
          } else {
            self.emit(TokenType::Delim);
            State::Initial
          }
        }
        State::EOF => break,
      };

      self.iter.next();
      self.colnmu += 1;
      self.offset = offset;
    }

    &self.tokens
  }

  fn get_cursor(&self) -> Pos {
    Pos {
      offset: self.offset,
      line: self.line,
      column: self.colnmu,
    }
  }

  fn emit(&mut self, token_type: TokenType) {
    let cur_cursor = self.get_cursor();

    let mut token = Token::default();
    token.token_type = token_type;
    token.start_pos = self.pre_cursor;
    token.end_pos = cur_cursor;
    token.content.append(&mut self.buffer);

    self.pre_cursor = cur_cursor;
    self.tokens.push(token)
  }
}
