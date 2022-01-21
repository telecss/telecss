use tele_tokenizer::*;

fn main() {
  let mut tokenizer: Tokenizer = "#foo {}".into();
  let tokens = tokenizer.tokenize();

  println!("Tokens: {:#?}", tokens);
}
