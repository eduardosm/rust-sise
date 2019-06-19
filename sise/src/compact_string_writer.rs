// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::convert::Infallible;

use crate::Writer;
use crate::VoidWriterOptions;
use crate::util::check_atom;

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
/// writer.finish(&sise::VoidWriterOptions);
///
/// let expected_result = "(example (1 2 3) (a b c))";
/// assert_eq!(result, expected_result);
/// ```
pub struct CompactStringWriter<'a> {
    dst: &'a mut String,
    need_space: bool,
    depth: usize,
}

impl<'a> CompactStringWriter<'a> {
    #[inline]
    pub fn new(dst: &'a mut String) -> Self {
        Self {
            dst,
            need_space: false,
            depth: 1,
        }
    }
}

impl<'a> Writer for CompactStringWriter<'a> {
    type Result = ();
    // To be replaced with `!` when stable.
    type Error = Infallible;
    type AtomOptions = VoidWriterOptions;
    type BeginListOptions = VoidWriterOptions;
    type EndListOptions = VoidWriterOptions;
    type FinishOptions = VoidWriterOptions;

    fn write_atom(&mut self, atom: &str, _opts: &VoidWriterOptions) -> Result<(), Infallible> {
        assert_ne!(self.depth, 0, "writing already finished");
        assert!(check_atom(atom), "invalid atom {:?}", atom);

        if self.need_space {
            self.dst.push(' ');
        }
        self.dst.push_str(atom);

        self.need_space = true;
        // This atom is the root node, do not allow more writes.
        if self.depth == 1 {
            self.depth = 0;
        }

        Ok(())
    }

    fn begin_list(&mut self, _opts: &VoidWriterOptions) -> Result<(), Infallible> {
        assert_ne!(self.depth, 0, "writing already finished");

        if self.need_space {
            self.dst.push(' ');
        }
        self.dst.push('(');

        self.need_space = false;
        self.depth += 1;

        Ok(())
    }

    fn end_list(&mut self, _opts: &Self::EndListOptions) -> Result<(), Infallible> {
        assert_ne!(self.depth, 0, "writing already finished");

        // never write a space in this case
        self.dst.push(')');
        self.need_space = true;

        self.depth -= 1;
        if self.depth == 1 {
            self.depth = 0;
        }

        Ok(())
    }

    fn finish(self, _opts: &Self::FinishOptions) -> Result<(), Infallible> {
        assert_eq!(self.depth, 0, "writing not finished yet");
        Ok(())
    }
}
