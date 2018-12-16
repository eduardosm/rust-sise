// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::Pos;
use crate::ReprPosValue;
use crate::PosTree;
use crate::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum ReadUtilError {
    ExpectedAtom {
        pos: Option<Pos>,
    },
    ExpectedList {
        pos: Option<Pos>,
    },
    ExpectedListEnd {
        node_pos: Option<Pos>,
    },
    ExpectedNodeInList {
        list_pos: Option<Pos>,
    },
    InvalidValue {
        value_type: String,
        pos: Option<Pos>,
    },
}

impl std::fmt::Display for ReadUtilError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReadUtilError::ExpectedAtom { pos } => {
                if let Some(pos) = pos {
                    write!(f, "expected atom at {}:{}",
                           ReprPosValue(pos.line),
                           ReprPosValue(pos.column))
                } else {
                    f.write_str("expected atom")
                }
            }
            ReadUtilError::ExpectedList { pos } => {
                if let Some(pos) = pos {
                    write!(f, "expected list at {}:{}",
                           ReprPosValue(pos.line),
                           ReprPosValue(pos.column))
                } else {
                    f.write_str("expected list")
                }
            }
            ReadUtilError::ExpectedListEnd { node_pos } => {
                if let Some(node_pos) = node_pos {
                    write!(f, "unexpected node in list at {}:{}",
                           ReprPosValue(node_pos.line),
                           ReprPosValue(node_pos.column))
                } else {
                    f.write_str("unexpected node in list")
                }
            }
            ReadUtilError::ExpectedNodeInList { list_pos } => {
                if let Some(list_pos) = list_pos {
                    write!(f, "expected node in list at {}:{}",
                           ReprPosValue(list_pos.line),
                           ReprPosValue(list_pos.column))
                } else {
                    f.write_str("expected node in list")
                }
            }
            ReadUtilError::InvalidValue { value_type, pos } => {
                if let Some(pos) = pos {
                    write!(f, "invalid value of type {:?} at {}:{}",
                           value_type,
                           ReprPosValue(pos.line),
                           ReprPosValue(pos.column))
                } else {
                    write!(f, "invalid value of type {:?}", value_type)
                }
            }
        }
    }
}

impl std::error::Error for ReadUtilError {
    fn description(&self) -> &str {
        match self {
            ReadUtilError::ExpectedAtom { .. } => "expected atom",
            ReadUtilError::ExpectedList { .. } => "expected list",
            ReadUtilError::ExpectedListEnd { .. } => "unexpected node in list",
            ReadUtilError::ExpectedNodeInList { .. } => "expected node in list",
            ReadUtilError::InvalidValue { .. } => "invalid value",
        }
    }
}

/// Utility to read nodes.
///
/// See `as_atom` and `as_list` methods.
#[derive(Clone, Debug)]
pub struct NodeReadUtil<'a, 'b> {
    node: &'a Node,
    pos_tree: Option<&'b PosTree>,
}

impl<'a, 'b> NodeReadUtil<'a, 'b> {
    #[inline]
    pub fn new(node: &'a Node, pos_tree: Option<&'b PosTree>) -> Self {
        Self {
            node: node,
            pos_tree: pos_tree,
        }
    }

    /// Returns the node in this utility.
    #[inline]
    pub fn node(&self) -> &'a Node {
        self.node
    }

    /// Returns the position tree of the node in this utility.
    #[inline]
    pub fn pos_tree(&self) -> Option<&'b PosTree> {
        self.pos_tree
    }

    /// If the node is an atom, returns a `AtomNodeReadUtil` with such
    /// atom. Otherwise, it returns a `ReadError::ExpectedAtom` error.
    ///
    /// # Example
    ///
    /// ```
    /// use sise::sise_expr;
    ///
    /// let node = sise_expr!("example");
    /// let node_read_util = sise::NodeReadUtil::new(&node, None);
    /// assert_eq!(node_read_util.as_atom().unwrap().atom(), "example");
    /// ```
    pub fn as_atom(&self) -> Result<AtomNodeReadUtil<'a>, ReadUtilError> {
        match self.node {
            Node::Atom(ref atom) => {
                let pos = self.pos_tree.map(|pos_tree| pos_tree.pos);
                Ok(AtomNodeReadUtil::new(atom.as_str(), pos))
            }
            _ => Err(ReadUtilError::ExpectedAtom {
                pos: self.pos_tree.map(|pos_tree| pos_tree.pos),
            }),
        }
    }

    /// If the node is a list, returns a `ListNodeReadUtil` with such
    /// list. Otherwise, it returns a `ReadError::ExpectedList` error.
    ///
    /// # Example
    ///
    /// ```
    /// use sise::sise_expr;
    ///
    /// let node = sise_expr!(["example"]);
    /// let node_read_util = sise::NodeReadUtil::new(&node, None);
    /// assert_eq!(node_read_util.as_list().unwrap().list(), [sise_expr!("example")]);
    /// ```
    pub fn as_list(&self) -> Result<ListNodeReadUtil<'a, 'b>, ReadUtilError> {
        match self.node {
            Node::List(ref list) => {
                Ok(ListNodeReadUtil::new(list.as_slice(), self.pos_tree))
            }
            _ => Err(ReadUtilError::ExpectedList {
                pos: self.pos_tree.map(|pos_tree| pos_tree.pos),
            }),
        }
    }
}

