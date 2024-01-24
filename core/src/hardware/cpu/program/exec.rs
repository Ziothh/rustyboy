use super::{super::CPU, unprefixed::JumpCondition};
use crate::hardware::{
    bus,
    cpu::memory::{In, Out, Reg16},
};

// Utils
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

    /// Reads the next 2 little endian bytes and combines them to create a `u16`.
    ///
    /// The `program_counter` is incremented by `2` (`1` for each byte).
    ///
    /// Read bytes:
    ///  - first byte is the lower nibble (lsb)
    ///  - second byte is the upper nibble (msb)
    pub fn next_u16(&mut self, bus: &bus::Interface) -> u16 {
        u16::from_le_bytes([self.next_byte(bus), self.next_byte(bus)])
    }
}

// [Misc]
impl CPU {
    pub fn prefix(&mut self) -> &mut Self {
        self.is_prefixed = true;
        // TODO: check if this prefix is removed when reading prefixed opcodes
        return self;
    }

    pub fn nop(&mut self) -> &mut Self {
        self
    }

    pub fn stop(&mut self) -> &mut Self {
        todo!();
        self
    }

    pub fn halt(&mut self) -> &mut Self {
        todo!();
        self
    }

    /// # Decimal Adjust Accumulator
    /// Decimal Adjust Accumulator to get a correct binary-coded decimal (BCD) representation after an arithmetic instruction.
    #[rustfmt::skip]
    pub fn daa(&mut self) -> &mut Self {
        let mut acc = self.registers.a;

        let mut adjust = if self.registers.f.carry { 0x60 } else { 0x00 };
        if self.registers.f.half_carry { adjust |= 0x06; };
        if !self.registers.f.subtract {
            if acc & 0x0F > 0x09 { adjust |= 0x06; };
            if acc > 0x99 { adjust |= 0x60; };

            acc = acc.wrapping_add(adjust);
        } else {
            acc = acc.wrapping_sub(adjust);
        }

        self.registers.f.carry = adjust >= 0x60;
        self.registers.f.half_carry = false;
        self.registers.f.zero = acc == 0;

        self.registers.a = acc;

        return self;
    }

    /// # Complement Accumulator (A = ~A)
    /// Inverts the bits of the A register
    pub fn cpl(&mut self) -> &mut Self {
        self.registers.a = !self.registers.a;

        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;

        return self;
    }

    /// # Set Carry Flag
    /// N = 0
    /// H = 0
    /// C = 1
    pub fn scf(&mut self) -> &mut Self {
        self.registers.f.carry = true;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;

        return self;
    }
    /// # Complement Carry Flag
    /// N = 0
    /// H = 0
    /// C = !C
    pub fn ccf(&mut self) -> &mut Self {
        self.registers.f.carry = !self.registers.f.carry;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;

        return self;
    }
}

// [Jump]
impl CPU {
    /// # Jump (with condition)
    pub fn jp(&mut self, condition: JumpCondition, addr: bus::Addr) -> &mut Self {
        let should_jump = match condition {
            JumpCondition::Always => true,
            JumpCondition::Zero => self.registers.f.zero,
            JumpCondition::NotZero => !self.registers.f.zero,
            JumpCondition::Carry => self.registers.f.carry,
            JumpCondition::NotCarry => !self.registers.f.carry,
        };

        if should_jump {
            self.registers.stack.pointer = addr;
        }

        return self;
    }
    /// # Jump relative (with condition)
    pub fn jr(&mut self, condition: JumpCondition, offset: i8) -> &mut Self {
        self.jp(
            condition,
            self.registers.pc.wrapping_add_signed(offset as i16),
        )
    }

    /// # Call function (with condition)
    pub fn call(&mut self, condition: JumpCondition, addr: bus::Addr) -> &mut Self {
        todo!();
        return self;
    }
}

// [Load]
impl CPU {
    /// LD: Load
    pub fn ld<Dest: Copy, Src: Copy, T>(
        &mut self,
        bus: &mut bus::Interface,
        dest: Dest,
        src: Src,
    ) -> &mut Self
    where
        Self: Out<Src, T> + In<Dest, T>,
    {
        self.write(dest, self.read(src, bus), bus)
    }

