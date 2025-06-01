//! # CPU
//!
//! ## Useful links
//!  - [Hardware registers](https://gbdev.io/pandocs/Hardware_Reg_List.html)
use crate::{
    cartridge::Cartridge,
    memory_bus::{self, MemoryMappedRegion},
    prelude::LittleEndian,
    utils::UNDEFINED_READ,
    GameBoy, Hardware,
};

mod memory;
use self::memory::{Registers};

mod instructions;
pub use instructions::Instruction;

pub struct CPU {
    pub registers: Registers,
    /// The Program Counter (PC), the index into the program
    ///
    /// On the Game Boy this is a 16-bit register on the CPU,
    pub pc: u16,

    /// Determines in which opcode table the CPU should look
    pub is_opcode_prefixed: bool,
}

impl CPU {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
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
            registers: Default::default(),
        };
    }
}

impl GameBoy {
    /// Reads the opcode at `[PC++]` and executes it
    pub(super) fn decode_fetch_exec(&mut self) {
        let opcode = self.fetch_u8();

        match self.cpu.is_opcode_prefixed {
            false => self.execute_unprefixed(opcode),
            true => self.execute_cb_prefixed(opcode),
        }
    }

    /// TODO: unimplemented
    fn cycle(&mut self) {
        // TODO: emulate OAM DMA
        // TODO: tick_cycle PPU
        // TODO: tick_cycle timer
        // TODO: tick_cycle APU
    }

    /// Reads `[PC++]` and cycles
    fn fetch_u8(&mut self) -> u8 {
        let byte = self.read_addr(self.cpu.pc);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);

        self.cycle();

