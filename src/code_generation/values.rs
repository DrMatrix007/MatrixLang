use inkwell::values::{
    AnyValue, AnyValueEnum, BasicValue, BasicValueEnum, FloatValue, FunctionValue, InstructionValue,
};

#[derive(Debug)]
pub enum Value<'a, 'b> {
    Any(Box<dyn AnyValue<'a> + 'b>),
    Basic(Box<dyn BasicValue<'a> + 'b>),
    Function(FunctionValue<'a>),
    Instruction(InstructionValue<'a>),
}

impl<'a, 'b> Value<'a, 'b> {
    pub fn get_any(&self) -> Option<AnyValueEnum<'a>> {
        match self {
            Self::Any(b) => Some(b.as_any_value_enum()),
            Self::Basic(b) => Some(b.as_ref().as_any_value_enum()),
            Self::Function(f) => Some(f.as_any_value_enum()),
            Self::Instruction(i) => Some(i.as_any_value_enum()),
        }
    }
    pub fn get_basic(&self) -> Option<BasicValueEnum<'a>> {
        match self {
            Self::Basic(b) => Some(b.as_basic_value_enum()),
            _ => None,
        }
    }
    pub fn as_any_value_enum(&self) -> Option<AnyValueEnum<'_>> {
        self.get_any()
    }
    pub fn into_float_value(&self) -> FloatValue {
        self.as_any_value_enum().unwrap().into_float_value()
    }
}

pub trait IntoAnyValue<'a, 'b> {
    fn into_any_value(self) -> Value<'a, 'b>;
}

pub trait IntoBasicValue<'a, 'b> {
    fn into_basic_value(self) -> Value<'a, 'b>;
}

impl<'a, 'b, T: BasicValue<'a> + 'b> IntoBasicValue<'a, 'b> for Box<T> {
    fn into_basic_value(self) -> Value<'a, 'b> {
        Value::Basic(self)
    }
}

impl<'a, 'b, T: AnyValue<'a> + 'b> IntoAnyValue<'a, 'b> for Box<T> {
    fn into_any_value(self) -> Value<'a, 'b> {
        Value::Any(self)
    }
}
