use crate::{CPU::{Instruction, instructions::ArithmeticTarget, memory::{Reg8, Reg16}}, program::Program};

impl Instruction {
    pub fn try_from_opcode_unprefixed(byte: u8, program: &mut Program) -> Result<Self, String> {
        match byte {
            /* [ADC] */
            // Instruction: ADC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x88 => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::B))),
            // Instruction: ADC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x89 => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::C))),
            // Instruction: ADC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x8A => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::D))),
            // Instruction: ADC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x8B => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::E))),
            // Instruction: ADC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x8C => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::H))),
            // Instruction: ADC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x8D => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::L))),
            // Instruction: ADC 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x8E => Ok(Instruction::ADC(ArithmeticTarget::Indirect)),
            // Instruction: ADC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x8F => Ok(Instruction::ADC(ArithmeticTarget::Reg8(Reg8::A))),
            // Instruction: ADC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0xCE => Ok(Instruction::ADC(ArithmeticTarget::Immediate { value: *program.next_byte() })),

            /* ADD */
            // Instruction: ADD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   },
            //   {
            //     "name": "BC",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x09 => Ok(Instruction::ADD(ArithmeticTarget::Reg16(Reg16::BC))),

            // Instruction: ADD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   },
            //   {
            //     "name": "DE",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x19 => Ok(Instruction::ADD(ArithmeticTarget::Reg16(Reg16::DE))),

            // Instruction: ADD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x29 => Ok(Instruction::ADD(ArithmeticTarget::Reg16(Reg16::HL))),

            // Instruction: ADD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   },
            //   {
            //     "name": "SP",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x39 => Ok(Instruction::ADD(ArithmeticTarget::StackPointer)),

            // Instruction: ADD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x80 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::B))),

            // Instruction: ADD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x81 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::C))),

            // Instruction: ADD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x82 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::D))),

            // Instruction: ADD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x83 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::E))),

            // Instruction: ADD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x84 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::H))),

            // Instruction: ADD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x85 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::L))),

            // Instruction: ADD 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x86 => Ok(Instruction::ADD(ArithmeticTarget::Indirect)),

            // Instruction: ADD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0x87 => Ok(Instruction::ADD(ArithmeticTarget::Reg8(Reg8::A))),

            // Instruction: ADD 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0xC6 => Ok(Instruction::ADD(ArithmeticTarget::Immediate { value: *program.next_byte() })),

            // Instruction: ADD 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "SP",
            //     "immediate": true
            //   },
            //   {
            //     "name": "e8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "0",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0xE8 => Ok(Instruction::ADD(ArithmeticTarget::SignedU8ToSP { value: *program.next_byte() as i8 })),

            // Instruction: AND 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "1",
            //   "C": "0"
            // }
            0xA0 => Ok(Instruction::AND),

            // Instruction: AND 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "1",
            //   "C": "0"
            // }
            0xA1 => Ok(Instruction::AND),

            // Instruction: AND 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "1",
            //   "C": "0"
            // }
            0xA2 => Ok(Instruction::AND),

            // Instruction: AND 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "1",
            //   "C": "0"
            // }
            0xA3 => Ok(Instruction::AND),

            // Instruction: AND 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "1",
            //   "C": "0"
            // }
            0xA4 => Ok(Instruction::AND),

            // Instruction: AND 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "1",
            //   "C": "0"
            // }
            0xA5 => Ok(Instruction::AND),

            // Instruction: AND 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "1",
            //   "C": "0"
            // }
            0xA6 => Ok(Instruction::AND),

            // Instruction: AND 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "1",
            //   "C": "0"
            // }
            0xA7 => Ok(Instruction::AND),

            // Instruction: AND 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "1",
            //   "C": "0"
            // }
            0xE6 => Ok(Instruction::AND),

            // Instruction: CALL 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "NZ",
            //     "immediate": true
            //   },
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xC4 => Ok(Instruction::CALL),

            // Instruction: CALL 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "Z",
            //     "immediate": true
            //   },
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xCC => Ok(Instruction::CALL),

            // Instruction: CALL 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xCD => Ok(Instruction::CALL),

            // Instruction: CALL 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "NC",
            //     "immediate": true
            //   },
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xD4 => Ok(Instruction::CALL),

            // Instruction: CALL 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xDC => Ok(Instruction::CALL),

            // Instruction: CCF 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x3F => Ok(Instruction::CCF),

            // Instruction: CP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0xB8 => Ok(Instruction::CP),

            // Instruction: CP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0xB9 => Ok(Instruction::CP),

            // Instruction: CP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0xBA => Ok(Instruction::CP),

            // Instruction: CP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0xBB => Ok(Instruction::CP),

            // Instruction: CP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0xBC => Ok(Instruction::CP),

            // Instruction: CP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0xBD => Ok(Instruction::CP),

            // Instruction: CP 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0xBE => Ok(Instruction::CP),

            // Instruction: CP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "1",
            //   "N": "1",
            //   "H": "0",
            //   "C": "0"
            // }
            0xBF => Ok(Instruction::CP),

            // Instruction: CP 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0xFE => Ok(Instruction::CP),

            // Instruction: CPL 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "1",
            //   "H": "1",
            //   "C": "-"
            // }
            0x2F => Ok(Instruction::CPL),

            // Instruction: DAA 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "Z",
            //   "N": "-",
            //   "H": "0",
            //   "C": "C"
            // }
            0x27 => Ok(Instruction::DAA),

            // Instruction: DEC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "-"
            // }
            0x05 => Ok(Instruction::DEC),

            // Instruction: DEC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "BC",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x0B => Ok(Instruction::DEC),

            // Instruction: DEC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "-"
            // }
            0x0D => Ok(Instruction::DEC),

            // Instruction: DEC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "-"
            // }
            0x15 => Ok(Instruction::DEC),

            // Instruction: DEC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "DE",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x1B => Ok(Instruction::DEC),

            // Instruction: DEC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "-"
            // }
            0x1D => Ok(Instruction::DEC),

            // Instruction: DEC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "-"
            // }
            0x25 => Ok(Instruction::DEC),

            // Instruction: DEC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x2B => Ok(Instruction::DEC),

            // Instruction: DEC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "-"
            // }
            0x2D => Ok(Instruction::DEC),

            // Instruction: DEC 1 byte 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "-"
            // }
            0x35 => Ok(Instruction::DEC),

            // Instruction: DEC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "SP",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x3B => Ok(Instruction::DEC),

            // Instruction: DEC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "-"
            // }
            0x3D => Ok(Instruction::DEC),

            // Instruction: DI 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xF3 => Ok(Instruction::DI),

            // Instruction: EI 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xFB => Ok(Instruction::EI),

            // Instruction: HALT 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x76 => Ok(Instruction::HALT),

            // Instruction: ILLEGAL_D3 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xD3 => Ok(Instruction::ILLEGAL_D3),

            // Instruction: ILLEGAL_DB 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xDB => Ok(Instruction::ILLEGAL_DB),

            // Instruction: ILLEGAL_DD 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xDD => Ok(Instruction::ILLEGAL_DD),

            // Instruction: ILLEGAL_E3 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xE3 => Ok(Instruction::ILLEGAL_E3),

            // Instruction: ILLEGAL_E4 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xE4 => Ok(Instruction::ILLEGAL_E4),

            // Instruction: ILLEGAL_EB 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xEB => Ok(Instruction::ILLEGAL_EB),

            // Instruction: ILLEGAL_EC 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xEC => Ok(Instruction::ILLEGAL_EC),

            // Instruction: ILLEGAL_ED 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xED => Ok(Instruction::ILLEGAL_ED),

            // Instruction: ILLEGAL_F4 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xF4 => Ok(Instruction::ILLEGAL_F4),

            // Instruction: ILLEGAL_FC 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xFC => Ok(Instruction::ILLEGAL_FC),

            // Instruction: ILLEGAL_FD 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xFD => Ok(Instruction::ILLEGAL_FD),

            // Instruction: INC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "BC",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x03 => Ok(Instruction::INC),

            // Instruction: INC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "-"
            // }
            0x04 => Ok(Instruction::INC),

            // Instruction: INC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "-"
            // }
            0x0C => Ok(Instruction::INC),

            // Instruction: INC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "DE",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x13 => Ok(Instruction::INC),

            // Instruction: INC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "-"
            // }
            0x14 => Ok(Instruction::INC),

            // Instruction: INC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "-"
            // }
            0x1C => Ok(Instruction::INC),

            // Instruction: INC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x23 => Ok(Instruction::INC),

            // Instruction: INC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "-"
            // }
            0x24 => Ok(Instruction::INC),

            // Instruction: INC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "-"
            // }
            0x2C => Ok(Instruction::INC),

            // Instruction: INC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "SP",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x33 => Ok(Instruction::INC),

            // Instruction: INC 1 byte 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "-"
            // }
            0x34 => Ok(Instruction::INC),

            // Instruction: INC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "H",
            //   "C": "-"
            // }
            0x3C => Ok(Instruction::INC),

            // Instruction: JP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xE9 => Ok(Instruction::JP),

            // Instruction: JP 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "NZ",
            //     "immediate": true
            //   },
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xC2 => Ok(Instruction::JP),

            // Instruction: JP 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xC3 => Ok(Instruction::JP),

            // Instruction: JP 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "Z",
            //     "immediate": true
            //   },
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xCA => Ok(Instruction::JP),

            // Instruction: JP 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "NC",
            //     "immediate": true
            //   },
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xD2 => Ok(Instruction::JP),

            // Instruction: JP 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xDA => Ok(Instruction::JP),

            // Instruction: JR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "e8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x18 => Ok(Instruction::JR),

            // Instruction: JR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "NZ",
            //     "immediate": true
            //   },
            //   {
            //     "name": "e8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x20 => Ok(Instruction::JR),

            // Instruction: JR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "Z",
            //     "immediate": true
            //   },
            //   {
            //     "name": "e8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x28 => Ok(Instruction::JR),

            // Instruction: JR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "NC",
            //     "immediate": true
            //   },
            //   {
            //     "name": "e8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x30 => Ok(Instruction::JR),

            // Instruction: JR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "e8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x38 => Ok(Instruction::JR),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "BC",
            //     "immediate": false
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x02 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "BC",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x0A => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "DE",
            //     "immediate": false
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x12 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "DE",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x1A => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "increment": true,
            //     "immediate": false
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x22 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "increment": true,
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x2A => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "decrement": true,
            //     "immediate": false
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x32 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "decrement": true,
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x3A => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x40 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x41 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x42 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x43 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x44 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x45 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x46 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x47 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x48 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x49 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x4A => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x4B => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x4C => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x4D => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x4E => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x4F => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x50 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x51 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x52 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x53 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x54 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x55 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x56 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x57 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x58 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x59 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x5A => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x5B => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x5C => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x5D => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x5E => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x5F => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x60 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x61 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x62 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x63 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x64 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x65 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x66 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x67 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x68 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x69 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x6A => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x6B => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x6C => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x6D => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x6E => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x6F => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x70 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x71 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x72 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x73 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x74 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x75 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x77 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x78 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x79 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x7A => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x7B => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x7C => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x7D => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x7E => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x7F => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": false
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xE2 => Ok(Instruction::LD),

            // Instruction: LD 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xF2 => Ok(Instruction::LD),

            // Instruction: LD 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "SP",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xF9 => Ok(Instruction::LD),

            // Instruction: LD 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x06 => Ok(Instruction::LD),

            // Instruction: LD 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x0E => Ok(Instruction::LD),

            // Instruction: LD 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x16 => Ok(Instruction::LD),

            // Instruction: LD 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x1E => Ok(Instruction::LD),

            // Instruction: LD 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x26 => Ok(Instruction::LD),

            // Instruction: LD 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x2E => Ok(Instruction::LD),

            // Instruction: LD 2 bytes 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x36 => Ok(Instruction::LD),

            // Instruction: LD 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x3E => Ok(Instruction::LD),

            // Instruction: LD 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   },
            //   {
            //     "name": "SP",
            //     "increment": true,
            //     "immediate": true
            //   },
            //   {
            //     "name": "e8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "0",
            //   "N": "0",
            //   "H": "H",
            //   "C": "C"
            // }
            0xF8 => Ok(Instruction::LD),

            // Instruction: LD 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "BC",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x01 => Ok(Instruction::LD),

            // Instruction: LD 3 bytes 
            // Operands: [
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": false
            //   },
            //   {
            //     "name": "SP",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x08 => Ok(Instruction::LD),

            // Instruction: LD 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "DE",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x11 => Ok(Instruction::LD),

            // Instruction: LD 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x21 => Ok(Instruction::LD),

            // Instruction: LD 3 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "SP",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n16",
            //     "bytes": 2,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x31 => Ok(Instruction::LD),

            // Instruction: LD 3 bytes 
            // Operands: [
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": false
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xEA => Ok(Instruction::LD),

            // Instruction: LD 3 bytes 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "a16",
            //     "bytes": 2,
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xFA => Ok(Instruction::LD),

            // Instruction: LDH 2 bytes 
            // Operands: [
            //   {
            //     "name": "a8",
            //     "bytes": 1,
            //     "immediate": false
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xE0 => Ok(Instruction::LDH),

            // Instruction: LDH 2 bytes 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "a8",
            //     "bytes": 1,
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xF0 => Ok(Instruction::LDH),

            // Instruction: NOP 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x00 => Ok(Instruction::NOP),

            // Instruction: OR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xB0 => Ok(Instruction::OR),

            // Instruction: OR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xB1 => Ok(Instruction::OR),

            // Instruction: OR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xB2 => Ok(Instruction::OR),

            // Instruction: OR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xB3 => Ok(Instruction::OR),

            // Instruction: OR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xB4 => Ok(Instruction::OR),

            // Instruction: OR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xB5 => Ok(Instruction::OR),

            // Instruction: OR 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xB6 => Ok(Instruction::OR),

            // Instruction: OR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xB7 => Ok(Instruction::OR),

            // Instruction: OR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xF6 => Ok(Instruction::OR),

            // Instruction: POP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "BC",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xC1 => Ok(Instruction::POP),

            // Instruction: POP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "DE",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xD1 => Ok(Instruction::POP),

            // Instruction: POP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xE1 => Ok(Instruction::POP),

            // Instruction: POP 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "AF",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "N",
            //   "H": "H",
            //   "C": "C"
            // }
            0xF1 => Ok(Instruction::POP),

            // Instruction: PREFIX 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xCB => Ok(Instruction::PREFIX),

            // Instruction: PUSH 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "BC",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xC5 => Ok(Instruction::PUSH),

            // Instruction: PUSH 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "DE",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xD5 => Ok(Instruction::PUSH),

            // Instruction: PUSH 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xE5 => Ok(Instruction::PUSH),

            // Instruction: PUSH 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "AF",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xF5 => Ok(Instruction::PUSH),

            // Instruction: RET 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "NZ",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xC0 => Ok(Instruction::RET),

            // Instruction: RET 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "Z",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xC8 => Ok(Instruction::RET),

            // Instruction: RET 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xC9 => Ok(Instruction::RET),

            // Instruction: RET 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "NC",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xD0 => Ok(Instruction::RET),

            // Instruction: RET 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xD8 => Ok(Instruction::RET),

            // Instruction: RETI 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xD9 => Ok(Instruction::RETI),

            // Instruction: RLA 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "0",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x17 => Ok(Instruction::RLA),

            // Instruction: RLCA 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "0",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x07 => Ok(Instruction::RLCA),

            // Instruction: RRA 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "0",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x1F => Ok(Instruction::RRA),

            // Instruction: RRCA 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "0",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x0F => Ok(Instruction::RRCA),

            // Instruction: RST 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "$00",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xC7 => Ok(Instruction::RST),

            // Instruction: RST 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "$08",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xCF => Ok(Instruction::RST),

            // Instruction: RST 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "$10",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xD7 => Ok(Instruction::RST),

            // Instruction: RST 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "$18",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xDF => Ok(Instruction::RST),

            // Instruction: RST 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "$20",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xE7 => Ok(Instruction::RST),

            // Instruction: RST 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "$28",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xEF => Ok(Instruction::RST),

            // Instruction: RST 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "$30",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xF7 => Ok(Instruction::RST),

            // Instruction: RST 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "$38",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xFF => Ok(Instruction::RST),

            // Instruction: SBC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x98 => Ok(Instruction::SBC),

            // Instruction: SBC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x99 => Ok(Instruction::SBC),

            // Instruction: SBC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x9A => Ok(Instruction::SBC),

            // Instruction: SBC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x9B => Ok(Instruction::SBC),

            // Instruction: SBC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x9C => Ok(Instruction::SBC),

            // Instruction: SBC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x9D => Ok(Instruction::SBC),

            // Instruction: SBC 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x9E => Ok(Instruction::SBC),

            // Instruction: SBC 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "-"
            // }
            0x9F => Ok(Instruction::SBC),

            // Instruction: SBC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0xDE => Ok(Instruction::SBC),

            // Instruction: SCF 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "0",
            //   "H": "0",
            //   "C": "1"
            // }
            0x37 => Ok(Instruction::SCF),

            // Instruction: STOP 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x10 => Ok(Instruction::STOP),

            // Instruction: SUB 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x90 => Ok(Instruction::SUB),

            // Instruction: SUB 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x91 => Ok(Instruction::SUB),

            // Instruction: SUB 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x92 => Ok(Instruction::SUB),

            // Instruction: SUB 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x93 => Ok(Instruction::SUB),

            // Instruction: SUB 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x94 => Ok(Instruction::SUB),

            // Instruction: SUB 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x95 => Ok(Instruction::SUB),

            // Instruction: SUB 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0x96 => Ok(Instruction::SUB),

            // Instruction: SUB 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "1",
            //   "N": "1",
            //   "H": "0",
            //   "C": "0"
            // }
            0x97 => Ok(Instruction::SUB),

            // Instruction: SUB 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "1",
            //   "H": "H",
            //   "C": "C"
            // }
            0xD6 => Ok(Instruction::SUB),

            // Instruction: XOR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xA8 => Ok(Instruction::XOR),

            // Instruction: XOR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xA9 => Ok(Instruction::XOR),

            // Instruction: XOR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xAA => Ok(Instruction::XOR),

            // Instruction: XOR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xAB => Ok(Instruction::XOR),

            // Instruction: XOR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xAC => Ok(Instruction::XOR),

            // Instruction: XOR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xAD => Ok(Instruction::XOR),

            // Instruction: XOR 1 byte 
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xAE => Ok(Instruction::XOR),

            // Instruction: XOR 1 byte (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "1",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xAF => Ok(Instruction::XOR),

            // Instruction: XOR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   },
            //   {
            //     "name": "n8",
            //     "bytes": 1,
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "0"
            // }
            0xEE => Ok(Instruction::XOR),



}

}
}
