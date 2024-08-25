use bitflags::bitflags;

use super::tiles::{TileAddressMode, TileMap};
use crate::{memory_bus as bus, ppu, utils::bit};

/// The pixel width of the actual game boy screen
pub const WINDOW_WIDTH: usize = 160;
/// The pixel height of the actual game boy screen
pub const WINDOW_HEIGHT: usize = 144;

bitflags! {
    /// # LCDC: LCD control
    /// LCDC is the main LCD Control register. Its bits toggle what elements are displayed on the screen, and how.
    pub struct LCDControl: u8 {
        /// # LCDC.7 — LCD & PPU enable
        /// This bit controls whether the LCD is on and the PPU is active.
        /// Setting it to 0 turns both off, which grants immediate and full access to VRAM, OAM, etc.
        ///
        /// When the display is disabled the screen is blank, which on DMG is displayed as a white “whiter” than color #0.
        ///
        /// When re-enabling the LCD, the PPU will immediately start drawing again, but the screen will stay blank during the first frame.
        ///
        /// NOTE: SGB not supported atm
        const LCD_ENABLED = 1 << 7;


        /// # LCDC.6 — Window tile map area
        ///
        /// This bit controls which background map the Window uses for rendering.
        /// When it’s clear (0), the `$9800` tilemap is used, otherwise it’s the `$9C00` one.
        const WINDOW_TILEMAP_AREA = 1 << 6;
        /// # LCDC.5 — Window enable
        /// This bit controls whether the window shall be displayed or not. This bit is overridden on DMG by bit 0 if that bit is clear.
        ///
        /// Changing the value of this register mid-frame triggers a more complex behaviour: see further below (TODO).
        ///
        /// NOTE: SGB not supported atm
        const WINDOW_ENABLED = 1 << 5;

        /// # LCDC.4 BG and Window tile data area
        /// This bit controls which [addressing mode](https://gbdev.io/pandocs/Tile_Data.html#vram-tile-data) the BG and Window use to pick tiles.
        ///
        /// Objects (sprites) aren’t affected by this, and will always use the $8000 addressing mode.
        const TILE_DATA_AREA = 1 << 4;

        /// # LCDC.3 — BG tile map area
        /// This bit works similarly to LCDC bit 6: if the bit is clear (0), the BG uses tilemap $9800, otherwise tilemap $9C00.
        const BACKGROUND_TILEMAP_AREA = 1 << 3;

        /// # LCDC.2 — OBJ size
        /// This bit controls the size of all objects.
        ///
        /// `false`: 8x8 (1 tile)
        /// `true`:  8x16 (2 tiles stacked vertically)
        const OBJECT_SIZE = 1 << 2;

        /// # LCDC.1 — OBJ enable
        ///
        /// This bit toggles whether objects are displayed or not.
        const OBJECT_ENABLED = 1 << 1;

        /// # LCDC.0 — BG and Window display
        /// ## Non-CGB Mode (DMG, SGB and CGB in compatibility mode):
        /// When Bit 0 is cleared, both background and window become blank (white), and the Window Display Bit is ignored in that case.
        /// Only objects may still be displayed (if enabled in Bit 1).
        ///
        /// ## CGB Mode
        /// This means something different on CGB but is not supported atm.
        const WIN_BG_ENABLED = 1 << 0;
    }
}
impl LCDControl {
    pub fn window_tilemap_area(&self) -> TileMap {
        match self.contains(Self::WINDOW_TILEMAP_AREA) {
            false => TileMap::Map0,
            true => TileMap::Map1,
        }
    }
    pub fn bg_tilemap(&self) -> TileMap {
        match self.contains(Self::BACKGROUND_TILEMAP_AREA) {
            false => TileMap::Map0,
            true => TileMap::Map1,
        }
    }

    pub fn window_enabled(&self) -> bool {
        return self.contains(Self::WINDOW_ENABLED) && self.contains(Self::WIN_BG_ENABLED);
    }

    pub fn tile_addressing_mode(&self) -> TileAddressMode {
        match self.contains(Self::TILE_DATA_AREA) {
            false => TileAddressMode::Lower,
            true => TileAddressMode::Upper,
        }
    }
}

bitflags! {
    pub struct LCDStat: u8 {
        /// # STAT.6 — LYC int select (Read/Write):
        /// If set, selects the LYC == LY condition for the STAT interrupt.
        const LYC_INT_SELECT = 1 << 6;

        /// # STAT.5 — Mode 2 int select (Read/Write): 
        /// If set, selects the Mode 2 condition for the STAT interrupt.
        const ACCESS_OAM_INT_SELECT = 1 << 5;

        /// # STAT.4 — Mode 1 int select (Read/Write): 
        /// If set, selects the Mode 2 condition for the STAT interrupt.
        const VBLANK_INT_SELECT = 1 << 4;

        /// # STAT.3 — Mode 1 int select (Read/Write): 
        /// If set, selects the Mode 2 condition for the STAT interrupt.
        const HBLANK_INT_SELECT = 1 << 3;
    }
}
