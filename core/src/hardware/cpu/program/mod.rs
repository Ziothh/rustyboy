use super::CPU;
use crate::hardware::bus;

mod decode;
mod exec;

mod unprefixed;

struct Instruction {
    repr: &'static str,
    exec: Box<dyn Fn(&mut CPU, &mut bus::Interface) -> &'static [u8]>,
}

impl Instruction {
    const fn new(repr: &'static str, exec: impl Fn(&mut CPU, &mut bus::Interface) -> &'static [u8]) -> Self {
        Self { repr, exec: Box::new(exec) }
    }
}
