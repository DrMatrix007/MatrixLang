#![allow(dead_code)]

pub mod code_generation;
pub mod error;
pub mod expressions;
pub mod tokens;

use std::io::Write;

use inkwell::{context::Context, OptimizationLevel};

use crate::{
    code_generation::ToValue, expressions::parse::parse_expression, tokens::parse::parse_tokens,
};

fn main() {
    let mut string = String::new();

    let ctx = Context::create();
    let builder = ctx.create_builder();
    loop {
        let module = ctx.create_module("inter");
        let engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        print!("|=>");
        std::io::stdout().flush().unwrap();
        string.clear();
        std::io::stdin().read_line(&mut string).unwrap();
        let tokens = parse_tokens(string.as_str()).unwrap();
        let expr = parse_expression(tokens.iter()).unwrap();

        let fn_res = ctx.f64_type().fn_type(&[], false);
        let func = module.add_function("main", fn_res, None);
        let block = ctx.append_basic_block(func, "main");
        builder.position_at_end(block);
        // builder.position_at(block, &block.get_first_instruction().unwrap());
        let v = expr.to_value(&ctx, &builder).ok();
        let v = v.as_ref();
        builder.build_return(v.map(|x| x.as_ref())).unwrap();

        // let main = module.get_function("main").unwrap();
        // let call = builder.build_call(main, &[], "main").unwrap();
        let main = unsafe {
            engine
                .get_function::<unsafe extern "C" fn() -> f64>("main")
                .ok()
                .unwrap()
        };

        println!("{:?}", unsafe { main.call() });
    }
}
