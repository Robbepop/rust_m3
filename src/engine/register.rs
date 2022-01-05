
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Register(u64);

impl From<Register> for i32 {
    fn from(reg: Register) -> Self {
        reg.0 as i32
    }
}

impl From<i32> for Register {
    fn from(value: i32) -> Self {
        Self(value as u64)
    }
}
