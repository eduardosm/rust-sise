// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! SISE (Simple S-expression) is a file format based on S-expressions.

use std::collections::HashMap;

#[cfg(test)]
mod tests;

/// Macro to define values with a lighter syntax.
///
/// Example
/// -------
/// ```
/// use sise::sise_expr;
///
/// // atom
/// let value1 = Box::new(sise::Node::Atom(String::from("atom")));
/// let value2 = sise_expr!("atom");
/// assert_eq!(value1, value2);
///
/// // ()
/// let value1 = Box::new(sise::Node::List(vec![]));
/// let value2 = sise_expr!([]);
/// assert_eq!(value1, value2);
///
/// // (atom)
/// let value1 = Box::new(sise::Node::List(vec![
///     Box::new(sise::Node::Atom(String::from("atom")))
/// ]));
/// let value2 = sise_expr!(["atom"]);
/// assert_eq!(value1, value2);
///
/// // (atom (1 2 3) (a b c))
/// let value1 = Box::new(sise::Node::List(vec![
///     Box::new(sise::Node::Atom(String::from("atom"))),
///     Box::new(sise::Node::List(vec![
///         Box::new(sise::Node::Atom(String::from("1"))),
///         Box::new(sise::Node::Atom(String::from("2"))),
///         Box::new(sise::Node::Atom(String::from("3"))),
///     ])),
///     Box::new(sise::Node::List(vec![
///         Box::new(sise::Node::Atom(String::from("a"))),
///         Box::new(sise::Node::Atom(String::from("b"))),
///         Box::new(sise::Node::Atom(String::from("c"))),
///     ])),
/// ]));
/// let value2 = sise_expr!(["atom", ["1", "2", "3"], ["a", "b", "c"]]);
/// assert_eq!(value1, value2);
/// ```
#[macro_export]
macro_rules! sise_expr {
    ([$($item:tt),*]) => { Box::new($crate::Node::List(vec![$(sise_expr!($item)),*])) };
    ([$($item:tt,)*]) => { Box::new($crate::Node::List(vec![$(sise_expr!($item)),*])) };
    ($node:expr) => { Box::new($crate::Node::from($node)) };
}

/// Represents a position in a text file.
///
/// `line` and `column` begin to count with zero.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    pub line: u32,
    pub column: u32,
}

impl Pos {
    #[inline]
    pub fn new(line: u32, column: u32) -> Self {
        Self { line: line, column: column }
    }
}

/// Wrapper whose `Display` implementation prints
/// `self.0 + 1`, taking care of overflow.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReprPosValue(pub u32);

impl std::fmt::Display for ReprPosValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0.checked_add(1) {
            Some(value) => std::fmt::Display::fmt(&value, f),
            None => f.write_str("4294967296"),
        }
    }
}

/// Maps nodes with their positions in the original text file.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PosTree {
    pub pos: Pos,
    pub children: Vec<PosTree>,
}

impl PosTree {
    #[inline]
    pub fn new(pos: Pos) -> Self {
        Self {
            pos: pos,
            children: Vec::new(),
        }
    }

    /// Traverses a tree with indices from `path`. Similar to `Node::index_path`.
    pub fn index_path(&self, path: &[usize]) -> Option<&Self> {
        let mut current_node = self;
        for &index in path {
            if let Some(next_node) = self.children.get(index) {
                current_node = next_node;
            } else {
                return None;
            }
        }
        Some(current_node)
    }
}

/// A SISE node.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    /// An atom, that matches the following regular expression:
    ///
    /// > `"([:atomchar:]|\"(\\([:stringchar:]|\\|\")|[:stringchar:])+\")+"`
    ///
    /// Where `:atomchar:` is one of:
    ///
    /// > `!`, `#`, `$`, `%`, `&`, `*`, `+`, `-`, `.`, `/`, `:`, `<`, `=`,
    /// `>`, `?`, `@`, `_`, `~`
    ///
    /// And `:stringchar:` is any character between ASCII space and `~`,
    /// except `\` and `"`.
    ///
    /// Atoms are not interpreted in any way, the crate `sise_atom` provides
    /// functions to encode and decode atoms as strings, numbers, booleans...
    Atom(String),

    /// A list of nodes
    List(Vec<Box<Node>>),
}

impl Node {
    /// Bitcasts the reference to `self` to `usize`. Useful
    /// to insert it in a `HashSet<usize>`, which is used
    /// by `sise_encoder::SpacedStyle`.
    ///
    /// # Example
    ///
    /// ```
    /// let node = sise::Node::Atom(String::from("example"));
    /// assert_eq!(node.ref_as_usize(), &node as *const sise::Node as usize);
    /// ```
    #[inline]
    pub fn ref_as_usize(&self) -> usize {
        self as *const Self as usize
    }

    /// Return whether the node is an `Atom`.
    #[inline]
    pub fn is_atom(&self) -> bool {
        match *self {
            Node::Atom(_) => true,
            _ => false,
        }
    }

    /// Return whether the node is a `List`.
    #[inline]
    pub fn is_list(&self) -> bool {
        match *self {
            Node::List(_) => true,
            _ => false,
        }
    }

    /// Consumes the node and returns the atom value if it is an
    /// `Atom`.
    #[inline]
    pub fn into_atom(self) -> Option<String> {
        match self {
            Node::Atom(s) => Some(s),
            _ => None,
        }
    }

