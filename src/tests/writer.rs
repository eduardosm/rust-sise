// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use alloc::string::String;

use crate::sise_expr;
use crate::write_from_tree;
use crate::CompactStringWriter;
use crate::Node;
use crate::SpacedStringWriter;
use crate::SpacedStringWriterNodeOptions;
use crate::SpacedStringWriterStyle;
use crate::TreeWriter;
use crate::Writer as _;

const SPACED_STYLE: SpacedStringWriterStyle<'static> = SpacedStringWriterStyle {
    line_break: "\n",
    indentation: "\t",
};

struct StringWriterTest<'a> {
    root_node: Node,
    expected_compact: &'a str,
    expected_spaced: &'a str,
}

impl<'a> StringWriterTest<'a> {
    fn run(&self) {
        // compact
        let mut result = String::new();
        let mut writer = CompactStringWriter::new(&mut result);
        write_from_tree(&mut writer, &self.root_node).unwrap();
        writer.finish(()).unwrap();
        assert_eq!(result, self.expected_compact);

        // spaced
        let mut result = String::new();
        let mut writer = SpacedStringWriter::new(SPACED_STYLE, &mut result);
        write_from_tree(&mut writer, &self.root_node).unwrap();
        writer.finish(()).unwrap();
        assert_eq!(result, self.expected_spaced);

        // tree
        let mut writer = TreeWriter::new();
        write_from_tree(&mut writer, &self.root_node).unwrap();
        let result = writer.finish(()).unwrap();
        assert_eq!(result, self.root_node);
    }
}

#[test]
fn test_empty_list() {
    StringWriterTest {
        root_node: sise_expr!([]),
        expected_compact: "()",
        expected_spaced: "()",
    }
    .run();
}

#[test]
fn test_single_atom() {
    StringWriterTest {
        root_node: sise_expr!("atom"),
        expected_compact: "atom",
        expected_spaced: "atom",
    }
    .run();
}

#[test]
fn test_list_with_one_atom() {
    StringWriterTest {
        root_node: sise_expr!(["1"]),
        expected_compact: "(1)",
        expected_spaced: "(1)",
    }
    .run();
}

#[test]
fn test_list_with_two_atoms() {
    StringWriterTest {
        root_node: sise_expr!(["1", "2"]),
        expected_compact: "(1 2)",
        expected_spaced: "(1\n\t2\n)",
    }
    .run();
}

#[test]
fn test_list_with_three_atoms() {
    StringWriterTest {
        root_node: sise_expr!(["1", "2", "3"]),
        expected_compact: "(1 2 3)",
        expected_spaced: "(1\n\t2\n\t3\n)",
    }
    .run();
}

#[test]
fn test_nested_list_1() {
    StringWriterTest {
        root_node: sise_expr!([[]]),
        expected_compact: "(())",
        expected_spaced: "(\n\t()\n)",
    }
    .run();
}

#[test]
fn test_nested_list_2() {
    StringWriterTest {
        root_node: sise_expr!([[], []]),
        expected_compact: "(() ())",
        expected_spaced: "(\n\t()\n\t()\n)",
    }
    .run();
}

#[test]
fn test_mixed() {
    StringWriterTest {
        root_node: sise_expr!([
            "atom-1",
            ["atom-2"],
            ["atom-3", ["atom-4"], "atom-5"],
            "atom-6"
        ]),
        expected_compact: "(atom-1 (atom-2) (atom-3 (atom-4) atom-5) atom-6)",
        expected_spaced:
            "(atom-1\n\t(atom-2)\n\t(atom-3\n\t\t(atom-4)\n\t\tatom-5\n\t)\n\tatom-6\n)",
    }
    .run();
}

