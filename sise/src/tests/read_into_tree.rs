// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::sise_expr;
use crate::Node;
use crate::Reader;
use crate::Parser;
use crate::Pos;
use crate::PosTree;
use crate::read_into_tree;
use crate::read_into_tree_without_pos;

macro_rules! pos_tree {
    ($line:expr, $column:expr) => {
        crate::PosTree {
            pos: crate::Pos::new($line, $column),
            list: None,
        }
    };
    (
        $beg_line:expr, $beg_column:expr,
        [$(($($children:tt)*)),*],
        $end_line:expr, $end_column:expr
    ) => {
        crate::PosTree {
            pos: crate::Pos::new($beg_line, $beg_column),
            list: Some(crate::PosTreeList {
                items: vec![$(pos_tree!($($children)*)),*],
                ending_pos: Pos::new($end_line, $end_column),
            }),
        }
    };
}

struct ReadTreeTest<'a> {
    src_data: &'a [u8],
    expected_tree: Node,
    expected_positions: PosTree<Pos>,
}

impl<'a> ReadTreeTest<'a> {
    fn run(&self) {
        // with position tree
        let mut parser = Parser::new(self.src_data);
        let (root_node, root_pos_tree) = read_into_tree(&mut parser).unwrap();
        parser.finish().unwrap();
        assert_eq!(root_node, self.expected_tree);
        assert_eq!(root_pos_tree, self.expected_positions);

        // without position tree
        let mut parser = Parser::new(self.src_data);
        let root_node = read_into_tree_without_pos(&mut parser).unwrap();
        parser.finish().unwrap();
        assert_eq!(root_node, self.expected_tree);
    }
}

#[test]
fn test_empty_list() {
    ReadTreeTest {
        src_data: b"()",
        expected_tree: sise_expr!([]),
        expected_positions: pos_tree!(0, 0, [], 0, 1),
    }.run();
}

#[test]
fn test_single_atom() {
    ReadTreeTest {
        src_data: b"atom",
        expected_tree: sise_expr!("atom"),
        expected_positions: pos_tree!(0, 0),
    }.run();
}

#[test]
fn test_simple_list_1() {
    ReadTreeTest {
        src_data: b"(atom-1)",
        expected_tree: sise_expr!(["atom-1"]),
        expected_positions: pos_tree!(0, 0, [(0, 1)], 0, 7),
    }.run();
}

#[test]
fn test_simple_list_2() {
    ReadTreeTest {
        src_data: b"(atom-1 atom-2)",
        expected_tree: sise_expr!(["atom-1", "atom-2"]),
        expected_positions: pos_tree!(0, 0, [(0, 1), (0, 8)], 0, 14),
    }.run();
}

#[test]
fn test_nested_list_1() {
    ReadTreeTest {
        src_data: b"(())",
        expected_tree: sise_expr!([[]]),
        expected_positions: pos_tree!(0, 0, [(0, 1, [], 0, 2)], 0, 3),
    }.run();
}

#[test]
fn test_nested_list_2() {
    ReadTreeTest {
        src_data: b"(() ())",
        expected_tree: sise_expr!([[], []]),
        expected_positions: pos_tree!(0, 0, [(0, 1, [], 0, 2), (0, 4, [], 0, 5)], 0, 6),
    }.run();
}

#[test]
fn test_nested_list_3() {
    ReadTreeTest {
        src_data: b"((atom-1) (atom-2 atom-3))",
        expected_tree: sise_expr!([["atom-1"], ["atom-2", "atom-3"]]),
        expected_positions: pos_tree!(0, 0, [
            (0, 1, [(0, 2)], 0, 8),
            (0, 10, [(0, 11), (0, 18)], 0, 24)
        ], 0, 25),
    }.run();
}
