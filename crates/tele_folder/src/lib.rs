#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

use tele_parser::{AtRuleNode, DeclarationNode, RuleSetNode, StatementNode, StyleSheetNode};

pub trait Folder<'s> {
  fn fold_ss_node(&mut self, mut ss_node: StyleSheetNode<'s>) -> StyleSheetNode<'s> {
    ss_node.statements = ss_node
      .statements
      .into_iter()
      .map(|stat| match stat {
        StatementNode::AtRule(node) => StatementNode::AtRule(self.fold_at_rule_node(node)),
        StatementNode::RuleSet(node) => StatementNode::RuleSet(self.fold_rule_set_node(node)),
      })
      .collect();

    ss_node
  }

  fn fold_rule_set_node(&mut self, mut rule_set_node: RuleSetNode<'s>) -> RuleSetNode<'s> {
    rule_set_node.declarations = rule_set_node
      .declarations
      .into_iter()
      .map(|decl| self.fold_decl_node(decl))
      .collect();

    rule_set_node
  }

  fn fold_at_rule_node(&mut self, mut at_rule_node: AtRuleNode<'s>) -> AtRuleNode<'s> {
    at_rule_node.block = at_rule_node
      .block
      .into_iter()
      .map(|stat| match stat {
        StatementNode::AtRule(node) => StatementNode::AtRule(self.fold_at_rule_node(node)),
        StatementNode::RuleSet(node) => StatementNode::RuleSet(self.fold_rule_set_node(node)),
      })
      .collect();

    at_rule_node
  }

  fn fold_decl_node(&mut self, decl_node: DeclarationNode<'s>) -> DeclarationNode<'s> {
    let mut new_decl_node_node = DeclarationNode::default();
    new_decl_node_node.loc = decl_node.loc;
    new_decl_node_node.name = decl_node.name.clone();
    new_decl_node_node.value = decl_node.value.clone();
    new_decl_node_node.important = decl_node.important;

    new_decl_node_node
  }
}

pub fn transform<'a, T: Folder<'a>>(ast: StyleSheetNode<'a>, mut folder: T) -> StyleSheetNode<'a> {
  folder.fold_ss_node(ast)
}
