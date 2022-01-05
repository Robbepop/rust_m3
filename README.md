# Rust Threaded Code Architecture

An attempt at recreating recursive threaded-code interpreter architecture in Rust.

## Benchmarks

So far benchmarks look as if the current implementation is comparable with `wasmi` performance.
This is sad since `wasmi` is a simple switch-based interpreter without any major optimizations.

## Note

This implementation heavily relies on Rust to optimize function calls into tail calls.
Otherwise the call stack is easily going to overflow since this interpreter architecture works
by each executed instruction recursively calling the next instruction at its end.

## How does it work?

All instructions in this interpreter architecture have the following structure:
```rust
struct Instruction {
    f: fn(&mut Context, reg: Register, aux: Register) -> Resutl<(), Trap>,
    aux: Register,
}
```

Where `f` is a function pointer to the underlying implementation of the instruction
and `aux` is some auxiliary register value that is used by some instructions to fulfil
special purposes. For example:

- For `i32.const` it contains the constant value `c`.
- For `br_if` it contains the target destination value.
- For `br_table` it contains an index into the targets and default target.
    - The targets and default targets of `br_table` instructions reside in a special buffer.

Basically all instructions of the Wasm specification can be handled in one or another way
similar to the instructions shown above.

This architecture follows the principles of a stack based virtual machine therefore
the `Context` contains the current program counter `pc` and the value stack.
Registers are just fancy wrappers around a `u64` value and their interpretation
depends on the instruction that is using them.

Functions are registered into a database that we call `Engine`.

When a function `f` is called the following things are happening:

- The instructions of `f` are extracted from the `Engine` `e` into a buffer.
- A special execution `Context` `ctx` with `pc = 0` is initialized.
- The function parameters are stored on the value stack.
- The top value stack element is popped (if any) and set to `reg`.
- The instruction `Instruction { f, aux }` at index `0` is queried and executed with `(f)(&mut ctx, reg, aux)`.
- At the end of the instruction execution the instruction itself updates the `pc` and calls the next instruction.
    - This only works properly if the last execution is tail call optimized by Rust/LLVM.

## Run Tests

To run tests type

```
cargo test --release
```

If you forget `--release` the tests might fail with a stack overflow since Rust won't optimize for tail calls:

```
thread 'bench_counter_loop' has overflowed its stack
fatal runtime error: stack overflow
```

## Run Benchmarks

To run benchmarks type

```
cargo bench
```
