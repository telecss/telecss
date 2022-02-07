use tele_parser::*;
use tele_tokenizer::*;

fn main() {
  let mut tokenizer: Tokenizer = r#"
  @media screen and (min-width: 900px) {
    article {
      padding: 1rem 3rem;
    }
  }
  "#
  .into();
  let tokens = tokenizer.tokenize();

  println!("Tokens: {:#?}", tokens);

  let parser = Parser::from(tokens.unwrap());
  println!("AST: {:#?}", parser.parse());
}
