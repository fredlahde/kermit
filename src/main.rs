extern crate walkdir;
mod compose;

use compose::Scanner;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let s = Scanner::default();
    let results = s.scan(args[1].as_str());

    results.into_iter().for_each(|res| println!("{}", res.path));
}
