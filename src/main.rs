extern crate glob;
use glob::glob;

fn main() {
    for path in glob("**/*.*").unwrap().filter_map(Result::ok) {
        println!("{}", path.display());
    }
}
