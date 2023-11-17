#![allow(dead_code)]

use std::io::Read;

pub mod error;
pub mod literals;
pub mod runtime;
pub mod token;

fn main() {
    let mut string = String::new();
    loop {
        string.clear();
        std::io::stdin().read_line(&mut string).unwrap();
        println!("?");
        println!("{:?}", token::tokenize(string.as_str()));
    }
}
