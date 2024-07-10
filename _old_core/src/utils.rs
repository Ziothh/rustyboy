/// Utilities to work with bits
pub mod bit {
    /// Checks if the bit at the `bit_index` is set to 1
    ///
    /// TODO: maybe refactor this to a trait if it is needed in multiple data types
    pub fn is_set(byte: u8, bit_index: u8) -> bool {
        byte & (1 << bit_index) != 0
    }
}

