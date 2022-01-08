#[derive(Debug, PartialEq)]
pub enum State {
  Initial,
  MayBeNumber,
  NewLine,
  EOF,
}

impl Default for State {
  fn default() -> Self {
    Self::Initial
  }
}
