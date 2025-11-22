// use crate::{expressions::parse_expression};

use crate::tokens::lexer::Lexer;

pub mod errors;
// pub mod expressions;
pub mod tokens;
// pub mod code_generation;

fn main() {
    let code = include_str!("../code.mal");

    let lexer = Lexer::new(code);

    for i in lexer {
        match i {
            Ok(tok) => {
                print!("({}, {}),", tok.value, tok.len);
            }
            Err(err) => {
                println!("\n\n error! {:?}", err);
            }
        }
    }
    println!();

    // let expr = parse_expression(&mut tokens.into_iter().peekable());

    // println!("{}", expr.unwrap());
}
