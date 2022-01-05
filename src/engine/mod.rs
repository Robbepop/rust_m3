mod function;
mod value_stack;
mod inst;
mod register;

pub use self::function::Function;
pub use self::value_stack::ValueStack;
pub use self::inst::Instruction;
pub use self::register::Register;
use crate::Trap;

#[derive(Debug, Default)]
pub struct Engine {
    functions: Vec<Function>,
    value_stack: ValueStack,
}

#[derive(Debug, Copy, Clone)]
pub struct Func(usize);

impl Engine {
    pub fn push_function(&mut self, func: Function) -> Func {
        let index = self.functions.len();
        self.functions.push(func);
        Func(index)
    }

    pub fn execute(
        &mut self,
        func: Func,
        params: &[Register],
        results: &mut [Register],
    ) -> Result<(), Trap> {
        let func_index = func.0;
        let func = &self.functions[func_index];
        if params.len() != func.len_params || results.len() != func.len_results {
            return Err(Trap::UnmatchedSignature);
        }
        self.value_stack.clear();
        for param in params {
            self.value_stack.push(*param);
        }
        let reg = self.value_stack.try_pop().unwrap_or(Register::default());
        let mut context = ExecutionContext {
            pc: 0,
            insts: &func.instructions,
            value_stack: &mut self.value_stack,
        };
        context.next_instruction().execute(&mut context, reg)?;
        if self.value_stack.len() != func.len_results {
            panic!(
                "expected {} values on the stack upon execution completion but found {}",
                func.len_results,
                self.value_stack.len()
            );
        }
        for (returned, result) in self.value_stack.drain().zip(results) {
            *result = returned;
        }
        Ok(())
    }
}

pub struct ExecutionContext<'engine, 'func> {
    pc: usize,
    insts: &'func [Instruction],
    pub value_stack: &'engine mut ValueStack,
}

impl<'engine, 'func> ExecutionContext<'engine, 'func> {
    pub fn next_instruction(&mut self) -> Instruction {
        let inst = self.insts[self.pc];
        self.pc += 1;
        inst
    }

    pub fn update_pc(&mut self, new_pc: usize) {
        self.pc = new_pc;
    }

    pub fn pc(&self) -> usize {
        self.pc
    }
}
