extern crate serde;
extern crate serde_json;
extern crate walkdir;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate serde_derive;
mod compose_runner;
mod compose_scanner;

use compose_scanner::Scanner;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = Scanner::default();
    let results = s.to_file(args[1].as_str(), "");

}
