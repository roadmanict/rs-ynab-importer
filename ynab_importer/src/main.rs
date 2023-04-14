use std::fs;

fn main() {
    let paths = fs::read_dir("/tmp/asn-bank").unwrap();

    for path in paths {
        println!("path {}", path.unwrap().path().display());
    }
}
