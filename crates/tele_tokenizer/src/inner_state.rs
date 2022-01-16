#[derive(Debug, PartialEq)]
pub enum State {
  Initial,
  MayBeNumber,
  Number,
  WhiteSpace,
  IdentStart,
  Ident,
  Solidus,
  MayBeComment,
  Comment,
  MayBeEndOfComment,
  EOF,
}

impl Default for State {
  fn default() -> Self {
    Self::Initial
  }
}
