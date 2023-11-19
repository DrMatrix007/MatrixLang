use inkwell::{builder::Builder, context::Context, values::BasicValue};

use crate::{expressions::Expression, tokens::Operator};

pub trait ToValue {
    fn to_value<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        builder: &'a Builder,
    ) -> Option<Box<dyn BasicValue<'a> + 'b>>;
}

impl ToValue for Expression {
    fn to_value<'a: 'b, 'b>(
        &'b self,
        ctx: &'a Context,
        builder: &'a Builder,
    ) -> Option<Box<dyn BasicValue<'a> + 'b>> {
        match self {
            Expression::BinaryExpression(b) => {}
            Expression::UnaryExpression(un) => {
                let v = un.val.to_value(ctx, builder)?;
                match un.op {
                    Operator::Minus => {
                        let v = v.as_any_value_enum().into_float_value();
                        Some(Box::new(builder.build_float_neg(v, "test_temp").unwrap()))
                    }
                    Operator::Not => unimplemented!(),
                    _ => unimplemented!(),
                }
            }
            Expression::Number(num) => Some(Box::new(
                ctx.f64_type().const_float_from_string(num.num.as_str()),
            )),
            Expression::String(_) => todo!(),
            Expression::Identifier(_) => todo!(),
        }
    }
}
