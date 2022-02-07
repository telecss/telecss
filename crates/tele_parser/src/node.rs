use tele_tokenizer::Pos;

#[derive(Debug, Default, PartialEq)]
pub struct Loc {
  pub start: Pos,
  pub end: Pos,
}

#[derive(Debug, Default, PartialEq)]
pub struct DeclarationNode {
  pub loc: Loc,
  pub name: String,
  pub value: String,
  pub important: bool,
}

#[derive(Debug, Default, PartialEq)]
pub struct RuleSetNode {
  pub loc: Loc,
  pub prelude: String,
  pub declarations: Vec<DeclarationNode>,
}

#[derive(Debug, Default, PartialEq)]
pub struct AtRuleNode {
  pub loc: Loc,
  pub name: String,
  pub prelude: String,
  pub block: Vec<StatementNode>,
}

#[derive(Debug, PartialEq)]
pub enum StatementNode {
  RuleSet(RuleSetNode),
  AtRule(AtRuleNode),
}

#[derive(Debug, Default, PartialEq)]
pub struct StyleSheetNode {
  pub loc: Loc,
  pub statements: Vec<StatementNode>,
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
  StyleSheet(StyleSheetNode),
  Statements(StatementNode),
  RuleSet(RuleSetNode),
  AtRule(AtRuleNode),
  Declaration(DeclarationNode),
}
