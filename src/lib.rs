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

/// Creates a function that takes a parameter `n` and
/// counts from `n` down to 0 before it returns 0.
///
/// # Note
///
/// This is a function intended for benchmarking purposes.
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

/// Creates a function that takes 2 parameters `a` and `b`.
///
/// # Note
///
/// - Performs the following calculation: (a+b)*(a+b)
/// - Returns the result of the calculation.
fn add_mul() -> Function {
    Function::build(2, 1, 0)
        .push_inst(Instruction::i32_add())
        .push_inst(Instruction::dup())
        .push_inst(Instruction::i32_mul())
        .push_inst(Instruction::ret())
        .finish()
}

#[test]
fn add_mul_works() {
    let mut engine = Engine::default();
    let add_mul = engine.push_function(add_mul());
    let mut results = [Register::default(); 1];
    engine
        .execute(
            add_mul,
            &[Register::from(2), Register::from(3)],
            &mut results,
        )
        .unwrap();
    assert_eq!(results[0], Register::from((2 + 3) * (2 + 3)));
}

#[test]
fn counter_loop_works() {
    let mut engine = Engine::default();
    let cl = engine.push_function(counter_loop());
    engine.execute(cl, &[Register::from(10_000)], &mut []).unwrap();
}

#[bench]
fn bench_counter_loop(b: &mut Bencher) {
    let mut engine = Engine::default();
    let cl = engine.push_function(counter_loop());
    b.iter(|| {
        engine
            .execute(cl, &[Register::from(100_000)], &mut [])
            .unwrap()
    })
}
