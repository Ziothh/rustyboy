use std::fmt::Display;

#[derive(Default)]
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
}

impl Registers {
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

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "A:{:02X} F:{} B:{:02X} C:{:02X} \
            D:{:02X} E:{:02X} H:{:02X} L:{:02X}",
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l
        )
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Enum of the virtual 16 bit register names on the CPU
pub enum Reg16 {
    BC,
    DE,
    HL,
    SP,
    AF,
}

#[derive(Default)]
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
impl Display for FlagsRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let zero = match self.zero {
            true => 'Z',
            false => 'z',
        };
        let subtract = match self.subtract {
            true => 'S',
            false => 's',
        };
        let half_carry = match self.half_carry {
            true => 'H',
            false => 'h',
        };
        let carry = match self.carry {
            true => 'C',
            false => 'c',
        };

        write!(f, "{zero}{subtract}{half_carry}{carry}")
    }
}

impl FlagsRegister {
    // Left shift info
    const CARRY_FLAG_BYTE_POSITION: u8 = 4;
    const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
    const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
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
    fn from(byte: &u8) -> Self {
        Self::from(*byte)
    }
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
