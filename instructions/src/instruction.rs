#[rustfmt::skip]
pub enum Reg8 {
    B, C,
    D, E,
    H, L,
    A, F,
}
impl TryFrom<&str> for Reg8 {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub enum Reg16 {
    BC,
    DE,
    HL,
    AF,
    SP,
}
impl TryFrom<&str> for Reg16 {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub enum Operand {
    Immediate8(u8),
    // Immediate 16-bit value in little endian
    Immediate16LE(u8, u8),

    ImmediateSigned(i8),

    AddrHRAM(u8),
    Addr16LE(u8, u8),
    Addr16(u16),

    Reg8(Reg8),
    Reg16(Reg16),
}

pub struct Operand {
    Value {
        ByteValue {
            u8,
            u16,
        },
        Reg {
            Reg8,
            Reg16,
        }
    }
    direct: bool,
    increment: bool,
    decrement: bool,
}

// Possible reading options
//  - Methods
//      - immediate (next mem bytes / register)
//      - indirect (addr pointer to mem byte)
//          - 16-bit: addr
//          - 8-bit: HRAM addr
//  - Values
//      - u8
//      - i8
//      - u16 (LE)
//      - Reg8
//      - Reg16
//  - Modifiers
//      - Increment ([HL+])
//      - Decrement ([HL-])
//      - +e8 ([SP + e8])

impl Operand {
    fn from_str(str: &str, program: &mut impl Iterator<Item = u8>) -> Self {
        match str {
            "n8" => Operand::Immediate8(program.next().unwrap()),
            "n16" => Operand::Immediate16LE(program.next().unwrap(), program.next().unwrap()),

            "e8" => Operand::ImmediateSigned(program.next().unwrap() as i8),

            "a8" => Operand::AddrHRAM(program.next().unwrap()),
            "a16" => Operand::Addr16LE(program.next().unwrap(), program.next().unwrap()),

            _ if let Ok(reg8) = Reg8::try_from(str) => Operand::Reg8(reg8),
            _ if let Ok(reg16) = Reg16::try_from(str) => Operand::Reg16(reg16),

            _ if let Some(addr) = str.strip_prefix('$') => {
                Operand::Addr16(u16::from_str_radix(addr, 16).expect("Addr to be a valid hex string"))
            }

            _ => unreachable!(r#""{str}" is not a valid operand indicator"#),
        };

        todo!()
    }
}
