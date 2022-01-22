pub fn is_digit(c: char) -> bool {
  c >= '0' && c <= '9'
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
  is_ident_start(c) || is_digit(c) || c == '-'
}

// https://www.w3.org/TR/css-syntax-3/#would-start-an-identifier
pub fn would_start_an_ident_seq(s: &[u8]) -> bool {
  let cp1 = char_at(s, 0);
  let cp2 = char_at(s, 1);
  if cp1 == '-' {
    return is_ident_start(cp2) || cp2 == '-';
  }
  if is_ident_start(cp1) {
    return true;
  }
  false
}

pub fn is_start_a_number(s: &[u8]) -> bool {
  let cp1 = char_at(s, 0);
  let cp2 = char_at(s, 1);
  let cp3 = char_at(s, 2);

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
  let cp1 = char_at(s, 0);
  let cp2 = char_at(s, 1);

  cp1 == '/' && cp2 == '*'
}

pub fn is_comment_end(s: &[u8]) -> bool {
  let cp1 = char_at(s, 0);
  let cp2 = char_at(s, 1);

  cp1 == '*' && cp2 == '/'
}

pub fn char_at(s: &[u8], index: usize) -> char {
  *s.get(index).unwrap_or(&b'\0') as char
}

// https://www.w3.org/TR/css-syntax-3/#non-printable-code-point
pub fn is_non_printable(c: char) -> bool {
  match c {
    '\u{0000}'..='\u{0008}' | '\u{000B}' | '\u{000E}'..='\u{001F}' | '\u{007F}' => true,
    _ => false,
  }
}
