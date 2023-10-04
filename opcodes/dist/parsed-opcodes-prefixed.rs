/* NOTE: THIS FILE HAS BEEN GENERATED. DO NOT EDIT. */

impl Instruction {
    pub fn try_from_opcode_prefixed(byte: u8, program: &mut Program) -> Result<Self, String> {
        match byte {
            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            //   "C": "-"
            // }
            0x40 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            //   "C": "-"
            // }
            0x41 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            //   "C": "-"
            // }
            0x42 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            //   "C": "-"
            // }
            0x43 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            //   "C": "-"
            // }
            0x44 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            //   "C": "-"
            // }
            0x45 => Instruction::BIT,

            // Instruction: BIT 2 bytes 
            // Operands: [
            //   {
            //     "name": "0",
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
            //   "C": "-"
            // }
            0x46 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            //   "C": "-"
            // }
            0x47 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            //   "C": "-"
            // }
            0x48 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            //   "C": "-"
            // }
            0x49 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            //   "C": "-"
            // }
            0x4A => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            //   "C": "-"
            // }
            0x4B => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            //   "C": "-"
            // }
            0x4C => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            //   "C": "-"
            // }
            0x4D => Instruction::BIT,

            // Instruction: BIT 2 bytes 
            // Operands: [
            //   {
            //     "name": "1",
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
            //   "C": "-"
            // }
            0x4E => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            //   "C": "-"
            // }
            0x4F => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            //   "C": "-"
            // }
            0x50 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            //   "C": "-"
            // }
            0x51 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            //   "C": "-"
            // }
            0x52 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            //   "C": "-"
            // }
            0x53 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            //   "C": "-"
            // }
            0x54 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            //   "C": "-"
            // }
            0x55 => Instruction::BIT,

            // Instruction: BIT 2 bytes 
            // Operands: [
            //   {
            //     "name": "2",
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
            //   "C": "-"
            // }
            0x56 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            //   "C": "-"
            // }
            0x57 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            //   "C": "-"
            // }
            0x58 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            //   "C": "-"
            // }
            0x59 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            //   "C": "-"
            // }
            0x5A => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            //   "C": "-"
            // }
            0x5B => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            //   "C": "-"
            // }
            0x5C => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            //   "C": "-"
            // }
            0x5D => Instruction::BIT,

            // Instruction: BIT 2 bytes 
            // Operands: [
            //   {
            //     "name": "3",
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
            //   "C": "-"
            // }
            0x5E => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            //   "C": "-"
            // }
            0x5F => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            //   "C": "-"
            // }
            0x60 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            //   "C": "-"
            // }
            0x61 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            //   "C": "-"
            // }
            0x62 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            //   "C": "-"
            // }
            0x63 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            //   "C": "-"
            // }
            0x64 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            //   "C": "-"
            // }
            0x65 => Instruction::BIT,

            // Instruction: BIT 2 bytes 
            // Operands: [
            //   {
            //     "name": "4",
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
            //   "C": "-"
            // }
            0x66 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            //   "C": "-"
            // }
            0x67 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            //   "C": "-"
            // }
            0x68 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            //   "C": "-"
            // }
            0x69 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            //   "C": "-"
            // }
            0x6A => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            //   "C": "-"
            // }
            0x6B => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            //   "C": "-"
            // }
            0x6C => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            //   "C": "-"
            // }
            0x6D => Instruction::BIT,

            // Instruction: BIT 2 bytes 
            // Operands: [
            //   {
            //     "name": "5",
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
            //   "C": "-"
            // }
            0x6E => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            //   "C": "-"
            // }
            0x6F => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            //   "C": "-"
            // }
            0x70 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            //   "C": "-"
            // }
            0x71 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            //   "C": "-"
            // }
            0x72 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            //   "C": "-"
            // }
            0x73 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            //   "C": "-"
            // }
            0x74 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            //   "C": "-"
            // }
            0x75 => Instruction::BIT,

            // Instruction: BIT 2 bytes 
            // Operands: [
            //   {
            //     "name": "6",
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
            //   "C": "-"
            // }
            0x76 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            //   "C": "-"
            // }
            0x77 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            //   "C": "-"
            // }
            0x78 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            //   "C": "-"
            // }
            0x79 => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            //   "C": "-"
            // }
            0x7A => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            //   "C": "-"
            // }
            0x7B => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            //   "C": "-"
            // }
            0x7C => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            //   "C": "-"
            // }
            0x7D => Instruction::BIT,

            // Instruction: BIT 2 bytes 
            // Operands: [
            //   {
            //     "name": "7",
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
            //   "C": "-"
            // }
            0x7E => Instruction::BIT,

            // Instruction: BIT 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            //   "C": "-"
            // }
            0x7F => Instruction::BIT,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0x80 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0x81 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0x82 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0x83 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0x84 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0x85 => Instruction::RES,

            // Instruction: RES 2 bytes 
            // Operands: [
            //   {
            //     "name": "0",
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
            0x86 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0x87 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0x88 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0x89 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0x8A => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0x8B => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0x8C => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0x8D => Instruction::RES,

            // Instruction: RES 2 bytes 
            // Operands: [
            //   {
            //     "name": "1",
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
            0x8E => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0x8F => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0x90 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0x91 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0x92 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0x93 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0x94 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0x95 => Instruction::RES,

            // Instruction: RES 2 bytes 
            // Operands: [
            //   {
            //     "name": "2",
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
            0x96 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0x97 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0x98 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0x99 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0x9A => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0x9B => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0x9C => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0x9D => Instruction::RES,

            // Instruction: RES 2 bytes 
            // Operands: [
            //   {
            //     "name": "3",
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
            0x9E => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0x9F => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xA0 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xA1 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xA2 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xA3 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xA4 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xA5 => Instruction::RES,

            // Instruction: RES 2 bytes 
            // Operands: [
            //   {
            //     "name": "4",
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
            0xA6 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xA7 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xA8 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xA9 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xAA => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xAB => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xAC => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xAD => Instruction::RES,

            // Instruction: RES 2 bytes 
            // Operands: [
            //   {
            //     "name": "5",
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
            0xAE => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xAF => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xB0 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xB1 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xB2 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xB3 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xB4 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xB5 => Instruction::RES,

            // Instruction: RES 2 bytes 
            // Operands: [
            //   {
            //     "name": "6",
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
            0xB6 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xB7 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xB8 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xB9 => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xBA => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xBB => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xBC => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xBD => Instruction::RES,

            // Instruction: RES 2 bytes 
            // Operands: [
            //   {
            //     "name": "7",
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
            0xBE => Instruction::RES,

            // Instruction: RES 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xBF => Instruction::RES,

            // Instruction: RL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x10 => Instruction::RL,

            // Instruction: RL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x11 => Instruction::RL,

            // Instruction: RL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x12 => Instruction::RL,

            // Instruction: RL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x13 => Instruction::RL,

            // Instruction: RL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x14 => Instruction::RL,

            // Instruction: RL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x15 => Instruction::RL,

            // Instruction: RL 2 bytes 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x16 => Instruction::RL,

            // Instruction: RL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x17 => Instruction::RL,

            // Instruction: RLC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x00 => Instruction::RLC,

            // Instruction: RLC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x01 => Instruction::RLC,

            // Instruction: RLC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x02 => Instruction::RLC,

            // Instruction: RLC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x03 => Instruction::RLC,

            // Instruction: RLC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x04 => Instruction::RLC,

            // Instruction: RLC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x05 => Instruction::RLC,

            // Instruction: RLC 2 bytes 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x06 => Instruction::RLC,

            // Instruction: RLC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x07 => Instruction::RLC,

            // Instruction: RR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x18 => Instruction::RR,

            // Instruction: RR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x19 => Instruction::RR,

            // Instruction: RR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x1A => Instruction::RR,

            // Instruction: RR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x1B => Instruction::RR,

            // Instruction: RR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x1C => Instruction::RR,

            // Instruction: RR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x1D => Instruction::RR,

            // Instruction: RR 2 bytes 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x1E => Instruction::RR,

            // Instruction: RR 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x1F => Instruction::RR,

            // Instruction: RRC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x08 => Instruction::RRC,

            // Instruction: RRC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x09 => Instruction::RRC,

            // Instruction: RRC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x0A => Instruction::RRC,

            // Instruction: RRC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x0B => Instruction::RRC,

            // Instruction: RRC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x0C => Instruction::RRC,

            // Instruction: RRC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x0D => Instruction::RRC,

            // Instruction: RRC 2 bytes 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x0E => Instruction::RRC,

            // Instruction: RRC 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x0F => Instruction::RRC,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0xC0 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0xC1 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0xC2 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0xC3 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0xC4 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0xC5 => Instruction::SET,

            // Instruction: SET 2 bytes 
            // Operands: [
            //   {
            //     "name": "0",
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
            0xC6 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "0",
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
            0xC7 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0xC8 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0xC9 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0xCA => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0xCB => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0xCC => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0xCD => Instruction::SET,

            // Instruction: SET 2 bytes 
            // Operands: [
            //   {
            //     "name": "1",
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
            0xCE => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "1",
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
            0xCF => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0xD0 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0xD1 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0xD2 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0xD3 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0xD4 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0xD5 => Instruction::SET,

            // Instruction: SET 2 bytes 
            // Operands: [
            //   {
            //     "name": "2",
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
            0xD6 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "2",
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
            0xD7 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0xD8 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0xD9 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0xDA => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0xDB => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0xDC => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0xDD => Instruction::SET,

            // Instruction: SET 2 bytes 
            // Operands: [
            //   {
            //     "name": "3",
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
            0xDE => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "3",
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
            0xDF => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xE0 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xE1 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xE2 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xE3 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xE4 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xE5 => Instruction::SET,

            // Instruction: SET 2 bytes 
            // Operands: [
            //   {
            //     "name": "4",
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
            0xE6 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "4",
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
            0xE7 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xE8 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xE9 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xEA => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xEB => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xEC => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xED => Instruction::SET,

            // Instruction: SET 2 bytes 
            // Operands: [
            //   {
            //     "name": "5",
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
            0xEE => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "5",
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
            0xEF => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xF0 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xF1 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xF2 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xF3 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xF4 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xF5 => Instruction::SET,

            // Instruction: SET 2 bytes 
            // Operands: [
            //   {
            //     "name": "6",
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
            0xF6 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "6",
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
            0xF7 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xF8 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xF9 => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xFA => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xFB => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xFC => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xFD => Instruction::SET,

            // Instruction: SET 2 bytes 
            // Operands: [
            //   {
            //     "name": "7",
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
            0xFE => Instruction::SET,

            // Instruction: SET 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "7",
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
            0xFF => Instruction::SET,

            // Instruction: SLA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x20 => Instruction::SLA,

            // Instruction: SLA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x21 => Instruction::SLA,

            // Instruction: SLA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x22 => Instruction::SLA,

            // Instruction: SLA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x23 => Instruction::SLA,

            // Instruction: SLA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x24 => Instruction::SLA,

            // Instruction: SLA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x25 => Instruction::SLA,

            // Instruction: SLA 2 bytes 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x26 => Instruction::SLA,

            // Instruction: SLA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x27 => Instruction::SLA,

            // Instruction: SRA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x28 => Instruction::SRA,

            // Instruction: SRA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x29 => Instruction::SRA,

            // Instruction: SRA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x2A => Instruction::SRA,

            // Instruction: SRA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x2B => Instruction::SRA,

            // Instruction: SRA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x2C => Instruction::SRA,

            // Instruction: SRA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x2D => Instruction::SRA,

            // Instruction: SRA 2 bytes 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x2E => Instruction::SRA,

            // Instruction: SRA 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x2F => Instruction::SRA,

            // Instruction: SRL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "B",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x38 => Instruction::SRL,

            // Instruction: SRL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "C",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x39 => Instruction::SRL,

            // Instruction: SRL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "D",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x3A => Instruction::SRL,

            // Instruction: SRL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "E",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x3B => Instruction::SRL,

            // Instruction: SRL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "H",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x3C => Instruction::SRL,

            // Instruction: SRL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "L",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x3D => Instruction::SRL,

            // Instruction: SRL 2 bytes 
            // Operands: [
            //   {
            //     "name": "HL",
            //     "immediate": false
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x3E => Instruction::SRL,

            // Instruction: SRL 2 bytes (immediate)
            // Operands: [
            //   {
            //     "name": "A",
            //     "immediate": true
            //   }
            // ]
            // Flags: {
            //   "Z": "Z",
            //   "N": "0",
            //   "H": "0",
            //   "C": "C"
            // }
            0x3F => Instruction::SRL,

            // Instruction: SWAP 2 bytes (immediate)
            // Operands: [
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
            0x30 => Instruction::SWAP,

            // Instruction: SWAP 2 bytes (immediate)
            // Operands: [
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
            0x31 => Instruction::SWAP,

            // Instruction: SWAP 2 bytes (immediate)
            // Operands: [
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
            0x32 => Instruction::SWAP,

            // Instruction: SWAP 2 bytes (immediate)
            // Operands: [
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
            0x33 => Instruction::SWAP,

            // Instruction: SWAP 2 bytes (immediate)
            // Operands: [
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
            0x34 => Instruction::SWAP,

            // Instruction: SWAP 2 bytes (immediate)
            // Operands: [
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
            0x35 => Instruction::SWAP,

            // Instruction: SWAP 2 bytes 
            // Operands: [
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
            0x36 => Instruction::SWAP,

            // Instruction: SWAP 2 bytes (immediate)
            // Operands: [
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
            0x37 => Instruction::SWAP,



}

}
}