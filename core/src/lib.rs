#![allow(dead_code)] // TODO: Dev only
#![feature(generic_arg_infer, stmt_expr_attributes)]
#![allow(non_snake_case)]
// #![feature(inherent_associated_types)] // Cool feature but rust-analyzer doesn't support autocomplete for const values with these types
#![allow(unused, private_interfaces)] // TODO: remove

use utils::KIBI_BYTE;

mod cartridge;
pub use cartridge::Cartridge;

mod cpu;
mod joypad;
mod memory_bus;
mod ppu;

mod prelude;
mod utils;

pub struct GameBoy {
    /// Central Processing Unit
    pub cpu: cpu::CPU,
    pub hardware: Hardware,
}

struct Hardware {
    pub bootrom: Bootrom,

    /// External Cartridge (32KiB)
    pub cartridge: cartridge::Cartridge,

    /// Pixel Processing Unit
    pub ppu: ppu::PPU,

    // Memory
    pub wram: Box<[u8; 8 * KIBI_BYTE]>,

    pub joypad: joypad::Joypad,

    /// High RAM
    pub hram: [u8; memory_bus::regions::size(memory_bus::regions::HRAM)],
}

impl GameBoy {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cpu: cpu::CPU::default(),
            hardware: Hardware {
                hram: [0; _],
                bootrom: Bootrom::default(),
                joypad: joypad::Joypad::default(),
                ppu: ppu::PPU::default(),
                wram: Box::new([0; _]),
                cartridge,
            },
        }
    }

    /// Main execution function of `GameBoy`.
    ///
    /// Emulates one lifetime cycle, which may result in 1 or more m_cycles
    pub fn emulate(&mut self) {
        let has_interupts: bool = false; // TODO
        let ime: bool = false; // TODO

        if has_interupts && ime {
            todo!("Handle interupts");
        }

        let halted: bool = false;
        if halted {
            match has_interupts {
                true => todo!("Handle interupts"),
                false => todo!("Tick and continue halting"),
            }
        }

        self.decode_fetch_exec();
    }
}

/// The rom that gets read on startup.
/// Will be mapped to `0x0000..=0x00FF` and then deactivates itself
struct Bootrom {
    rom: [u8; Self::SIZE],
    is_active: bool,
}

impl Bootrom {
    pub const SIZE: usize = 256;

    pub fn new(rom: [u8; Self::SIZE]) -> Self {
        return Self {
            rom,
            // Active on start-up
            is_active: true,
        };
    }
}

impl Default for Bootrom {
    /// # Bootix Game Boy bootrom
    /// Original file located at ~/roms/gb/bootix.gb
    ///
    /// This is a non-copyrighted fork of the original Nintendo bootrom
    fn default() -> Self {
        return Self::new([
            49, 254, 255, 33, 255, 159, 175, 50, 203, 124, 32, 250, 14, 17, 33, 38, 255, 62, 128,
            50, 226, 12, 62, 243, 50, 226, 12, 62, 119, 50, 226, 17, 4, 1, 33, 16, 128, 26, 205,
            184, 0, 26, 203, 55, 205, 184, 0, 19, 123, 254, 52, 32, 240, 17, 204, 0, 6, 8, 26, 19,
            34, 35, 5, 32, 249, 33, 4, 153, 1, 12, 1, 205, 177, 0, 62, 25, 119, 33, 36, 153, 14,
            12, 205, 177, 0, 62, 145, 224, 64, 6, 16, 17, 212, 0, 120, 224, 67, 5, 123, 254, 216,
            40, 4, 26, 224, 71, 19, 14, 28, 205, 167, 0, 175, 144, 224, 67, 5, 14, 28, 205, 167, 0,
            175, 176, 32, 224, 224, 67, 62, 131, 205, 159, 0, 14, 39, 205, 167, 0, 62, 193, 205,
            159, 0, 17, 138, 1, 240, 68, 254, 144, 32, 250, 27, 122, 179, 32, 245, 24, 73, 14, 19,
            226, 12, 62, 135, 226, 201, 240, 68, 254, 144, 32, 250, 13, 32, 247, 201, 120, 34, 4,
            13, 32, 250, 201, 71, 14, 4, 175, 197, 203, 16, 23, 193, 203, 16, 23, 13, 32, 245, 34,
            35, 34, 35, 201, 60, 66, 185, 165, 185, 165, 66, 60, 0, 84, 168, 252, 66, 79, 79, 84,
            73, 88, 46, 68, 77, 71, 32, 118, 49, 46, 50, 0, 62, 255, 198, 1, 11, 30, 216, 33, 77,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 1, 224, 80,
        ]);
    }
}
