#![warn(missing_docs)]

//! CSS Tokenizer

use std::{
  any::Any,
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
          c if is_digit(c) => State::Numeric,
          '.' => {
            // https://www.w3.org/TR/css-syntax-3/#starts-with-a-number
            if is_start_a_number(&self.bytes[offset..]) {
              State::Numeric
            } else {
              self.consume(offset, 1, false);
              self.emit(TokenType::Delim);
              State::Initial
            }
          }
          '"' | '\'' => State::String,
          '{' => {
            self.consume(offset, 1, false);
            self.emit(TokenType::LeftCurlyBracket);
            State::Initial
          }
          '}' => {
            self.consume(offset, 1, false);
            self.emit(TokenType::RightCurlyBracket);
            State::Initial
          }
          ')' => {
            self.consume(offset, 1, false);
            self.emit(TokenType::RightParentheses);
            State::Initial
          }
          ':' => {
            self.consume(offset, 1, false);
            self.emit(TokenType::Colon);
            State::Initial
          }
          ';' => {
            self.consume(offset, 1, false);
            self.emit(TokenType::SemiColon);
            State::Initial
          }
          '/' if is_comment_start(&self.bytes[offset..]) => {
            self.consume(offset, 2, false);
            State::Comment
          }
          ',' => {
            self.consume(offset, 1, false);
            self.emit(TokenType::Comma);
            State::Initial
          }
          '#' => {
            let cp1 = char_at(&self.bytes[offset..], 1);

            if is_ident_char(cp1) {
              self.consume(offset, 1, true); // consume #
              self.consume_ident_seq();
              self.emit(TokenType::Hash);
            } else {
              self.consume(offset, 1, false);
              self.emit(TokenType::Delim);
            }
            State::Initial
          }
          '+' => {
            if is_start_a_number(&self.bytes[offset..]) {
              self.consume_a_numeric();
            } else {
              self.consume(offset, 1, false);
              self.emit(TokenType::Delim);
            }
            State::Initial
          }
          '-' => {
            let mut returned_state = State::Initial;
            let cp1 = char_at(&self.bytes[offset..], 1);
            let cp2 = char_at(&self.bytes[offset..], 2);
            if is_start_a_number(&self.bytes[offset..]) {
              self.consume_a_numeric();
            } else if cp1 == '-' && cp2 == '>' {
              self.consume(offset, 3, false);
              self.emit(TokenType::CDC);
            } else if would_start_an_ident_seq(&self.bytes[offset..]) {
              returned_state = State::IdentLike;
            } else {
              self.consume(offset, 1, false);
              self.emit(TokenType::Delim);
            }
            returned_state
          }
          '<' => {
            let cp1 = char_at(&self.bytes[offset..], 1);
            let cp2 = char_at(&self.bytes[offset..], 2);
            let cp3 = char_at(&self.bytes[offset..], 3);

            if cp1 == '!' && cp2 == '-' && cp3 == '-' {
              self.consume(offset, 4, false);
              self.emit(TokenType::CDO);
            } else {
              self.consume(offset, 1, false);
              self.emit(TokenType::Delim);
            }
            State::Initial
          }
          '@' => {
            if would_start_an_ident_seq(&self.bytes[(offset + 1)..]) {
              self.consume(offset, 1, true); // ignore @
              self.consume_ident_seq();
              self.emit(TokenType::AtKeyword);
            } else {
              self.consume(offset, 1, false);
              self.emit(TokenType::Delim);
            }
            State::Initial
          }
          _ => {
            self.consume(offset, 1, false);
            // anything else, return a <delim-token>
            self.emit(TokenType::Delim);
            State::Initial
          }
        },
        State::WhiteSpace => {
          if is_whitespace(c) {
            if is_newline(c) {
              self.line += 1;
              self.colnmu = 1;
            }
            self.consume(offset, 1, false);
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

            let next = char_at(&self.bytes[end_pos.offset..], 0);
            let mut returned_state = State::Initial;

            if ident == "url" && next == '(' {
              self.emit(TokenType::URL);
              // consume '('
              self.consume(end_pos.offset, 1, false);
              self.emit(TokenType::LeftParentheses);

              let (start_pos, end_pos, is_function_token) = self.consume_whitespace_for_url_call();

              if start_pos.offset < end_pos.offset {
                // should emit whitespace token
                self.emit(TokenType::WhiteSpace);
              }
              if is_function_token {
                // change URL token to Function token
                self
                  .tokens
                  .iter_mut()
                  .rev()
                  .skip_while(|ty| {
                    ty.token_type == TokenType::WhiteSpace
                      || ty.token_type == TokenType::LeftParentheses
                  })
                  .next()
                  .map(|token| token.token_type = TokenType::Function);
              } else {
                let ws = self.tokens.pop(); // preserve whitespace token
                self.tokens.pop(); // remove TokenType::URL
                self.tokens.pop(); // remove TokenType::LeftParentheses

                if let Some(ty) = ws {
                  if ty.token_type == TokenType::WhiteSpace {
                    self.tokens.push(ty);
                  }
                }
                returned_state = State::URL;
              }
            } else if next == '(' {
              self.emit(TokenType::Function);
              self.consume(end_pos.offset, 1, false);
              self.emit(TokenType::LeftParentheses);
            } else {
              self.emit(TokenType::Ident);
            }
            returned_state
          } else {
            return Err(Error::from(ErrorKind::InvalidIdentSeq));
          }
        }
        State::URL => {
          // https://www.w3.org/TR/css-syntax-3/#consume-a-url-token
          self.consume_url_token()?;
          State::Initial
        }
        // https://www.w3.org/TR/css-syntax-3/#consume-a-string-token
        State::String => {
          let ending_char = c;
          self.consume(offset, 1, true);
          while let Some(&(offset, &c)) = self.iter.peek() {
            let c = c as char;
            if is_newline(c) {
              // TODO: need testing
              self.emit(TokenType::BadString);
              break;
            } else if c == ending_char {
              self.emit(TokenType::String);
              // ignore ending_char
              self.consume(offset, 1, true);
              break;
            } else {
              self.consume(offset, 1, false);
            }
          }
          State::Initial
        }
        // https://www.w3.org/TR/css-syntax-3/#consume-comment
        State::Comment => {
          if is_comment_end(&self.bytes[offset..]) {
            self.consume(offset, 2, false);
            self.emit(TokenType::Comment);
            State::Initial
          } else {
            self.consume(offset, 1, false);
            State::Comment
          }
        }
        State::Numeric => {
          self.consume_a_numeric();
          State::Initial
        }
        State::EOF => break,
      };
    }

    // EOF
    match self.current_state {
      State::WhiteSpace => {
        self.emit(TokenType::WhiteSpace);
      }
      State::Comment | State::URL => return Err(Error::from(ErrorKind::UnexpectedEOF)),
      _ => {}
    }
    self.emit(TokenType::EOF);

    Ok(&mut self.tokens)
  }

  // https://www.w3.org/TR/css-syntax-3/#consume-numeric-token
  fn consume_a_numeric(&mut self) {
    let (_, end_pos) = self.consume_a_number();
    let next = char_at(&self.bytes[end_pos.offset..], 0);

    if would_start_an_ident_seq(&self.bytes[end_pos.offset..]) {
      // According to the spec: https://www.w3.org/TR/css-syntax-3/#consume-numeric-token,
      // we need to continue to analyze the unit of the Die token,
      // but we can also choose to analyze the unit as a separate `Ident` token
      // e.g.
      // `30px` -> TokenType::Dimension + TokenType::Ident
      //                    (30)                   (px)
      self.emit(TokenType::Dimension);
    } else if next == '%' {
      self.emit(TokenType::Percentage);
      self.consume(end_pos.offset, 1, true);
    } else {
      self.emit(TokenType::Number);
    }
  }

  // https://www.w3.org/TR/css-syntax-3/#consume-a-number
  fn consume_a_number(&mut self) -> (Pos, Pos) {
    let start_pos = self.get_cursor();
    match self.iter.peek() {
      Some(&(offset, &b'+')) | Some(&(offset, &b'-')) => self.consume(offset, 1, false),
      _ => {}
    }

    self.try_consume_digit();

    if let Some(&(offset, _)) = self.iter.peek() {
      let cp1 = char_at(&self.bytes[offset..], 0);
      let cp2 = char_at(&self.bytes[offset..], 1);

      if cp1 == '.' && is_digit(cp2) {
        self.consume(offset, 2, false);
        self.try_consume_digit();
      }
    }

    if let Some(&(offset, _)) = self.iter.peek() {
      let cp1 = char_at(&self.bytes[offset..], 0);
      let cp2 = char_at(&self.bytes[offset..], 1);
      let cp3 = char_at(&self.bytes[offset..], 2);

      if cp1 == 'E' || cp1 == 'e' {
        if (cp2 == '+' || cp2 == '-') && is_digit(cp3) {
          self.consume(offset, 3, false);
          self.try_consume_digit();
        } else if is_digit(cp2) {
          self.consume(offset, 2, false);
          self.try_consume_digit();
        }
      }
    }
    let end_pos = self.get_cursor();

    (start_pos, end_pos)
  }

  fn try_consume_digit(&mut self) {
    while let Some(&(offset, &c)) = self.iter.peek() {
      if is_digit(c as char) {
        self.consume(offset, 1, false);
      } else {
        break;
      }
    }
  }

  fn consume_ident_seq(&mut self) -> (Pos, Pos) {
    let start_cursor = self.get_cursor();
    while let Some(&(offset, &c)) = self.iter.peek() {
      if is_ident_char(c as char) {
        self.consume(offset, 1, false);
      } else {
        break;
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
        self.consume(offset, 1, false);
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

  fn consume_url_token(&mut self) -> Result<()> {
    while let Some(&(offset, &c)) = self.iter.peek() {
      let c = c as char;
      match c {
        ')' => {
          self.emit(TokenType::URL);
          break;
        }
        '"' | '\'' | '(' => return Err(Error::from(ErrorKind::BadURL)),
        c if is_non_printable(c) => return Err(Error::from(ErrorKind::BadURL)),
        c if is_whitespace(c) => {
          self.emit(TokenType::URL);
          let (start_pos, end_pos, _) = self.consume_whitespace_for_url_call();
          let next = char_at(&self.bytes[end_pos.offset..], 0);
          if next != ')' {
            self.tokens.pop();
            return Err(Error::from(ErrorKind::BadURL));
          } else {
            if start_pos.offset != end_pos.offset {
              self.emit(TokenType::WhiteSpace);
            }
            self.consume(end_pos.offset, 1, true);
            break;
          }
        }
        _ => self.consume(offset, 1, false),
      }
    }
    Ok(())
  }

  fn consume(&mut self, offset: usize, count: usize, advance_cursor: bool) {
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
      offset: if self.colnmu == 1 { 0 } else { self.offset + 1 },
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
