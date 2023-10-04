use std::ops::Index;

use crate::{
    prelude::NibbleFrom16bit,
    program::Program,
    CPU::memory::{Reg16, Reg8, Registers},
};


impl Instruction {
    pub const PREFIX_INDICATION_BYTE: u8 = 0xCB;

    pub fn try_from_opcode(byte: u8, program: &mut Program) -> Result<Self, String> {
        if program.is_prefixed() {
            Self::try_from_opcode_prefixed(byte, program)
        } else {
            Self::try_from_opcode_default(byte, program)
        }
    }

    /// Non-prefixed instructions
    pub fn try_from_opcode_default(byte: u8, program: &mut Program) -> Result<Self, String> {
        match byte {
            // [8-bit load instructions]
            // LD r, r’: Load register (register)
            0x41 => Ok(Instruction::LD(
                LoadType::Register { destination: program.next_byte().try_into()?, source: program.next_byte().try_into()?, }
            )),
            // LD r, n: Load register (immediate)
            0x06 => Ok(Instruction::LD(
                LoadType::Immediate { destination: program.next_byte().try_into()? }
            )),
            // LD r, (HL): Load register (indirect HL)
            0x46 => Ok(Instruction::LD(
                LoadType::FromIndirect { destination: program.next_byte().try_into()?, source: Reg16::HL }
            )),
            // LD (HL), r: Load from register (indirect HL)
            0x70 => Ok(Instruction::LD(
                LoadType::ToIndirect { source: program.next_byte().try_into()?, destination: Reg16::HL }
            )),
            // LD (HL), n: Load from immediate data (indirect HL)
            0x36 => Ok(Instruction::LD(
                LoadType::ImmediateToIndirect
            )),
            // LD A, (BC): Load accumulator (indirect BC)
            0x0A => Ok(Instruction::LD(
                LoadType::FromIndirect { destination: Reg8::A, source: Reg16::BC }
            )),
            // LD A, (DE): Load accumulator (indirect DE)
            0x1A => Ok(Instruction::LD(
                LoadType::FromIndirect { destination: Reg8::A, source: Reg16::DE }
            )),
            // LD (BC), A: Load from accumulator (indirect BC)
            0x02 => Ok(Instruction::LD(
                LoadType::ToIndirect { source: Reg8::A, destination: Reg16::BC }
            )),
            // LD (DE), A: Load from accumulator (indirect DE)
            0x12 => Ok(Instruction::LD(
                LoadType::ToIndirect { source: Reg8::A, destination: Reg16::DE }
            )),
            // LD A, (nn): Load accumulator (direct)
            0xFA => Ok(Instruction::LD(
                LoadType::FromDirect { destination: Reg8::A, source: u16::from_nibbles(*program.next_byte(), *program.next_byte()) } 
            )),
            // LD (nn), A: Load from accumulator (direct)
            0xEA => Ok(Instruction::LD(
                LoadType::ToDirect { destination: u16::from_nibbles(*program.next_byte(), *program.next_byte()), source: Reg8::A } 
            )),
            // LDH A, (C): Load accumulator (indirect 0xFF00+C)
            0xF2 => Ok(Instruction::LDH(
                LoadHalfwordType::FromIndirect { source: Reg8::C }
            )),
            // LDH (C), A: Load from accumulator (indirect 0xFF00+C)
            0xE2 => Ok(Instruction::LDH(
                LoadHalfwordType::ToIndirect { destination: Reg8::C }
            )),
            // LDH A, (n): Load accumulator (direct 0xFF00+n)
            0xF0 => Ok(Instruction::LDH(
                LoadHalfwordType::FromDirect { source: *program.next_byte() }
            )),
            // LDH (n), A: Load from accumulator (direct 0xFF00+n)
            0xE0 => Ok(Instruction::LDH(
                LoadHalfwordType::ToDirect { destination: *program.next_byte() }
            )),
            // LD A, (HL-): Load accumulator (indirect HL, decrement)
            0x3A => Ok(Instruction::LD(LoadType::DecFromIndirect)),
            // LD (HL-), A: Load from accumulator (indirect HL, decrement)
            0x32 => Ok(Instruction::LD(LoadType::DecToIndirect)),
            // LD A, (HL+): Load accumulator (indirect HL, increment)
            0x2A => Ok(Instruction::LD(LoadType::IncFromIndirect)),
            // LD (HL+), A: Load from accumulator (indirect HL, increment)
            0x22 => Ok(Instruction::LD(LoadType::IncToIndirect)),

            // [16-bit load instructions]
            0x01 => Ok(Instruction::LD(LoadType::DirectToReg16)),
            


            // Errors
            Self::PREFIX_INDICATION_BYTE => Err(format!("Instruction prefix indication byte 0x{byte:X} was passed instead of a valid instruction byte")),
            _ => Err(format!("Unkown instruction found for: 0x{byte:X}")),
        }
    }

