use crate::{memory_bus, prelude::LittleEndian};

mod memory;
use self::memory::{Registers, Stack};

mod instructions;
pub use instructions::Instruction;

#[derive(Default)]
pub struct CPU {
    registers: Registers,
    stack: Stack,
    program: Program,
}

impl CPU {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }

    pub fn exec_fetch(&mut self, bus: memory_bus::Bus) {
        // TODO: check if the prefixed table has an instruction with opcode PREFIX_INDICATION_BYTE
        // If so: adjust this to also check if it is prefixed or not
        match self.program.next_byte() {
            Instruction::PREFIX_INDICATION_BYTE => self.program.prefixed = true,
            prefixed_opcode if self.program.is_prefixed() => todo!(),
            unprefixed_opcode => {
                Instruction::try_from_opcode_unprefixed(unprefixed_opcode, todo!()).unwrap();
            }
        };
    }
    // pub fn run_program(&mut self, program: Program) -> &mut Self {
    //     for instruction in program {
    //         self.execute(instruction);
    //     }
    //
    //     return self;
    // }
}

/// An iterator that iterates over all of the program bytes and parses them into `Instruction`s
struct Program {
    /// The Program Counter (PC) is the index into the program bytes
    pub program_counter: u16,
    /// Wether the next opcode is prefixed.
    pub prefixed: bool,
}

impl Default for Program {
    fn default() -> Self {
        return Self {
            program_counter: 0,
            prefixed: false,
        };
    }
}

impl Program {
    /// Returns the Program Counter (PC), the index into the program
    ///
    /// On the Game Boy this is a 16-bit register on the CPU,
    /// but we keep it here for separation on concerns
    pub fn pc(&self) -> u16 {
        self.program_counter
    }

    #[inline]
    /// Wether the next opcode byte is prefixed
    pub fn is_prefixed(&self) -> bool {
        self.prefixed
    }

    /// Reads the current (immediate) byte, pointed to by the `program_counter` in memory
    /// without incrementing the `program_counter`.
    pub fn read_immediate(&self) -> u8 {
        todo!()
        // self.bus.read8(self.program_counter)
    }

    /// Reads the current (immediate) byte, pointed to by the `program_counter` in memory,
    /// then increments the `program_counter` by `1`.
    pub fn next_byte(&mut self) -> u8 {
        todo!()
        // let byte = self.bus.read8(self.program_counter);
        // self.program_counter += 1;
        //
        // return byte;
    }

    /// Reads the next byte as an `i8`.
    /// Increments the `program_counter` by `1`.
    pub fn next_i8(&mut self) -> i8 {
        self.next_byte() as i8
    }

    /// Reads the next 2 bytes and combines them to create a `u16`.
    ///
    /// The `program_counter` is incremented by `2` (`1` for each byte).
    ///
    /// Read bytes:
    ///  - first byte is the lower nibble (lsb)
    ///  - second byte is the upper nibble (msb)
    pub fn next_u16(&mut self) -> u16 {
        u16::from_bytes((self.next_byte(), self.next_byte()))
    }

    /// Decodes the next instruction of the program.
    ///
    /// The `program_counter` is incremented by `n` (based on the instruction).
    ///
    /// @alias self.next()
    #[inline]
    pub fn next_instruction(&mut self) -> Option<Instruction> {
        self.next()
    }

    /// Skips the next byte in the program by incrementing the the `program_counter` by `1`
    pub fn skip_byte(&mut self) -> &mut Self {
        self.program_counter += 1;
        return self;
    }
}

impl Iterator for Program {
    type Item = Instruction;

    /// Decodes the next instruction of the program
    ///
    /// @alias self.next_instruction()
    fn next(&mut self) -> Option<Self::Item> {
        let opcode = self.next_byte();

        // TODO: check if the prefixed table has an instruction with opcode PREFIX_INDICATION_BYTE
        // If so: adjust this to also check if it is prefixed or not
        if opcode == Instruction::PREFIX_INDICATION_BYTE {
            self.prefixed = true;
            return self.next();
        }

        return Some(if self.is_prefixed() {
            // Self::try_from_opcode_prefixed(byte, program)
            todo!()
        } else {
            Instruction::try_from_opcode_unprefixed(opcode, self).unwrap()
        });
    }
}
