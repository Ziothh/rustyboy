use std::ops;

use crate::prelude::LittleEndian;

pub struct MemoryBus([u8; u16::MAX as usize]);
impl MemoryBus {
    pub fn read8(&self, address: u16) -> u8 {
        return self[address];
    }

    /// Reads `u16` from memory.
    ///
    /// `0x{self[index + 1]}_{self[index]}`
    ///
    /// The Game Boy is **little endian** which means that when you have numbers that are larger than 1 byte, 
    /// the bytes are stored in memory from least significant to most significant.
    ///
    /// # Panics
    /// This function panics when reading on `address = u16::MAX` when reading the `msb` because
    /// it'll read on `mem[address + 1]` which is out of bounds.
    pub fn read16(&self, address: u16) -> u16 {
        return u16::from_bytes((self[address], self[address + 1]));
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
