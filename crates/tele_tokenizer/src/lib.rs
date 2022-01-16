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
          c if is_ident_start(c) => State::IdentLike,
          '.' => {
            // https://www.w3.org/TR/css-syntax-3/#starts-with-a-number
            if is_start_a_number(&self.bytes[offset..]) {
              State::Numeric
            } else {
              self.advance(offset, 1);
              self.emit(TokenType::Delim);
              State::Initial
            }
          }
          '{' => {
            self.advance(offset, 1);
            self.emit(TokenType::LeftCurlyBracket);
            State::Initial
          }
          '}' => {
            self.advance(offset, 1);
            self.emit(TokenType::RightCurlyBracket);
            State::Initial
          }
          ':' => {
            self.advance(offset, 1);
            self.emit(TokenType::Colon);
            State::Initial
          }
          ';' => {
            self.advance(offset, 1);
            self.emit(TokenType::SemiColon);
            State::Initial
          }
          '/' if is_comment_start(&self.bytes[offset..]) => {
            self.advance(offset, 2);
            State::Comment
          }
          ',' => {
            self.advance(offset, 1);
            self.emit(TokenType::Comma);
            State::Initial
          }
          _ => {
            self.advance(offset, 1);
            State::Initial
          }
        },
        State::WhiteSpace => {
          if is_whitespace(c) {
            if is_newline(c) {
              self.line += 1;
              self.colnmu = 1;
            }
            self.advance(offset, 1);
            State::WhiteSpace
          } else {
            self.emit(TokenType::WhiteSpace);
            State::Initial
          }
        }
        // https://www.w3.org/TR/css-syntax-3/#consume-an-ident-like-token
        State::IdentLike => {
          if would_start_an_ident_seq(&self.bytes[offset..]) {
            self.consume_ident_seq();
            // TODO: ident-like analysis
            self.emit(TokenType::Ident);
            State::Initial
          } else {
            return Err(Error::from(ErrorKind::InvalidIdentSeq));
          }
        }
        // https://www.w3.org/TR/css-syntax-3/#consume-comment
        State::Comment => {
          if is_comment_end(&self.bytes[offset..]) {
            self.advance(offset, 2);
            self.emit(TokenType::Comment);
            State::Initial
          } else {
            self.advance(offset, 1);
            State::Comment
          }
        }
        State::Numeric => {
          // TODO
          State::EOF
        }
        State::EOF => break,
      };
    }

    // EOF
    match self.current_state {
      State::WhiteSpace => {
        self.emit(TokenType::WhiteSpace);
      }
      State::Comment => return Err(Error::from(ErrorKind::UnexpectedEOF)),
      _ => {}
    }
    self.emit(TokenType::EOF);

    Ok(&mut self.tokens)
  }

  fn consume_ident_seq(&mut self) -> (Pos, Pos) {
    let start_cursor = self.get_cursor();
    while let Some(&(offset, &c)) = self.iter.peek() {
      if is_ident_char(c as char) {
        self.advance(offset, 1);
      } else {
        let cp1 = *self.bytes.get(offset).unwrap_or(&b'\0') as char;
        let cp2 = *self.bytes.get(offset).unwrap_or(&b'\0') as char;
        if is_valid_escape(cp1, cp2) {
          self.advance(offset, 2);
        } else {
          break;
        }
      }
    }
    let end_cursor = self.get_cursor();
    (start_cursor, end_cursor)
  }

  fn advance(&mut self, offset: usize, count: usize) {
    for n in 0..count {
      self.iter.next();
      self.colnmu += 1;
      self.offset = offset + n;
    }
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
