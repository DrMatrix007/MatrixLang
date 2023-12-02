use inkwell::{builder::Builder, context::Context, module::Module};

pub struct Compiler {
    ctx: Context,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ctx: Context::create(),
        }
    }

    pub fn create_module_compiler(&self, name: &str) -> ModuleCompiler<'_> {
        ModuleCompiler {
            builder: self.ctx.create_builder(),
            module: self.ctx.create_module(name),
        }
    }
}

pub struct ModuleCompiler<'a> {
    builder: Builder<'a>,
    module: Module<'a>,
}
