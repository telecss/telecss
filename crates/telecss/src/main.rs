use tele_parser::*;
use tele_tokenizer::*;

fn main() {
  let mut tokenizer: Tokenizer = "div + p".into();
  let tokens = tokenizer.tokenize();

  let parser = Parser::from(tokens.unwrap());

  println!("Tokens: {:#?}", parser.parse());
}
