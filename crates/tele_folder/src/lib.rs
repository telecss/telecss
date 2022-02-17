#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

use tele_parser::{AtRuleNode, DeclarationNode, RuleSetNode, StatementNode, StyleSheetNode};

pub trait Folder {
  fn fold_ss_node(&self, ss_node: &StyleSheetNode) -> StyleSheetNode {
    let mut new_ss_node = StyleSheetNode::default();
    new_ss_node.loc = ss_node.loc;
    new_ss_node.statements = ss_node
      .statements
      .iter()
      .map(|stat| match stat {
        StatementNode::AtRule(node) => StatementNode::AtRule(self.fold_at_rule_node(node)),
        StatementNode::RuleSet(node) => StatementNode::RuleSet(self.fold_rule_set_node(node)),
      })
      .collect();

    new_ss_node
  }

  fn fold_rule_set_node(&self, rule_set_node: &RuleSetNode) -> RuleSetNode {
    let mut new_rule_set_node = RuleSetNode::default();
    new_rule_set_node.loc = rule_set_node.loc;
    new_rule_set_node.prelude = rule_set_node.prelude.clone();
    new_rule_set_node.declarations = rule_set_node
      .declarations
      .iter()
      .map(|decl| self.fold_decl_node(decl))
      .collect();

    new_rule_set_node
  }

  fn fold_at_rule_node(&self, at_rule_node: &AtRuleNode) -> AtRuleNode {
    let mut new_at_rule_node = AtRuleNode::default();
    new_at_rule_node.loc = at_rule_node.loc;
    new_at_rule_node.name = at_rule_node.name.clone();
    new_at_rule_node.prelude = at_rule_node.prelude.clone();
    new_at_rule_node.block = at_rule_node
      .block
      .iter()
      .map(|stat| match stat {
        StatementNode::AtRule(node) => StatementNode::AtRule(self.fold_at_rule_node(node)),
        StatementNode::RuleSet(node) => StatementNode::RuleSet(self.fold_rule_set_node(node)),
      })
      .collect();

    new_at_rule_node
  }

  fn fold_decl_node(&self, decl_node: &DeclarationNode) -> DeclarationNode {
    let mut new_decl_node_node = DeclarationNode::default();
    new_decl_node_node.loc = decl_node.loc;
    new_decl_node_node.name = decl_node.name.clone();
    new_decl_node_node.value = decl_node.value.clone();
    new_decl_node_node.important = decl_node.important;

    new_decl_node_node
  }
}
