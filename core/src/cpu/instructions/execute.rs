use crate::{
    cpu::memory::{Read8, Reg16, Write8},
    GameBoy,
};

impl GameBoy {
    // --- 8-bit load
    pub(super) fn load<O, I>(&mut self, destination: O, source: I) -> ()
    where
        Self: Write8<O> + Read8<I>,
    {
        let data = self.read(source);
        self.write(destination, data);
    }

    // --- 8-bit bitwise operations
    /// XOR s
    ///
    /// Flags: Z N H C
    ///        * 0 0 0
    pub(super) fn xor<I>(&mut self, source: I) -> ()
    where
        Self: Read8<I>,
    {
        let data = self.read(source);
        self.cpu.registers.a ^= data;
        self.cpu.registers.f.zero = self.cpu.registers.a == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = false;
    }

    pub(super) fn bit<I>(&mut self, bit_idx: u8, source: I) -> ()
    where
        Self: Read8<I>,
    {
        debug_assert!(bit_idx < 8);

        let byte = self.read(source);

        let has_bit = 1 << bit_idx != 0;
        self.cpu.registers.f.zero = !has_bit;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = true;

        self.cycle();
    }

    // --- 16-bit load
    pub(super) fn load16_imm(&mut self, register: Reg16) {
        let value = self.fetch_u16();

        debug_assert!(register != Reg16::AF, "AF Does not support load");

        self.cpu.registers.write16(register, value)
    }

    // --- Misc
    pub(super) fn prefix(&mut self) {
        self.cpu.is_opcode_prefixed = true;
    }
}
