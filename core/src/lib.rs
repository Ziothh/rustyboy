#![allow(dead_code)] // TODO: Dev only
#![feature(generic_arg_infer)]
// #![feature(inherent_associated_types)] // Cool feature but rust-analyzer doesn't support
// autocomplete for const values with these types

pub mod cartridge;
pub mod cpu;
pub mod ppu;

pub mod prelude;
pub mod utils;

pub struct GameBoy {
    /// Central Processing Unit
    cpu: cpu::CPU,
    /// Pixel Processing Unit
    ppu: ppu::PPU,

    /// External Cartridge
    cartridge: cartridge::Cartridge,
}

impl GameBoy {
    pub fn new(rom: &[u8]) {
        todo!()
    }
}
