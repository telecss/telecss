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

pub fn is_letter(c: char) -> bool {
  match c {
    'a'..='z' | 'A'..='Z' => true,
    _ => false,
  }
}

pub fn is_ident_start(c: char) -> bool {
  is_letter(c) || c > '\u{0080}' || c == '_'
}

pub fn is_ident_char(c: char) -> bool {
  is_ident_start(c) || is_digit(c) || c == '\u{002D}' /* HYPHEN-MINUS (-) */
}

pub fn is_start_a_number(s: &[u8]) -> bool {
  let cp1 = *s.get(0).unwrap_or(&b'\0') as char;
  let cp2 = *s.get(1).unwrap_or(&b'\0') as char;
  let cp3 = *s.get(2).unwrap_or(&b'\0') as char;

  if cp1 == '+' || cp1 == '-' {
    if is_digit(cp2) {
      return true;
    }
    return cp2 == '.' && is_digit(cp3);
  } else if cp1 == '.' {
    return is_digit(cp2);
  }
  return is_digit(cp1);
}

pub fn is_comment_start(s: &[u8]) -> bool {
  let cp1 = *s.get(0).unwrap_or(&b'\0') as char;
  let cp2 = *s.get(1).unwrap_or(&b'\0') as char;

  cp1 == '/' && cp2 == '*'
}

pub fn is_comment_end(s: &[u8]) -> bool {
  let cp1 = *s.get(0).unwrap_or(&b'\0') as char;
  let cp2 = *s.get(1).unwrap_or(&b'\0') as char;

  cp1 == '*' && cp2 == '/'
}
