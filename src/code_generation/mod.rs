pub mod compiler;
pub mod values;

use inkwell::{
    basic_block::BasicBlock,
    builder::{Builder, BuilderError},
    context::Context,
    module::Module,
};

use crate::{
    expressions::{
        function::FunctionDecleration, BinaryExpression, Expression, UnaryExpression,
        VariableDecleration,
    },
    tokens::{literals::NumberLiteral, Operator},
};

use self::values::{IntoBasicValue, Value};

pub trait ToValue {
    fn generate_code<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        module: &'b Module<'a>,
        builder: &'b Builder<'a>,
    ) -> Result<Value<'a, 'b>, BuilderError>;
}
impl ToValue for Expression {
    fn generate_code<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        module: &'b Module<'a>,
        builder: &'b Builder<'a>,
    ) -> Result<Value<'a, 'b>, BuilderError> {
        match self {
            Expression::BinaryExpression(bn) => bn.generate_code(ctx, module, builder),
            Expression::UnaryExpression(un) => un.generate_code(ctx, module, builder),
            Expression::Number(num) => num.generate_code(ctx, module, builder),
            Expression::String(_) => todo!(),
            Expression::Identifier(_) => todo!(),
            Expression::VariableDecleration(var) => var.generate_code(ctx, module, builder),
            Expression::FunctionDecleration(func) => func.generate_code(ctx, module, builder),
        }
    }
}

impl ToValue for NumberLiteral {
    fn generate_code<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        _moudle: &'b Module,
        _builder: &'b Builder,
    ) -> Result<Value<'a, 'b>, BuilderError> {
        Ok(Box::new(ctx.f64_type().const_float_from_string(self.num.as_str())).into_basic_value())
    }
}

impl ToValue for UnaryExpression {
    fn generate_code<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        module: &'b Module<'a>,
        builder: &'b Builder<'a>,
    ) -> Result<Value<'a, 'b>, BuilderError> {
        let v = self.val.generate_code(ctx, module, builder)?;
        match self.op {
            Operator::Minus => {
                let v = v.get_basic().ok_or(BuilderError::ValueTypeMismatch(""))?;
                let v = if v.is_float_value() {
                    v.into_float_value()
                } else {
                    return Err(BuilderError::ValueTypeMismatch("values mismatch!"));
                };
                Ok(Value::Basic(Box::new(
                    builder.build_float_neg(v, "test_temp").unwrap(),
                )))
            }
            Operator::Not => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

impl ToValue for BinaryExpression {
    fn generate_code<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        module: &'b Module<'a>,
        builder: &'b Builder<'a>,
    ) -> Result<Value<'a, 'b>, BuilderError> {
        let _r = self
            .right
            .generate_code(ctx, module, builder)?
            .as_any_value_enum()
            .unwrap();
        let _l = self
            .left
            .generate_code(ctx, module, builder)?
            .as_any_value_enum()
            .unwrap();
        unimplemented!();
        // match self.op {
        //     Operator::Plus => Ok(Box::new(builder.build_float_add(
        //         l.into_float_value(),
        //         r.into_float_value(),
        //         "addtmp",
        //     )?)
        //     .into_basic_value()),
        //     Operator::Minus => Ok(Box::new(builder.build_float_sub(
        //         l.into_float_value(),
        //         r.into_float_value(),
        //         "subtmp",
        //     )?)
        //     .into_basic_value()),
        //     Operator::Mul => Ok(Box::new(builder.build_float_mul(
        //         l.into_float_value(),
        //         r.into_float_value(),
        //         "multmp",
        //     )?)
        //     .into_basic_value()),
        //     Operator::Div => Ok(Box::new(builder.build_float_div(
        //         l.into_float_value(),
        //         r.into_float_value(),
        //         "divtmp",
        //     )?)
        //     .into_basic_value()),
        //     _ => unimplemented!(),
        // }
    }
}

impl ToValue for VariableDecleration {
    fn generate_code<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        module: &'b Module<'a>,
        builder: &'b Builder<'a>,
    ) -> Result<Value<'a, 'b>, BuilderError> {
        let v = self.value.generate_code(ctx, module, builder)?;
        let v_type = v.get_basic().unwrap().get_type();

        let var = builder.build_alloca(v_type, self.var_name.name.as_str())?;
        let val = builder.build_store(var, v.get_basic().unwrap())?;
        Ok(Value::Instruction(val))
    }
}

impl ToValue for FunctionDecleration {
    fn generate_code<'a: 'b, 'b>(
        &'b self,
        ctx: &'a inkwell::context::Context,
        module: &'b Module<'a>,
        builder: &'b inkwell::builder::Builder<'a>,
    ) -> Result<crate::code_generation::values::Value<'a, 'b>, inkwell::builder::BuilderError> {
        let fn_type = ctx.void_type().fn_type(&[], false);
        let f = module.add_function(&self.name.name, fn_type, None);
        let block = ctx.append_basic_block(f, &self.name.name);
        builder.position_at_end(block);
        for expr in &self.vals {
            let _ = expr.generate_code(ctx, module, builder);
        }

        Ok(Value::Function(f))
    }
}
