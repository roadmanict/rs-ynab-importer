extern crate xml;

use std::{fs::File, io::BufReader};

use xml::{attribute::OwnedAttribute, reader::XmlEvent, EventReader};

fn main() {
    let file = File::open("resources/example.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut in_entry = false;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => match name.local_name.to_lowercase().as_str() {
                "ntry" => in_entry = true,
                st => {
                    println!("{}, {}, {:?}", st, in_entry, attributes);
                }
            },
            Ok(XmlEvent::EndElement { name }) => match name.local_name.to_lowercase().as_str() {
                "ntry" => in_entry = false,
                _ => {}
            },
            Err(_) => todo!(),
            _ => {}
        }
    }
}

fn parse_entry(attributes: Vec<OwnedAttribute>) {
    println!("entry {:?}", attributes);
    for attribute in attributes.iter() {
        println!("{}", attribute);
        match attribute.name.local_name.to_lowercase().as_str() {
            "amt" => println!("{}", attribute.value),
            _ => {}
        }
    }
}
