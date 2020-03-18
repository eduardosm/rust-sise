// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! SISE (Simple S-expression) is a file format based on S-expressions.
//!
//! # Minimum Rust version
//!
//! The minimum Rust version required by this crate is 1.36.

#![deny(
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_import_braces,
    unused_must_use,
    unused_qualifications
)]

#[cfg(test)]
mod tests {
    mod parser;
    mod read_into_tree;
    mod read_util;
    mod tree_reader;
    mod util;
    mod writer;
}

mod pos;
pub use self::pos::Pos;
pub use self::pos::ReprPosValue;

mod node;
pub use self::node::Node;

mod reader;
pub use self::reader::ReadItem;
pub use self::reader::ReadItemKind;
pub use self::reader::Reader;

mod parser;
pub use self::parser::ParseError;
pub use self::parser::Parser;
pub use self::parser::TokenKind;

mod tree_reader;
pub use self::tree_reader::TreeReader;

mod read_into_tree;
pub use self::read_into_tree::read_into_tree;

mod writer;
pub use self::writer::MaybeMultilineOptions;
pub use self::writer::Writer;

mod compact_string_writer;
pub use self::compact_string_writer::CompactStringWriter;

mod spaced_string_writer;
pub use self::spaced_string_writer::SpacedStringWriter;
pub use self::spaced_string_writer::SpacedStringWriterNodeOptions;
pub use self::spaced_string_writer::SpacedStringWriterStyle;

mod tree_writer;
pub use self::tree_writer::TreeWriter;

mod write_from_tree;
pub use self::write_from_tree::write_from_tree;
pub use self::write_from_tree::WriteFromTreeAtomOptions;

mod read_util;
pub use self::read_util::AtomReadUtil;
pub use self::read_util::ListReadUtil;
pub use self::read_util::NodeReadUtil;
pub use self::read_util::ReadUtilError;

mod util;
pub use self::util::check_atom;
pub use self::util::is_atom_chr;
pub use self::util::is_atom_string_chr;

/// Macro to define trees of nodes with a lighter syntax.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
///
/// // atom
/// let value1 = sise::Node::Atom(String::from("atom"));
/// let value2 = sise_expr!("atom");
/// assert_eq!(value1, value2);
///
/// // ()
/// let value1 = sise::Node::List(vec![]);
/// let value2 = sise_expr!([]);
/// assert_eq!(value1, value2);
///
/// // (atom)
/// let value1 = sise::Node::List(vec![
///     sise::Node::Atom(String::from("atom"))
/// ]);
/// let value2 = sise_expr!(["atom"]);
/// assert_eq!(value1, value2);
///
/// // (atom (1 2 3) (a b c))
/// let value1 = sise::Node::List(vec![
///     sise::Node::Atom(String::from("atom")),
///     sise::Node::List(vec![
///         sise::Node::Atom(String::from("1")),
///         sise::Node::Atom(String::from("2")),
///         sise::Node::Atom(String::from("3")),
///     ]),
///     sise::Node::List(vec![
///         sise::Node::Atom(String::from("a")),
///         sise::Node::Atom(String::from("b")),
///         sise::Node::Atom(String::from("c")),
///     ]),
/// ]);
/// let value2 = sise_expr!(["atom", ["1", "2", "3"], ["a", "b", "c"]]);
/// assert_eq!(value1, value2);
/// ```
#[macro_export]
macro_rules! sise_expr {
    ([$($item:tt),*]) => { $crate::Node::List(vec![$(sise_expr!($item)),*]) };
    ([$($item:tt,)*]) => { $crate::Node::List(vec![$(sise_expr!($item)),*]) };
    ($node:expr) => { $crate::Node::from($node) };
}
