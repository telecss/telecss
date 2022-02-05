use tele_tokenizer::Pos;

#[derive(Debug, Default, PartialEq)]
pub struct Loc {
  start: Pos,
  end: Pos,
}

#[derive(Debug, Default, PartialEq)]
pub struct DeclarationNode {
  loc: Loc,
  property: String,
  value: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct RuleSetNode {
  pub loc: Loc,
  pub prelude: String,
  pub declarations: Vec<DeclarationNode>,
}

#[derive(Debug, PartialEq)]
pub enum BlockItemNode {
  RuleSet(RuleSetNode),
  AtRule(AtRuleNode),
}

#[derive(Debug, Default, PartialEq)]
pub struct AtRuleNode {
  loc: Loc,
  prelude: String,
  block: Vec<BlockItemNode>,
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
  BlockItem(BlockItemNode),
}
