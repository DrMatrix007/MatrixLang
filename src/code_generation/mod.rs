pub mod generators_expressions_impl;

use inkwell::values::AnyValueEnum;

use crate::{errors::LangError, expressions::Expression};

pub type CodeGeneratorResult<'a> = Result<AnyValueEnum<'a>, LangError>; 

pub trait CodeGenerator {
    fn generate<'a>(&'a self) -> CodeGeneratorResult<'a>;
}

