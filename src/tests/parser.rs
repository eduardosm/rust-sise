use crate::{ParseError, ParsedItem, Parser};

struct ParserPassTest<'a> {
    src_data: &'a str,
    expected_items: &'a [ParsedItem<'a>],
}

impl<'a> ParserPassTest<'a> {
    #[track_caller]
    fn run(&self) {
        let mut parser = Parser::new(self.src_data);
        for parsed_item in self.expected_items.iter() {
            assert_eq!(parser.next_item().unwrap(), *parsed_item);
        }
        parser.finish().unwrap();
    }
}

struct ParserFailTest<'a> {
    src_data: &'a str,
    expected_items: &'a [ParsedItem<'a>],
    error_at_finish: bool,
    expected_error: ParseError,
}

impl<'a> ParserFailTest<'a> {
    #[track_caller]
    fn run(&self) {
        let mut parser = Parser::new(self.src_data);
        for parsed_item in self.expected_items.iter() {
            assert_eq!(parser.next_item().unwrap(), *parsed_item);
        }
        if self.error_at_finish {
            assert_eq!(parser.finish().unwrap_err(), self.expected_error);
        } else {
            assert_eq!(parser.next_item().unwrap_err(), self.expected_error);
        }
    }
}

#[test]
fn test_empty_list() {
    ParserPassTest {
        src_data: "()",
        expected_items: &[ParsedItem::ListStart(0), ParsedItem::ListEnd(1)],
    }
    .run();
}

#[test]
fn test_single_atom() {
    ParserPassTest {
        src_data: "atom",
        expected_items: &[ParsedItem::Atom("atom", 0)],
    }
    .run();
}

#[test]
fn test_simple_list_1() {
    ParserPassTest {
        src_data: "(atom-1)",
        expected_items: &[
            ParsedItem::ListStart(0),
            ParsedItem::Atom("atom-1", 1),
            ParsedItem::ListEnd(7),
        ],
    }
    .run();
}

#[test]
fn test_simple_list_2() {
    ParserPassTest {
        src_data: "(atom-1 atom-2)",
        expected_items: &[
            ParsedItem::ListStart(0),
            ParsedItem::Atom("atom-1", 1),
            ParsedItem::Atom("atom-2", 8),
            ParsedItem::ListEnd(14),
        ],
    }
    .run();
}

#[test]
fn test_nested_list_1() {
    ParserPassTest {
        src_data: "(())",
        expected_items: &[
            ParsedItem::ListStart(0),
            ParsedItem::ListStart(1),
            ParsedItem::ListEnd(2),
            ParsedItem::ListEnd(3),
        ],
    }
    .run();
}

#[test]
fn test_nested_list_2() {
    ParserPassTest {
        src_data: "(() ())",
        expected_items: &[
            ParsedItem::ListStart(0),
            ParsedItem::ListStart(1),
            ParsedItem::ListEnd(2),
            ParsedItem::ListStart(4),
            ParsedItem::ListEnd(5),
            ParsedItem::ListEnd(6),
        ],
    }
    .run();
}

#[test]
fn test_nested_list_3() {
    ParserPassTest {
        src_data: "((atom-1) (atom-2 atom-3))",
        expected_items: &[
            ParsedItem::ListStart(0),
            ParsedItem::ListStart(1),
            ParsedItem::Atom("atom-1", 2),
            ParsedItem::ListEnd(8),
            ParsedItem::ListStart(10),
            ParsedItem::Atom("atom-2", 11),
            ParsedItem::Atom("atom-3", 18),
            ParsedItem::ListEnd(24),
            ParsedItem::ListEnd(25),
        ],
    }
    .run();
}

#[test]
fn test_nested_lists() {
    ParserPassTest {
        src_data: "(((((((((())))))))))",
        expected_items: &[
            ParsedItem::ListStart(0),
            ParsedItem::ListStart(1),
            ParsedItem::ListStart(2),
            ParsedItem::ListStart(3),
            ParsedItem::ListStart(4),
            ParsedItem::ListStart(5),
            ParsedItem::ListStart(6),
            ParsedItem::ListStart(7),
            ParsedItem::ListStart(8),
            ParsedItem::ListStart(9),
            ParsedItem::ListEnd(10),
            ParsedItem::ListEnd(11),
            ParsedItem::ListEnd(12),
            ParsedItem::ListEnd(13),
            ParsedItem::ListEnd(14),
            ParsedItem::ListEnd(15),
            ParsedItem::ListEnd(16),
            ParsedItem::ListEnd(17),
            ParsedItem::ListEnd(18),
            ParsedItem::ListEnd(19),
        ],
    }
    .run();
}

#[test]
fn test_mixed() {
    ParserPassTest {
        src_data: "(atom-1 (atom-2) (atom-3 (atom-4) atom-5) atom-6)",
        expected_items: &[
            ParsedItem::ListStart(0),
            ParsedItem::Atom("atom-1", 1),
            ParsedItem::ListStart(8),
            ParsedItem::Atom("atom-2", 9),
            ParsedItem::ListEnd(15),
            ParsedItem::ListStart(17),
            ParsedItem::Atom("atom-3", 18),
            ParsedItem::ListStart(25),
            ParsedItem::Atom("atom-4", 26),
            ParsedItem::ListEnd(32),
            ParsedItem::Atom("atom-5", 34),
            ParsedItem::ListEnd(40),
            ParsedItem::Atom("atom-6", 42),
            ParsedItem::ListEnd(48),
        ],
    }
    .run();
}