/// Utility to read atom nodes.
#[derive(Clone, Debug)]
pub struct AtomNodeReadUtil<'a> {
    atom: &'a str,
    pos: Option<Pos>,
}

impl<'a> AtomNodeReadUtil<'a> {
    #[inline]
    pub fn new(atom: &'a str, pos: Option<Pos>) -> Self {
        Self {
            atom: atom,
            pos: pos,
        }
    }

    /// Returns the atom value in this utility.
    #[inline]
    pub fn atom(&self) -> &'a str {
        self.atom
    }

    /// Returns the position of the atom in this utility.
    #[inline]
    pub fn pos(&self) -> Option<Pos> {
        self.pos
    }

    /// Decodes the atom using an user-provided function.
    ///
    /// The function shall return `None` if the decodification
    /// fails. In such case, `ReadError::InvalidValue` will be
    /// returned, with the value of `value_type`.
    ///
    /// # Example
    ///
    /// ```
    /// use sise::sise_expr;
    ///
    /// let node = sise_expr!("example");
    /// let node_read_util = sise::NodeReadUtil::new(&node, None);
    /// let atom_read_util = node_read_util.as_atom().unwrap();
    /// let decoded = atom_read_util.decode(|atom| Some(atom.len()), "decode_as_atom").unwrap();
    /// assert_eq!(decoded, 7);
    /// ```
    pub fn decode<T, F>(&self, f: F, value_type: &str)
                        -> Result<T, ReadUtilError>
        where F: FnOnce(&str) -> Option<T>
    {
        if let Some(value) = f(self.atom) {
            Ok(value)
        } else {
            Err(ReadUtilError::InvalidValue {
                value_type: value_type.to_string(),
                pos: self.pos,
            })
        }
    }
}

/// Utility to read list nodes.
#[derive(Clone, Debug)]
pub struct ListNodeReadUtil<'a, 'b> {
    list: &'a [Node],
    pos_tree: Option<&'b PosTree>,
    index: usize,
}

impl<'a, 'b> ListNodeReadUtil<'a, 'b> {
    #[inline]
    pub fn new(list: &'a [Node], pos_tree: Option<&'b PosTree>) -> Self {
        Self {
            list: list,
            pos_tree: pos_tree,
            index: 0,
        }
    }

