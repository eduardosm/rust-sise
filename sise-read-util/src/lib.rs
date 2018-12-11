// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(test)]
mod tests;

extern crate sise;

#[derive(Clone, Debug, PartialEq)]
pub enum ReadError {
    ExpectedAtom {
        pos: Option<sise::Pos>,
    },
    ExpectedList {
        pos: Option<sise::Pos>,
    },
    ExpectedNodeInList {
        list_pos: Option<sise::Pos>,
    },
    UnexpectedNodeInList {
        node_pos: Option<sise::Pos>,
    },
    InvalidValue {
        value_type: String,
        pos: Option<sise::Pos>,
    },
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReadError::ExpectedAtom { pos } => {
                if let Some(pos) = pos {
                    write!(f, "expected atom at {}:{}",
                           sise::ReprPosValue(pos.line),
                           sise::ReprPosValue(pos.column))
                } else {
                    f.write_str("expected atom")
                }
            }
            ReadError::ExpectedList { pos } => {
                if let Some(pos) = pos {
                    write!(f, "expected list at {}:{}",
                           sise::ReprPosValue(pos.line),
                           sise::ReprPosValue(pos.column))
                } else {
                    f.write_str("expected list")
                }
            }
            ReadError::ExpectedNodeInList { list_pos } => {
                if let Some(list_pos) = list_pos {
                    write!(f, "expected node in list at {}:{}",
                           sise::ReprPosValue(list_pos.line),
                           sise::ReprPosValue(list_pos.column))
                } else {
                    f.write_str("expected node in list")
                }
            }
            ReadError::UnexpectedNodeInList { node_pos } => {
                if let Some(node_pos) = node_pos {
                    write!(f, "unexpected node in list at {}:{}",
                           sise::ReprPosValue(node_pos.line),
                           sise::ReprPosValue(node_pos.column))
                } else {
                    f.write_str("unexpected node in list")
                }
            }
            ReadError::InvalidValue { value_type, pos } => {
                if let Some(pos) = pos {
                    write!(f, "invalid value of type {:?} at {}:{}",
                           value_type,
                           sise::ReprPosValue(pos.line),
                           sise::ReprPosValue(pos.column))
                } else {
                    write!(f, "invalid value of type {:?}", value_type)
                }
            }
        }
    }
}

impl std::error::Error for ReadError {
    fn description(&self) -> &str {
        match self {
            ReadError::ExpectedAtom { .. } => "expected atom",
            ReadError::ExpectedList { .. } => "expected list",
            ReadError::ExpectedNodeInList { .. } => "expected node in list",
            ReadError::UnexpectedNodeInList { .. } => "unexpected node in list",
            ReadError::InvalidValue { .. } => "invalid value",
        }
    }
}

/// If `node` is an atom, it returns its value. Otherwise it returns
/// a `ReadError::ExpectedAtom` error.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
///
/// let pos_map = sise::PosMap::new();
/// let node = sise_expr!("example");
/// assert_eq!(sise_read_util::get_as_atom(&node, &pos_map).unwrap(), "example");
/// ```
pub fn get_as_atom<'a>(node: &'a sise::Node, pos_map: &sise::PosMap)
    -> Result<&'a str, ReadError>
{
    if let sise::Node::Atom(ref atom) = *node {
        Ok(atom)
    } else {
        Err(ReadError::ExpectedAtom {
            pos: pos_map.get_pos(node),
        })
    }
}

/// If `node` is a list, it returns the list. Otherwise it returns
/// a `ReadError::ExpectedList` error.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
///
/// let pos_map = sise::PosMap::new();
/// let node = sise_expr!(["1", "2"]);
/// let expected_list = [sise_expr!("1"), sise_expr!("2")];
/// assert_eq!(sise_read_util::get_as_list(&node, &pos_map).unwrap(), expected_list);
/// ```
pub fn get_as_list<'a>(node: &'a sise::Node, pos_map: &sise::PosMap)
    -> Result<&'a [Box<sise::Node>], ReadError>
{
    if let sise::Node::List(ref list) = *node {
        Ok(list)
    } else {
        Err(ReadError::ExpectedList {
            pos: pos_map.get_pos(node),
        })
    }
}

/// Returns the next node in `iter`, if any. Otherwise, it returns
/// a `ReadError::ExpectedNodeInList` error.
///
/// `list_node` is the list node. Together with `pos_map`, it is used
/// to get its position in case of error.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
///
/// let pos_map = sise::PosMap::new();
/// let list_node = sise_expr!(["1", "2"]);
/// let mut iter = list_node.as_list().unwrap().iter();
/// assert_eq!(sise_read_util::get_node_from_list(&mut iter, &list_node, &pos_map).unwrap(), &sise::Node::Atom("1".into()));
/// assert_eq!(sise_read_util::get_node_from_list(&mut iter, &list_node, &pos_map).unwrap(), &sise::Node::Atom("2".into()));
/// ```
pub fn get_node_from_list<'a, I>(iter: &mut I, list_node: &'a sise::Node, pos_map: &sise::PosMap)
    -> Result<&'a sise::Node, ReadError>
    where I: ?Sized + Iterator<Item=&'a Box<sise::Node>>
{
    if let Some(node) = iter.next() {
        Ok(&*node)
    } else {
        Err(ReadError::ExpectedNodeInList {
            list_pos: pos_map.get_pos(list_node),
        })
    }
}

