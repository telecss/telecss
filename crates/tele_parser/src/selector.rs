use crate::{Loc, Parser};
use serde::Serialize;

/// https://www.w3.org/TR/selectors-4/#type-selectors
/// https://www.w3.org/TR/selectors-4/#the-universal-selector
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct TypeSelectorNode {
  /// Location information in the source file
  pub loc: Loc,
  /// the name of the Type Selector
  pub name: String,
}

/// https://www.w3.org/TR/selectors-4/#attribute-selectors
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct AttrSelectorNode {
  /// Location information in the source file
  pub loc: Loc,
  /// the name of the Attribute Selector
  pub name: String,
  /// the value of the Attribute Selector
  pub value: String,
  /// the flag of the Attribute Selector
  pub flag: String,
}

/// https://www.w3.org/TR/selectors-4/#class-html
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct ClassSelectorNode {
  /// Location information in the source file
  pub loc: Loc,
  /// the name of the Class Selector
  pub name: String,
}

/// https://www.w3.org/TR/selectors-4/#id-selectors
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct IdSelectorNode {
  /// Location information in the source file
  pub loc: Loc,
  /// the name of the ID Selector
  pub name: String,
}

/// https://www.w3.org/TR/selectors-4/#pseudo-classes
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct PseudoClassSelectorNode {
  /// Location information in the source file
  pub loc: Loc,
  /// the name of the Pseudo-Class Selector
  pub name: String,
  // @TODO: the name of the Pseudo-Class Selector
  // pub children: Vec<>
}

/// https://www.w3.org/TR/selectors-4/#pseudo-elements
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct PseudoElementSelectorNode {
  /// Location information in the source file
  pub loc: Loc,
  /// the name of the Pseudo-Element Selector
  pub name: String,
  // @TODO: the name of the Pseudo-Element Selector
  // pub children: Vec<>
}

impl<'s> Parser<'s> {
  pub fn parse_selector(&self) {
    unimplemented!()
  }
}
