#[derive(Debug, PartialEq)]
pub enum State {
  Initial,
  Numeric,
  WhiteSpace,
  IdentLike,
  String,
  URL,
  Comment,
  EOF,
}

impl Default for State {
  fn default() -> Self {
    Self::Initial
  }
}
