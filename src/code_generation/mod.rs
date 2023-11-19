use inkwell::{
    builder::{Builder, BuilderError},
    context::Context,
    values::BasicValue,
};

use crate::{
    expressions::{BinaryExpression, Expression, UnaryExpression},
    tokens::{literals::NumberLiteral, Operator},
};

pub trait ToValue {
    fn to_value<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        builder: &'a Builder,
    ) -> Result<Box<dyn BasicValue<'a> + 'b>, BuilderError>;
}

impl ToValue for Expression {
    fn to_value<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        builder: &'a Builder,
    ) -> Result<Box<dyn BasicValue<'a> + 'b>, BuilderError> {
        match self {
            Expression::BinaryExpression(bn) => bn.to_value(ctx, builder),
            Expression::UnaryExpression(un) => un.to_value(ctx, builder),
            Expression::Number(num) => num.to_value(ctx, builder),
            Expression::String(_) => todo!(),
            Expression::Identifier(_) => todo!(),
        }
    }
}

impl ToValue for NumberLiteral {
    fn to_value<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        _builder: &'a Builder,
    ) -> Result<Box<dyn BasicValue<'a> + 'b>, BuilderError> {
        Ok(Box::new(
            ctx.f64_type().const_float_from_string(self.num.as_str()),
        ))
    }
}

impl ToValue for UnaryExpression {
    fn to_value<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        builder: &'a Builder,
    ) -> Result<Box<dyn BasicValue<'a> + 'b>, BuilderError> {
        let v = self.val.to_value(ctx, builder)?;
        match self.op {
            Operator::Minus => {
                let v = v.as_any_value_enum().into_float_value();
                Ok(Box::new(builder.build_float_neg(v, "test_temp").unwrap()))
            }
            Operator::Not => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

impl ToValue for BinaryExpression {
    fn to_value<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        builder: &'a Builder,
    ) -> Result<Box<dyn BasicValue<'a> + 'b>, BuilderError> {
        let r = self.right.to_value(ctx, builder)?.as_basic_value_enum();
        let l = self.left.to_value(ctx, builder)?.as_basic_value_enum();
        match self.op {
            Operator::Plus => Ok(Box::new(builder.build_float_add(
                l.into_float_value(),
                r.into_float_value(),
                "addtmp",
            )?)),
            Operator::Minus => Ok(Box::new(builder.build_float_sub(
                l.into_float_value(),
                r.into_float_value(),
                "subtmp",
            )?)),
            Operator::Mul => Ok(Box::new(builder.build_float_mul(
                l.into_float_value(),
                r.into_float_value(),
                "multmp",
            )?)),
            Operator::Div => Ok(Box::new(builder.build_float_div(
                l.into_float_value(),
                r.into_float_value(),
                "divtmp",
            )?)),
            _ => unimplemented!(),
        }
    }
}
