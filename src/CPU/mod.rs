use self::{instructions::Instruction, registers::Registers, stack::Stack};
use crate::hardware::memory::MemoryBus;

mod instructions;
mod execute;
mod registers;
mod stack;

pub struct CPU {
    registers: Registers,
    program_counter: u16,
    stack: Stack,
    /// Memory bus
    bus: MemoryBus,
}

impl CPU {
    fn step(&mut self) {
        let instruction_byte = self.bus[self.program_counter];

        let is_prefixed = instruction_byte == Instruction::PREFIX_INDICATION_BYTE;

        #[rustfmt::skip]
        let instruction = Instruction::try_from_byte(
            if is_prefixed {
                instruction_byte
            } else {
                self.bus[self.program_counter + 1]
            },
            is_prefixed
        ).unwrap();

        // Execute the instruction and updated the program counter
        self.program_counter = self.execute(instruction);
    }

    /// Adds a given `value` to the `a` register and updates the flags.
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        return new_value;
    }
}
