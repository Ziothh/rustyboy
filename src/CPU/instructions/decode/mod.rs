use crate::program::Program;

use super::Instruction;

mod unprefixed;
mod prefixed;

impl Instruction {
    pub fn try_from_opcode(byte: u8, program: &mut Program) -> Result<Self, String> {
        if program.is_prefixed() {
            Self::try_from_opcode_prefixed(byte, program)
        } else {
            Self::try_from_opcode_unprefixed(byte, program)
        }
    }

}
