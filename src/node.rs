// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use alloc::string::String;
use alloc::vec::Vec;

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
    List(Vec<Node>),
}

impl Node {
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
    pub fn into_list(self) -> Option<Vec<Node>> {
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
    pub fn as_list(&self) -> Option<&Vec<Node>> {
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
    pub fn as_mut_list(&mut self) -> Option<&mut Vec<Node>> {
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
    /// // (example (1 2 3) (a b c))
    /// let tree = sise_expr!(["example", ["1", "2", "3"], ["a", "b", "c"]]);
    /// assert_eq!(*tree.index_path(&[]).unwrap(), tree);
    /// assert_eq!(*tree.index_path(&[0]).unwrap(), "example");
    /// assert_eq!(*tree.index_path(&[1]).unwrap(), sise_expr!(["1", "2", "3"]));
    /// assert_eq!(tree.index_path(&[1, 0]).unwrap(), "1");
    /// assert_eq!(tree.index_path(&[2, 0]).unwrap(), "a");
    /// assert!(tree.index_path(&[3]).is_none());
    /// assert!(tree.index_path(&[0, 1]).is_none());
    /// ```
    pub fn index_path(&self, path: &[usize]) -> Option<&Self> {
        let mut current_node = self;
        for &index in path {
            match current_node {
                Node::Atom(_) => return None,
                Node::List(ref list) => current_node = list.get(index)?,
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

impl PartialEq<&str> for Node {
    fn eq(&self, other: &&str) -> bool {
        match *self {
            Node::Atom(ref atom) => atom == *other,
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

impl From<Vec<Node>> for Node {
    #[inline]
    fn from(list: Vec<Node>) -> Node {
        Node::List(list)
    }
}
