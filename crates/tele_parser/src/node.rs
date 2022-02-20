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

#[derive(Debug, Default, PartialEq)]
/// Represents a ruleset
pub struct RuleSetNode<'s> {
  /// Location information in the source file
  pub loc: Loc,
  /// prelude of the ruleset
  pub prelude: String,
  /// The tokens that made up the prelude of the ruleset
  pub prelude_tokens: Vec<&'s Token<'s>>,
  /// All declarations of the ruleset
  pub declarations: Vec<DeclarationNode<'s>>,
}

#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, PartialEq)]
/// A statement is either a ruleset or an at-rule
pub enum StatementNode<'s> {
  /// Contains a ruleset node
  RuleSet(RuleSetNode<'s>),
  /// Contains a at-rule node
  AtRule(AtRuleNode<'s>),
}

#[derive(Debug, Default, PartialEq)]
/// Represents a stylesheet
pub struct StyleSheetNode<'s> {
  /// Location information in the source file
  pub loc: Loc,
  /// All statements of the stylesheet
  pub statements: Vec<StatementNode<'s>>,
}

#[derive(Debug, PartialEq)]
/// The root node of an AST
pub enum ASTNode<'s> {
  /// Contains a stylesheet node
  StyleSheet(StyleSheetNode<'s>),
  /// Contains a ruleset node
  RuleSet(RuleSetNode<'s>),
  /// Contains a at-rule node
  AtRule(AtRuleNode<'s>),
  /// Contains a declaration node
  Declaration(DeclarationNode<'s>),
}
