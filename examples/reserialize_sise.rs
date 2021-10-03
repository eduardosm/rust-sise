// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![deny(
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_must_use,
    unused_qualifications
)]
#![forbid(unsafe_code)]

use std::io::Read as _;

fn read_file(path: &std::path::Path) -> Result<Vec<u8>, std::io::Error> {
    let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} [compact|spaced] [input-file]",
            args[0].to_string_lossy()
        );
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
    let file_data = String::from_utf8(file_data).unwrap();
    let mut parser = sise::Parser::new(&file_data);
    let parsed = sise::parse_tree(&mut parser).unwrap();
    parser.finish().unwrap();

    let break_line_at = match serialize_style {
        SerializeStyle::Compact => usize::MAX,
        SerializeStyle::Spaced => 0,
    };
    let mut reserialized = String::new();
    let mut serializer = sise::Serializer::new(
        sise::SerializerStyle {
            line_break: "\n",
            indentation: "  ",
        },
        &mut reserialized,
    );
    sise::serialize_tree(&mut serializer, &parsed, break_line_at);
    serializer.finish(false);
    println!("{}", reserialized);
}
