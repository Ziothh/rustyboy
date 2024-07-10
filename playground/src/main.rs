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
    fn bus(&mut self) -> Bus {
        Bus {
            cartridge: self.cardridge.as_mut(),
            wram: &mut self.wram,
        }
    }

    /// Machine clock tick (4 clock ticks)
    fn m_clock_tick(&mut self) {
        let mut bus = Bus {
            wram: &mut self.wram,
            cartridge: self.cardridge.as_mut(),
        };

        self.cpu.run_exec_fetch(&mut bus);
    }
}

/// SoC
mod dmg {
    use super::*;

    /// Sharp SM83 CPU Core
    /// Closely resembles the Zilog Z80 and Intel 8080 but is neither.
    #[derive(Default)]
    pub struct CPU {
        pc: u16,
        HRAM: Memory<127>,
    }
    impl CPU {
        pub fn run_exec_fetch(&mut self, bus: &mut Bus) {
            let opcode = bus.read(self.pc);
            // Do something with opcode
            bus.write(0x0420 /* Some addr */, 0x69 /* Some value */);

            self.pc += 1;
        }
    }

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
}

struct Bus<'a> {
    cartridge: Option<&'a mut Cartridge>,
    wram: &'a mut Memory<{ 8 * Memory::KIBI_BYTE }>,
}
impl Bus<'_> {
    fn read(&self, addr: Memory::Addr) -> u8 {
        match addr {
            0 => self.cartridge.as_ref().unwrap().rom.buffer[0],
            1 => self.cartridge.as_ref().unwrap().ram.buffer[1],
            2 => self.wram.buffer[2],
            _ => todo!(),
        }
    }

    fn write(&mut self, addr: Memory::Addr, byte: u8) {
        let ptr = match addr {
            0 => &mut self.cartridge.as_mut().unwrap().rom.buffer[0],
            1 => &mut self.cartridge.as_mut().unwrap().ram.buffer[1],
            2 => &mut self.wram.buffer[2],
            _ => todo!(),
        };

        *ptr = byte;
    }
}

// impl<const SIZE: usize> Memory<SIZE> {
// }

impl Memory {
    pub const KIBI_BYTE: usize = 1024;
}

impl<const SIZE: usize> Default for Memory<SIZE> {
    fn default() -> Self {
        Self { buffer: [0; SIZE] }
    }
}
