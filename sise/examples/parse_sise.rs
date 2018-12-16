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
    if args.len() != 2 {
        eprintln!("Usage: {} [input-file]", args[0].to_string_lossy());
        std::process::exit(1);
    }

    let file_data = read_file(std::path::Path::new(&args[1])).unwrap();

    let parse_limits = sise::ParseLimits::unlimited();
    let (parsed, _) = sise::parse(&file_data, &parse_limits).unwrap();

    println!("{:#?}", parsed);
}