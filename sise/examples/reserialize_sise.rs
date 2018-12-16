// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate sise;

fn read_file(path: &std::path::Path) -> Result<Vec<u8>, std::io::Error> {
    use std::io::Read;

    let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} [compact|spaced] [input-file]", args[0].to_string_lossy());
        std::process::exit(1);
    }

    enum SerializeStyle {
        Compact,
        Spaced,
    }

    let serialize_style = match &*args[1].to_string_lossy() {
        "compact" => SerializeStyle::Compact,
        "spaced" => SerializeStyle::Spaced,
        _ => {
            eprintln!("Unknown style {:?}.", args[1]);
            std::process::exit(1);
        }
    };

    let file_data = read_file(std::path::Path::new(&args[2])).unwrap();

    let parse_limits = sise::ParseLimits::unlimited();
    let (parsed, _) = sise::parse(&file_data, &parse_limits).unwrap();

    match serialize_style {
        SerializeStyle::Compact => {
            let reserialized = sise::serialize(&parsed, &mut sise::CompactSerializeStyle::new());
            println!("{}", reserialized);
        }
        SerializeStyle::Spaced => {
            let spacing_config = sise::SerializeSpacingConfig {
                line_ending: sise::SerializeLineEnding::Lf,
                indent_len: 2,
                indent_char: sise::SerializeIndentChar::Space,
            };
            let keep_same_line = std::collections::HashSet::new();
            let reserialized = sise::serialize(&parsed, &mut sise::SpacedSerializeStyle::new(spacing_config, keep_same_line));
            print!("{}", reserialized);
        }
    }
}