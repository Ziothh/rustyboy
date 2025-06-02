mod registers;
pub use registers::{Reg16, Reg8, Registers};

mod stack;

use crate::{cpu::CPU, GameBoy};

pub trait CpuRead<T, O> {
    fn read(&mut self, source: T) -> O;
}
pub trait CpuWrite<T, I> {
    fn write(&mut self, destination: T, data: I) -> &mut Self;
}

/// A representation of for the reading of the "immediate" memory at the `program_counter` position.
#[derive(Debug, Clone, Copy)]
pub struct Immediate<T>(std::marker::PhantomData<T>);
#[allow(non_upper_case_globals)]
impl Immediate<()> {
    /// # Immediate at `[PC++]`;
    /// Read/write takes 1 m_cyle
    pub const u8: Immediate<u8> = Immediate(std::marker::PhantomData::<u8>);
    /// # Immediate at `u16::from_le_bytes(lsb: [PC++], msb: [PC++])`;
    /// Read takes 2_cycles
    /// Write takes 4 cycles (2 read, 2 write)
    ///
    /// NOTE: currently only used for `LD (nn) SP`,
    /// maybe this should be removed and we should `impl CpuRead<Address, u16> for GameBoy {}`
    pub const u16: Immediate<u16> = Immediate(std::marker::PhantomData::<u16>);
}

impl CpuRead<Immediate<u8>, u8> for GameBoy {
    fn read(&mut self, source: Immediate<u8>) -> u8 {
        return self.fetch_u8();
    }
}

impl CpuRead<Immediate<u16>, u16> for GameBoy {
    fn read(&mut self, source: Immediate<u16>) -> u16 {
        return self.fetch_u16();
    }
}
impl CpuWrite<Immediate<u16>, u16> for GameBoy {
    fn write(&mut self, destination: Immediate<u16>, data: u16) -> &mut Self {
        let addr = self.fetch_u16();

        let [lsb, msb] = data.to_le_bytes();
        self.write_addr(addr, lsb);
        self.write_addr(addr.wrapping_add(1), msb);

        return self;
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

impl CpuRead<Address, u8> for GameBoy {
    fn read(&mut self, source: Address) -> u8 {
        let addr: u16 = self.addr_to_bus_addr(source);

        todo!("Cycle on read");
        // NOTE: immediate has correct cycles
        // Others currently have have no cycles. Fetch immdiate bytes here instead while decoding
        return self.read_addr(addr);
    }
}
impl CpuWrite<Address, u8> for GameBoy {
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
            }
            Address::HLI => {
                let addr = self.cpu.registers.hl();
                self.cpu.registers.write16(Reg16::HL, addr.wrapping_add(1));
                addr
            }
            Address::Immediate => self.fetch_u16(),
            Address::ZeroPage => u16::from_le_bytes([self.fetch_u8(), 0xFF]),
            Address::ZeroPageC => u16::from_be_bytes([0xFF, self.cpu.registers.c]),
        };
    }
}

impl CpuRead<Reg8, u8> for GameBoy {
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

impl CpuWrite<Reg8, u8> for GameBoy {
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

impl CpuRead<Reg16, u16> for GameBoy {
    fn read(&mut self, source: Reg16) -> u16 {
        return self.cpu.registers.read16(source);
    }
}

impl CpuWrite<Reg16, u16> for GameBoy {
    fn write(&mut self, destination: Reg16, data: u16) -> &mut Self {
        debug_assert!(
            destination != Reg16::AF,
            "{destination:?} Does not support load"
        );

        self.cpu.registers.write16(destination, data);

        return self;
    }
}
