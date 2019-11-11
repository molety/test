extern crate glob;
use glob::glob;

fn main() {
    for entry in glob("**/*.*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}
