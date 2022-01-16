#[derive(Debug, PartialEq)]
pub enum State {
  Initial,
  Numeric,
  WhiteSpace,
  IdentStart,
  Ident,
  Comment,
  EOF,
}

impl Default for State {
  fn default() -> Self {
    Self::Initial
  }
}
