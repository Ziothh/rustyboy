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
            0x40 => Ok(Instruction::BIT),

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
            0x41 => Ok(Instruction::BIT),

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
            0x42 => Ok(Instruction::BIT),

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
            0x43 => Ok(Instruction::BIT),

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
            0x44 => Ok(Instruction::BIT),

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
            0x45 => Ok(Instruction::BIT),

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
            0x46 => Ok(Instruction::BIT),

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
            0x47 => Ok(Instruction::BIT),

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
            0x48 => Ok(Instruction::BIT),

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
            0x49 => Ok(Instruction::BIT),

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
            0x4A => Ok(Instruction::BIT),

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
            0x4B => Ok(Instruction::BIT),

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
            0x4C => Ok(Instruction::BIT),

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
            0x4D => Ok(Instruction::BIT),

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
            0x4E => Ok(Instruction::BIT),

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
            0x4F => Ok(Instruction::BIT),

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
            0x50 => Ok(Instruction::BIT),

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
            0x51 => Ok(Instruction::BIT),

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
            0x52 => Ok(Instruction::BIT),

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
            0x53 => Ok(Instruction::BIT),

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
            0x54 => Ok(Instruction::BIT),

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
            0x55 => Ok(Instruction::BIT),

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
            0x56 => Ok(Instruction::BIT),

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
            0x57 => Ok(Instruction::BIT),

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
            0x58 => Ok(Instruction::BIT),

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
            0x59 => Ok(Instruction::BIT),

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
            0x5A => Ok(Instruction::BIT),

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
            0x5B => Ok(Instruction::BIT),

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
            0x5C => Ok(Instruction::BIT),

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
            0x5D => Ok(Instruction::BIT),

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
            0x5E => Ok(Instruction::BIT),

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
            0x5F => Ok(Instruction::BIT),

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
            0x60 => Ok(Instruction::BIT),

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
            0x61 => Ok(Instruction::BIT),

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
            0x62 => Ok(Instruction::BIT),

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
            0x63 => Ok(Instruction::BIT),

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
            0x64 => Ok(Instruction::BIT),

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
            0x65 => Ok(Instruction::BIT),

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
            0x66 => Ok(Instruction::BIT),

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
            0x67 => Ok(Instruction::BIT),

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
            0x68 => Ok(Instruction::BIT),

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
            0x69 => Ok(Instruction::BIT),

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
            0x6A => Ok(Instruction::BIT),

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
            0x6B => Ok(Instruction::BIT),

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
            0x6C => Ok(Instruction::BIT),

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
            0x6D => Ok(Instruction::BIT),

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
            0x6E => Ok(Instruction::BIT),

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
            0x6F => Ok(Instruction::BIT),

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
            0x70 => Ok(Instruction::BIT),

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
            0x71 => Ok(Instruction::BIT),

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
            0x72 => Ok(Instruction::BIT),

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
            0x73 => Ok(Instruction::BIT),

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
            0x74 => Ok(Instruction::BIT),

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
            0x75 => Ok(Instruction::BIT),

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
            0x76 => Ok(Instruction::BIT),

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
            0x77 => Ok(Instruction::BIT),

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
            0x78 => Ok(Instruction::BIT),

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
            0x79 => Ok(Instruction::BIT),

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
            0x7A => Ok(Instruction::BIT),

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
            0x7B => Ok(Instruction::BIT),

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
            0x7C => Ok(Instruction::BIT),

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
            0x7D => Ok(Instruction::BIT),

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
            0x7E => Ok(Instruction::BIT),

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
            0x7F => Ok(Instruction::BIT),

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
            0x80 => Ok(Instruction::RES),

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
            0x81 => Ok(Instruction::RES),

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
            0x82 => Ok(Instruction::RES),

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
            0x83 => Ok(Instruction::RES),

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
            0x84 => Ok(Instruction::RES),

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
            0x85 => Ok(Instruction::RES),

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
            0x86 => Ok(Instruction::RES),

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
            0x87 => Ok(Instruction::RES),

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
            0x88 => Ok(Instruction::RES),

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
            0x89 => Ok(Instruction::RES),

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
            0x8A => Ok(Instruction::RES),

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
            0x8B => Ok(Instruction::RES),

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
            0x8C => Ok(Instruction::RES),

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
            0x8D => Ok(Instruction::RES),

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
            0x8E => Ok(Instruction::RES),

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
            0x8F => Ok(Instruction::RES),

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
            0x90 => Ok(Instruction::RES),

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
            0x91 => Ok(Instruction::RES),

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
            0x92 => Ok(Instruction::RES),

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
            0x93 => Ok(Instruction::RES),

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
            0x94 => Ok(Instruction::RES),

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
            0x95 => Ok(Instruction::RES),

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
            0x96 => Ok(Instruction::RES),

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
            0x97 => Ok(Instruction::RES),

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
            0x98 => Ok(Instruction::RES),

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
            0x99 => Ok(Instruction::RES),

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
            0x9A => Ok(Instruction::RES),

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
            0x9B => Ok(Instruction::RES),

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
            0x9C => Ok(Instruction::RES),

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
            0x9D => Ok(Instruction::RES),

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
            0x9E => Ok(Instruction::RES),

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
            0x9F => Ok(Instruction::RES),

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
            0xA0 => Ok(Instruction::RES),

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
            0xA1 => Ok(Instruction::RES),

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
            0xA2 => Ok(Instruction::RES),

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
            0xA3 => Ok(Instruction::RES),

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
            0xA4 => Ok(Instruction::RES),

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
            0xA5 => Ok(Instruction::RES),

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
            0xA6 => Ok(Instruction::RES),

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
            0xA7 => Ok(Instruction::RES),

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
            0xA8 => Ok(Instruction::RES),

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
            0xA9 => Ok(Instruction::RES),

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
            0xAA => Ok(Instruction::RES),

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
            0xAB => Ok(Instruction::RES),

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
            0xAC => Ok(Instruction::RES),

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
            0xAD => Ok(Instruction::RES),

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
            0xAE => Ok(Instruction::RES),

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
            0xAF => Ok(Instruction::RES),

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
            0xB0 => Ok(Instruction::RES),

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
            0xB1 => Ok(Instruction::RES),

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
            0xB2 => Ok(Instruction::RES),

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
            0xB3 => Ok(Instruction::RES),

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
            0xB4 => Ok(Instruction::RES),

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
            0xB5 => Ok(Instruction::RES),

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
            0xB6 => Ok(Instruction::RES),

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
            0xB7 => Ok(Instruction::RES),

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
            0xB8 => Ok(Instruction::RES),

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
            0xB9 => Ok(Instruction::RES),

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
            0xBA => Ok(Instruction::RES),

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
            0xBB => Ok(Instruction::RES),

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
            0xBC => Ok(Instruction::RES),

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
            0xBD => Ok(Instruction::RES),

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
            0xBE => Ok(Instruction::RES),

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
            0xBF => Ok(Instruction::RES),

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
            0x10 => Ok(Instruction::RL),

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
            0x11 => Ok(Instruction::RL),

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
            0x12 => Ok(Instruction::RL),

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
            0x13 => Ok(Instruction::RL),

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
            0x14 => Ok(Instruction::RL),

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
            0x15 => Ok(Instruction::RL),

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
            0x16 => Ok(Instruction::RL),

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
            0x17 => Ok(Instruction::RL),

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
            0x00 => Ok(Instruction::RLC),

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
            0x01 => Ok(Instruction::RLC),

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
            0x02 => Ok(Instruction::RLC),

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
            0x03 => Ok(Instruction::RLC),

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
            0x04 => Ok(Instruction::RLC),

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
            0x05 => Ok(Instruction::RLC),

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
            0x06 => Ok(Instruction::RLC),

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
            0x07 => Ok(Instruction::RLC),

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
            0x18 => Ok(Instruction::RR),

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
            0x19 => Ok(Instruction::RR),

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
            0x1A => Ok(Instruction::RR),

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
            0x1B => Ok(Instruction::RR),

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
            0x1C => Ok(Instruction::RR),

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
            0x1D => Ok(Instruction::RR),

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
            0x1E => Ok(Instruction::RR),

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
            0x1F => Ok(Instruction::RR),

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
            0x08 => Ok(Instruction::RRC),

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
            0x09 => Ok(Instruction::RRC),

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
            0x0A => Ok(Instruction::RRC),

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
            0x0B => Ok(Instruction::RRC),

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
            0x0C => Ok(Instruction::RRC),

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
            0x0D => Ok(Instruction::RRC),

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
            0x0E => Ok(Instruction::RRC),

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
            0x0F => Ok(Instruction::RRC),

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
            0xC0 => Ok(Instruction::SET),

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
            0xC1 => Ok(Instruction::SET),

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
            0xC2 => Ok(Instruction::SET),

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
            0xC3 => Ok(Instruction::SET),

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
            0xC4 => Ok(Instruction::SET),

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
            0xC5 => Ok(Instruction::SET),

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
            0xC6 => Ok(Instruction::SET),

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
            0xC7 => Ok(Instruction::SET),

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
            0xC8 => Ok(Instruction::SET),

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
            0xC9 => Ok(Instruction::SET),

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
            0xCA => Ok(Instruction::SET),

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
            0xCB => Ok(Instruction::SET),

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
            0xCC => Ok(Instruction::SET),

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
            0xCD => Ok(Instruction::SET),

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
            0xCE => Ok(Instruction::SET),

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
            0xCF => Ok(Instruction::SET),

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
            0xD0 => Ok(Instruction::SET),

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
            0xD1 => Ok(Instruction::SET),

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
            0xD2 => Ok(Instruction::SET),

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
            0xD3 => Ok(Instruction::SET),

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
            0xD4 => Ok(Instruction::SET),

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
            0xD5 => Ok(Instruction::SET),

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
            0xD6 => Ok(Instruction::SET),

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
            0xD7 => Ok(Instruction::SET),

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
            0xD8 => Ok(Instruction::SET),

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
            0xD9 => Ok(Instruction::SET),

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
            0xDA => Ok(Instruction::SET),

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
            0xDB => Ok(Instruction::SET),

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
            0xDC => Ok(Instruction::SET),

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
            0xDD => Ok(Instruction::SET),

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
            0xDE => Ok(Instruction::SET),

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
            0xDF => Ok(Instruction::SET),

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
            0xE0 => Ok(Instruction::SET),

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
            0xE1 => Ok(Instruction::SET),

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
            0xE2 => Ok(Instruction::SET),

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
            0xE3 => Ok(Instruction::SET),

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
            0xE4 => Ok(Instruction::SET),

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
            0xE5 => Ok(Instruction::SET),

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
            0xE6 => Ok(Instruction::SET),

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
            0xE7 => Ok(Instruction::SET),

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
            0xE8 => Ok(Instruction::SET),

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
            0xE9 => Ok(Instruction::SET),

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
            0xEA => Ok(Instruction::SET),

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
            0xEB => Ok(Instruction::SET),

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
            0xEC => Ok(Instruction::SET),

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
            0xED => Ok(Instruction::SET),

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
            0xEE => Ok(Instruction::SET),

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
            0xEF => Ok(Instruction::SET),

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
            0xF0 => Ok(Instruction::SET),

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
            0xF1 => Ok(Instruction::SET),

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
            0xF2 => Ok(Instruction::SET),

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
            0xF3 => Ok(Instruction::SET),

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
            0xF4 => Ok(Instruction::SET),

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
            0xF5 => Ok(Instruction::SET),

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
            0xF6 => Ok(Instruction::SET),

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
            0xF7 => Ok(Instruction::SET),

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
            0xF8 => Ok(Instruction::SET),

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
            0xF9 => Ok(Instruction::SET),

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
            0xFA => Ok(Instruction::SET),

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
            0xFB => Ok(Instruction::SET),

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
            0xFC => Ok(Instruction::SET),

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
            0xFD => Ok(Instruction::SET),

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
            0xFE => Ok(Instruction::SET),

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
            0xFF => Ok(Instruction::SET),

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
            0x20 => Ok(Instruction::SLA),

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
            0x21 => Ok(Instruction::SLA),

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
            0x22 => Ok(Instruction::SLA),

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
            0x23 => Ok(Instruction::SLA),

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
            0x24 => Ok(Instruction::SLA),

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
            0x25 => Ok(Instruction::SLA),

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
            0x26 => Ok(Instruction::SLA),

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
            0x27 => Ok(Instruction::SLA),

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
            0x28 => Ok(Instruction::SRA),

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
            0x29 => Ok(Instruction::SRA),

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
            0x2A => Ok(Instruction::SRA),

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
            0x2B => Ok(Instruction::SRA),

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
            0x2C => Ok(Instruction::SRA),

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
            0x2D => Ok(Instruction::SRA),

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
            0x2E => Ok(Instruction::SRA),

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
            0x2F => Ok(Instruction::SRA),

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
            0x38 => Ok(Instruction::SRL),

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
            0x39 => Ok(Instruction::SRL),

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
            0x3A => Ok(Instruction::SRL),

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
            0x3B => Ok(Instruction::SRL),

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
            0x3C => Ok(Instruction::SRL),

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
            0x3D => Ok(Instruction::SRL),

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
            0x3E => Ok(Instruction::SRL),

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
            0x3F => Ok(Instruction::SRL),

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
            0x30 => Ok(Instruction::SWAP),

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
            0x31 => Ok(Instruction::SWAP),

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
            0x32 => Ok(Instruction::SWAP),

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
            0x33 => Ok(Instruction::SWAP),

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
            0x34 => Ok(Instruction::SWAP),

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
            0x35 => Ok(Instruction::SWAP),

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
            0x36 => Ok(Instruction::SWAP),

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
            0x37 => Ok(Instruction::SWAP),



}

}
}