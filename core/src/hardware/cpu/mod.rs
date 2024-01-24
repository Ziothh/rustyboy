mod program;

mod memory;
use memory::Registers;
use crate::program::Program;


mod instructions;
pub use instructions::Instruction;

pub struct CPU {
    registers: Registers,
    is_prefixed: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            is_prefixed: false,
        }
    }

    pub fn run_program(&mut self, program: Program) -> &mut Self {
        for instruction in program {
            self.execute(instruction);
        }

        return self;
    }
}
