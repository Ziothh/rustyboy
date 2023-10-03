use crate::hardware::memory::MemoryBus;

mod memory;
use self::memory::{Registers, Stack};

mod instructions;
pub use instructions::Instruction;


pub struct CPU {
    registers: Registers,
    program_counter: u16,
    stack: Stack,
    /// Memory bus
    bus: MemoryBus,
}
