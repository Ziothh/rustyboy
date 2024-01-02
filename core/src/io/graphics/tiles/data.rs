use crate::io::graphics::ColorID;

/// A tile (8*8 pixels) consists of 16 bytes.
/// 2 bytes for each row.
/// Each bit represents a part of the `ColorID`
/// (e.g. for the first ColorID: byte0\[bit0] = lsb, byte1\[bit0] = msb)
pub struct Tile<'a>(&'a [u8; Tile::BYTES]);

impl<'a> Tile<'a> {
    /// The amount of bytes a `Tile` consists of
    const BYTES: usize = 16;
    /// Width in pixels
    pub const PIXEL_WIDTH: usize = 8;
    /// Height in pixels
    pub const PIXEL_HEIGHT: usize = 8;
    /// Width * height in pixels
    const AREA: usize = Self::PIXEL_WIDTH * Self::PIXEL_HEIGHT;

    pub fn new(tile_bytes: &'a [u8; Tile::BYTES]) -> Self {
        Self(tile_bytes)
    }

    #[inline]
    pub fn bytes(&self) -> &[u8; Tile::BYTES] {
        self.0
    }

    /// # Example
    /// ```
    /// use gb::io::graphics::{ColorID, Tile};
    ///
    /// let mut bytes = [0u8; 16];
    /// assert_eq!(
    ///     Tile::new(&bytes.clone()).to_color_ids()[0],
    ///     ColorID::ZERO
    /// );
    ///
    /// bytes[0] |= 0b1 << 7; // Set lsb of pixel 0
    /// assert_eq!(
    ///     Tile::new(&bytes.clone()).to_color_ids()[0],
    ///     ColorID::ONE
    /// );
    /// bytes[1] |= 0b1 << 7; // Set msb of pixel 0
    /// assert_eq!(
    ///     Tile::new(&bytes.clone()).to_color_ids()[0],
    ///     ColorID::THREE
    /// );
    ///
    /// bytes[2] |= 0b1 << 5; // Set lsb of pixel 10
    /// bytes[3] |= 0b1 << 5; // Set msb of pixel 10
    /// assert_eq!(
    ///     Tile::new(&bytes.clone()).to_color_ids()[10],
    ///     ColorID::THREE
    /// );
    ///
    /// bytes[3] |= 0b1 << 4; // Set lsb of pixel 11
    /// assert_eq!(
    ///     Tile::new(&bytes.clone()).to_color_ids()[11],
    ///     ColorID::TWO
    /// );
    /// ```
    pub fn to_color_ids(&self) -> [ColorID; Tile::AREA] {
        let mut arr = [ColorID::default(); Tile::AREA];

        for (i, x) in arr.iter_mut().enumerate() {
            let row = i / Self::PIXEL_HEIGHT;
            let column = i % Self::PIXEL_WIDTH;

            let lsb = self.bytes()[row * 2];
            let msb = self.bytes()[row * 2 + 1];

            *x = ColorID::new(
                (0b1 & lsb >> (Self::PIXEL_WIDTH - 1 - column))
                    | ((0b1 & msb >> (Self::PIXEL_WIDTH - 1 - column)) << 1),
            );
        }

        return arr;
    }
}

/// Each tile takes 16 bytes, thus allows for 384 tiles.
mod vram_regions {
    use std::ops;

    use crate::hardware::MemoryBus;

    /// The full VRAM memory range of the tile data
    /// It contains 384 tiles.
    pub const FULL_RANGE: MemoryBus::Region = 0x8000..=0x97FF;

    const BLOCK_AMOUNT: usize = 3;
    const SIZE: u16 = (*FULL_RANGE.end() + 1) - *FULL_RANGE.start();
    const BLOCK_SIZE: u16 = SIZE / BLOCK_AMOUNT as u16;
    const TILES_PER_BLOCK: u16 = BLOCK_SIZE / 16;

    const fn calc_block(block_index: u16) -> ops::RangeInclusive<u16> {
        (*FULL_RANGE.start() + (block_index * BLOCK_SIZE))
            ..=(*FULL_RANGE.start() - 1 + ((block_index + 1) * BLOCK_SIZE))
    }

    /// The memory ranges of the 3 blocks of 128 tiles
    pub const BLOCKS: [MemoryBus::Region; BLOCK_AMOUNT] =
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

    // mod 
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_block_boundries() {
        use super::vram_regions::*;

        assert_eq!(BLOCKS[0].len() / 16, 128);
        assert_eq!(BLOCKS[1].len() / 16, 128);
        assert_eq!(BLOCKS[2].len() / 16, 128);

        assert_eq!(*BLOCKS[0].start(), *FULL_RANGE.start());
        assert_eq!(*BLOCKS[0].end(), *BLOCKS[1].start() - 1);
        assert_eq!(*BLOCKS[1].end(), *BLOCKS[2].start() - 1);
        assert_eq!(*BLOCKS[2].end(), *FULL_RANGE.end());
    }


    #[test]
    fn parse_tile_data() {
        #[rustfmt::skip]
        const TILE_BYTES: [u8; 16] = [
            0x3C, 0x7E, 
            0x42, 0x42, 
            0x42, 0x42, 
            0x42, 0x42, 
            0x7E, 0x5E, 
            0x7E, 0x0A, 
            0x7C, 0x56,
            0x38, 0x7C,
        ];

        #[rustfmt::skip]
        const LEAST_SIGNIFICANT_BITS: [u8; Tile::PIXEL_HEIGHT] = [
            0b00111100,
            0b01000010,
            0b01000010,
            0b01000010,
            0b01111110,
            0b01111110,
            0b01111100,
            0b00111000,
        ];
        TILE_BYTES
            .iter()
            .step_by(2)
            .zip(LEAST_SIGNIFICANT_BITS)
            .for_each(|(a, b)| assert_eq!(*a, b));

        const MOST_SIGNIFICANT_BITS: [u8; Tile::PIXEL_HEIGHT] = [
            0b01111110,
            0b01000010,
            0b01000010,
            0b01000010,
            0b01011110,
            0b00001010,
            0b01010110,
            0b01111100,
        ];
        TILE_BYTES
            .iter()
            .skip(1)
            .step_by(2)
            .zip(MOST_SIGNIFICANT_BITS)
            .for_each(|(a, b)| assert_eq!(*a, b));
        


        #[rustfmt::skip]
        const COLOR_IDS: [u8; Tile::AREA] = [
            0b00, 0b10, 0b11, 0b11, 0b11, 0b11, 0b10, 0b00, 
            0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b11, 0b00, 
            0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b11, 0b00, 
            0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b11, 0b00, 
            0b00, 0b11, 0b01, 0b11, 0b11, 0b11, 0b11, 0b00, 
            0b00, 0b01, 0b01, 0b01, 0b11, 0b01, 0b11, 0b00, 
            0b00, 0b11, 0b01, 0b11, 0b01, 0b11, 0b10, 0b00, 
            0b00, 0b10, 0b11, 0b11, 0b11, 0b10, 0b00, 0b00,
        ];
        assert_eq!(
            Tile::new(&TILE_BYTES).to_color_ids(),
            COLOR_IDS.map(|b| ColorID::new(b))
        );
    }
}
