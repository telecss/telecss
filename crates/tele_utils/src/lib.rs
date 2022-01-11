pub fn is_digit(c: char) -> bool {
  c > '0' && c < '9'
}

pub fn is_newline(c: char) -> bool {
  c == '\n'
}

pub fn is_whitespace(c: char) -> bool {
  is_newline(c) ||
  c == '\u{0009}' /* CHARACTER TABULATION */ ||
  c == '\u{000C}' /* FF */ ||
  c == '\u{000D}' /* CR */ ||
  c == '\u{0020}' /* SPACE */
}
