#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

use tele_parser::{AtRuleNode, DeclarationNode, RuleSetNode};

pub trait Folder {
  fn fold_rule_set_node(&self) -> RuleSetNode;
  fn fold_at_rule_node(&self) -> AtRuleNode;
  fn fold_decl_node(&self) -> DeclarationNode;
}
