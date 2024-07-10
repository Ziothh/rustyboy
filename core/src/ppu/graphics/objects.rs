//! # Object attribute memory (OAM)
//!
//!
//! The Game Boy PPU can display up to 40 movable objects (or sprites), each 8×8 or 8×16 pixels.
//! Because of a limitation of hardware, only ten objects can be displayed per scanline.
//! Object tiles have the same format as BG tiles, but they are taken from tile blocks 0 and 1 located at $8000-8FFF and have unsigned numbering.
//!
//! 40 entries consisting of four bytes.
//! - Byte 0: Y Position
//! - Byte 1: X Position
//! - Byte 2: Tile index
//! - Byte 3: Attributes/Flags
//!
//! See [gbdev.io](https://gbdev.io/pandocs/OAM.html) for more info.

use crate::hardware::bus;

#[derive(Debug)]
pub struct Object<'a>(&'a [u8; Object::BYTE_SIZE]);

impl Object<'_> {
    /// The byte size in memory for one object
    const BYTE_SIZE: usize = 4;
    /// Amount of objects there are in the OAM memory
    const OAM_ENTRIES: usize = bus::regions::size(bus::regions::OAM) / Self::BYTE_SIZE;
}

impl<'a> Object<'a> {
    pub fn new(bytes: &'a [u8; Object::BYTE_SIZE]) -> Self {
        Self(bytes)
    }

    pub fn read_all_from_bus(memory_bus: &'a bus::Interface) -> [Self; Object::OAM_ENTRIES] {
        memory_bus[bus::regions::OAM]
            .windows(Object::BYTE_SIZE)
            .step_by(Object::BYTE_SIZE)
            .map(|bytes| Self::new(bytes.try_into().expect("OAM slice should be 4 bytes long")))
            .collect::<Vec<_>>()
            .try_into()
            .expect("#OAM entries should be exactly 40")
    }

    #[inline]
    pub fn bytes(&self) -> &[u8; Object::BYTE_SIZE] {
        self.0
    }

    /// Location of the top pixel of an object.
    /// Equal to the position on the (screen + 16).
    ///
    /// Some examples:
    /// - Y=0 hides an object
    /// - Y=8 hides an 8x8 object, but show the last 2 rows of an 8x16 object
    /// - Y=16 shows an object at the top of the screen
    /// - Y=160 hides an object
    ///
    /// See [gbdev.io](https://gbdev.io/pandocs/OAM.html#byte-0--y-position)
    pub fn y(&self) -> u8 {
        self.bytes()[0]
    }
    /// The Y position of the object on the screen
    pub fn pos_y(&self) -> i32 {
        self.y() as i32 - 16 /* 16 lines above the screen */
    }
    /// X = Object’s horizontal position on the screen + 8.
    /// This works similarly to the examples above, except that the width of an object is always 8.
    /// An off-screen value (X=0 or X>=168) hides the object, but the object still contributes to the limit of ten objects per scanline.
    pub fn x(&self) -> u8 {
        self.bytes()[1]
    }
    /// The X position of the object on the screen
    pub fn pos_x(&self) -> i32 {
        self.x() as i32 - 8 /* 8 lines left of the screen */
    }

    /// In 8×8 mode (LCDC bit 2 = 0), this byte specifies the object’s only tile index ($00-$FF).
    /// This unsigned value selects a tile from the memory area at $8000-$8FFF.
    /// In CGB Mode this could be either in VRAM bank 0 or 1, depending on bit 3 of the following byte.
    /// In 8×16 mode (LCDC bit 2 = 1), the memory area at $8000-$8FFF is still interpreted as a series of 8×8 tiles, where every 2 tiles form an object.
    /// In this mode, this byte specifies the index of the first (top) tile of the object.
    /// This is enforced by the hardware: the least significant bit of the tile index is ignored; that is, the top 8×8 tile is “NN & $FE”, and the bottom 8×8 tile is “NN | $01”.
    pub fn tile_index(&self) -> u8 {
        self.bytes()[2]
    }
    /// TODO
    pub fn tile(&self) -> ! {
        todo!()
    }

    /// Parses all the flags from the flags byte.
    ///
    /// NOTE: only DMG flags are supported
    pub fn flags(&self) -> impl Iterator<Item = &ObjectFlag> {
        let byte = self.bytes()[3];
        ObjectFlag::ALL
            .iter()
            .filter(move |flag| byte & (**flag as u8) != 0)
    }
}

/// Attributes/Flags
// #[enumflags2::bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
/// [gbdev.io](https://gbdev.io/pandocs/OAM.html#byte-3--attributesflags)
pub enum ObjectFlag {
    /// `0` = No, `1` = BG and Window colors 1–3 are drawn over this OBJ
    Priority = 0b1000_0000,
    /// `0` = Normal, `1` = Entire OBJ is vertically mirrored
    FlipY = 0b0100_0000,
    /// `0` = Normal, `1` = Entire OBJ is horizontally mirrored
    FlipX = 0b0010_0000,
    /// [Non CGB Mode only\]*: `0` = OBP0, `1` = OBP1
    DMGPalette = 0b0001_0000,
    // [CGB Mode Only] Not used atm
    // /// [CGB Mode Only\]*: `0` = Fetch tile from VRAM bank 0, `1` = Fetch tile from VRAM bank 1
    // Bank = 0b0000_1000,
    // /// [CGB Mode Only\]*: Which of OBP0–7 to use
    // CGBPalette = 0b0000_0111, // TODO: if I were to support CGB mode these'll need to get parsed
}
impl ObjectFlag {
    /// All DMG-only flags
    ///
    /// TODO: change this if CGB would be supported
    pub const ALL: [Self; 4] = [Self::Priority, Self::FlipY, Self::FlipX, Self::DMGPalette];
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hardware::bus;

    #[test]
    fn assert_amount_entries_in_bus() {
        let n_bytes = bus::regions::OAM.end() + 1 - bus::regions::OAM.start();

        dbg!(n_bytes);
        assert_eq!(40, n_bytes / Object::BYTE_SIZE as u16);
        assert_eq!(0, n_bytes % Object::BYTE_SIZE as u16);
    }
}
