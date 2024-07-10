use std::str;
use crate::memory_bus as bus;

pub struct CartridgeHeader<'a>(&'a [u8]);

impl CartridgeHeader<'_> {
    pub fn read_from_bus(memory_bus: &bus::Bus<'_>) -> Self {
        todo!()
        // Self(&memory_bus[bus::regions::rom::CARTRIDGE_HEADER])
    }

    pub fn title(&self) -> Result<&str, str::Utf8Error> {
        std::str::from_utf8(
            self.read_relative_region(regions::TITLE), // &bus[regions::TITLE]
        )
        .and_then(|x| Ok(x.trim_end_matches(char::from(0))))
    }

    pub fn licensee(&self) -> Option<&'static str> {
        let code = self.0[Self::relative_address(regions::OLD_LICENSEE_CODE)];

        return match code {
            0x33 => licensee::get_new(
                self.read_relative_region(regions::NEW_LICENSEE_CODE)
                    .try_into()
                    .expect("New licensee code should be 2 bytes long"),
            ),
            code => licensee::get_old(code),
        };
    }

    // /// This doens't work as intended atm
    // pub fn manufacturer_code(&self) -> Result<&str, std::str::Utf8Error> {
    //     std::str::from_utf8(self.relative_range_inclusive(regions::MANUFACTURER_CODE))
    // }

    /// Calculates the header checksum and validates it with the the checksum byte at `$014D`.
    pub fn validate_checksum(&self) -> bool {
        self.read_relative_region(0x0134..=0x014C)
            .iter()
            .fold(0u8, |checksum, byte| {
                checksum.wrapping_sub(*byte).wrapping_sub(1)
            })
            .eq(&self.read_relative_addr(regions::HEADER_CHECKSUM))
    }

    /// Calculates the `address` relative to the start of the cartridge header
    /// and returns it as a `usize`
    fn relative_address(address: bus::Addr) -> usize {
        (address - regions::START) as usize
    }
    fn read_relative_addr(&self, address: bus::Addr) -> u8 {
        self.0[Self::relative_address(address)]
    }
    fn read_relative_region(&self, range: bus::Region) -> &[u8] {
        &self.0[Self::relative_address(*range.start())..=Self::relative_address(*range.end())]
    }
}

