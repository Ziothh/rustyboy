/*
 * This file implements the CPU instructions, based on https://gekkio.fi/files/gb-docs/gbctr.pdf
 */

use crate::cpu;

impl cpu::CPU {
    /// Executes a single instruction
    pub fn execute(&mut self, instruction: cpu::Instruction) {
        match instruction {
            // Instruction::LD(load_type) => match load_type {
            //     LoadType::Byte(target, source) => {
            //         let source_value = match source {
            //             LoadByteSource::A => self.registers.a,
            //             LoadByteSource::D8 => {
            //                 // Read the next byte
            //                 self.program_counter += 1;
            //                 *self.bus.read8(self.program_counter)
            //             }
            //             LoadByteSource::HLI => *self.bus.read8(self.registers.hl()),
            //             _ => todo!("TODO: implement other sources"),
            //         };
            //         match target {
            //             LoadByteTarget::A => self.registers.a = source_value,
            //             LoadByteTarget::HLI => {
            //                 self.bus[self.registers.hl()] = source_value;
            //             }
            //             _ => todo!("TODO: implement other targets"),
            //         };
            //         return self.program_counter.wrapping_add(1);
            //     }
            //     _ => todo!("TODO: implement other load types"),
            // },
            //
            // Instruction::ADD(target) => {
            //     let (new_value, did_overflow) =
            //         self.registers.a.overflowing_add(self.registers[target]);
            //
            //     self.registers.f.zero = new_value == 0;
            //     self.registers.f.subtract = false;
            //     self.registers.f.carry = did_overflow;
            //     self.registers.f.half_carry = (self.registers.a & 0xF) + (new_value & 0xF) > 0xF;
            //
            //     self.registers.a = new_value;
            //     self.program_counter = self.program_counter.wrapping_add(1);
            // }
            //
            // Instruction::JP(test) => {
            //     let should_jump = match test {
            //         JumpTest::NotZero => !self.registers.f.zero,
            //         JumpTest::NotCarry => !self.registers.f.carry,
            //         JumpTest::Zero => self.registers.f.zero,
            //         JumpTest::Carry => self.registers.f.carry,
            //         JumpTest::Always => true,
            //     };
            //
            //     if should_jump {
            //         return self.bus.read16(self.program_counter + 1);
            //     } else {
            //         // If we don't jump we need to still move the program
            //         // counter forward by 3 since the jump instruction is
            //         // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
            //         return self.program_counter.wrapping_add(3);
            //     }
            // }
            //
            // Instruction::CALL(test) => {
            //     let should_jump = match test {
            //         JumpTest::NotZero => !self.registers.f.zero,
            //         _ => {
            //             todo!("TODO: support more conditions")
            //         }
            //     };
            //
            //     let next_pc = self.program_counter.wrapping_add(3);
            //     if should_jump {
            //         self.stack.push(next_pc, &mut self.bus);
            //         // Read the next word
            //         return self.bus.read16(self.program_counter + 1);
            //     } else {
            //         return next_pc;
            //     }
            // }
            // Instruction::RET(test) => {
            //     let should_jump = match test {
            //         JumpTest::NotZero => !self.registers.f.zero,
            //         _ => {
            //             todo!("TODO: support more conditions")
            //         }
            //     };
            //     return if should_jump {
            //         self.stack.pop(&mut self.bus)
            //     } else {
            //         self.program_counter.wrapping_add(1)
            //     };
            // }

            _ => {
                todo!();
            }
        };

        // TODO: return correct PC
        // return self.program_counter;
    }
}
