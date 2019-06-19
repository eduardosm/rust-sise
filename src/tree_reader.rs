// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::convert::Infallible;

use crate::Node;
use crate::Reader;
use crate::ReadItem;
use crate::ReadItemKind;

/// Reader that allows reading from a tree of `Node`.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
/// use sise::Reader as _;
/// let root_node = sise_expr!(["test", ["1", "2", "3"]]);
/// let mut parser = sise::TreeReader::new(&root_node);
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListBeginning);
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("test"));
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListBeginning);
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("1"));
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("2"));
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("3"));
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListEnding);
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListEnding);
/// parser.finish().unwrap();
/// ```
pub struct TreeReader<'a> {
    stack: Vec<std::slice::Iter<'a, Node>>,
    current_node: CurrentNode<'a>,
}

enum CurrentNode<'a> {
    None,
    Atom(&'a str),
    List(std::slice::Iter<'a, Node>),
}

impl<'a> TreeReader<'a> {
    pub fn new(root_node: &'a Node) -> Self {
        match root_node {
            Node::Atom(atom) => Self {
                stack: Vec::new(),
                current_node: CurrentNode::Atom(atom),
            },
            Node::List(list) => Self {
                stack: vec![list.iter()],
                current_node: CurrentNode::None,
            },
        }
    }
}

impl<'a> Reader for TreeReader<'a> {
    // To be replaced with `!` when stable.
    type Error = Infallible;
    type String = &'a str;
    type Pos = ();

    fn read(&mut self) -> Result<ReadItem<&'a str, ()>, Infallible> {
        match self.current_node {
            CurrentNode::None => {
                if self.stack.is_empty() {
                    panic!("reading finished");
                }
                self.current_node = CurrentNode::List(self.stack.pop().unwrap());
                Ok(ReadItem {
                    pos: (),
                    kind: ReadItemKind::ListBeginning,
                })
            },
            CurrentNode::Atom(atom) => {
                self.current_node = CurrentNode::None;
                Ok(ReadItem {
                    pos: (),
                    kind: ReadItemKind::Atom(atom),
                })
            }
            CurrentNode::List(ref mut list) => {
                if let Some(node) = list.next() {
                    match node {
                        Node::Atom(atom) => {
                            Ok(ReadItem {
                                pos: (),
                                kind: ReadItemKind::Atom(atom),
                            })
                        }
                        Node::List(new_list) => {
                            self.stack.push(std::mem::replace(list, new_list.iter()));
                            Ok(ReadItem {
                                pos: (),
                                kind: ReadItemKind::ListBeginning,
                            })
                        }
                    }
                } else {
                    if let Some(current_node) = self.stack.pop() {
                        self.current_node = CurrentNode::List(current_node);
                    } else {
                        self.current_node = CurrentNode::None;
                    }
                    Ok(ReadItem {
                        pos: (),
                        kind: ReadItemKind::ListEnding,
                    })
                }
            }
        }
    }

    fn finish(self) -> Result<(), Infallible> {
        match self.current_node {
            CurrentNode::None if self.stack.is_empty() => Ok(()),
            _ => panic!("reading not finished"),
        }
    }
}