mod licensee {
    pub fn get_old(code: u8) -> Option<&'static str> {
        match code {
            0x00 => Some("None"),
            0x01 => Some("Nintendo"),
            0x08 => Some("Capcom"),
            0x09 => Some("Hot-B"),
            0x0A => Some("Jaleco"),
            0x0B => Some("Coconuts Japan"),
            0x0C => Some("Elite Systems"),
            0x13 => Some("EA (Electronic Arts)"),
            0x18 => Some("Hudsonsoft"),
            0x19 => Some("ITC Entertainment"),
            0x1A => Some("Yanoman"),
            0x1D => Some("Japan Clary"),
            0x1F => Some("Virgin Interactive"),
            0x24 => Some("PCM Complete"),
            0x25 => Some("San-X"),
            0x28 => Some("Kotobuki Systems"),
            0x29 => Some("Seta"),
            0x30 => Some("Infogrames"),
            0x31 => Some("Nintendo"),
            0x32 => Some("Bandai"),
            0x33 => unreachable!("USE NEW_LICENSEE_CODE TABLE INSTEAD"),
            0x34 => Some("Konami"),
            0x35 => Some("HectorSoft"),
            0x38 => Some("Capcom"),
            0x39 => Some("Banpresto"),
            0x3C => Some(".Entertainment i"),
            0x3E => Some("Gremlin"),
            0x41 => Some("Ubisoft"),
            0x42 => Some("Atlus"),
            0x44 => Some("Malibu"),
            0x46 => Some("Angel"),
            0x47 => Some("Spectrum Holoby"),
            0x49 => Some("Irem"),
            0x4A => Some("Virgin Interactive"),
            0x4D => Some("Malibu"),
            0x4F => Some("U.S. Gold"),
            0x50 => Some("Absolute"),
            0x51 => Some("Acclaim"),
            0x52 => Some("Activision"),
            0x53 => Some("American Sammy"),
            0x54 => Some("GameTek"),
            0x55 => Some("Park Place"),
            0x56 => Some("LJN"),
            0x57 => Some("Matchbox"),
            0x59 => Some("Milton Bradley"),
            0x5A => Some("Mindscape"),
            0x5B => Some("Romstar"),
            0x5C => Some("Naxat Soft"),
            0x5D => Some("Tradewest"),
            0x60 => Some("Titus"),
            0x61 => Some("Virgin Interactive"),
            0x67 => Some("Ocean Interactive"),
            0x69 => Some("EA (Electronic Arts)"),
            0x6E => Some("Elite Systems"),
            0x6F => Some("Electro Brain"),
            0x70 => Some("Infogrames"),
            0x71 => Some("Interplay"),
            0x72 => Some("Broderbund"),
            0x73 => Some("Sculptered Soft"),
            0x75 => Some("The Sales Curve"),
            0x78 => Some("t.hq"),
            0x79 => Some("Accolade"),
            0x7A => Some("Triffix Entertainment"),
            0x7C => Some("Microprose"),
            0x7F => Some("Kemco"),
            0x80 => Some("Misawa Entertainment"),
            0x83 => Some("Lozc"),
            0x86 => Some("Tokuma Shoten Intermedia"),
            0x8B => Some("Bullet-Proof Software"),
            0x8C => Some("Vic Tokai"),
            0x8E => Some("Ape"),
            0x8F => Some("I'Max"),
            0x91 => Some("Chunsoft Co."),
            0x92 => Some("Video System"),
            0x93 => Some("Tsubaraya Productions Co."),
            0x95 => Some("Varie Corporation"),
            0x96 => Some("Yonezawa/S'Pal"),
            0x97 => Some("Kaneko"),
            0x99 => Some("Arc"),
            0x9A => Some("Nihon Bussan"),
            0x9B => Some("Tecmo"),
            0x9C => Some("Imagineer"),
            0x9D => Some("Banpresto"),
            0x9F => Some("Nova"),
            0xA1 => Some("Hori Electric"),
            0xA2 => Some("Bandai"),
            0xA4 => Some("Konami"),
            0xA6 => Some("Kawada"),
            0xA7 => Some("Takara"),
            0xA9 => Some("Technos Japan"),
            0xAA => Some("Broderbund"),
            0xAC => Some("Toei Animation"),
            0xAD => Some("Toho"),
            0xAF => Some("Namco"),
            0xB0 => Some("acclaim"),
            0xB1 => Some("ASCII or Nexsoft"),
            0xB2 => Some("Bandai"),
            0xB4 => Some("Square Enix"),
            0xB6 => Some("HAL Laboratory"),
            0xB7 => Some("SNK"),
            0xB9 => Some("Pony Canyon"),
            0xBA => Some("Culture Brain"),
            0xBB => Some("Sunsoft"),
            0xBD => Some("Sony Imagesoft"),
            0xBF => Some("Sammy"),
            0xC0 => Some("Taito"),
            0xC2 => Some("Kemco"),
            0xC3 => Some("Squaresoft"),
            0xC4 => Some("Tokuma Shoten Intermedia"),
            0xC5 => Some("Data East"),
            0xC6 => Some("Tonkinhouse"),
            0xC8 => Some("Koei"),
            0xC9 => Some("UFL"),
            0xCA => Some("Ultra"),
            0xCB => Some("Vap"),
            0xCC => Some("Use Corporation"),
            0xCD => Some("Meldac"),
            0xCE => Some(".Pony Canyon or"),
            0xCF => Some("Angel"),
            0xD0 => Some("Taito"),
            0xD1 => Some("Sofel"),
            0xD2 => Some("Quest"),
            0xD3 => Some("Sigma Enterprises"),
            0xD4 => Some("ASK Kodansha Co."),
            0xD6 => Some("Naxat Soft"),
            0xD7 => Some("Copya System"),
            0xD9 => Some("Banpresto"),
            0xDA => Some("Tomy"),
            0xDB => Some("LJN"),
            0xDD => Some("NCS"),
            0xDE => Some("Human"),
            0xDF => Some("Altron"),
            0xE0 => Some("Jaleco"),
            0xE1 => Some("Towa Chiki"),
            0xE2 => Some("Yutaka"),
            0xE3 => Some("Varie"),
            0xE5 => Some("Epcoh"),
            0xE7 => Some("Athena"),
            0xE8 => Some("Asmik ACE Entertainment"),
            0xE9 => Some("Natsume"),
            0xEA => Some("King Records"),
            0xEB => Some("Atlus"),
            0xEC => Some("Epic/Sony Records"),
            0xEE => Some("IGS"),
            0xF0 => Some("A Wave"),
            0xF3 => Some("Extreme Entertainment"),
            0xFF => Some("LJN"),
            _ => None,
        }
    }

    pub fn get_new(code: [u8; 2]) -> Option<&'static str> {
        match code {
            [b'0', b'0'] => Some("None"),
            [b'0', b'1'] => Some("Nintendo R&D1"),
            [b'0', b'8'] => Some("Capcom"),
            [b'1', b'3'] => Some("Electronic Arts"),
            [b'1', b'8'] => Some("Hudson Soft"),
            [b'1', b'9'] => Some("b-ai"),
            [b'2', b'0'] => Some("kss"),
            [b'2', b'2'] => Some("pow"),
            [b'2', b'4'] => Some("PCM Complete"),
            [b'2', b'5'] => Some("san-x"),
            [b'2', b'8'] => Some("Kemco Japan"),
            [b'2', b'9'] => Some("seta"),
            [b'3', b'0'] => Some("Viacom"),
            [b'3', b'1'] => Some("Nintendo"),
            [b'3', b'2'] => Some("Bandai"),
            [b'3', b'3'] => Some("Ocean/Acclaim"),
            [b'3', b'4'] => Some("Konami"),
            [b'3', b'5'] => Some("Hector"),
            [b'3', b'7'] => Some("Taito"),
            [b'3', b'8'] => Some("Hudson"),
            [b'3', b'9'] => Some("Banpresto"),
            [b'4', b'1'] => Some("Ubi Soft"),
            [b'4', b'2'] => Some("Atlus"),
            [b'4', b'4'] => Some("Malibu"),
            [b'4', b'6'] => Some("angel"),
            [b'4', b'7'] => Some("Bullet-Proof"),
            [b'4', b'9'] => Some("irem"),
            [b'5', b'0'] => Some("Absolute"),
            [b'5', b'1'] => Some("Acclaim"),
            [b'5', b'2'] => Some("Activision"),
            [b'5', b'3'] => Some("American sammy"),
            [b'5', b'4'] => Some("Konami"),
            [b'5', b'5'] => Some("Hi tech entertainment"),
            [b'5', b'6'] => Some("LJN"),
            [b'5', b'7'] => Some("Matchbox"),
            [b'5', b'8'] => Some("Mattel"),
            [b'5', b'9'] => Some("Milton Bradley"),
            [b'6', b'0'] => Some("Titus"),
            [b'6', b'1'] => Some("Virgin"),
            [b'6', b'4'] => Some("LucasArts"),
            [b'6', b'7'] => Some("Ocean"),
            [b'6', b'9'] => Some("Electronic Arts"),
            [b'7', b'0'] => Some("Infogrames"),
            [b'7', b'1'] => Some("Interplay"),
            [b'7', b'2'] => Some("Broderbund"),
            [b'7', b'3'] => Some("sculptured"),
            [b'7', b'5'] => Some("sci"),
            [b'7', b'8'] => Some("THQ"),
            [b'7', b'9'] => Some("Accolade"),
            [b'8', b'0'] => Some("misawa"),
            [b'8', b'3'] => Some("lozc"),
            [b'8', b'6'] => Some("Tokuma Shoten Intermedia"),
            [b'8', b'7'] => Some("Tsukuda Original"),
            [b'9', b'1'] => Some("Chunsoft"),
            [b'9', b'2'] => Some("Video system"),
            [b'9', b'3'] => Some("Ocean/Acclaim"),
            [b'9', b'5'] => Some("Varie"),
            [b'9', b'6'] => Some("Yonezawa/s'pal"),
            [b'9', b'7'] => Some("Kaneko"),
            [b'9', b'9'] => Some("Pack in soft"),
            [b'9', b'H'] => Some("Bottom Up"),
            [b'A', b'4'] => Some("Konami (Yu-Gi-Oh!)"),
            _ => None,
        }
    }
}