    /// Consumes the node and returns the list if it is a
    /// `List`.
    #[inline]
    pub fn into_list(self) -> Option<Vec<Box<Node>>> {
        match self {
            Node::List(l) => Some(l),
            _ => None,
        }
    }

    /// Returns a reference to the atom value if the node is
    /// an `Atom`.
    #[inline]
    pub fn as_atom(&self) -> Option<&String> {
        match *self {
            Node::Atom(ref s) => Some(s),
            _ => None,
        }
    }

    /// Returns a reference to the list if the node is
    /// a `List`.
    #[inline]
    pub fn as_list(&self) -> Option<&Vec<Box<Node>>> {
        match *self {
            Node::List(ref l) => Some(l),
            _ => None,
        }
    }

    /// Returns a mutable reference to the atom value if the node is
    /// an `Atom`.
    #[inline]
    pub fn as_mut_atom(&mut self) -> Option<&mut String> {
        match *self {
            Node::Atom(ref mut s) => Some(s),
            _ => None,
        }
    }

    /// Returns mutable a reference to the list if the node is
    /// a `List`.
    #[inline]
    pub fn as_mut_list(&mut self) -> Option<&mut Vec<Box<Node>>> {
        match *self {
            Node::List(ref mut l) => Some(l),
            _ => None,
        }
    }

    /// Traverses a tree with indices from `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use sise::sise_expr;
    ///
    /// let tree = sise_expr!(["example", ["1", "2", "3"], ["a", "b", "c"]]);
    /// assert_eq!(tree.index_path(&[0]).unwrap(), "example");
    /// assert_eq!(tree.index_path(&[1]).unwrap(), &*sise_expr!(["1", "2", "3"]));
    /// assert_eq!(tree.index_path(&[1, 0]).unwrap(), "1");
    /// assert_eq!(tree.index_path(&[2, 0]).unwrap(), "a");
    /// assert!(tree.index_path(&[3]).is_none());
    /// assert!(tree.index_path(&[0, 1]).is_none());
    /// ```
    pub fn index_path(&self, path: &[usize]) -> Option<&Self> {
        let mut current_node = self;
        for &index in path {
            match current_node {
                Node::List(ref list) => {
                    if let Some(next_node) = list.get(index) {
                        current_node = next_node;
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }
        Some(current_node)
    }
}

impl PartialEq<str> for Node {
    fn eq(&self, other: &str) -> bool {
        match *self {
            Node::Atom(ref atom) => atom == other,
            _ => false,
        }
    }
}

impl PartialEq<String> for Node {
    fn eq(&self, other: &String) -> bool {
        match *self {
            Node::Atom(ref atom) => atom == other,
            _ => false,
        }
    }
}

impl<'a> From<&'a str> for Node {
    #[inline]
    fn from(atom: &'a str) -> Node {
        Node::Atom(String::from(atom))
    }
}

impl From<String> for Node {
    #[inline]
    fn from(atom: String) -> Node {
        Node::Atom(atom)
    }
}

impl From<Vec<Box<Node>>> for Node {
    #[inline]
    fn from(list: Vec<Box<Node>>) -> Node {
        Node::List(list)
    }
}

/// Returns whether `chr` is a valid atom character outside a
/// string (i.e. one of `:atomchar:` documented at `Node::Atom`).
pub fn is_atom_chr(chr: u8) -> bool {
    let chars = [
        b'!', b'#', b'$', b'%', b'&', b'*', b'+', b'-',
        b'.', b'/', b':', b'<', b'=', b'>', b'?', b'@',
        b'_', b'~'
    ];
    chr.is_ascii_alphanumeric() || chars.contains(&chr)
}

/// Returns whether `chr` is a valid atom character inside a
/// string, excluding `"` and `\` (i.e. one of `:stringchar:`
/// documented at `Node::Atom`).
pub fn is_atom_string_chr(chr: u8) -> bool {
    (chr.is_ascii_graphic() && chr != b'"' && chr != b'\\') || chr == b' '
}

/// Checks whether `atom` is a valid atom (i.e. matches the regular
/// expression documented at `Node::Atom`).
pub fn check_atom(atom: &str) -> bool {
    enum State {
        Beginning,
        Normal,
        String,
        StringBackslash,
    }

    let mut state = State::Beginning;
    let mut iter = atom.as_bytes().iter().map(|&c| c);
    loop {
        let chr = iter.next();
        match state {
            State::Beginning => {
                match chr {
                    Some(b'"') => {
                        state = State::String;
                    }
                    Some(c) if is_atom_chr(c) => {
                        state = State::Normal;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Empty atom
                        return false;
                    }
                }
            }
            State::Normal => {
                match chr {
                    Some(b'"') => {
                        state = State::String;
                    }
                    Some(c) if is_atom_chr(c) => {
                        state = State::Normal;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Valid atom
                        return true;
                    }
                }
            }
            State::String => {
                match chr {
                    Some(b'"') => {
                        state = State::Normal;
                    }
                    Some(b'\\') => {
                        state = State::StringBackslash;
                    }
                    Some(c) if is_atom_string_chr(c) => {
                        state = State::String;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Incomplete string
                        return false;
                    }
                }
            }
            State::StringBackslash => {
                match chr {
                    Some(c) if is_atom_string_chr(c) || c == b'"' || c == b'\\' => {
                        state = State::String;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Incomplete string
                        return false;
                    }
                }
            }
        }
    }
}
