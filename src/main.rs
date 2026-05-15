use crate::tokens::tokenizer::Tokenizer;

mod tokens;

fn main() {
    let str = include_str!("../main.mat");

    let tokens = Tokenizer::new(str).collect::<Result<Vec<_>, _>>().unwrap();

    for token in &tokens {
        print!("{}, ", token);
    }
}