#[allow(dead_code)]
pub mod regions {
    use super::bus;

    pub const START: bus::Addr = *bus::regions::rom::CARTRIDGE_HEADER.start();

    /// After displaying the Nintendo logo, the built-in boot ROM jumps to the address $0100, which should then jump to the actual main program in the cartridge. Most commercial games fill this 4-byte area with a nop instruction followed by a jp $0150.
    pub const ENTRY_POINT: bus::Region = 0x0100..=0x0103;

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
    pub const NINTENDO_LOGO: bus::Region = 0x0104..=0x0133;

    /// These bytes contain the title of the game in upper case ASCII.
    /// If the title is less than 16 characters long, the remaining bytes should be padded with `$00`s.
    ///
    /// Parts of this area actually have a different meaning on later cartridges, reducing the actual title size to 15 (`$0134`–`$0142`) or 11 (`$0134`–`$013E`) characters; see `Self::MANUFACTURER_CODE`.
    pub const TITLE: bus::Region = 0x0134..=0x0143;

    /// In older cartridges these bytes were part of the Title (see above).
    /// In newer cartridges they contain a 4-character manufacturer code (in uppercase ASCII).
    /// The purpose of the manufacturer code is unknown.
    pub const MANUFACTURER_CODE: bus::Region = 0x013F..=0x0142;

