use tele_parser::*;
use tele_tokenizer::*;

fn main() {
  let mut tokenizer: Tokenizer = "color: '\''".into();
  let tokens = tokenizer.tokenize();

  println!("Tokens: {:#?}", tokens.unwrap());

  // let parser = Parser::from(tokens.unwrap());
  // println!("AST: {:#?}", parser.parse());
}
