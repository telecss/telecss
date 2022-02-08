#![feature(test)]

use std::{env, fs::read_to_string};
use tele_parser::Parser;
use tele_tokenizer::Tokenizer;

extern crate test;

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn parse_normalize_css(b: &mut Bencher) {
    let mut file_path = env::current_dir().unwrap();
    file_path.push("../telecss/examples/normalizecss/normalize.css");
    let file_path = file_path.as_os_str().to_str().unwrap();
    let css = read_to_string(file_path).unwrap();

    let mut tokenizer: Tokenizer = css.as_str().into();

    b.iter(|| Parser::from(tokenizer.tokenize().unwrap()).parse());
  }
}
