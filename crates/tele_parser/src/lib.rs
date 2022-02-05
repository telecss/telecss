#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

//! CSS Parser

mod error;
mod node;

pub use error::Result as ParserResult;
use error::{SyntaxError, SyntaxErrorKind};
pub use node::*;

use std::{cell::RefCell, iter::Peekable, slice::Iter};

use tele_tokenizer::*;

pub struct Parser<'s> {
  tokens: &'s Vec<Token<'s>>,
  iter: RefCell<Peekable<Iter<'s, Token<'s>>>>,
  index: RefCell<usize>,
}

impl<'s> From<&'s Vec<Token<'s>>> for Parser<'s> {
  fn from(tokens: &'s Vec<Token<'s>>) -> Self {
    Self {
      tokens,
      iter: RefCell::new(tokens.iter().peekable()),
      index: RefCell::new(0),
    }
  }
}

impl<'s> Parser<'s> {
  pub fn parse(&self) -> ParserResult<StyleSheetNode> {
    let mut ss_node = StyleSheetNode::default();

    while let Some(node) = self.parse_statements()? {
      ss_node.statements.push(node)
    }

    // TODO: correct loc
    Ok(ss_node)
  }

  fn peek(&self) -> Option<&Token> {
    self.iter.borrow_mut().peek().map(|&token| token)
  }

  fn consume(&self) -> Option<&Token> {
    self.iter.borrow_mut().next()
  }

  fn parse_statements(&self) -> ParserResult<Option<StatementNode>> {
    self.skip_ws();

    let next = self.peek();

    if next.is_none() || unsafe { next.unwrap_unchecked() }.is_eof() {
      return Ok(None);
    }

    let next = unsafe { next.unwrap_unchecked() };

    if next.is_at_keyword() {
      Ok(self.parse_at_rule().map(|node| StatementNode::AtRule(node)))
    } else if next.is_eof() {
      return Err(Box::new(SyntaxError::from(SyntaxErrorKind::UnexpectedEOF)));
    } else {
      Ok(
        self
          .parse_rule_set()?
          .map(|node| StatementNode::RuleSet(node)),
      )
    }
  }

  fn parse_rule_set(&self) -> ParserResult<Option<RuleSetNode>> {
    let mut rule_set_node = RuleSetNode::default();

    while let Some(token) = self.peek() {
      if token.is_eof() {
        return Err(Box::new(SyntaxError::from(SyntaxErrorKind::UnexpectedEOF)));
      } else if token.is_lcb() {
        while let Some(node) = self.parse_decls() {
          rule_set_node.declarations.push(node);
        }
      } else if token.is_rcb() {
        self.consume();
        break;
      } else {
        rule_set_node.prelude.push_str(&token.to_string());
        self.consume();
      }
    }

    // TODO: correct loc
    Ok(Some(rule_set_node))
  }

  fn parse_at_rule(&self) -> Option<AtRuleNode> {
    unimplemented!()
  }

  fn parse_decls(&self) -> Option<DeclarationNode> {
    // TODO: real impl
    while let Some(token) = self.peek() {
      if token.is_rcb() {
        break;
      }
      self.consume();
    }
    None
  }

  fn skip_ws(&self) {
    while let Some(token) = self.peek() {
      if token.token_type == TokenType::WhiteSpace {
        self.consume();
      } else {
        break;
      }
    }
  }
}
