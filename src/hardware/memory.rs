use std::ops::{Index, IndexMut};

use crate::prelude::NibbleFrom16bit;

pub struct MemoryBus([u8; 0xFFFF]);
impl MemoryBus {
    pub fn read8(&self, index: u16) -> &u8 {
        return self.index(index);
    }

    /// Reads u16 from memory.
    ///
    /// `0x{self[index + 1]}_{self[index]}`
    ///
    /// The Game Boy is little endian which means that when you have numbers that are larger than 1 byte, the least significant is stored first in memory and then the most significant byte.
    pub fn read16(&self, index: u16) -> u16 {
        return ((self[index + 1] as u16) << 8) | (self[index] as u16);
    }
}

impl Index<u16> for MemoryBus {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        return self.0.index(index as usize);
    }
}
impl IndexMut<u16> for MemoryBus {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        return self.0.index_mut(index as usize);
    }
}
