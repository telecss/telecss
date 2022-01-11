use tele_tokenizer::*;

fn main() {
  let mut tokenizer: Tokenizer = ".foo { color: red;    \t}".into();

  println!("Tokens: {:?}", tokenizer.tokenize());
}
