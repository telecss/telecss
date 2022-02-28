use serde::Serialize;
use std::{cell::RefCell, rc::Rc};
use tele_tokenizer::{Pos, Token};

#[derive(Debug, Default, PartialEq, Copy, Clone, Serialize)]
/// Represents the location information of the token in the source file
pub struct Loc {
  /// The start position
  pub start: Pos,
  /// The end position
  pub end: Pos,
}

#[derive(Debug, Default, PartialEq, Serialize)]
/// Represents a CSS declaration
pub struct DeclarationNode<'s> {
  /// Location information in the source file
  pub loc: Loc,
  /// Decalration name
  pub name: String,
  /// The tokens that made up the name of the declaration
  pub name_tokens: Vec<&'s Token<'s>>,
  /// Decalration name
  pub value: String,
  /// The tokens that made up the value of the declaration
  pub value_tokens: Vec<&'s Token<'s>>,
  /// Whether the value of this declaration defines `!important`
  pub important: bool,
}

#[derive(Debug, Default, PartialEq, Serialize)]
/// Represents a ruleset
pub struct RuleSetNode<'s> {
  /// Location information in the source file
  pub loc: Loc,
  /// prelude of the ruleset
  pub prelude: String,
  /// The tokens that made up the prelude of the ruleset
  pub prelude_tokens: Vec<&'s Token<'s>>,
  /// All declarations of the ruleset
  pub declarations: Vec<Rc<RefCell<DeclarationNode<'s>>>>,
}

#[derive(Debug, Default, PartialEq, Serialize)]
/// Represents a at-rule
pub struct AtRuleNode<'s> {
  /// Location information in the source file
  pub loc: Loc,
  /// The name of the at-rule
  pub name: String,
  /// The tokens that made up the name of the at-rule
  pub name_tokens: Vec<&'s Token<'s>>,
  /// prelude of the at-rule
  pub prelude: String,
  /// The tokens that made up the prelude of the at-rule
  pub prelude_tokens: Vec<&'s Token<'s>>,
  /// The block of the at-rule, consists of statements
  pub block: Vec<StatementNode<'s>>,
}

#[derive(Debug, PartialEq, Serialize)]
/// A statement is either a ruleset or an at-rule
pub enum StatementNode<'s> {
  /// Contains a ruleset node
  RuleSet(Rc<RefCell<RuleSetNode<'s>>>),
  /// Contains a at-rule node
  AtRule(Rc<RefCell<AtRuleNode<'s>>>),
}

#[derive(Debug, Default, PartialEq, Serialize)]
/// Represents a stylesheet
pub struct StyleSheetNode<'s> {
  /// Location information in the source file
  pub loc: Loc,
  /// All statements of the stylesheet
  pub statements: Vec<StatementNode<'s>>,
}

/// The root node of an AST
pub type AstType<'s> = Rc<RefCell<StyleSheetNode<'s>>>;
