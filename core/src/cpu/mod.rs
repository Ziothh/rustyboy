//! # CPU
//!
//! ## Useful links
//!  - [Hardware registers](https://gbdev.io/pandocs/Hardware_Reg_List.html)
use crate::{
    cartridge::Cartridge,
    memory_bus::{self, MemoryMappedRegion},
    prelude::LittleEndian,
    GameBoy, Hardware,
};

mod memory;
use self::memory::{Registers, Stack};

mod instructions;
pub use instructions::Instruction;

pub struct CPU {
    registers: Registers,
    stack: Stack,
    /// The Program Counter (PC), the index into the program
    ///
    /// On the Game Boy this is a 16-bit register on the CPU,
    pc: u16,

    /// Determines in which opcode table the CPU should look
    is_opcode_prefixed: bool,

    hram: [u8; memory_bus::regions::size(memory_bus::regions::HRAM)],
}

impl CPU {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }

    // fn fetch_byte(&mut self) -> u8 {}

    pub fn exec_fetch(&mut self, hardware: &mut Hardware) {
        // TODO: check if the prefixed table has an instruction with opcode PREFIX_INDICATION_BYTE
        // If so: adjust this to also check if it is prefixed or not
        match self.fetch(hardware) {
            Instruction::PREFIX_INDICATION_BYTE => self.is_opcode_prefixed = true,
            prefixed_opcode if self.is_opcode_prefixed => todo!(),
            unprefixed_opcode => {
                Instruction::try_from_opcode_unprefixed(unprefixed_opcode, todo!()).unwrap();
            }
        };
    }

    pub fn fetch(&mut self, hardware: &mut Hardware) -> u8 {
        let byte = self.read(hardware, self.pc);
        self.pc.wrapping_add(1);

        return byte;
    }

    // /// Reads the current (immediate) byte, pointed to by the `program_counter` in memory,
    // /// then increments the `program_counter` by `1`.
    // pub fn next_byte(&mut self) -> u8 {
    //     let byte = self.bus.read8(self.program_counter);
    //     self.program_counter += 1;
    //
    //     return byte;
    // }

    fn read(&mut self, hardware: &mut Hardware, addr: u16) -> u8 {
        use crate::memory_bus::regions;

        match addr {
            0x0000..=0x00FF if hardware.bootrom.is_active => hardware.bootrom.rom[addr as usize],
            0x0000..=0x7FFF => hardware.cartridge.bus_read(addr),
            0x8000..=0x9FFF => todo!("VRAM"),
            0xA000..=0xBFFF => todo!("EXTERNAL RAM (ROM)"),
            0xC000..=0xCFFF => todo!("WRAM_FIXED"),
            // I won't be supporting CGB for now so prob don't need to make a distinction
            0xD000..=0xDFFF => todo!("WRAM_SWITCHABLE (CGB)"),
            0xE000..=0xFDFF => todo!("ECHO RAM (C000~DDFF)"),
            0xFE00..=0xFE9F => {
                if todo!("CHECK if PPU is drawing") {
                    return 0xFF;
                }

                return todo!("VRAM read");
            }
            0xFEA0..=0xFEFF => todo!("NOT USABLE (read comments for impl)"),
            0xFF00..=0xFFFF => self.read_high(hardware, addr as u8),
        }
    }
    /// Read high memory.
    ///
    /// The given addr should be in range `0x0000..=0x00FF`
    fn read_high(&mut self, hardware: &mut Hardware, addr: u8) -> u8 {
        match addr {
            0x00 => hardware.joypad.read_register(),
            0x01 => todo!("SB: Serial Transfer Data"),
            0x02 => todo!("SC: Serial Transfer Control"),
            _ => 0xFF,
        }
    }

    fn write(&mut self, hardware: &mut Hardware, addr: u16, byte: u8) {
        use crate::memory_bus::regions;

        match addr {
            0x0000..=0x00FF if hardware.bootrom.is_active => todo!(),
            0x0000..=0x7FFF => todo!("CARTRIDGE"),
            0x8000..=0x9FFF => todo!("VRAM"),
            0xA000..=0xBFFF => todo!("EXTERNAL RAM (ROM)"),
            0xC000..=0xCFFF => todo!("WRAM_FIXED"),
            // I won't be supporting CGB for now so prob don't need to make a distinction
            0xD000..=0xDFFF => todo!("WRAM_SWITCHABLE (CGB)"),
            0xE000..=0xFDFF => todo!("ECHO RAM (C000~DDFF)"),
            0xFE00..=0xFE9F => todo!("VRAM read"),
            0xFEA0..=0xFEFF => todo!("NOT USABLE (read comments for impl)"),
            0xFF00..=0xFFFF => self.write_high(hardware, addr as u8, byte),
        };

        return;
    }
    /// Write high memory.
    ///
    /// The given addr should be in range `0x0000..=0x00FF`
    fn write_high(&mut self, hardware: &mut Hardware, addr: u8, byte: u8) {
        match addr {
            0x00 => hardware.joypad.write_register(byte),
            _ => { /* no-op */ }
        };

        return;
    }

    // /// Reads the current (immediate) byte, pointed to by the `program_counter` in memory
    // /// without incrementing the `program_counter`.
    // pub fn read_immediate(&self) -> u8 {
    //
    //     todo!()
    //     // self.bus.read8(self.program_counter)
    // }

    //
    //
    // /// Reads the next byte as an `i8`.
    // /// Increments the `program_counter` by `1`.
    // pub fn next_i8(&mut self) -> i8 {
    //     self.next_byte() as i8
    // }
    //
    // /// Reads the next 2 bytes and combines them to create a `u16`.
    // ///
    // /// The `program_counter` is incremented by `2` (`1` for each byte).
    // ///
    // /// Read bytes:
    // ///  - first byte is the lower nibble (lsb)
    // ///  - second byte is the upper nibble (msb)
    // pub fn next_u16(&mut self) -> u16 {
    //     u16::from_bytes((self.next_byte(), self.next_byte()))
    // }
    //
    // /// Decodes the next instruction of the program.
    // ///
    // /// The `program_counter` is incremented by `n` (based on the instruction).
    // ///
    // /// @alias self.next()
    // #[inline]
    // pub fn next_instruction(&mut self) -> Option<Instruction> {
    //     self.next()
    // }
    //
    // /// Skips the next byte in the program by incrementing the the `program_counter` by `1`
    // pub fn skip_byte(&mut self) -> &mut Self {
    //     self.program_counter += 1;
    //     return self;
    // }
}

