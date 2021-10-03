use alloc::string::String;
use alloc::vec::Vec;

/// A SISE tree node.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TreeNode {
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
    List(Vec<TreeNode>),
}

impl TreeNode {
    /// Return whether the node is an `Atom`.
    #[inline]
    pub fn is_atom(&self) -> bool {
        matches!(self, Self::Atom(_))
    }

    /// Return whether the node is a `List`.
    #[inline]
    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    /// Consumes the node and returns the atom value if it is an
    /// `Atom`.
    #[inline]
    pub fn into_atom(self) -> Option<String> {
        match self {
            Self::Atom(s) => Some(s),
            _ => None,
        }
    }

    /// Consumes the node and returns the list if it is a
    /// `List`.
    #[inline]
    pub fn into_list(self) -> Option<Vec<Self>> {
        match self {
            Self::List(l) => Some(l),
            _ => None,
        }
    }

    /// Returns a reference to the atom value if the node is
    /// an `Atom`.
    #[inline]
    pub fn as_atom(&self) -> Option<&String> {
        match *self {
            Self::Atom(ref s) => Some(s),
            _ => None,
        }
    }

    /// Returns a reference to the list if the node is
    /// a `List`.
    #[inline]
    pub fn as_list(&self) -> Option<&Vec<Self>> {
        match *self {
            Self::List(ref l) => Some(l),
            _ => None,
        }
    }

    /// Returns a mutable reference to the atom value if the node is
    /// an `Atom`.
    #[inline]
    pub fn as_mut_atom(&mut self) -> Option<&mut String> {
        match *self {
            Self::Atom(ref mut s) => Some(s),
            _ => None,
        }
    }

    /// Returns mutable a reference to the list if the node is
    /// a `List`.
    #[inline]
    pub fn as_mut_list(&mut self) -> Option<&mut Vec<Self>> {
        match *self {
            Self::List(ref mut l) => Some(l),
            _ => None,
        }
    }

    /// Traverses a tree with indices from `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use sise::sise_tree;
    ///
    /// // (example (1 2 3) (a b c))
    /// let tree = sise_tree!(["example", ["1", "2", "3"], ["a", "b", "c"]]);
    /// assert_eq!(*tree.index_path(&[]).unwrap(), tree);
    /// assert_eq!(*tree.index_path(&[0]).unwrap(), "example");
    /// assert_eq!(*tree.index_path(&[1]).unwrap(), sise_tree!(["1", "2", "3"]));
    /// assert_eq!(tree.index_path(&[1, 0]).unwrap(), "1");
    /// assert_eq!(tree.index_path(&[2, 0]).unwrap(), "a");
    /// assert!(tree.index_path(&[3]).is_none());
    /// assert!(tree.index_path(&[0, 1]).is_none());
    /// ```
    pub fn index_path(&self, path: &[usize]) -> Option<&Self> {
        let mut current_node = self;
        for &index in path {
            match current_node {
                Self::Atom(_) => return None,
                Self::List(ref list) => current_node = list.get(index)?,
            }
        }
        Some(current_node)
    }
}

impl PartialEq<str> for TreeNode {
    fn eq(&self, other: &str) -> bool {
        match *self {
            Self::Atom(ref atom) => atom == other,
            _ => false,
        }
    }
}

impl PartialEq<&str> for TreeNode {
    fn eq(&self, other: &&str) -> bool {
        match *self {
            Self::Atom(ref atom) => atom == *other,
            _ => false,
        }
    }
}

impl PartialEq<String> for TreeNode {
    fn eq(&self, other: &String) -> bool {
        match *self {
            Self::Atom(ref atom) => atom == other,
            _ => false,
        }
    }
}

impl<'a> From<&'a str> for TreeNode {
    #[inline]
    fn from(atom: &'a str) -> Self {
        Self::Atom(String::from(atom))
    }
}

impl From<String> for TreeNode {
    #[inline]
    fn from(atom: String) -> Self {
        Self::Atom(atom)
    }
}

impl From<Vec<TreeNode>> for TreeNode {
    #[inline]
    fn from(list: Vec<Self>) -> Self {
        Self::List(list)
    }
}
