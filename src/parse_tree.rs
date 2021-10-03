use alloc::vec::Vec;

use crate::{ParseError, ParsedItem, Parser, TreeNode};

/// Parses into a tree of `TreeNode`.
///
/// # Example
///
/// ```
/// use sise::sise_tree;
///
/// let data = "(test (1 2 3))";
/// let mut parser = sise::Parser::new(data);
/// let root_node = sise::parse_tree(&mut parser).unwrap();
/// // Do not forget calling `finish` on the parser.
/// parser.finish().unwrap();
/// let expected_result = sise_tree!(["test", ["1", "2", "3"]]);
/// assert_eq!(root_node, expected_result);
/// ```
///
/// It does not consume the parser, so it can also be used to parse
/// a sub-tree:
///
/// ```
/// use sise::sise_tree;
///
/// let data = "(head (1 2 3) tail)";
/// let mut parser = sise::Parser::new(data);
///
/// // Parse the head
/// assert_eq!(parser.next_item().unwrap(), sise::ParsedItem::ListStart(0));
/// assert_eq!(
///     parser.next_item().unwrap(),
///     sise::ParsedItem::Atom("head", 1),
/// );
///
/// // Parse the subtree
/// let root_node = sise::parse_tree(&mut parser).unwrap();
/// let expected_result = sise_tree!(["1", "2", "3"]);
/// assert_eq!(root_node, expected_result);
///
/// // Parse the tail
/// assert_eq!(
///     parser.next_item().unwrap(),
///     sise::ParsedItem::Atom("tail", 14)
/// );
/// assert_eq!(parser.next_item().unwrap(), sise::ParsedItem::ListEnd(18));
/// parser.finish().unwrap();
/// ```
pub fn parse_tree(parser: &mut Parser<'_>) -> Result<TreeNode, ParseError> {
    struct StackItem {
        list_items: Vec<TreeNode>,
    }

    enum State {
        Beginning,
        Parsing {
            stack: Vec<StackItem>,
            current: StackItem,
        },
        Finished(TreeNode),
    }

    let mut state = State::Beginning;

    loop {
        match state {
            State::Beginning => match parser.next_item()? {
                ParsedItem::Atom(atom, _) => {
                    let root_node = TreeNode::Atom(atom.into());
                    state = State::Finished(root_node);
                }
                ParsedItem::ListStart(_) => {
                    state = State::Parsing {
                        stack: Vec::new(),
                        current: StackItem {
                            list_items: Vec::new(),
                        },
                    };
                }
                ParsedItem::ListEnd(_) => unreachable!(),
            },
            State::Parsing {
                ref mut stack,
                ref mut current,
            } => match parser.next_item()? {
                ParsedItem::Atom(atom, _) => {
                    current.list_items.push(TreeNode::Atom(atom.into()));
                }
                ParsedItem::ListStart(_) => {
                    let new_current = StackItem {
                        list_items: Vec::new(),
                    };
                    stack.push(core::mem::replace(current, new_current));
                }
                ParsedItem::ListEnd(_) => {
                    if let Some(previous) = stack.pop() {
                        let old_current = core::mem::replace(current, previous);
                        current
                            .list_items
                            .push(TreeNode::List(old_current.list_items));
                    } else {
                        let root_node = TreeNode::List(core::mem::take(&mut current.list_items));
                        state = State::Finished(root_node);
                    }
                }
            },
            State::Finished(root_node) => return Ok(root_node),
        }
    }
}
