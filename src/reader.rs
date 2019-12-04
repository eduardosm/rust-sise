// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// A trait to allow reading SISE nodes from a generic source. See
/// example from `Parser`.
pub trait Reader {
    /// The error type that may be produced while reading.
    type Error;

    /// A type that can be either a borrowed as a `str` or converted
    /// into an owned `String`.
    type String: Into<String> + AsRef<str>;

    /// A type that represent the position of a node.
    type Pos;

    /// Reads from the source. It shall panic if the root node has
    /// been completely read (e.g., the `ListEnding` that matches the
    /// first `ListBeginning` has been read).
    fn read(&mut self) -> Result<ReadItem<Self::String, Self::Pos>, Self::Error>;

    /// Finished the reader, consuming it. It must be called only after
    /// the root node has been completely read. It may return an error
    /// if an error is encountered after the root node (e.g., trailing
    /// tokens at the end of the file).
    fn finish(self) -> Result<(), Self::Error>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadItem<S: Into<String> + AsRef<str>, P> {
    pub pos: P,
    pub kind: ReadItemKind<S>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReadItemKind<S: Into<String> + AsRef<str>> {
    /// An atom, as documented in `Node::Atom`.
    Atom(S),
    /// The beginning of a list (i.e., a `(`).
    ListBeginning,
    /// The ending of a list (i.e., a `)`).
    ListEnding,
}

impl<S: Into<String> + AsRef<str>> ReadItemKind<S> {
    #[inline]
    pub fn is_atom(&self) -> bool {
        match self {
            ReadItemKind::Atom(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn as_atom(&self) -> Option<&S> {
        match self {
            ReadItemKind::Atom(atom) => Some(atom),
            _ => None,
        }
    }

    #[inline]
    pub fn into_atom(self) -> Option<S> {
        match self {
            ReadItemKind::Atom(atom) => Some(atom),
            _ => None,
        }
    }
}
