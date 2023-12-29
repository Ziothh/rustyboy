use std::ops;

use crate::prelude::NibbleFrom16bit;

pub struct MemoryBus([u8; u16::MAX as usize]);
impl MemoryBus {
    pub fn read8(&self, index: u16) -> u8 {
        return self[index];
    }

    /// Reads `u16` from memory.
    ///
    /// `0x{self[index + 1]}_{self[index]}`
    ///
    /// The Game Boy is **little endian** which means that when you have numbers that are larger than 1 byte, 
    /// the bytes are stored in memory from least significant to most significant.
    pub fn read16(&self, index: u16) -> u16 {
        return u16::from_nibbles(self[index], self[index + 1]);
        // ((self[index + 1] as u16) << 8) | (self[index] as u16);
    }
}

impl ops::Index<u16> for MemoryBus {
    type Output = u8;

    #[inline]
    fn index(&self, index: u16) -> &Self::Output {
        return self.0.index(index as usize);
    }
}
impl ops::IndexMut<u16> for MemoryBus {
    #[inline]
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        return self.0.index_mut(index as usize);
    }
}
