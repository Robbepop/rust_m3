#[derive(Debug, Copy, Clone)]
pub enum Trap {
    StackOverflow,
    ResourceExhaustion,
    AccessOutOfBounds,
    UninitializedElement,
    Unreachable,
    UnmatchedSignature,
}