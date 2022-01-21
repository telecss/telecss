use tele_tokenizer::*;

fn main() {
  let mut tokenizer: Tokenizer = "height: 30px;".into();
  let tokens = tokenizer.tokenize();

  println!("Tokens: {:#?}", tokens);
}
