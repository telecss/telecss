#![warn(missing_docs)]
//! A custom Error type for parser

use std::{error::Error, fmt::Display};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
pub struct SyntaxError {
  pub kind: SyntaxErrorKind,
}

impl Error for SyntaxError {}

impl Display for SyntaxError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.kind.as_str())
  }
}

impl From<SyntaxErrorKind> for SyntaxError {
  fn from(kind: SyntaxErrorKind) -> Self {
    SyntaxError { kind }
  }
}

#[derive(Debug, PartialEq)]
pub enum SyntaxErrorKind {
  UnexpectedEOF,
}

impl SyntaxErrorKind {
  pub fn as_str(&self) -> &'static str {
    match *self {
      Self::UnexpectedEOF => "Unexpected End Of File",
    }
  }
}
