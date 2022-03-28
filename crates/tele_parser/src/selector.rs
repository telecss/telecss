use std::{cell::RefCell, rc::Rc};

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
  /// The name of the Pseudo-Class Selector
  pub name: String,
  /// Indicates whether the Pseudo-Class is functional: https://www.w3.org/TR/selectors-4/#functional-pseudo-class
  pub functional: bool,
  /// The children of the Pseudo-Class Selector
  pub children: Vec<PseudoClassSelectorChildren>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum PseudoClassSelectorChildren {
  AnPlusB(Rc<RefCell<AnPlusBNode>>),
  Ident(Rc<RefCell<IdentNode>>),
}

#[derive(Debug, Default, PartialEq, Serialize)]
pub struct AnPlusBNode {
  /// Location information in the source file
  pub loc: Loc,
  pub a: Option<String>,
  pub b: Option<String>,
}

#[derive(Debug, Default, PartialEq, Serialize)]
pub struct IdentNode {
  /// Location information in the source file
  pub loc: Loc,
  /// the name of the Ident Node
  pub name: String,
}

/// https://www.w3.org/TR/selectors-4/#pseudo-elements
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct PseudoElementSelectorNode {
  /// Location information in the source file
  pub loc: Loc,
  /// the name of the Pseudo-Element Selector
  pub name: String,
  /// Indicates whether the Pseudo-Element is functional: https://www.w3.org/TR/selectors-4/#functional-pseudo-class
  pub functional: bool,
  /// The children of the Pseudo-Element Selector
  pub children: Vec<PseudoElementSelectorChildren>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum PseudoElementSelectorChildren {
  // for `::cue()`
  SelectorList(Rc<RefCell<SelectorList>>),
  // for `::part()`
  Ident(Rc<RefCell<IdentNode>>),
  // for `::slotted()`
  CompoundSelector(Rc<RefCell<CompoundSelectorNode>>),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum SimpleSelector {
  TypeSelector(Rc<RefCell<TypeSelectorNode>>),
  AttrSelector(Rc<RefCell<AttrSelectorNode>>),
  ClassSelector(Rc<RefCell<ClassSelectorNode>>),
  IdSelector(Rc<RefCell<IdSelectorNode>>),
  PseudoClassSelector(Rc<RefCell<PseudoClassSelectorNode>>),
  PseudoElementSelector(Rc<RefCell<PseudoElementSelectorNode>>),
}

/// https://www.w3.org/TR/selectors-4/#compound
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct CompoundSelectorNode {
  /// Location information in the source file
  pub loc: Loc,
  /// the children of the Compound Selector
  pub children: Vec<SimpleSelector>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Combinator {
  Descendant(char),        // ' '
  Child(char),             // '>'
  NextSibling(char),       // '+'
  SubsequentSibling(char), // '~'
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ComplexSelectorChildren {
  Combinator,
  SimpleSelector,
}

/// https://www.w3.org/TR/selectors-4/#complex
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct ComplexSelectorNode {
  /// Location information in the source file
  pub loc: Loc,
  /// the children of the Complex Selector
  pub children: Vec<ComplexSelectorChildren>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Selector {
  SimpleSelector,
  CompoundSelector(Rc<RefCell<CompoundSelectorNode>>),
  ComplexSelector(Rc<RefCell<ComplexSelectorNode>>),
}

#[derive(Debug, Default, PartialEq, Serialize)]
pub struct SelectorList {
  /// Location information in the source file
  pub loc: Loc,
  /// the children of the Selector List
  pub children: Vec<Selector>,
}

impl<'s> Parser<'s> {
  pub fn parse_selector(&self) {
    unimplemented!()
  }
}
