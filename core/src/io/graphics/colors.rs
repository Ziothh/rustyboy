use crate::{utils::bit, hardware::bus};

/// 2 bit color ID a pixel
///
/// When a tile is used in the Background or Window, these color IDs are associated with a palette.
/// When a tile is used in an object, the IDs 1 to 3 are associated with a palette, but ID 0 means transparent.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ColorID {
    White = 0b0000_0000,
    LightGray = 0b0000_0001,
    DarkGray = 0b0000_0010,
    Black = 0b0000_0011,
}
impl ColorID {
    pub const MIN: u8 = Self::White.as_byte();
    pub const MAX: u8 = Self::Black.as_byte();

    #[inline]
    pub const fn as_byte(&self) -> u8 {
        *self as u8
    }

    /// Masks the `byte` with `0b0000_0011` and returns it as a `ColorID`
    pub fn from_byte(byte: u8) -> Self {
        match byte & Self::MAX {
            0 => Self::White,
            1 => Self::LightGray,
            2 => Self::DarkGray,
            3 => Self::Black,
            _ => unreachable!(),
        }
    }
    pub fn try_from_byte(byte: u8) -> Result<Self, u8> {
        match byte {
            Self::MIN..=Self::MAX => Ok(Self::from_byte(byte)),
            _ => Err(byte),
        }
    }
}


/// Array of 4 colors (2 bits each)
pub struct Palette(u8);
impl Palette {
    /// Fetches the color palette from the background palette register
    pub fn from_bgp(memory_bus: &bus::Interface) -> Self {
        Self(memory_bus[bus::regions::io_registers::lcd::BGP])
    }
    /// Fetches the color palette from the object palette register
    /// `palette_index` = 0 | 1;
    pub fn from_obp(memory_bus: &bus::Interface, palette_index: usize) -> Self {
        Self(memory_bus[bus::regions::io_registers::lcd::OBP[palette_index]])
    }

    pub fn get_color(&self, color_id: &ColorID) -> [bool; 2] {
        [
            bit::is_set(self.0, (color_id.as_byte() * 2) + 1), // 2n+1
            bit::is_set(self.0, color_id.as_byte() * 2), // 2n
        ]
    }
    
}
