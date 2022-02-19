use tele_tokenizer::{Pos, Token};

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Loc {
  pub start: Pos,
  pub end: Pos,
}

#[derive(Debug, Default, PartialEq)]
pub struct DeclarationNode<'s> {
  pub loc: Loc,
  pub name: String,
  pub name_tokens: Vec<&'s Token<'s>>,
  pub value: String,
  pub value_tokens: Vec<&'s Token<'s>>,
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
