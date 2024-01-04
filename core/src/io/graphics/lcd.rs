use super::tiles;
use crate::{hardware::bus, utils::bit};

/// The pixel width of the actual game boy screen
pub const WINDOW_WIDTH: usize = 160;
/// The pixel height of the actual game boy screen
pub const WINDOW_HEIGHT: usize = 144;

/// # LCDC: LCD control
/// LCDC is the main LCD Control register. Its bits toggle what elements are displayed on the screen, and how.
struct LCDControl(u8);
impl LCDControl {
    pub fn from_bus(memory_bus: &bus::Interface) -> Self {
        Self(memory_bus[bus::regions::io_registers::lcd::LCDC])
    }
    #[inline]
    fn bit_at(&self, bit_index: u8) -> bool {
        bit::is_set(self.0, bit_index)
    }

    /// # LCDC.7 — LCD enable
    /// This bit controls whether the LCD is on and the PPU is active.
    /// Setting it to 0 turns both off, which grants immediate and full access to VRAM, OAM, etc.
    ///
    /// When the display is disabled the screen is blank, which on DMG is displayed as a white “whiter” than color #0.
    ///
    /// When re-enabling the LCD, the PPU will immediately start drawing again, but the screen will stay blank during the first frame.
    ///
    /// NOTE: SGB not supported atm
    pub fn lcd_enabled(&self) -> bool {
        self.bit_at(7)
    }

    /// # LCDC.6 — Window tile map area
    ///
    /// This bit controls which background map the Window uses for rendering.
    /// When it’s clear (0), the `$9800` tilemap is used, otherwise it’s the `$9C00` one.
    pub fn window_tilemap(&self) -> tiles::Map {
        tiles::Map::from_index(self.bit_at(6) as usize)
    }

    /// # LCDC.5 — Window enable
    /// This bit controls whether the window shall be displayed or not. This bit is overridden on DMG by bit 0 if that bit is clear.
    ///
    /// Changing the value of this register mid-frame triggers a more complex behaviour: see further below (TODO).
    ///
    /// NOTE: SGB not supported atm
    pub fn window_enabled(&self) -> bool {
        self.bit_at(5)
    }

    /// # LCDC.4 BG and Window tile data area
    /// This bit controls which [addressing mode](https://gbdev.io/pandocs/Tile_Data.html#vram-tile-data) the BG and Window use to pick tiles.
    ///
    /// Objects (sprites) aren’t affected by this, and will always use the $8000 addressing mode.
    pub fn tile_addressing_mode(&self) -> tiles::AddresMode {
        match self.bit_at(4) {
            false => tiles::AddresMode::Lower,
            true => tiles::AddresMode::Upper,
        }
    }

    /// # LCDC.3 — BG tile map area
    /// This bit works similarly to LCDC bit 6: if the bit is clear (0), the BG uses tilemap $9800, otherwise tilemap $9C00.
    pub fn bg_tilemap(&self) -> tiles::Map {
        tiles::Map::from_index(self.bit_at(3) as usize)
    }

    /// # LCDC.2 — OBJ size
    /// This bit controls the size of all objects.
    ///
    /// `false`: 8x8 (1 tile)
    /// `true`:  8x16 (2 tiles stacked vertically)
    pub fn double_object_size(&self) -> bool {
        self.bit_at(2)
    }

    /// # LCDC.1 — OBJ enable
    ///
    /// This bit toggles whether objects are displayed or not.
    pub fn object_enabled(&self) -> bool {
        self.bit_at(1)
    }
}


pub struct LCDStatus(u8);
impl LCDStatus {
    pub fn from_bus(memory_bus: &bus::Interface) -> Self {
        Self(memory_bus[bus::regions::io_registers::lcd::STAT])
    }
    #[inline]
    fn bit_at(&self, bit_index: u8) -> bool {
        bit::is_set(self.0, bit_index)
    }
    
    /// # STAT.6 — LYC int select (Read/Write): 
    /// If set, selects the LYC == LY condition for the STAT interrupt.
    pub fn lyc_int_select(&self) -> bool {
        self.bit_at(6)
    }

    /// # STAT[3..=5] — Mode 0..=2 int select (Read/Write): 
    /// If bit x is set, selects the Mode x condition for the STAT interrupt.
    pub fn modes(&self) -> [bool; 3] {
        [
            self.bit_at(3), // Mode 0
            self.bit_at(4), // Mode 1
            self.bit_at(5), // Mode 2
        ]
    }
    

    /// # STAT.2 — LYC == LY (Read-only): 
    /// Set when LY contains the same value as LYC; it is constantly updated.
    pub fn should_ly_compare(&self) -> bool {
        self.bit_at(2)

    }

    // # STAT[1..=0] — PPU mode (Read-only): 
    // Indicates the PPU’s current status.
    pub fn ppu_mode(&self) -> [bool; 2] {
        [self.bit_at(1), self.bit_at(0)]
    }



    
}
