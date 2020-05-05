// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::convert::Infallible;

use crate::Node;
use crate::ReadItem;
use crate::ReadItemKind;
use crate::Reader;

/// Reader that allows reading from a tree of `Node`.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
/// use sise::Reader as _;
/// let root_node = sise_expr!(["test", ["1", "2", "3"]]);
/// let mut reader = sise::TreeReader::new(&root_node);
/// assert_eq!(
///     reader.read().unwrap().kind,
///     sise::ReadItemKind::ListBeginning,
/// );
/// assert_eq!(
///     reader.read().unwrap().kind,
///     sise::ReadItemKind::Atom("test"),
/// );
/// assert_eq!(
///     reader.read().unwrap().kind,
///     sise::ReadItemKind::ListBeginning,
/// );
/// assert_eq!(reader.read().unwrap().kind, sise::ReadItemKind::Atom("1"));
/// assert_eq!(reader.read().unwrap().kind, sise::ReadItemKind::Atom("2"));
/// assert_eq!(reader.read().unwrap().kind, sise::ReadItemKind::Atom("3"));
/// assert_eq!(reader.read().unwrap().kind, sise::ReadItemKind::ListEnding);
/// assert_eq!(reader.read().unwrap().kind, sise::ReadItemKind::ListEnding);
/// reader.finish().unwrap();
/// ```
pub struct TreeReader<'a> {
    state: State<'a>,
}

enum State<'a> {
    Beginning(&'a Node),
    Reading {
        stack: Vec<std::slice::Iter<'a, Node>>,
        current_list: std::slice::Iter<'a, Node>,
    },
    Finished,
}

impl<'a> TreeReader<'a> {
    #[inline]
    pub fn new(root_node: &'a Node) -> Self {
        Self {
            state: State::Beginning(root_node),
        }
    }
}

impl<'a> Reader for TreeReader<'a> {
    // To be replaced with `!` when stable.
    type Error = Infallible;
    type String = &'a str;
    type Pos = ();

    fn read(&mut self) -> Result<ReadItem<&'a str, ()>, Infallible> {
        match self.state {
            State::Beginning(root_node) => match root_node {
                Node::Atom(atom) => {
                    self.state = State::Finished;
                    Ok(ReadItem {
                        pos: (),
                        kind: ReadItemKind::Atom(atom),
                    })
                }
                Node::List(list) => {
                    self.state = State::Reading {
                        stack: Vec::new(),
                        current_list: list.iter(),
                    };
                    Ok(ReadItem {
                        pos: (),
                        kind: ReadItemKind::ListBeginning,
                    })
                }
            },
            State::Reading {
                ref mut stack,
                ref mut current_list,
            } => {
                if let Some(node) = current_list.next() {
                    match node {
                        Node::Atom(atom) => Ok(ReadItem {
                            pos: (),
                            kind: ReadItemKind::Atom(atom),
                        }),
                        Node::List(list) => {
                            stack.push(std::mem::replace(current_list, list.iter()));
                            Ok(ReadItem {
                                pos: (),
                                kind: ReadItemKind::ListBeginning,
                            })
                        }
                    }
                } else {
                    if let Some(parent_list) = stack.pop() {
                        *current_list = parent_list;
                    } else {
                        self.state = State::Finished;
                    }
                    Ok(ReadItem {
                        pos: (),
                        kind: ReadItemKind::ListEnding,
                    })
                }
            }
            State::Finished => panic!("reading already finished"),
        }
    }

    fn finish(self) -> Result<(), Infallible> {
        match self.state {
            State::Finished => Ok(()),
            _ => panic!("reading not finished yet"),
        }
    }
}
