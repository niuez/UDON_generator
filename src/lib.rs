extern crate nom;
extern crate log;
use wasm_bindgen::prelude::*;

pub mod parser;

use parser::file_input::*;

#[wasm_bindgen]
pub fn greet(prog: &str) -> String {
    match FileInput::parse(prog) {
        Ok((s, tree)) => {
            tree.transpile()
        }
        Err(e) => {
            format!("Error: {:?}", e)
        }
    }
}
