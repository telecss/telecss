use tele_tokenizer::*;

fn main() {
  let tokens = Tokenizer::tokenize("\n\n   .foo { color: red; }");

  println!("Tokens: {:?}", tokens);
}
