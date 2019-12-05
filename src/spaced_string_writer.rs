// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::convert::Infallible;

use crate::check_atom;
use crate::MaybeMultilineOptions;
use crate::Writer;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpacedStringWriterStyle<'a> {
    pub line_break: &'a str,
    pub indentation: &'a str,
}

#[derive(Copy, Clone, Debug)]
pub struct SpacedStringWriterNodeOptions {
    pub break_line_len: usize,
}

impl SpacedStringWriterNodeOptions {
    #[inline]
    pub const fn break_line() -> Self {
        Self { break_line_len: 0 }
    }

    #[inline]
    pub const fn no_break_line() -> Self {
        Self {
            break_line_len: usize::max_value(),
        }
    }
}

impl Default for SpacedStringWriterNodeOptions {
    /// Returns `SpacedStringWriterNodeOptions::break_line()`
    #[inline]
    fn default() -> Self {
        Self::break_line()
    }
}

impl MaybeMultilineOptions for SpacedStringWriterNodeOptions {
    #[inline]
    fn break_line() -> Self {
        Self { break_line_len: 0 }
    }

    #[inline]
    fn break_line_at(len: usize) -> Self {
        Self {
            break_line_len: len,
        }
    }

    #[inline]
    fn no_break_line() -> Self {
        Self {
            break_line_len: usize::max_value(),
        }
    }
}

/// Writer that writes everything into a multi-line string.
///
/// # Example
///
/// ```
/// use sise::Writer as _;
///
/// let style = sise::SpacedStringWriterStyle {
///    line_break: "\n",
///    indentation: " ",
/// };
///
/// let mut result = String::new();
/// let mut writer = sise::SpacedStringWriter::new(style, &mut result);
///
/// writer.begin_list(sise::SpacedStringWriterNodeOptions::default()).unwrap();
/// writer.write_atom("example", sise::SpacedStringWriterNodeOptions::default()).unwrap();
/// writer.begin_list(sise::SpacedStringWriterNodeOptions::default()).unwrap();
/// // Write the three atoms in a single line.
/// writer.write_atom("1", sise::SpacedStringWriterNodeOptions::default()).unwrap();
/// writer.write_atom("2", sise::SpacedStringWriterNodeOptions::default()).unwrap();
/// writer.write_atom("3", sise::SpacedStringWriterNodeOptions::default()).unwrap();
/// writer.end_list(()).unwrap();
/// writer.begin_list(sise::SpacedStringWriterNodeOptions::default()).unwrap();
/// // Write the three atoms in a single line.
/// writer.write_atom("a", sise::SpacedStringWriterNodeOptions::default()).unwrap();
/// writer.write_atom("b", sise::SpacedStringWriterNodeOptions::default()).unwrap();
/// writer.write_atom("c", sise::SpacedStringWriterNodeOptions::default()).unwrap();
/// writer.end_list(()).unwrap();
/// writer.end_list(()).unwrap();
/// writer.finish(()).unwrap();
///
/// let expected_result = "(\n example\n (\n  1\n  2\n  3\n )\n (\n  a\n  b\n  c\n )\n)";
/// assert_eq!(result, expected_result);
/// ```
///
/// Using `SpacedStringWriterNodeOptions` allows to write specified nodes
/// in a single line:
///
/// ```
/// use sise::Writer as _;
///
/// let style = sise::SpacedStringWriterStyle {
///    line_break: "\n",
///    indentation: " ",
/// };
///
/// let mut result = String::new();
/// let mut writer = sise::SpacedStringWriter::new(style, &mut result);
///
/// writer.begin_list(sise::SpacedStringWriterNodeOptions::default()).unwrap();
/// writer.write_atom("example", sise::SpacedStringWriterNodeOptions::no_break_line()).unwrap();
/// writer.begin_list(sise::SpacedStringWriterNodeOptions::break_line()).unwrap();
/// // Write the three atoms in a single line.
/// writer.write_atom("1", sise::SpacedStringWriterNodeOptions::no_break_line()).unwrap();
/// writer.write_atom("2", sise::SpacedStringWriterNodeOptions::no_break_line()).unwrap();
/// writer.write_atom("3", sise::SpacedStringWriterNodeOptions::no_break_line()).unwrap();
/// writer.end_list(()).unwrap();
/// writer.begin_list(sise::SpacedStringWriterNodeOptions::break_line()).unwrap();
/// // Write the three atoms in a single line.
/// writer.write_atom("a", sise::SpacedStringWriterNodeOptions::no_break_line()).unwrap();
/// writer.write_atom("b", sise::SpacedStringWriterNodeOptions::no_break_line()).unwrap();
/// writer.write_atom("c", sise::SpacedStringWriterNodeOptions::no_break_line()).unwrap();
/// writer.end_list(()).unwrap();
/// writer.end_list(()).unwrap();
/// writer.finish(()).unwrap();
///
/// let expected_result = "(example\n (1 2 3)\n (a b c)\n)";
/// assert_eq!(result, expected_result);
/// ```
pub struct SpacedStringWriter<'a, 'b> {
    style: SpacedStringWriterStyle<'a>,
    dst: &'b mut String,
    state: State,
}

