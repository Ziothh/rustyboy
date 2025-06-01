mod registers;
pub use registers::{Reg16, Reg8, Registers};

mod stack;

use super::CPU;
use crate::{cpu, GameBoy};

pub trait Read8<T> {
    fn read(&mut self, source: T) -> u8;
}
pub trait Write8<T> {
    fn write(&mut self, destination: T, data: u8) -> &mut Self;
}

/// A representation of for the reading of the "immediate" memory at the `program_counter` position.
#[derive(Debug, Clone, Copy)]
pub struct Immediate8;
impl Read8<Immediate8> for GameBoy {
    fn read(&mut self, source: Immediate8) -> u8 {
        return self.fetch_u8();
    }
}

/// Memory address pointer variants
#[derive(Debug, Clone, Copy)]
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
    ///
    /// `u16::from_nibbles(lsb=mem[PC++], msb=mem[PC++])`
    Immediate,
    // {
    //     /// First byte following the Program Counter of the current opcode
    //     lsb: u8,
    //     /// Second byte following the Program Counter of the current opcode
    //     msb: u8
    // },
    /// The pointer is the value of `0xFF{lsb=mem[PC++]}`
    ZeroPage,
    /// The pointer is the value of `0xFF{Reg8::C.read()}`
    ZeroPageC,
}

impl Read8<Address> for GameBoy {
    fn read(&mut self, source: Address) -> u8 {
        let addr: u16 = self.addr_to_bus_addr(source);

        todo!("Cycle on read");
        // NOTE: immediate has correct cycles
        // Others currently have have no cycles. Fetch immdiate bytes here instead while decoding
        return self.read_addr(addr);
    }
}
impl Write8<Address> for GameBoy {
    fn write(&mut self, destination: Address, data: u8) -> &mut Self {
        let addr: u16 = self.addr_to_bus_addr(destination);

        self.write_addr(addr, data);
        self.cycle();

        return self;
    }
}
impl GameBoy {
    fn addr_to_bus_addr(&mut self, addr: Address) -> u16 {
        return match addr {
            Address::BC => self.cpu.registers.bc(),
            Address::DE => self.cpu.registers.de(),
            Address::HL => self.cpu.registers.hl(),
            Address::HLD => {
                let addr = self.cpu.registers.hl();
                self.cpu.registers.write16(Reg16::HL, addr.wrapping_sub(1));
                addr
            },
            Address::HLI => {
                let addr = self.cpu.registers.hl();
                self.cpu.registers.write16(Reg16::HL, addr.wrapping_add(1));
                addr
            },
            Address::Immediate => self.fetch_u16(),
            Address::ZeroPage => u16::from_le_bytes([self.fetch_u8(), 0xFF]),
            Address::ZeroPageC => u16::from_be_bytes([0xFF, self.cpu.registers.c]),
        };
    }
}

impl Read8<Reg8> for GameBoy {
    fn read(&mut self, source: Reg8) -> u8 {
        use Reg8::*;

        match source {
            A => self.cpu.registers.a,
            B => self.cpu.registers.b,
            C => self.cpu.registers.c,
            D => self.cpu.registers.d,
            E => self.cpu.registers.e,
            H => self.cpu.registers.h,
            L => self.cpu.registers.l,
        }
    }
}

impl Write8<Reg8> for GameBoy {
    fn write(&mut self, destination: Reg8, data: u8) -> &mut Self {
        use Reg8::*;

        match destination {
            A => self.cpu.registers.a = data,
            B => self.cpu.registers.b = data,
            C => self.cpu.registers.c = data,
            D => self.cpu.registers.d = data,
            E => self.cpu.registers.e = data,
            H => self.cpu.registers.h = data,
            L => self.cpu.registers.l = data,
        };

        return self;
    }
}
