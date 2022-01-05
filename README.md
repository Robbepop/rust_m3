# Rust Threaded Code Architecture

An attempt at recreating recursive threaded-code interpreter architecture in Rust.

## Benchmarks

So far benchmarks look as if the current implementation is comparable with `wasmi` performance.
This is sad since `wasmi` is a simple switch-based interpreter without any major optimizations.

## Note

This implementation heavily relies on Rust to optimize function calls into tail calls.
Otherwise the call stack is easily going to overflow since this interpreter architecture works
by each executed instruction recursively calling the next instruction at its end.

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
