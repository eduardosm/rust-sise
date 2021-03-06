// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Returns whether `chr` is a valid atom character outside a
/// string (i.e. one of `:atomchar:` documented at `Node::Atom`).
pub fn is_atom_chr(chr: char) -> bool {
    let chars = [
        '!', '#', '$', '%', '&', '*', '+', '-', '.', '/', ':', '<', '=', '>', '?', '@', '_', '~',
    ];
    chr.is_ascii_alphanumeric() || chars.contains(&chr)
}

/// Returns whether `chr` is a valid atom character inside a
/// string, excluding `"` and `\` (i.e. one of `:stringchar:`
/// documented at `Node::Atom`).
pub fn is_atom_string_chr(chr: char) -> bool {
    (chr.is_ascii_graphic() && chr != '"' && chr != '\\') || chr == ' '
}

/// Checks whether `atom` is a valid atom (i.e. matches the regular
/// expression documented at `Node::Atom`).
pub fn check_atom(atom: &str) -> bool {
    enum State {
        Beginning,
        Normal,
        String,
        StringBackslash,
    }

    let mut state = State::Beginning;
    let mut iter = atom.chars();
    loop {
        let chr = iter.next();
        match state {
            State::Beginning => {
                match chr {
                    Some('"') => {
                        state = State::String;
                    }
                    Some(chr) if is_atom_chr(chr) => {
                        state = State::Normal;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Empty atom
                        return false;
                    }
                }
            }
            State::Normal => {
                match chr {
                    Some('"') => {
                        state = State::String;
                    }
                    Some(chr) if is_atom_chr(chr) => {
                        state = State::Normal;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Valid atom
                        return true;
                    }
                }
            }
            State::String => {
                match chr {
                    Some('"') => {
                        state = State::Normal;
                    }
                    Some('\\') => {
                        state = State::StringBackslash;
                    }
                    Some(chr) if is_atom_string_chr(chr) => {
                        state = State::String;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Incomplete string
                        return false;
                    }
                }
            }
            State::StringBackslash => {
                match chr {
                    Some(chr) if is_atom_string_chr(chr) || chr == '"' || chr == '\\' => {
                        state = State::String;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Incomplete string
                        return false;
                    }
                }
            }
        }
    }
}
