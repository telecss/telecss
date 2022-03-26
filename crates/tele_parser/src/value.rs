use crate::node::*;
use crate::{Parser, ParserResult};
use std::{cell::RefCell, rc::Rc};

impl<'s> Parser<'s> {
  pub fn parse_value(&self) -> ParserResult<Vec<Value>> {
    let mut values: Vec<Value> = Vec::new();

    loop {
      let token = self.peek();
      self.asset_eof(token)?;

      if token.is_semi_colon() {
        self.consume();
        break;
      } else if token.is_rcb() {
        break;
      }

      self.consume();

      if !token.is_comment() {
        if token.is_ident() {
          let mut value_node = IdentNode::default();
          value_node.loc.start = token.start_pos;
          value_node.loc.end = token.end_pos;
          value_node.name = token.to_string();
          values.push(Value::Ident(Rc::new(RefCell::new(value_node))));
          self.skip_ws_and_comments();
        } else if token.is_string() {
          let mut value_node = StringNode::default();
          value_node.loc.start = token.start_pos;
          value_node.loc.end = token.end_pos;
          value_node.value = token.to_string();
          values.push(Value::String(Rc::new(RefCell::new(value_node))));
          self.skip_ws_and_comments();
        } else if token.is_url() {
          let mut value_node = URLNode::default();
          value_node.loc.start = token.start_pos;
          value_node.loc.end = token.end_pos;
          value_node.value = token.to_string();
          values.push(Value::URL(Rc::new(RefCell::new(value_node))));
          // consume RightParentheses
          self.consume();
          self.skip_ws_and_comments();
        } else if token.is_delim_loose() || token.is_comma() {
          let mut value_node = OperatorNode::default();
          value_node.loc.start = token.start_pos;
          value_node.loc.end = token.end_pos;
          value_node.value = token.to_string();
          values.push(Value::Operator(Rc::new(RefCell::new(value_node))));
          self.skip_ws_and_comments();
        } else if token.is_number() {
          let mut value_node = NumberNode::default();
          value_node.loc.start = token.start_pos;
          value_node.loc.end = token.end_pos;
          value_node.value = token.to_string();
          values.push(Value::Number(Rc::new(RefCell::new(value_node))));
          self.skip_ws_and_comments();
        } else if token.is_percentage() {
          let mut value_node = PercentageNode::default();
          value_node.loc.start = token.start_pos;
          value_node.loc.end = token.end_pos;
          value_node.value = token.to_string();
          values.push(Value::Percentage(Rc::new(RefCell::new(value_node))));
          self.skip_ws_and_comments();
        } else if token.is_dimension() {
          let mut value_node = DimensionNode::default();
          value_node.loc.start = token.start_pos;
          value_node.value = token.to_string();

          // unit
          let token = self.peek();
          debug_assert!(token.is_ident());
          value_node.unit = token.to_string();
          self.consume();

          value_node.loc.end = token.end_pos;

          values.push(Value::Dimension(Rc::new(RefCell::new(value_node))));
          self.skip_ws_and_comments();
        } else if token.is_function() {
          // Function
          values.push(Value::Function(Rc::new(RefCell::new(
            self.parse_function(token.to_string())?,
          ))));
          self.skip_ws_and_comments();
        } else {
          if token.is_lp() {
            // push stack
            self.value_fncall_stack.borrow_mut().push(0)
          } else if token.is_rp() {
            let mut stack = self.value_fncall_stack.borrow_mut();
            let stack_top = stack.pop();
            match stack_top {
              Some(1) => {
                // the end of function call
                // indicates that we are currently parsing the children of a function
                break;
              }
              _ => {}
            }
          }

          // TODO: Here we can do further analysis
          // Raw
          let mut value_node = RawNode::default();
          value_node.loc.start = token.start_pos;
          value_node.loc.end = token.end_pos;
          value_node.value = token.to_string();
          values.push(Value::Raw(Rc::new(RefCell::new(value_node))));
          self.skip_ws_and_comments();
        }
      }
    }

    Ok(values)
  }

  fn parse_function(&self, fn_name: String) -> ParserResult<FunctionNode> {
    let mut fn_node = FunctionNode::default();

    // function name
    fn_node.name = fn_name;

    let token = self.peek();
    debug_assert!(token.is_lp());
    self.value_fncall_stack.borrow_mut().push(1);
    self.consume();

    // function children
    fn_node.children = self.parse_value()?;

    Ok(fn_node)
  }
}
