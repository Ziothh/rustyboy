use std::{fs, io::prelude::*, path};

mod cpu;
mod hardware;
mod prelude;
mod program;

fn main() {
    // let rom = ROM::try_new("./roms/gb/Super-Mario-Land-4.gb").expect("Failed to find rom");
    let rom = ROM::try_new("./roms/gb/Pokemon-Red.gb").expect("Failed to find rom");

    let mut bus = hardware::bus::MemoryBus::new();
    bus.load(&rom.read().unwrap()[0..u16::MAX as usize]);

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

    let header = hardware::cartridge::CartridgeHeader::read_from_bus(&bus);
    let title = header.title();

    dbg!(title);
    dbg!(&header.licensee());
    dbg!(&header.validate_checksum());
}

#[derive(Debug)]
struct ROM {
    path: path::PathBuf,
}

impl ROM {
    pub fn try_new<P>(location: P) -> Result<Self, std::io::Error>
    where
        P: AsRef<std::ffi::OsStr>,
    {
        return Ok(Self {
            path: path::Path::new(&location).canonicalize()?,
        });
    }

    pub fn read(&self) -> Result<Vec<u8>, std::io::Error> {
        fs::read(&self.path)
    }
}

impl std::fmt::Display for ROM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:",
            self.path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("NO FILE NAME")
        )?;
        write!(f, "\n> {} kB:", self.path.metadata().unwrap().len() / 1024)?;
        write!(f, "\n> {} bytes", self.path.metadata().unwrap().len())
    }
}
