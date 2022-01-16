use tele_tokenizer::*;

#[test]
fn error_tests() {
  assert_eq!(
    Error::from(ErrorKind::UnexpectedEOF),
    Error {
      kind: ErrorKind::UnexpectedEOF
    }
  )
}

#[test]
fn error_format() {
  assert_eq!(
    format!("{}", Error::from(ErrorKind::UnexpectedEOF)),
    "Unexpected End Of File"
  );
}
