use inkwell::{
    memory_buffer::MemoryBuffer, module::Module, passes::{PassBuilderOptions, PassManager}, support::LLVMString, targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple}, values::FunctionValue, OptimizationLevel
};

pub struct TargetCompiler {
    target: Target,
    triple: TargetTriple,
    machine: TargetMachine,
}

impl TargetCompiler {
    pub fn current_target(opt: OptimizationLevel) -> Result<Self, String> {
        Target::initialize_all(&InitializationConfig::default());
        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple).map_err(|e| e.to_string())?;

        let machine = target
            .create_target_machine(
                &triple,
                "generic",
                "",
                opt,
                RelocMode::PIC,
                CodeModel::Default,
            )
            .ok_or(("Cant create machine!").to_owned())?;
        Ok(Self {
            target,
            triple,
            machine,
        })
    }
    pub fn compile(&self, module: &Module) -> Result<MemoryBuffer, String> {
        let passes: &[&str] = &[
            "instcombine",
            "reassociate",
            "gvn",
            "simplifycfg",
            // "basic-aa",
            "mem2reg",
        ];

        let options = PassBuilderOptions::create();
        // options.set_debug_logging(true);
        // options.set_call_graph_profile(true);
        options.set_verify_each(true);

        // module
        //     .run_passes(&passes.join(","), self.machine(), options)
        //     .map_err(|x| x.to_string())?;

        self.machine.write_to_memory_buffer(module, FileType::Object).map_err(|x|x.to_string())
        // Ok(())
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn triple(&self) -> &TargetTriple {
        &self.triple
    }

    pub fn machine(&self) -> &TargetMachine {
        &self.machine
    }
}
