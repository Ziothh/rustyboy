use crate::{
    cpu::{
        instructions::{ArithmeticTarget, JumpAddress, JumpCondition, LoadTarget},
        memory::{Address, Reg16, Reg8},
        Instruction, CPU,
    },
    Hardware,
};

impl CPU {
    pub fn decode_instruction_unprefixed(
        &mut self,
        byte: u8,
        hardware: &mut Hardware,
    ) -> Instruction {
        match byte {
            /* [ADC] */
            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x88 => Instruction::ADC(ArithmeticTarget::Reg8(Reg8::B)),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x89 => Instruction::ADC(ArithmeticTarget::Reg8(Reg8::C)),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x8A => Instruction::ADC(ArithmeticTarget::Reg8(Reg8::D)),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x8B => Instruction::ADC(ArithmeticTarget::Reg8(Reg8::E)),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x8C => Instruction::ADC(ArithmeticTarget::Reg8(Reg8::H)),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x8D => Instruction::ADC(ArithmeticTarget::Reg8(Reg8::L)),

            // Instruction: ADC
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 0 H C
            0x8E => Instruction::ADC(ArithmeticTarget::Indirect),

            // Instruction: ADC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x8F => Instruction::ADC(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: ADC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0xCE => Instruction::ADC(ArithmeticTarget::Immediate {
                value: self.fetch_u8(hardware),
            }),

