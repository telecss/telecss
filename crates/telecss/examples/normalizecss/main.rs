use std::{env, fs::read_to_string};
use tele_parser::Parser;
use tele_tokenizer::*;

fn main() {
  let mut file_path = env::current_dir().unwrap();
  file_path.push("crates/telecss/examples/normalizecss/normalize.css");
  let file_path = file_path.as_os_str().to_str().unwrap();

  let css = read_to_string(file_path).unwrap();
  let css: &str = css.as_ref();

  // println!("CSS: {:?}", css.as_bytes());

  let mut tokenizer: Tokenizer = css.into();
  println!(
    "Result of tokenizing normalize.css: {:#?}",
    Parser::from(tokenizer.tokenize().unwrap()).parse()
  )
}
