use crate::{CPU::Instruction, program::Program};

impl Instruction {
    pub fn try_from_opcode_unprefixed(byte: u8, program: &mut Program) -> Result<Self, String> {
        match byte {
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
            0x88 => Instruction::ADC,

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
            0x89 => Instruction::ADC,

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
            0x8A => Instruction::ADC,

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
            0x8B => Instruction::ADC,

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
            0x8C => Instruction::ADC,

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
            0x8D => Instruction::ADC,

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
            0x8E => Instruction::ADC,

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
            0x8F => Instruction::ADC,

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
            0xCE => Instruction::ADC,

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
            0x09 => Instruction::ADD,

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
            0x19 => Instruction::ADD,

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
            0x29 => Instruction::ADD,

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
            0x39 => Instruction::ADD,

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
            0x80 => Instruction::ADD,

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
            0x81 => Instruction::ADD,

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
            0x82 => Instruction::ADD,

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
            0x83 => Instruction::ADD,

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
            0x84 => Instruction::ADD,

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
            0x85 => Instruction::ADD,

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
            0x86 => Instruction::ADD,

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
            0x87 => Instruction::ADD,

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
            0xC6 => Instruction::ADD,

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
            0xE8 => Instruction::ADD,

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
            0xA0 => Instruction::AND,

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
            0xA1 => Instruction::AND,

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
            0xA2 => Instruction::AND,

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
            0xA3 => Instruction::AND,

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
            0xA4 => Instruction::AND,

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
            0xA5 => Instruction::AND,

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
            0xA6 => Instruction::AND,

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
            0xA7 => Instruction::AND,

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
            0xE6 => Instruction::AND,

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
            0xC4 => Instruction::CALL,

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
            0xCC => Instruction::CALL,

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
            0xCD => Instruction::CALL,

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
            0xD4 => Instruction::CALL,

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
            0xDC => Instruction::CALL,

            // Instruction: CCF 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x3F => Instruction::CCF,

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
            0xB8 => Instruction::CP,

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
            0xB9 => Instruction::CP,

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
            0xBA => Instruction::CP,

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
            0xBB => Instruction::CP,

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
            0xBC => Instruction::CP,

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
            0xBD => Instruction::CP,

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
            0xBE => Instruction::CP,

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
            0xBF => Instruction::CP,

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
            0xFE => Instruction::CP,

            // Instruction: CPL 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "1",
            //   "H": "1",
            //   "C": "-"
            // }
            0x2F => Instruction::CPL,

            // Instruction: DAA 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "Z",
            //   "N": "-",
            //   "H": "0",
            //   "C": "C"
            // }
            0x27 => Instruction::DAA,

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
            0x05 => Instruction::DEC,

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
            0x0B => Instruction::DEC,

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
            0x0D => Instruction::DEC,

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
            0x15 => Instruction::DEC,

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
            0x1B => Instruction::DEC,

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
            0x1D => Instruction::DEC,

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
            0x25 => Instruction::DEC,

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
            0x2B => Instruction::DEC,

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
            0x2D => Instruction::DEC,

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
            0x35 => Instruction::DEC,

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
            0x3B => Instruction::DEC,

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
            0x3D => Instruction::DEC,

            // Instruction: DI 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xF3 => Instruction::DI,

            // Instruction: EI 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xFB => Instruction::EI,

            // Instruction: HALT 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x76 => Instruction::HALT,

            // Instruction: ILLEGAL_D3 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xD3 => Instruction::ILLEGAL_D3,

            // Instruction: ILLEGAL_DB 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xDB => Instruction::ILLEGAL_DB,

            // Instruction: ILLEGAL_DD 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xDD => Instruction::ILLEGAL_DD,

            // Instruction: ILLEGAL_E3 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xE3 => Instruction::ILLEGAL_E3,

            // Instruction: ILLEGAL_E4 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xE4 => Instruction::ILLEGAL_E4,

            // Instruction: ILLEGAL_EB 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xEB => Instruction::ILLEGAL_EB,

            // Instruction: ILLEGAL_EC 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xEC => Instruction::ILLEGAL_EC,

            // Instruction: ILLEGAL_ED 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xED => Instruction::ILLEGAL_ED,

            // Instruction: ILLEGAL_F4 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xF4 => Instruction::ILLEGAL_F4,

            // Instruction: ILLEGAL_FC 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xFC => Instruction::ILLEGAL_FC,

            // Instruction: ILLEGAL_FD 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xFD => Instruction::ILLEGAL_FD,

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
            0x03 => Instruction::INC,

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
            0x04 => Instruction::INC,

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
            0x0C => Instruction::INC,

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
            0x13 => Instruction::INC,

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
            0x14 => Instruction::INC,

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
            0x1C => Instruction::INC,

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
            0x23 => Instruction::INC,

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
            0x24 => Instruction::INC,

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
            0x2C => Instruction::INC,

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
            0x33 => Instruction::INC,

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
            0x34 => Instruction::INC,

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
            0x3C => Instruction::INC,

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
            0xE9 => Instruction::JP,

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
            0xC2 => Instruction::JP,

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
            0xC3 => Instruction::JP,

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
            0xCA => Instruction::JP,

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
            0xD2 => Instruction::JP,

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
            0xDA => Instruction::JP,

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
            0x18 => Instruction::JR,

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
            0x20 => Instruction::JR,

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
            0x28 => Instruction::JR,

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
            0x30 => Instruction::JR,

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
            0x38 => Instruction::JR,

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
            0x02 => Instruction::LD,

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
            0x0A => Instruction::LD,

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
            0x12 => Instruction::LD,

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
            0x1A => Instruction::LD,

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
            0x22 => Instruction::LD,

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
            0x2A => Instruction::LD,

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
            0x32 => Instruction::LD,

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
            0x3A => Instruction::LD,

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
            0x40 => Instruction::LD,

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
            0x41 => Instruction::LD,

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
            0x42 => Instruction::LD,

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
            0x43 => Instruction::LD,

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
            0x44 => Instruction::LD,

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
            0x45 => Instruction::LD,

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
            0x46 => Instruction::LD,

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
            0x47 => Instruction::LD,

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
            0x48 => Instruction::LD,

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
            0x49 => Instruction::LD,

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
            0x4A => Instruction::LD,

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
            0x4B => Instruction::LD,

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
            0x4C => Instruction::LD,

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
            0x4D => Instruction::LD,

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
            0x4E => Instruction::LD,

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
            0x4F => Instruction::LD,

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
            0x50 => Instruction::LD,

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
            0x51 => Instruction::LD,

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
            0x52 => Instruction::LD,

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
            0x53 => Instruction::LD,

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
            0x54 => Instruction::LD,

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
            0x55 => Instruction::LD,

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
            0x56 => Instruction::LD,

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
            0x57 => Instruction::LD,

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
            0x58 => Instruction::LD,

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
            0x59 => Instruction::LD,

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
            0x5A => Instruction::LD,

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
            0x5B => Instruction::LD,

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
            0x5C => Instruction::LD,

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
            0x5D => Instruction::LD,

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
            0x5E => Instruction::LD,

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
            0x5F => Instruction::LD,

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
            0x60 => Instruction::LD,

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
            0x61 => Instruction::LD,

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
            0x62 => Instruction::LD,

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
            0x63 => Instruction::LD,

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
            0x64 => Instruction::LD,

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
            0x65 => Instruction::LD,

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
            0x66 => Instruction::LD,

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
            0x67 => Instruction::LD,

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
            0x68 => Instruction::LD,

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
            0x69 => Instruction::LD,

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
            0x6A => Instruction::LD,

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
            0x6B => Instruction::LD,

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
            0x6C => Instruction::LD,

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
            0x6D => Instruction::LD,

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
            0x6E => Instruction::LD,

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
            0x6F => Instruction::LD,

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
            0x70 => Instruction::LD,

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
            0x71 => Instruction::LD,

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
            0x72 => Instruction::LD,

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
            0x73 => Instruction::LD,

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
            0x74 => Instruction::LD,

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
            0x75 => Instruction::LD,

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
            0x77 => Instruction::LD,

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
            0x78 => Instruction::LD,

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
            0x79 => Instruction::LD,

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
            0x7A => Instruction::LD,

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
            0x7B => Instruction::LD,

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
            0x7C => Instruction::LD,

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
            0x7D => Instruction::LD,

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
            0x7E => Instruction::LD,

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
            0x7F => Instruction::LD,

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
            0xE2 => Instruction::LD,

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
            0xF2 => Instruction::LD,

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
            0xF9 => Instruction::LD,

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
            0x06 => Instruction::LD,

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
            0x0E => Instruction::LD,

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
            0x16 => Instruction::LD,

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
            0x1E => Instruction::LD,

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
            0x26 => Instruction::LD,

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
            0x2E => Instruction::LD,

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
            0x36 => Instruction::LD,

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
            0x3E => Instruction::LD,

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
            0xF8 => Instruction::LD,

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
            0x01 => Instruction::LD,

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
            0x08 => Instruction::LD,

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
            0x11 => Instruction::LD,

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
            0x21 => Instruction::LD,

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
            0x31 => Instruction::LD,

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
            0xEA => Instruction::LD,

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
            0xFA => Instruction::LD,

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
            0xE0 => Instruction::LDH,

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
            0xF0 => Instruction::LDH,

            // Instruction: NOP 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0x00 => Instruction::NOP,

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
            0xB0 => Instruction::OR,

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
            0xB1 => Instruction::OR,

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
            0xB2 => Instruction::OR,

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
            0xB3 => Instruction::OR,

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
            0xB4 => Instruction::OR,

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
            0xB5 => Instruction::OR,

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
            0xB6 => Instruction::OR,

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
            0xB7 => Instruction::OR,

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
            0xF6 => Instruction::OR,

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
            0xC1 => Instruction::POP,

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
            0xD1 => Instruction::POP,

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
            0xE1 => Instruction::POP,

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
            0xF1 => Instruction::POP,

            // Instruction: PREFIX 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xCB => Instruction::PREFIX,

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
            0xC5 => Instruction::PUSH,

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
            0xD5 => Instruction::PUSH,

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
            0xE5 => Instruction::PUSH,

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
            0xF5 => Instruction::PUSH,

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
            0xC0 => Instruction::RET,

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
            0xC8 => Instruction::RET,

            // Instruction: RET 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xC9 => Instruction::RET,

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
            0xD0 => Instruction::RET,

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
            0xD8 => Instruction::RET,

            // Instruction: RETI 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "-",
            //   "H": "-",
            //   "C": "-"
            // }
            0xD9 => Instruction::RETI,

            // Instruction: RLA 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "0",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x17 => Instruction::RLA,

            // Instruction: RLCA 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "0",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x07 => Instruction::RLCA,

            // Instruction: RRA 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "0",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x1F => Instruction::RRA,

            // Instruction: RRCA 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "0",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x0F => Instruction::RRCA,

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
            0xC7 => Instruction::RST,

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
            0xCF => Instruction::RST,

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
            0xD7 => Instruction::RST,

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
            0xDF => Instruction::RST,

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
            0xE7 => Instruction::RST,

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
            0xEF => Instruction::RST,

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
            0xF7 => Instruction::RST,

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
            0xFF => Instruction::RST,

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
            0x98 => Instruction::SBC,

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
            0x99 => Instruction::SBC,

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
            0x9A => Instruction::SBC,

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
            0x9B => Instruction::SBC,

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
            0x9C => Instruction::SBC,

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
            0x9D => Instruction::SBC,

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
            0x9E => Instruction::SBC,

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
            0x9F => Instruction::SBC,

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
            0xDE => Instruction::SBC,

            // Instruction: SCF 1 byte (immediate)
            // Operands: []
            // Flags: {
            //   "Z": "-",
            //   "N": "0",
            //   "H": "0",
            //   "C": "1"
            // }
            0x37 => Instruction::SCF,

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
            0x10 => Instruction::STOP,

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
            0x90 => Instruction::SUB,

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
            0x91 => Instruction::SUB,

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
            0x92 => Instruction::SUB,

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
            0x93 => Instruction::SUB,

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
            0x94 => Instruction::SUB,

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
            0x95 => Instruction::SUB,

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
            0x96 => Instruction::SUB,

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
            0x97 => Instruction::SUB,

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
            0xD6 => Instruction::SUB,

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
            0xA8 => Instruction::XOR,

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
            0xA9 => Instruction::XOR,

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
            0xAA => Instruction::XOR,

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
            0xAB => Instruction::XOR,

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
            0xAC => Instruction::XOR,

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
            0xAD => Instruction::XOR,

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
            0xAE => Instruction::XOR,

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
            0xAF => Instruction::XOR,

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
            0xEE => Instruction::XOR,



}

}
}
