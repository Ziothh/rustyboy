use std::ops;

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
    /// ```text
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
    /// Stack Pointer
    pub sp: u16,
    /// Program counter/pointer
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister {
                zero: false,
                carry: false,
                subtract: false,
                half_carry: false,
            },
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn read16(&self, register: Reg16) -> u16 {
        use self::Reg16::*;
        match register {
            AF => ((self.a as u16) << 8) | (u8::from(&self.f) as u16),
            BC => ((self.b as u16) << 8) | (self.c as u16),
            DE => ((self.d as u16) << 8) | (self.e as u16),
            HL => ((self.h as u16) << 8) | (self.l as u16),
            // TODO: find a way to better do this
            SP => unreachable!(),
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
            }
            SP => unreachable!(),
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

impl ops::Index<Reg8> for Registers {
    type Output = u8;
    fn index(&self, register: Reg8) -> &Self::Output {
        return &match register {
            Reg8::A => self.a,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::F => self.f.into(),
            Reg8::H => self.h,
            Reg8::L => self.l,
        };
    }
}
impl ops::Index<Reg16> for Registers {
    type Output = u16;
    fn index(&self, register: Reg16) -> &Self::Output {
        return &match register {
            Reg16::SP => self.sp,
            reg => self.read16(reg),
        };
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
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

// impl TryFrom<u8> for Reg8 {
//     type Error = String;
//
//     fn try_from(byte: u8) -> Result<Self, Self::Error> {
//         match byte {
//             0x00 => Ok(Reg8::A),
//             0x01 => Ok(Reg8::B),
//             0x02 => Ok(Reg8::C),
//             0x03 => Ok(Reg8::D),
//             0x04 => Ok(Reg8::E),
//             0x05 => Ok(Reg8::H),
//             0x06 => Ok(Reg8::L),
//             _ => Err(format!("Byte {byte:X} is not a valid register identifier")),
//         }
//     }
// }
// impl TryFrom<&u8> for Reg8 {
//     type Error = String;
//
//     fn try_from(byte: &u8) -> Result<Self, Self::Error> {
//         Self::try_from(*byte)
//     }
// }

#[derive(Clone, Copy, Debug)]
/// Enum of the virtual 16 bit register names on the CPU
pub enum Reg16 {
    BC,
    DE,
    HL,
    SP,
    AF,
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
    /// ```text
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
    // Left shift info
    const CARRY_FLAG_BYTE_POSITION: u8 = 4;
    const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
    const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
    const ZERO_FLAG_BYTE_POSITION: u8 = 7;

    #[rustfmt::skip]
    fn as_byte(&self) -> u8 {
        (self.zero as u8) << FlagsRegister::ZERO_FLAG_BYTE_POSITION |
        (self.subtract as u8) << FlagsRegister::SUBTRACT_FLAG_BYTE_POSITION |
        (self.half_carry as u8) << FlagsRegister::HALF_CARRY_FLAG_BYTE_POSITION |
        (self.carry as u8) << FlagsRegister::CARRY_FLAG_BYTE_POSITION
    }
}

impl From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        flag.as_byte()
    }
}
// impl From<&FlagsRegister> for u8 {
//     #[rustfmt::skip]
//     fn from(flag: &FlagsRegister) -> u8 {
//         flag.as_byte()
//     }
// }

// impl From<&u8> for FlagsRegister {
//     fn from(byte: &u8) -> Self {
//         Self::from(*byte)
//     }
// }
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
