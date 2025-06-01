use crate::{
    cpu::memory::{Read8, Reg16, Write8},
    GameBoy,
};

impl GameBoy {
    pub(super) fn load<O, I>(&mut self, destination: O, source: I) -> ()
    where
        Self: Write8<O> + Read8<I>,
    {
        let data = self.read(source);
        self.write(destination, data);
    }

    pub(super) fn load16_imm(&mut self, register: Reg16) {
        let value = self.fetch_u16();

        debug_assert!(register != Reg16::AF, "AF Does not support load");

        self.cpu.registers.write16(register, value)
    }
}