    /// # LD HL, SP + e8
    pub fn ld_hl_spe8(&mut self, bus: &mut bus::Interface) -> &mut Self {
        self.registers.write16(
            Reg16::HL,
            self.registers
                .sp()
                .wrapping_add_signed(self.next_i8(bus) as i16),
        );

        return self;
    }
}

// [Arithmatic operations]
impl CPU {
    // [ADD]
    /// # Add the 8-bit value and the carry flag to `A`.
    pub fn adc<Src: Copy>(&mut self, bus: &mut bus::Interface, src: Src) -> &mut Self
    where
        Self: Out<Src, u8>,
    {
        struct Res {
            acc: u8,
            carry: bool,
            half_carry: bool,
        }

        let res = {
            let (v_src, c_old) = (self.read(src, bus), self.registers.f.carry);
            let (v1, c1) = self.registers.a.overflowing_add(c_old as u8);
            let (v2, c2) = v1.overflowing_add(v_src);

            Res {
                acc: v2,
                carry: c1 | c2,
                half_carry: (self.registers.a & 0xF) + (v_src & 0xF) + c_old as u8 > 0xF,
            }
        };

        self.registers.a = res.acc;
        self.registers.f.zero = res.acc == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = res.half_carry;
        self.registers.f.carry = res.carry;

        return self;
    }
    /// Add 8-bit value (without carry flag) to `A`
    pub fn add<Src: Copy>(&mut self, bus: &mut bus::Interface, src: Src) -> &mut Self
    where
        Self: Out<Src, u8>,
    {
        self.registers.f.carry = false;
        self.adc(bus, src)
    }
    /// Add 16-bit register to `HL`
    /// ```text
    /// Z N H C
    /// - 0 * *
    /// ```
    pub fn add_hl<Dest: Copy, Src: Copy>(
        &mut self,
        bus: &mut bus::Interface,
        register: Reg16,
    ) -> &mut Self {
        let old = self.read(Reg16::HL, bus);
        let (new, carry) = old.overflowing_add(self.read(register, bus));

        self.registers.f.subtract = false;
        self.registers.f.half_carry = (old & (1 << 11)) != 0;
        self.registers.f.carry = carry;

        return self.write(Reg16::HL, new, bus);
    }

    // [SUB]
    /// # Subtract the 8-bit value and the carry flag from `A`.
    pub fn sbc<Src: Copy>(&mut self, bus: &mut bus::Interface, src: Src) -> &mut Self
    where
        Self: Out<Src, u8>,
    {
        struct Res {
            acc: u8,
            carry: bool,
            half_carry: bool,
        }

        let res = {
            let (v_src, c_old) = (self.read(src, bus), self.registers.f.carry);
            let (v1, c1) = self.registers.a.overflowing_sub(c_old as u8);
            let (v2, c2) = v1.overflowing_sub(v_src);

            Res {
                acc: v2,
                carry: c1 | c2,
                half_carry: (self.registers.a & 0xF)
                .wrapping_sub(v_src & 0xF)
                .wrapping_sub(c_old as u8)
                & (0xF + 1)
                != 0,
            }
        };

        self.registers.a = res.acc;
        self.registers.f.zero = res.acc == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = res.half_carry;
        self.registers.f.carry = res.carry;

        return self;
    }
    /// Subtract 8-bit value from A (and ignore carry flag)
    pub fn sub<Src: Copy>(&mut self, bus: &mut bus::Interface, src: Src) -> &mut Self
    where
        Self: Out<Src, u8>,
    {
        self.registers.f.carry = false;
        self.sbc(bus, src)
    }
    // /// Add 16-bit register to `HL`
    // /// ```text
    // /// Z N H C
    // /// - 0 * *
    // /// ```
    // pub fn sub_hl<Dest: Copy, Src: Copy>(
    //     &mut self,
    //     bus: &mut bus::Interface,
    //     register: Reg16,
    // ) -> &mut Self {
    //     todo!();
    //     let old = self.read(Reg16::HL, bus);
    //     let (new, carry) = old.overflowing_add(self.read(register, bus));
    //
    //     self.registers.f.subtract = false;
    //     self.registers.f.half_carry = (old & (1 << 11)) != 0;
    //     self.registers.f.carry = carry;
    //
    //     return self.write(Reg16::HL, new, bus);
    // }

