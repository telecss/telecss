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
}

impl<'s> From<&'s Vec<Token<'s>>> for Parser<'s> {
  fn from(tokens: &'s Vec<Token<'s>>) -> Self {
    Self {
      tokens,
      iter: RefCell::new(tokens.iter().peekable()),
    }
  }
}

impl<'s> Parser<'s> {
  pub fn parse(&self) -> ParserResult<StyleSheetNode> {
    let mut ss_node = StyleSheetNode::default();

    while let Some(node) = self.parse_statements()? {
      ss_node.statements.push(node)
    }

    self.skip_ws();

    let next = self.peek();
    debug_assert!(next.is_eof());

    ss_node.loc.end = next.end_pos;

    Ok(ss_node)
  }

  fn peek(&self) -> &Token {
    let next = self.iter.borrow_mut().peek().map(|&token| token);
    // Because the last token will always be EOF2, and we will handle it reasonably.
    // So in general/theoretically, we don't encounter `None`.
    debug_assert!(next.is_some());
    unsafe { next.unwrap_unchecked() }
  }

  fn consume(&self) -> Option<&Token> {
    self.iter.borrow_mut().next()
  }

  fn parse_statements(&self) -> ParserResult<Option<StatementNode>> {
    self.skip_ws();

    let next = self.peek();

    if next.is_eof() {
      return Ok(None);
    }

    if next.is_at_keyword() {
      Ok(self.parse_at_rule().map(|node| StatementNode::AtRule(node)))
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

    let mut token = self.peek();
    rule_set_node.loc.start = token.start_pos;

    loop {
      token = self.peek();
      if token.is_eof() {
        self.asset_eof(token)?;
      } else if token.is_lcb() {
        self.consume();
        while let Some(node) = self.parse_decl()? {
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

    rule_set_node.prelude = rule_set_node.prelude.trim_end().to_string();
    rule_set_node.loc.end = token.end_pos;

    Ok(Some(rule_set_node))
  }

  fn parse_at_rule(&self) -> Option<AtRuleNode> {
    unimplemented!()
  }

  fn parse_decl(&self) -> ParserResult<Option<DeclarationNode>> {
    self.skip_ws();

    // declaration name

    let next = self.peek();
    if next.is_rcb() {
      return Ok(None);
    }
    self.asset_eof(next)?;

    if !next.is_ident() {
      return Err(Box::new(SyntaxError::from(
        SyntaxErrorKind::InvalidDeclName,
      )));
    }

    let mut decl_node = DeclarationNode::default();
    decl_node.name = next.to_string();
    decl_node.loc.start = next.start_pos;

    self.consume();
    self.skip_ws();

    // :

    let next = self.peek();
    if !next.is_colon() {
      return Err(Box::new(SyntaxError::from(SyntaxErrorKind::MissingColon)));
    }
    self.asset_eof(next)?;
    self.consume();
    self.skip_ws();

    // declaration value

    let mut last_one: Option<&Token> = None;
    let mut second_last: Option<&Token> = None;
    loop {
      let token = self.peek();
      if token.is_eof() {
        self.asset_eof(token)?;
      } else if token.is_semi_colon() {
        self.consume();
        break;
      } else if token.is_rcb() {
        break;
      }

      self.consume();

      if !token.is_ws() {
        if last_one.is_some() {
          second_last = last_one;
        }
        last_one = Some(token);
      }

      decl_node.value.push_str(&token.to_string());
    }

    decl_node.important = second_last.is_some()
      && unsafe { second_last.unwrap_unchecked() }.is_delim(&[b'!'])
      && last_one.is_some()
      && unsafe { last_one.unwrap_unchecked() }.is_ident_with(b"important");

    last_one.map(|token| decl_node.loc.end = token.end_pos);

    Ok(Some(decl_node))
  }

  fn asset_eof(&self, token: &Token) -> ParserResult<()> {
    if token.is_eof() {
      return Err(Box::new(SyntaxError::from(SyntaxErrorKind::UnexpectedEOF)));
    }
    Ok(())
  }

  fn skip_ws(&self) {
    loop {
      let token = self.peek();
      if token.token_type == TokenType::WhiteSpace {
        self.consume();
      } else {
        break;
      }
    }
  }
}
