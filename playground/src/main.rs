#![allow(dead_code, unused_imports, unused_variables)]
#![allow(non_snake_case)]
#![allow(incomplete_features)]
#![feature(inherent_associated_types)]

use std::fs;

fn main() {
    let bytes = fs::read("./roms/gb/bootix.bin").unwrap();

    dbg!(bytes.len());

    dbg!(bytes);

    // let mut gb = GameBoy {
    //     cardridge: Some(Default::default()),
    //     ..Default::default()
    // };
    //
    // let bus = gb.bus();
    // let x = bus.read(0);
    //
    // let mut is_running = true;
    // while is_running {
    //     gb.m_clock_tick();
    //
    //     is_running = false;
    // }
}

#[derive(Default)]
struct GameBoy {
    // DMG-01
    cpu: dmg::CPU,
    ppu: dmg::PPU,

    // Memory
    wram: Memory<{ 8 * Memory::KIBI_BYTE }>,
    vram: Memory<{ 8 * Memory::KIBI_BYTE }>,

    // Cardridge
    cardridge: Option<Cartridge>,
    // pub bootrom: Bootrom,
    // pub cartridge: Cartridge,
    // work_ram: WorkRam,
    // hiram: HiramData,
    // ppu: Ppu,
    // apu: Apu,
    // joypad: Joypad,
    // serial: Serial,
    // pub timer: Timer,
    // oam_dma: OamDma,
}
impl GameBoy {
    /// Machine clock tick (4 clock ticks)
    fn m_clock_tick(&mut self) {
        self.run_exec_fetch();
    }

    pub fn run_exec_fetch(&mut self) {
        let opcode = self.bus_read(self.cpu.pc);
        // Do something with opcode
        // bus.write(0x0420 /* Some addr */, 0x69 /* Some value */);

        self.cpu.pc += 1;
    }

    fn bus_read(&self, addr: u16) -> u8 {
        todo!();
    }
}

/// SoC
mod dmg {
    use super::*;

    /// Sharp SM83 CPU Core
    /// Closely resembles the Zilog Z80 and Intel 8080 but is neither.
    #[derive(Default)]
    pub struct CPU {
        pub pc: u16,
        pub HRAM: Memory<127>,
    }
    impl CPU {}

    #[derive(Default)]
    pub struct PPU {
        pub OAM: Memory<160>,
    }
}

#[derive(Default)]
struct Cartridge {
    pub rom: Memory<{ 32 * Memory::KIBI_BYTE }>,
    // External RAM
    pub ram: Memory<{ 8 * Memory::KIBI_BYTE }>,
}

struct Memory<const SIZE: usize = 0> {
    buffer: [u8; SIZE],
}
impl Memory {
    type Addr = u16;
    pub const KIBI_BYTE: usize = 1024;
}
impl<const SIZE: usize> Default for Memory<SIZE> {
    fn default() -> Self {
        Self { buffer: [0; SIZE] }
    }
}

trait BusItem {
    const BEGIN_ADDR: u16;
    const END_ADDR: u16;

    fn bus_addr_to_own_addr(&self, bus_addr: u16) -> u16 {
        if (Self::BEGIN_ADDR..=Self::END_ADDR).contains(&bus_addr) {
            return bus_addr - Self::BEGIN_ADDR;
        }

        unreachable!()
    }
}
