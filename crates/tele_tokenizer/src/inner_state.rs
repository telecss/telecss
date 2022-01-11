#[derive(Debug, PartialEq)]
pub enum State {
  Initial,
  MayBeNumber,
  WhiteSpace,
  EOF,
}

impl Default for State {
  fn default() -> Self {
    Self::Initial
  }
}
