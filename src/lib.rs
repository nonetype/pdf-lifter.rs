mod lifter;
use pdf_parser::object::PDF;

// This package is called `pdf-lifter`.
// This library lifts the internal PDF structure of pdf-parser into PDF.

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::lifter::Liftable;

    use super::*;
    use std::{fs::File, io::Read};

    #[test]
    fn basic_parser() {
        let path = std::path::Path::new("assets/test.pdf");
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let contents = buffer.as_slice();

        let pdf = PDF::parse(contents).expect("Failed to parse PDF file");
        let pdf_bytes = pdf.lift();
        // pdf_bytes to String
        let pdf_string = String::from_utf8(pdf_bytes).unwrap();
        println!("pdf_string: \n{}", pdf_string);
        
    }
}
