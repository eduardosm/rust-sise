#![deny(
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_must_use,
    unused_qualifications
)]
#![forbid(unsafe_code)]

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [input-file]", args[0].to_string_lossy());
        std::process::exit(1);
    }

    let file_data = std::fs::read(&args[1]).unwrap();
    let file_data = String::from_utf8(file_data).unwrap();
    let mut parser = sise::Parser::new(&file_data);
    let parsed = sise::parse_tree(&mut parser).unwrap();
    parser.finish().unwrap();

    println!("{:#?}", parsed);
}
