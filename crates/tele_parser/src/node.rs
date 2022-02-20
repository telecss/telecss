use tele_tokenizer::{Pos, Token};

#[derive(Debug, Default, PartialEq, Copy, Clone)]
/// Represents the location information of the token in the source file
pub struct Loc {
  /// The start position
  pub start: Pos,
  /// The end position
  pub end: Pos,
}

#[derive(Debug, Default, PartialEq)]
/// Represents a CSS declaration
pub struct DeclarationNode<'s> {
  /// Location information
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

#[derive(Debug, Default, PartialEq)]
pub struct RuleSetNode<'s> {
  pub loc: Loc,
  pub prelude: String,
  pub prelude_tokens: Vec<&'s Token<'s>>,
  pub declarations: Vec<DeclarationNode<'s>>,
}

#[derive(Debug, Default, PartialEq)]
pub struct AtRuleNode<'s> {
  pub loc: Loc,
  pub name: String,
  pub name_tokens: Vec<&'s Token<'s>>,
  pub prelude: String,
  pub prelude_tokens: Vec<&'s Token<'s>>,
  pub block: Vec<StatementNode<'s>>,
}

#[derive(Debug, PartialEq)]
pub enum StatementNode<'s> {
  RuleSet(RuleSetNode<'s>),
  AtRule(AtRuleNode<'s>),
}

#[derive(Debug, Default, PartialEq)]
pub struct StyleSheetNode<'s> {
  pub loc: Loc,
  pub statements: Vec<StatementNode<'s>>,
}

#[derive(Debug, PartialEq)]
pub enum ASTNode<'s> {
  StyleSheet(StyleSheetNode<'s>),
  RuleSet(RuleSetNode<'s>),
  AtRule(AtRuleNode<'s>),
  Declaration(DeclarationNode<'s>),
}
