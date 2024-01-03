use std::ops;

use crate::prelude::LittleEndian;

/// A memory bus address of a byte
///
/// `Index<MemoryBus::Addr> for MemoryBus` has been implemented.
pub type Addr = u16;
/// An inclusive range of memory bus addresses
///
/// `Index<bus::Region> for MemoryBus` has been implemented.
pub type Region = ops::RangeInclusive<Addr>;

// All information about the memory bus comes from these links:
// - [gbdev.io](https://gbdev.io/pandocs/Memory_Map.html)

/// # Game Boy memory bus
/// 16-bit memory bus with `64kB` (0xFFFF * 8 bit) memory
///
/// ## Regions
///  - ROM (32kB | (16kB static, 16kB mapped))
///  - RAM
///    - VRAM (8kB)
///    - WRAM (8kB)
///      - Normal: (8kB)
///      - CGB mode: (4kB static, 4kB switchable)
///  - I/O registers
///
/// ## Units
/// 1kB = 1024B (base 2)
pub struct Interface([u8; u16::MAX as usize]);

impl Interface {
    const SIZE: usize = u16::MAX as usize;

    pub fn new() -> Self {
        Self([0; u16::MAX as usize])
    }

    pub fn load(&mut self, bytes: &[u8]) -> &mut Self {
        if bytes.len() > Self::SIZE {
            todo!("You can only load 64kB into the memory bus");
        }

        for (i, x) in bytes.iter().enumerate() {
            self.0[i] = *x;
        }

        return self;
    }

    pub fn read8(&self, address: u16) -> u8 {
        return self[address];
    }

    /// Reads `u16` from memory.
    ///
    /// `0x{self[index + 1]}_{self[index]}`
    ///
    /// The Game Boy is **little endian** which means that when you have numbers that are larger than 1 byte,
    /// the bytes are stored in memory from least significant to most significant.
    ///
    /// # Panics
    /// This function panics when reading on `address = u16::MAX` when reading the `msb` because
    /// it'll read on `mem[address + 1]` which is out of bounds.
    pub fn read16(&self, address: u16) -> u16 {
        return u16::from_bytes((self[address], self[address + 1]));
        // ((self[index + 1] as u16) << 8) | (self[index] as u16);
    }
}

impl ops::Index<u16> for Interface {
    type Output = u8;

    #[inline]
    fn index(&self, index: u16) -> &Self::Output {
        return self.0.index(index as usize);
    }
}
impl ops::IndexMut<u16> for Interface {
    #[inline]
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        return self.0.index_mut(index as usize);
    }
}
// impl ops::Index<ops::Range<u16>> for MemoryBus {
//     type Output = [u8];
//
//     #[inline]
//     fn index(&self, range: ops::Range<u16>) -> &Self::Output {
//         &self.0[(range.start as usize)..(range.end as usize)]
//     }
// }
impl ops::Index<ops::RangeInclusive<u16>> for Interface {
    type Output = [u8];

    #[inline]
    fn index(&self, range: ops::RangeInclusive<u16>) -> &Self::Output {
        &self.0[(*range.start() as usize)..=(*range.end() as usize)]
    }
}

#[allow(dead_code)]
pub mod regions {
    use super::super::bus;

    /// Calculates the amount of bytes in the range.
    pub const fn size(range: bus::Region) -> usize {
        (*range.end() + 1 - *range.start()) as usize
    }

