use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
extern crate glob;
use glob::glob;

fn main() -> std::io::Result<()> {
    for path in glob("**/*.*").unwrap().filter_map(Result::ok) {
        println!("{}", path.display());
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        println!("{}", buf);
    }
    Ok(())
}
