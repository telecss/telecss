use std::fmt::{Display, Formatter, Result};

/// The CSS TokenType
#[derive(Debug, PartialEq)]
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
#[derive(Debug, Default, PartialEq)]
pub struct Pos {
  pub offset: usize,
  pub line: usize,
  pub column: usize,
}

/// Token
#[derive(Debug, PartialEq)]
pub struct Token<'t> {
  pub token_type: TokenType,
  pub start_pos: Pos,
  pub end_pos: Pos,
  pub content: &'t str,
}
