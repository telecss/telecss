use tele_utils::*;

#[test]
fn is_start_a_number_tests() {
  assert!(is_start_a_number("+1".as_bytes()));
  assert!(is_start_a_number("-1".as_bytes()));
  assert!(is_start_a_number("+.1".as_bytes()));
  assert!(is_start_a_number("-.1".as_bytes()));
  assert!(is_start_a_number(".1".as_bytes()));
  assert!(is_start_a_number("1".as_bytes()));

  assert!(!is_start_a_number(".".as_bytes()));
  assert!(!is_start_a_number(".a".as_bytes()));
  assert!(!is_start_a_number("+a".as_bytes()));
  assert!(!is_start_a_number("-a".as_bytes()));
}
