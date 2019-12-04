// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::sise_expr;
use crate::Node;
use crate::ReadItem;
use crate::ReadItemKind;
use crate::Reader as _;
use crate::TreeReader;

struct TreeReaderTest<'a> {
    tree: Node,
    expected_items: &'a [ReadItem<&'a str, ()>],
}

impl<'a> TreeReaderTest<'a> {
    fn run(&self) {
        let mut reader = TreeReader::new(&self.tree);
        for read_item in self.expected_items.iter() {
            assert_eq!(reader.read().unwrap(), *read_item);
        }
        reader.finish().unwrap();
    }
}

#[test]
fn test_empty_list() {
    TreeReaderTest {
        tree: sise_expr!([]),
        expected_items: &[
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_single_atom() {
    TreeReaderTest {
        tree: sise_expr!("atom"),
        expected_items: &[ReadItem {
            pos: (),
            kind: ReadItemKind::Atom("atom"),
        }],
    }
    .run();
}

#[test]
fn test_simple_list_1() {
    TreeReaderTest {
        tree: sise_expr!(["atom-1"]),
        expected_items: &[
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_simple_list_2() {
    TreeReaderTest {
        tree: sise_expr!(["atom-1", "atom-2"]),
        expected_items: &[
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-2"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_nested_list_1() {
    TreeReaderTest {
        tree: sise_expr!([[]]),
        expected_items: &[
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_nested_list_2() {
    TreeReaderTest {
        tree: sise_expr!([[], []]),
        expected_items: &[
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_nested_list_3() {
    TreeReaderTest {
        tree: sise_expr!([["atom-1"], ["atom-2", "atom-3"]]),
        expected_items: &[
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-2"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-3"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_nested_lists() {
    TreeReaderTest {
        tree: sise_expr!([[[[[[[[[[]]]]]]]]]]),
        expected_items: &[
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_mixed() {
    TreeReaderTest {
        tree: sise_expr!([
            "atom-1",
            ["atom-2"],
            ["atom-3", ["atom-4"], "atom-5"],
            "atom-6"
        ]),
        expected_items: &[
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-2"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-3"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-4"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-5"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::Atom("atom-6"),
            },
            ReadItem {
                pos: (),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}
