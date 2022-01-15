#![warn(missing_docs)]
//! A custom Error type for tokenization

use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
  kind: ErrorKind,
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.kind.as_str())
  }
}

#[derive(Debug)]
pub enum ErrorKind {
  UnexpectedEOF,
}

impl ErrorKind {
  pub fn as_str(&self) -> &'static str {
    match *self {
      Self::UnexpectedEOF => "Unexpected End Of File",
    }
  }
}
