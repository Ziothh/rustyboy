pub mod tiles;
// pub use tiles::*;

mod colors;
pub use colors::{ColorID, Palette};

mod objects;
pub use objects::{Object, ObjectFlag};

pub mod lcd;
pub use lcd::{LCDControl, LCDStat};


/// Ordered from front to back.
///
/// Some features and behaviors break this abstraction, but it works for the most part.
enum Layer {
    /// The background is composed of a (32x32 byte) tilemap.
    /// A tilemap is a large grid of tiles.
    /// However, tiles aren’t directly written to tilemaps, they merely contain references to the tiles.
    /// This makes reusing tiles very cheap, both in CPU time and in required memory space, and it is the main mechanism that helps work around the paltry 8 KiB of video RAM.
    ///
    /// The background can be made to scroll as a whole, writing to two hardware registers (`SCY` & `SCX`).
    /// This makes scrolling very cheap.
    Background,
    /// The window is sort of a second background layer on top of the background.
    /// It is fairly limited: it has no transparency, it’s always a rectangle
    /// and only the position of the top-left pixel can be controlled (via registers `WX` & `WY`).
    ///
    /// The screen coordinates of the top left corner of the Window are (`WX-7`,`WY`)
    ///
    /// The tiles for the Window are stored in the Tile Data Table.
    /// Both the Background and the Window share the same Tile Data Table.
    ///
    /// Whether the Window is displayed can be toggled using LCDC bit 5.
    /// But in Non-CGB mode this bit is only functional as long as LCDC bit 0 is set.
    /// Enabling the Window makes Mode 3 slightly longer on scanlines where it’s visible.
    /// (See WX and WY for the definition of “Window visibility”.)
    ///
    /// ## Window Internal Line Counter
    /// The window keeps an internal line counter that’s functionally similar to LY, and increments alongside it.
    /// However, it only gets incremented when the window is visible, as described here.
    ///
    /// This line counter determines what window line is to be rendered on the current scanline.
    ///
    /// ## Usage
    /// Possible usage include a fixed status bar in an otherwise scrolling game (e.g. Super Mario Land 2).
    Window,
    /// The objects layer is designed to fill this gap: objects are made of 1 or 2 stacked tiles (8×8 or 8×16 pixels) and can be displayed anywhere on the screen.
    ///
    /// Object tiles have the same format as BG tiles, but they are taken from tile blocks 0 and 1 located at $8000-8FFF and have unsigned numbering.
    ///
    /// NOTE: Several objects can be combined (they can be called metasprites) to draw a larger graphical element, usually called “sprite”.
    Objects,
}

mod background {
    use crate::memory_bus as bus;

    /// (x, y)
    pub fn calc_bottom_right(memory_bus: &bus::Bus) -> (u8, u8) {
        (
            memory_bus[bus::regions::io_registers::lcd::SCX]
                .overflowing_add(159)
                .0, // x
            memory_bus[bus::regions::io_registers::lcd::SCY]
                .overflowing_add(143)
                .0, // y
        )
    }
}

/// The Window is visible (if enabled) when both coordinates are in the ranges WX=0..166, WY=0..143 respectively.
/// Values WX=7, WY=0 place the Window at the top left of the screen, completely covering the background.
mod window {}