    // [INC]
    /// Increment 8-bit value
    pub fn inc<T: Copy>(&mut self, bus: &mut bus::Interface, register: T) -> &mut Self
    where
        Self: Out<T, u8> + In<T, u8>,
    {
        let old = self.read(register, bus);

        let new = old.wrapping_add(1);

        self.write(register, new, bus);
        self.registers.f.zero = new == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (old & 0b1111) == 0b1111;

        return self;
    }
    /// Increment 16-bit value
    pub fn inc16<T: Copy>(&mut self, bus: &mut bus::Interface, register: T) -> &mut Self
    where
        Self: Out<T, u16> + In<T, u16>,
    {
        self.write(register, self.read(register, bus).wrapping_add(1), bus)
    }

    // [DEC]
    /// Decrement 8-bit value
    pub fn dec<T: Copy>(&mut self, bus: &mut bus::Interface, register: T) -> &mut Self
    where
        Self: Out<T, u8> + In<T, u8>,
    {
        let old = self.read(register, bus);

        let new = old.wrapping_sub(1);

        self.write(register, new, bus);
        self.registers.f.zero = new == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (old & 0b1111) == 0;

        return self;
    }
    /// Decrement 16-bit value
    pub fn dec16<T: Copy>(&mut self, bus: &mut bus::Interface, register: T) -> &mut Self
    where
        Self: Out<T, u16> + In<T, u16>,
    {
        self.write(register, self.read(register, bus).wrapping_sub(1), bus)
    }
}

// [Bitwise operations]
impl CPU {
    /// # Rotate 8-bit value left
    /// ```text
    /// Z N H C
    /// * 0 0 *
    /// ```
    pub fn rl<T: Copy>(&mut self, bus: &mut bus::Interface, register: T) -> &mut Self
    where
        Self: Out<T, u8> + In<T, u8>,
    {
        let (new, carry) = self.read(register, bus).overflowing_shl(1);
        self.write(register, new | self.registers.f.carry as u8, bus);

        self.registers.f.zero = new == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;

        return self;
    }
    /// # Rotate 8-bit value left with carry
    /// ```text
    /// Z N H C
    /// * 0 0 *
    /// ```
    pub fn rlc<T: Copy>(&mut self, bus: &mut bus::Interface, register: T) -> &mut Self
    where
        Self: Out<T, u8> + In<T, u8>,
    {
        // Replace the old carry flag so that it will be used in self.rl
        self.registers.f.carry = self.read(register, bus) & (0b1000_0000) != 0;
        self.rl(bus, register)
    }
    /// # Rotate 8-bit value right
    /// ```text
    /// Z N H C
    /// * 0 0 *
    /// ```
    pub fn rr<T: Copy>(&mut self, bus: &mut bus::Interface, register: T) -> &mut Self
    where
        Self: Out<T, u8> + In<T, u8>,
    {
        let (new, carry) = self.read(register, bus).overflowing_shr(1);
        self.write(register, new | self.registers.f.carry as u8, bus);

        self.registers.f.zero = new == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;

        return self;
    }
    /// # Rotate 8-bit value right with carry
    /// ```text
    /// Z N H C
    /// * 0 0 *
    /// ```
    pub fn rrc<T: Copy>(&mut self, bus: &mut bus::Interface, register: T) -> &mut Self
    where
        Self: Out<T, u8> + In<T, u8>,
    {
        // Replace the old carry flag so that it will be used in self.rr
        self.registers.f.carry = self.read(register, bus) & (0b0000_0001) != 0;
        self.rr(bus, register)
    }
}
