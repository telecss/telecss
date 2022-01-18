#![warn(missing_docs)]

//! CSS Tokenizer

use std::{
  iter::{Enumerate, Peekable},
  slice::Iter,
  str::from_utf8,
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
              self.advance(offset, 1, false);
              self.emit(TokenType::Delim);
              State::Initial
            }
          }
          '"' | '\'' => State::String,
          '{' => {
            self.advance(offset, 1, false);
            self.emit(TokenType::LeftCurlyBracket);
            State::Initial
          }
          '}' => {
            self.advance(offset, 1, false);
            self.emit(TokenType::RightCurlyBracket);
            State::Initial
          }
          ')' => {
            self.advance(offset, 1, false);
            self.emit(TokenType::RightParentheses);
            State::Initial
          }
          ':' => {
            self.advance(offset, 1, false);
            self.emit(TokenType::Colon);
            State::Initial
          }
          ';' => {
            self.advance(offset, 1, false);
            self.emit(TokenType::SemiColon);
            State::Initial
          }
          '/' if is_comment_start(&self.bytes[offset..]) => {
            self.advance(offset, 2, false);
            State::Comment
          }
          ',' => {
            self.advance(offset, 1, false);
            self.emit(TokenType::Comma);
            State::Initial
          }
          _ => {
            self.advance(offset, 1, false);
            State::Initial
          }
        },
        State::WhiteSpace => {
          if is_whitespace(c) {
            if is_newline(c) {
              self.line += 1;
              self.colnmu = 1;
            }
            self.advance(offset, 1, false);
            State::WhiteSpace
          } else {
            self.emit(TokenType::WhiteSpace);
            State::Initial
          }
        }
        // https://www.w3.org/TR/css-syntax-3/#consume-an-ident-like-token
        State::IdentLike => {
          if would_start_an_ident_seq(&self.bytes[offset..]) {
            let (start_pos, end_pos) = self.consume_ident_seq();
            let ident = from_utf8(&self.bytes[start_pos.offset..end_pos.offset])
              .unwrap_or("")
              .to_ascii_lowercase();

            let next = self.bytes[end_pos.offset] as char;

            if ident == "url" && next == '(' {
              // TODO: temporarily as a URL token
              self.emit(TokenType::URL);
              self.advance(end_pos.offset, 1, false);
              self.emit(TokenType::LeftParentheses);

              let (start_pos, end_pos, is_function_token) = self.consume_whitespace_for_url_call();
              if is_function_token {
                self
                  .tokens
                  .iter_mut()
                  .rev()
                  .skip(1)
                  .next()
                  .map(|token| token.token_type = TokenType::Function);
              } else {
                // TODO: https://www.w3.org/TR/css-syntax-3/#consume-a-url-token
              }
              if start_pos.offset < end_pos.offset {
                // should emit whitespace token
                self.emit(TokenType::WhiteSpace);
              }
            } else if next == '(' {
              self.emit(TokenType::Function);
              self.advance(end_pos.offset, 1, false);
              self.emit(TokenType::LeftParentheses);
            } else {
              self.emit(TokenType::Ident);
            }
            State::Initial
          } else {
            return Err(Error::from(ErrorKind::InvalidIdentSeq));
          }
        }
        // https://www.w3.org/TR/css-syntax-3/#consume-a-string-token
        State::String => {
          let ending_char = c;
          self.advance(offset, 1, true);
          while let Some(&(offset, &c)) = self.iter.peek() {
            let c = c as char;
            if is_newline(c) {
              self.emit(TokenType::BadString);
              break;
            } else if c == ending_char {
              self.emit(TokenType::String);
              // ignore ending_char
              self.advance(offset, 1, true);
              break;
            } else {
              self.advance(offset, 1, false);
            }
          }
          State::Initial
        }
        // https://www.w3.org/TR/css-syntax-3/#consume-comment
        State::Comment => {
          if is_comment_end(&self.bytes[offset..]) {
            self.advance(offset, 2, false);
            self.emit(TokenType::Comment);
            State::Initial
          } else {
            self.advance(offset, 1, false);
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
        self.advance(offset, 1, false);
      } else {
        let cp1 = char_at(&self.bytes, 0);
        let cp2 = char_at(&self.bytes, 1);
        if is_valid_escape(cp1, cp2) {
          self.advance(offset, 2, false);
        } else {
          break;
        }
      }
    }
    let end_cursor = self.get_cursor();
    (start_cursor, end_cursor)
  }

  fn consume_whitespace_for_url_call(&mut self) -> (Pos, Pos, bool) {
    let start_cursor = self.get_cursor();
    let mut is_function_token = false;
    while let Some(&(offset, &c)) = self.iter.peek() {
      let c = c as char;
      if is_whitespace(c) {
        self.advance(offset, 1, false);
      } else if c == '"' || c == '\'' {
        is_function_token = true;
        break;
      } else {
        break;
      }
    }
    let end_cursor = self.get_cursor();
    (start_cursor, end_cursor, is_function_token)
  }

  fn advance(&mut self, offset: usize, count: usize, advance_cursor: bool) {
    for n in 0..count {
      self.iter.next();
      self.colnmu += 1;
      self.offset = offset + n;
    }
    if advance_cursor {
      self.pre_cursor = self.get_cursor();
    }
  }

  fn get_cursor(&self) -> Pos {
    Pos {
      offset: self.offset + 1,
      line: self.line,
      column: self.colnmu,
    }
  }

  fn emit(&mut self, token_type: TokenType) {
    let cur_cursor = self.get_cursor();

    let token = Token::new(
      token_type,
      self.pre_cursor,
      cur_cursor,
      &self.bytes[self.pre_cursor.offset..cur_cursor.offset],
    );

    self.pre_cursor = cur_cursor;
    self.tokens.push(token);
  }
}