            /* [ADD] */
            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate),
            //     BC (immediate)
            // Flags: Z N H C
            //        - 0 H C
            0x09 => Instruction::ADD16(Reg16::BC),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate),
            //     DE (immediate)
            // Flags: Z N H C
            //        - 0 H C
            0x19 => Instruction::ADD16(Reg16::DE),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate),
            //     HL (immediate)
            // Flags: Z N H C
            //        - 0 H C
            0x29 => Instruction::ADD16(Reg16::HL),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate),
            //     SP (immediate)
            // Flags: Z N H C
            //        - 0 H C
            0x39 => Instruction::ADD16_SP,

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x80 => Instruction::ADD(ArithmeticTarget::Reg8(Reg8::B)),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x81 => Instruction::ADD(ArithmeticTarget::Reg8(Reg8::C)),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x82 => Instruction::ADD(ArithmeticTarget::Reg8(Reg8::D)),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x83 => Instruction::ADD(ArithmeticTarget::Reg8(Reg8::E)),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x84 => Instruction::ADD(ArithmeticTarget::Reg8(Reg8::H)),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x85 => Instruction::ADD(ArithmeticTarget::Reg8(Reg8::L)),

            // Instruction: ADD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 0 H C
            0x86 => Instruction::ADD(ArithmeticTarget::Indirect),

            // Instruction: ADD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0x87 => Instruction::ADD(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: ADD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 0 H C
            0xC6 => Instruction::ADD(ArithmeticTarget::Immediate {
                value: self.fetch_u8(hardware),
            }),

            // Instruction: ADD (immediate)
            // { bytes: 2, cycles: 16 }
            // Operands:
            //     SP (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        0 0 H C
            0xE8 => Instruction::ADD_SP_i8(self.fetch_i8(hardware)),

            /* [AND] */
            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA0 => Instruction::AND(ArithmeticTarget::Reg8(Reg8::B)),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA1 => Instruction::AND(ArithmeticTarget::Reg8(Reg8::C)),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA2 => Instruction::AND(ArithmeticTarget::Reg8(Reg8::D)),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA3 => Instruction::AND(ArithmeticTarget::Reg8(Reg8::E)),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA4 => Instruction::AND(ArithmeticTarget::Reg8(Reg8::H)),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA5 => Instruction::AND(ArithmeticTarget::Reg8(Reg8::L)),

            // Instruction: AND
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 0 1 0
            0xA6 => Instruction::AND(ArithmeticTarget::Indirect),

            // Instruction: AND (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xA7 => Instruction::AND(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: AND (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 0 1 0
            0xE6 => Instruction::AND(ArithmeticTarget::Immediate {
                value: self.fetch_u8(hardware),
            }),

            // Instruction: CALL (immediate)
            // { bytes: 3, cycles: [24,12] }
            // Operands:
            //     NZ (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC4 => Instruction::CALL {
                address: self.fetch_u16(hardware),
                condition: JumpCondition::NotZero,
            },

            // Instruction: CALL (immediate)
            // { bytes: 3, cycles: [24,12] }
            // Operands:
            //     Z (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCC => Instruction::CALL {
                address: self.fetch_u16(hardware),
                condition: JumpCondition::Zero,
            },

            // Instruction: CALL (immediate)
            // { bytes: 3, cycles: 24 }
            // Operands:
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCD => Instruction::CALL {
                address: self.fetch_u16(hardware),
                condition: JumpCondition::Always,
            },

            // Instruction: CALL (immediate)
            // { bytes: 3, cycles: [24,12] }
            // Operands:
            //     NC (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD4 => Instruction::CALL {
                address: self.fetch_u16(hardware),
                condition: JumpCondition::NotCarry,
            },

            // Instruction: CALL (immediate)
            // { bytes: 3, cycles: [24,12] }
            // Operands:
            //     C (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xDC => Instruction::CALL {
                address: self.fetch_u16(hardware),
                condition: JumpCondition::NotCarry,
            },

            // Instruction: CCF (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - 0 0 C
            0x3F => Instruction::CCF,

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xB8 => Instruction::CP(ArithmeticTarget::Reg8(Reg8::B)),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xB9 => Instruction::CP(ArithmeticTarget::Reg8(Reg8::C)),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xBA => Instruction::CP(ArithmeticTarget::Reg8(Reg8::D)),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xBB => Instruction::CP(ArithmeticTarget::Reg8(Reg8::E)),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xBC => Instruction::CP(ArithmeticTarget::Reg8(Reg8::H)),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xBD => Instruction::CP(ArithmeticTarget::Reg8(Reg8::L)),

            // Instruction: CP
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 1 H C
            0xBE => Instruction::CP(ArithmeticTarget::Indirect),

            // Instruction: CP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        1 1 0 0
            0xBF => Instruction::CP(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: CP (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xFE => Instruction::CP(ArithmeticTarget::Immediate {
                value: self.fetch_u8(hardware),
            }),

            // Instruction: CPL (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - 1 1 -
            0x2F => Instruction::CPL,

            // Instruction: DAA (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        Z - 0 C
            0x27 => Instruction::DAA,

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x05 => Instruction::DEC(ArithmeticTarget::Reg8(Reg8::B)),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     BC (immediate)
            // Flags: Z N H C
            //        - - - -
            0x0B => Instruction::DEC16(Reg16::BC),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x0D => Instruction::DEC(ArithmeticTarget::Reg8(Reg8::C)),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x15 => Instruction::DEC(ArithmeticTarget::Reg8(Reg8::D)),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     DE (immediate)
            // Flags: Z N H C
            //        - - - -
            0x1B => Instruction::DEC16(Reg16::DE),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x1D => Instruction::DEC(ArithmeticTarget::Reg8(Reg8::E)),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x25 => Instruction::DEC(ArithmeticTarget::Reg8(Reg8::H)),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0x2B => Instruction::DEC16(Reg16::HL),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x2D => Instruction::DEC(ArithmeticTarget::Reg8(Reg8::L)),

            // Instruction: DEC
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     HL
            // Flags: Z N H C
            //        Z 1 H -
            0x35 => Instruction::DEC(ArithmeticTarget::Indirect),

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     SP (immediate)
            // Flags: Z N H C
            //        - - - -
            0x3B => Instruction::DEC16_SP,

            // Instruction: DEC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x3D => Instruction::DEC(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: DI (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0xF3 => Instruction::DI,

            // Instruction: EI (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0xFB => Instruction::EI,

            // Instruction: HALT (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0x76 => Instruction::HALT,

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     BC (immediate)
            // Flags: Z N H C
            //        - - - -
            0x03 => Instruction::INC16(Reg16::BC),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x04 => Instruction::INC(ArithmeticTarget::Reg8(Reg8::B)),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x0C => Instruction::INC(ArithmeticTarget::Reg8(Reg8::C)),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     DE (immediate)
            // Flags: Z N H C
            //        - - - -
            0x13 => Instruction::INC16(Reg16::DE),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x14 => Instruction::INC(ArithmeticTarget::Reg8(Reg8::D)),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x1C => Instruction::INC(ArithmeticTarget::Reg8(Reg8::E)),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0x23 => Instruction::INC16(Reg16::HL),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x24 => Instruction::INC(ArithmeticTarget::Reg8(Reg8::H)),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x2C => Instruction::INC(ArithmeticTarget::Reg8(Reg8::L)),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     SP (immediate)
            // Flags: Z N H C
            //        - - - -
            0x33 => Instruction::INC16_SP,

            // Instruction: INC
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     HL
            // Flags: Z N H C
            //        Z 0 H -
            0x34 => Instruction::INC(ArithmeticTarget::Indirect),

            // Instruction: INC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 H -
            0x3C => Instruction::INC(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: JP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE9 => Instruction::JP {
                target: JumpAddress::HL,
                condition: JumpCondition::Always,
            },

            // Instruction: JP (immediate)
            // { bytes: 3, cycles: [16,12] }
            // Operands:
            //     NZ (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC2 => Instruction::JP {
                target: JumpAddress::Immediate(self.fetch_u16(hardware)),
                condition: JumpCondition::NotZero,
            },

            // Instruction: JP (immediate)
            // { bytes: 3, cycles: 16 }
            // Operands:
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC3 => Instruction::JP {
                target: JumpAddress::Immediate(self.fetch_u16(hardware)),
                condition: JumpCondition::Always,
            },

            // Instruction: JP (immediate)
            // { bytes: 3, cycles: [16,12] }
            // Operands:
            //     Z (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCA => Instruction::JP {
                target: JumpAddress::Immediate(self.fetch_u16(hardware)),
                condition: JumpCondition::Zero,
            },

            // Instruction: JP (immediate)
            // { bytes: 3, cycles: [16,12] }
            // Operands:
            //     NC (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD2 => Instruction::JP {
                target: JumpAddress::Immediate(self.fetch_u16(hardware)),
                condition: JumpCondition::NotCarry,
            },

            // Instruction: JP (immediate)
            // { bytes: 3, cycles: [16,12] }
            // Operands:
            //     C (immediate),
            //     a16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xDA => Instruction::JP {
                target: JumpAddress::Immediate(self.fetch_u16(hardware)),
                condition: JumpCondition::Carry,
            },

            // Instruction: JR (immediate)
            // { bytes: 2, cycles: 12 }
            // Operands:
            //     e8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x18 => Instruction::JR {
                target: self.fetch_i8(hardware),
                condition: JumpCondition::Always,
            },

            // Instruction: JR (immediate)
            // { bytes: 2, cycles: [12,8] }
            // Operands:
            //     NZ (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x20 => Instruction::JR {
                target: self.fetch_i8(hardware),
                condition: JumpCondition::NotZero,
            },

            // Instruction: JR (immediate)
            // { bytes: 2, cycles: [12,8] }
            // Operands:
            //     Z (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x28 => Instruction::JR {
                target: self.fetch_i8(hardware),
                condition: JumpCondition::Zero,
            },

            // Instruction: JR (immediate)
            // { bytes: 2, cycles: [12,8] }
            // Operands:
            //     NC (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x30 => Instruction::JR {
                target: self.fetch_i8(hardware),
                condition: JumpCondition::NotCarry,
            },

            // Instruction: JR (immediate)
            // { bytes: 2, cycles: [12,8] }
            // Operands:
            //     C (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x38 => Instruction::JR {
                target: self.fetch_i8(hardware),
                condition: JumpCondition::Carry,
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     BC ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x02 => Instruction::LD {
                destination: LoadTarget::Address(Address::BC),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     BC
            // Flags: Z N H C
            //        - - - -
            0x0A => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::BC),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     DE ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x12 => Instruction::LD {
                destination: LoadTarget::Address(Address::DE),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     DE
            // Flags: Z N H C
            //        - - - -
            0x1A => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::DE),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x22 => Instruction::LD {
                destination: LoadTarget::Address(Address::HLI),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x2A => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::HLI),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x32 => Instruction::LD {
                destination: LoadTarget::Address(Address::HLD),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x3A => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::HLD),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x40 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::B),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x41 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::C),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x42 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::D),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x43 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::E),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x44 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::H),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x45 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::L),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     B (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x46 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Address(Address::HL),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     B (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x47 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x48 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::B),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x49 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::C),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x4A => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::D),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x4B => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::E),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x4C => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::H),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x4D => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::L),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     C (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x4E => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Address(Address::HL),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     C (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x4F => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x50 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::B),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x51 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::C),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x52 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::D),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x53 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::E),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x54 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::H),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x55 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::L),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     D (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x56 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Address(Address::HL),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     D (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x57 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x58 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::B),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x59 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::C),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x5A => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::D),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x5B => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::E),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x5C => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::H),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x5D => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::L),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     E (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x5E => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Address(Address::HL),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     E (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x5F => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x60 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::B),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x61 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::C),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x62 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::D),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x63 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::E),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x64 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::H),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x65 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::L),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     H (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x66 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Address(Address::HL),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     H (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x67 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x68 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::B),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x69 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::C),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x6A => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::D),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x6B => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::E),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x6C => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::H),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x6D => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::L),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     L (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x6E => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Address(Address::HL),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     L (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x6F => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x70 => Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::B),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x71 => Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::C),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x72 => Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::D),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x73 => Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::E),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x74 => Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::H),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x75 => Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::L),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     HL ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x77 => Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x78 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::B),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x79 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::C),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x7A => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::D),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x7B => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::E),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x7C => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::H),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x7D => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::L),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        - - - -
            0x7E => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::HL),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x7F => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     C ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE2 => Instruction::LD {
                destination: LoadTarget::Address(Address::ZeroPageC),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     C
            // Flags: Z N H C
            //        - - - -
            0xF2 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::ZeroPageC),
            },

            // Instruction: LD (immediate)
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     SP (immediate),
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF9 => Instruction::LD_SP_HL,

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     B (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x06 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Immediate(self.fetch_u8(hardware)),
            },

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     C (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x0E => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::C),
                source: LoadTarget::Immediate(self.fetch_u8(hardware)),
            },

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     D (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x16 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::D),
                source: LoadTarget::Immediate(self.fetch_u8(hardware)),
            },

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     E (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x1E => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::E),
                source: LoadTarget::Immediate(self.fetch_u8(hardware)),
            },

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     H (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x26 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::H),
                source: LoadTarget::Immediate(self.fetch_u8(hardware)),
            },

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     L (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x2E => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::L),
                source: LoadTarget::Immediate(self.fetch_u8(hardware)),
            },

            // Instruction: LD
            // { bytes: 2, cycles: 12 }
            // Operands:
            //     HL ,
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x36 => Instruction::LD {
                destination: LoadTarget::Address(Address::HL),
                source: LoadTarget::Immediate(self.fetch_u8(hardware)),
            },

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x3E => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::B),
                source: LoadTarget::Immediate(self.fetch_u8(hardware)),
            },

            // Instruction: LD (immediate)
            // { bytes: 2, cycles: 12 }
            // Operands:
            //     HL (immediate),
            //     SP (immediate),
            //     e8 (immediate)
            // Flags: Z N H C
            //        0 0 H C
            0xF8 => Instruction::LD_HL_SP_e(self.fetch_i8(hardware)),

            // Instruction: LD (immediate)
            // { bytes: 3, cycles: 12 }
            // Operands:
            //     BC (immediate),
            //     n16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x01 => Instruction::LD16 {
                destination: Reg16::BC,
                value: self.fetch_u16(hardware),
            },

            // Instruction: LD
            // { bytes: 3, cycles: 20 }
            // Operands:
            //     a16 ,
            //     SP (immediate)
            // Flags: Z N H C
            //        - - - -
            0x08 => Instruction::LD_nn_SP(self.fetch_u16(hardware)),

            // Instruction: LD (immediate)
            // { bytes: 3, cycles: 12 }
            // Operands:
            //     DE (immediate),
            //     n16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x11 => Instruction::LD16 {
                destination: Reg16::DE,
                value: self.fetch_u16(hardware),
            },

            // Instruction: LD (immediate)
            // { bytes: 3, cycles: 12 }
            // Operands:
            //     HL (immediate),
            //     n16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x21 => Instruction::LD16 {
                destination: Reg16::HL,
                value: self.fetch_u16(hardware),
            },

            // Instruction: LD (immediate)
            // { bytes: 3, cycles: 12 }
            // Operands:
            //     SP (immediate),
            //     n16 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x31 => Instruction::LD16 {
                destination: Reg16::SP,
                value: self.fetch_u16(hardware),
            },

            // Instruction: LD
            // { bytes: 3, cycles: 16 }
            // Operands:
            //     a16 ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xEA => Instruction::LD {
                destination: LoadTarget::Address(Address::Direct {
                    lsb: self.fetch_u8(hardware),
                    msb: self.fetch_u8(hardware),
                }),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LD
            // { bytes: 3, cycles: 16 }
            // Operands:
            //     A (immediate),
            //     a16
            // Flags: Z N H C
            //        - - - -
            0xFA => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::Direct {
                    lsb: self.fetch_u8(hardware),
                    msb: self.fetch_u8(hardware),
                }),
            },

            // Instruction: LDH
            // { bytes: 2, cycles: 12 }
            // Operands:
            //     a8 ,
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE0 => Instruction::LD {
                destination: LoadTarget::Address(Address::ZeroPage {
                    lsb: self.fetch_u8(hardware),
                }),
                source: LoadTarget::Reg8(Reg8::A),
            },

            // Instruction: LDH
            // { bytes: 2, cycles: 12 }
            // Operands:
            //     A (immediate),
            //     a8
            // Flags: Z N H C
            //        - - - -
            0xF0 => Instruction::LD {
                destination: LoadTarget::Reg8(Reg8::A),
                source: LoadTarget::Address(Address::ZeroPage {
                    lsb: self.fetch_u8(hardware),
                }),
            },

            // Instruction: NOP (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0x00 => Instruction::NOP,

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB0 => Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::B),
            },

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB1 => Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::C),
            },

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB2 => Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::D),
            },

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB3 => Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::E),
            },

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB4 => Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::H),
            },

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB5 => Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::L),
            },

            // Instruction: OR
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 0 0 0
            0xB6 => Instruction::OR {
                source: ArithmeticTarget::Indirect,
            },

            // Instruction: OR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xB7 => Instruction::OR {
                source: ArithmeticTarget::Reg8(Reg8::A),
            },

            // Instruction: OR (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xF6 => Instruction::OR {
                source: ArithmeticTarget::Immediate {
                    value: self.fetch_u8(hardware),
                },
            },

            // Instruction: POP (immediate)
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     BC (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC1 => Instruction::POP(Reg16::BC),

            // Instruction: POP (immediate)
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     DE (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD1 => Instruction::POP(Reg16::DE),

            // Instruction: POP (immediate)
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE1 => Instruction::POP(Reg16::HL),

            // Instruction: POP (immediate)
            // { bytes: 1, cycles: 12 }
            // Operands:
            //     AF (immediate)
            // Flags: Z N H C
            //        Z N H C
            0xF1 => Instruction::POP(Reg16::AF),

            // Instruction: PREFIX (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0xCB => Instruction::PREFIX,

            // Instruction: PUSH (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     BC (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC5 => Instruction::PUSH(Reg16::BC),

            // Instruction: PUSH (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     DE (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD5 => Instruction::PUSH(Reg16::DE),

            // Instruction: PUSH (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     HL (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE5 => Instruction::PUSH(Reg16::HL),

            // Instruction: PUSH (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     AF (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF5 => Instruction::PUSH(Reg16::AF),

            // Instruction: RET (immediate)
            // { bytes: 1, cycles: [20,8] }
            // Operands:
            //     NZ (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC0 => Instruction::RET(JumpCondition::NotZero),

            // Instruction: RET (immediate)
            // { bytes: 1, cycles: [20,8] }
            // Operands:
            //     Z (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC8 => Instruction::RET(JumpCondition::Zero),

            // Instruction: RET (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0xC9 => Instruction::RET(JumpCondition::Always),

            // Instruction: RET (immediate)
            // { bytes: 1, cycles: [20,8] }
            // Operands:
            //     NC (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD0 => Instruction::RET(JumpCondition::NotCarry),

            // Instruction: RET (immediate)
            // { bytes: 1, cycles: [20,8] }
            // Operands:
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD8 => Instruction::RET(JumpCondition::Carry),

            // Instruction: RETI (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            // Flags: Z N H C
            //        - - - -
            0xD9 => Instruction::RETI,

            // Instruction: RLA (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        0 0 0 C
            0x17 => Instruction::RL(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: RLCA (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        0 0 0 C
            0x07 => Instruction::RLC(ArithmeticTarget::Reg8(Reg8::A)), // Just merged RLCA to RLC A for ease
            // 0x07 => Ok(Instruction::RLCA),

            // Instruction: RRA (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        0 0 0 C
            0x1F => Instruction::RR(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: RRCA (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        0 0 0 C
            0x0F => Instruction::RRC(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $00 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC7 => Instruction::RST(0x00),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $08 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCF => Instruction::RST(0x08),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $10 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD7 => Instruction::RST(0x10),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $18 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xDF => Instruction::RST(0x18),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $20 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE7 => Instruction::RST(0x20),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $28 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xEF => Instruction::RST(0x28),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $30 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF7 => Instruction::RST(0x30),

            // Instruction: RST (immediate)
            // { bytes: 1, cycles: 16 }
            // Operands:
            //     $38 (immediate)
            // Flags: Z N H C
            //        - - - -
            0xFF => Instruction::RST(0x38),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x98 => Instruction::SBC(ArithmeticTarget::Reg8(Reg8::B)),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x99 => Instruction::SBC(ArithmeticTarget::Reg8(Reg8::C)),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x9A => Instruction::SBC(ArithmeticTarget::Reg8(Reg8::D)),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x9B => Instruction::SBC(ArithmeticTarget::Reg8(Reg8::E)),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x9C => Instruction::SBC(ArithmeticTarget::Reg8(Reg8::H)),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x9D => Instruction::SBC(ArithmeticTarget::Reg8(Reg8::L)),

            // Instruction: SBC
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 1 H C
            0x9E => Instruction::SBC(ArithmeticTarget::Indirect),

            // Instruction: SBC (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 1 H -
            0x9F => Instruction::SBC(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: SBC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xDE => Instruction::SBC(ArithmeticTarget::Immediate {
                value: self.fetch_u8(hardware),
            }),

            // Instruction: SCF (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            // Flags: Z N H C
            //        - 0 0 1
            0x37 => Instruction::SCF,

            // Instruction: STOP (immediate)
            // { bytes: 2, cycles: 4 }
            // Operands:
            //     n8 (immediate)
            // Flags: Z N H C
            //        - - - -
            0x10 => Instruction::STOP,

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x90 => Instruction::SUB(ArithmeticTarget::Reg8(Reg8::B)),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x91 => Instruction::SUB(ArithmeticTarget::Reg8(Reg8::C)),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x92 => Instruction::SUB(ArithmeticTarget::Reg8(Reg8::D)),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x93 => Instruction::SUB(ArithmeticTarget::Reg8(Reg8::E)),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x94 => Instruction::SUB(ArithmeticTarget::Reg8(Reg8::H)),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0x95 => Instruction::SUB(ArithmeticTarget::Reg8(Reg8::L)),

            // Instruction: SUB
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 1 H C
            0x96 => Instruction::SUB(ArithmeticTarget::Indirect),

            // Instruction: SUB (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        1 1 0 0
            0x97 => Instruction::SUB(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: SUB (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 1 H C
            0xD6 => Instruction::SUB(ArithmeticTarget::Immediate {
                value: self.fetch_u8(hardware),
            }),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xA8 => Instruction::XOR(ArithmeticTarget::Reg8(Reg8::B)),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xA9 => Instruction::XOR(ArithmeticTarget::Reg8(Reg8::C)),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xAA => Instruction::XOR(ArithmeticTarget::Reg8(Reg8::D)),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xAB => Instruction::XOR(ArithmeticTarget::Reg8(Reg8::E)),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xAC => Instruction::XOR(ArithmeticTarget::Reg8(Reg8::H)),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xAD => Instruction::XOR(ArithmeticTarget::Reg8(Reg8::L)),

            // Instruction: XOR
            // { bytes: 1, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     HL
            // Flags: Z N H C
            //        Z 0 0 0
            0xAE => Instruction::XOR(ArithmeticTarget::Indirect),

            // Instruction: XOR (immediate)
            // { bytes: 1, cycles: 4 }
            // Operands:
            //     A (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        1 0 0 0
            0xAF => Instruction::XOR(ArithmeticTarget::Reg8(Reg8::A)),

            // Instruction: XOR (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands:
            //     A (immediate),
            //     n8 (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0xEE => Instruction::XOR(ArithmeticTarget::Immediate {
                value: self.fetch_u8(hardware),
            }),

            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                Instruction::UNDEFINED
            }
        }
    }
}
