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
