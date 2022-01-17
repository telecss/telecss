use tele_tokenizer::*;

fn main() {
  let mut tokenizer: Tokenizer = ".foo { background: url(  'xxx.png'); }".into();
  let tokens = tokenizer.tokenize();

  println!("Tokens: {:#?}", tokens);
}
