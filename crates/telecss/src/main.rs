use tele_tokenizer::*;

fn main() {
  let mut tokenizer: Tokenizer = "2 .3 4.4 +5 -6 7E7 8e8 9E+9 9e-9".into();
  let tokens = tokenizer.tokenize();

  println!("Tokens: {:#?}", tokens);
}
