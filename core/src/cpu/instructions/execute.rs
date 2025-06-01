use crate::{
    cpu::memory::{Read8, Write8},
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
}
