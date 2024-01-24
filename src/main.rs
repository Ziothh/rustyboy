use std::{fs, path};

use gb::hardware;

fn main() {
    // let rom = ROM::try_new("./roms/gb/Super-Mario-Land-4.gb").expect("Failed to find rom");
    let rom = ROM::try_new("./roms/gb/test.gb")
        .expect("Failed to find rom")
        .read()
        .unwrap();

    let mut bus = hardware::bus::Interface::new();
    bus.load(&rom[0..rom.len().min(u16::MAX as usize)]);

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

    #[allow(unused_must_use)]
    {
        dbg!(title);
    }
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
