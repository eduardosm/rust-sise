// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::MaybeMultilineOptions;
use crate::Node;
use crate::Writer;

pub trait WriteFromTreeAtomOptions {
    fn list_beginning() -> Self;
    fn non_list_beginning() -> Self;
}

impl<T: MaybeMultilineOptions> WriteFromTreeAtomOptions for T {
    #[inline]
    fn list_beginning() -> Self {
        Self::no_break_line()
    }

    #[inline]
    fn non_list_beginning() -> Self {
        Self::break_line()
    }
}

/// Write the tree of nodes `root_node` into `writer`.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
/// use sise::Writer as _;
///
/// let tree = sise_expr!(["example", ["1", "2", "3"], ["a", "b", "c"]]);
///
/// let mut result = String::new();
/// let mut writer = sise::CompactStringWriter::new(&mut result);
///
/// sise::write_from_tree(&mut writer, &tree).unwrap();
/// writer.finish(()).unwrap();
///
/// let expected_result = "(example (1 2 3) (a b c))";
/// assert_eq!(result, expected_result);
/// ```
///
/// If you use `SpacedStringWriter`, atoms at the beginning of a list
/// will be placed in the same line as the openning `(`:
///
/// ```
/// use sise::sise_expr;
/// use sise::Writer as _;
///
/// let tree = sise_expr!(["example", ["1", "2", "3"], ["a", "b", "c"]]);
///
/// let style = sise::SpacedStringWriterStyle {
///    line_break: "\n",
///    indentation: " ",
/// };
///
/// let mut result = String::new();
/// let mut writer = sise::SpacedStringWriter::new(style, &mut result);
///
/// sise::write_from_tree(&mut writer, &tree).unwrap();
/// writer.finish(()).unwrap();
///
/// let expected_result = "(example\n (1\n  2\n  3\n )\n (a\n  b\n  c\n )\n)";
/// assert_eq!(result, expected_result);
/// ```
///
/// It does not consume the writer, so it can also be used to write
/// a sub-tree:
///
/// ```
/// use sise::sise_expr;
/// use sise::Writer as _;
///
/// let tree = sise_expr!(["1", "2", "3"]);
///
/// let mut result = String::new();
/// let mut writer = sise::CompactStringWriter::new(&mut result);
///
/// // Write the head
/// writer.begin_list(()).unwrap();
/// writer.write_atom("head", ()).unwrap();
///
/// // Write the subtree
/// sise::write_from_tree(&mut writer, &tree).unwrap();
///
/// // Write the tail
/// writer.write_atom("tail", ()).unwrap();
/// writer.end_list(()).unwrap();
/// writer.finish(()).unwrap();
///
/// let expected_result = "(head (1 2 3) tail)";
/// assert_eq!(result, expected_result);
/// ```
pub fn write_from_tree<W: Writer>(writer: &mut W, root_node: &Node) -> Result<(), W::Error>
where
    W::AtomOptions: Default + WriteFromTreeAtomOptions,
    W::BeginListOptions: Default,
    W::EndListOptions: Default,
{
    enum State<'a> {
        Beginning(&'a Node),
        Writing {
            stack: Vec<std::slice::Iter<'a, Node>>,
            current_list: std::slice::Iter<'a, Node>,
            list_beginning: bool,
        },
        Finished,
    }

    let mut state = State::Beginning(root_node);

    loop {
        match state {
            State::Beginning(node) => match node {
                Node::Atom(atom) => {
                    writer.write_atom(atom, W::AtomOptions::default())?;
                    state = State::Finished;
                }
                Node::List(list) => {
                    writer.begin_list(W::BeginListOptions::default())?;
                    state = State::Writing {
                        stack: Vec::new(),
                        current_list: list.iter(),
                        list_beginning: true,
                    };
                }
            },
            State::Writing {
                ref mut stack,
                ref mut current_list,
                ref mut list_beginning,
            } => {
                if let Some(node) = current_list.next() {
                    match node {
                        Node::Atom(atom) => {
                            if *list_beginning {
                                writer.write_atom(atom, W::AtomOptions::list_beginning())?;
                            } else {
                                writer.write_atom(atom, W::AtomOptions::non_list_beginning())?;
                            }
                            *list_beginning = false;
                        }
                        Node::List(list) => {
                            writer.begin_list(W::BeginListOptions::default())?;
                            stack.push(std::mem::replace(current_list, list.iter()));
                            *list_beginning = true;
                        }
                    }
                } else {
                    writer.end_list(W::EndListOptions::default())?;
                    if let Some(parent_list) = stack.pop() {
                        *current_list = parent_list;
                        *list_beginning = false;
                    } else {
                        state = State::Finished;
                    }
                }
            }
            State::Finished => return Ok(()),
        }
    }
}
