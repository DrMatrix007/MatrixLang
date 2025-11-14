use crate::tokens::parse_tokens;


pub mod tokens;
pub mod errors;
pub mod expressions;
fn main() {
    let data = "💀 += +💀";

    println!("{:?}", parse_tokens(data.chars()).collect::<Vec<_>>());
    
    
}
