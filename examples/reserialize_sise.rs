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

    let file_data = std::fs::read(&args[2]).unwrap();
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
