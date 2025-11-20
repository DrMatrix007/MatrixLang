use crate::{code_generation::CodeGenerator, expressions::{Expression, expression_types::UnaryExpression}, tokens::op::Op};

impl CodeGenerator for Expression {
    fn generate<'a>(&'a self) -> super::CodeGeneratorResult<'a> {
        todo!();
    }
}

impl CodeGenerator for UnaryExpression {
    fn generate<'a>(&'a self) -> super::CodeGeneratorResult<'a> {
        match self.op {
            Op::Add => {

            },
            _=> { Err(crate::errors::LangError::CantCompile(format!("{}", self))) }
        }
    }
}