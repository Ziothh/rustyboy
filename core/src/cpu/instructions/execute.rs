use super::JumpCondition;
use crate::{
    cpu::memory::{CpuRead, CpuWrite, Reg16},
    utils::bit,
    GameBoy,
};

impl GameBoy {
    // --- 8-bit & 16-bit load
    pub(super) fn load<O, I>(&mut self, destination: O, source: I) -> ()
    where
        Self: CpuWrite<O, u8> + CpuRead<I, u8>,
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
        Self: CpuRead<I, u8>,
    {
        let data = self.read(source);
        self.cpu.registers.a ^= data;
        self.cpu.registers.f.zero = self.cpu.registers.a == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = false;
    }

    /// BIT b, s
    ///
    /// Flags: Z N H C
    ///        * 0 1 -
    pub(super) fn bit<I>(&mut self, bit_idx: u8, source: I) -> ()
    where
        Self: CpuRead<I, u8>,
    {
        debug_assert!(bit_idx < 8);

        let byte = self.read(source);

        self.cpu.registers.f.zero = !bit::is_set(byte, bit_idx);
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = true;

        self.cycle();
    }

    /// SET b, s
    ///
    /// Flags: Z N H C
    ///        - - - -
    pub(super) fn set<IO: Copy>(&mut self, bit_idx: u8, io: IO) -> ()
    where
        Self: CpuRead<IO, u8> + CpuWrite<IO, u8>,
    {
        debug_assert!(bit_idx < 8);

        let byte = self.read(io);
        self.write(io, byte & (1 << bit_idx));

        self.cycle();
    }

    /// SLA s
    ///
    /// Flags: Z N H C
    ///        * 0 0 *
    pub(super) fn sla<IO: Copy>(&mut self, io: IO) -> ()
    where
        Self: CpuRead<IO, u8> + CpuWrite<IO, u8>,
    {
        let (data, has_overflown) = self.read(io).overflowing_shl(1);
        self.write(io, data);

        self.cpu.registers.f.zero = data == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = has_overflown;

        self.cycle();
    }


    // --- 16-bit load
    pub(super) fn load16<O, I>(&mut self, destination: O, source: I) -> ()
    where
        Self: CpuWrite<O, u16> + CpuRead<I, u16>,
    {
        let data = self.read(source);
        self.write(destination, data);
    }
    pub(super) fn load16_hl_sp_e(&mut self) {
        let e = self.fetch_i8();
        let e_abs = e.abs() as u16;

        let (data, has_overflown) = match e.is_positive() {
            true => self.cpu.pc.overflowing_add(e_abs),
            false => self.cpu.pc.overflowing_sub(e_abs),
        };
        self.write(Reg16::HL, data);
        self.cpu.registers.f.zero = false;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = u16_test_addition_bit_carry(3, self.cpu.pc, e_abs);
        self.cpu.registers.f.carry = u16_test_addition_bit_carry(7, self.cpu.pc, e_abs);
    }

    // --- Control flow
    /// JR e
    ///
    /// Flags: Z N H C
    ///        - - - -
    pub(super) fn jr(&mut self) {
        let offset = self.fetch_i8();
        dbg!(offset);
        self.cpu.pc = u16_add_i8(self.cpu.pc, offset);
    }

    /// JR cc, e
    ///
    /// Flags: Z N H C
    ///        - - - -
    pub(super) fn jr_cc(&mut self, condition: JumpCondition) {
        if !match condition {
            JumpCondition::NotZero => !self.cpu.registers.f.zero,
            JumpCondition::Zero => self.cpu.registers.f.zero,
            JumpCondition::NotCarry => !self.cpu.registers.f.carry,
            JumpCondition::Carry => self.cpu.registers.f.carry,
        } {
            return;
        }

        self.jr();
    }

    // --- Misc
    pub(super) fn prefix(&mut self) {
        self.cpu.is_opcode_prefixed = true;
    }
}

/// Tests if addition results in a carry from the specified bit.
/// Does not support overflow, so cannot be used to check carry from the leftmost bit
#[inline(always)]
fn u16_test_addition_bit_carry(bit_idx: u16, a: u16, b: u16) -> bool {
    debug_assert!(bit_idx < u16::BITS as u16, "Last bit can not be checked");

    let mask: u16 = 1 << bit_idx;

    // Sets all rightmost 0-bits to 1
    // e.g. 1010 1000 -> 1010 1111
    //
    // Equivalent to Intel BMI1 instruction BLSMSK
    //
    // Examples
    //   bit=0 -> 0000 0001
    //   bit=3 -> 0000 1111
    //   bit=6 -> 0111 1111
    let mask = mask | mask.wrapping_sub(1);

    (a & mask) + (b & mask) > mask
}

fn u16_add_i8(addr: u16, offset: i8) -> u16 {
    let abs_offset = offset.abs() as u16;

    match offset.signum() {
        1 => addr.wrapping_add(abs_offset),
        -1 => addr.wrapping_sub(abs_offset),
        0 => addr,
        _ => unreachable!(),
    }
}