enum State {
    Beginning,
    Writing(WritingState),
    Finished,
}

struct WritingState {
    stack: Vec<StackItem>,
    list_beginning: bool,
    current_list_line_broken: bool,
    line_len: usize,
}

struct StackItem {
    line_broken: bool,
}

impl<'a, 'b> SpacedStringWriter<'a, 'b> {
    pub fn new(style: SpacedStringWriterStyle<'a>, dst: &'b mut String) -> Self {
        Self {
            style,
            dst,
            state: State::Beginning,
        }
    }

    fn write_indent(indentation: &str, n: usize, dst: &mut String) -> usize {
        let mut len = 0;
        for _ in 0..n {
            dst.push_str(indentation);
            len += indentation.len();
        }
        len
    }
}

impl<'a, 'b> Writer for SpacedStringWriter<'a, 'b> {
    type Result = ();
    // To be replaced with `!` when stable.
    type Error = Infallible;
    type AtomOptions = SpacedStringWriterNodeOptions;
    type BeginListOptions = SpacedStringWriterNodeOptions;
    type EndListOptions = ();
    type FinishOptions = ();

    fn write_atom(
        &mut self,
        atom: &str,
        opts: SpacedStringWriterNodeOptions,
    ) -> Result<(), Infallible> {
        assert!(check_atom(atom), "invalid atom {:?}", atom);

        match self.state {
            State::Beginning => {
                self.dst.push_str(atom);
                self.state = State::Finished;
            }
            State::Writing(ref mut state) => {
                if state.line_len < opts.break_line_len {
                    if !state.list_beginning {
                        self.dst.push(' ');
                        state.line_len += 1;
                    }
                    self.dst.push_str(atom);
                    state.line_len += atom.len();
                } else {
                    self.dst.push_str(self.style.line_break);
                    let indent_len =
                        Self::write_indent(self.style.indentation, state.stack.len() + 1, self.dst);
                    self.dst.push_str(atom);
                    state.current_list_line_broken = true;
                    state.line_len = indent_len + atom.len();
                }
                state.list_beginning = false;
            }
            State::Finished => panic!("writing already finished"),
        }

        Ok(())
    }

    fn begin_list(&mut self, opts: SpacedStringWriterNodeOptions) -> Result<(), Infallible> {
        match self.state {
            State::Beginning => {
                self.dst.push('(');
                self.state = State::Writing(WritingState {
                    stack: Vec::new(),
                    list_beginning: true,
                    current_list_line_broken: false,
                    line_len: 1,
                });
            }
            State::Writing(ref mut state) => {
                if state.line_len < opts.break_line_len {
                    if !state.list_beginning {
                        self.dst.push(' ');
                        state.line_len += 1;
                    }
                    self.dst.push('(');
                    state.line_len += 1;
                } else {
                    self.dst.push_str(self.style.line_break);
                    state.line_len =
                        Self::write_indent(self.style.indentation, state.stack.len() + 1, self.dst);
                    self.dst.push('(');
                    state.current_list_line_broken = true;
                    state.line_len += 1;
                }

                state.stack.push(StackItem {
                    line_broken: state.current_list_line_broken,
                });
                state.list_beginning = true;
                state.current_list_line_broken = false;
            }
            State::Finished => panic!("writing already finished"),
        }

        Ok(())
    }

    fn end_list(&mut self, _opts: ()) -> Result<(), Infallible> {
        match self.state {
            State::Beginning => panic!("no list to end"),
            State::Writing(ref mut state) => {
                if state.current_list_line_broken {
                    self.dst.push_str(self.style.line_break);
                    state.line_len =
                        Self::write_indent(self.style.indentation, state.stack.len(), self.dst);
                }
                self.dst.push(')');
                state.line_len += 1;

                if let Some(previous) = state.stack.pop() {
                    state.current_list_line_broken |= previous.line_broken;
                    state.list_beginning = false;
                } else {
                    self.state = State::Finished;
                }
            }
            State::Finished => panic!("writing already finished"),
        }

        Ok(())
    }

    fn finish(self, _opts: ()) -> Result<(), Infallible> {
        match self.state {
            State::Finished => Ok(()),
            _ => panic!("writing not finished yet"),
        }
    }
}
