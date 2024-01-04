//! The memory bus module
//! # Game Boy memory bus
//! 16-bit memory bus with `64kB` (0xFFFF * 8 bit) memory
//!
//! ## Regions
//!  - ROM (32kB | (16kB static, 16kB mapped))
//!  - RAM
//!    - VRAM (8kB)
//!    - WRAM (8kB)
//!      - Normal: (8kB)
//!      - CGB mode: (4kB static, 4kB switchable)
//!  - I/O registers
//!
//! ## Units
//! 1kB = 1024B (base 2)
//!
//! All information about the memory bus comes from these links:
//! - [gbdev.io](https://gbdev.io/pandocs/Memory_Map.html)

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

/// The interface to interract with the Game Boys memory bus
pub struct Interface {
    bytes: [u8; Self::SIZE],
    locked_regions: Vec<Region>,
}
impl Interface {
    const SIZE: usize = u16::MAX as usize;

    pub fn new() -> Self {
        Self {
            bytes: [0; u16::MAX as usize],
            locked_regions: Vec::new(),
        }
    }

    pub fn load(&mut self, bytes: &[u8]) -> &mut Self {
        if bytes.len() > Self::SIZE {
            todo!("You can only load 64kB into the memory bus");
        }

        for (i, x) in bytes.iter().enumerate() {
            self.bytes[i] = *x;
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

    pub fn lock_region(&mut self, region: Region) -> &mut Self {
        self.locked_regions.push(region);
        return self;
    }
    pub fn unlock_region(&mut self, region: Region) -> &mut Self {
        self.locked_regions.remove(
            self.locked_regions
                .iter()
                .enumerate()
                .find(|(_, x)| **x == region)
                .map(|(i, _)| i)
                .expect("Can not unlock region that's not been locked"),
        );
        return self;
    }
    fn is_addr_locked(&self, addr: Addr) -> bool {
        self.locked_regions
            .iter()
            .find(|x| x.contains(&addr))
            .is_some()
    }
    fn is_region_locked(&self, region: &Region) -> bool {
        self.locked_regions
            .iter()
            .find(|x| x.start() <= region.start() && x.end() >= region.end())
            .is_some()
    }
}

impl ops::Index<Addr> for Interface {
    type Output = u8;

    #[inline]
    fn index(&self, addr: u16) -> &Self::Output {
        if self.is_addr_locked(addr) {
            println!("WARN: Reading locked memory at {addr:#04X}. Returning 0xFF")
        }
        return self.bytes.index(addr as usize);
    }
}
impl ops::IndexMut<Addr> for Interface {
    #[inline]
    fn index_mut(&mut self, addr: u16) -> &mut Self::Output {
        if self.is_addr_locked(addr) {
            unreachable!("ERROR: Writing to locked memory at {addr:#04X} is not allowed.")
        }
        return self.bytes.index_mut(addr as usize);
    }
}
impl ops::Index<Region> for Interface {
    type Output = [u8];

    #[inline]
    fn index(&self, region: Region) -> &Self::Output {
        if self.is_region_locked(&region) {
            unreachable!("ERROR: Reading locked memory at {region:#?}. Returning slice of 0xFF")
        }

        return &self.bytes[(*region.start() as usize)..=(*region.end() as usize)];
    }
}
impl ops::Index<ops::Range<Addr>> for Interface {
    type Output = [u8];

    #[inline]
    fn index(&self, range: ops::Range<Addr>) -> &Self::Output {
        &self[range.start..=(range.end - 1)]
    }
}

#[allow(dead_code)]
pub mod regions {
    use super::super::bus;

    /// Calculates the amount of bytes in the range.
    pub const fn size(range: bus::Region) -> usize {
        (*range.end() + 1 - *range.start()) as usize
    }

    pub mod jump_vectors {
        use super::bus;

        /// However, this memory area (0000-00FF) may be used for any other purpose in case that your program doesn’t use any (or only some) rst instructions or interrupts. rst is a 1-byte instruction that works similarly to the 3-byte call instruction, except that the destination address is restricted. Since it is 1-byte sized, it is also slightly faster.
        pub const AREA: bus::Region = 0x0000..=0x00FF;
        pub const RST_INSTRUCTION: [bus::Addr; 8] = [
            0x0000, 0x0008, 0x0010, 0x0018, 0x0020, 0x0028, 0x0030, 0x0038,
        ];
        pub const INTERRUPTS: [bus::Addr; 5] = [0x0040, 0x0048, 0x0050, 0x0058, 0x0060];
    }

    /// This memory area contains the cartridge header.
    /// It contains information about the program, its entry point, checksums, information about the used MBC chip, the ROM and RAM sizes, etc.
    /// Most of the bytes in this area are required to be specified correctly.
    pub const CARTRIDGE_HEADER: bus::Region = 0x0100..=0x014F;

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

    /// Currently only includes DMG register maps
    ///
    /// See [gbdev.io](https://gbdev.io/pandocs/Hardware_Reg_List.html) for more info
    pub mod io_registers {
        use super::bus;

        /// The full address range of I/O addresses
        const RANGE: bus::Region = 0xFF00..=0xFF7F;

        /// # P1/JOYP (Mixed)
        const JOYPAD: bus::Addr = 0xFF00;
        const SERIAL_TRANSFER: bus::Region = 0xFF01..=0xFF02;
        const TIME_AND_DIVIDER: bus::Region = 0xFF04..=0xFF07;
        const AUDIO: bus::Region = 0xFF10..=0xFF26;
        const WAVE_PATTERN: bus::Region = 0xFF30..=0xFF3F;

        /// LCD Control, Status, Position, Scrolling, and Palettes
        pub mod lcd {
            use crate::hardware::bus;

            const RANGE: bus::Region = 0xFF40..=0xFF4B;

            /// # LCD control
            /// LCDC is the main LCD Control register.
            /// Its bits toggle what elements are displayed on the screen, and how.
            pub const LCDC: bus::Addr = 0xFF40;
            /// # LCD Status
            pub const STAT: bus::Addr = 0xFF41;

            /// # Background viewport Y position
            pub const SCY: bus::Addr = 0xFF42;
            /// # Background viewport X position
            pub const SCX: bus::Addr = 0xFF43;

            /// # LCD Y coordinate [read-only]
            /// LY indicates the current horizontal line, which might be about to be drawn, being drawn, or just been drawn.
            /// LY can hold any value from 0 to 153, with values from 144 to 153 indicating the VBlank period.
            pub const LY: bus::Addr = 0xFF44;
            /// # LYC: LY compare
            /// The Game Boy constantly compares the value of the LYC and LY registers.
            /// When both values are identical, the “LYC=LY” flag in the STAT register is set, and (if enabled) a STAT interrupt is requested.
            pub const LYC: bus::Addr = 0xFF45;

            /// # OAM Direct Memory Access (DMA) transfer source address & start (R/W)
            /// Writing to this register starts a DMA transfer from ROM or RAM to OAM (Object Attribute Memory).
            ///
            /// The written value specifies the transfer source address divided by `$100`, that is, source and destination are:
            /// Source (ROM):                    $XX00-$XX9F   ;XX = $00 to $DF
            /// Destination (OAM region in RAM): $FE00-$FE9F
            pub const DMA: bus::Addr = 0xFF46;

            /// # BG palette data (Non-CGB Mode only)
            /// This register assigns gray shades to the color IDs of the BG and Window tiles.
            ///
            /// Color ID | bits
            /// ---------------
            /// Color 3  | \[7, 6]
            /// Color 2  | \[5, 4]
            /// Color 1  | \[3, 2]
            /// Color 0  | \[1, 0]
            pub const BGP: bus::Addr = 0xFF47;
            /// # OBJ palette 0, 1 data (Non-CGB Mode only)
            /// These registers assigns gray shades to the color indexes of the OBJs that use the corresponding palette.
            /// They work exactly like BGP, except that the lower two bits are ignored because color index 0 is transparent for OBJs.
            pub const OBP: [bus::Addr; 2] = [0xFF48, 0xFF49];

            /// # Window Y position
            pub const WY: bus::Addr = 0xFF4A;
            /// # Window X position plus 7
            pub const WX: bus::Addr = 0xFF4B;
        }

        // Disabled because it first appeared on CGB instead of DMG
        // const VRAM_BANK_SELECT: u16 = 0xFF4F;
        /// Set to non-zero to disable boot ROM
        const DISABLE_BOOT_ROM: bus::Addr = 0xFF50;

        // CGB only registers so I'm not tackling this atm
        // $FF51	$FF55	CGB	VRAM DMA
        // $FF68	$FF6B	CGB	BG / OBJ Palettes
        // $FF70		CGB	WRAM Bank Select
    }

    /// High RAM (HRAM)
    pub const HRAM: bus::Region = 0xFF80..=0xFFFE;
    /// Interrupt Enable register (IE)
    pub const IE: bus::Region = 0xFFFF..=0xFFFF;
}
