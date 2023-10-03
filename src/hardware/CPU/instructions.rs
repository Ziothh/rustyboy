use std::ops::Index;

use super::registers::Registers;

pub enum Instruction {
    ADD(ArithmeticTarget),
    INC(IncDecTarget),
    /// Jump
    JP(JumpTest),
    LD(LoadType),
    // [Register data instructions]
    // TODO

    // ADDHL (add to HL) - just like ADD except that the target is added to the HL register
    // ADC (add with carry) - just like ADD except that the value of the carry flag is also added to the number
    // SUB (subtract) - subtract the value stored in a specific register with the value in the A register
    // SBC (subtract with carry) - just like ADD except that the value of the carry flag is also subtracted from the number
    // AND (logical and) - do a bitwise and on the value in a specific register and the value in the A register
    // OR (logical or) - do a bitwise or on the value in a specific register and the value in the A register
    // XOR (logical xor) - do a bitwise xor on the value in a specific register and the value in the A register
    // CP (compare) - just like SUB except the result of the subtraction is not stored back into A
    // INC (increment) - increment the value in a specific register by 1
    // DEC (decrement) - decrement the value in a specific register by 1
    // CCF (complement carry flag) - toggle the value of the carry flag
    // SCF (set carry flag) - set the carry flag to true
    // RRA (rotate right A register) - bit rotate A register right through the carry flag
    // RLA (rotate left A register) - bit rotate A register left through the carry flag
    // RRCA (rotate right A register) - bit rotate A register right (not through the carry flag)
    // RRLA (rotate left A register) - bit rotate A register left (not through the carry flag)
    // CPL (complement) - toggle every bit of the A register
    // BIT (bit test) - test to see if a specific bit of a specific register is set
    // RESET (bit reset) - set a specific bit of a specific register to 0
    // SET (bit set) - set a specific bit of a specific register to 1
    // SRL (shift right logical) - bit shift a specific register right by 1
    // RR (rotate right) - bit rotate a specific register right by 1 through the carry flag
    // RL (rotate left) - bit rotate a specific register left by 1 through the carry flag
    // RRC (rorate right) - bit rotate a specific register right by 1 (not through the carry flag)
    // RLC (rorate left) - bit rotate a specific register left by 1 (not through the carry flag)
    // SRA (shift right arithmetic) - arithmetic shift a specific register right by 1
    // SLA (shift left arithmetic) - arithmetic shift a specific register left by 1
    // SWAP (swap nibbles) - switch upper and lower nibble of a specific register
}

impl Instruction {
    pub const PREFIX_INDICATION_BYTE: u8 = 0xCB;

    pub fn try_from_byte(byte: u8, prefixed: bool) -> Result<Self, String> {
        if prefixed {
            Self::try_from_byte_prefixed(byte)
        } else {
            Self::try_from_byte_default(byte)
        }
    }

    /// Non-prefixed instructions
    pub fn try_from_byte_default(byte: u8) -> Result<Self, String> {
        match byte {
            0x02 => Ok(Instruction::INC(IncDecTarget::BC)),
            0x13 => Ok(Instruction::INC(IncDecTarget::DE)),

            // Errors
            Self::PREFIX_INDICATION_BYTE => Err(format!("Instruction prefix indication byte 0x{byte:X} was passed instead of a valid instruction byte")),
            _ => Err(format!("Unkown instruction found for: 0x{byte:X}")),
        }
    }

    /// Prefixed instructions
    pub fn try_from_byte_prefixed(byte: u8) -> Result<Self, String> {
        match byte {
            // Errors
            Self::PREFIX_INDICATION_BYTE => Err(format!("Instruction prefix indication byte 0xCB{byte:X} was passed instead of a valid prefixed instruction byte")),
            _ => Err(format!("Unkown prefixed instruction found for: 0xCB{byte:x}")),
        }
    }
}

#[rustfmt::skip]
pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

#[rustfmt::skip]
pub enum LoadByteSource {
    A, B, C, D, E, H, L, 
    /// Direct 8bit value
    D8, 
    /// Indicates that we use the HL virtual u16 as a load address
    HLI
}
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    
    // Word: just like the Byte type except with 16-bit values
    // AFromIndirect: load the A register with the contents from a value from a memory location whose address is stored in some location
    // IndirectFromA: load a memory location whose address is stored in some location with the contents of the A register
    // AFromByteAddress: Just like AFromIndirect except the memory address is some address in the very last byte of memory.
    // ByteAddressFromA: Just like IndirectFromA except the memory address is some address in the very last byte of memory.

}
pub enum IncDecTarget {
    // TODO: add other targets
    BC,
    DE,
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl ArithmeticTarget {
    pub fn to_register(&self, registers: &Registers) -> u8 {
        match *self {
            Self::A => registers.a,
            Self::B => registers.b,
            Self::C => registers.c,
            Self::D => registers.d,
            Self::E => registers.e,
            Self::H => registers.h,
            Self::L => registers.l,
        }
    }
}

impl Index<ArithmeticTarget> for Registers {
    type Output = u8;

    fn index(&self, index: ArithmeticTarget) -> &Self::Output {
        match index {
            ArithmeticTarget::A => &self.a,
            ArithmeticTarget::B => &self.b,
            ArithmeticTarget::C => &self.c,
            ArithmeticTarget::D => &self.d,
            ArithmeticTarget::E => &self.e,
            ArithmeticTarget::H => &self.h,
            ArithmeticTarget::L => &self.l,
        }
    }
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}
