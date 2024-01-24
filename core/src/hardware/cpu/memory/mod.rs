mod registers;
pub use registers::{Reg16, Reg8, Registers};

mod stack;
pub use stack::Stack;

use super::CPU;
use crate::hardware::bus;

/// A representation of for the reading of the "immediate" memory at the `program_counter` position.
#[derive(Debug, Clone, Copy)]
pub struct Immediate<const BitSize: usize>;
impl Immediate<8> {
    #[allow(non_upper_case_globals)]
    pub const n8: Self = Self;
}
impl Immediate<16> {
    #[allow(non_upper_case_globals)]
    pub const n16: Self = Self;
}

/// Memory address pointer variants
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum AddrPtr {
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
    /// The 2 bytes (little endian) following the current Program Counter at the opcode
    /// contain a pointer to memory address
    a16(u16),
    // {
    //     /// First byte following the Program Counter of the current opcode
    //     lsb: u8,
    //     /// Second byte following the Program Counter of the current opcode
    //     msb: u8
    // },
    /// The pointer is the value of `0xFF{lsb}`
    ZeroPage {
        /// `a8`
        lsb: u8,
    },
    /// The pointer is the value of `0xFF{Reg8::C.read()}`
    ZeroPageC,
}
impl AddrPtr {
    /// Converts addr to an absolute pointer to an address on the bus
    pub fn as_addr(&self, cpu: &mut CPU) -> bus::Addr {
        match self {
            AddrPtr::BC => cpu.registers.bc(),
            AddrPtr::DE => cpu.registers.de(),
            AddrPtr::HL => cpu.registers.hl(),
            AddrPtr::HLD => {
                let hl = cpu.registers.hl();
                cpu.registers.write16(Reg16::HL, hl - 1);
                hl
            }
            AddrPtr::HLI => {
                let hl = cpu.registers.hl();
                cpu.registers.write16(Reg16::HL, hl + 1);
                hl
            }
            AddrPtr::ZeroPage { lsb } => bus::regions::HIGH_MEMORY | (*lsb as bus::Addr),
            AddrPtr::ZeroPageC => bus::regions::HIGH_MEMORY | (cpu.registers.c as bus::Addr),
            AddrPtr::a16(addr) => *addr,
        }
    }
}

pub trait Out<Src: Copy, T = u8> {
    fn read(&mut self, source: Src, bus: &bus::Interface) -> T;
}
pub trait In<Dest: Copy, T = u8> {
    fn write(&mut self, destination: Dest, data: T, bus: &mut bus::Interface) -> &mut Self;
}

impl Out<Reg8> for CPU {
    fn read(&mut self, register: Reg8, bus: &bus::Interface) -> u8 {
        use Reg8::*;

        match register {
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
impl In<Reg8> for CPU {
    fn write(&mut self, register: Reg8, data: u8, bus: &mut bus::Interface) -> &mut Self {
        use Reg8::*;

        match register {
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

impl Out<Reg16, u16> for CPU {
    fn read(&mut self, register: Reg16, bus: &bus::Interface) -> u16 {
        self.registers.read16(register)
    }
}
impl In<Reg16, u16> for CPU {
    fn write(&mut self, register: Reg16, data: u16, bus: &mut bus::Interface) -> &mut Self {
        use Reg16::*;

        let [lsb, msb] = data.to_le_bytes();
        match register {
            // TODO: check if the which register needs the lsb and msb
            BC => {
                self.registers.b = lsb;
                self.registers.c = msb;
            }
            DE => {
                self.registers.d = lsb;
                self.registers.e = msb;
            }
            HL => {
                self.registers.h = lsb;
                self.registers.l = msb;
            }
            AF => unreachable!(),
            SP => self.registers.stack.pointer = data,
        };

        return self;
    }
}

impl In<AddrPtr, u8> for CPU {
    fn write(&mut self, destination: AddrPtr, data: u8, bus: &mut bus::Interface) -> &mut Self {
        bus[destination.as_addr(self)] = data;
        return self;
    }
}
impl Out<AddrPtr, u8> for CPU {
    fn read(&mut self, source: AddrPtr, bus: &bus::Interface) -> u8 {
        bus[source.as_addr(self)]
    }
}

impl Out<Immediate<8>, u8> for CPU {
    fn read(&mut self, _: Immediate<8>, bus: &bus::Interface) -> u8 {
        self.next_byte(bus)
    }
}
impl Out<Immediate<16>, u16> for CPU {
    fn read(&mut self, source: Immediate<16>, bus: &bus::Interface) -> u16 {
        self.next_u16(bus)
    }
}

impl Out<u8, u8> for CPU {
    #[inline]
    fn read(&mut self, source: u8, bus: &bus::Interface) -> u8 {
        source
    }
}
