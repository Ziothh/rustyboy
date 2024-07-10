
mod header;

#[derive(Default)]
pub struct Cartridge { 
    rom: Box<[u8]>
}

impl Cartridge {
    
}



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
