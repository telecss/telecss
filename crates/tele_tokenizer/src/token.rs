use serde::Serialize;
use std::fmt::{Debug, Display, Formatter, Result};
use std::str::from_utf8;

/// The CSS TokenType
#[derive(Debug, PartialEq, Serialize)]
pub enum TokenType {
  Ident,
  Function,
  AtKeyword,
  Hash,
  String,
  BadString,
  URL,
  BadURL,
  Delim,
  Number,
  Percentage,
  Dimension,
  WhiteSpace,
  CDO,
  CDC,
  Colon,
  SemiColon,
  Comma,
  LeftSquareBracket,
  RightSquareBracket,
  LeftParentheses,
  RightParentheses,
  LeftCurlyBracket,
  RightCurlyBracket,
  // custom
  Comment,
  EOF,
}

impl Default for TokenType {
  fn default() -> Self {
    Self::EOF
  }
}

impl Display for TokenType {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      &TokenType::Ident => write!(f, "<ident-token>"),
      &TokenType::Function => write!(f, "<function-token>"),
      &TokenType::AtKeyword => write!(f, "<at-keyword-token>"),
      &TokenType::Hash => write!(f, "<hash-token>"),
      &TokenType::String => write!(f, "<string-token>"),
      &TokenType::BadString => write!(f, "<bad-string-token>"),
      &TokenType::URL => write!(f, "<url-token>"),
      &TokenType::BadURL => write!(f, "<bad-url-token>"),
      &TokenType::Delim => write!(f, "<delim-token>"),
      &TokenType::Number => write!(f, "<number-token>"),
      &TokenType::Percentage => write!(f, "<percentage-token>"),
      &TokenType::Dimension => write!(f, "<dimension-token>"),
      &TokenType::WhiteSpace => write!(f, "<whitespace-token>"),
      &TokenType::CDO => write!(f, "<CDO-token>"),
      &TokenType::CDC => write!(f, "<CDC-token>"),
      &TokenType::Colon => write!(f, "<colon-token>"),
      &TokenType::SemiColon => write!(f, "<semicolon-token>"),
      &TokenType::Comma => write!(f, "<comma-token>"),
      &TokenType::LeftSquareBracket => write!(f, "<[-token>"),
      &TokenType::RightSquareBracket => write!(f, "<]-token>"),
      &TokenType::LeftParentheses => write!(f, "<(-token>"),
      &TokenType::RightParentheses => write!(f, "<)-token>"),
      &TokenType::LeftCurlyBracket => write!(f, "<{{-token>"),
      &TokenType::RightCurlyBracket => write!(f, "<}}-token>"),
      &TokenType::Comment => write!(f, "<comment-token>"),
      &TokenType::EOF => write!(f, "<EOF-token>"),
    }
  }
}

/// Position
#[derive(Debug, PartialEq, Copy, Clone, Serialize)]
pub struct Pos {
  pub offset: usize,
  pub line: usize,
  pub column: usize,
}

impl Default for Pos {
  fn default() -> Self {
    Pos {
      offset: 0,
      line: 1,
      column: 1,
    }
  }
}

/// Token
#[derive(PartialEq, Serialize)]
pub struct Token<'s> {
  pub token_type: TokenType,
  pub start_pos: Pos,
  pub end_pos: Pos,
  pub content: &'s [u8],
}

impl<'s> Token<'s> {
  pub fn new(token_type: TokenType, start_pos: Pos, end_pos: Pos, content: &'s [u8]) -> Self {
    Token {
      token_type,
      start_pos,
      end_pos,
      content,
    }
  }

  pub fn is_eof(&self) -> bool {
    self.token_type == TokenType::EOF
  }

  pub fn is_at_keyword(&self) -> bool {
    self.token_type == TokenType::AtKeyword
  }
}

impl<'s> Debug for Token<'s> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("Token")
      .field("token_type", &self.token_type)
      .field("start_pos", &self.start_pos)
      .field("end_pos", &self.end_pos)
      .field("content", &from_utf8(self.content).unwrap())
      .finish()
  }
}
