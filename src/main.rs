use crate::{expressions::parser::Parser, tokens::tokenizer::Tokenizer};

pub mod errors;
pub mod expressions;
mod tokens;

fn main() {
    let str = include_str!("../main.mat");

    let tokens = Tokenizer::new(str);

    let expressions = Parser::new(tokens);

    let expressions = expressions.collect::<Result<Vec<_>, _>>().unwrap();

    for expr in expressions {
        println!("{:?}", expr);
    }
}
