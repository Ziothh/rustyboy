use std::{fs, mem};

use crate::memory_bus::{self, regions, Addr, MemoryMappedRegion};

mod header;

pub struct Cartridge {
    pub rom: Box<[u8]>,
    pub mbc: MBC,
}

impl Cartridge {
    pub fn try_from_file<P: AsRef<std::path::Path>>(file_path: P) -> std::io::Result<Self> {
        return Ok(Self {
            rom: Box::from(fs::read(file_path.as_ref())?),
            mbc: MBC::None, // TODO: parse mbc from ROM
        });
    }
}

// Cartridge Header functionality
impl Cartridge {
    pub fn title(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.read_region(header::regions::TITLE))
            .and_then(|x| Ok(x.trim_end_matches(char::from(0))))
    }

    pub fn licensee(&self) -> Option<&'static str> {
        let code = self.read_addr(header::regions::OLD_LICENSEE_CODE);

        return match code {
            0x33 => header::licensee::get_new(
                self.read_region(header::regions::NEW_LICENSEE_CODE)
                    .try_into()
                    .expect("New licensee code should be 2 bytes long"),
            ),
            code => header::licensee::get_old(code),
        };
    }

    /// Calculates the header checksum and validates it with the the checksum byte at `$014D`.
    pub fn validate_checksum(&self) -> bool {
        self.read_region(0x0134..=0x014C)
            .iter()
            .fold(0u8, |checksum, byte| {
                checksum.wrapping_sub(*byte).wrapping_sub(1)
            })
            .eq(&self.read_addr(header::regions::HEADER_CHECKSUM))
    }

    fn read_addr(&self, addr: memory_bus::Addr) -> u8 {
        return self.rom[addr as usize];
    }
    fn read_region(&self, range: memory_bus::Region) -> &[u8] {
        return &self.rom[*range.start() as usize..=*range.end() as usize];
    }

    pub fn bus_read_0x0000_0x3FFF(&self, bus_addr: Addr) -> u8 {
        assert!(memory_bus::regions::rom::BANK_00.contains(&bus_addr));

        return self.rom[bus_addr as usize];
    }
    pub fn bus_read_0x4000_0x7FFF(&self, bus_addr: Addr) -> u8 {
        assert!(memory_bus::regions::rom::BANK_NN.contains(&bus_addr));

        todo!("MBC mapping hasn't been implemented");
    }

    pub fn bus_write_0x0000_0x3FFF(&mut self, bus_addr: Addr, byte: u8) {
        assert!(memory_bus::regions::rom::BANK_00.contains(&bus_addr));

        self.rom[bus_addr as usize] = byte;
    }
    pub fn bus_write_0x4000_0x7FFF(&mut self, bus_addr: Addr, byte: u8) {
        assert!(memory_bus::regions::rom::BANK_NN.contains(&bus_addr));

        todo!("MBC mapping hasn't been implemented");
    }
}

/// Memory Bank Controller
///
/// See [gbdev.io](https://gbdev.io/pandocs/MBCs.html#mbcs)
enum MBC {
    None,
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
