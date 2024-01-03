use crate::hardware::bus;

use super::Tile;

const BYTE_WIDTH: usize = 32;
const BYTE_HEIGHT: usize = 32;
const PIXEL_WIDTH: usize = BYTE_WIDTH * Tile::PIXEL_WIDTH;
const PIXEL_HEIGHT: usize = BYTE_HEIGHT * Tile::PIXEL_HEIGHT;

const TILE_MAP_1: bus::Region = 0x9800..=0x9BFF;
const TILE_MAP_2: bus::Region = 0x9C00..=0x9FFF;

type Index = u8;


/// Tiles are always indexed using an 8-bit integer, but the addressing method may differ. 
/// The “$8000 method” uses $8000 as its base pointer and uses an unsigned addressing, meaning that tiles 0-127 are in block 0, and tiles 128-255 are in block 1. 
/// The “$8800 method” uses $9000 as its base pointer and uses a signed addressing, meaning that tiles 0-127 are in block 2, and tiles -128 to -1 are in block 1, or to put it differently, “$8800 addressing” takes tiles 0-127 from block 2 and tiles 128-255 from block 1. 
/// (You can notice that block 1 is shared by both addressing methods)
///
/// Objects always use “$8000 addressing”, but the BG and Window can use either mode, controlled by LCDC bit 4.
///
/// See [gbdev.io](https://gbdev.io/pandocs/Tile_Data.html#vram-tile-data) for more info.
fn index() -> ! {
    todo!()
}
