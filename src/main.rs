use crate::tokens::Tokenizer;

mod tokens;


fn main() {
    
    let str = include_str!("../main.mat");

    let tokens = Tokenizer::new(str).collect::<Vec<_>>();

    println!("{:?}", tokens);
}
