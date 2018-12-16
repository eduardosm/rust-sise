// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! SISE (Simple S-expression) is a file format based on S-expressions.

#[cfg(test)]
mod tests {
    mod parser;
    mod serializer;
    mod read_util;
    mod util;
}

mod pos;
pub use self::pos::Pos;
pub use self::pos::ReprPosValue;
pub use self::pos::PosTree;

mod node;
pub use self::node::Node;

mod builder;
pub use self::builder::BuilderBase;
pub use self::builder::Builder;

mod parser;
pub use self::parser::ParseLimits;
pub use self::parser::ParseError;
pub use self::parser::parse;
pub use self::parser::Token;

mod serializer;
pub use self::serializer::SerializeStyle;
pub use self::serializer::CompactSerializeStyle;
pub use self::serializer::SerializeLineEnding;
pub use self::serializer::SerializeIndentChar;
pub use self::serializer::SerializeSpacingConfig;
pub use self::serializer::SpacedSerializeStyle;
pub use self::serializer::serialize_into;
pub use self::serializer::serialize;

mod read_util;
pub use self::read_util::ReadUtilError;
pub use self::read_util::NodeReadUtil;
pub use self::read_util::AtomNodeReadUtil;
pub use self::read_util::ListNodeReadUtil;
pub use self::read_util::ListNodeReadUtilIter;

mod util;
pub use self::util::is_atom_chr;
pub use self::util::is_atom_string_chr;
pub use self::util::check_atom;

/// Macro to define values with a lighter syntax.
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