    /// Prefixed instructions
    pub fn try_from_opcode_prefixed(byte: u8, program: &mut Program) -> Result<Self, String> {
        match byte {
            // Errors
            Self::PREFIX_INDICATION_BYTE => Err(format!("Instruction prefix indication byte 0xCB{byte:X} was passed instead of a valid prefixed instruction byte")),
            _ => Err(format!("Unkown prefixed instruction found for: 0xCB{byte:x}")),
        }
    }
}

pub enum LoadType {
    /// Load the bytes from the `source` register into the `destination` register.
    ///
    /// # Pseudo code
    /// ```ignore
    /// // example: LD B, C
    /// B = C
    /// ```
    Register {
        /// The first byte after the opcode
        destination: Reg8,
        /// The second byte after the opcode
        source: Reg8,
    },

    /// Load the bytes from the immidiate memory address at the `program_counter` into the `destination` register.
    ///
    /// # Pseudo code
    /// ```ignore
    /// // example: LD B, n
    /// B = read(PC++)
    /// ```
    Immediate { destination: Reg8 },

    /// Load the bytes from the memory address pointed to by the `HL` combined register into the `destination` register.
    ///
    /// # Pseudo code
    /// ```ignore
    /// // example: LD B, (HL)
    /// B = read(HL)
    /// ```
    FromIndirect {
        /// The register receiving the value.
        destination: Reg8,
        /// The combined register containing a pointer to the memory addres.
        source: Reg16,
    },

    /// Load to the 8-bit register, data from the absolute address specified by the 16-bit register.
    ///
    /// # Pseudo code
    /// ```ignore
    /// // example: LD (HL), B
    /// write(HL, B)
    /// ```
    ToIndirect {
        /// The combined register containing a pointer to the memory addres.
        destination: Reg16,
        /// The register containing the value.
        source: Reg8,
    },

    /// Load the bytes from `destination` register into the memory address pointed to by the `HL` register.
    ///
    /// # Pseudo code
    /// ```ignore
    /// n = read(PC++)
    /// write(HL, n)
    /// ```
    ImmediateToIndirect,

    /// Load to the 8-bit register, data from the absolute address specified by the 16-bit operand nn
    ///
    /// # Pseudo code
    /// ```ignore
    /// nn = unsigned_16(lsb=read(PC++), msb=read(PC++))
    /// A = read(nn)
    /// ```
    FromDirect {
        /// The 8-bit register that the data gets loaded into
        destination: Reg8,
        /// The memory address of the data that needs to be written to the destination
        source: u16,
    },

    /// Load to the absolute address specified by the 16-bit operand nn, data from the 8-bit register.    
    /// 
    /// # Pseudo code
    /// ```ignore
    /// nn = unsigned_16(lsb=read(PC++), msb=read(PC++))
    /// write(nn, A)
    /// ```
    ToDirect {
        /// The memory address where the data gets loaded into
        destination: u16,
        /// The 8-bit register with the memory address of the data
        source: Reg8,
    },

    /// Load to the 8-bit `A` register, data from the absolute address specified by the 16-bit register `HL`. 
    /// The value of `HL` is decremented after the memory read.
    ///
    /// # Pseudo code
    /// ```ignore
    /// A = read(HL--)
    /// ```
    DecFromIndirect,
    /// Load to the absolute address specified by the 16-bit register `HL`, data from the 8-bit `A` register. 
    /// The value of HL is decremented after the memory write.
    ///
    /// # Pseudo code
    /// ```ignore
    /// write(HL--, A)
    /// ```
    DecToIndirect,

