// This file extends the std library

pub trait NibbleFrom16bit<T, U> {
    fn top_nibble(&self) -> T;
    fn bottom_nibble(&self) -> T;

    /// @param `lsb` the least significant byte
    /// @param `msb` the most significant byte
    fn from_nibbles(lsb: T, msb: T) -> U;
}

impl NibbleFrom16bit<u8, u16> for u16 {
    fn top_nibble(&self) -> u8 {
        ((self & 0xFF00) >> 8) as u8
    }

    fn bottom_nibble(&self) -> u8 {
        (self & 0xFF) as u8
    }

    fn from_nibbles(lsb: u8, msb: u8) -> u16 {
        ((msb as u16) << 8) | (lsb as u16)
    }
}
impl NibbleFrom16bit<u8, u16> for &u16 {
    fn top_nibble(&self) -> u8 {
        ((*self & 0xFF00) >> 8) as u8
    }

    fn bottom_nibble(&self) -> u8 {
        (*self & 0xFF) as u8
    }

    fn from_nibbles(lsb: u8, msb: u8) -> u16 {
        u16::from_nibbles(lsb, msb)
    }
}
