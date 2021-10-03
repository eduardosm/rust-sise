//! SISE (Simple S-expression) is a file format based on S-expressions.
//!
//! # Minimum Rust version
//!
//! The minimum Rust version required by this crate is 1.53.

#![deny(
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_must_use,
    unused_qualifications
)]
#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(test)]
mod tests;

mod parse_tree;
mod parser;
mod serialize_tree;
mod serializer;
mod tree;
mod util;

pub use parse_tree::parse_tree;
pub use parser::{ParseError, ParsedItem, Parser};
pub use serialize_tree::serialize_tree;
pub use serializer::{Serializer, SerializerStyle};
pub use tree::TreeNode;
pub use util::{check_atom, is_atom_chr, is_atom_string_chr};

/// Macro to define trees of nodes with a lighter syntax.
///
/// # Example
///
/// ```
/// use sise::sise_tree;
///
/// // atom
/// let value1 = sise::TreeNode::Atom(String::from("atom"));
/// let value2 = sise_tree!("atom");
/// assert_eq!(value1, value2);
///
/// // ()
/// let value1 = sise::TreeNode::List(vec![]);
/// let value2 = sise_tree!([]);
/// assert_eq!(value1, value2);
///
/// // (atom)
/// let value1 = sise::TreeNode::List(vec![sise::TreeNode::Atom(String::from("atom"))]);
/// let value2 = sise_tree!(["atom"]);
/// assert_eq!(value1, value2);
///
/// // (atom (1 2 3) (a b c))
/// let value1 = sise::TreeNode::List(vec![
///     sise::TreeNode::Atom(String::from("atom")),
///     sise::TreeNode::List(vec![
///         sise::TreeNode::Atom(String::from("1")),
///         sise::TreeNode::Atom(String::from("2")),
///         sise::TreeNode::Atom(String::from("3")),
///     ]),
///     sise::TreeNode::List(vec![
///         sise::TreeNode::Atom(String::from("a")),
///         sise::TreeNode::Atom(String::from("b")),
///         sise::TreeNode::Atom(String::from("c")),
///     ]),
/// ]);
/// let value2 = sise_tree!(["atom", ["1", "2", "3"], ["a", "b", "c"]]);
/// assert_eq!(value1, value2);
/// ```
#[macro_export]
macro_rules! sise_tree {
    ([$($item:tt),*]) => { $crate::TreeNode::List($crate::__vec![$($crate::sise_tree!($item)),*]) };
    ([$($item:tt,)*]) => { $crate::TreeNode::List($crate::__vec![$($crate::sise_tree!($item)),*]) };
    ($node:expr) => { $crate::TreeNode::from($node) };
}

#[doc(hidden)]
pub use alloc::vec as __vec;
