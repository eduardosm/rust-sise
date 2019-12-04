// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Returns whether `chr` is a valid atom character outside a
/// string (i.e. one of `:atomchar:` documented at `Node::Atom`).
pub fn is_atom_chr(chr: u8) -> bool {
    let chars = [
        b'!', b'#', b'$', b'%', b'&', b'*', b'+', b'-', b'.', b'/', b':', b'<', b'=', b'>', b'?',
        b'@', b'_', b'~',
    ];
    chr.is_ascii_alphanumeric() || chars.contains(&chr)
}

/// Returns whether `chr` is a valid atom character inside a
/// string, excluding `"` and `\` (i.e. one of `:stringchar:`
/// documented at `Node::Atom`).
pub fn is_atom_string_chr(chr: u8) -> bool {
    (chr.is_ascii_graphic() && chr != b'"' && chr != b'\\') || chr == b' '
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
    let mut iter = atom.as_bytes().iter().cloned();
    loop {
        let chr = iter.next();
        match state {
            State::Beginning => {
                match chr {
                    Some(b'"') => {
                        state = State::String;
                    }
                    Some(c) if is_atom_chr(c) => {
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
                    Some(b'"') => {
                        state = State::String;
                    }
                    Some(c) if is_atom_chr(c) => {
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
                    Some(b'"') => {
                        state = State::Normal;
                    }
                    Some(b'\\') => {
                        state = State::StringBackslash;
                    }
                    Some(c) if is_atom_string_chr(c) => {
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
                    Some(c) if is_atom_string_chr(c) || c == b'"' || c == b'\\' => {
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
