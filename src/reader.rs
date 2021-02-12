// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use alloc::string::String;

/// A trait to allow reading SISE nodes from a generic source.
///
/// Readers that implement this trait produces a sequence of `ReadItem`,
/// that compose a S-expression document.
///
/// # Example
///
/// See the example from `Parser`.
pub trait Reader {
    /// The error type that may be produced while reading.
    type Error;

    /// A type that can be either borrowed as a `str` or converted
    /// into an owned `String`.
    type String: Into<String> + AsRef<str>;

    /// A type that represent the position of a node.
    type Pos;

    /// Reads from the source and returns a `ReadItem`.
    ///
    /// # Panics
    ///
    /// Panics if the root node has been completely read. If the root
    /// node is a list, this means the `ListEnding` that matches the
    /// first `ListBeginning` has already been read. If the root node
    /// is an atom, this means that such atom has already been read.
    fn read(&mut self) -> Result<ReadItem<Self::String, Self::Pos>, Self::Error>;

    /// Finishes the reader, consuming it. It must be called only after
    /// the root node has been completely read. It may return an error
    /// if an error is encountered after the root node (e.g., trailing
    /// tokens at the end of the file).
    ///
    /// # Panics
    ///
    /// Panics if the root node has not been completely read.
    fn finish(self) -> Result<(), Self::Error>;
}

/// Items produced by readers that implement `Reader`.
///
/// The `S` type parameter is depends on the reader and is used to
/// represent strings with the atom values. For example, `S` can be
/// `String` or `Box<str>` to hold owned strings or `&str` to hold
/// borrowed strings. It must implement `Into<String>` and `AsRef<str>`,
/// so an owned or borrowed string can be easily obtained from `S`.
///
/// The `P` type parameter is used to represent the position of the
/// read items and also depends on the reader. For example, it can
/// be a scalar to specify the byte index in a file, or a `()` if position
/// information is not available.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadItem<S: Into<String> + AsRef<str>, P> {
    /// Position of the read item.
    pub pos: P,
    /// The kind of item, including its value if it is an atom.
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
    /// Returns whether `self` is an `Atom`.
    #[inline]
    pub fn is_atom(&self) -> bool {
        matches!(self, ReadItemKind::Atom(_))
    }

    /// If `self` is an atom, returns it value, otherwise
    /// returns `None`.
    #[inline]
    pub fn as_atom(&self) -> Option<&S> {
        match self {
            ReadItemKind::Atom(atom) => Some(atom),
            _ => None,
        }
    }

    /// Consumes `self` and returns its value if it is an atom,
    /// otherwise returns `None`.
    #[inline]
    pub fn into_atom(self) -> Option<S> {
        match self {
            ReadItemKind::Atom(atom) => Some(atom),
            _ => None,
        }
    }

    /// Returns whether `self` is a `ListBeginning`.
    #[inline]
    pub fn is_list_beginning(&self) -> bool {
        matches!(self, ReadItemKind::ListBeginning)
    }

    /// Returns whether `self` is a `ListEnding`.
    #[inline]
    pub fn is_list_ending(&self) -> bool {
        matches!(self, ReadItemKind::ListEnding)
    }
}
