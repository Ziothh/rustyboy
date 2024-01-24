use std::fmt::Display;

/// Every pair of bytes represents a tile.
/// B`x`: byte `x`
/// n`x`: nibble `x`: 0 = bits 0..4, 1 = bits 4..8
/// b`x`: bit of nibble `x`: 0..4
///
/// # Example of first 4x4 tile on row 0 (16 bits = 16 pixels)
/// |B0n1b3|B0n1b2|B0n1b1|B0n1b0||B2n1b3| // And so on
/// |B0n0b3|B0n0b2|B0n0b1|B0n0b0|
/// |B1n1b3|B1n1b2|B1n1b1|B1n1b0|
/// |B1n0b3|B1n0b2|B1n0b1|B1n0b0|
///
/// See [visual represnation](https://imgur.com/BikSgOo)
/// See [article about layout](https://catskull.net/gameboy-boot-screen-logo.html)
pub struct Logo([u8; Self::BYTE_LENGTH]);

impl Logo {
    const BYTE_LENGTH: usize = 48;

    #[rustfmt::skip]
    pub const NINTENDO_LOGO_BYTES: [u8; Self::BYTE_LENGTH] = [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 
        0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 
        0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 
        0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
    ];

    const EMPTY: [u8; Self::BYTE_LENGTH] = [0; Self::BYTE_LENGTH];

    pub const fn bytes(&self) -> [u8; 48] {
        self.0
    }

    pub fn new(bytes: [u8; Self::BYTE_LENGTH]) -> Self {
        Self(bytes)
    }
    
    pub fn from_slice(bytes: &[u8]) -> Self {
        let mut buf = Self::EMPTY;

        for (i,x) in bytes.iter().enumerate() {
            buf[i] = *x;
        }

        return Self(buf);
    }

    fn tile_half_as_pixels(&self, index: usize) -> [Vec<bool>; 4] {
        assert!(index <= 1);

        let bytes = {
            let half = Self::BYTE_LENGTH / 2;
            &self.bytes()[(half * index)..(half * (index + 1))]
        };

        fn parse_pixel_row(tile_byte_index: usize, pixel_row_index: usize, half_logo_bytes: &[u8]) -> Vec<bool> {
            assert!(tile_byte_index <= 1);
            assert!(pixel_row_index <= 1);

            half_logo_bytes
                .iter()
                .skip(tile_byte_index)
                .step_by(2)
                .map(|byte| {
                    let nibble = (byte >> (4 - pixel_row_index * 4)) & 0b1111;

                    let mut pixels = [false; 4];

                    for i in 0..4 {
                        pixels[i] = (nibble & (1 << (3 - i))) > 0;
                    }

                    return pixels;
                })
                .flatten()
                .collect()
        }

        return [
            parse_pixel_row(0, 0, &bytes),
            parse_pixel_row(0, 1, &bytes),
            parse_pixel_row(1, 0, &bytes),
            parse_pixel_row(1, 1, &bytes),
        ];
    }

    pub fn as_pixels(&self) -> [[Vec<bool>; 4]; 2] {
        return [
            self.tile_half_as_pixels(0),
            self.tile_half_as_pixels(1),
        ];
    }
}

impl Default for Logo {
    fn default() -> Self {
        Self::new(Self::NINTENDO_LOGO_BYTES)
    }
}

impl Display for Logo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.as_pixels().iter().flatten() {
            for bit in row {
                write!(f, "{}", if *bit { 'X' } else { ' ' })?;
            }
            write!(f, "\n")?;
        }

        std::fmt::Result::Ok(())
    }
}


pub struct Tile {

}