    /// Load to the 8-bit `A` register, data from the absolute address specified by the 16-bit register `HL`. 
    /// The value of `HL` is incremented after the memory read.
    ///
    /// # Pseudo code
    /// ```ignore
    /// A = read(HL++)
    /// ```
    IncFromIndirect,
    /// Load to the absolute address specified by the 16-bit register `HL`, data from the 8-bit `A` register. 
    /// The value of `HL` is incremented after the memory write.
    ///
    /// # Pseudo code
    /// ```ignore
    /// write(HL++, A)
    /// ```
    IncToIndirect,

    /// Load to the 16-bit register `Reg16`, the immediate 16-bit data nn.
    ///
    /// # Pseudo code
    /// ```ignore
    /// nn = unsigned_16(lsb=read(PC++), msb=read(PC++))
    /// BC = nn
    /// ```
    DirectToReg16,
}

pub enum LoadHalfwordType {
    /// Load to the 8-bit `A` register, data from the address specified by the 8-bit `source` register. 
    /// The full 16-bit absolute address is obtained by setting the most significant `byte` to 0xFF 
    /// and the least significant byte to the value of `source`, so the possible range is `0xFF00..=0xFFFF`.
    ///
    /// # Pseudo code
    /// ```ignore
    /// // Example
    ///
    /// A = read(unsigned_16(lsb=C, msb=0xFF))
    /// ```
    FromIndirect {
        /// The register containing the lsb of the memory address
        source: Reg8,
    },

    /// Load to the address specified by the 8-bit `destination` register, data from the 8-bit `A` register. 
    /// The full 16-bit absolute address is obtained by setting the most significant byte to `0xFF` 
    /// and the least significant byte to the value of `destination`, so the possible range is `0xFF00..=0xFFFF`.
    ///
    /// # Pseudo code
    /// ```ignore
    /// // Example
    ///
    /// write(unsigned_16(lsb=C, msb=0xFF), A)
    /// ```
    ToIndirect {
        /// The register containing the lsb of the memory address
        destination: Reg8,
    },

    /// Load to the 8-bit `A` register, data from the address specified by the 8-bit immediate data `source`. 
    /// The full 16-bit absolute address is obtained by setting the most significant byte to `0xFF` and the least significant byte 
    /// to the value of `source`, so the possible range is `0xFF00..=0xFFFF`.
    ///
    /// # Pseudo code
    /// ```ignore
    /// // Example
    ///
    /// n = read(PC++)
    /// A = read(unsigned_16(lsb=n, msb=0xFF))
    /// ```
    FromDirect {
        /// The lsb of the memory address of the value
        source: u8,
    },

    /// Load to the address specified by the 8-bit immediate data `destination`, data from the 8-bit `A` register. 
    /// The full 16-bit absolute address is obtained by setting the most significant byte to `0xFF` 
    /// and the least significant byte to the value of `A`, so the possible range is `0xFF00-0xFFFF`.
    ///
    /// # Pseudo code
    /// ```ignore
    /// // Example
    ///
    /// n = read(PC++)
    /// write(unsigned_16(lsb=n, msb=0xFF), A)
    /// ```
    ToDirect {
        /// The lsb of the memory address containnig the value
        destination: u8,
    },
}




//
// pub enum IncDecTarget {
//     // TODO: add other targets
//     BC,
//     DE,
// }
//
// pub enum ArithmeticTarget {
//     A,
//     B,
//     C,
//     D,
//     E,
//     H,
//     L,
// }
//
// impl ArithmeticTarget {
//     pub fn to_register(&self, registers: &Registers) -> u8 {
//         match *self {
//             Self::A => registers.a,
//             Self::B => registers.b,
//             Self::C => registers.c,
//             Self::D => registers.d,
//             Self::E => registers.e,
//             Self::H => registers.h,
//             Self::L => registers.l,
//         }
//     }
// }
//
// impl Index<ArithmeticTarget> for Registers {
//     type Output = u8;
//
//     fn index(&self, index: ArithmeticTarget) -> &Self::Output {
//         match index {
//             ArithmeticTarget::A => &self.a,
//             ArithmeticTarget::B => &self.b,
//             ArithmeticTarget::C => &self.c,
//             ArithmeticTarget::D => &self.d,
//             ArithmeticTarget::E => &self.e,
//             ArithmeticTarget::H => &self.h,
//             ArithmeticTarget::L => &self.l,
//         }
//     }
// }
//
// pub enum JumpTest {
//     NotZero,
//     Zero,
//     NotCarry,
//     Carry,
//     Always,
// }
