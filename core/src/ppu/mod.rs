use crate::utils::UNDEFINED_READ;

use self::graphics::{lcd, LCDControl, LCDStat, ColorID, Palette};
use super::memory_bus as bus;

mod graphics;

pub struct PPU {
    pub control: LCDControl,
    pub stat: LCDStat,
    pub mode: PPUMode,

    /// # LY Compare (LYC)
    /// When `LYC == LY` the flag in STAT is set and (if enabled) a STAT interrupt is requested
    pub lyc: u8,
    /// # LCD Y coordinate
    /// Indicates the current line which is being drawn or has just been drawn.
    ///
    /// Ranges between `0..=153`, with `144..=153` indicating VBlank
    pub ly: u8,

    /// # 8 KiB Video RAM (VRAM)
    /// From cartridge, switchable bank if CGB
    ///
    /// ## Tile Data (`0x8000..=0x97FF`)
    /// [Pandocs reference](https://gbdev.io/pandocs/Tile_Data.html#vram-tile-data)
    ///
    /// Each tile consists of 8x8 pixels takes 16 bytes of space.
    ///
    /// The tile data is split up in blocks of 128 bytes.
    /// There are 3×128 = 384 tiles in total (6×128 = 768 in CGB mode because of 2 switchable banks).
    ///
    /// There are 2 ways of addressing into the tile data.
    /// Read the docs for `LCDC.4` for more info
    ///
    /// ## Tile Maps (`0x9800..=0x9FFF`)
    /// The Game Boy contains 2 32×32 tile maps in VRAM.
    ///  - `0x9800..=0x9BFF`
    ///  - `0x9C00..=0x9FFF`
    ///
    ///  Any of these maps can be used to display the Background or the Window.
    ///
    ///  Since one tile has 8×8 pixels, each map holds a 256×256 pixels picture.
    ///  Only 160×144 of those pixels are displayed on the LCD at any given time.
    pub vram: Box<[u8; bus::regions::size(bus::regions::VRAM)]>,

    /// # BG palette data
    ///
    /// ```text
    /// Bits     | 7 6 | 5 4 | 3 2 | 1 0
    /// Color ID |  3  |  2  |  1  |  0
    /// ```
    pub bg_palette: Palette,
    /// Works exactly like `bg_palette`, except that the lower two bits are ignored because color index 0 is transparent for OBJs.
    pub obj0_palette: Palette,
    /// Works exactly like `bg_palette`, except that the lower two bits are ignored because color index 0 is transparent for OBJs.
    pub obj1_palette: Palette,

    /* [Background] */
    /// Background Scroll Y
    /// Specifies the origin of the visible 160×144 pixel area within the total 256×256 pixel Background map.
    pub scy: u8,
    /// Background Scroll X
    /// Specifies the origin of the visible 160×144 pixel area within the total 256×256 pixel Background map.
    pub scx: u8,

    /* [Window] */
    /// Window Y position
    pub wy: u8,
    /// Window X position
    pub wx: u8,

    /// # Object Attribute Memory (OAM)
    /// Contains 160 bytes
    ///
    /// The Game Boy PPU can display up to 40 movable objects (or sprites), each 8×8 or 8×16 pixels.
    /// Because of a limitation of hardware, only ten objects can be displayed per scanline.
    ///
    /// Object tiles have the same format as Background tiles, but may only be taken
    /// from the lower 2 tile blocks 0 and 1 (`0x8000..=0x8FFF`) using unsigned indexing
    pub oam: Box<[u8; bus::regions::size(bus::regions::OAM)]>,

    /// Writing to this register starts a DMA transfer from ROM or RAM to OAM
    pub oam_dma: OamDma,


    /* [Rendering] */
    /// Background pixels FIFO
    pub bg_fifo: FIFO,
    // bg_fifo: FIFOPixelFetcher<'static>, // TODO
    /// Object (sprite) pixels FIFO
    pub obj_fifo: FIFO,
}
impl Default for PPU {
    fn default() -> Self {
        Self {
            bg_fifo: FIFO::empty(),
            obj_fifo: FIFO::empty(),

            oam: Box::new([0; _]),
            vram: Box::new([0; _]),

            oam_dma: Default::default(),
            ly: Default::default(),
            lyc: Default::default(),
            wy: Default::default(),
            wx: Default::default(),
            scy: Default::default(),
            scx: Default::default(),

            stat: LCDStat::empty(), // TODO: check if true
            mode: PPUMode::OAMScan, // TODO: check if true
            control: LCDControl::empty(), // TODO: check if true
            bg_palette: Default::default(), // TODO: check if true
            obj0_palette: Default::default(), // TODO: check if true
            obj1_palette: Default::default(), // TODO: check if true

        }
    }
}

