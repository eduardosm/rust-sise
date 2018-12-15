// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate sise;

#[cfg(test)]
mod tests;

/// Trait that allows to implement serialization styles.
pub trait Style<'a> {
    /// Called at the beginning of the serialization (nothing has been writen to
    /// output yet).
    fn begin(&mut self, output: &mut String);

    /// Called at the beginning of a list (before writing `(` to `output`)
    fn begin_list(&mut self, list_node: &'a sise::Node, output: &mut String);

    /// Called at the end of a list (before writing `)` to `output`)
    fn end_list(&mut self, output: &mut String);

    /// Called before writing an atom to output.
    fn atom(&mut self, atom_node: &'a sise::Node, output: &mut String);

    /// Called at the end of the serialization (nothing more will be writen
    /// to output).
    fn finish(&mut self, output: &mut String);
}

mod compact_style {
    #[derive(Debug)]
    enum State {
        Invalid,

        Beginning,
        ListBeginning,
        List,
        Finishing,
    }

    /// Compact style that only inserts spaces between list elements.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate sise;
    /// use sise::sise_expr;
    ///
    /// let tree = sise_expr!(["example", ["1", "2", "3"], ["a", "b", "c"]]);
    ///
    /// let compact = sise_encoder::serialize(&tree, &mut sise_encoder::CompactStyle::new());
    /// assert_eq!(compact, "(example (1 2 3) (a b c))");
    /// ```
    #[derive(Debug)]
    pub struct CompactStyle {
        state: State,
        depth: usize,
    }

    impl CompactStyle {
        pub fn new() -> Self {
            CompactStyle {
                state: State::Invalid,
                depth: 0,
            }
        }

        fn get_state(&mut self) -> State {
            std::mem::replace(&mut self.state, State::Invalid)
        }
    }

    impl<'a> crate::Style<'a> for CompactStyle {
        fn begin(&mut self, _output: &mut String) {
            self.state = State::Beginning;
        }

        fn begin_list(&mut self, _list_node: &'a sise::Node, output: &mut String) {
            match self.get_state() {
                State::Invalid => unreachable!(),
                State::Beginning => {
                    self.depth += 1;
                    self.state = State::ListBeginning;
                }
                State::ListBeginning => {
                    self.depth += 1;
                    self.state = State::ListBeginning;
                }
                State::List => {
                    output.push(' ');
                    self.depth += 1;
                    self.state = State::ListBeginning;
                }
                State::Finishing => unreachable!(),
            }
        }

        fn end_list(&mut self, _output: &mut String) {
            match self.get_state() {
                State::Invalid => unreachable!(),
                State::Beginning => unreachable!(),
                State::ListBeginning => {
                    self.depth -= 1;
                    if self.depth == 0 {
                        self.state = State::Finishing;
                    } else {
                        self.state = State::List;
                    }
                }
                State::List => {
                    self.depth -= 1;
                    if self.depth == 0 {
                        self.state = State::Finishing;
                    } else {
                        self.state = State::List;
                    }
                }
                State::Finishing => unreachable!(),
            }
        }

        fn atom(&mut self, _atom_node: &'a sise::Node, output: &mut String) {
            match self.get_state() {
                State::Invalid => unreachable!(),
                State::Beginning => {
                    self.state = State::Finishing;
                }
                State::ListBeginning => {
                    self.state = State::List;
                }
                State::List => {
                    output.push(' ');
                    self.state = State::List;
                }
                State::Finishing => unreachable!(),
            }
        }

        fn finish(&mut self, _output: &mut String) {
            match self.get_state() {
                State::Finishing => {
                    assert_eq!(self.depth, 0);
                }
                _ => unreachable!(),
            }
        }
    }
}
pub use self::compact_style::CompactStyle;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LineEnding {
    Lf,
    CrLf,
    Cr,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IndentChar {
    Space,
    Tab,
}

impl IndentChar {
    #[inline]
    fn get_char(&self) -> char {
        match *self {
            IndentChar::Space => ' ',
            IndentChar::Tab => '\t',
        }
    }
}

/// Structure to configure the `SpacedStyle` style.
#[derive(Clone, Debug)]
pub struct SpacingConfig {
    pub line_ending: LineEnding,
    pub indent_len: usize,
    pub indent_char: IndentChar,
}

impl SpacingConfig {
    fn put_new_line(&self, output: &mut String) {
        match self.line_ending {
            LineEnding::Lf => output.push('\n'),
            LineEnding::CrLf => output.push_str("\r\n"),
            LineEnding::Cr => output.push('\r'),
        }
    }

    fn put_indent(&self, depth: usize, output: &mut String) {
        let total = depth.checked_mul(self.indent_len).unwrap();
        output.reserve(total);
        let chr = self.indent_char.get_char();
        for _ in 0 .. total {
            output.push(chr);
        }
    }
}

mod spaced_style {
    use std::collections::HashSet;

    #[derive(Debug)]
    enum State {
        Invalid,

        Beginning,
        ListBeginning,
        List(bool),
        Finishing,
    }

    #[derive(Debug)]
    enum StackItem {
        List(bool),
    }

    /// Style that breaks list in lines and indents them.
    ///
    /// # Example
    ///
    /// ```
    /// use sise::sise_expr;
    ///
    /// let tree = sise_expr!(["example", ["1", "2", "3"], ["a", "b", "c"]]);
    ///
    /// let spacing_config = sise_encoder::SpacingConfig {
    ///     line_ending: sise_encoder::LineEnding::Lf,
    ///     indent_len: 4,
    ///     indent_char: sise_encoder::IndentChar::Space,
    /// };
    /// let mut keep_same_line = std::collections::HashSet::new();
    /// keep_same_line.insert(tree.index_path(&[1, 1]).unwrap().ref_as_usize());
    /// keep_same_line.insert(tree.index_path(&[1, 2]).unwrap().ref_as_usize());
    /// keep_same_line.insert(tree.index_path(&[2, 1]).unwrap().ref_as_usize());
    /// keep_same_line.insert(tree.index_path(&[2, 2]).unwrap().ref_as_usize());
    /// let spaced = sise_encoder::serialize(&tree, &mut sise_encoder::SpacedStyle::new(spacing_config, keep_same_line));
    /// assert_eq!(spaced, "(example\n    (1 2 3)\n    (a b c)\n)\n");
    /// ```
    ///
    /// # Example with 'sise::Builder'
    ///
    /// ```
    /// let mut builder_base = sise::BuilderBase::new();
    /// let mut builder = builder_base.builder();
    ///
    /// let mut keep_same_line_paths = Vec::new();
    ///
    /// builder.push_atom(String::from("example"));
    /// builder.begin_list();
    /// builder.push_atom(String::from("1"));
    /// builder.push_atom(String::from("2"));
    /// keep_same_line_paths.push(builder.last_index_path());
    /// builder.push_atom(String::from("3"));
    /// keep_same_line_paths.push(builder.last_index_path());
    /// builder.end_list();
    /// builder.begin_list();
    /// builder.push_atom(String::from("a"));
    /// builder.push_atom(String::from("b"));
    /// keep_same_line_paths.push(builder.last_index_path());
    /// builder.push_atom(String::from("c"));
    /// keep_same_line_paths.push(builder.last_index_path());
    /// builder.end_list();
    /// builder.finish();
    /// let tree = builder_base.into_node();
    ///
    /// let spacing_config = sise_encoder::SpacingConfig {
    ///     line_ending: sise_encoder::LineEnding::Lf,
    ///     indent_len: 4,
    ///     indent_char: sise_encoder::IndentChar::Space,
    /// };
    /// let mut keep_same_line = std::collections::HashSet::new();
    /// for keep_same_line_path in keep_same_line_paths {
    ///     keep_same_line.insert(tree.index_path(&keep_same_line_path).unwrap().ref_as_usize());
    /// }
    /// let spaced = sise_encoder::serialize(&tree, &mut sise_encoder::SpacedStyle::new(spacing_config, keep_same_line));
    /// assert_eq!(spaced, "(example\n    (1 2 3)\n    (a b c)\n)\n");
    /// ```
    #[derive(Debug)]
    pub struct SpacedStyle {
        spacing_config: crate::SpacingConfig,
        keep_same_line: HashSet<usize>,

        indent_depth: usize,
        state: State,
        stack: Vec<StackItem>,
    }

    impl SpacedStyle {
        pub fn new(spacing_config: crate::SpacingConfig, keep_same_line: HashSet<usize>) -> Self {
            SpacedStyle {
                spacing_config: spacing_config,
                keep_same_line: keep_same_line,

                indent_depth: 0,
                state: State::Invalid,
                stack: Vec::new(),
            }
        }

        fn get_state(&mut self) -> State {
            std::mem::replace(&mut self.state, State::Invalid)
        }

        fn keep_same_line(&self, node: &sise::Node) -> bool {
            self.keep_same_line.contains(&node.ref_as_usize())
        }
    }

    impl<'a> crate::Style<'a> for SpacedStyle {
        fn begin(&mut self, _output: &mut String) {
            self.state = State::Beginning;
        }

        fn begin_list(&mut self, list_node: &'a sise::Node, output: &mut String) {
            match self.get_state() {
                State::Invalid => unreachable!(),
                State::Beginning => {
                    self.state = State::ListBeginning;
                }
                State::ListBeginning => {
                    if self.keep_same_line(list_node) {
                        self.stack.push(StackItem::List(false));
                        self.state = State::ListBeginning;
                    } else {
                        self.indent_depth += 1;
                        self.spacing_config.put_new_line(output);
                        self.spacing_config.put_indent(self.indent_depth, output);
                        self.stack.push(StackItem::List(true));
                        self.state = State::ListBeginning;
                    }
                }
                State::List(line_broken) => {
                    if self.keep_same_line(list_node) {
                        output.push(' ');
                        self.stack.push(StackItem::List(line_broken));
                        self.state = State::ListBeginning;
                    } else {
                        if !line_broken {
                            self.indent_depth += 1;
                        }
                        self.spacing_config.put_new_line(output);
                        self.spacing_config.put_indent(self.indent_depth, output);
                        self.stack.push(StackItem::List(true));
                        self.state = State::ListBeginning;
                    }
                }
                State::Finishing => unreachable!(),
            }
        }

        fn end_list(&mut self, output: &mut String) {
            match self.get_state() {
                State::Invalid => unreachable!(),
                State::Beginning => unreachable!(),
                State::ListBeginning => {
                    match self.stack.pop() {
                        Some(StackItem::List(parent_line_broken)) => {
                            self.state = State::List(parent_line_broken);
                        }
                        None => {
                            self.state = State::Finishing;
                        }
                    }
                }
                State::List(line_broken) => {
                    if line_broken {
                        self.indent_depth -= 1;
                        self.spacing_config.put_new_line(output);
                        self.spacing_config.put_indent(self.indent_depth, output);
                    }
                    match self.stack.pop() {
                        Some(StackItem::List(parent_line_broken)) => {
                            self.state = State::List(parent_line_broken);
                        }
                        None => {
                            self.state = State::Finishing;
                        }
                    }
                }
                State::Finishing => unreachable!(),
            }
        }

        fn atom(&mut self, atom_node: &'a sise::Node, output: &mut String) {
            match self.get_state() {
                State::Invalid => unreachable!(),
                State::Beginning => {
                    self.state = State::Finishing;
                }
                State::ListBeginning => {
                    self.state = State::List(false);
                }
                State::List(line_broken) => {
                    if self.keep_same_line(atom_node) {
                        output.push(' ');
                        self.state = State::List(line_broken);
                    } else {
                        if !line_broken {
                            self.indent_depth += 1;
                        }
                        self.spacing_config.put_new_line(output);
                        self.spacing_config.put_indent(self.indent_depth, output);
                        self.state = State::List(true);
                    }
                }
                State::Finishing => unreachable!(),
            }
        }

        fn finish(&mut self, output: &mut String) {
            match self.get_state() {
                State::Finishing => {
                    assert!(self.stack.is_empty());
                    assert_eq!(self.indent_depth, 0);
                    self.spacing_config.put_new_line(output);
                }
                _ => unreachable!(),
            }
        }
    }
}
pub use self::spaced_style::SpacedStyle;

/// Serializes `root_node`, appending the result to `output`.
///
/// # Panics
///
/// Panics if there are invalid atoms (i.e. they fail `sise::check_atom`).
pub fn serialize_into<'a>(root_node: &'a sise::Node, style: &'a mut Style<'a>, output: &mut String) {
    enum State<'b> {
        Beginning(&'b sise::Node),
        List(&'b sise::Node, std::slice::Iter<'b, Box<sise::Node>>),
        Finish,
    }

    enum StackItem<'b> {
        List(&'b sise::Node, std::slice::Iter<'b, Box<sise::Node>>),
    }

    let mut state = State::Beginning(root_node);
    let mut stack = Vec::new();

    loop {
        match state {
            State::Beginning(root_node) => {
                style.begin(output);
                match *root_node {
                    sise::Node::Atom(ref atom) => {
                        assert!(sise::check_atom(atom));
                        style.atom(root_node, output);
                        output.push_str(atom);
                        state = State::Finish;
                    }
                    sise::Node::List(ref list) => {
                        style.begin_list(root_node, output);
                        output.push('(');
                        state = State::List(root_node, list.iter());
                    }
                }
            }
            State::List(list_node, mut list_iter) => {
                if let Some(item) = list_iter.next() {
                    match **item {
                        sise::Node::Atom(ref atom) => {
                            assert!(sise::check_atom(atom));
                            style.atom(item, output);
                            output.push_str(atom);
                            state = State::List(list_node, list_iter);
                        }
                        sise::Node::List(ref list) => {
                            style.begin_list(item, output);
                            output.push('(');
                            stack.push(StackItem::List(list_node, list_iter));
                            state = State::List(item, list.iter());
                        }
                    }
                } else {
                    style.end_list(output);
                    output.push(')');
                    match stack.pop() {
                        Some(StackItem::List(parent_list_node, parent_list_iter)) => {
                            state = State::List(parent_list_node, parent_list_iter);
                        }
                        None => {
                            state = State::Finish;
                        }
                    }
                }
            }
            State::Finish => {
                assert!(stack.is_empty());
                style.finish(output);
                return;
            }
        }
    }
}