impl Default for CPU {
    fn default() -> Self {
        return Self {
            pc: 0,
            is_opcode_prefixed: false,
            stack: Default::default(),
            registers: Default::default(),
            hram: [0; _],
        };
    }
}

/// An iterator that iterates over all of the program bytes and parses them into `Instruction`s
struct Program {
    /// The Program Counter (PC) is the index into the program bytes
    pub program_counter: u16,
    /// Wether the next opcode is prefixed.
    pub prefixed: bool,
}

impl Default for Program {
    fn default() -> Self {
        return Self {
            program_counter: 0,
            prefixed: false,
        };
    }
}

impl Program {
    /// Returns the Program Counter (PC), the index into the program
    ///
    /// On the Game Boy this is a 16-bit register on the CPU,
    /// but we keep it here for separation on concerns
    pub fn pc(&self) -> u16 {
        self.program_counter
    }

    #[inline]
    /// Wether the next opcode byte is prefixed
    pub fn is_prefixed(&self) -> bool {
        self.prefixed
    }

    /// Reads the current (immediate) byte, pointed to by the `program_counter` in memory
    /// without incrementing the `program_counter`.
    pub fn read_immediate(&self) -> u8 {
        todo!()
        // self.bus.read8(self.program_counter)
    }

    /// Reads the current (immediate) byte, pointed to by the `program_counter` in memory,
    /// then increments the `program_counter` by `1`.
    pub fn next_byte(&mut self) -> u8 {
        todo!()
        // let byte = self.bus.read8(self.program_counter);
        // self.program_counter += 1;
        //
        // return byte;
    }

    /// Reads the next byte as an `i8`.
    /// Increments the `program_counter` by `1`.
    pub fn next_i8(&mut self) -> i8 {
        self.next_byte() as i8
    }

    /// Reads the next 2 bytes and combines them to create a `u16`.
    ///
    /// The `program_counter` is incremented by `2` (`1` for each byte).
    ///
    /// Read bytes:
    ///  - first byte is the lower nibble (lsb)
    ///  - second byte is the upper nibble (msb)
    pub fn next_u16(&mut self) -> u16 {
        u16::from_bytes((self.next_byte(), self.next_byte()))
    }

    /// Decodes the next instruction of the program.
    ///
    /// The `program_counter` is incremented by `n` (based on the instruction).
    ///
    /// @alias self.next()
    #[inline]
    pub fn next_instruction(&mut self) -> Option<Instruction> {
        self.next()
    }

    /// Skips the next byte in the program by incrementing the the `program_counter` by `1`
    pub fn skip_byte(&mut self) -> &mut Self {
        self.program_counter += 1;
        return self;
    }
}

impl Iterator for Program {
    type Item = Instruction;

    /// Decodes the next instruction of the program
    ///
    /// @alias self.next_instruction()
    fn next(&mut self) -> Option<Self::Item> {
        let opcode = self.next_byte();

        // TODO: check if the prefixed table has an instruction with opcode PREFIX_INDICATION_BYTE
        // If so: adjust this to also check if it is prefixed or not
        if opcode == Instruction::PREFIX_INDICATION_BYTE {
            self.prefixed = true;
            return self.next();
        }

        return Some(if self.is_prefixed() {
            // Self::try_from_opcode_prefixed(byte, program)
            todo!()
        } else {
            Instruction::try_from_opcode_unprefixed(opcode, self).unwrap()
        });
    }
}
