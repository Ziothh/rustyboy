/* NOTE: THIS FILE HAS BEEN GENERATED. DO NOT EDIT. */

impl Instruction {
    pub fn try_from_opcode_prefixed(byte: u8, program: &mut Program) -> Result<Self, String> {
        match byte {
            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x40 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x41 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x42 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x43 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x44 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x45 => Ok(Instruction::BIT),

            // Instruction: BIT 
            // { bytes: 2, cycles: 12 }
            // Operands: 
            //     0 (immediate),
            //     HL 
            // Flags: Z N H C
            //        Z 0 1 -
            0x46 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x47 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x48 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x49 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x4A => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x4B => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x4C => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x4D => Ok(Instruction::BIT),

            // Instruction: BIT 
            // { bytes: 2, cycles: 12 }
            // Operands: 
            //     1 (immediate),
            //     HL 
            // Flags: Z N H C
            //        Z 0 1 -
            0x4E => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x4F => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x50 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x51 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x52 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x53 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x54 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x55 => Ok(Instruction::BIT),

            // Instruction: BIT 
            // { bytes: 2, cycles: 12 }
            // Operands: 
            //     2 (immediate),
            //     HL 
            // Flags: Z N H C
            //        Z 0 1 -
            0x56 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x57 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x58 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x59 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x5A => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x5B => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x5C => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x5D => Ok(Instruction::BIT),

            // Instruction: BIT 
            // { bytes: 2, cycles: 12 }
            // Operands: 
            //     3 (immediate),
            //     HL 
            // Flags: Z N H C
            //        Z 0 1 -
            0x5E => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x5F => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x60 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x61 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x62 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x63 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x64 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x65 => Ok(Instruction::BIT),

            // Instruction: BIT 
            // { bytes: 2, cycles: 12 }
            // Operands: 
            //     4 (immediate),
            //     HL 
            // Flags: Z N H C
            //        Z 0 1 -
            0x66 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x67 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x68 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x69 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x6A => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x6B => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x6C => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x6D => Ok(Instruction::BIT),

            // Instruction: BIT 
            // { bytes: 2, cycles: 12 }
            // Operands: 
            //     5 (immediate),
            //     HL 
            // Flags: Z N H C
            //        Z 0 1 -
            0x6E => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x6F => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x70 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x71 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x72 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x73 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x74 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x75 => Ok(Instruction::BIT),

            // Instruction: BIT 
            // { bytes: 2, cycles: 12 }
            // Operands: 
            //     6 (immediate),
            //     HL 
            // Flags: Z N H C
            //        Z 0 1 -
            0x76 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x77 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x78 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x79 => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x7A => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x7B => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x7C => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x7D => Ok(Instruction::BIT),

            // Instruction: BIT 
            // { bytes: 2, cycles: 12 }
            // Operands: 
            //     7 (immediate),
            //     HL 
            // Flags: Z N H C
            //        Z 0 1 -
            0x7E => Ok(Instruction::BIT),

            // Instruction: BIT (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 1 -
            0x7F => Ok(Instruction::BIT),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x80 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x81 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x82 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x83 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x84 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x85 => Ok(Instruction::RES),

            // Instruction: RES 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     0 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0x86 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x87 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x88 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x89 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x8A => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x8B => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x8C => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x8D => Ok(Instruction::RES),

            // Instruction: RES 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     1 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0x8E => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x8F => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x90 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x91 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x92 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x93 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x94 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x95 => Ok(Instruction::RES),

            // Instruction: RES 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     2 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0x96 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x97 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0x98 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0x99 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0x9A => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0x9B => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0x9C => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0x9D => Ok(Instruction::RES),

            // Instruction: RES 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     3 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0x9E => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0x9F => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xA0 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xA1 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xA2 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xA3 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xA4 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xA5 => Ok(Instruction::RES),

            // Instruction: RES 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     4 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xA6 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xA7 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xA8 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xA9 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xAA => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xAB => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xAC => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xAD => Ok(Instruction::RES),

            // Instruction: RES 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     5 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xAE => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xAF => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xB0 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xB1 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xB2 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xB3 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xB4 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xB5 => Ok(Instruction::RES),

            // Instruction: RES 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     6 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xB6 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xB7 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xB8 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xB9 => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xBA => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xBB => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xBC => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xBD => Ok(Instruction::RES),

            // Instruction: RES 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     7 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xBE => Ok(Instruction::RES),

            // Instruction: RES (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xBF => Ok(Instruction::RES),

            // Instruction: RL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x10 => Ok(Instruction::RL),

            // Instruction: RL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x11 => Ok(Instruction::RL),

            // Instruction: RL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x12 => Ok(Instruction::RL),

            // Instruction: RL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x13 => Ok(Instruction::RL),

            // Instruction: RL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x14 => Ok(Instruction::RL),

            // Instruction: RL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x15 => Ok(Instruction::RL),

            // Instruction: RL 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     HL 
            // Flags: Z N H C
            //        Z 0 0 C
            0x16 => Ok(Instruction::RL),

            // Instruction: RL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x17 => Ok(Instruction::RL),

            // Instruction: RLC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x00 => Ok(Instruction::RLC),

            // Instruction: RLC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x01 => Ok(Instruction::RLC),

            // Instruction: RLC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x02 => Ok(Instruction::RLC),

            // Instruction: RLC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x03 => Ok(Instruction::RLC),

            // Instruction: RLC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x04 => Ok(Instruction::RLC),

            // Instruction: RLC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x05 => Ok(Instruction::RLC),

            // Instruction: RLC 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     HL 
            // Flags: Z N H C
            //        Z 0 0 C
            0x06 => Ok(Instruction::RLC),

            // Instruction: RLC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x07 => Ok(Instruction::RLC),

            // Instruction: RR (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x18 => Ok(Instruction::RR),

            // Instruction: RR (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x19 => Ok(Instruction::RR),

            // Instruction: RR (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x1A => Ok(Instruction::RR),

            // Instruction: RR (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x1B => Ok(Instruction::RR),

            // Instruction: RR (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x1C => Ok(Instruction::RR),

            // Instruction: RR (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x1D => Ok(Instruction::RR),

            // Instruction: RR 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     HL 
            // Flags: Z N H C
            //        Z 0 0 C
            0x1E => Ok(Instruction::RR),

            // Instruction: RR (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x1F => Ok(Instruction::RR),

            // Instruction: RRC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x08 => Ok(Instruction::RRC),

            // Instruction: RRC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x09 => Ok(Instruction::RRC),

            // Instruction: RRC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x0A => Ok(Instruction::RRC),

            // Instruction: RRC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x0B => Ok(Instruction::RRC),

            // Instruction: RRC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x0C => Ok(Instruction::RRC),

            // Instruction: RRC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x0D => Ok(Instruction::RRC),

            // Instruction: RRC 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     HL 
            // Flags: Z N H C
            //        Z 0 0 C
            0x0E => Ok(Instruction::RRC),

            // Instruction: RRC (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x0F => Ok(Instruction::RRC),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC0 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC1 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC2 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC3 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC4 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC5 => Ok(Instruction::SET),

            // Instruction: SET 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     0 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xC6 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     0 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC7 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC8 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xC9 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCA => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCB => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCC => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCD => Ok(Instruction::SET),

            // Instruction: SET 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     1 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xCE => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     1 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xCF => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD0 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD1 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD2 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD3 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD4 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD5 => Ok(Instruction::SET),

            // Instruction: SET 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     2 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xD6 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     2 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD7 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD8 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xD9 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xDA => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xDB => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xDC => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xDD => Ok(Instruction::SET),

            // Instruction: SET 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     3 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xDE => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     3 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xDF => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE0 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE1 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE2 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE3 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE4 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE5 => Ok(Instruction::SET),

            // Instruction: SET 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     4 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xE6 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     4 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE7 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE8 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xE9 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xEA => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xEB => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xEC => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xED => Ok(Instruction::SET),

            // Instruction: SET 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     5 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xEE => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     5 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xEF => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF0 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF1 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF2 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF3 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF4 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF5 => Ok(Instruction::SET),

            // Instruction: SET 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     6 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xF6 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     6 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF7 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     B (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF8 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     C (immediate)
            // Flags: Z N H C
            //        - - - -
            0xF9 => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     D (immediate)
            // Flags: Z N H C
            //        - - - -
            0xFA => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     E (immediate)
            // Flags: Z N H C
            //        - - - -
            0xFB => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     H (immediate)
            // Flags: Z N H C
            //        - - - -
            0xFC => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     L (immediate)
            // Flags: Z N H C
            //        - - - -
            0xFD => Ok(Instruction::SET),

            // Instruction: SET 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     7 (immediate),
            //     HL 
            // Flags: Z N H C
            //        - - - -
            0xFE => Ok(Instruction::SET),

            // Instruction: SET (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     7 (immediate),
            //     A (immediate)
            // Flags: Z N H C
            //        - - - -
            0xFF => Ok(Instruction::SET),

            // Instruction: SLA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x20 => Ok(Instruction::SLA),

            // Instruction: SLA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x21 => Ok(Instruction::SLA),

            // Instruction: SLA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x22 => Ok(Instruction::SLA),

            // Instruction: SLA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x23 => Ok(Instruction::SLA),

            // Instruction: SLA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x24 => Ok(Instruction::SLA),

            // Instruction: SLA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x25 => Ok(Instruction::SLA),

            // Instruction: SLA 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     HL 
            // Flags: Z N H C
            //        Z 0 0 C
            0x26 => Ok(Instruction::SLA),

            // Instruction: SLA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x27 => Ok(Instruction::SLA),

            // Instruction: SRA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x28 => Ok(Instruction::SRA),

            // Instruction: SRA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x29 => Ok(Instruction::SRA),

            // Instruction: SRA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x2A => Ok(Instruction::SRA),

            // Instruction: SRA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x2B => Ok(Instruction::SRA),

            // Instruction: SRA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x2C => Ok(Instruction::SRA),

            // Instruction: SRA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x2D => Ok(Instruction::SRA),

            // Instruction: SRA 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     HL 
            // Flags: Z N H C
            //        Z 0 0 C
            0x2E => Ok(Instruction::SRA),

            // Instruction: SRA (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x2F => Ok(Instruction::SRA),

            // Instruction: SRL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x38 => Ok(Instruction::SRL),

            // Instruction: SRL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x39 => Ok(Instruction::SRL),

            // Instruction: SRL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x3A => Ok(Instruction::SRL),

            // Instruction: SRL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x3B => Ok(Instruction::SRL),

            // Instruction: SRL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x3C => Ok(Instruction::SRL),

            // Instruction: SRL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x3D => Ok(Instruction::SRL),

            // Instruction: SRL 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     HL 
            // Flags: Z N H C
            //        Z 0 0 C
            0x3E => Ok(Instruction::SRL),

            // Instruction: SRL (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 0 C
            0x3F => Ok(Instruction::SRL),

            // Instruction: SWAP (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     B (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0x30 => Ok(Instruction::SWAP),

            // Instruction: SWAP (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     C (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0x31 => Ok(Instruction::SWAP),

            // Instruction: SWAP (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     D (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0x32 => Ok(Instruction::SWAP),

            // Instruction: SWAP (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     E (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0x33 => Ok(Instruction::SWAP),

            // Instruction: SWAP (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     H (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0x34 => Ok(Instruction::SWAP),

            // Instruction: SWAP (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     L (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0x35 => Ok(Instruction::SWAP),

            // Instruction: SWAP 
            // { bytes: 2, cycles: 16 }
            // Operands: 
            //     HL 
            // Flags: Z N H C
            //        Z 0 0 0
            0x36 => Ok(Instruction::SWAP),

            // Instruction: SWAP (immediate)
            // { bytes: 2, cycles: 8 }
            // Operands: 
            //     A (immediate)
            // Flags: Z N H C
            //        Z 0 0 0
            0x37 => Ok(Instruction::SWAP),



}

}
}