#[test]
fn test_spaced_with_keep_line_1() {
    //let root_node = sise_expr!(["atom", ["1", "2", "3"], ["a", "b", "c"]]);

    let mut result = String::new();
    let mut writer = SpacedStringWriter::new(SPACED_STYLE, &mut result);

    let no_break_line_opts = SpacedStringWriterNodeOptions {
        break_line_len: usize::max_value(),
    };
    let break_line_opts = SpacedStringWriterNodeOptions { break_line_len: 0 };

    writer.begin_list(no_break_line_opts).unwrap();
    writer.write_atom("atom", no_break_line_opts).unwrap();
    writer.begin_list(break_line_opts).unwrap();
    writer.write_atom("1", no_break_line_opts).unwrap();
    writer.write_atom("2", no_break_line_opts).unwrap();
    writer.write_atom("3", no_break_line_opts).unwrap();
    writer.end_list(()).unwrap();
    writer.begin_list(break_line_opts).unwrap();
    writer.write_atom("a", no_break_line_opts).unwrap();
    writer.write_atom("b", no_break_line_opts).unwrap();
    writer.write_atom("c", no_break_line_opts).unwrap();
    writer.end_list(()).unwrap();
    writer.end_list(()).unwrap();
    writer.finish(()).unwrap();

    let expected = "(atom\n\t(1 2 3)\n\t(a b c)\n)";
    assert_eq!(result, expected);
}

#[test]
fn test_spaced_with_keep_line_2() {
    //let root_node = sise_expr!(["atom", ["1", "2", "3"], ["a", "b", "c"]]);

    let mut result = String::new();
    let mut writer = SpacedStringWriter::new(SPACED_STYLE, &mut result);

    let no_break_line_opts = SpacedStringWriterNodeOptions {
        break_line_len: usize::max_value(),
    };
    let break_line_opts = SpacedStringWriterNodeOptions { break_line_len: 0 };

    writer.begin_list(no_break_line_opts).unwrap();
    writer.write_atom("atom", no_break_line_opts).unwrap();
    writer.begin_list(break_line_opts).unwrap();
    writer.write_atom("1", no_break_line_opts).unwrap();
    writer.write_atom("2", no_break_line_opts).unwrap();
    writer.write_atom("3", no_break_line_opts).unwrap();
    writer.end_list(()).unwrap();
    writer.begin_list(no_break_line_opts).unwrap();
    writer.write_atom("a", no_break_line_opts).unwrap();
    writer.write_atom("b", no_break_line_opts).unwrap();
    writer.write_atom("c", no_break_line_opts).unwrap();
    writer.end_list(()).unwrap();
    writer.end_list(()).unwrap();
    writer.finish(()).unwrap();

    let expected = "(atom\n\t(1 2 3) (a b c)\n)";
    assert_eq!(result, expected);
}

#[test]
fn test_spaced_with_keep_line_3() {
    //let root_node = sise_expr!(["atom", ["1", "2", "3"], ["a", "b", "c"]]);

    let mut result = String::new();
    let mut writer = SpacedStringWriter::new(SPACED_STYLE, &mut result);

    let no_break_line_opts = SpacedStringWriterNodeOptions {
        break_line_len: usize::max_value(),
    };
    let break_line_opts = SpacedStringWriterNodeOptions { break_line_len: 0 };

    writer.begin_list(no_break_line_opts).unwrap();
    writer.write_atom("atom", no_break_line_opts).unwrap();
    writer.begin_list(break_line_opts).unwrap();
    writer.write_atom("1", no_break_line_opts).unwrap();
    writer.write_atom("2", no_break_line_opts).unwrap();
    writer.write_atom("3", break_line_opts).unwrap();
    writer.end_list(()).unwrap();
    writer.begin_list(break_line_opts).unwrap();
    writer.write_atom("a", no_break_line_opts).unwrap();
    writer.write_atom("b", no_break_line_opts).unwrap();
    writer.write_atom("c", break_line_opts).unwrap();
    writer.end_list(()).unwrap();
    writer.end_list(()).unwrap();
    writer.finish(()).unwrap();

    let expected = "(atom\n\t(1 2\n\t\t3\n\t)\n\t(a b\n\t\tc\n\t)\n)";
    assert_eq!(result, expected);
}
