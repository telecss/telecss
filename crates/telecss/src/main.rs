use tele_tokenizer::*;

fn main() {
  let mut tokenizer: Tokenizer = "div + p".into();
  let tokens = tokenizer.tokenize();

  println!("Tokens: {:#?}", tokens);
}
