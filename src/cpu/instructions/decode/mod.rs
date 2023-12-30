use super::Instruction;

mod unprefixed;
// mod prefixed;

impl Instruction {
    /// If the CPU reads this byte it should parse the next instructino from the prefixed opcode
    /// table.
    pub const PREFIX_INDICATION_BYTE: u8 = 0xCB;
}
