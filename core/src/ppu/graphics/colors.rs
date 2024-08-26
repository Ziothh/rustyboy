use crate::{memory_bus as bus, utils::bit};

/// 2 bit color ID a pixel
///
/// When a tile is used in the Background or Window, these color IDs are associated with a palette.
/// When a tile is used in an object, the IDs 1 to 3 are associated with a palette, but ID 0 means transparent.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ColorID {
    /// Color 0 has a special meaning in objects: it’s transparent,
    /// allowing the background or other objects behind it to show through.
    White = 0b00,
    LightGray = 0b01,
    DarkGray = 0b10,
    Black = 0b11,
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
pub struct Palette {
    pub bits: u8,
}
impl Palette {
    const PALETTE_SIZE: usize = 4;
    pub fn colors(&self) -> [ColorID; Self::PALETTE_SIZE] {
        let mut colors = [ColorID::White; Self::PALETTE_SIZE];

        for i in 0..Self::PALETTE_SIZE {
            colors[Self::PALETTE_SIZE - 1 - i] = ColorID::from_byte(self.bits >> (i * 2));
        }

        return colors;
    }
}

