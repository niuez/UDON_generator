extern crate nom;
extern crate log;

pub mod parser;

use parser::file_input::*;

fn main() {
    env_logger::init();
    let args = std::env::args().collect::<Vec<String>>();
    let file_name = &args[1];
    let program = std::fs::read_to_string(file_name).expect("cant read");

    let tree = FileInput::parse(&program).expect("parse error");
    log::debug!("{:?}", tree);
    println!("{}", tree.1.transpile())
}
