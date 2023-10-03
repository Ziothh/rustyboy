// This file extends the std library

pub trait NibbleFrom16bit<T> {
    fn top_nibble(&self) -> T;
    fn bottom_nibble(&self) -> T;

    fn from_nibbles(top: T, bottom: T) -> Self;
}

impl NibbleFrom16bit<u8> for u16 {
    fn top_nibble(&self) -> u8 {
        ((self & 0xFF00) >> 8) as u8
    }

    fn bottom_nibble(&self) -> u8 {
        (self & 0xFF) as u8
    }

    fn from_nibbles(top: u8, bottom: u8) -> Self {
        ((top as u16) << 8) | (bottom as u16)
    }
}
impl NibbleFrom16bit<u8> for &u16 {
    fn top_nibble(&self) -> u8 {
        ((*self & 0xFF00) >> 8) as u8
    }

    fn bottom_nibble(&self) -> u8 {
        (*self & 0xFF) as u8
    }

    fn from_nibbles(top: u8, bottom: u8) -> Self {
        &u16::from_nibbles(top, bottom)
    }
}
