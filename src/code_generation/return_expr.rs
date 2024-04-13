use super::{values::Value, ToValue};

pub struct Return;

impl ToValue for Return {
    fn generate_code<'a: 'b, 'b>(
        &'b self,
        ctx: &'a inkwell::context::Context,
        module: &'b inkwell::module::Module<'a>,
        builder: &'b inkwell::builder::Builder<'a>,
    ) -> Result<super::values::Value<'a, 'b>, inkwell::builder::BuilderError> {
        match builder.build_return(None) {
            Ok(data) => {Ok(Value::Instruction(data))},
            Err(err) => {Err(err)},
        }
    }
}
