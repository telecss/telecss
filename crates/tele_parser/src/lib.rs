#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

//! A simpler and faster CSS parser.
//!
//! # Usages
//! ```rust
//! use tele_tokenizer::Tokenizer;
//! use tele_parser::Parser;
//!
//! // Create a tokenizer
//! let mut tokenizer: Tokenizer = r".foo { color: red; }".into();
//! // Tokenize based on the given raw string,
//! let tokens = tokenizer.tokenize().unwrap();
//! // Create a parser from a sequence of tokens
//! let parser = Parser::from(tokens);
//! // parsing it
//! let ast = parser.parse();
//! ```

mod error;
mod node;
mod selector;
mod value;

pub use error::Result as ParserResult;
use error::{SyntaxError, SyntaxErrorKind};
pub use node::*;

use std::{cell::RefCell, iter::Peekable, rc::Rc, slice::Iter};

use tele_tokenizer::*;

/// Represents the CSS parser.
pub struct Parser<'s> {
  iter: RefCell<Peekable<Iter<'s, Token<'s>>>>,
  value_fncall_stack: RefCell<Vec<u8>>,
}

impl<'s> From<&'s Vec<Token<'s>>> for Parser<'s> {
  fn from(tokens: &'s Vec<Token<'s>>) -> Self {
    Self {
      iter: RefCell::new(tokens.iter().peekable()),
      value_fncall_stack: RefCell::new(vec![]),
    }
  }
}

impl<'s> Parser<'s> {
  /// Perform parsing
  pub fn parse(&self) -> ParserResult<AstType> {
    let mut ss_node = StyleSheetNode::default();

    while let Some(node) = self.parse_statements()? {
      ss_node.statements.push(node)
    }

    self.skip_ws_and_comments();

    let next = self.peek();
    debug_assert!(next.is_eof());

    ss_node.loc.end = next.end_pos;

    Ok(Rc::new(RefCell::new(ss_node)))
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
    self.skip_ws_and_comments();

    let next = self.peek();

    if next.is_eof() {
      return Ok(None);
    }

    if next.is_at_keyword() {
      Ok(
        self
          .parse_at_rule()?
          .map(|node| StatementNode::AtRule(Rc::new(RefCell::new(node)))),
      )
    } else {
      Ok(
        self
          .parse_rule_set()?
          .map(|node| StatementNode::RuleSet(Rc::new(RefCell::new(node)))),
      )
    }
  }

  fn parse_rule_set(&self) -> ParserResult<Option<RuleSetNode>> {
    let mut rule_set_node = RuleSetNode::default();

    let mut token = self.peek();
    rule_set_node.loc.start = token.start_pos;

    loop {
      token = self.peek();
      self.asset_eof(token)?;

      if token.is_lcb() {
        self.consume();
        while let Some(node) = self.parse_decl()? {
          rule_set_node.declarations.push(Rc::new(RefCell::new(node)));
        }
      } else if token.is_rcb() {
        self.consume();
        break;
      } else {
        if !token.is_comment() {
          rule_set_node.prelude.push_str(&token.to_string());
          rule_set_node.prelude_tokens.push(token);
        }
        self.consume();
      }
    }

    rule_set_node.prelude = rule_set_node.prelude.trim_end().to_string();
    rule_set_node.loc.end = token.end_pos;

    Ok(Some(rule_set_node))
  }

  fn parse_at_rule(&self) -> ParserResult<Option<AtRuleNode>> {
    let mut at_rule_node = AtRuleNode::default();

    // unsafe: we are sure that the next token is `@`
    let mut token = unsafe { self.consume().unwrap_unchecked() };
    at_rule_node.loc.start = token.start_pos;
    at_rule_node.name = token.to_string();
    at_rule_node.name_tokens.push(token);

    self.skip_ws_and_comments();

    loop {
      token = self.peek();
      self.asset_eof(token)?;

      if token.is_lcb() {
        self.consume();

        loop {
          self.skip_ws_and_comments();
          let next = self.peek();
          if next.is_rcb() {
            // end right curly bracket
            self.consume();
            break;
          }

          let statement = self.parse_statements()?;
          if statement.is_none() {
            break;
          }

          // unsafe: checked above
          let statement = unsafe { statement.unwrap_unchecked() };

          at_rule_node.block.push(statement);
        }
        break;
      } else if token.is_semi_colon() {
        // no-block
        self.consume();
        break;
      } else {
        // prelude
        self.consume();
        at_rule_node.prelude.push_str(&token.to_string());
        at_rule_node.prelude_tokens.push(token);
      }
    }

    at_rule_node.loc.end = token.end_pos;
    at_rule_node.prelude = at_rule_node.prelude.trim_end().to_string();

    Ok(Some(at_rule_node))
  }

  fn parse_decl(&self) -> ParserResult<Option<DeclarationNode>> {
    self.skip_ws_and_comments();

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
    decl_node.name_tokens.push(next);
    decl_node.loc.start = next.start_pos;

    self.consume();
    self.skip_ws_and_comments();

    // :

    let next = self.peek();
    if !next.is_colon() {
      return Err(Box::new(SyntaxError::from(SyntaxErrorKind::MissingColon)));
    }
    self.asset_eof(next)?;
    self.consume();
    self.skip_ws_and_comments();

    // parse value
    decl_node.value = self.parse_value()?;

    // check !important
    let mut rev_valus = decl_node.value.iter().rev();
    let last_value = rev_valus.next();
    let second_last_value = rev_valus.next();
    if second_last_value.is_some() && last_value.is_some() {
      let last_value = unsafe { last_value.unwrap_unchecked() };
      let second_last_value = unsafe { second_last_value.unwrap_unchecked() };
      if let Value::Operator(node) = second_last_value {
        if node.borrow().value.as_bytes() == b"!" {
          if let Value::Ident(node) = last_value {
            if node.borrow().name.as_bytes() == b"important" {
              decl_node.important = true;
            }
          }
        }
      }
    }

    Ok(Some(decl_node))
  }

  fn asset_eof(&self, token: &Token) -> ParserResult<()> {
    if token.is_eof() {
      return Err(Box::new(SyntaxError::from(SyntaxErrorKind::UnexpectedEOF)));
    }
    Ok(())
  }

  fn skip_ws_and_comments(&self) {
    loop {
      let token = self.peek();
      if token.is_ws() || token.is_comment() {
        self.consume();
      } else {
        break;
      }
    }
  }
}
