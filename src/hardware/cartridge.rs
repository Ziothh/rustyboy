use std::ops;

use super::bus;

pub struct CartridgeHeader<'a>(&'a [u8]);

impl<'a> CartridgeHeader<'a> {
    pub fn read_from_bus(memory_bus: &'a bus::MemoryBus) -> Self {
        let bytes = &memory_bus[bus::regions::CARTRIDGE_HEADER];

        // let logo =

        return Self(bytes);
    }

    pub fn title(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(
            self.relative_range_inclusive(regions::TITLE), // &bus[regions::TITLE]
        )
        .and_then(|x| Ok(x.trim_end_matches(char::from(0))))
    }

    fn relative_range_inclusive(&self, range: ops::RangeInclusive<u16>) -> &[u8] {
        const HEADER_START: u16 = 256;
        let start = range.start() - HEADER_START;
        let end = range.end() - HEADER_START;

        return &self.0[start as usize..=end as usize];
    }
}

#[allow(dead_code)]
pub mod regions {
    use std::ops;

    /// After displaying the Nintendo logo, the built-in boot ROM jumps to the address $0100, which should then jump to the actual main program in the cartridge. Most commercial games fill this 4-byte area with a nop instruction followed by a jp $0150.
    pub const ENTRY_POINT: ops::RangeInclusive<u16> = 0x0100..=0x0103;

    /// This area contains a bitmap image that is displayed when the Game Boy is powered on. It must match the following (hexadecimal) dump, otherwise the boot ROM won’t allow the game to run:
    /// ```text
    /// CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D
    /// 00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99
    /// BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E
    /// ```
    /// The way the pixels are encoded is as follows: ([more visual aid](https://github.com/ISSOtm/gb-bootroms/blob/2dce25910043ce2ad1d1d3691436f2c7aabbda00/src/dmg.asm#L259-L269))
    ///
    /// - The bytes `$0104`—`$011B` encode the top half of the logo while the bytes `$011C`–`$0133` encode the bottom half.
    /// - For each half, each nibble encodes 4 pixels (the MSB corresponds to the leftmost pixel, the LSB to the rightmost); a pixel is lit if the corresponding bit is set.
    /// - The 4-pixel "groups" are laid out top to bottom, left to right.
    /// - Finally, the monochrome models upscale the entire thing by a factor of 2 (leading to somewhat chunky pixels).
    ///
    /// The Game Boy's boot procedure [first displays the logo and then checks](<#Bypass>) that it matches the dump above.
    /// If it doesn't, the boot ROM **locks itself up**.
    ///
    /// The CGB and later models [only check the top half of the logo](Power_Up_Sequence.html?highlight=half#behavior) (the first `$18` bytes).
    pub const NINTENDO_LOGO: ops::RangeInclusive<u16> = 0x0104..=0x0133;

    /// These bytes contain the title of the game in upper case ASCII.
    /// If the title is less than 16 characters long, the remaining bytes should be padded with `$00`s.
    ///
    /// Parts of this area actually have a different meaning on later cartridges, reducing the actual title size to 15 (`$0134`–`$0142`) or 11 (`$0134`–`$013E`) characters; see `Self::MANUFACTURER_CODE`.
    pub const TITLE: ops::RangeInclusive<u16> = 0x0134..=0x0143;

    /// In older cartridges these bytes were part of the Title (see above).
    /// In newer cartridges they contain a 4-character manufacturer code (in uppercase ASCII).
    /// The purpose of the manufacturer code is unknown.
    pub const MANUFACTURER_CODE: ops::RangeInclusive<u16> = 0x013F..=0x0142;

    /// This area contains a two-character ASCII “licensee code” indicating the game’s publisher. It is only meaningful if the Old licensee is exactly `$33` (which is the case for essentially all games made after the SGB was released); otherwise, the old code must be considered.
    ///
    /// See [gbdev.io](https://gbdev.io/pandocs/The_Cartridge_Header.html#01440145--new-licensee-code) for sample licensee codes.
    pub const NEW_LICENSEE_CODE: ops::RangeInclusive<u16> = 0x0144..=0x0145;

    /// In older cartridges this byte was part of the Title (see above).
    /// The CGB and later models interpret this byte to decide whether to enable Color mode ("CGB Mode") or to fall back to monochrome compatibility mode ("Non-CGB Mode").
    ///
    /// Typical values are:
    ///
    /// Value | Meaning
    /// ------|----------------------------------------------------------------------------------------------------
    /// `$80` | The game supports CGB enhancements, but is backwards compatible with monochrome Game Boys
    /// `$C0` | The game works on CGB only (the hardware ignores bit 6, so this really functions the same as `$80`)
    ///
    /// Values with bit 7 and either bit 2 or 3 set will switch the Game Boy into a special non-CGB-mode called "PGB mode".
    pub const CGB_FLAGS: u16 = 0x146;

    /// This byte indicates what kind of hardware is present on the cartridge — most notably its [mapper](https://gbdev.io/pandocs/MBCs.html#mbcs).
    ///
    /// See [gbdev.io](https://gbdev.io/pandocs/The_Cartridge_Header.html#0147--cartridge-type) for
    /// more info.
    pub const CARTRIDGE_TYPE: u16 = 0x0147;

    /// This byte indicates how much ROM is present on the cartridge. In most cases, the ROM size is given by 32 KiB × (1 << <value>):
    ///
    /// See [gbdev.io](https://gbdev.io/pandocs/The_Cartridge_Header.html#0148--rom-size) for rom
    /// size values.
    pub const ROM_SIZE: u16 = 0x0148;

    /// This byte indicates how much RAM is present on the cartridge, if any.
    ///
    /// If the cartridge type does not include “RAM” in its name, this should be set to 0. This includes MBC2, since its 512 × 4 bits of memory are built directly into the mapper.
    pub const RAM_SIZE: u16 = 0x0148;

    /// This byte specifies whether this version of the game is intended to be sold in Japan (`0x00`) or elsewhere (`0x01`).
    pub const DESTINATION_CODE: u16 = 0x014A;

    /// This byte is used in older (pre-SGB) cartridges to specify the game’s publisher. However, the value `$33` indicates that the New licensee codes must be considered instead. (The SGB will ignore any command packets unless this value is `$33`.)
    ///
    /// See [gbdev.io](https://gbdev.io/pandocs/The_Cartridge_Header.html#014b--old-licensee-code)
    /// for table of licensee codes.
    pub const OLD_LICENSEE_CODE: u16 = 0x014B;

    /// This byte specifies the version number of the game. It is usually `$00`.
    pub const ROM_VERSION_NUMBER: u16 = 0x014C;

    /// This byte contains an 8-bit checksum computed from the cartridge header bytes $0134–014C. The boot ROM computes the checksum as follows:
    /// ```ignore
    /// uint8_t checksum = 0;
    /// for (uint16_t address = 0x0134; address <= 0x014C; address++) {
    ///   checksum = checksum - rom[address] - 1;
    /// }
    /// ```
    /// The boot ROM verifies this checksum. If the byte at `$014D` does not match the lower 8 bits of checksum, the boot ROM will lock up and the program in the cartridge **won’t run**.
    pub const HEADER_CHECKSUM: u16 = 0x014D;

    /// These bytes contain a 16-bit (big-endian) checksum simply computed as the sum of all the bytes of the cartridge ROM (except these two checksum bytes).
    ///
    /// This checksum is not verified, except by Pokémon Stadium’s “GB Tower” emulator (presumably to detect Transfer Pak errors).
    pub const GLOBAL_CHECKSUM: ops::RangeInclusive<u16> = 0x014E..=0x014F;
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