#[test]
fn test_atom_chars() {
    ParserPassTest {
        src_data: "!#$%&*+-./:<=>?@_~",
        expected_items: &[ParsedItem::Atom("!#$%&*+-./:<=>?@_~", 0)],
    }
    .run();
}

#[test]
fn test_string_1() {
    ParserPassTest {
        src_data: "\"atom-1\"",
        expected_items: &[ParsedItem::Atom("\"atom-1\"", 0)],
    }
    .run();
}

#[test]
fn test_string_2() {
    ParserPassTest {
        src_data: "prefix\"atom-1\"suffix",
        expected_items: &[ParsedItem::Atom("prefix\"atom-1\"suffix", 0)],
    }
    .run();
}

#[test]
fn test_string_3() {
    ParserPassTest {
        src_data: "\" \\\\ \\\" \"",
        expected_items: &[ParsedItem::Atom("\" \\\\ \\\" \"", 0)],
    }
    .run();
}

#[test]
fn test_multiline() {
    ParserPassTest {
        src_data: "\n(1 2\r3\r\n4 5)\n",
        expected_items: &[
            ParsedItem::ListStart(1),
            ParsedItem::Atom("1", 2),
            ParsedItem::Atom("2", 4),
            ParsedItem::Atom("3", 6),
            ParsedItem::Atom("4", 9),
            ParsedItem::Atom("5", 11),
            ParsedItem::ListEnd(12),
        ],
    }
    .run();
}

#[test]
fn test_comment_1_lf() {
    ParserPassTest {
        src_data: "; comment\n(1 2)",
        expected_items: &[
            ParsedItem::ListStart(10),
            ParsedItem::Atom("1", 11),
            ParsedItem::Atom("2", 13),
            ParsedItem::ListEnd(14),
        ],
    }
    .run();
}

#[test]
fn test_comment_1_cr() {
    ParserPassTest {
        src_data: "; comment\r(1 2)",
        expected_items: &[
            ParsedItem::ListStart(10),
            ParsedItem::Atom("1", 11),
            ParsedItem::Atom("2", 13),
            ParsedItem::ListEnd(14),
        ],
    }
    .run();
}

#[test]
fn test_comment_2() {
    ParserPassTest {
        src_data: "(1 2); comment",
        expected_items: &[
            ParsedItem::ListStart(0),
            ParsedItem::Atom("1", 1),
            ParsedItem::Atom("2", 3),
            ParsedItem::ListEnd(4),
        ],
    }
    .run();
}

#[test]
fn test_comment_3() {
    ParserPassTest {
        src_data: "(1; comment\n2)",
        expected_items: &[
            ParsedItem::ListStart(0),
            ParsedItem::Atom("1", 1),
            ParsedItem::Atom("2", 12),
            ParsedItem::ListEnd(13),
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
        expected_error: ParseError::UnexpectedEof { pos: 0 },
    }
    .run();
}

#[test]
fn test_fail_expected_eof() {
    ParserFailTest {
        src_data: "() ()",
        expected_items: &[ParsedItem::ListStart(0), ParsedItem::ListEnd(1)],
        error_at_finish: true,
        expected_error: ParseError::ExpectedEof { pos: 3 },
    }
    .run();
}

#[test]
fn test_fail_unfinished_string() {
    ParserFailTest {
        src_data: "\"atom-1",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::UnfinishedString { pos: 7 },
    }
    .run();
}

#[test]
fn test_fail_unclosed_list() {
    ParserFailTest {
        src_data: "(atom-1",
        expected_items: &[ParsedItem::ListStart(0), ParsedItem::Atom("atom-1", 1)],
        error_at_finish: false,
        expected_error: ParseError::UnexpectedEof { pos: 7 },
    }
    .run();
}

#[test]
fn test_fail_unexpected_closing() {
    ParserFailTest {
        src_data: ")",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::UnexpectedRightParen { pos: 0 },
    }
    .run();
}

#[test]
fn test_fail_unclosed_list_with_comment() {
    ParserFailTest {
        src_data: "(atom-1 ; comment)",
        expected_items: &[ParsedItem::ListStart(0), ParsedItem::Atom("atom-1", 1)],
        error_at_finish: false,
        expected_error: ParseError::UnexpectedEof { pos: 18 },
    }
    .run();
}

#[test]
fn test_fail_illegal_chr() {
    ParserFailTest {
        src_data: "\0",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::IllegalChr { pos: 0, chr: '\0' },
    }
    .run();
}

#[test]
fn test_fail_illegal_chr_in_string() {
    ParserFailTest {
        src_data: "\"\0",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::IllegalChrInString { pos: 1, chr: '\0' },
    }
    .run();
}

#[test]
fn test_fail_illegal_chr_in_comment() {
    ParserFailTest {
        src_data: "() ; \0",
        expected_items: &[ParsedItem::ListStart(0), ParsedItem::ListEnd(1)],
        error_at_finish: true,
        expected_error: ParseError::IllegalChrInComment { pos: 5, chr: '\0' },
    }
    .run();
}
