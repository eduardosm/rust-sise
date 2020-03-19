// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::BytePos;
use crate::ParseError;
use crate::Parser;
use crate::ReadItem;
use crate::ReadItemKind;
use crate::Reader as _;
use crate::TokenKind;

struct ParserPassTest<'a> {
    src_data: &'a str,
    expected_items: &'a [ReadItem<&'a str, BytePos>],
}

impl<'a> ParserPassTest<'a> {
    fn run(&self) {
        let mut parser = Parser::new(self.src_data);
        for read_item in self.expected_items.iter() {
            assert_eq!(parser.read().unwrap(), *read_item);
        }
        parser.finish().unwrap();
    }
}

struct ParserFailTest<'a> {
    src_data: &'a str,
    expected_items: &'a [ReadItem<&'a str, BytePos>],
    error_at_finish: bool,
    expected_error: ParseError,
}

impl<'a> ParserFailTest<'a> {
    fn run(&self) {
        let mut parser = Parser::new(self.src_data);
        for read_item in self.expected_items.iter() {
            assert_eq!(parser.read().unwrap(), *read_item);
        }
        if self.error_at_finish {
            assert_eq!(parser.finish().unwrap_err(), self.expected_error);
        } else {
            assert_eq!(parser.read().unwrap_err(), self.expected_error);
        }
    }
}

