#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

use tele_parser::{AtRuleNode, DeclarationNode, RuleSetNode, StyleSheetNode};

pub trait CodeGenerator {
  fn gen_ss_node(&self, ss_node: &StyleSheetNode);
  fn gen_rule_set_node(&self, rule_set_node: &RuleSetNode);
  fn gen_at_rule_node(&self, rule_set_node: &AtRuleNode);
  fn gen_decl_node(&self, rule_set_node: &DeclarationNode);
}
