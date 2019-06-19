// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Type to be used as `AtomOpts`, `BeginListOpts`, `EndListOpts`
/// or `FinishOpts` in `Writer` when there are no options.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct VoidWriterOptions;

/// A trait to allow writing SISE nodes into a generic destination.
pub trait Writer {
    /// Type of data returned by the writer.
    type Result;

    /// The error type that may be produced while writing.
    type Error;

    /// Type of options that can be passed to `write_atom`.
    type AtomOptions;

    /// Type of options that can be passed to `begin_list`.
    type BeginListOptions;

    /// Type of options that can be passed to `end_list`.
    type EndListOptions;

    /// Type of options that can be passed to `finish`.
    type FinishOptions;

    /// Writes an atom.
    fn write_atom(&mut self, atom: &str, opts: &Self::AtomOptions) -> Result<(), Self::Error>;

    /// Begins a list.
    fn begin_list(&mut self, opts: &Self::BeginListOptions) -> Result<(), Self::Error>;

    /// Ends a list.
    fn end_list(&mut self, opts: &Self::EndListOptions) -> Result<(), Self::Error>;

    /// Consumes the writer and returns the result. It must be called
    /// only after the root node has been completely written.
    fn finish(self, opts: &Self::FinishOptions) -> Result<Self::Result, Self::Error>;
}
