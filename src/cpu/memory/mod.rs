mod registers;
pub use registers::{Reg16, Reg8, Registers};

mod stack;
pub use stack::Stack;

use crate::cpu::CPU;

/// A representation of for the reading of the "immediate" memory at the `program_counter` position.
#[derive(Debug)]
pub struct Immediate8;

/// Memory address pointer variants
#[derive(Debug)]
pub enum Address {
    /// BC, a combined 16-bit register, contains the pointer to the memory address
    BC,
    /// DE, a combined 16-bit register, contains the pointer to the memory address
    DE,
    /// HL, a combined 16-bit register, contains the pointer to the memory address
    HL,
    /// HL, a combined 16-bit register, contains the pointer to the memory address
    /// HL is DECREMENTED after the pointer is read
    HLD,
    /// HL, a combined 16-bit register, contains the pointer to the memory address
    /// HL is INCREMENTED after the pointer is read
    HLI,
    /// The 2 bytes (nibbles) following the current Program Counter on the opcode
    /// contain the pointer to the memory address
    Direct(u16),
    // { 
    //     /// First byte following the Program Counter of the current opcode
    //     lsb: u8, 
    //     /// Second byte following the Program Counter of the current opcode
    //     msb: u8 
    // },
    /// The pointer is the value of `0xFF{lsb}`
    ZeroPage { lsb: u8 },
    /// The pointer is the value of `0xFF{Reg8::C.read()}`
    ZeroPageC,
}

pub trait Read8<T: Copy> {
    fn read(&mut self, source: T) -> u8;
}
pub trait Write8<T: Copy> {
    fn write(&mut self, destination: T, data: u8) -> &mut Self;
}

impl Read8<Reg8> for CPU {
    fn read(&mut self, source: Reg8) -> u8 {
        use Reg8::*;

        match source {
            A => self.registers.a,
            B => self.registers.b,
            C => self.registers.c,
            D => self.registers.d,
            E => self.registers.e,
            H => self.registers.h,
            L => self.registers.l,
        }
    }
}

impl Write8<Reg8> for CPU {
    fn write(&mut self, destination: Reg8, data: u8) -> &mut Self {
        use Reg8::*;

        match destination {
            A => self.registers.a = data,
            B => self.registers.b = data,
            C => self.registers.c = data,
            D => self.registers.d = data,
            E => self.registers.e = data,
            H => self.registers.h = data,
            L => self.registers.l = data,
        };

        return self;
    }
}
