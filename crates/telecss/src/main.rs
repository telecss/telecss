use tele_parser::*;
use tele_tokenizer::*;

fn main() {
  let mut tokenizer: Tokenizer = r"  .foo { color: red; }  ".into();
  let tokens = tokenizer.tokenize();

  println!("Tokens: {:#?}", tokens);

  let parser = Parser::from(tokens.unwrap());
  println!("AST: {:#?}", parser.parse());
}
