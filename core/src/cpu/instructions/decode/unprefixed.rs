use crate::cpu::Program;

use super::super::super::{
    instructions::{ArithmeticTarget, JumpAddress, JumpCondition, LoadTarget},
    memory::{Address, Reg16, Reg8},
    Instruction,
};

impl Instruction {
    pub fn try_from_opcode_unprefixed(byte: u8, program: &mut Program) -> Result<Self, String> {
        match byte {
            /* [ADC] */
            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x88 => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::B))),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x89 => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::C))),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x8A => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::D))),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x8B => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::E))),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x8C => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::H))),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x8D => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::L))),

            // Instruction: ADC
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 0 H C
            0x8E => Ok(Instruction::ADC(ArithmeticTarget::Indirect)),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x8F => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: ADC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0xCE => Ok(Instruction::ADC(ArithmeticTarget::Immediate {
                value: program.next_byte(),
            })),

            /* [ADD] */
            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate),
            //     BC (immediate)
            // Flags: Z N H C
            //        - 0 H C
            0x09 => Ok(Instruction::ADD16(Reg16::BC)),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate),
            //     DE (immediate)
            // Flags: Z N H C
            //        - 0 H C
            0x19 => Ok(Instruction::ADD16(Reg16::DE)),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate),
            //     HL (immediate)
            // Flags: Z N H C
            //        - 0 H C
            0x29 => Ok(Instruction::ADD16(Reg16::HL)),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate),
            //     SP (immediate)
            // Flags: Z N H C
            //        - 0 H C
            0x39 => Ok(Instruction::ADD16_SP),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x80 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::B))),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x81 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::C))),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x82 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::D))),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x83 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::E))),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x84 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::H))),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x85 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::L))),

            // Instruction: ADD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 0 H C
            0x86 => Ok(Instruction::ADD(ArithmeticTarget::Indirect)),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x87 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: ADD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0xC6 => Ok(Instruction::ADD(ArithmeticTarget::Immediate {
                value: program.next_byte(),
            })),

            // Instruction: ADD (immediate)
            // { bytes: 2, cycles: 16 }
            // Operands:
            //     SP (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        0 0 H C
            0xE8 => Ok(Instruction::ADD_SP_i8(program.next_i8())),

            /* [AND] */
            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA0 => Ok(Instruction::AND(ArithmeticTarget::Reg8(Reg8::B))),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA1 => Ok(Instruction::AND(ArithmeticTarget::Reg8(Reg8::C))),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA2 => Ok(Instruction::AND(ArithmeticTarget::Reg8(Reg8::D))),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA3 => Ok(Instruction::AND(ArithmeticTarget::Reg8(Reg8::E))),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA4 => Ok(Instruction::AND(ArithmeticTarget::Reg8(Reg8::H))),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA5 => Ok(Instruction::AND(ArithmeticTarget::Reg8(Reg8::L))),

            // Instruction: AND
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 0 1 0
            0xA6 => Ok(Instruction::AND(ArithmeticTarget::Indirect)),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA7 => Ok(Instruction::AND(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: AND (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xE6 => Ok(Instruction::AND(ArithmeticTarget::Immediate {
                value: program.next_byte(),
            })),

            // Instruction: CALL (immediate)
            // { bytes: 3, cycles: [24,12] }
            // Operands:
            //     NZ (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC4 => Ok(Instruction::CALL {
                address: program.next_u16(),
                condition: JumpCondition::NotZero,
            }),

            // Instruction: CALL (immediate)
            // { bytes: 3, cycles: [24,12] }
            // Operands:
            //     Z (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCC => Ok(Instruction::CALL {
                address: program.next_u16(),
                condition: JumpCondition::Zero,
            }),

            // Instruction: CALL (immediate)
            // { bytes: 3, cycles: 24 }
            // Operands:
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCD => Ok(Instruction::CALL {
                address: program.next_u16(),
                condition: JumpCondition::Always,
            }),

            // Instruction: CALL (immediate)
            // { bytes: 3, cycles: [24,12] }
            // Operands:
            //     NC (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD4 => Ok(Instruction::CALL {
                address: program.next_u16(),
                condition: JumpCondition::NotCarry,
            }),

            // Instruction: CALL (immediate)
            // { bytes: 3, cycles: [24,12] }
            // Operands:
            //     C (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xDC => Ok(Instruction::CALL {
                address: program.next_u16(),
                condition: JumpCondition::NotCarry,
            }),

            // Instruction: CCF (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - 0 0 C
            0x3F => Ok(Instruction::CCF),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xB8 => Ok(Instruction::CP(ArithmeticTarget::Reg8(Reg8::B))),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xB9 => Ok(Instruction::CP(ArithmeticTarget::Reg8(Reg8::C))),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xBA => Ok(Instruction::CP(ArithmeticTarget::Reg8(Reg8::D))),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xBB => Ok(Instruction::CP(ArithmeticTarget::Reg8(Reg8::E))),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xBC => Ok(Instruction::CP(ArithmeticTarget::Reg8(Reg8::H))),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xBD => Ok(Instruction::CP(ArithmeticTarget::Reg8(Reg8::L))),

            // Instruction: CP
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 1 H C
            0xBE => Ok(Instruction::CP(ArithmeticTarget::Indirect)),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        1 1 0 0
            0xBF => Ok(Instruction::CP(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: CP (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xFE => Ok(Instruction::CP(ArithmeticTarget::Immediate {
                value: program.next_byte(),
            })),

            // Instruction: CPL (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - 1 1 -
            0x2F => Ok(Instruction::CPL),

            // Instruction: DAA (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        Z - 0 C
            0x27 => Ok(Instruction::DAA),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x05 => Ok(Instruction::DEC(ArithmeticTarget::Reg8(Reg8::B))),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     BC (immediate)
            // Flags: Z N H C
            //        - - - -
            0x0B => Ok(Instruction::DEC16(Reg16::BC)),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x0D => Ok(Instruction::DEC(ArithmeticTarget::Reg8(Reg8::C))),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x15 => Ok(Instruction::DEC(ArithmeticTarget::Reg8(Reg8::D))),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     DE (immediate)
            // Flags: Z N H C
            //        - - - -
            0x1B => Ok(Instruction::DEC16(Reg16::DE)),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x1D => Ok(Instruction::DEC(ArithmeticTarget::Reg8(Reg8::E))),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x25 => Ok(Instruction::DEC(ArithmeticTarget::Reg8(Reg8::H))),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0x2B => Ok(Instruction::DEC16(Reg16::HL)),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x2D => Ok(Instruction::DEC(ArithmeticTarget::Reg8(Reg8::L))),

            // Instruction: DEC
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     HL
            // Flags: Z N H C
            //        Z 1 H -
            0x35 => Ok(Instruction::DEC(ArithmeticTarget::Indirect)),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     SP (immediate)
            // Flags: Z N H C
            //        - - - -
            0x3B => Ok(Instruction::DEC16_SP),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x3D => Ok(Instruction::DEC(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: DI (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0xF3 => Ok(Instruction::DI),

            // Instruction: EI (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0xFB => Ok(Instruction::EI),

            // Instruction: HALT (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0x76 => Ok(Instruction::HALT),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     BC (immediate)
            // Flags: Z N H C
            //        - - - -
            0x03 => Ok(Instruction::INC16(Reg16::BC)),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x04 => Ok(Instruction::INC(ArithmeticTarget::Reg8(Reg8::B))),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x0C => Ok(Instruction::INC(ArithmeticTarget::Reg8(Reg8::C))),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     DE (immediate)
            // Flags: Z N H C
            //        - - - -
            0x13 => Ok(Instruction::INC16(Reg16::DE)),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x14 => Ok(Instruction::INC(ArithmeticTarget::Reg8(Reg8::D))),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x1C => Ok(Instruction::INC(ArithmeticTarget::Reg8(Reg8::E))),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0x23 => Ok(Instruction::INC16(Reg16::HL)),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x24 => Ok(Instruction::INC(ArithmeticTarget::Reg8(Reg8::H))),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x2C => Ok(Instruction::INC(ArithmeticTarget::Reg8(Reg8::L))),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     SP (immediate)
            // Flags: Z N H C
            //        - - - -
            0x33 => Ok(Instruction::INC16_SP),

            // Instruction: INC
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     HL
            // Flags: Z N H C
            //        Z 0 H -
            0x34 => Ok(Instruction::INC(ArithmeticTarget::Indirect)),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x3C => Ok(Instruction::INC(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: JP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE9 => Ok(Instruction::JP {
                target: JumpAddress::HL,
                condition: JumpCondition::Always,
            }),

            // Instruction: JP (immediate)
            // { bytes: 3, cycles: [16,12] }
            // Operands:
            //     NZ (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC2 => Ok(Instruction::JP {
                target: JumpAddress::Immediate(program.next_u16()),
                condition: JumpCondition::NotZero,
            }),

            // Instruction: JP (immediate)
            // { bytes: 3, cycles: 16 }
            // Operands:
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC3 => Ok(Instruction::JP {
                target: JumpAddress::Immediate(program.next_u16()),
                condition: JumpCondition::Always,
            }),

            // Instruction: JP (immediate)
            // { bytes: 3, cycles: [16,12] }
            // Operands:
            //     Z (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCA => Ok(Instruction::JP {
                target: JumpAddress::Immediate(program.next_u16()),
                condition: JumpCondition::Zero,
            }),

            // Instruction: JP (immediate)
            // { bytes: 3, cycles: [16,12] }
            // Operands:
            //     NC (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD2 => Ok(Instruction::JP {
                target: JumpAddress::Immediate(program.next_u16()),
                condition: JumpCondition::NotCarry,
            }),

            // Instruction: JP (immediate)
            // { bytes: 3, cycles: [16,12] }
            // Operands:
            //     C (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xDA => Ok(Instruction::JP {
                target: JumpAddress::Immediate(program.next_u16()),
                condition: JumpCondition::Carry,
            }),

            // Instruction: JR (immediate)
            // { bytes: 2, cycles: 12 }
            // Operands:
            //     e8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x18 => Ok(Instruction::JR {
                target: program.next_i8(),
                condition: JumpCondition::Always,
            }),

            // Instruction: JR (immediate)
            // { bytes: 2, cycles: [12,8] }
            // Operands:
            //     NZ (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x20 => Ok(Instruction::JR {
                target: program.next_i8(),
                condition: JumpCondition::NotZero,
            }),

            // Instruction: JR (immediate)
            // { bytes: 2, cycles: [12,8] }
            // Operands:
            //     Z (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x28 => Ok(Instruction::JR {
                target: program.next_i8(),
                condition: JumpCondition::Zero,
            }),

            // Instruction: JR (immediate)
            // { bytes: 2, cycles: [12,8] }
            // Operands:
            //     NC (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x30 => Ok(Instruction::JR {
                target: program.next_i8(),
                condition: JumpCondition::NotCarry,
            }),

            // Instruction: JR (immediate)
            // { bytes: 2, cycles: [12,8] }
            // Operands:
            //     C (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x38 => Ok(Instruction::JR {
                target: program.next_i8(),
                condition: JumpCondition::Carry,
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     BC ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x02 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::BC),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     BC
            // Flags: Z N H C
            //        - - - -
            0x0A => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::BC),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     DE ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x12 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::DE),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     DE
            // Flags: Z N H C
            //        - - - -
            0x1A => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::DE),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x22 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::HLI),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x2A => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::HLI),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x32 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::HLD),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x3A => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::HLD),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x40 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::B),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x41 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::C),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x42 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::D),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x43 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::E),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x44 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::H),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x45 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::L),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     B (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x46 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Address(Address::HL),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x47 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x48 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::B),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x49 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::C),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x4A => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::D),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x4B => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::E),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x4C => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::H),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x4D => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::L),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     C (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x4E => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Address(Address::HL),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x4F => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x50 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::B),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x51 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::C),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x52 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::D),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x53 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::E),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x54 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::H),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x55 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::L),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     D (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x56 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Address(Address::HL),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x57 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x58 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::B),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x59 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::C),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x5A => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::D),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x5B => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::E),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x5C => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::H),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x5D => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::L),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     E (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x5E => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Address(Address::HL),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x5F => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x60 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::B),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x61 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::C),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x62 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::D),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x63 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::E),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x64 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::H),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x65 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::L),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     H (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x66 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Address(Address::HL),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x67 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x68 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::B),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x69 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::C),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x6A => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::D),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x6B => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::E),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x6C => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::H),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x6D => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::L),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     L (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x6E => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Address(Address::HL),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x6F => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x70 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::B),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x71 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::C),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x72 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::D),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x73 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::E),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x74 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::H),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x75 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::L),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x77 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x78 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::B),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x79 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::C),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x7A => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::D),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x7B => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::E),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x7C => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::H),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x7D => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::L),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x7E => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::HL),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x7F => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     C ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE2 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::ZeroPageC),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     C
            // Flags: Z N H C
            //        - - - -
            0xF2 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::ZeroPageC),
            }),

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     SP (immediate),
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF9 => Ok(Instruction::LD_SP_HL),

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     B (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x06 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Address(Address::Direct(program.skip_byte().pc())),
            }),

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     C (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x0E => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Address(Address::Direct(program.skip_byte().pc())),
            }),

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     D (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x16 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Address(Address::Direct(program.skip_byte().pc())),
            }),

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     E (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x1E => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Address(Address::Direct(program.skip_byte().pc())),
            }),

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     H (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x26 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Address(Address::Direct(program.skip_byte().pc())),
            }),

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     L (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x2E => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Address(Address::Direct(program.skip_byte().pc())),
            }),

            // Instruction: LD
            // { bytes: 2, cycles: 12 }
            // Operands:
            //     HL ,
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x36 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Address(Address::Direct(program.skip_byte().pc())),
            }),

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x3E => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Address(Address::Direct(program.skip_byte().pc())),
            }),

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 12 }
            // Operands:
            //     HL (immediate),
            //     SP (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        0 0 H C
            0xF8 => Ok(Instruction::LD_HL_SP_e(program.next_i8())),

            // Instruction: LD (immediate)
            // { bytes: 3, cycles: 12 }
            // Operands:
            //     BC (immediate),
            //     n16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x01 => Ok(Instruction::LD16 {
                destination: Reg16::BC,
                value: program.next_u16(),
            }),

            // Instruction: LD
            // { bytes: 3, cycles: 20 }
            // Operands:
            //     a16 ,
            //     SP (immediate)
            // Flags: Z N H C
            //        - - - -
            0x08 => Ok(Instruction::LD_nn_SP(program.next_u16())),

            // Instruction: LD (immediate)
            // { bytes: 3, cycles: 12 }
            // Operands:
            //     DE (immediate),
            //     n16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x11 => Ok(Instruction::LD16 {
                destination: Reg16::DE,
                value: program.next_u16(),
            }),

            // Instruction: LD (immediate)
            // { bytes: 3, cycles: 12 }
            // Operands:
            //     HL (immediate),
            //     n16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x21 => Ok(Instruction::LD16 {
                destination: Reg16::HL,
                value: program.next_u16(),
            }),

            // Instruction: LD (immediate)
            // { bytes: 3, cycles: 12 }
            // Operands:
            //     SP (immediate),
            //     n16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x31 => Ok(Instruction::LD16 {
                destination: Reg16::SP,
                value: program.next_u16(),
            }),

            // Instruction: LD
            // { bytes: 3, cycles: 16 }
            // Operands:
            //     a16 ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xEA => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::Direct(program.skip_byte().pc())),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LD
            // { bytes: 3, cycles: 16 }
            // Operands:
            //     A (immediate),
            //     a16
            // Flags: Z N H C
            //        - - - -
            0xFA => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::Direct(program.skip_byte().pc())),
            }),

            // Instruction: LDH
            // { bytes: 2, cycles: 12 }
            // Operands:
            //     a8 ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE0 => Ok(Instruction::LD {
                destination: LoadTarget::Address(Address::ZeroPage {
                    lsb: program.next_byte(),
                }),
                source: LoadTarget::Reg8(Reg8::A),
            }),

            // Instruction: LDH
            // { bytes: 2, cycles: 12 }
            // Operands:
            //     A (immediate),
            //     a8
            // Flags: Z N H C
            //        - - - -
            0xF0 => Ok(Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::ZeroPage {
                    lsb: program.next_byte(),
                }),
            }),

            // Instruction: NOP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0x00 => Ok(Instruction::NOP),

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB0 => Ok(Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::B),
            }),

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB1 => Ok(Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::C),
            }),

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB2 => Ok(Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::D),
            }),

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB3 => Ok(Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::E),
            }),

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB4 => Ok(Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::H),
            }),

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB5 => Ok(Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::L),
            }),

            // Instruction: OR
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 0 0 0
            0xB6 => Ok(Instruction::OR {
                source: ArithmeticTarget::Indirect,
            }),

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB7 => Ok(Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::A),
            }),

            // Instruction: OR (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xF6 => Ok(Instruction::OR {
                source: ArithmeticTarget::Immediate {
                    value: program.next_byte(),
                },
            }),

            // Instruction: POP (immediate)
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     BC (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC1 => Ok(Instruction::POP(Reg16::BC)),

            // Instruction: POP (immediate)
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     DE (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD1 => Ok(Instruction::POP(Reg16::DE)),

            // Instruction: POP (immediate)
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE1 => Ok(Instruction::POP(Reg16::HL)),

            // Instruction: POP (immediate)
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     AF (immediate)
            // Flags: Z N H C
            //        Z N H C
            0xF1 => Ok(Instruction::POP(Reg16::AF)),

            // Instruction: PREFIX (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0xCB => Ok(Instruction::PREFIX),

            // Instruction: PUSH (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     BC (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC5 => Ok(Instruction::PUSH(Reg16::BC)),

            // Instruction: PUSH (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     DE (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD5 => Ok(Instruction::PUSH(Reg16::DE)),

            // Instruction: PUSH (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE5 => Ok(Instruction::PUSH(Reg16::HL)),

            // Instruction: PUSH (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     AF (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF5 => Ok(Instruction::PUSH(Reg16::AF)),

            // Instruction: RET (immediate)
            // { bytes: 1, cycles: [20,8] }
            // Operands:
            //     NZ (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC0 => Ok(Instruction::RET(JumpCondition::NotZero)),

            // Instruction: RET (immediate)
            // { bytes: 1, cycles: [20,8] }
            // Operands:
            //     Z (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC8 => Ok(Instruction::RET(JumpCondition::Zero)),

            // Instruction: RET (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0xC9 => Ok(Instruction::RET(JumpCondition::Always)),

            // Instruction: RET (immediate)
            // { bytes: 1, cycles: [20,8] }
            // Operands:
            //     NC (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD0 => Ok(Instruction::RET(JumpCondition::NotCarry)),

            // Instruction: RET (immediate)
            // { bytes: 1, cycles: [20,8] }
            // Operands:
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD8 => Ok(Instruction::RET(JumpCondition::Carry)),

            // Instruction: RETI (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0xD9 => Ok(Instruction::RETI),

            // Instruction: RLA (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        0 0 0 C
            0x17 => Ok(Instruction::RL(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: RLCA (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        0 0 0 C
            0x07 => Ok(Instruction::RLC(ArithmeticTarget::Reg8(Reg8::A))), // Just merged RLCA to RLC A for ease
            // 0x07 => Ok(Instruction::RLCA),

            // Instruction: RRA (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        0 0 0 C
            0x1F => Ok(Instruction::RR(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: RRCA (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        0 0 0 C
            0x0F => Ok(Instruction::RRC(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $00 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC7 => Ok(Instruction::RST(0x00)),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $08 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCF => Ok(Instruction::RST(0x08)),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $10 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD7 => Ok(Instruction::RST(0x10)),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $18 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xDF => Ok(Instruction::RST(0x18)),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $20 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE7 => Ok(Instruction::RST(0x20)),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $28 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xEF => Ok(Instruction::RST(0x28)),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $30 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF7 => Ok(Instruction::RST(0x30)),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $38 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xFF => Ok(Instruction::RST(0x38)),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x98 => Ok(Instruction::SBC(ArithmeticTarget::Reg8(Reg8::B))),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x99 => Ok(Instruction::SBC(ArithmeticTarget::Reg8(Reg8::C))),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x9A => Ok(Instruction::SBC(ArithmeticTarget::Reg8(Reg8::D))),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x9B => Ok(Instruction::SBC(ArithmeticTarget::Reg8(Reg8::E))),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x9C => Ok(Instruction::SBC(ArithmeticTarget::Reg8(Reg8::H))),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x9D => Ok(Instruction::SBC(ArithmeticTarget::Reg8(Reg8::L))),

            // Instruction: SBC
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 1 H C
            0x9E => Ok(Instruction::SBC(ArithmeticTarget::Indirect)),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x9F => Ok(Instruction::SBC(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: SBC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xDE => Ok(Instruction::SBC(ArithmeticTarget::Immediate {
                value: program.next_byte(),
            })),

            // Instruction: SCF (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - 0 0 1
            0x37 => Ok(Instruction::SCF),

            // Instruction: STOP (immediate)
            // { bytes: 2, cycles: 4 }
            // Operands:
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x10 => Ok(Instruction::STOP),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x90 => Ok(Instruction::SUB(ArithmeticTarget::Reg8(Reg8::B))),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x91 => Ok(Instruction::SUB(ArithmeticTarget::Reg8(Reg8::C))),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x92 => Ok(Instruction::SUB(ArithmeticTarget::Reg8(Reg8::D))),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x93 => Ok(Instruction::SUB(ArithmeticTarget::Reg8(Reg8::E))),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x94 => Ok(Instruction::SUB(ArithmeticTarget::Reg8(Reg8::H))),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x95 => Ok(Instruction::SUB(ArithmeticTarget::Reg8(Reg8::L))),

            // Instruction: SUB
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 1 H C
            0x96 => Ok(Instruction::SUB(ArithmeticTarget::Indirect)),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        1 1 0 0
            0x97 => Ok(Instruction::SUB(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: SUB (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xD6 => Ok(Instruction::SUB(ArithmeticTarget::Immediate {
                value: program.next_byte(),
            })),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xA8 => Ok(Instruction::XOR(ArithmeticTarget::Reg8(Reg8::B))),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xA9 => Ok(Instruction::XOR(ArithmeticTarget::Reg8(Reg8::C))),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xAA => Ok(Instruction::XOR(ArithmeticTarget::Reg8(Reg8::D))),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xAB => Ok(Instruction::XOR(ArithmeticTarget::Reg8(Reg8::E))),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xAC => Ok(Instruction::XOR(ArithmeticTarget::Reg8(Reg8::H))),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xAD => Ok(Instruction::XOR(ArithmeticTarget::Reg8(Reg8::L))),

            // Instruction: XOR
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 0 0 0
            0xAE => Ok(Instruction::XOR(ArithmeticTarget::Indirect)),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        1 0 0 0
            0xAF => Ok(Instruction::XOR(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: XOR (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xEE => Ok(Instruction::XOR(ArithmeticTarget::Immediate {
                value: program.next_byte(),
            })),

            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                Ok(Instruction::UNDEFINED)
            }
        }
    }
}
