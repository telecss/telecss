use tele_tokenizer::*;

fn main() {
  let mut tokenizer: Tokenizer = ".foo { color: red; }".into();

  println!("Tokens: {:?}", tokenizer.tokenize());
}
