use pdf_parser::object::CrossReferenceEntry;
use pdf_parser::object::CrossReferenceTable;
use pdf_parser::object::PDF;
use pdf_parser::object::Object;
use pdf_parser::object::NameObject;
use pdf_parser::object::Trailer;


// We use this to lift the internal PDF structure of pdf-parser into PDF bytes.
// Define Liftable trait, implement it for PDF<'a>, and implement it for Object<'a>.
// This is a trait that can be lifted to PDF bytes.
pub trait Liftable {
    fn lift(&self) -> Vec<u8>;
}

// Implement Liftable for PDF<'a>.
impl<'a> Liftable for PDF<'a> {
    fn lift(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"%PDF-");
        bytes.extend_from_slice(&self.header.major.to_string().as_bytes());
        bytes.extend_from_slice(b".");
        bytes.extend_from_slice(&self.header.minor.to_string().as_bytes());
        bytes.extend_from_slice(b"\n");
        for object in self.body.iter() {
            bytes.extend_from_slice(&object.lift());
        }
        for table in self.cross_reference_tables.iter() {
            bytes.extend_from_slice(&table.lift());
        }
        bytes.extend_from_slice(&self.trailer.lift());
        bytes.extend_from_slice(b"%%EOF");
        bytes.extend_from_slice(b"\n");
        bytes
    }
}

impl<'a> Liftable for NameObject<'a> {
    fn lift(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"/");
        bytes.extend_from_slice(self.0.as_bytes());
        bytes
    }
}

// Implement Liftable for Object<'a>.
impl<'a> Liftable for Object<'a> {
    fn lift(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Object::Boolean(b) => {
                bytes.extend_from_slice(if *b { b"true" } else { b"false" });
            },
            Object::Integer(i) => {
                bytes.extend_from_slice(&i.to_string().as_bytes());
            },
            Object::Real(f) => {
                bytes.extend_from_slice(&f.to_string().as_bytes());
            },
            Object::LiteralString(s) => {
                bytes.extend_from_slice(b"(");
                bytes.extend_from_slice(s.as_bytes());
                bytes.extend_from_slice(b")");
            },
            Object::HexadecimalString(s) => {
                bytes.extend_from_slice(b"<");
                bytes.extend_from_slice(s.as_bytes());
                bytes.extend_from_slice(b">");
            },
            Object::Name(n) => {
                bytes.extend_from_slice(&n.lift());
            },
            Object::Array(a) => {
                bytes.extend_from_slice(b"[");
                for o in a {
                    bytes.extend_from_slice(&o.lift());
                    bytes.extend_from_slice(b" ");
                }
                bytes.extend_from_slice(b"]");
            },
            Object::Dictionary(d,) => {
                bytes.extend_from_slice(b"<<\n");
                for (k, v) in d.iter() {
                    bytes.extend_from_slice(&k.lift());
                    bytes.extend_from_slice(b" ");
                    bytes.extend_from_slice(&v.lift());
                    bytes.extend_from_slice(b"\n");
                }
                bytes.extend_from_slice(b">>");
            },
            Object::Stream(s) => {
                bytes.extend_from_slice(b"stream");
                bytes.extend_from_slice(b"\n");
                bytes.extend_from_slice(&s);
                bytes.extend_from_slice(b"\n");
                bytes.extend_from_slice(b"endstream");
                bytes.extend_from_slice(b"\n");
            },
            Object::Null => {
                bytes.extend_from_slice(b"null");
            },
            Object::Comment(s) => {
                bytes.extend_from_slice(b"%");
                bytes.extend_from_slice(s.as_bytes());
            },
            Object::IndirectReference { id, generation } => {
                bytes.extend_from_slice(&id.to_string().as_bytes());
                bytes.extend_from_slice(b" ");
                bytes.extend_from_slice(&generation.to_string().as_bytes());
                bytes.extend_from_slice(b" R");
            },
            Object::IndirectObject { id, generation, dictionary, stream} => {
                bytes.extend_from_slice(&id.to_string().as_bytes());
                bytes.extend_from_slice(b" ");
                bytes.extend_from_slice(&generation.to_string().as_bytes());
                bytes.extend_from_slice(b" obj ");
                bytes.extend_from_slice(&dictionary.lift());
                bytes.extend_from_slice(b"\n");
                if let Some(s) = stream {
                    bytes.extend_from_slice(&s.lift());
                }
                bytes.extend_from_slice(b"endobj");
                bytes.extend_from_slice(b"\n");
            },
        }
        bytes
    }
}

impl Liftable for CrossReferenceTable {
    fn lift(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"xref");
        bytes.extend_from_slice(b"\n");
        bytes.extend_from_slice(&self.id.to_string().as_bytes());
        bytes.extend_from_slice(b" ");
        bytes.extend_from_slice(&self.entries.len().to_string().as_bytes());
        bytes.extend_from_slice(b"\n");
        for entry in self.entries.iter() {
            bytes.extend_from_slice(&entry.lift());
        }
        bytes
    }
}

impl Liftable for CrossReferenceEntry {
    fn lift(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(format!("{:0>10}", &self.offset).as_bytes());
        bytes.extend_from_slice(b" ");
        bytes.extend_from_slice(format!("{:0>5}", &self.generation).as_bytes());
        bytes.extend_from_slice(b" ");
        bytes.extend_from_slice(match self.free {
            true => b"f",
            false => b"n",
        });
        bytes.extend_from_slice(b"\n");
        bytes
    }
}

impl<'a> Liftable for Trailer<'a> {
    fn lift(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"trailer");
        bytes.extend_from_slice(b"\n");
        bytes.extend_from_slice(&self.dictionary.lift());
        bytes.extend_from_slice(b"\n");
        bytes.extend_from_slice(b"startxref");
        bytes.extend_from_slice(b"\n");
        bytes.extend_from_slice(&self.startxref.to_string().as_bytes());
        bytes.extend_from_slice(b"\n");
        bytes
    }
}
