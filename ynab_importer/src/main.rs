extern crate xml;

use std::{fs::File, io::BufReader};

use xml::{reader::XmlEvent, EventReader};

fn main() {
    let file = File::open("resources/example.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            }) => {
                match name.local_name.to_lowercase().as_str() {
                    "ntry" => println!("{:?}", attributes),
                    _ => {}
                }
            }
            Ok(XmlEvent::EndElement { name }) => println!("{}", name),
            Err(_) => todo!(),
            _ => {}
        }
    }
}
