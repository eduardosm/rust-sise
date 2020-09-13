// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use alloc::string::String;
use core::convert::Infallible;

use crate::util::check_atom;
use crate::Writer;

/// Writer that writes everything into a single line string.
///
/// # Example
///
/// ```
/// use sise::Writer as _;
///
/// let mut result = String::new();
/// let mut writer = sise::CompactStringWriter::new(&mut result);
///
/// writer.begin_list(()).unwrap();
/// writer.write_atom("example", ()).unwrap();
/// writer.begin_list(()).unwrap();
/// // Write the three atoms in a single line.
/// writer.write_atom("1", ()).unwrap();
/// writer.write_atom("2", ()).unwrap();
/// writer.write_atom("3", ()).unwrap();
/// writer.end_list(()).unwrap();
/// writer.begin_list(()).unwrap();
/// // Write the three atoms in a single line.
/// writer.write_atom("a", ()).unwrap();
/// writer.write_atom("b", ()).unwrap();
/// writer.write_atom("c", ()).unwrap();
/// writer.end_list(()).unwrap();
/// writer.end_list(()).unwrap();
/// writer.finish(()).unwrap();
///
/// let expected_result = "(example (1 2 3) (a b c))";
/// assert_eq!(result, expected_result);
/// ```
pub struct CompactStringWriter<'a> {
    dst: &'a mut String,
    state: State,
}

enum State {
    Beginning,
    Writing { list_beginning: bool, depth: usize },
    Finished,
}

impl<'a> CompactStringWriter<'a> {
    #[inline]
    pub fn new(dst: &'a mut String) -> Self {
        Self {
            dst,
            state: State::Beginning,
        }
    }
}

impl<'a> Writer for CompactStringWriter<'a> {
    type Result = ();
    // To be replaced with `!` when stable.
    type Error = Infallible;
    type AtomOptions = ();
    type BeginListOptions = ();
    type EndListOptions = ();
    type FinishOptions = ();

    fn write_atom(&mut self, atom: &str, _opts: ()) -> Result<(), Infallible> {
        assert!(check_atom(atom), "invalid atom {:?}", atom);

        match self.state {
            State::Beginning => {
                self.dst.push_str(atom);
                self.state = State::Finished;
                Ok(())
            }
            State::Writing {
                ref mut list_beginning,
                ..
            } => {
                if !*list_beginning {
                    self.dst.push(' ');
                }
                self.dst.push_str(atom);
                *list_beginning = false;
                Ok(())
            }
            State::Finished => panic!("writing already finished"),
        }
    }

    fn begin_list(&mut self, _opts: ()) -> Result<(), Infallible> {
        match self.state {
            State::Beginning => {
                self.dst.push('(');
                self.state = State::Writing {
                    list_beginning: true,
                    depth: 0,
                };
                Ok(())
            }
            State::Writing {
                ref mut list_beginning,
                ref mut depth,
            } => {
                if !*list_beginning {
                    self.dst.push_str(" (");
                } else {
                    self.dst.push('(');
                }
                *list_beginning = true;
                *depth += 1;
                Ok(())
            }
            State::Finished => panic!("writing already finished"),
        }
    }

    fn end_list(&mut self, _opts: ()) -> Result<(), Infallible> {
        match self.state {
            State::Beginning => panic!("no list to end"),
            State::Writing {
                ref mut list_beginning,
                ref mut depth,
            } => {
                self.dst.push(')');
                if *depth == 0 {
                    self.state = State::Finished;
                } else {
                    *depth -= 1;
                    *list_beginning = false;
                }
                Ok(())
            }
            State::Finished => panic!("writing already finished"),
        }
    }

    fn finish(self, _opts: ()) -> Result<(), Infallible> {
        match self.state {
            State::Finished => Ok(()),
            _ => panic!("writing not finished yet"),
        }
    }
}