    /// This area contains a two-character ASCII “licensee code” indicating the game’s publisher. It is only meaningful if the Old licensee is exactly `$33` (which is the case for essentially all games made after the SGB was released); otherwise, the old code must be considered.
    ///
    /// See [gbdev.io](https://gbdev.io/pandocs/The_Cartridge_Header.html#01440145--new-licensee-code) for sample licensee codes.
    pub const NEW_LICENSEE_CODE: bus::Region = 0x0144..=0x0145;

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
    pub const CGB_FLAGS: bus::Addr = 0x0146;

    /// This byte indicates what kind of hardware is present on the cartridge — most notably its [mapper](https://gbdev.io/pandocs/MBCs.html#mbcs).
    ///
    /// See [gbdev.io](https://gbdev.io/pandocs/The_Cartridge_Header.html#0147--cartridge-type) for
    /// more info.
    pub const CARTRIDGE_TYPE: bus::Addr = 0x0147;

    /// This byte indicates how much ROM is present on the cartridge. In most cases, the ROM size is given by 32 KiB × (1 << <value>):
    ///
    /// See [gbdev.io](https://gbdev.io/pandocs/The_Cartridge_Header.html#0148--rom-size) for rom
    /// size values.
    pub const ROM_SIZE: bus::Addr = 0x0148;

    /// This byte indicates how much RAM is present on the cartridge, if any.
    ///
    /// If the cartridge type does not include “RAM” in its name, this should be set to 0. This includes MBC2, since its 512 × 4 bits of memory are built directly into the mapper.
    pub const RAM_SIZE: bus::Addr = 0x0148;

    /// This byte specifies whether this version of the game is intended to be sold in Japan (`0x00`) or elsewhere (`0x01`).
    pub const DESTINATION_CODE: bus::Addr = 0x014A;

    /// This byte is used in older (pre-SGB) cartridges to specify the game’s publisher. However, the value `$33` indicates that the New licensee codes must be considered instead. (The SGB will ignore any command packets unless this value is `$33`.)
    ///
    /// See [gbdev.io](https://gbdev.io/pandocs/The_Cartridge_Header.html#014b--old-licensee-code)
    /// for table of licensee codes.
    pub const OLD_LICENSEE_CODE: bus::Addr = 0x014B;

    /// This byte specifies the version number of the game. It is usually `$00`.
    pub const ROM_VERSION_NUMBER: bus::Addr = 0x014C;

    /// This byte contains an 8-bit checksum computed from the cartridge header bytes $0134–014C. The boot ROM computes the checksum as follows:
    /// ```text
    /// uint8_t checksum = 0;
    /// for (uint16_t address = 0x0134; address <= 0x014C; address++) {
    ///   checksum = checksum - rom[address] - 1;
    /// }
    /// ```
    /// The boot ROM verifies this checksum. If the byte at `$014D` does not match the lower 8 bits of checksum, the boot ROM will lock up and the program in the cartridge **won’t run**.
    pub const HEADER_CHECKSUM: bus::Addr = 0x014D;

    /// These bytes contain a 16-bit (big-endian) checksum simply computed as the sum of all the bytes of the cartridge ROM (except these two checksum bytes).
    ///
    /// This checksum is not verified, except by Pokémon Stadium’s “GB Tower” emulator (presumably to detect Transfer Pak errors).
    pub const GLOBAL_CHECKSUM: bus::Region = 0x014E..=0x014F;
}
