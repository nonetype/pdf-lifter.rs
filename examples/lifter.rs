use std::{fs::File, io::Read};

use pdf_parser::object::PDF;
use pdf_lifter::lifter::Liftable;


fn main() {
    // open and read as &[u8] file from argument
    let args: Vec<String> = std::env::args().collect();
    // check argument
    if args.len() != 3 {
        println!("Usage: lifter <input> <output>");
        std::process::exit(1);
    }
    let input_path = std::path::Path::new(&args[1]);
    let output_path = std::path::Path::new(&args[2]);
    let mut file = File::open(input_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let contents = buffer.as_slice();

    let pdf = PDF::parse(contents).expect("Failed to parse PDF file");
    let pdf_bytes = pdf.lift();
    // Write to output file
    std::fs::write(output_path, pdf_bytes).expect("Unable to write file");
}