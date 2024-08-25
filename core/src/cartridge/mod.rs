use crate::memory_bus::{Addr, MemoryMappedRegion};

mod header;

pub struct Cartridge {
    pub rom: Box<[u8]>,
    pub mbc: MBC,
}

impl Cartridge {}

/// Memory Bank Controller
///
/// See [gbdev.io](https://gbdev.io/pandocs/MBCs.html#mbcs)
enum MBC {}

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

impl MemoryMappedRegion for Cartridge {
    const START_ADDR: Addr = 0x0000;
    const END_ADDR: Addr = 0x7FFF;

    fn bus_read(&self, bus_addr: Addr) -> u8 {
        return match bus_addr {
            /// Bank 00
            0x0000..=0x3FFF => self.rom[Self::bus_addr_to_own_addr(bus_addr) as usize],
            /// Bank 01~NN
            0x4000..=0x7FFF => todo!("MBC mapping hasn't been implemented"),
            _ => unreachable!(),
        };
    }

    fn bus_write(&mut self, bus_addr: Addr, byte: u8) -> &mut Self {
        match bus_addr {
            /// Bank 00
            0x0000..=0x3FFF => self.rom[Self::bus_addr_to_own_addr(bus_addr) as usize] = byte,
            /// Bank 01~NN
            0x4000..=0x7FFF => todo!("MBC mapping hasn't been implemented"),
            _ => unreachable!(),
        };
        return self;
    }
}
