#![allow(dead_code, unused)]

use gb;

mod roms {
    pub const SUPER_MARIO_LAND_4: &'static str = "./roms/gb/Super-Mario-Land-4.gb";
    pub const POKEMON_RED: &'static str = "./roms/gb/Pokemon-Red.gb";
}

fn main() {
    let cartridge = gb::Cartridge::try_from_file(roms::POKEMON_RED).unwrap();
    {
        dbg!(cartridge.title());
        dbg!(cartridge.licensee());
        dbg!(cartridge.validate_checksum());
    }

    let mut gb = gb::GameBoy::new(cartridge);

    let fps = 1;

    let mut cycles = 0;
    loop {
        println!("Cycle {cycles}");

        gb.emulate();

        cycles += 1;

        // std::thread::sleep(std::time::Duration::from_millis(1000 / fps));
    }

    // let rom = ROM::try_new("./roms/gb/Super-Mario-Land-4.gb").expect("Failed to find rom");
    // let rom = ROM::try_new("./roms/gb/Pokemon-Red.gb").expect("Failed to find rom");

    // let mut bus = hardware::bus::Interface::new();
    // bus.load(&rom.read().unwrap()[0..u16::MAX as usize]);

    // let program = program::Program::new(bus);
    //
    // for instruction in program {
    //     println!("{instruction:#X?}");
    //
    //     std::io::stdin().read(&mut [0u8]).unwrap();
    // }

    // let title = std::str::from_utf8(&bus[hardware::cartridge::regions::TITLE])
    //     .expect("Title is not valid ascii")
    //     .trim_end_matches(char::from(0));

    // let header = hardware::cartridge::CartridgeHeader::read_from_bus(&bus);
    // let title = header.title();
}

// #[derive(Debug)]
// struct ROM {
//     path: path::PathBuf,
// }
//
// impl ROM {
//     pub fn try_new<P>(location: P) -> Result<Self, std::io::Error>
//     where
//         P: AsRef<std::ffi::OsStr>,
//     {
//         return Ok(Self {
//             path: path::Path::new(&location).canonicalize()?,
//         });
//     }
//
//     pub fn read(&self) -> Result<Vec<u8>, std::io::Error> {
//         fs::read(&self.path)
//     }
// }
//
// impl std::fmt::Display for ROM {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}:",
//             self.path
//                 .file_name()
//                 .and_then(|name| name.to_str())
//                 .unwrap_or("NO FILE NAME")
//         )?;
//         write!(f, "\n> {} kB:", self.path.metadata().unwrap().len() / 1024)?;
//         write!(f, "\n> {} bytes", self.path.metadata().unwrap().len())
//     }
// }
