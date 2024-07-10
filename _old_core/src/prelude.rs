// This file extends the std library

/// Trait for parsing and storing values in a little endian format.
///
/// Meaning that bytes are stored in memory from least significant to most significant.
pub trait LittleEndian<Bytes> {
    /// Gets the least significant byte from the value.
    fn lsb(&self) -> u8;
    /// Gets the most significant byte from the value.
    fn msb(&self) -> u8;
    /// Constructs `Self` from the `bytes` which should be ordered from least significant to
    /// most significant.
    fn from_bytes(bytes: Bytes) -> Self;
    /// Returns a sequence of bytes, ordered from least significant to most significant, that are parsed from `self`.
    fn as_bytes(&self) -> Bytes;
}

impl LittleEndian<(u8, u8)> for u16 {
    fn msb(&self) -> u8 {
        ((self & 0xFF00) >> 8) as u8
    }

    fn lsb(&self) -> u8 {
        (self & 0xFF) as u8
    }

    fn from_bytes((lsb, msb): (u8, u8)) -> Self {
        ((msb as u16) << 8) | (lsb as u16)
    }

    fn as_bytes(&self) -> (u8, u8) {
        (self.lsb(), self.msb())
    }
}
