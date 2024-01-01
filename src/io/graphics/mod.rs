mod tiles;
pub use tiles::Tile;

/// 2 bit color ID a pixel
///
/// When a tile is used in the Background or Window, these color IDs are associated with a palette.
/// When a tile is used in an object, the IDs 1 to 3 are associated with a palette, but ID 0 means transparent.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ColorID(u8);
impl ColorID {
    pub const ZERO: Self = Self(0b0000_0000);
    pub const ONE: Self = Self(0b0000_0001);
    pub const TWO: Self = Self(0b0000_0010);
    pub const THREE: Self = Self(0b0000_0011);

    pub const MIN: u8 = Self::ZERO.as_byte();
    pub const MAX: u8 = Self::THREE.as_byte();

    #[inline]
    pub const fn as_byte(&self) -> u8 {
        self.0
    }

    /// Masks the `byte` with `0b0000_0011` and returns it as a `ColorID`
    pub fn new(byte: u8) -> Self {
        Self(byte & Self::THREE.as_byte())
    }
    pub fn try_from_u8(byte: u8) -> Result<Self, u8> {
        match byte {
            Self::MIN..=Self::MAX => Ok(Self(byte)),
            _ => Err(byte),
        }
    }
}
impl Default for ColorID {
    fn default() -> Self {
        Self::ZERO
    }
}

/// Ordered from front to back.
///
/// Some features and behaviors break this abstraction, but it works for the most part.
enum Layer {
    /// The background is composed of a tilemap.
    /// A tilemap is a large grid of tiles.
    /// However, tiles aren’t directly written to tilemaps, they merely contain references to the tiles.
    /// This makes reusing tiles very cheap, both in CPU time and in required memory space, and it is the main mechanism that helps work around the paltry 8 KiB of video RAM.
    ///
    /// The background can be made to scroll as a whole, writing to two hardware registers. This makes scrolling very cheap.
    Background,
    /// The window is sort of a second background layer on top of the background.
    /// It is fairly limited: it has no transparency, it’s always a rectangle and only the position of the top-left pixel can be controlled.
    ///
    /// Possible usage include a fixed status bar in an otherwise scrolling game (e.g. Super Mario Land 2).
    Window,
    /// The objects layer is designed to fill this gap: objects are made of 1 or 2 stacked tiles (8×8 or 8×16 pixels) and can be displayed anywhere on the screen.
    ///
    /// NOTE: Several objects can be combined (they can be called metasprites) to draw a larger graphical element, usually called “sprite”.
    Objects,
}


/// Array of 4 colors
type Palette = [(); 4];

mod lcd {
    pub const PX_WIDTH: usize = 160;
    pub const PX_HEIGHT: usize = 144;
}

mod vram_regions {
    use std::ops;

    /// Each tile takes 16 bytes, thus allows for 384 tiles.
    pub mod tile_data {
        use std::ops;

        /// The full VRAM memory range of the tile data
        /// It contains 384 tiles.
        const FULL_RANGE: ops::RangeInclusive<u16> = 0x8000..=0x97FF;

        const BLOCK_AMOUNT: usize = 3;
        const SIZE: u16 = (*FULL_RANGE.end() + 1) - *FULL_RANGE.start();
        const BLOCK_SIZE: u16 = SIZE / BLOCK_AMOUNT as u16;
        const TILES_PER_BLOCK: u16 = BLOCK_SIZE / 16;

        const fn calc_block(block_index: u16) -> ops::RangeInclusive<u16> {
            (*FULL_RANGE.start() + (block_index * BLOCK_SIZE))
                ..=(*FULL_RANGE.start() - 1 + ((block_index + 1) * BLOCK_SIZE))
        }

        /// The memory ranges of the 3 blocks of 128 tiles
        pub const BLOCKS: [ops::RangeInclusive<u16>; BLOCK_AMOUNT] =
            [calc_block(0), calc_block(1), calc_block(2)];

        /// Tiles are always indexed using an 8-bit integer, but the addressing method may differ.
        /// The “$8000 method” uses $8000 as its base pointer and uses an unsigned addressing, meaning that tiles 0-127 are in block 0, and tiles 128-255 are in block 1.
        /// The “$8800 method” uses $9000 as its base pointer and uses a signed addressing, meaning that tiles 0-127 are in block 2, and tiles -128 to -1 are in block 1, or to put it differently, “$8800 addressing” takes tiles 0-127 from block 2 and tiles 128-255 from block 1.
        /// (You can notice that block 1 is shared by both addressing methods)
        ///
        /// Objects always use “$8000 addressing”, but the BG and Window can use either mode, controlled by **LCDC bit 4**.
        pub fn get_tile() -> ! {
            todo!()
        }

        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn check_block_boundries() {
                assert_eq!(BLOCKS[0].len() / 16, 128);
                assert_eq!(BLOCKS[1].len() / 16, 128);
                assert_eq!(BLOCKS[2].len() / 16, 128);

                assert_eq!(*BLOCKS[0].start(), *FULL_RANGE.start());
                assert_eq!(*BLOCKS[0].end(), *BLOCKS[1].start() - 1);
                assert_eq!(*BLOCKS[1].end(), *BLOCKS[2].start() - 1);
                assert_eq!(*BLOCKS[2].end(), *FULL_RANGE.end());
            }
        }
    }
}
