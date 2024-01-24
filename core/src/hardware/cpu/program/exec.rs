use super::super::CPU;
use crate::hardware::bus;

impl CPU {
    /// Reads the byte from mem[PC] and increments PC by 1
    pub fn next_byte(&mut self, bus: &bus::Interface) -> u8 {
        let value = bus[self.registers.pc];
        self.registers.pc += 1;

        return value;
    }

    /// Reads the next byte as an `i8`.
    /// Increments the `program_counter` by `1`.
    pub fn next_i8(&mut self, bus: &bus::Interface) -> i8 {
        self.next_byte(bus) as i8
    }

    /// Reads the next 2 bytes and combines them to create a `u16`.
    ///
    /// The `program_counter` is incremented by `2` (`1` for each byte).
    ///
    /// Read bytes:
    ///  - first byte is the lower nibble (lsb)
    ///  - second byte is the upper nibble (msb)
    pub fn next_u16(&mut self, bus: &bus::Interface) -> u16 {
        u16::from_le_bytes([self.next_byte(bus), self.next_byte(bus)])
    }

    pub fn nop(&mut self) -> u8 {
        4
    }

    pub fn load<R, W, T>(&mut self, dest: impl Write<W, T>, src: impl Read<R, T>) -> u8 
        where R: Read<impl Copy, T>
    {
        dest.write(src.read(), );

        4
    }
}


pub trait Read<T, O = u8> {
    fn read(cpu: &CPU, bus: &bus::Interface) -> O;
}
pub trait Write<T, I = u8> {
    fn write(value: I, cpu: &mut CPU, bus: &mut bus::Interface);
}


// trait In<T,>