    /// 16 KiB ROM bank 00
    /// From cartridge, usually a fixed bank
    pub const ROM_BANK_00: bus::Region = 0x0000..=0x3FFF;
    /// 16 KiB ROM Bank 01~NN
    /// From cartridge, switchable bank via mapper (if any)
    pub const ROM_BANK_NN: bus::Region = 0x4000..=0x7FFF;
    /// 8 KiB Video RAM (VRAM)
    /// From cartridge, switchable bank if any
    pub const VRAM: bus::Region = 0x8000..=0x9FFF;
    /// 8 KiB External RAM
    /// From cartridge, switchable bank if any
    pub const EXTERNAL_RAM: bus::Region = 0xA000..=0xBFFF;
    /// 4 KiB Work RAM (WRAM)
    pub const WRAM_FIXED: bus::Region = 0xC000..=0xCFFF;
    /// 4 KiB Work RAM (WRAM)
    /// In CGB mode, switchable bank 1~7
    pub const WRAM_SWITCHABLE: bus::Region = 0xD000..=0xDFFF;
    /// Mirror of C000~DDFF (ECHO RAM)
    /// Nintendo says use of this area is prohibited.
    pub const ECHO_RAM: bus::Region = 0xE000..=0xFDFF;
    /// Object attribute memory (OAM)
    pub const OAM: bus::Region = 0xFE00..=0xFE9F;
    /// Not Usable
    ///
    /// Nintendo says use of this area is prohibited
    /// This area returns $FF when OAM is blocked, and otherwise the behavior depends on the hardware revision.
    /// On DMG, MGB, SGB, and SGB2, reads during OAM block trigger OAM corruption.
    /// Reads otherwise return $00
    pub const NOT_USABLE: bus::Region = 0xFEA0..=0xFEFF;
    /// I/O Registers
    pub const IO_REGISTERS: bus::Region = 0xFF00..=0xFF7F;
    /// High RAM (HRAM)
    pub const HRAM: bus::Region = 0xFF80..=0xFFFE;
    /// Interrupt Enable register (IE)
    pub const IE: bus::Region = 0xFFFF..=0xFFFF;

    /// Currently only includes DMG register maps
    ///
    /// See [gbdev.io](https://gbdev.io/pandocs/Hardware_Reg_List.html) for more info
    pub mod io_registers {
        use super::bus;

        const JOYPAD: bus::Addr = 0xFF00;
        const SERIAL_TRANSFER: bus::Region = 0xFF01..=0xFF02;
        const TIME_AND_DIVIDER: bus::Region = 0xFF04..=0xFF07;
        const AUDIO: bus::Region = 0xFF10..=0xFF26;
        const WAVE_PATTERN: bus::Region = 0xFF30..=0xFF3F;
        /// LCD Control, Status, Position, Scrolling, and Palettes
        const LCD: bus::Region = 0xFF40..=0xFF4B;
        /// # OAM Direct Memory Access (DMA) transfer source address & start (R/W)
        /// Writing to this register starts a DMA transfer from ROM or RAM to OAM (Object Attribute Memory).
        ///
        /// The written value specifies the transfer source address divided by `$100`, that is, source and destination are:
        /// Source (ROM):                    $XX00-$XX9F   ;XX = $00 to $DF 
        /// Destination (OAM region in RAM): $FE00-$FE9F 
        pub const DMA: bus::Addr = 0xFF46;

        // Disabled because it first appeared on CGB instead of DMG
        // const VRAM_BANK_SELECT: u16 = 0xFF4F;
        /// Set to non-zero to disable boot ROM
        const DISABLE_BOOT_ROM: bus::Addr = 0xFF50;

        // CGB only registers so I'm not tackling this atm
        // $FF51	$FF55	CGB	VRAM DMA
        // $FF68	$FF6B	CGB	BG / OBJ Palettes
        // $FF70		CGB	WRAM Bank Select
    }

    #[allow(dead_code)]
    pub mod jump_vectors {
        use super::bus;

        /// However, this memory area (0000-00FF) may be used for any other purpose in case that your program doesn’t use any (or only some) rst instructions or interrupts. rst is a 1-byte instruction that works similarly to the 3-byte call instruction, except that the destination address is restricted. Since it is 1-byte sized, it is also slightly faster.
        pub const AREA: bus::Region = 0x0000..=0x00FF;
        pub const RST_INSTRUCTION: [bus::Addr; 8] = [
            0x0000, 0x0008, 0x0010, 0x0018, 0x0020, 0x0028, 0x0030, 0x0038,
        ];
        pub const INTERRUPTS: [bus::Addr; 5] = [0x0040, 0x0048, 0x0050, 0x0058, 0x0060];
    }

    /// The memory area at 0100-014F contains the cartridge header. This area contains information about the program, its entry point, checksums, information about the used MBC chip, the ROM and RAM sizes, etc. Most of the bytes in this area are required to be specified correctly.
    pub const CARTRIDGE_HEADER: bus::Region = 0x0100..=0x014F;
}
