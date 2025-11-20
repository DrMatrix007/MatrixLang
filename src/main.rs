use crate::{expressions::parse_expression, tokens::parse_tokens};

pub mod errors;
pub mod expressions;
pub mod tokens;
pub mod code_generation;
pub mod layers;

fn main() {


    let code = include_str!("../code.mal");

    let tokens = parse_tokens(code.chars()).collect::<Vec<_>>();

    println!("{:?}", tokens);

    let expr = parse_expression(&mut tokens.into_iter().peekable());

    println!("{}", expr.unwrap());
}
