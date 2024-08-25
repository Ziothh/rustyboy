use crate::memory_bus as bus;
use crate::ppu::graphics;

/// A tile (8*8 pixels) consisting of 16 bytes.
///
/// Every row (8 pixels) is represented in memory as 2 bytes.
///  - 1 pixel (= ColorID) = 2 bits
///  - The first byte of the row contains the least significant bits
///  - The second byte of the row contains the most significant bits
///  - Example: pixel 0 = byte0\[bit0] = lsb, byte1\[bit0] = msb
pub struct Tile<'a>(&'a [u8; Tile::BYTES]);

impl<'a> Tile<'a> {
    /// The amount of bytes a `Tile` consists of
    const BYTES: usize = 16;
    /// The amount of bytes it takes to represent an 8 pixel row in memory
    const ROW_BYTES: usize = 2;
    /// Tile width in pixels
    pub const PIXEL_WIDTH: usize = 8;
    /// Tile height in pixels
    pub const PIXEL_HEIGHT: usize = 8;
    /// Width * height in pixels
    const AREA: usize = Self::PIXEL_WIDTH * Self::PIXEL_HEIGHT;

    pub fn new(tile_bytes: &'a [u8; Tile::BYTES]) -> Self {
        Self(tile_bytes)
    }

    /// Tiles are always indexed using an 8-bit integer, but the addressing method may differ.
    /// The “$8000 method” uses $8000 as its base pointer and uses an unsigned addressing, meaning that tiles 0-127 are in block 0, and tiles 128-255 are in block 1.
    /// The “$8800 method” uses $9000 as its base pointer and uses a signed addressing, meaning that tiles 0-127 are in block 2, and tiles -128 to -1 are in block 1, or to put it differently, “$8800 addressing” takes tiles 0-127 from block 2 and tiles 128-255 from block 1.
    /// (You can notice that block 1 is shared by both addressing methods)
    ///
    /// Objects always use “$8000 addressing”, but the BG and Window can use either mode, controlled by **LCDC bit 4**.
    pub fn from_bus(memory_bus: &'a bus::Bus, pointer: u8, is_object_tile: bool) -> Self {
        todo!("Fix broken code after refactor")
        // let mode = graphics::LCDControl::from_bus(memory_bus).tile_addressing_mode();
        // 
        // let addr: bus::Addr = match !is_object_tile && mode == graphics::tiles::AddresMode::Upper {
        //     // $8000: objects & lower addressing
        //     false => vram_regions::BLOCKS[0].start() + pointer as u16,
        //     // $8800: upper addressing
        //     true => {
        //         let start = *vram_regions::BLOCKS[2].start() as i32;
        //         // TODO: check if this numbers conversion works
        //         let index = pointer as i8 as i32;
        //         (start + index) as u16
        //     },
        // };
        //
        // return Self::new(
        //     memory_bus[addr..(addr + Self::BYTES as bus::Addr)]
        //         .try_into()
        //         .expect("Tile should be exactly 16 bytes long")
        // );
    }

    #[inline]
    pub fn bytes(&self) -> &[u8; Tile::BYTES] {
        self.0
    }

    fn get_pixel(&self, x: usize, y: usize) -> graphics::ColorID {
        let lsb = self.bytes()[y * 2];
        let msb = self.bytes()[y * 2 + 1];

        // TODO: check for vertical flip

        return graphics::ColorID::from_byte(
            (0b1 & lsb >> (Self::PIXEL_WIDTH - 1 - x))
                | ((0b1 & msb >> (Self::PIXEL_WIDTH - 1 - x)) << 1),
        );
    }

    pub fn get_pixel_row(&self, y: u8) -> [graphics::ColorID; Tile::PIXEL_WIDTH] {
        let y = y as usize;
        (0..Self::PIXEL_WIDTH)
            .map(|x| self.get_pixel(x, y))
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap()
    }

    /// # Example
    /// ```
    /// use gb::io::graphics::{ColorID, tiles::Tile};
    ///
    /// let mut bytes = [0u8; 16];
    /// assert_eq!(
    ///     Tile::new(&bytes.clone()).to_color_ids()[0],
    ///     ColorID::White
    /// );
    ///
    /// bytes[0] |= 0b1 << 7; // Set lsb of pixel 0
    /// assert_eq!(
    ///     Tile::new(&bytes.clone()).to_color_ids()[0],
    ///     ColorID::LightGray
    /// );
    /// bytes[1] |= 0b1 << 7; // Set msb of pixel 0
    /// assert_eq!(
    ///     Tile::new(&bytes.clone()).to_color_ids()[0],
    ///     ColorID::Black
    /// );
    ///
    /// bytes[2] |= 0b1 << 5; // Set lsb of pixel 10
    /// bytes[3] |= 0b1 << 5; // Set msb of pixel 10
    /// assert_eq!(
    ///     Tile::new(&bytes.clone()).to_color_ids()[10],
    ///     ColorID::Black
    /// );
    ///
    /// bytes[3] |= 0b1 << 4; // Set lsb of pixel 11
    /// assert_eq!(
    ///     Tile::new(&bytes.clone()).to_color_ids()[11],
    ///     ColorID::DarkGray
    /// );
    /// ```
    pub fn to_color_ids(&self) -> [graphics::ColorID; Tile::AREA] {
        let mut arr = [graphics::ColorID::White; Tile::AREA];

        for (i, x) in arr.iter_mut().enumerate() {
            *x = self.get_pixel(
                i % Self::PIXEL_WIDTH,
                i / Self::PIXEL_WIDTH,
            );
        }

        return arr;
    }
}

/// Each tile takes 16 bytes, thus allows for 384 tiles.
mod vram_regions {
    use std::ops;

    use super::bus;

    /// The full VRAM memory range of the tile data
    /// It contains 384 tiles.
    pub const FULL_RANGE: bus::Region = 0x8000..=0x97FF;

    const BLOCK_AMOUNT: usize = 3;
    const SIZE: u16 = (*FULL_RANGE.end() + 1) - *FULL_RANGE.start();
    const BLOCK_SIZE: u16 = SIZE / BLOCK_AMOUNT as u16;
    const TILES_PER_BLOCK: u16 = BLOCK_SIZE / 16;

    const fn calc_block(block_index: u16) -> ops::RangeInclusive<u16> {
        (*FULL_RANGE.start() + (block_index * BLOCK_SIZE))
            ..=(*FULL_RANGE.start() - 1 + ((block_index + 1) * BLOCK_SIZE))
    }

    /// The memory ranges of the 3 blocks of 128 tiles
    pub const BLOCKS: [bus::Region; BLOCK_AMOUNT] =
        [calc_block(0), calc_block(1), calc_block(2)];
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

        assert_eq!(*BLOCKS[2].start(), 0x9000);
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
            COLOR_IDS.map(|b| graphics::ColorID::from_byte(b))
        );
    }
}
