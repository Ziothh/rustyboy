use crate::{hardware::bus::MemoryBus, program::Program};

mod memory;
use self::memory::{Registers, Stack};

mod instructions;
pub use instructions::Instruction;


pub struct CPU {
    registers: Registers,
    // program_counter: u16,
    stack: Stack,
    /// Memory bus
    bus: MemoryBus,
}

impl CPU {
    pub fn run_program(&mut self, program: Program) -> &mut Self {
        for instruction in program {
            self.execute(instruction);
        }

        return self;
    }
}
