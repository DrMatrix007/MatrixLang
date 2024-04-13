#![allow(dead_code)]

pub mod code_generation;
pub mod error;
pub mod expressions;
pub mod target_compiler;
pub mod tokens;
pub mod types;

use std::fs;

use inkwell::context::Context;

use crate::{
    code_generation::ToValue,
    expressions::parse::{parse_expression, parse_file},
    target_compiler::TargetCompiler,
    tokens::parse::parse_tokens,
};

fn main() {

    let mut args = std::env::args();

    let path = args.nth(1).expect("need to compile file");

    let output_path = args.next().expect("need output file");

    let compiler = TargetCompiler::current_target(inkwell::OptimizationLevel::None).unwrap();



    let lines = std::fs::read_to_string(path).unwrap();

    let tokens = parse_tokens(lines.as_str()).unwrap();

    let expressions = parse_file(tokens.iter()).unwrap();

    let ctx = Context::create();

    let builder = ctx.create_builder();
    let module = ctx.create_module("main");

    // module.set_data_layout(&compiler.machine().get_target_data().get_data_layout());
    // module.set_triple(compiler.triple());


    // let r = expression
    //     .iter()
    //     .map(|v| v.generate_code(&ctx, &module, &builder))
    //     .collect::<Vec<_>>();

    for expr in expressions {
        let a = expr.generate_code(&ctx, &module, &builder).unwrap();
        println!("{a:?}");
    }

    // println!("{:#?}", module);
    //
    // for value in r {
    //     let value = value.unwrap();
    //     // println!("{:#?}", value);
    // }

    let data = compiler.compile(&module).unwrap();
    
    fs::write(output_path, data.as_slice()).unwrap();
    

    // let mut string = String::new();
    // loop {
    //     let module = ctx.create_module("inter");
    //     let engine = module
    //         .create_jit_execution_engine(OptimizationLevel::None)
    //         .unwrap();
    //     print!("|=>");
    //     std::io::stdout().flush().unwrap();
    //     string.clear();
    //     std::io::stdin().read_line(&mut string).unwrap();
    //     let tokens = parse_tokens(string.as_str()).unwrap();
    //     let expr = parse_expression(tokens.iter()).unwrap();
    //
    //     let fn_res = ctx.f64_type().fn_type(&[], false);
    //     let func = module.add_function("main", fn_res, None);
    //     let block = ctx.append_basic_block(func, "main");
    //     builder.position_at_end(block);
    //     // builder.position_at(block, &block.get_first_instruction().unwrap());
    //     let v = expr.to_value(&ctx, &builder).ok();
    //     let v = v.as_ref();
    //     builder.build_return(v.map(|x| x.as_ref())).unwrap();
    //
    //     // let main = module.get_function("main").unwrap();
    //     // let call = builder.build_call(main, &[], "main").unwrap();
    //     let main = unsafe {
    //         engine
    //             .get_function::<unsafe extern "C" fn() -> f64>("main")
    //             .ok()
    //             .unwrap()
    //     };
    //     println!("func: {:?}", module.get_function("main"));
    //     println!("call {:?}", unsafe { main.call() });
    // }
}