        return byte;
    }

    /// Reads `[PC++] as i8` and cycles
    fn fetch_i8(&mut self) -> i8 {
        return self.fetch_u8() as i8;
    }

    /// Reads the next 2 bytes and combines them to create a `u16`.
    ///
    /// The `program_counter` is incremented by `2` (`1` for each byte).
    ///
    /// Read bytes:
    ///  - first byte is the lower nibble (lsb)
    ///  - second byte is the upper nibble (msb)
    fn fetch_u16(&mut self) -> u16 {
        u16::from_le_bytes([self.fetch_u8(), self.fetch_u8()])
    }

    // /// Reads the current (immediate) byte, pointed to by the `program_counter` in memory,
    // /// then increments the `program_counter` by `1`.
    // pub fn next_byte(&mut self) -> u8 {
    //     let byte = self.bus.read8(self.program_counter);
    //     self.program_counter += 1;
    //
    //     return byte;
    // }

    /// Read high memory.
    ///
    /// The given addr should be in range `0x0000..=0x00FF`
    fn read_high(&mut self, addr: u8) -> u8 {
        match addr {
            // # I/O Registers
            0x00 => self.hardware.joypad.read_register(),

            0x01 => todo!("SB: Serial Transfer Data"),
            0x02 => todo!("SC: Serial Transfer Control"),

            0x04 => todo!("DIV: Divider register"),
            0x05 => todo!("TIMA: Timer counter"),
            0x06 => todo!("TMA: Timer modulo"),
            0x07 => todo!("TAC: Timer control"),

            0x0F => todo!("IF: Interupt flag"),

            0x10..=0x26 => todo!("Audio"),
            0x30..=0x3F => todo!("Wave pattern"),

            0x40 => self.hardware.ppu.read_control_register(),
            0x41 => self.hardware.ppu.read_stat_register(),
            0x42 => self.hardware.ppu.scy,
            0x43 => self.hardware.ppu.scx,
            0x44 => self.hardware.ppu.ly,
            0x45 => self.hardware.ppu.lyc,
            0x46 => self.hardware.ppu.oam_dma.source,
            0x47 => self.hardware.ppu.bg_palette.bits,
            0x48 => self.hardware.ppu.obj0_palette.bits,
            0x49 => self.hardware.ppu.obj1_palette.bits,
            0x4A => self.hardware.ppu.wy,
            0x4B => self.hardware.ppu.wx,

            0x50 => !(self.hardware.bootrom.is_active as u8), // Active low

            0x80..=0xFE => todo!("HRAM"),

            0xFF => todo!("IE: Interupt Enable"),

            _ => UNDEFINED_READ,
        }
    }

    /// Write high memory.
    ///
    /// The given addr should be in range `0x0000..=0x00FF`
    fn write_high(&mut self, addr: u8, byte: u8) {
        match addr {
            // # I/O Registers
            0x00 => self.hardware.joypad.write_register(byte),

            0x01 => todo!("SB: Serial Transfer Data"),
            0x02 => todo!("SC: Serial Transfer Control"),

            0x04 => todo!("DIV: Divider register"),
            0x05 => todo!("TIMA: Timer counter"),
            0x06 => todo!("TMA: Timer modulo"),
            0x07 => todo!("TAC: Timer control"),

            0x0F => todo!("IF: Interupt flag"),

            0x10..=0x26 => todo!("Audio"),
            0x30..=0x3F => todo!("Wave pattern"),

            0x40 => self.hardware.ppu.write_control_register(byte),
            0x41 => self.hardware.ppu.write_stat_register(byte),
            0x42 => self.hardware.ppu.scy = byte,
            0x43 => self.hardware.ppu.scx = byte,
            0x44 => self.hardware.ppu.ly = 0,
            0x45 => self.hardware.ppu.lyc = byte,
            0x46 => self.hardware.ppu.oam_dma.requested = Some(byte),
            0x47 => self.hardware.ppu.bg_palette.bits = byte,
            0x48 => self.hardware.ppu.obj0_palette.bits = byte,
            0x49 => self.hardware.ppu.obj1_palette.bits = byte,
            0x4A => self.hardware.ppu.wy = byte,
            0x4B => self.hardware.ppu.wx = byte,

            0x50 => {
                if self.hardware.bootrom.is_active {
                    self.hardware.bootrom.is_active = byte & 0b0000_0001 == 0
                }
            }

            0x80..=0xFE => todo!("HRAM"),

            0xFF => todo!("IE: Interupt Enable"),

            _ => { /* no-op */ }
        };

        return;
    }

    /// NOTE(Louis): should not cause m_cycles.
    fn read_addr(&mut self, addr: u16) -> u8 {
        use crate::memory_bus::regions;

        match addr {
            0x0000..=0x00FF if self.hardware.bootrom.is_active => {
                self.hardware.bootrom.rom[addr as usize]
            }
            0x0000..=0x3FFF => self.hardware.cartridge.bus_read_0x0000_0x3FFF(addr),
            0x4000..=0x7FFF => self.hardware.cartridge.bus_read_0x4000_0x7FFF(addr),
            0x8000..=0x9FFF => self.hardware.ppu.read_vram(addr),
            0xA000..=0xBFFF => todo!("EXTERNAL RAM (ROM)"),
            0xC000..=0xCFFF => todo!("WRAM_FIXED"),
            // I won't be supporting CGB for now so prob don't need to make a distinction
            0xD000..=0xDFFF => todo!("WRAM_SWITCHABLE (CGB)"),
            0xE000..=0xFDFF => todo!("ECHO RAM (C000~DDFF)"),
            0xFE00..=0xFE9F => {
                if todo!("CHECK if PPU is drawing") {
                    return UNDEFINED_READ;
                }

                return todo!("OAM Read");
            }
            /// https://gbdev.io/pandocs/Memory_Map.html#fea0feff-range
            0xFEA0..=0xFEFF => todo!("NOT USABLE (read comments for impl)"),
            0xFF00..=0xFFFF => self.read_high(addr as u8),
        }
    }

    fn write_addr(&mut self, addr: u16, byte: u8) {
        use crate::memory_bus::regions;

        match addr {
            0x0000..=0x00FF if self.hardware.bootrom.is_active => todo!(),
            0x0000..=0x3FFF => self.hardware.cartridge.bus_write_0x0000_0x3FFF(addr, byte),
            0x4000..=0x7FFF => self.hardware.cartridge.bus_write_0x4000_0x7FFF(addr, byte),
            0x8000..=0x9FFF => self.hardware.ppu.write_vram(addr, byte),
            0xA000..=0xBFFF => todo!("EXTERNAL RAM (ROM)"),
            0xC000..=0xCFFF => todo!("WRAM_FIXED"),
            // I won't be supporting CGB for now so prob don't need to make a distinction
            0xD000..=0xDFFF => todo!("WRAM_SWITCHABLE (CGB)"),
            0xE000..=0xFDFF => todo!("ECHO RAM (C000~DDFF)"),
            0xFE00..=0xFE9F => todo!("OAM Read"),
            /// https://gbdev.io/pandocs/Memory_Map.html#fea0feff-range
            0xFEA0..=0xFEFF => todo!("NOT USABLE (read comments for impl)"),
            0xFF00..=0xFFFF => self.write_high(addr as u8, byte),
        };

        return;
    }
}
