#![warn(missing_docs)]
//! A custom Error type for tokenization

use serde::Serialize;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, PartialEq)]
pub struct Error {
  pub kind: ErrorKind,
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.kind.as_str())
  }
}

impl From<ErrorKind> for Error {
  fn from(kind: ErrorKind) -> Self {
    Error { kind }
  }
}

#[derive(Debug, Serialize, PartialEq)]
pub enum ErrorKind {
  UnexpectedEOF,
  InvalidIdentSeq,
  BadString,
}

impl ErrorKind {
  pub fn as_str(&self) -> &'static str {
    match *self {
      Self::UnexpectedEOF => "Unexpected End Of File",
      Self::InvalidIdentSeq => "invalid identifier sequence",
      Self::BadString => "Bad String",
    }
  }
}
