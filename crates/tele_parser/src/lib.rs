#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

//! CSS Parser

mod node;

pub use node::*;

use std::{
  cell::RefCell,
  iter::{Enumerate, Peekable},
  slice::Iter,
};

use tele_tokenizer::*;

pub struct Parser<'s> {
  tokens: &'s Vec<Token<'s>>,
  iter: RefCell<Peekable<Enumerate<Iter<'s, Token<'s>>>>>,
  index: RefCell<usize>,
}

impl<'s> From<&'s Vec<Token<'s>>> for Parser<'s> {
  fn from(tokens: &'s Vec<Token<'s>>) -> Self {
    Self {
      tokens,
      iter: RefCell::new(tokens.iter().enumerate().peekable()),
      index: RefCell::new(0),
    }
  }
}

impl<'s> Parser<'s> {
  pub fn parse(&self) {
    let mut ss_node = StyleSheetNode::default();

    while let Some(node) = self.parse_statements() {
      ss_node.statements.push(node)
    }
  }

  fn next(&self) -> Option<&Token> {
    self.iter.borrow_mut().peek().map(|&(_, token)| token)
  }

  fn parse_statements(&self) -> Option<StatementNode> {
    self.skip_ws();

    let next = self.next();

    if next.is_none() {
      return None;
    }

    if unsafe { next.unwrap_unchecked() }.is_at_keyword() {
      self
        .parse_rule_set()
        .map(|node| StatementNode::RuleSet(node))
    } else {
      self.parse_at_rule().map(|node| StatementNode::AtRule(node))
    }
  }

  fn parse_rule_set(&self) -> Option<RuleSetNode> {
    unimplemented!()
  }

  fn parse_at_rule(&self) -> Option<AtRuleNode> {
    unimplemented!()
  }

  fn skip_ws(&self) {
    let mut iter = self.iter.borrow_mut();
    while let Some(&(_, token)) = iter.peek() {
      if token.token_type == TokenType::WhiteSpace {
        iter.next();
      } else {
        break;
      }
    }
  }
}
