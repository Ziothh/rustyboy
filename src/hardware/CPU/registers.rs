/// A struct holding all the CPU registers
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    /// The "flags" register
    ///
    /// When converted to a u8,
    /// the lower four bits of the register are always 0s and
    /// the CPU automatically writes to the upper four bits when certain things happen.
    ///
    /// ```ignore
    ///   ┌-> Carry
    /// ┌-+> Subtraction
    /// | |
    /// 1111 0000
    /// | |
    /// └-+> Zero
    ///   └-> Half Carry
    /// ```
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn read16(&self, register: Reg16) -> u16 {
        use self::Reg16::*;
        match register {
            AF => ((self.a as u16) << 8) | (u8::from(&self.f) as u16),
            BC => ((self.b as u16) << 8) | (self.c as u16),
            DE => ((self.d as u16) << 8) | (self.e as u16),
            HL => ((self.h as u16) << 8) | (self.l as u16),
            // SP => self.sp,
        }
    }

    pub fn write16(&mut self, register: Reg16, value: u16) {
        use self::Reg16::*;
        match register {
            AF => {
                self.a = (value >> 8) as u8;
                self.f = FlagsRegister::from(value as u8)
            }
            BC => {
                self.b = (value >> 8) as u8;
                self.c = value as u8
            }
            DE => {
                self.d = (value >> 8) as u8;
                self.e = value as u8
            }
            HL => {
                self.h = (value >> 8) as u8;
                self.l = value as u8
            } // SP => self.sp = value,
        }
    }
}

#[rustfmt::skip]
impl Registers {
    pub fn af(&self) -> u16 { self.read16(Reg16::AF) }
    pub fn bc(&self) -> u16 { self.read16(Reg16::BC) }
    pub fn de(&self) -> u16 { self.read16(Reg16::DE) }
    pub fn hl(&self) -> u16 { self.read16(Reg16::HL) }
}

#[derive(Clone, Copy, Debug)]
/// Enum of the 8 bit register names on the CPU
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy, Debug)]
/// Enum of the virtual 16 bit register names on the CPU
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    // SP,
}

pub struct FlagsRegister {
    /// Set to `true` if the result of the operation is equal to `0`.
    pub zero: bool,
    /// Set to `true` if the operation was a subtraction.
    pub subtract: bool,
    /// Set to `true` if the operation resulted in an overflow.
    pub carry: bool,
    /// Set to `true` if there is an operation results in an overflow from the lower nibble (the lower four bits) to the upper nibble (the upper four bits).
    ///
    /// ```ignore
    /// // Example
    ///
    ///       lower nibble            lower nibble
    ///         ┌--┐                    ┌--┐
    ///    1000 1111  +   1   ==   1001 0000
    ///    └--┘                    └--┘
    /// upper nibble            upper nibble
    /// ```
    pub half_carry: bool,
}

impl FlagsRegister {
    const CARRY_FLAG_BYTE_POSITION: u8 = 4;
    const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
    const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
    // Left shift info
    const ZERO_FLAG_BYTE_POSITION: u8 = 7;
}

impl From<FlagsRegister> for u8 {
    #[rustfmt::skip]
    fn from(flag: FlagsRegister) -> u8 {
        (flag.zero as u8) << FlagsRegister::ZERO_FLAG_BYTE_POSITION |
        (flag.subtract as u8) << FlagsRegister::SUBTRACT_FLAG_BYTE_POSITION |
        (flag.half_carry as u8) << FlagsRegister::HALF_CARRY_FLAG_BYTE_POSITION |
        (flag.carry as u8) << FlagsRegister::CARRY_FLAG_BYTE_POSITION
    }
}
impl From<&FlagsRegister> for u8 {
    #[rustfmt::skip]
    fn from(flag: &FlagsRegister) -> u8 {
        (flag.zero as u8) << FlagsRegister::ZERO_FLAG_BYTE_POSITION |
        (flag.subtract as u8) << FlagsRegister::SUBTRACT_FLAG_BYTE_POSITION |
        (flag.half_carry as u8) << FlagsRegister::HALF_CARRY_FLAG_BYTE_POSITION |
        (flag.carry as u8) << FlagsRegister::CARRY_FLAG_BYTE_POSITION
    }
}

impl From<&u8> for FlagsRegister {
    fn from(byte: &u8) -> Self { Self::from(*byte) }
}
impl From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> Self::ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> Self::SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> Self::HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> Self::CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}
