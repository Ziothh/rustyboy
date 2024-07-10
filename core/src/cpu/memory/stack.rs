use crate::{memory_bus, prelude::LittleEndian};

/// A stack containing a stack pointer
///
/// It starts at the end of the memory and grows down.
///
/// FILO order of push/pop
pub struct Stack {
    pointer: u16,
}

impl Default for Stack {
    fn default() -> Self {
        return Self { pointer: 0 };
    }
}

impl Stack {
    /// Push a given `value` to the stack
    pub fn push(&mut self, value: u16, memory: &mut memory_bus::Bus) -> &mut Self {
        // Least significant byte
        self.pointer = self.pointer.wrapping_sub(1);
        memory[self.pointer] = value.msb();

        // Most significant byte
        self.pointer = self.pointer.wrapping_sub(1);
        memory[self.pointer] = value.msb();

        return self;
    }

    /// Pop the latest value from the stack
    pub fn pop(&mut self, memory: &mut memory_bus::Bus) -> u16 {
        let lsb = memory[self.pointer];
        self.pointer = self.pointer.wrapping_add(1);

        let msb = memory[self.pointer];
        self.pointer = self.pointer.wrapping_add(1);

        return u16::from_bytes((lsb, msb));
    }
}
