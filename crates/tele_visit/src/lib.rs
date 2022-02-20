#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

//! Visitor/Folder pattern for transforming the AST

use tele_parser::{AtRuleNode, DeclarationNode, RuleSetNode, StatementNode, StyleSheetNode};

/// [VisitMut] can modify AST nodes *in place*
pub trait VisitMut<'s> {
  /// Visit [StyleSheetNode] node
  fn visit_ss_node(&mut self, mut ss_node: StyleSheetNode<'s>) -> StyleSheetNode<'s> {
    ss_node.statements = ss_node
      .statements
      .into_iter()
      .map(|stat| match stat {
        StatementNode::AtRule(node) => StatementNode::AtRule(self.visit_at_rule_node(node)),
        StatementNode::RuleSet(node) => StatementNode::RuleSet(self.visit_rule_set_node(node)),
      })
      .collect();

    ss_node
  }

  /// Visit [RuleSetNode] node
  fn visit_rule_set_node(&mut self, mut rule_set_node: RuleSetNode<'s>) -> RuleSetNode<'s> {
    rule_set_node.declarations = rule_set_node
      .declarations
      .into_iter()
      .map(|decl| self.visit_decl_node(decl))
      .collect();

    rule_set_node
  }

  /// Visit [AtRuleNode] node
  fn visit_at_rule_node(&mut self, mut at_rule_node: AtRuleNode<'s>) -> AtRuleNode<'s> {
    at_rule_node.block = at_rule_node
      .block
      .into_iter()
      .map(|stat| match stat {
        StatementNode::AtRule(node) => StatementNode::AtRule(self.visit_at_rule_node(node)),
        StatementNode::RuleSet(node) => StatementNode::RuleSet(self.visit_rule_set_node(node)),
      })
      .collect();

    at_rule_node
  }

  /// Visit [DeclarationNode] node
  fn visit_decl_node(&mut self, decl_node: DeclarationNode<'s>) -> DeclarationNode<'s> {
    decl_node
  }
}

/// Transform the AST according to the given visitor.
pub fn transform<'a, T: VisitMut<'a>>(
  ast: StyleSheetNode<'a>,
  mut visitor: T,
) -> StyleSheetNode<'a> {
  visitor.visit_ss_node(ast)
}
