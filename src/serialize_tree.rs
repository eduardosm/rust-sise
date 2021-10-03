use alloc::vec::Vec;

use crate::{Serializer, TreeNode};

/// Serializes a tree of nodes into `serializer`.
///
/// # Example
///
/// ```
/// use sise::sise_tree;
///
/// let tree = sise_tree!(["example", ["1", "2", "3"], ["a", "b", "c"]]);
///
/// let style = sise::SerializerStyle {
///     line_break: "\n",
///     indentation: " ",
/// };
///
/// let mut result = String::new();
/// let mut serializer = sise::Serializer::new(style, &mut result);
///
/// sise::serialize_tree(&mut serializer, &tree, usize::MAX);
/// // Don't forget to finish the serializer
/// serializer.finish(false);
///
/// let expected_result = "(example (1 2 3) (a b c))";
/// assert_eq!(result, expected_result);
/// ```
///
/// If you use multi-line style, atoms at the beginning of a list
/// will be placed in the same line as the openning `(`:
///
/// ```
/// use sise::sise_tree;
///
/// let tree = sise_tree!(["example", ["1", "2", "3"], ["a", "b", "c"]]);
///
/// let style = sise::SerializerStyle {
///     line_break: "\n",
///     indentation: " ",
/// };
///
/// let mut result = String::new();
/// let mut serializer = sise::Serializer::new(style, &mut result);
///
/// sise::serialize_tree(&mut serializer, &tree, 0);
/// // Don't forget to finish the serializer
/// serializer.finish(true);
///
/// let expected_result = "(example\n (1\n  2\n  3\n )\n (a\n  b\n  c\n )\n)\n";
/// assert_eq!(result, expected_result);
/// ```
///
/// It does not consume the serializer, so it can also be used to serialize
/// a sub-tree:
///
/// ```
/// use sise::sise_tree;
///
/// let tree = sise_tree!(["1", "2", "3"]);
///
/// let style = sise::SerializerStyle {
///     line_break: "\n",
///     indentation: " ",
/// };
///
/// let mut result = String::new();
/// let mut serializer = sise::Serializer::new(style, &mut result);
///
/// // Serialize the head
/// serializer.begin_list(usize::MAX);
/// serializer.put_atom("head", usize::MAX);
///
/// // Serialize the subtree
/// sise::serialize_tree(&mut serializer, &tree, usize::MAX);
///
/// // Serialize the tail
/// serializer.put_atom("tail", usize::MAX);
/// serializer.end_list();
/// serializer.finish(false);
///
/// let expected_result = "(head (1 2 3) tail)";
/// assert_eq!(result, expected_result);
/// ```
pub fn serialize_tree(
    serializer: &mut Serializer<'_, '_>,
    root_node: &TreeNode,
    break_line_at: usize,
) {
    enum State<'a> {
        Beginning(&'a TreeNode),
        Writing {
            stack: Vec<core::slice::Iter<'a, TreeNode>>,
            current_list: core::slice::Iter<'a, TreeNode>,
            list_beginning: bool,
        },
        Finished,
    }

    let mut state = State::Beginning(root_node);

    loop {
        match state {
            State::Beginning(node) => match node {
                TreeNode::Atom(atom) => {
                    serializer.put_atom(atom, break_line_at);
                    state = State::Finished;
                }
                TreeNode::List(list) => {
                    serializer.begin_list(break_line_at);
                    state = State::Writing {
                        stack: Vec::new(),
                        current_list: list.iter(),
                        list_beginning: true,
                    };
                }
            },
            State::Writing {
                ref mut stack,
                ref mut current_list,
                ref mut list_beginning,
            } => {
                if let Some(node) = current_list.next() {
                    match node {
                        TreeNode::Atom(atom) => {
                            if *list_beginning {
                                serializer.put_atom(atom, usize::MAX);
                            } else {
                                serializer.put_atom(atom, break_line_at);
                            }
                            *list_beginning = false;
                        }
                        TreeNode::List(list) => {
                            serializer.begin_list(break_line_at);
                            stack.push(core::mem::replace(current_list, list.iter()));
                            *list_beginning = true;
                        }
                    }
                } else {
                    serializer.end_list();
                    if let Some(parent_list) = stack.pop() {
                        *current_list = parent_list;
                        *list_beginning = false;
                    } else {
                        state = State::Finished;
                    }
                }
            }
            State::Finished => return,
        }
    }
}