    /// Returns the list in this utility.
    #[inline]
    pub fn list(&self) -> &'a [Node] {
        self.list
    }

    /// Returns the position tree of the list in this utility.
    #[inline]
    pub fn pos_tree(&self) -> Option<&'b PosTree> {
        self.pos_tree
    }

    /// Checks if all the nodes in the list have been read.
    /// If not, it returns a `ReadError::ExpectedListEnd` error.
    ///
    /// # Example
    ///
    /// ```
    /// use sise::sise_expr;
    ///
    /// let node = sise_expr!([]);
    /// let node_read_util = sise::NodeReadUtil::new(&node, None);
    /// let list_read_util = node_read_util.as_list().unwrap();
    /// assert!(list_read_util.expect_end().is_ok());
    /// ```
    pub fn expect_end(&self) -> Result<(), ReadUtilError> {
        if self.index == self.list.len() {
            Ok(())
        } else {
            Err(ReadUtilError::ExpectedListEnd {
                node_pos: self.pos_tree.map(|pos_tree| pos_tree.children[self.index].pos)
            })
        }
    }

    /// Reads the next item in the list. If none are left, it
    /// returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use sise::sise_expr;
    ///
    /// let node = sise_expr!(["a", "b"]);
    /// let node_read_util = sise::NodeReadUtil::new(&node, None);
    /// let mut list_read_util = node_read_util.as_list().unwrap();
    ///
    /// assert_eq!(list_read_util.try_next_item().unwrap().node(), "a");
    /// assert_eq!(list_read_util.try_next_item().unwrap().node(), "b");
    /// assert!(list_read_util.try_next_item().is_none());
    /// ```
    pub fn try_next_item(&mut self) -> Option<NodeReadUtil<'a, 'b>> {
        if self.index == self.list.len() {
            None
        } else {
            let item_pos_tree = self.pos_tree.map(|pos_tree| &pos_tree.children[self.index]);
            let node = NodeReadUtil::new(&self.list[self.index], item_pos_tree);
            self.index += 1;
            Some(node)
        }
    }

    /// Reads the next item in the list. If none are left, it
    /// returns a `ReadError::ExpectedNodeInList` error.
    ///
    /// # Example
    ///
    /// ```
    /// use sise::sise_expr;
    ///
    /// let node = sise_expr!(["a", "b"]);
    /// let node_read_util = sise::NodeReadUtil::new(&node, None);
    /// let mut list_read_util = node_read_util.as_list().unwrap();
    ///
    /// assert_eq!(list_read_util.next_item().unwrap().node(), "a");
    /// assert_eq!(list_read_util.next_item().unwrap().node(), "b");
    /// ```
    pub fn next_item(&mut self) -> Result<NodeReadUtil<'a, 'b>, ReadUtilError> {
        self.try_next_item().ok_or_else(|| ReadUtilError::ExpectedNodeInList {
            list_pos: self.pos_tree.map(|pos_tree| pos_tree.pos)
        })
    }

    /// Gets the remaining items from the list, checks if they
    /// are atoms and decodes their value using `f`.
    ///
    /// # Example
    ///
    /// ```
    /// use sise::sise_expr;
    ///
    /// let node = sise_expr!(["1", "12", "123"]);
    /// let node_read_util = sise::NodeReadUtil::new(&node, None);
    /// let list_read_util = node_read_util.as_list().unwrap();
    /// let decoded = list_read_util.decode_atoms(|atom| Some(atom.len()), "decode_as_atom", false).unwrap();
    /// assert_eq!(decoded, [1, 2, 3]);
    /// ```
    pub fn decode_atoms<T, F>(self, mut f: F, value_type: &str, can_be_empty: bool)
                              -> Result<Vec<T>, ReadUtilError>
        where F: FnMut(&str) -> Option<T>
    {
        let pos_tree = self.pos_tree;
        let mut values = Vec::with_capacity(self.list.len() - self.index);
        for item in self {
            values.push(item.as_atom()?.decode(|atom| f(atom), value_type)?);
        }
        if !can_be_empty && values.is_empty() {
            Err(ReadUtilError::ExpectedNodeInList {
                list_pos: pos_tree.map(|pos_tree| pos_tree.pos),
            })
        } else {
            Ok(values)
        }
    }
}

impl<'a, 'b> IntoIterator for ListNodeReadUtil<'a, 'b> {
    type Item = NodeReadUtil<'a, 'b>;
    type IntoIter = ListNodeReadUtilIter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = ListNodeReadUtilIter {
            iter: self.list.iter().enumerate(),
            pos_tree: self.pos_tree,
        };
        if self.index != 0 {
            iter.iter.nth(self.index - 1);
        }
        iter
    }
}

/// Iterator over the items of the list in a `ListNodeReadUtil`
/// that produces `NodeReadUtil`.
///
/// This iterator is created by the `IntoIterator` implementation
/// of `ListNodeReadUtil`.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
///
/// let node = sise_expr!(["a", "b"]);
/// let node_read_util = sise::NodeReadUtil::new(&node, None);
/// let list_read_util = node_read_util.as_list().unwrap();
///
/// let mut atoms = Vec::new();
/// for item in list_read_util {
///     atoms.push(item.as_atom().unwrap().atom());
/// }
/// assert_eq!(atoms, ["a", "b"]);
/// ```
pub struct ListNodeReadUtilIter<'a, 'b> {
    iter: std::iter::Enumerate<std::slice::Iter<'a, Node>>,
    pos_tree: Option<&'b PosTree>,
}

impl<'a, 'b> ListNodeReadUtilIter<'a, 'b> {
    #[inline]
    fn map_fn(i: usize, node: &'a Node, pos_tree: Option<&'b PosTree>)
              -> NodeReadUtil<'a, 'b>
    {
        let node_pos_tree = pos_tree.map(|pos_tree| &pos_tree.children[i]);
        NodeReadUtil::new(node, node_pos_tree)
    }
}

impl<'a, 'b> Iterator for ListNodeReadUtilIter<'a, 'b> {
    type Item = NodeReadUtil<'a, 'b>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, node)| Self::map_fn(i, node, self.pos_tree))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn last(self) -> Option<Self::Item> {
        let iter = self.iter;
        let pos_tree = self.pos_tree;
        iter.last().map(|(i, node)| Self::map_fn(i, node, pos_tree))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|(i, node)| Self::map_fn(i, node, self.pos_tree))
    }
}

impl<'a, 'b> DoubleEndedIterator for ListNodeReadUtilIter<'a, 'b> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(i, node)| Self::map_fn(i, node, self.pos_tree))
    }
}

impl<'a, 'b> ExactSizeIterator for ListNodeReadUtilIter<'a, 'b> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, 'b> std::iter::FusedIterator for ListNodeReadUtilIter<'a, 'b> {}