/// Returns a `ReadError::UnexpectedNodeInList` error if there
/// are more nodes remaining nodes in the `iter`.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
///
/// let pos_map = sise::PosMap::new();
/// let list_node = sise_expr!([]);
/// let mut iter = list_node.as_list().unwrap().iter();
/// assert!(sise_read_util::expect_end_of_list(&mut iter, &pos_map).is_ok());
/// ```
pub fn expect_end_of_list<'a, I>(iter: &mut I, pos_map: &sise::PosMap) -> Result<(), ReadError>
    where I: ?Sized + Iterator<Item=&'a Box<sise::Node>>
{
    if let Some(node) = iter.next() {
        Err(ReadError::UnexpectedNodeInList {
            node_pos: pos_map.get_pos(node),
        })
    } else {
        Ok(())
    }
}

/// If `node` is an atom, it tries to decode its value using `f`.
///
/// If `f` returns `None`, this function returns a `ReadError::InvalidValue`
/// error.
///
/// If `node` is not an atom, it returns a `ReadError::ExpectedAtom`
/// error. The field `value_type` is set with the value of the
/// `value_type` parameter.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
///
/// let pos_map = sise::PosMap::new();
/// let node = sise_expr!("aa");
/// assert_eq!(sise_read_util::decode_atom(&node, |atom| Some(atom.len()), "decode_as_length", &pos_map).unwrap(), 2);
/// ```
pub fn decode_atom<T, F>(node: &sise::Node, f: F, value_type: &str, pos_map: &sise::PosMap)
    -> Result<T, ReadError>
    where F: FnOnce(&str) -> Option<T>
{
    if let Some(value) = f(get_as_atom(node, pos_map)?) {
        Ok(value)
    } else {
        Err(ReadError::InvalidValue {
            value_type: String::from(value_type),
            pos: pos_map.get_pos(node),
        })
    }
}

/// Gets the next node from `iter`, checks if it is an
/// atom and decodes its value using `f`.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
///
/// let pos_map = sise::PosMap::new();
/// let list_node = sise_expr!(["12", "123"]);
/// let mut iter = list_node.as_list().unwrap().iter();
/// assert_eq!(sise_read_util::get_and_decode_atom_from_list(&mut iter, &list_node, |atom| Some(atom.len()), "decode_as_length", &pos_map).unwrap(), 2);
/// assert_eq!(sise_read_util::get_and_decode_atom_from_list(&mut iter, &list_node, |atom| Some(atom.len()), "decode_as_length", &pos_map).unwrap(), 3);
/// ```
pub fn get_and_decode_atom_from_list<'a, I, T, F>(iter: &mut I,
                                                  list_node: &'a sise::Node,
                                                  f: F,
                                                  value_type: &str,
                                                  pos_map: &sise::PosMap)
    -> Result<T, ReadError>
    where I: ?Sized + Iterator<Item=&'a Box<sise::Node>>,
          F: FnOnce(&str) -> Option<T>,
{
    let node = get_node_from_list(iter, list_node, pos_map)?;
    decode_atom(node, f, value_type, pos_map)
}

/// Gets the remaining nodes from `iter`, checks if they are atoms
/// atom and decodes their value using `f`.
///
///
/// # Example
///
/// ```
/// use sise::sise_expr;
///
/// let pos_map = sise::PosMap::new();
/// let list_node = sise_expr!(["12", "123"]);
/// let mut iter = list_node.as_list().unwrap().iter();
/// let expected_list = [2, 3];
/// assert_eq!(sise_read_util::get_and_decode_atoms_from_list(&mut iter,
///                                                           &list_node,
///                                                           true,
///                                                           |atom| Some(atom.len()),
///                                                           "decode_as_length", &pos_map).unwrap(), expected_list);
/// ```
pub fn get_and_decode_atoms_from_list<'a, I, T, F>(iter: &mut I,
                                                   list_node: &'a sise::Node,
                                                   can_be_empty: bool,
                                                   mut f: F,
                                                   value_type: &str,
                                                   pos_map: &sise::PosMap)
    -> Result<Vec<T>, ReadError>
    where I: ?Sized + Iterator<Item=&'a Box<sise::Node>>,
          F: FnMut(&str) -> Option<T>,
{
    let mut decoded_values = Vec::new();
    for value_node in iter {
        if let Some(value) = f(get_as_atom(value_node, pos_map)?) {
            decoded_values.push(value);
        } else {
            return Err(ReadError::InvalidValue {
                value_type: String::from(value_type),
                pos: pos_map.get_pos(value_node),
            });
        }
    }

    if !can_be_empty && decoded_values.is_empty() {
        return Err(ReadError::ExpectedNodeInList {
            list_pos: pos_map.get_pos(list_node),
        });
    }

    Ok(decoded_values)
}
