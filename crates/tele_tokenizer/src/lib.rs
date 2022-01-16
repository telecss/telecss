#![warn(missing_docs)]

//! CSS Tokenizer

use std::{
  iter::{Enumerate, Peekable},
  slice::Iter,
};

mod error;
mod inner_state;
mod token;

use inner_state::*;
use tele_utils::*;
use token::{Pos, Token, TokenType};

pub use error::*;
pub use token::*;

#[derive(Debug)]
pub struct Tokenizer<'s> {
  bytes: &'s [u8],
  iter: Peekable<Enumerate<Iter<'s, u8>>>,
  line: usize,
  colnmu: usize,
  offset: usize,
  pre_cursor: Pos,
  current_state: State,
  tokens: Vec<Token<'s>>,
}

impl<'s> From<&'s str> for Tokenizer<'s> {
  fn from(source: &'s str) -> Self {
    Tokenizer {
      bytes: source.as_bytes(),
      iter: source.as_bytes().iter().enumerate().peekable(),
      // position related
      line: 1,
      colnmu: 1,
      offset: 0,
      pre_cursor: Pos::default(),
      current_state: State::Initial,
      tokens: Vec::new(),
    }
  }
}

impl<'s> Tokenizer<'s> {
  pub fn tokenize(&mut self) -> Result<&mut Vec<Token<'s>>> {
    while let Some(&(offset, &c)) = self.iter.peek() {
      let c = c as char;
      self.current_state = match self.current_state {
        State::Initial => match c {
          c if is_whitespace(c) => State::WhiteSpace,
          c if is_ident_start(c) => State::IdentStart,
          '.' => State::MayBeNumber,
          '{' => {
            self.consume(offset);
            self.emit(TokenType::LeftCurlyBracket);
            State::Initial
          }
          '}' => {
            self.consume(offset);
            self.emit(TokenType::RightCurlyBracket);
            State::Initial
          }
          ':' => {
            self.consume(offset);
            self.emit(TokenType::Colon);
            State::Initial
          }
          ';' => {
            self.consume(offset);
            self.emit(TokenType::SemiColon);
            State::Initial
          }
          '/' => State::Solidus,
          ',' => {
            self.consume(offset);
            self.emit(TokenType::Comma);
            State::Initial
          }
          _ => {
            self.consume(offset);
            State::Initial
          }
        },
        State::WhiteSpace => {
          if is_whitespace(c) {
            if is_newline(c) {
              self.line += 1;
              self.colnmu = 1;
            }
            self.consume(offset);
            State::WhiteSpace
          } else {
            self.emit(TokenType::WhiteSpace);
            State::Initial
          }
        }
        State::IdentStart => {
          self.consume(offset);
          State::Ident
        }
        State::Ident => {
          if is_ident_char(c) {
            self.consume(offset);
            State::Ident
          } else {
            self.emit(TokenType::Ident);
            State::Initial
          }
        }
        State::Solidus => {
          self.consume(offset);
          State::MayBeComment
        }
        State::MayBeComment => {
          if c == '*' {
            self.consume(offset);
            State::Comment
          } else {
            // error
            State::Initial
          }
        }
        State::Comment => {
          self.consume(offset);
          if c == '*' {
            State::MayBeEndOfComment
          } else {
            State::Comment
          }
        }
        State::MayBeEndOfComment => {
          self.consume(offset);
          if c == '/' {
            self.emit(TokenType::Comment);
            State::Initial
          } else {
            State::Comment
          }
        }
        State::MayBeNumber => {
          self.consume(offset);
          if is_digit(c) {
            State::Number
          } else {
            self.emit(TokenType::Delim);
            State::Initial
          }
        }
        State::Number => {
          if is_digit(c) {
            self.consume(offset);
            State::Number
          } else {
            State::Initial
          }
        }
        State::EOF => break,
      };
    }

    // EOF
    match self.current_state {
      State::WhiteSpace => {
        self.emit(TokenType::WhiteSpace);
      }
      State::Comment | State::MayBeEndOfComment => {
        return Err(Error::from(ErrorKind::UnexpectedEOF))
      }
      _ => {}
    }
    self.emit(TokenType::EOF);

    Ok(&mut self.tokens)
  }

  fn consume(&mut self, offset: usize) {
    self.iter.next();
    self.colnmu += 1;
    self.offset = offset;
  }

  fn get_cursor(&self) -> Pos {
    Pos {
      offset: self.offset,
      line: self.line,
      column: self.colnmu,
    }
  }

  fn emit(&mut self, token_type: TokenType) {
    let mut cur_cursor = self.get_cursor();
    cur_cursor.offset += 1;

    let token = Token::new(
      token_type,
      self.pre_cursor,
      cur_cursor,
      &self.bytes[self.pre_cursor.offset..cur_cursor.offset],
    );

    self.pre_cursor = cur_cursor;
    self.tokens.push(token)
  }
}
