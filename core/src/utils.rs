/// Utilities to work with bits
pub mod bit {
    /// Checks if the bit at the `bit_idx` is set to 1
    ///
    /// TODO: maybe refactor this to a trait if it is needed in multiple data types
    #[inline(always)]
    pub fn is_set(byte: u8, bit_idx: u8) -> bool {
        debug_assert!(bit_idx < 8);

        byte & (1 << bit_idx) != 0
    }
}

/// 1 KiB
pub const KIBI_BYTE: usize = 1024;

/// When a read isn't allowed or supported, the Game Boy returns undefined data.
/// This is mostly equal to `0xFF`
pub const UNDEFINED_READ: u8 = 0xFF;
