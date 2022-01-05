#![feature(test)]
#![allow(unused_imports, dead_code)]

#[cfg(test)]
extern crate test;

mod context;
mod error;
mod register;

#[cfg(test)]
use test::Bencher;

use self::context::{Engine, ExecutionContext, Func, Function, Instruction, ValueStack};
use self::error::Trap;
use self::register::Register;

fn counter_loop() -> Function {
    Function::build(1, 0, 0)
        .push_inst(Instruction::dup()) // 2
        .push_inst(Instruction::i32_const(1)) // 3
        .push_inst(Instruction::i32_sub()) // 2
        .push_inst(Instruction::dup()) // 3
        .push_inst(Instruction::br_eqz(1)) // 2
        .push_inst(Instruction::drop()) // 1
        .push_inst(Instruction::drop()) // 0
        .push_inst(Instruction::ret_drop())
        .finish()
}

#[test]
fn it_works() {
    let mut engine = Engine::default();
    // let function = Function::build(3, 1, 0)
    //     .push_inst(Instruction::i32_add())
    //     .push_inst(Instruction::i32_mul())
    //     .push_inst(Instruction::i32_const(42))
    //     .push_inst(Instruction::drop())
    //     .push_inst(Instruction::ret())
    //     .finish();
    // let func = engine.push_function(function);
    // let mut results = [Register::default(); 1];
    // let result = engine.execute(
    //     func,
    //     &[Register::from(1), Register::from(2), Register::from(3)],
    //     &mut results,
    // );
    // println!("result = {:?}", result);
    let cl = engine.push_function(counter_loop());
    let result = engine.execute(cl, &[Register::from(10_000)], &mut []);
    // println!("result = {:?}", result);
}

#[bench]
fn bench_counter_loop(b: &mut Bencher) {
    let mut engine = Engine::default();
    let cl = engine.push_function(counter_loop());
    b.iter(|| {
        engine.execute(cl, &[Register::from(100_000)], &mut []).unwrap()
    })
}
