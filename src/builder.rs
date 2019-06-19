// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::Node;

/// Base struct from which `Builder` are created.
/// See `Builder` example.
pub struct BuilderBase {
    stack: Vec<Vec<Node>>,
    current: Vec<Node>,
}

/// Helper struct to build SISE trees and get index paths
/// of the inserted nodes.
///
/// # Example
///
/// ```
/// use sise::sise_expr;
///
/// let mut builder_base = sise::BuilderBase::new();
/// let mut builder = builder_base.builder();
///
/// // Build (atom-1 atom-2 (atom-3 atom-4) atom-5)
/// builder.add_node("atom-1");
/// assert_eq!(builder.last_index_path(), [0]);
/// builder.add_node("atom-2");
/// assert_eq!(builder.last_index_path(), [1]);
/// builder.begin_list();
/// builder.add_node("atom-3");
/// assert_eq!(builder.last_index_path(), [2, 0]);
/// builder.add_node("atom-4");
/// assert_eq!(builder.last_index_path(), [2, 1]);
/// builder.end_list();
/// assert_eq!(builder.last_index_path(), [2]);
/// builder.add_node("atom-5");
/// assert_eq!(builder.last_index_path(), [3]);
/// builder.finish();
///
/// let root_node = builder_base.into_node();
/// let expected = sise_expr!(["atom-1", "atom-2", ["atom-3", "atom-4"], "atom-5"]);
/// assert_eq!(root_node, expected);
/// ```
pub struct Builder<'a> {
    base: &'a mut BuilderBase,
    min_depth: usize,
}

impl BuilderBase {
    #[inline]
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            current: Vec::new(),
        }
    }

    #[inline]
    pub fn builder(&mut self) -> Builder<'_> {
        assert!(self.stack.is_empty());
        assert!(self.current.is_empty());
        Builder {
            base: self,
            min_depth: 0,
        }
    }

    #[inline]
    pub fn into_node(self) -> Node {
        assert!(self.stack.is_empty());
        Node::List(self.current)
    }
}

impl Default for BuilderBase {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Builder<'a> {
    /// Returns the index path of the last inserted node.
    pub fn last_index_path(&self) -> Vec<usize> {
        let mut path = Vec::with_capacity(self.base.stack.len() + 1);
        for stack_item in self.base.stack.iter() {
            path.push(stack_item.len());
        }
        if !self.base.current.is_empty() {
            path.push(self.base.current.len() - 1);
        }
        path
    }

    /// Creates a builder that won't allow to pop further.
    ///
    /// # Example
    ///
    /// ```
    /// let r = std::panic::catch_unwind(|| {
    ///     let mut builder_base = sise::BuilderBase::new();
    ///     let mut builder = builder_base.builder();
    ///
    ///     builder.begin_list();
    ///     let mut builder2 = builder.sub_builder();
    ///     builder2.end_list();
    /// });
    /// assert!(r.is_err());
    /// ```
    #[inline]
    pub fn sub_builder(&mut self) -> Builder<'_> {
        let min_depth = self.base.stack.len();
        Builder {
            base: self.base,
            min_depth,
        }
    }

    /// Adds `node` into the current list.
    pub fn add_node<T: Into<Node>>(&mut self, node: T) {
        self.base.current.push(node.into());
    }

    /// Creates a new list, pushing the current one into a stack.
    /// This new list will be pushed into the current one.
    pub fn begin_list(&mut self) {
        self.base.stack.push(std::mem::replace(&mut self.base.current, Vec::new()));
    }

    /// Finishes the current list, popping a list from the
    /// stack and setting it as current.
    pub fn end_list(&mut self) {
        assert!(self.base.stack.len() > self.min_depth);
        let parent_list = self.base.stack.pop().unwrap();
        let current_list = std::mem::replace(&mut self.base.current, parent_list);
        self.base.current.push(Node::List(current_list));
    }

    /// Finishes the builder, making sure that the stack depth
    /// is the same as when it was created.
    pub fn finish(self) {
        assert_eq!(self.base.stack.len(), self.min_depth);
    }
}
