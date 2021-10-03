use alloc::string::String;

use crate::{serialize_tree, sise_tree, Serializer, SerializerStyle, TreeNode};

const STYLE: SerializerStyle<'static> = SerializerStyle {
    line_break: "\n",
    indentation: "\t",
};

struct SerializerTest<'a> {
    root_node: TreeNode,
    expected_compact: &'a str,
    expected_spaced: &'a str,
}

impl<'a> SerializerTest<'a> {
    fn run(&self) {
        // compact
        let mut result = String::new();
        let mut serializer = Serializer::new(STYLE, &mut result);
        serialize_tree(&mut serializer, &self.root_node, usize::MAX);
        serializer.finish(false);
        assert_eq!(result, self.expected_compact);

        // spaced
        let mut result = String::new();
        let mut serializer = Serializer::new(STYLE, &mut result);
        serialize_tree(&mut serializer, &self.root_node, 0);
        serializer.finish(false);
        assert_eq!(result, self.expected_spaced);
    }
}

#[test]
fn test_empty_list() {
    SerializerTest {
        root_node: sise_tree!([]),
        expected_compact: "()",
        expected_spaced: "()",
    }
    .run();
}

#[test]
fn test_single_atom() {
    SerializerTest {
        root_node: sise_tree!("atom"),
        expected_compact: "atom",
        expected_spaced: "atom",
    }
    .run();
}

#[test]
fn test_list_with_one_atom() {
    SerializerTest {
        root_node: sise_tree!(["1"]),
        expected_compact: "(1)",
        expected_spaced: "(1)",
    }
    .run();
}

#[test]
fn test_list_with_two_atoms() {
    SerializerTest {
        root_node: sise_tree!(["1", "2"]),
        expected_compact: "(1 2)",
        expected_spaced: "(1\n\t2\n)",
    }
    .run();
}

#[test]
fn test_list_with_three_atoms() {
    SerializerTest {
        root_node: sise_tree!(["1", "2", "3"]),
        expected_compact: "(1 2 3)",
        expected_spaced: "(1\n\t2\n\t3\n)",
    }
    .run();
}

#[test]
fn test_nested_list_1() {
    SerializerTest {
        root_node: sise_tree!([[]]),
        expected_compact: "(())",
        expected_spaced: "(\n\t()\n)",
    }
    .run();
}

#[test]
fn test_nested_list_2() {
    SerializerTest {
        root_node: sise_tree!([[], []]),
        expected_compact: "(() ())",
        expected_spaced: "(\n\t()\n\t()\n)",
    }
    .run();
}

#[test]
fn test_mixed() {
    SerializerTest {
        root_node: sise_tree!([
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
    let mut result = String::new();
    let mut serializer = Serializer::new(STYLE, &mut result);

    serializer.begin_list(usize::MAX);
    serializer.put_atom("atom", usize::MAX);
    serializer.begin_list(0);
    serializer.put_atom("1", usize::MAX);
    serializer.put_atom("2", usize::MAX);
    serializer.put_atom("3", usize::MAX);
    serializer.end_list();
    serializer.begin_list(0);
    serializer.put_atom("a", usize::MAX);
    serializer.put_atom("b", usize::MAX);
    serializer.put_atom("c", usize::MAX);
    serializer.end_list();
    serializer.end_list();
    serializer.finish(false);

    let expected = "(atom\n\t(1 2 3)\n\t(a b c)\n)";
    assert_eq!(result, expected);
}

#[test]
fn test_spaced_with_keep_line_2() {
    let mut result = String::new();
    let mut serializer = Serializer::new(STYLE, &mut result);

    serializer.begin_list(usize::MAX);
    serializer.put_atom("atom", usize::MAX);
    serializer.begin_list(0);
    serializer.put_atom("1", usize::MAX);
    serializer.put_atom("2", usize::MAX);
    serializer.put_atom("3", usize::MAX);
    serializer.end_list();
    serializer.begin_list(usize::MAX);
    serializer.put_atom("a", usize::MAX);
    serializer.put_atom("b", usize::MAX);
    serializer.put_atom("c", usize::MAX);
    serializer.end_list();
    serializer.end_list();
    serializer.finish(false);

    let expected = "(atom\n\t(1 2 3) (a b c)\n)";
    assert_eq!(result, expected);
}

#[test]
fn test_spaced_with_keep_line_3() {
    let mut result = String::new();
    let mut serializer = Serializer::new(STYLE, &mut result);

    serializer.begin_list(usize::MAX);
    serializer.put_atom("atom", usize::MAX);
    serializer.begin_list(0);
    serializer.put_atom("1", usize::MAX);
    serializer.put_atom("2", usize::MAX);
    serializer.put_atom("3", 0);
    serializer.end_list();
    serializer.begin_list(0);
    serializer.put_atom("a", usize::MAX);
    serializer.put_atom("b", usize::MAX);
    serializer.put_atom("c", 0);
    serializer.end_list();
    serializer.end_list();
    serializer.finish(false);

    let expected = "(atom\n\t(1 2\n\t\t3\n\t)\n\t(a b\n\t\tc\n\t)\n)";
    assert_eq!(result, expected);
}
