use crate::{
    expressions::parser::matrix_parser::MatrixParser, tokens::matrix_tokenizer::MatrixTokenizer,
};

pub mod errors;
pub mod expressions;
mod tokens;

fn main() {
    let str = include_str!("../main.mat");

    let tokens = MatrixTokenizer::new(str);

    let expressions = MatrixParser::new(Box::new(tokens));

    let expressions = expressions.collect::<Result<Vec<_>, _>>().unwrap();

    for expr in expressions {
        println!("{:?}", expr);
    }
}
