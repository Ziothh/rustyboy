use super::bus;

mod graphics;

pub struct PPU {
    /// Background pixels FIFO
    bg_fifo: FIFO,
    // bg_fifo: FIFOPixelFetcher<'static>, // TODO
    /// Object (sprite) pixels FIFO
    obj_fifo: FIFO,

    /// Just a copy of OAM, taken before locking memory for the CPU
    oam_read: [u8; bus::regions::size(bus::regions::OAM)],
    /// Just a copy of VRAM, taken before locking memory for the CPU
    vram_read: [u8; bus::regions::size(bus::regions::VRAM)],
}
impl PPU {
    pub fn new() -> Self {
        Self {
            bg_fifo: FIFO::empty(),
            obj_fifo: FIFO::empty(),

            oam_read: [0; _],
            vram_read: [0; _],
        }
    }

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

    fn draw_ly_line<'s, 'bus>(&'s mut self, memory_bus: &'bus mut bus::Interface) -> FIFOPixelFetcher<'bus> 
        where 'bus: 's
    {
        FIFOPixelFetcher::new(memory_bus)
    }

    fn get_mode(memory_bus: &mut bus::Interface) -> PPUMode {
        graphics::lcd::LCDStatus::from_bus(memory_bus).get_ppu_mode()
    }

    fn set_mode(&mut self, mode: PPUMode, memory_bus: &mut bus::Interface) {
        // Unlock memory
        match PPU::get_mode(memory_bus) {
            PPUMode::Drawing => {
                memory_bus.unlock_region(bus::regions::VRAM);
            }
            PPUMode::OAMScan => {
                memory_bus.unlock_region(bus::regions::OAM);
            }
            _ => (), // Blanking modes don't block CPU from reading memory
        };

        // Lock memory
        match mode {
            PPUMode::Drawing => {
                self.vram_read = memory_bus[bus::regions::VRAM].try_into().unwrap();
                memory_bus.lock_region(bus::regions::VRAM);
            }
            PPUMode::OAMScan => {
                self.oam_read = memory_bus[bus::regions::OAM].try_into().unwrap();
                memory_bus.lock_region(bus::regions::OAM);
            }
            _ => (), // Blanking modes don't block CPU from reading memory
        }
    }
}

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
    pub fn from_bus(memory_bus: &bus::Interface, color_id: graphics::ColorID) -> Self {
        Self {
            color: color_id,
            palette: graphics::Palette::from_bgp(memory_bus), // TODO: also support objects
            bg_priority: false,                               // TODO
        }
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
    memory_bus: &'bus mut bus::Interface,
}
impl<'bus> FIFOPixelFetcher<'bus> {
    pub fn new(memory_bus: &'bus mut bus::Interface) -> Self {
        Self { 
            x: 0,
            memory_bus,
        }
    }
}
impl<'bus> Iterator for FIFOPixelFetcher<'bus> {
    type Item = [Pixel; 8];

    /// The fetcher fetches a row of 8 background or window pixels and queues them up to be mixed with object pixels.
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == graphics::lcd::WINDOW_WIDTH as u8 + 8 /* extra padding */ {
            return None;
        }

        let memory_bus = &self.memory_bus;


        let draw_y = memory_bus[bus::regions::io_registers::lcd::LY];
        let control = lcd::LCDControl::from_bus(memory_bus);

        let win_y = memory_bus[bus::regions::io_registers::lcd::WY];
        let win_x = memory_bus[bus::regions::io_registers::lcd::WX];

        let win_y_seen = draw_y >= win_y;

        // [1. Get the tile]
        let tile = match win_y_seen && control.window_enabled() && self.x >= win_x {
            // Window tile
            true => control.window_tilemap().get_tile(
                memory_bus,
                // TODO: check if this is right
                self.x - win_x,
                draw_y,
                false,
            ),
            // Background tile
            false => {
                let scx = memory_bus[bus::regions::io_registers::lcd::SCX];
                let scy = memory_bus[bus::regions::io_registers::lcd::SCY];
                control.bg_tilemap().get_tile(
                    memory_bus,
                    ((scx / 8) + self.x) & 0x1F, /* Clamps value to 0..32 */
                    self.x.clone().wrapping_add(scy), /* Clamps value to 0..255 */
                    false,
                )
            }
        };


        // TODO: check if this is correct
        let row = tile.get_pixel_row(draw_y % graphics::tiles::Tile::PIXEL_HEIGHT as u8);
        self.x += 1;
        return Some(row.map(|color_id| Pixel::from_bus(memory_bus, color_id)));
        
    }
    
}
