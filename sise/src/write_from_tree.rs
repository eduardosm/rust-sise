// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::Node;
use crate::Writer;

pub trait WriteFromTreeAtomOptions {
    fn list_beginning() -> Self;
    fn non_list_beginning() -> Self;
}

impl WriteFromTreeAtomOptions for crate::VoidWriterOptions {
    #[inline]
    fn list_beginning() -> Self {
        Self
    }

    #[inline]
    fn non_list_beginning() -> Self {
        Self
    }
}

impl WriteFromTreeAtomOptions for crate::SpacedStringWriterNodeOptions {
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
/// writer.finish(&sise::VoidWriterOptions);
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
/// writer.finish(&sise::VoidWriterOptions);
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
/// writer.begin_list(&sise::VoidWriterOptions).unwrap();
/// writer.write_atom("head", &sise::VoidWriterOptions).unwrap();
///
/// // Write the subtree
/// sise::write_from_tree(&mut writer, &tree).unwrap();
///
/// // Write the tail
/// writer.write_atom("tail", &sise::VoidWriterOptions).unwrap();
/// writer.end_list(&sise::VoidWriterOptions).unwrap();
/// writer.finish(&sise::VoidWriterOptions);
///
/// let expected_result = "(head (1 2 3) tail)";
/// assert_eq!(result, expected_result);
/// ```
pub fn write_from_tree<W: Writer>(writer: &mut W, root_node: &Node) -> Result<(), W::Error>
    where W::AtomOptions: Default + WriteFromTreeAtomOptions,
          W::BeginListOptions: Default,
          W::EndListOptions: Default,
{
    let single_atom_options = W::AtomOptions::default();
    let list_beginning_atom_options = W::AtomOptions::list_beginning();
    let non_list_beginning_atom_options = W::AtomOptions::non_list_beginning();
    let begin_list_options = W::BeginListOptions::default();
    let end_list_options = W::EndListOptions::default();

    match root_node {
        Node::Atom(atom) => writer.write_atom(atom, &single_atom_options)?,
        Node::List(list) => {
            writer.begin_list(&begin_list_options)?;

            struct StackItem<'a> {
                list_iter: std::slice::Iter<'a, Node>,
            }
            let mut stack = Vec::new();
            let mut current = StackItem {
                list_iter: list.iter(),
            };
            let mut list_beginning = true;
            loop {
                if let Some(item) = current.list_iter.next() {
                    match item {
                        Node::Atom(atom) => {
                            if list_beginning {
                                writer.write_atom(atom, &list_beginning_atom_options)?;
                            } else {
                                writer.write_atom(atom, &non_list_beginning_atom_options)?;
                            }
                            list_beginning = false;
                        },
                        Node::List(list) => {
                            writer.begin_list(&begin_list_options)?;
                            stack.push(current);
                            current = StackItem {
                                list_iter: list.iter(),
                            };
                            list_beginning = true;
                        }
                    }
                } else {
                    writer.end_list(&end_list_options)?;
                    if let Some(previous) = stack.pop() {
                        current = previous;
                        list_beginning = false;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
