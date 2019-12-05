// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

impl MaybeMultilineOptions for () {
    #[inline]
    fn break_line() -> Self {
        ()
    }

    #[inline]
    fn break_line_at(_len: usize) -> Self {
        ()
    }

    #[inline]
    fn no_break_line() -> Self {
        ()
    }
}

/// Trait to represent multi-line write options, that may
/// be honored or ignored depending on the writer. Types used for
/// `Writer::AtomOptions`, `Writer::BeginListOptions`,
/// `Writer::EndListOptions` and `Writer::FinishOptions` shall
/// implement this trait and ignore unsupported options.
///
/// # Example
///
/// ```
/// use sise::MaybeMultilineOptions as _;
///
/// // Function that writes some hardcoded nodes into `writer`.
/// fn write<W: sise::Writer>(mut writer: W) -> Result<(), W::Error>
/// where
///     W::AtomOptions: sise::MaybeMultilineOptions,
///     W::BeginListOptions: sise::MaybeMultilineOptions,
///     W::EndListOptions: Default,
///     W::FinishOptions: Default,
/// {
///     writer.begin_list(W::BeginListOptions::default())?;
///     writer.write_atom("example", W::AtomOptions::no_break_line())?;
///     writer.begin_list(W::BeginListOptions::break_line())?;
///     // Write the three atoms in a single line.
///     writer.write_atom("1", W::AtomOptions::no_break_line())?;
///     writer.write_atom("2", W::AtomOptions::no_break_line())?;
///     writer.write_atom("3", W::AtomOptions::no_break_line())?;
///     writer.end_list(W::EndListOptions::default())?;
///     writer.begin_list(W::BeginListOptions::break_line())?;
///     // Write the three atoms in a single line.
///     writer.write_atom("a", W::AtomOptions::no_break_line())?;
///     writer.write_atom("b", W::AtomOptions::no_break_line())?;
///     writer.write_atom("c", W::AtomOptions::no_break_line())?;
///     writer.end_list(W::EndListOptions::default())?;
///     writer.end_list(W::EndListOptions::default())?;
///     writer.finish(W::FinishOptions::default())?;
///     Ok(())
/// }
///
/// // Write with spaced writer, break line options shall
/// // be honored.
/// let style = sise::SpacedStringWriterStyle {
///    line_break: "\n",
///    indentation: " ",
/// };
///
/// let mut result = String::new();
/// let mut writer = sise::SpacedStringWriter::new(style, &mut result);
/// write(writer).unwrap();
/// let expected_result = "(example\n (1 2 3)\n (a b c)\n)";
/// assert_eq!(result, expected_result);
///
/// // Write with compact writer, options will be ignored.
/// let mut result = String::new();
/// let mut writer = sise::CompactStringWriter::new(&mut result);
/// write(writer).unwrap();
/// let expected_result = "(example (1 2 3) (a b c))";
/// assert_eq!(result, expected_result);
/// ```
pub trait MaybeMultilineOptions: Default {
    /// If supported, the node shall be written in a new line.
    fn break_line() -> Self;

    /// If supported, the node shall be written in a new line if
    /// the current line length exceeds `len`.
    fn break_line_at(len: usize) -> Self;

    /// If supported, the node shall be written in the current line.
    fn no_break_line() -> Self;
}

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
    fn write_atom(&mut self, atom: &str, opts: Self::AtomOptions) -> Result<(), Self::Error>;

    /// Begins a list.
    fn begin_list(&mut self, opts: Self::BeginListOptions) -> Result<(), Self::Error>;

    /// Ends a list.
    fn end_list(&mut self, opts: Self::EndListOptions) -> Result<(), Self::Error>;

    /// Consumes the writer and returns the result. It must be called
    /// only after the root node has been completely written.
    fn finish(self, opts: Self::FinishOptions) -> Result<Self::Result, Self::Error>;
}
