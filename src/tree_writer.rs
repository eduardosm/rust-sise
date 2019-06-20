// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::convert::Infallible;

use crate::Node;
use crate::Writer;
use crate::VoidWriterOptions;
use crate::check_atom;

/// A writer that creates a tree of `Node`.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
/// use sise::Writer as _;
///
/// let mut writer = sise::TreeWriter::new();
///
/// writer.begin_list(&sise::VoidWriterOptions).unwrap();
/// writer.write_atom("example", &sise::VoidWriterOptions).unwrap();
/// writer.begin_list(&sise::VoidWriterOptions).unwrap();
/// // Write the three atoms in a single line.
/// writer.write_atom("1", &sise::VoidWriterOptions).unwrap();
/// writer.write_atom("2", &sise::VoidWriterOptions).unwrap();
/// writer.write_atom("3", &sise::VoidWriterOptions).unwrap();
/// writer.end_list(&sise::VoidWriterOptions).unwrap();
/// writer.begin_list(&sise::VoidWriterOptions).unwrap();
/// // Write the three atoms in a single line.
/// writer.write_atom("a", &sise::VoidWriterOptions).unwrap();
/// writer.write_atom("b", &sise::VoidWriterOptions).unwrap();
/// writer.write_atom("c", &sise::VoidWriterOptions).unwrap();
/// writer.end_list(&sise::VoidWriterOptions).unwrap();
/// writer.end_list(&sise::VoidWriterOptions).unwrap();
/// let result = writer.finish(&sise::VoidWriterOptions).unwrap();
///
/// let expected_result = sise_expr!(["example", ["1", "2", "3"], ["a", "b", "c"]]);
/// assert_eq!(result, expected_result);
/// ```
pub struct TreeWriter {
    state: State,
}

enum State {
    Beginning,
    Writing {
        stack: Vec<Vec<Node>>,
        current_list: Vec<Node>,
    },
    Finished(Node),
}

impl TreeWriter {
    #[inline]
    pub fn new() -> Self {
        Self {
            state: State::Beginning,
        }
    }
}

impl Default for TreeWriter {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Writer for TreeWriter {
    type Result = Node;
    // To be replaced with `!` when stable.
    type Error = Infallible;
    type AtomOptions = VoidWriterOptions;
    type BeginListOptions = VoidWriterOptions;
    type EndListOptions = VoidWriterOptions;
    type FinishOptions = VoidWriterOptions;

    fn write_atom(&mut self, atom: &str, _opts: &VoidWriterOptions) -> Result<(), Infallible> {
        assert!(check_atom(atom), "invalid atom {:?}", atom);

        match self.state {
            State::Beginning => {
                self.state = State::Finished(Node::Atom(atom.to_string()));
                Ok(())
            },
            State::Writing { ref mut current_list, .. } => {
                current_list.push(Node::Atom(atom.to_string()));
                Ok(())
            }
            State::Finished(_) => panic!("writing already finished"),
        }
    }

    fn begin_list(&mut self, _opts: &VoidWriterOptions) -> Result<(), Infallible> {
        match self.state {
            State::Beginning => {
                self.state = State::Writing {
                    stack: Vec::new(),
                    current_list: Vec::new(),
                };
                Ok(())
            }
            State::Writing { ref mut stack, ref mut current_list } => {
                stack.push(std::mem::replace(current_list, Vec::new()));
                Ok(())
            }
            State::Finished(_) => panic!("writing already finished"),
        }
    }

    fn end_list(&mut self, _opts: &VoidWriterOptions) -> Result<(), Infallible> {
        match self.state {
            State::Beginning => panic!("no list to end"),
            State::Writing { ref mut stack, ref mut current_list } => {
                if let Some(parent_list) = stack.pop() {
                    let child_list = std::mem::replace(current_list, parent_list);
                    current_list.push(Node::List(child_list));
                } else {
                    let list = std::mem::replace(current_list, Vec::new());
                    self.state = State::Finished(Node::List(list));
                }
                Ok(())
            }
            State::Finished(_) => panic!("writing already finished"),
        }
    }

    fn finish(self, _opts: &VoidWriterOptions) -> Result<Node, Infallible> {
        match self.state {
            State::Finished(node) => Ok(node),
            _ => panic!("writing already finished"),
        }
    }
}
