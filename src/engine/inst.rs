use super::{ExecutionContext, Register, Trap};
use core::fmt;
use core::fmt::Debug;

#[derive(Copy, Clone)]
pub struct Instruction {
    op: Op,
    aux: Register,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_name = match self.op as usize {
            addr if addr == op::ret as usize => "ret",
            addr if addr == op::unreachable as usize => "unreachable",
            addr if addr == op::drop as usize => "drop",
            addr if addr == op::i32_add as usize => "i32_add",
            addr if addr == op::i32_mul as usize => "i32_mul",
            addr if addr == op::i32_const as usize => "i32_const",
            unknown => panic!(
                "encountered unknown instruction operator at address: {}",
                unknown
            ),
        };
        f.debug_struct("Instruction")
            .field("op", &op_name)
            .field("aux", &self.aux)
            .finish()
    }
}

impl Instruction {
    fn from_op(op: Op) -> Self {
        Self {
            op,
            aux: Register::default(),
        }
    }

    fn new(op: Op, aux: impl Into<Register>) -> Self {
        Self {
            op,
            aux: aux.into(),
        }
    }

    #[inline(always)]
    pub fn execute(self, ctx: &mut ExecutionContext, reg: Register) -> Result<(), Trap> {
        (self.op)(ctx, reg, self.aux)
    }

    pub fn ret() -> Self {
        Self::from_op(op::ret)
    }

    pub fn ret_drop() -> Self {
        Self::from_op(op::ret_drop)
    }

    pub fn unreachable() -> Self {
        Self::from_op(op::unreachable)
    }

    pub fn drop() -> Self {
        Self::from_op(op::drop)
    }

    pub fn dup() -> Self {
        Self::from_op(op::dup)
    }

    pub fn i32_add() -> Self {
        Self::from_op(op::i32_add)
    }

    pub fn i32_sub() -> Self {
        Self::from_op(op::i32_sub)
    }

    pub fn i32_mul() -> Self {
        Self::from_op(op::i32_mul)
    }

    pub fn i32_const(value: i32) -> Self {
        Self::new(op::i32_const, value)
    }

    pub fn br_eqz(target: u32) -> Self {
        Self::new(op::br_eqz, target as i32)
    }
}

#[inline(always)]
pub fn execute_next_instruction(ctx: &mut ExecutionContext, reg: Register) -> Result<(), Trap> {
    let instr = ctx.next_instruction();
    (instr.op)(ctx, reg, instr.aux)
}

#[inline(always)]
pub fn execute_next_instruction_at(
    ctx: &mut ExecutionContext,
    reg: Register,
    new_pc: usize,
) -> Result<(), Trap> {
    ctx.update_pc(new_pc);
    let instr = ctx.next_instruction();
    (instr.op)(ctx, reg, instr.aux)
}

type Op = fn(ctx: &mut ExecutionContext, reg: Register, aux: Register) -> Result<(), Trap>;

mod op {
    use super::{
        execute_next_instruction, execute_next_instruction_at, ExecutionContext, Register, Trap,
    };
    use core::mem::replace;

    pub fn ret(ctx: &mut ExecutionContext, reg: Register, _aux: Register) -> Result<(), Trap> {
        // println!("ret, pc: {}", ctx.pc());
        ctx.value_stack.push(reg);
        Ok(())
    }

    pub fn ret_drop(_ctx: &mut ExecutionContext, _reg: Register, _aux: Register) -> Result<(), Trap> {
        // println!("ret_drop, pc: {}", ctx.pc());
        Ok(())
    }

    pub fn unreachable(
        _ctx: &mut ExecutionContext,
        _reg: Register,
        _aux: Register,
    ) -> Result<(), Trap> {
        // println!("unreachable, pc: {}", ctx.pc());
        Err(Trap::Unreachable)
    }

    pub fn i32_add(
        ctx: &mut ExecutionContext,
        reg: Register,
        _aux: Register,
    ) -> Result<(), Trap> {
        let lhs: i32 = ctx.value_stack.pop_as();
        let rhs: i32 = reg.into();
        let result = lhs + rhs;
        // println!("i32_add, pc: {}, {} + {} = {}", ctx.pc(), lhs, rhs, result);
        execute_next_instruction(ctx, result.into())
    }

    pub fn i32_sub(
        ctx: &mut ExecutionContext,
        reg: Register,
        _aux: Register,
    ) -> Result<(), Trap> {
        let lhs: i32 = ctx.value_stack.pop_as();
        let rhs: i32 = reg.into();
        let result = lhs - rhs;
        // println!("i32_add, pc: {}, {} - {} = {}", ctx.pc(), lhs, rhs, result);
        execute_next_instruction(ctx, result.into())
    }

    pub fn i32_mul(
        ctx: &mut ExecutionContext,
        reg: Register,
        _aux: Register,
    ) -> Result<(), Trap> {
        let lhs: i32 = ctx.value_stack.pop_as();
        let rhs: i32 = reg.into();
        let result = lhs * rhs;
        // println!("i32_mul, pc: {}, {} * {} = {}", ctx.pc(), lhs, rhs, result);
        execute_next_instruction(ctx, result.into())
    }

    pub fn i32_const(
        ctx: &mut ExecutionContext,
        mut reg: Register,
        aux: Register,
    ) -> Result<(), Trap> {
        let reg = replace(&mut reg, aux);
        ctx.value_stack.push(reg);
        // println!("i32_const, pc: {}, const: {}", ctx.pc(), i32::from(aux));
        execute_next_instruction(ctx, aux)
    }

    pub fn drop(ctx: &mut ExecutionContext, mut reg: Register, _aux: Register) -> Result<(), Trap> {
        let popped = ctx.value_stack.pop();
        let _dropped = replace(&mut reg, popped);
        // println!("drop, pc: {}, dropped: {:?}", ctx.pc(), _dropped);
        execute_next_instruction(ctx, reg)
    }

    pub fn dup(ctx: &mut ExecutionContext, reg: Register, _aux: Register) -> Result<(), Trap> {
        ctx.value_stack.push(reg);
        // println!("dup, pc: {}, duplicated: {:?}", ctx.pc(), reg);
        execute_next_instruction(ctx, reg)
    }

    pub fn br_eqz(
        ctx: &mut ExecutionContext,
        mut reg: Register,
        aux: Register,
    ) -> Result<(), Trap> {
        let targets = [i32::from(aux) as usize, ctx.pc() + 1];
        let target = targets[(reg == Register::default()) as usize];
        reg = ctx.value_stack.pop();
        // println!("br_eqz, pc: {}, reg: {:?}, aux = {:?}", ctx.pc(), reg, aux);
        execute_next_instruction_at(ctx, reg, target)
    }
}
