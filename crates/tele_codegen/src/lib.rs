#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

use std::fmt::Write;
use tele_parser::{AtRuleNode, DeclarationNode, RuleSetNode, StyleSheetNode};

pub trait CodeGenerator {
  fn gen_ss_node(&mut self, ss_node: &StyleSheetNode);
  fn gen_rule_set_node(&mut self, rule_set_node: &RuleSetNode);
  fn gen_at_rule_node(&mut self, at_rule_node: &AtRuleNode);
  fn gen_decl_node(&mut self, decl_node: &DeclarationNode);
}

pub struct Codegen<W: Write> {
  css: W,
  line: usize,
  column: usize,
}

impl<W: Write> Codegen<W> {
  fn new(&self, writer: W) -> Self {
    Codegen {
      css: writer,
      line: 0,
      column: 1,
    }
  }

  fn new_line(&mut self) {
    self.line += 1;
    self.css.write_char('\n');
  }
}

impl<W: Write> CodeGenerator for Codegen<W> {
  fn gen_ss_node(&mut self, _ss_node: &StyleSheetNode) {
    unimplemented!()
  }
  fn gen_rule_set_node(&mut self, _rule_set_node: &RuleSetNode) {
    unimplemented!()
  }
  fn gen_at_rule_node(&mut self, _at_rule_node: &AtRuleNode) {
    unimplemented!()
  }
  fn gen_decl_node(&mut self, _decl_node: &DeclarationNode) {
    unimplemented!()
  }
}