impl PPU {
    // pub fn draw_screen<'s, 'bus>(&'s mut self, memory_bus: &'bus mut bus::Interface) -> impl Iterator<Item = FIFOPixelFetcher>
    //     where 'bus: 's
    // {
    //     (0..(graphics::lcd::WINDOW_HEIGHT + 16))
    //         .map(|i| (i, FIFOPixelFetcher::new(memory_bus)))
    //         .map(|(i, fetcher)| {
    //             memory_bus[bus::regions::io_registers::lcd::LY] = i as u8;
    //             return fetcher;
    //         })
    //
    // }

    /// TODO: find out where this is needed and maybe inline
    ///
    /// [Pandocs](https://gbdev.io/pandocs/Scrolling.html#ff42ff43--scy-scx-background-viewport-y-position-x-position)
    fn get_bg_scroll_bottom_right_coords(&self) -> (u8, u8) {
        return (
            self.scy.wrapping_add(143), // Bottom
            self.scx.wrapping_add(159), // Right
        );
    }

    /// TODO: find out where this is needed and maybe inline
    ///
    /// [Pandocs](https://gbdev.io/pandocs/Scrolling.html#ff42ff43--scy-scx-background-viewport-y-position-x-position)
    fn get_window_top_left_coords(&self) -> (u8, u8) {
        return (
            self.wy,                  // Top
            self.scx.wrapping_sub(7), // Left
        );
    }

    fn draw_ly_line<'s, 'bus>(
        &'s mut self,
        memory_bus: &'bus mut bus::Bus<'bus>,
    ) -> FIFOPixelFetcher<'bus>
    where
        'bus: 's,
    {
        FIFOPixelFetcher::new(memory_bus)
    }

    pub fn read_stat_register(&self) -> u8 {
        const STAT_UNUSED_MASK: u8 = (1 << 7);

        return if self.control.contains(LCDControl::LCD_ENABLED) {
            STAT_UNUSED_MASK 
                | self.stat.bits() 
                | ((self.lyc == self.ly) as u8) << 2  // This might cause issues. Maybe it should only "update" on every cycle
                | self.mode as u8
        } else {
            STAT_UNUSED_MASK
        };
    }
    pub fn write_stat_register(&mut self, byte: u8) {
        self.stat = LCDStat::from_bits_truncate(byte);
        // Mode and LYC are read-only
    }


    pub fn read_control_register(&self) -> u8 {
        self.control.bits()
    }
    pub fn write_control_register(&mut self, value: u8) {
        let new_control = LCDControl::from_bits_truncate(value);

        if !new_control.contains(LCDControl::LCD_ENABLED)
            && new_control.contains(LCDControl::LCD_ENABLED)
        {
            if self.mode != PPUMode::VerticalBlank {
                panic!("Warning! Turned LCD off, but not in VBlank")
            }

            todo!("Set current current line to 0");
        }

        if new_control.contains(LCDControl::LCD_ENABLED) && !self.control.contains(LCDControl::LCD_ENABLED) {
            self.mode = PPUMode::HorizontalBlank;
            todo!("Set cycles");
            // self.cycles = PPUMode::AccessOam.cycles(self.scroll_x); // TODO: cycles support
        }

        self.control = new_control;
    }

    pub fn write_vram(&mut self, addr: u16, byte: u8) {
        if self.mode == PPUMode::Drawing {
            return;
        }

        return self.vram[addr as usize & 0x1FFF] = byte;
    }
    pub fn read_vram(&mut self, addr: u16) -> u8 {
        match self.mode {
            PPUMode::Drawing => UNDEFINED_READ,
            _ =>self.vram[addr as usize & 0x1FFF] 
        }
    }

}



