use super::{instructions::*, CPU};

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                self.registers.a = self.add(self.registers[target]);
                self.program_counter = self.program_counter.wrapping_add(1);
            }

            Instruction::JP(test) => {
                let should_jump = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };

                if should_jump {
                    return self.bus.read16(self.program_counter + 1);
                } else {
                    // If we don't jump we need to still move the program
                    // counter forward by 3 since the jump instruction is
                    // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
                    return self.program_counter.wrapping_add(3);
                }
            }

            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::D8 => {
                            // Read the next byte
                            self.program_counter += 1;
                            *self.bus.read8(self.program_counter)
                        }
                        LoadByteSource::HLI => *self.bus.read8(self.registers.hl()),
                        _ => todo!("TODO: implement other sources"),
                    };
                    match target {
                        LoadByteTarget::A => self.registers.a = source_value,
                        LoadByteTarget::HLI => {
                            self.bus[self.registers.hl()] = source_value;
                        }
                        _ => todo!("TODO: implement other targets"),
                    };
                    return self.program_counter.wrapping_add(1);
                }
                _ => todo!("TODO: implement other load types"),
            },

            _ => {
                todo!();
            }
        };

        // TODO: return correct PC
        return self.program_counter;
    }
}
