// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use alloc::vec::Vec;

use crate::{Node, ParseError, ParsedItem, Parser};

/// Parses into a tree of `Node`.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
///
/// let data = "(test (1 2 3))";
/// let mut parser = sise::Parser::new(data);
/// let root_node = sise::parse_into_tree(&mut parser).unwrap();
/// // Do not forget calling `finish` on the parser.
/// parser.finish().unwrap();
/// let expected_result = sise_expr!(["test", ["1", "2", "3"]]);
/// assert_eq!(root_node, expected_result);
/// ```
///
/// It does not consume the parser, so it can also be used to parse
/// a sub-tree:
///
/// ```
/// use sise::sise_expr;
///
/// let data = "(head (1 2 3) tail)";
/// let mut parser = sise::Parser::new(data);
///
/// // Parse the head
/// assert_eq!(parser.next_item().unwrap(), sise::ParsedItem::ListStart(0));
/// assert_eq!(
///     parser.next_item().unwrap(),
///     sise::ParsedItem::Atom("head", 1),
/// );
///
/// // Parse the subtree
/// let root_node = sise::parse_into_tree(&mut parser).unwrap();
/// let expected_result = sise_expr!(["1", "2", "3"]);
/// assert_eq!(root_node, expected_result);
///
/// // Parse the tail
/// assert_eq!(
///     parser.next_item().unwrap(),
///     sise::ParsedItem::Atom("tail", 14)
/// );
/// assert_eq!(parser.next_item().unwrap(), sise::ParsedItem::ListEnd(18));
/// parser.finish().unwrap();
/// ```
pub fn parse_into_tree(parser: &mut Parser<'_>) -> Result<Node, ParseError> {
    struct StackItem {
        list_items: Vec<Node>,
    }

    enum State {
        Beginning,
        Parsing {
            stack: Vec<StackItem>,
            current: StackItem,
        },
        Finished(Node),
    }

    let mut state = State::Beginning;

    loop {
        match state {
            State::Beginning => match parser.next_item()? {
                ParsedItem::Atom(atom, _) => {
                    let root_node = Node::Atom(atom.into());
                    state = State::Finished(root_node);
                }
                ParsedItem::ListStart(_) => {
                    state = State::Parsing {
                        stack: Vec::new(),
                        current: StackItem {
                            list_items: Vec::new(),
                        },
                    };
                }
                ParsedItem::ListEnd(_) => unreachable!(),
            },
            State::Parsing {
                ref mut stack,
                ref mut current,
            } => match parser.next_item()? {
                ParsedItem::Atom(atom, _) => {
                    current.list_items.push(Node::Atom(atom.into()));
                }
                ParsedItem::ListStart(_) => {
                    let new_current = StackItem {
                        list_items: Vec::new(),
                    };
                    stack.push(core::mem::replace(current, new_current));
                }
                ParsedItem::ListEnd(_) => {
                    if let Some(previous) = stack.pop() {
                        let old_current = core::mem::replace(current, previous);
                        current.list_items.push(Node::List(old_current.list_items));
                    } else {
                        let root_node = Node::List(core::mem::take(&mut current.list_items));
                        state = State::Finished(root_node);
                    }
                }
            },
            State::Finished(root_node) => return Ok(root_node),
        }
    }
}
