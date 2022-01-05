use crate::Register;

#[derive(Debug)]
pub struct ValueStack {
    entries: Vec<Register>,
    sp: usize,
}

impl Default for ValueStack {
    fn default() -> Self {
        Self {
            entries: vec![Register::default(); 1024],
            sp: 0,
        }
    }
}

impl ValueStack {
    pub fn push(&mut self, value: impl Into<Register>) {
        self.entries[self.sp] = value.into();
        self.sp += 1;
    }

    pub fn pop(&mut self) -> Register {
        self.sp -= 1;
        self.entries[self.sp]
    }

    pub fn try_pop(&mut self) -> Option<Register> {
        if self.sp == 0 {
            return None
        }
        self.pop().into()
    }

    pub fn peek(&self, depth: u32) -> Register {
        let last = self.entries.len();
        self.entries[last - depth as usize - 1]
    }

    pub fn pop_as<T>(&mut self) -> T
    where
        T: From<Register>,
    {
        T::from(self.pop())
    }

    pub fn top(&self) -> Register {
        self.peek(0)
    }

    pub fn clear(&mut self) {
        self.sp = 0;
    }

    pub fn len(&self) -> usize {
        self.sp
    }

    pub fn drain(&mut self) -> std::vec::Drain<Register> {
        self.entries.drain(..self.sp)
    }
}