/// Serializes `root_node`, returning the result.
///
/// # Panics
///
/// Panics if there are invalid atoms (i.e. they fail `sise::check_atom`).
///
/// # Example
///
/// ```
/// extern crate sise;
/// use sise::sise_expr;
///
/// let tree = sise_expr!(["example", ["1", "2", "3"], ["a", "b", "c"]]);
///
/// // Compact
/// let compact = sise_encoder::serialize(&tree, &mut sise_encoder::CompactStyle::new());
/// assert_eq!(compact, "(example (1 2 3) (a b c))");
///
/// // Spaced
/// let spacing_config = sise_encoder::SpacingConfig {
///     line_ending: sise_encoder::LineEnding::Lf,
///     indent_len: 4,
///     indent_char: sise_encoder::IndentChar::Space,
/// };
/// let keep_same_line = std::collections::HashSet::new();
/// let spaced = sise_encoder::serialize(&tree, &mut sise_encoder::SpacedStyle::new(spacing_config, keep_same_line));
/// assert_eq!(spaced, "(example\n    (1\n        2\n        3\n    )\n    (a\n        b\n        c\n    )\n)\n");
/// ```
#[inline]
pub fn serialize<'a>(root_node: &'a sise::Node, style: &'a mut Style<'a>) -> String {
    let mut output = String::new();
    serialize_into(root_node, style, &mut output);
    output
}