pub struct OamDma {
    pub requested: Option<u8>,
    pub source: u8,
}
impl Default for OamDma {
    fn default() -> Self {
        Self {
            requested: None,
            source: 0xFF,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PPUMode {
    /// Waiting until the end of the scanline
    HorizontalBlank = 0,
    /// Waiting until the next frame
    VerticalBlank = 1,
    /// Accesses OAM memory and locks it.
    /// Searches for OBJs which overlap the current line
    OAMScan = 2,
    /// Accesses VRAM memory and locks it.
    /// Sends pixels to the LCD
    Drawing = 3,
}

struct Pixel {
    color: graphics::ColorID,
    /// On CGB a value between 0 and 7 and on DMG this only applies to objects
    palette: graphics::Palette,
    // sprite_priority // Doesn't exist on DMG
    /// Holds the value of the `ObjectFlag::Priority` bit
    bg_priority: bool,
}
impl Pixel {
    pub fn from_bus(memory_bus: &bus::Bus, color_id: graphics::ColorID) -> Self {
        todo!("FIXME after refactor")

        // Self {
        //     color: color_id,
        //     palette: graphics::Palette::from_bgp(memory_bus), // TODO: also support objects
        //     bg_priority: false,                               // TODO
        // }
    }
}

struct FIFO(Vec<Pixel>);
impl FIFO {
    const MIN_SIZE: usize = 16;
    const MAX_SIZE: usize = 16;

    pub fn empty() -> Self {
        Self(Vec::with_capacity(Self::MAX_SIZE))
    }

    pub fn push(&mut self, pixel: Pixel) -> &mut Self {
        self.0.push(pixel);
        return self;
    }
    pub fn pop(&mut self) -> Pixel {
        self.0.remove(0)
    }
}

struct FIFOPixelFetcher<'bus> {
    /// The tile X coordinate
    x: u8,
    memory_bus: &'bus mut bus::Bus<'bus>,
}
impl<'bus> FIFOPixelFetcher<'bus> {
    pub fn new(memory_bus: &'bus mut bus::Bus<'bus>) -> Self {
        Self { x: 0, memory_bus }
    }
}
impl<'bus> Iterator for FIFOPixelFetcher<'bus> {
    type Item = [Pixel; 8];

    /// The fetcher fetches a row of 8 background or window pixels and queues them up to be mixed with object pixels.
    fn next(&mut self) -> Option<Self::Item> {
        todo!("Fix broken code after refactor");
        // if self.x == graphics::lcd::WINDOW_WIDTH as u8 + 8
        // /* extra padding */
        // {
        //     return None;
        // }
        //
        // let memory_bus = &self.memory_bus;
        //
        // let draw_y = memory_bus[bus::regions::io_registers::lcd::LY];
        // let control = lcd::LCDControl::from_bus(memory_bus);
        //
        // let win_y = memory_bus[bus::regions::io_registers::lcd::WY];
        // let win_x = memory_bus[bus::regions::io_registers::lcd::WX];
        //
        // let win_y_seen = draw_y >= win_y;
        //
        // // [1. Get the tile]
        // let tile = match win_y_seen && control.window_enabled() && self.x >= win_x {
        //     // Window tile
        //     true => control.window_tilemap().get_tile(
        //         memory_bus,
        //         // TODO: check if this is right
        //         self.x - win_x,
        //         draw_y,
        //         false,
        //     ),
        //     // Background tile
        //     false => {
        //         let scx = memory_bus[bus::regions::io_registers::lcd::SCX];
        //         let scy = memory_bus[bus::regions::io_registers::lcd::SCY];
        //         control.bg_tilemap().get_tile(
        //             memory_bus,
        //             ((scx / 8) + self.x) & 0x1F, /* Clamps value to 0..32 */
        //             self.x.clone().wrapping_add(scy), /* Clamps value to 0..255 */
        //             false,
        //         )
        //     }
        // };
        //
        // // TODO: check if this is correct
        // let row = tile.get_pixel_row(draw_y % graphics::tiles::Tile::PIXEL_HEIGHT as u8);
        // self.x += 1;
        // return Some(row.map(|color_id| Pixel::from_bus(memory_bus, color_id)));
    }
}
