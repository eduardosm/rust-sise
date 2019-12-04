// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::Node;
use crate::ReadItemKind;
use crate::Reader;

/// Reads from `reader` and builds a tree of `Node`. Unlike
/// `read_tree`, it does not return a position tree.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
/// use sise::Reader as _;
/// let data = b"(test (1 2 3))";
/// let mut parser = sise::Parser::new(data);
/// let root_node = sise::read_into_tree(&mut parser).unwrap();
/// parser.finish().unwrap();
/// let expected_result = sise_expr!(["test", ["1", "2", "3"]]);
/// assert_eq!(root_node, expected_result);
/// ```
///
/// It does not consume the reader, so it can also be used to read
/// a sub-tree:
///
/// ```
/// use sise::sise_expr;
/// use sise::Reader as _;
/// let data = b"(head (1 2 3) tail)";
/// let mut parser = sise::Parser::new(data);
///
/// // Read the head
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListBeginning);
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("head"));
///
/// // Read the subtree
/// let root_node = sise::read_into_tree(&mut parser).unwrap();
/// let expected_result = sise_expr!(["1", "2", "3"]);
/// assert_eq!(root_node, expected_result);
///
/// // Read the tail
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("tail"));
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListEnding);
/// parser.finish().unwrap();
/// ```
pub fn read_into_tree<R: Reader>(reader: &mut R) -> Result<Node, R::Error> {
    struct StackItem {
        list_items: Vec<Node>,
    }

    enum State {
        Beginning,
        Reading {
            stack: Vec<StackItem>,
            current: StackItem,
        },
        Finished(Node),
    }

    let mut state = State::Beginning;

    loop {
        match state {
            State::Beginning => {
                let item = reader.read()?;
                match item.kind {
                    ReadItemKind::Atom(atom) => {
                        let root_node = Node::Atom(atom.into());
                        state = State::Finished(root_node);
                    }
                    ReadItemKind::ListBeginning => {
                        state = State::Reading {
                            stack: Vec::new(),
                            current: StackItem {
                                list_items: Vec::new(),
                            },
                        };
                    }
                    ReadItemKind::ListEnding => panic!("unexpected list ending"),
                }
            }
            State::Reading {
                ref mut stack,
                ref mut current,
            } => {
                let item = reader.read()?;
                match item.kind {
                    ReadItemKind::Atom(atom) => {
                        current.list_items.push(Node::Atom(atom.into()));
                    }
                    ReadItemKind::ListBeginning => {
                        let new_current = StackItem {
                            list_items: Vec::new(),
                        };
                        stack.push(std::mem::replace(current, new_current));
                    }
                    ReadItemKind::ListEnding => {
                        if let Some(previous) = stack.pop() {
                            let old_current = std::mem::replace(current, previous);
                            current.list_items.push(Node::List(old_current.list_items));
                        } else {
                            let root_node =
                                Node::List(std::mem::replace(&mut current.list_items, Vec::new()));
                            state = State::Finished(root_node);
                        }
                    }
                }
            }
            State::Finished(root_node) => return Ok(root_node),
        }
    }
}
