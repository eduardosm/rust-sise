// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::Node;
use crate::PosTree;
use crate::PosTreeList;
use crate::Reader;
use crate::ReadItemKind;

/// Reads from `reader` and builds a tree of `Node`. It also returns
/// a position map that allows to fetch the source position of each
/// node.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
/// use sise::Reader as _;
/// let data = b"(test (1 2 3))";
/// let mut parser = sise::Parser::new(data);
/// let (root_node, pos_tree) = sise::read_into_tree(&mut parser).unwrap();
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
/// let data = b"(test-1 (1 2 3) test-2)";
/// let mut parser = sise::Parser::new(data);
///
/// // Read the head
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListBeginning);
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("test-1"));
///
/// // Read the subtree
/// let (root_node, pos_tree) = sise::read_into_tree(&mut parser).unwrap();
/// let expected_result = sise_expr!(["1", "2", "3"]);
/// assert_eq!(root_node, expected_result);
///
/// // Read the tail
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("test-2"));
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListEnding);
/// parser.finish().unwrap();
/// ```
pub fn read_into_tree<R: Reader>(reader: &mut R) -> Result<(Node, PosTree<R::Pos>), R::Error> {
    let item = reader.read()?;
    match item.kind {
        ReadItemKind::Atom(atom) => {
            let root_node = Node::Atom(atom.into());
            let root_pos_tree = PosTree { pos: item.pos, list: None };
            Ok((root_node, root_pos_tree))
        }
        ReadItemKind::ListBeginning => {
            struct StackItem<P> {
                beginning_pos: P,
                list_items: Vec<Node>,
                pos_tree_items: Vec<PosTree<P>>,
            }
            let mut stack = Vec::new();
            let mut current = StackItem {
                beginning_pos: item.pos,
                list_items: Vec::new(),
                pos_tree_items: Vec::new(),
            };
            loop {
                let item = reader.read()?;
                match item.kind {
                    ReadItemKind::Atom(atom) => {
                        current.list_items.push(Node::Atom(atom.into()));
                        current.pos_tree_items.push(PosTree { pos: item.pos, list: None });
                    }
                    ReadItemKind::ListBeginning => {
                        stack.push(current);
                        current = StackItem {
                            beginning_pos: item.pos,
                            list_items: Vec::new(),
                            pos_tree_items: Vec::new(),
                        };
                    }
                    ReadItemKind::ListEnding => {
                        if let Some(mut previous) = stack.pop() {
                            previous.list_items.push(Node::List(current.list_items));
                            previous.pos_tree_items.push(PosTree {
                                pos: current.beginning_pos,
                                list: Some(PosTreeList {
                                    items: current.pos_tree_items,
                                    ending_pos: item.pos,
                                }),
                            });
                            current = previous;
                        } else {
                            let root_node = Node::List(current.list_items);
                            let root_pos_tree = PosTree {
                                pos: current.beginning_pos,
                                list: Some(PosTreeList {
                                    items: current.pos_tree_items,
                                    ending_pos: item.pos,
                                }),
                            };
                            return Ok((root_node, root_pos_tree));
                        }
                    }
                }
            }
        }
        ReadItemKind::ListEnding => unreachable!(),
    }
}

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
/// let root_node = sise::read_into_tree_without_pos(&mut parser).unwrap();
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
/// let root_node = sise::read_into_tree_without_pos(&mut parser).unwrap();
/// let expected_result = sise_expr!(["1", "2", "3"]);
/// assert_eq!(root_node, expected_result);
///
/// // Read the tail
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("tail"));
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListEnding);
/// parser.finish().unwrap();
/// ```
pub fn read_into_tree_without_pos<R: Reader>(reader: &mut R) -> Result<Node, R::Error> {
    let item = reader.read()?;
    match item.kind {
        ReadItemKind::Atom(atom) => {
            let root_node = Node::Atom(atom.into());
            Ok(root_node)
        }
        ReadItemKind::ListBeginning => {
            struct StackItem {
                list_items: Vec<Node>,
            }
            let mut stack = Vec::new();
            let mut current = StackItem {
                list_items: Vec::new(),
            };
            loop {
                let item = reader.read()?;
                match item.kind {
                    ReadItemKind::Atom(atom) => {
                        current.list_items.push(Node::Atom(atom.into()));
                    }
                    ReadItemKind::ListBeginning => {
                        stack.push(current);
                        current = StackItem {
                            list_items: Vec::new(),
                        };
                    }
                    ReadItemKind::ListEnding => {
                        if let Some(mut previous) = stack.pop() {
                            previous.list_items.push(Node::List(current.list_items));
                            current = previous;
                        } else {
                            let root_node = Node::List(current.list_items);
                            return Ok(root_node);
                        }
                    }
                }
            }
        }
        ReadItemKind::ListEnding => unreachable!(),
    }
}
