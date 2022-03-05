#![warn(missing_docs)]
#![deny(unused_imports)]
#![deny(unused_variables)]

//! Visitor/Folder pattern for transforming the AST

use std::{cell::RefCell, rc::Rc};
use tele_parser::{
  AtRuleNode, DeclarationNode, DimensionNode, FunctionNode, IdentNode, NumberNode, OperatorNode,
  PercentageNode, RawNode, RuleSetNode, StatementNode, StringNode, StyleSheetNode, URLNode, Value,
};

pub trait MutVisitor<'s> {
  fn visit_ss_node(&self, _ss_node: Rc<RefCell<StyleSheetNode<'s>>>) {}
  fn visit_rule_set_node(&self, _rule_set_node: Rc<RefCell<RuleSetNode<'s>>>) {}
  fn visit_at_rule_node(&self, _at_rule_node: Rc<RefCell<AtRuleNode<'s>>>) {}
  fn visit_decl_node(&self, _decl_node: Rc<RefCell<DeclarationNode<'s>>>) {}
  fn visit_ident_node(&self, _decl_node: Rc<RefCell<IdentNode>>) {}
  fn visit_url_node(&self, _decl_node: Rc<RefCell<URLNode>>) {}
  fn visit_dimension_node(&self, _decl_node: Rc<RefCell<DimensionNode>>) {}
  fn visit_number_node(&self, _decl_node: Rc<RefCell<NumberNode>>) {}
  fn visit_operator_node(&self, _decl_node: Rc<RefCell<OperatorNode>>) {}
  fn visit_percentage_node(&self, _decl_node: Rc<RefCell<PercentageNode>>) {}
  fn visit_string_node(&self, _decl_node: Rc<RefCell<StringNode>>) {}
  fn visit_fn_node(&self, _decl_node: Rc<RefCell<FunctionNode>>) {}
  fn visit_raw_node(&self, _decl_node: Rc<RefCell<RawNode>>) {}
}

pub struct VisitMut<'s> {
  ast: Rc<RefCell<StyleSheetNode<'s>>>,
  visitors: Vec<Box<dyn MutVisitor<'s>>>,
}

impl<'s> VisitMut<'s> {
  pub fn new(ast: Rc<RefCell<StyleSheetNode<'s>>>, visitors: Vec<Box<dyn MutVisitor<'s>>>) -> Self {
    Self { ast, visitors }
  }

  pub fn transform(&self) {
    self.visit_ss_node(Rc::clone(&self.ast));
  }

  /// Visit [StyleSheetNode] node
  fn visit_ss_node(&self, ss_node: Rc<RefCell<StyleSheetNode<'s>>>) {
    for visitor in self.visitors.iter() {
      visitor.visit_ss_node(Rc::clone(&ss_node));
    }

    for stat in ss_node.borrow().statements.iter() {
      match stat {
        StatementNode::AtRule(node) => self.visit_at_rule_node(Rc::clone(node)),
        StatementNode::RuleSet(node) => self.visit_rule_set_node(Rc::clone(node)),
      }
    }
  }

  /// Visit [RuleSetNode] node
  fn visit_rule_set_node(&self, rule_set_node: Rc<RefCell<RuleSetNode<'s>>>) {
    for visitor in self.visitors.iter() {
      visitor.visit_rule_set_node(Rc::clone(&rule_set_node));
    }

    for decl in rule_set_node.borrow().declarations.iter() {
      self.visit_decl_node(Rc::clone(decl))
    }
  }

  /// Visit [AtRuleNode] node
  fn visit_at_rule_node(&self, at_rule_node: Rc<RefCell<AtRuleNode<'s>>>) {
    for visitor in self.visitors.iter() {
      visitor.visit_at_rule_node(Rc::clone(&at_rule_node));
    }

    for stat in at_rule_node.borrow().block.iter() {
      match stat {
        StatementNode::AtRule(node) => self.visit_at_rule_node(Rc::clone(node)),
        StatementNode::RuleSet(node) => self.visit_rule_set_node(Rc::clone(node)),
      }
    }
  }

  /// Visit [DeclarationNode] node
  fn visit_decl_node(&self, decl_node: Rc<RefCell<DeclarationNode<'s>>>) {
    for visitor in self.visitors.iter() {
      visitor.visit_decl_node(Rc::clone(&decl_node));
    }

    for value in decl_node.borrow().value.iter() {
      match value {
        Value::Ident(node) => {
          for visitor in self.visitors.iter() {
            visitor.visit_ident_node(Rc::clone(node));
          }
        }
        Value::URL(node) => {
          for visitor in self.visitors.iter() {
            visitor.visit_url_node(Rc::clone(node));
          }
        }
        Value::Dimension(node) => {
          for visitor in self.visitors.iter() {
            visitor.visit_dimension_node(Rc::clone(node));
          }
        }
        Value::Number(node) => {
          for visitor in self.visitors.iter() {
            visitor.visit_number_node(Rc::clone(node));
          }
        }
        Value::Operator(node) => {
          for visitor in self.visitors.iter() {
            visitor.visit_operator_node(Rc::clone(node));
          }
        }
        Value::Percentage(node) => {
          for visitor in self.visitors.iter() {
            visitor.visit_percentage_node(Rc::clone(node));
          }
        }
        Value::String(node) => {
          for visitor in self.visitors.iter() {
            visitor.visit_string_node(Rc::clone(node));
          }
        }
        Value::Function(node) => {
          for visitor in self.visitors.iter() {
            visitor.visit_fn_node(Rc::clone(node));
          }
        }
        Value::Raw(node) => {
          for visitor in self.visitors.iter() {
            visitor.visit_raw_node(Rc::clone(node));
          }
        }
      };
    }
  }
}
