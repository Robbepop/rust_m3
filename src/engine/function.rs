use super::Instruction;

#[derive(Debug)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub len_params: usize,
    pub len_results: usize,
    pub len_locals: usize,
}

impl Function {
    pub fn build(len_params: usize, len_results: usize, len_locals: usize) -> FunctionBuilder {
        FunctionBuilder {
            function: Self {
                instructions: Vec::new(),
                len_params,
                len_results,
                len_locals,
            },
        }
    }
}

pub struct FunctionBuilder {
    function: Function,
}

impl FunctionBuilder {
    pub fn push_inst(mut self, inst: Instruction) -> Self {
        self.function.instructions.push(inst);
        self
    }

    pub fn finish(self) -> Function {
        self.function
    }
}
