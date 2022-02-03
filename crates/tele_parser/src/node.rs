use tele_tokenizer::Pos;

pub struct Loc {
  start: Pos,
  end: Pos,
  offset: usize,
}

pub struct DeclarationNode {
  loc: Loc,
  property: String,
  value: String,
}

pub struct RuleSetNode {
  loc: Loc,
  prelude: String,
  declarations: Vec<DeclarationNode>,
}

pub enum BlockItemNode {
  RuleSet(RuleSetNode),
  AtRule(AtRuleNode),
}

pub struct AtRuleNode {
  loc: Loc,
  prelude: String,
  block: Vec<BlockItemNode>,
}

pub enum StatementsNode {
  RuleSet(RuleSetNode),
  AtRule(AtRuleNode),
}

pub struct StyleSheetNode {
  loc: Loc,
  statements: Vec<StatementsNode>,
}

pub enum ASTNode {
  StyleSheet(StyleSheetNode),
  Statements(StatementsNode),
  RuleSet(RuleSetNode),
  AtRule(AtRuleNode),
  Declaration(DeclarationNode),
  BlockItem(BlockItemNode),
}