#[test]
fn test_empty_list() {
    ParserPassTest {
        src_data: "()",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_single_atom() {
    ParserPassTest {
        src_data: "atom",
        expected_items: &[ReadItem {
            pos: BytePos(0),
            kind: ReadItemKind::Atom("atom"),
        }],
    }
    .run();
}

#[test]
fn test_simple_list_1() {
    ParserPassTest {
        src_data: "(atom-1)",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: BytePos(7),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_simple_list_2() {
    ParserPassTest {
        src_data: "(atom-1 atom-2)",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: BytePos(8),
                kind: ReadItemKind::Atom("atom-2"),
            },
            ReadItem {
                pos: BytePos(14),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_nested_list_1() {
    ParserPassTest {
        src_data: "(())",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(2),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(3),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_nested_list_2() {
    ParserPassTest {
        src_data: "(() ())",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(2),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(4),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(5),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(6),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_nested_list_3() {
    ParserPassTest {
        src_data: "((atom-1) (atom-2 atom-3))",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(2),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: BytePos(8),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(10),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(11),
                kind: ReadItemKind::Atom("atom-2"),
            },
            ReadItem {
                pos: BytePos(18),
                kind: ReadItemKind::Atom("atom-3"),
            },
            ReadItem {
                pos: BytePos(24),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(25),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_nested_lists() {
    ParserPassTest {
        src_data: "(((((((((())))))))))",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(2),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(3),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(4),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(5),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(6),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(7),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(8),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(9),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(10),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(11),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(12),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(13),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(14),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(15),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(16),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(17),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(18),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(19),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_mixed() {
    ParserPassTest {
        src_data: "(atom-1 (atom-2) (atom-3 (atom-4) atom-5) atom-6)",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: BytePos(8),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(9),
                kind: ReadItemKind::Atom("atom-2"),
            },
            ReadItem {
                pos: BytePos(15),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(17),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(18),
                kind: ReadItemKind::Atom("atom-3"),
            },
            ReadItem {
                pos: BytePos(25),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(26),
                kind: ReadItemKind::Atom("atom-4"),
            },
            ReadItem {
                pos: BytePos(32),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(34),
                kind: ReadItemKind::Atom("atom-5"),
            },
            ReadItem {
                pos: BytePos(40),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: BytePos(42),
                kind: ReadItemKind::Atom("atom-6"),
            },
            ReadItem {
                pos: BytePos(48),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_atom_chars() {
    ParserPassTest {
        src_data: "!#$%&*+-./:<=>?@_~",
        expected_items: &[ReadItem {
            pos: BytePos(0),
            kind: ReadItemKind::Atom("!#$%&*+-./:<=>?@_~"),
        }],
    }
    .run();
}

#[test]
fn test_string_1() {
    ParserPassTest {
        src_data: "\"atom-1\"",
        expected_items: &[ReadItem {
            pos: BytePos(0),
            kind: ReadItemKind::Atom("\"atom-1\""),
        }],
    }
    .run();
}

#[test]
fn test_string_2() {
    ParserPassTest {
        src_data: "prefix\"atom-1\"suffix",
        expected_items: &[ReadItem {
            pos: BytePos(0),
            kind: ReadItemKind::Atom("prefix\"atom-1\"suffix"),
        }],
    }
    .run();
}

#[test]
fn test_string_3() {
    ParserPassTest {
        src_data: "\" \\\\ \\\" \"",
        expected_items: &[ReadItem {
            pos: BytePos(0),
            kind: ReadItemKind::Atom("\" \\\\ \\\" \""),
        }],
    }
    .run();
}

#[test]
fn test_multiline() {
    ParserPassTest {
        src_data: "\n(1 2\r3\r\n4 5)\n",
        expected_items: &[
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(2),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: BytePos(4),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: BytePos(6),
                kind: ReadItemKind::Atom("3"),
            },
            ReadItem {
                pos: BytePos(9),
                kind: ReadItemKind::Atom("4"),
            },
            ReadItem {
                pos: BytePos(11),
                kind: ReadItemKind::Atom("5"),
            },
            ReadItem {
                pos: BytePos(12),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_comment_1_lf() {
    ParserPassTest {
        src_data: "; comment\n(1 2)",
        expected_items: &[
            ReadItem {
                pos: BytePos(10),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(11),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: BytePos(13),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: BytePos(14),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_comment_1_cr() {
    ParserPassTest {
        src_data: "; comment\r(1 2)",
        expected_items: &[
            ReadItem {
                pos: BytePos(10),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(11),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: BytePos(13),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: BytePos(14),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_comment_2() {
    ParserPassTest {
        src_data: "(1 2); comment",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: BytePos(3),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: BytePos(4),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_comment_3() {
    ParserPassTest {
        src_data: "(1; comment\n2)",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: BytePos(12),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: BytePos(13),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }
    .run();
}

#[test]
fn test_fail_empty() {
    ParserFailTest {
        src_data: "",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::UnexpectedToken {
            pos: BytePos(0),
            token: TokenKind::Eof,
        },
    }
    .run();
}

#[test]
fn test_fail_expected_eof() {
    ParserFailTest {
        src_data: "() ()",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::ListEnding,
            },
        ],
        error_at_finish: true,
        expected_error: ParseError::ExpectedEof { pos: BytePos(3) },
    }
    .run();
}

#[test]
fn test_fail_unfinished_string() {
    ParserFailTest {
        src_data: "\"atom-1",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::UnfinishedString { pos: BytePos(7) },
    }
    .run();
}

#[test]
fn test_fail_unclosed_list() {
    ParserFailTest {
        src_data: "(atom-1",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::Atom("atom-1"),
            },
        ],
        error_at_finish: false,
        expected_error: ParseError::UnexpectedToken {
            pos: BytePos(7),
            token: TokenKind::Eof,
        },
    }
    .run();
}

#[test]
fn test_fail_unexpected_closing() {
    ParserFailTest {
        src_data: ")",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::UnexpectedToken {
            pos: BytePos(0),
            token: TokenKind::RightParen,
        },
    }
    .run();
}

#[test]
fn test_fail_unclosed_list_with_comment() {
    ParserFailTest {
        src_data: "(atom-1 ; comment)",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::Atom("atom-1"),
            },
        ],
        error_at_finish: false,
        expected_error: ParseError::UnexpectedToken {
            pos: BytePos(18),
            token: TokenKind::Eof,
        },
    }
    .run();
}

#[test]
fn test_fail_illegal_chr() {
    ParserFailTest {
        src_data: "\0",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::IllegalChr {
            pos: BytePos(0),
            chr: '\0',
        },
    }
    .run();
}

#[test]
fn test_fail_illegal_chr_in_string() {
    ParserFailTest {
        src_data: "\"\0",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::IllegalChrInString {
            pos: BytePos(1),
            chr: '\0',
        },
    }
    .run();
}

#[test]
fn test_fail_illegal_chr_in_comment() {
    ParserFailTest {
        src_data: "() ; \0",
        expected_items: &[
            ReadItem {
                pos: BytePos(0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: BytePos(1),
                kind: ReadItemKind::ListEnding,
            },
        ],
        error_at_finish: true,
        expected_error: ParseError::IllegalChrInComment {
            pos: BytePos(5),
            chr: '\0',
        },
    }
    .run();
}
