use super::Tile;
use crate::memory_bus as bus;

/// 32x32 tile maps
pub enum TileMap {
    /// `0x9800..=0x9BFF`
    Map0,
    /// `0x9C00..=0x9FFF`
    Map1,
}

/// A 32x32 byte slice of memory that contains indexes to
impl TileMap {
    const BYTE_WIDTH: usize = 32;
    const BYTE_HEIGHT: usize = 32;

    const PIXEL_WIDTH: usize = Self::BYTE_WIDTH * Tile::PIXEL_WIDTH;
    const PIXEL_HEIGHT: usize = Self::BYTE_HEIGHT * Tile::PIXEL_HEIGHT;

    pub fn region(&self) -> std::ops::RangeInclusive<u16> {
        return match self {
            TileMap::Map0 => 0x9800..=0x9BFF,
            TileMap::Map1 => 0x9C00..=0x9FFF,
        };
    }

    /// TODO: check if this is needed and maybe clean up
    pub fn get_tile<'s, 'bus>(
        &'s self,
        memory_bus: &'bus bus::Bus,
        x: u8,
        y: u8,
        is_object_tile: bool,
    ) -> Tile<'bus>
    where
        'bus: 's,
    {
        let ptr = memory_bus[(x as u16) + (y as u16 * Self::BYTE_WIDTH as u16)];

        return Tile::from_bus(memory_bus, ptr, is_object_tile);
    }
}

/// Tiles are always indexed using an 8-bit integer, but the addressing method may differ.
/// (You can notice that block 1 is shared by both addressing methods)
///
/// Objects always use “$8000 addressing”, but the BG and Window can use either mode, controlled by LCDC bit 4.
#[derive(Eq, PartialEq)]
pub enum TileAddressMode {
    /// # The “$8000 method”.
    /// Uses **$8000** as its base pointer and uses an unsigned addressing,
    /// meaning that tiles `0..=127` are in **block 0**, and tiles `128..=255` are in **block 1**.
    Lower,
    /// # The “$8800 method”
    /// Uses **$9000** as its base pointer and uses a signed addressing,
    /// meaning that tiles `0..=127` are in **block 2**, and tiles `-128..=-1` are in **block 1**,
    /// or to put it differently, “$8800 addressing” takes tiles `0..=127` from **block 2** and tiles `128..=255` from **block 1**.
    Upper,
}
