use super::{
    super::memory::{AddrPtr, Immediate, Reg16::*, Reg8::*},
    Instruction,
};

fn t() {}

impl Instruction {
    const UNPREFIXED: [Self; u8::MAX as usize + 1] = [
        Instruction::new("NOP", |cpu, bus| {
            cpu.nop();
            return &[4];
        }), // 0x00
        Instruction::new("LD BC,n16", |cpu, bus| {
            cpu.ld(bus, BC, Immediate::n16);
            return &[12];
        }), // 0x01
        Instruction::new("LD [BC],A", |cpu, bus| {
            cpu.ld(bus, AddrPtr::BC, A);
            return &[8];
        }), // 0x02
        Instruction::new("INC BC", |cpu, bus| {
            cpu.inc16(bus, BC);
            return &[8];
        }), // 0x03
        Instruction::new("INC B", |cpu, bus| {
            cpu.inc(bus, B);
            return &[4];
        }), // 0x04
        Instruction::new("DEC B", |cpu, bus| {
            cpu.dec(bus, B);
            return &[4];
        }), // 0x05
        Instruction::new("LD B,n8", |cpu, bus| {
            cpu.ld(bus, B, Immediate::n8);
            return &[8];
        }), // 0x06
        Instruction::new("RLCA", |cpu, bus| {
            // cpu.rlca();
            cpu.rlc(bus, A);
            return &[4];
        }), // 0x07
        Instruction::new("LD [a16],SP", |cpu, bus| {
            // cpu.ld([a16],SP);
            let addr = cpu.next_u16(bus);
            let [lsb, msb] = cpu.registers.sp().to_le_bytes();
            cpu.ld(bus, AddrPtr::a16(addr), lsb);
            cpu.ld(bus, AddrPtr::a16(addr + 1), msb);
            return &[20];
        }), // 0x08
        Instruction::new("ADD HL,BC", |cpu, bus| {
            cpu.add_hl(bus, BC);
            return &[8];
        }), // 0x09
        Instruction::new("LD A,[BC]", |cpu, bus| {
            cpu.ld(bus, A, AddrPtr::BC);
            return &[8];
        }), // 0x0A
        Instruction::new("DEC BC", |cpu, bus| {
            cpu.dec16(bus, BC);
            return &[8];
        }), // 0x0B
        Instruction::new("INC C", |cpu, bus| {
            cpu.inc(bus, C);
            return &[4];
        }), // 0x0C
        Instruction::new("DEC C", |cpu, bus| {
            cpu.dec(bus, C);
            return &[4];
        }), // 0x0D
        Instruction::new("LD C,n8", |cpu, bus| {
            cpu.ld(bus, C, Immediate::n8);
            return &[8];
        }), // 0x0E
        Instruction::new("RRCA", |cpu, bus| {
            // cpu.rrca();
            cpu.rrc(bus, A);
            return &[4];
        }), // 0x0F
        Instruction::new("STOP n8", |cpu, bus| {
            // cpu.stop(n8);
            let _ = cpu.next_byte(bus);
            cpu.stop();
            return &[4];
        }), // 0x10
        Instruction::new("LD DE,n16", |cpu, bus| {
            cpu.ld(bus, DE, Immediate::n16);
            return &[12];
        }), // 0x11
        Instruction::new("LD [DE],A", |cpu, bus| {
            cpu.ld(bus, AddrPtr::DE, A);
            return &[8];
        }), // 0x12
        Instruction::new("INC DE", |cpu, bus| {
            cpu.inc16(bus, DE);
            return &[8];
        }), // 0x13
        Instruction::new("INC D", |cpu, bus| {
            cpu.inc(bus, D);
            return &[4];
        }), // 0x14
        Instruction::new("DEC D", |cpu, bus| {
            cpu.dec(bus, D);
            return &[4];
        }), // 0x15
        Instruction::new("LD D,n8", |cpu, bus| {
            cpu.ld(bus, D, Immediate::n8);
            return &[8];
        }), // 0x16
        Instruction::new("RLA", |cpu, bus| {
            // cpu.rla();
            cpu.rl(bus, A);
            return &[4];
        }), // 0x17
        Instruction::new("JR e8", |cpu, bus| {
            cpu.jr(JumpCondition::Always, cpu.next_i8(bus));
            return &[12];
        }), // 0x18
        Instruction::new("ADD HL,DE", |cpu, bus| {
            cpu.add_hl(bus, DE);
            return &[8];
        }), // 0x19
        Instruction::new("LD A,[DE]", |cpu, bus| {
            cpu.ld(bus, A, AddrPtr::DE);
            return &[8];
        }), // 0x1A
        Instruction::new("DEC DE", |cpu, bus| {
            cpu.dec16(bus, DE);
            return &[8];
        }), // 0x1B
        Instruction::new("INC E", |cpu, bus| {
            cpu.inc(bus, E);
            return &[4];
        }), // 0x1C
        Instruction::new("DEC E", |cpu, bus| {
            cpu.dec(bus, E);
            return &[4];
        }), // 0x1D
        Instruction::new("LD E,n8", |cpu, bus| {
            cpu.ld(bus, E, Immediate::n8);
            return &[8];
        }), // 0x1E
        Instruction::new("RRA", |cpu, bus| {
            // cpu.rra();
            cpu.rr(bus, A);
            return &[4];
        }), // 0x1F
        Instruction::new("JR NZ,e8", |cpu, bus| {
            cpu.jr(JumpCondition::NotZero, cpu.next_i8(bus));
            return &[12, 8];
        }), // 0x20
        Instruction::new("LD HL,n16", |cpu, bus| {
            cpu.ld(bus, HL, Immediate::n16);
            return &[12];
        }), // 0x21
        Instruction::new("LD [HL+],A", |cpu, bus| {
            cpu.ld(bus, AddrPtr::HLI, A);
            return &[8];
        }), // 0x22
        Instruction::new("INC HL", |cpu, bus| {
            cpu.inc16(bus, HL);
            return &[8];
        }), // 0x23
        Instruction::new("INC H", |cpu, bus| {
            cpu.inc(bus, H);
            return &[4];
        }), // 0x24
        Instruction::new("DEC H", |cpu, bus| {
            cpu.dec(bus, H);
            return &[4];
        }), // 0x25
        Instruction::new("LD H,n8", |cpu, bus| {
            cpu.ld(bus, H, Immediate::n8);
            return &[8];
        }), // 0x26
        Instruction::new("DAA", |cpu, bus| {
            cpu.daa();
            return &[4];
        }), // 0x27
        Instruction::new("JR Z,e8", |cpu, bus| {
            cpu.jr(JumpCondition::Zero, cpu.next_i8(bus));
            return &[12, 8];
        }), // 0x28
        Instruction::new("ADD HL,HL", |cpu, bus| {
            cpu.add_hl(bus, HL);
            return &[8];
        }), // 0x29
        Instruction::new("LD A,[HL+]", |cpu, bus| {
            cpu.ld(bus, A, AddrPtr::HLI);
            return &[8];
        }), // 0x2A
        Instruction::new("DEC HL", |cpu, bus| {
            cpu.dec16(bus, HL);
            return &[8];
        }), // 0x2B
        Instruction::new("INC L", |cpu, bus| {
            cpu.inc(bus, L);
            return &[4];
        }), // 0x2C
        Instruction::new("DEC L", |cpu, bus| {
            cpu.dec(bus, L);
            return &[4];
        }), // 0x2D
        Instruction::new("LD L,n8", |cpu, bus| {
            cpu.ld(bus, L, Immediate::n8);
            return &[8];
        }), // 0x2E
        Instruction::new("CPL", |cpu, bus| {
            cpu.cpl();
            return &[4];
        }), // 0x2F
        Instruction::new("JR NC,e8", |cpu, bus| {
            cpu.jr(JumpCondition::NotCarry, cpu.next_i8(bus));
            return &[12, 8];
        }), // 0x30
        Instruction::new("LD SP,n16", |cpu, bus| {
            cpu.ld(bus, SP, Immediate::n16);
            return &[12];
        }), // 0x31
        Instruction::new("LD [HL-],A", |cpu, bus| {
            cpu.ld(bus, AddrPtr::HLD, A);
            return &[8];
        }), // 0x32
        Instruction::new("INC SP", |cpu, bus| {
            cpu.inc16(bus, SP);
            return &[8];
        }), // 0x33
        Instruction::new("INC [HL]", |cpu, bus| {
            cpu.inc(bus, AddrPtr::HL);
            return &[12];
        }), // 0x34
        Instruction::new("DEC [HL]", |cpu, bus| {
            cpu.dec(bus, AddrPtr::HL);
            return &[12];
        }), // 0x35
        Instruction::new("LD [HL],n8", |cpu, bus| {
            cpu.ld(bus, AddrPtr::HL, cpu.next_byte(bus));
            return &[12];
        }), // 0x36
        Instruction::new("SCF", |cpu, bus| {
            cpu.scf();
            return &[4];
        }), // 0x37
        Instruction::new("JR C,e8", |cpu, bus| {
            cpu.jr(JumpCondition::Carry, cpu.next_i8(bus));
            return &[12, 8];
        }), // 0x38
        Instruction::new("ADD HL,SP", |cpu, bus| {
            cpu.add_hl(bus, SP);
            return &[8];
        }), // 0x39
        Instruction::new("LD A,[HL-]", |cpu, bus| {
            cpu.ld(bus, A, AddrPtr::HLD);
            return &[8];
        }), // 0x3A
        Instruction::new("DEC SP", |cpu, bus| {
            cpu.dec16(bus, SP);
            return &[8];
        }), // 0x3B
        Instruction::new("INC A", |cpu, bus| {
            cpu.inc(bus, A);
            return &[4];
        }), // 0x3C
        Instruction::new("DEC A", |cpu, bus| {
            cpu.dec(bus, A);
            return &[4];
        }), // 0x3D
        Instruction::new("LD A,n8", |cpu, bus| {
            cpu.ld(bus, A, cpu.next_byte(bus));
            return &[8];
        }), // 0x3E
        Instruction::new("CCF", |cpu, bus| {
            cpu.ccf();
            return &[4];
        }), // 0x3F
        Instruction::new("LD B,B", |cpu, bus| {
            cpu.ld(bus, B, B);
            return &[4];
        }), // 0x40
        Instruction::new("LD B,C", |cpu, bus| {
            cpu.ld(bus, B, C);
            return &[4];
        }), // 0x41
        Instruction::new("LD B,D", |cpu, bus| {
            cpu.ld(bus, B, D);
            return &[4];
        }), // 0x42
        Instruction::new("LD B,E", |cpu, bus| {
            cpu.ld(bus, B, E);
            return &[4];
        }), // 0x43
        Instruction::new("LD B,H", |cpu, bus| {
            cpu.ld(bus, B, H);
            return &[4];
        }), // 0x44
        Instruction::new("LD B,L", |cpu, bus| {
            cpu.ld(bus, B, L);
            return &[4];
        }), // 0x45
        Instruction::new("LD B,[HL]", |cpu, bus| {
            cpu.ld(bus, B, AddrPtr::HL);
            return &[8];
        }), // 0x46
        Instruction::new("LD B,A", |cpu, bus| {
            cpu.ld(bus, B, A);
            return &[4];
        }), // 0x47
        Instruction::new("LD C,B", |cpu, bus| {
            cpu.ld(bus, C, B);
            return &[4];
        }), // 0x48
        Instruction::new("LD C,C", |cpu, bus| {
            cpu.ld(bus, C, C);
            return &[4];
        }), // 0x49
        Instruction::new("LD C,D", |cpu, bus| {
            cpu.ld(bus, C, D);
            return &[4];
        }), // 0x4A
        Instruction::new("LD C,E", |cpu, bus| {
            cpu.ld(bus, C, E);
            return &[4];
        }), // 0x4B
        Instruction::new("LD C,H", |cpu, bus| {
            cpu.ld(bus, C, H);
            return &[4];
        }), // 0x4C
        Instruction::new("LD C,L", |cpu, bus| {
            cpu.ld(bus, C, L);
            return &[4];
        }), // 0x4D
        Instruction::new("LD C,[HL]", |cpu, bus| {
            cpu.ld(bus, C, AddrPtr::HL);
            return &[8];
        }), // 0x4E
        Instruction::new("LD C,A", |cpu, bus| {
            cpu.ld(bus, C, A);
            return &[4];
        }), // 0x4F
        Instruction::new("LD D,B", |cpu, bus| {
            cpu.ld(bus, D, B);
            return &[4];
        }), // 0x50
        Instruction::new("LD D,C", |cpu, bus| {
            cpu.ld(bus, D, C);
            return &[4];
        }), // 0x51
        Instruction::new("LD D,D", |cpu, bus| {
            cpu.ld(bus, D, D);
            return &[4];
        }), // 0x52
        Instruction::new("LD D,E", |cpu, bus| {
            cpu.ld(bus, D, E);
            return &[4];
        }), // 0x53
        Instruction::new("LD D,H", |cpu, bus| {
            cpu.ld(bus, D, H);
            return &[4];
        }), // 0x54
        Instruction::new("LD D,L", |cpu, bus| {
            cpu.ld(bus, D, L);
            return &[4];
        }), // 0x55
        Instruction::new("LD D,[HL]", |cpu, bus| {
            cpu.ld(bus, D, AddrPtr::HL);
            return &[8];
        }), // 0x56
        Instruction::new("LD D,A", |cpu, bus| {
            cpu.ld(bus, D, A);
            return &[4];
        }), // 0x57
        Instruction::new("LD E,B", |cpu, bus| {
            cpu.ld(bus, E, B);
            return &[4];
        }), // 0x58
        Instruction::new("LD E,C", |cpu, bus| {
            cpu.ld(bus, E, C);
            return &[4];
        }), // 0x59
        Instruction::new("LD E,D", |cpu, bus| {
            cpu.ld(bus, E, D);
            return &[4];
        }), // 0x5A
        Instruction::new("LD E,E", |cpu, bus| {
            cpu.ld(bus, E, E);
            return &[4];
        }), // 0x5B
        Instruction::new("LD E,H", |cpu, bus| {
            cpu.ld(bus, E, H);
            return &[4];
        }), // 0x5C
        Instruction::new("LD E,L", |cpu, bus| {
            cpu.ld(bus, E, L);
            return &[4];
        }), // 0x5D
        Instruction::new("LD E,[HL]", |cpu, bus| {
            cpu.ld(bus, E, AddrPtr::HL);
            return &[8];
        }), // 0x5E
        Instruction::new("LD E,A", |cpu, bus| {
            cpu.ld(bus, E, A);
            return &[4];
        }), // 0x5F
        Instruction::new("LD H,B", |cpu, bus| {
            cpu.ld(bus, H, B);
            return &[4];
        }), // 0x60
        Instruction::new("LD H,C", |cpu, bus| {
            cpu.ld(bus, H, C);
            return &[4];
        }), // 0x61
        Instruction::new("LD H,D", |cpu, bus| {
            cpu.ld(bus, H, D);
            return &[4];
        }), // 0x62
        Instruction::new("LD H,E", |cpu, bus| {
            cpu.ld(bus, H, E);
            return &[4];
        }), // 0x63
        Instruction::new("LD H,H", |cpu, bus| {
            cpu.ld(bus, H, H);
            return &[4];
        }), // 0x64
        Instruction::new("LD H,L", |cpu, bus| {
            cpu.ld(bus, H, L);
            return &[4];
        }), // 0x65
        Instruction::new("LD H,[HL]", |cpu, bus| {
            cpu.ld(bus, H, AddrPtr::HL);
            return &[8];
        }), // 0x66
        Instruction::new("LD H,A", |cpu, bus| {
            cpu.ld(bus, H, A);
            return &[4];
        }), // 0x67
        Instruction::new("LD L,B", |cpu, bus| {
            cpu.ld(bus, L, B);
            return &[4];
        }), // 0x68
        Instruction::new("LD L,C", |cpu, bus| {
            cpu.ld(bus, L, C);
            return &[4];
        }), // 0x69
        Instruction::new("LD L,D", |cpu, bus| {
            cpu.ld(bus, L, D);
            return &[4];
        }), // 0x6A
        Instruction::new("LD L,E", |cpu, bus| {
            cpu.ld(bus, L, E);
            return &[4];
        }), // 0x6B
        Instruction::new("LD L,H", |cpu, bus| {
            cpu.ld(bus, L, H);
            return &[4];
        }), // 0x6C
        Instruction::new("LD L,L", |cpu, bus| {
            cpu.ld(bus, L, L);
            return &[4];
        }), // 0x6D
        Instruction::new("LD L,[HL]", |cpu, bus| {
            cpu.ld(bus, L, AddrPtr::HL);
            return &[8];
        }), // 0x6E
        Instruction::new("LD L,A", |cpu, bus| {
            cpu.ld(bus, L, A);
            return &[4];
        }), // 0x6F
        Instruction::new("LD [HL],B", |cpu, bus| {
            cpu.ld(bus, AddrPtr::HL, B);
            return &[8];
        }), // 0x70
        Instruction::new("LD [HL],C", |cpu, bus| {
            cpu.ld(bus, AddrPtr::HL, C);
            return &[8];
        }), // 0x71
        Instruction::new("LD [HL],D", |cpu, bus| {
            cpu.ld(bus, AddrPtr::HL, D);
            return &[8];
        }), // 0x72
        Instruction::new("LD [HL],E", |cpu, bus| {
            cpu.ld(bus, AddrPtr::HL, E);
            return &[8];
        }), // 0x73
        Instruction::new("LD [HL],H", |cpu, bus| {
            cpu.ld(bus, AddrPtr::HL, H);
            return &[8];
        }), // 0x74
        Instruction::new("LD [HL],L", |cpu, bus| {
            cpu.ld(bus, AddrPtr::HL, L);
            return &[8];
        }), // 0x75
        Instruction::new("HALT", |cpu, bus| {
            cpu.halt();
            return &[4];
        }), // 0x76
        Instruction::new("LD [HL],A", |cpu, bus| {
            cpu.ld(bus, AddrPtr::HL, A);
            return &[8];
        }), // 0x77
        Instruction::new("LD A,B", |cpu, bus| {
            cpu.ld(bus, A, B);
            return &[4];
        }), // 0x78
        Instruction::new("LD A,C", |cpu, bus| {
            cpu.ld(bus, A, C);
            return &[4];
        }), // 0x79
        Instruction::new("LD A,D", |cpu, bus| {
            cpu.ld(bus, A, D);
            return &[4];
        }), // 0x7A
        Instruction::new("LD A,E", |cpu, bus| {
            cpu.ld(bus, A, E);
            return &[4];
        }), // 0x7B
        Instruction::new("LD A,H", |cpu, bus| {
            cpu.ld(bus, A, H);
            return &[4];
        }), // 0x7C
        Instruction::new("LD A,L", |cpu, bus| {
            cpu.ld(bus, A, L);
            return &[4];
        }), // 0x7D
        Instruction::new("LD A,[HL]", |cpu, bus| {
            cpu.ld(bus, A, AddrPtr::HL);
            return &[8];
        }), // 0x7E
        Instruction::new("LD A,A", |cpu, bus| {
            cpu.ld(bus, A, A);
            return &[4];
        }), // 0x7F
        Instruction::new("ADD A,B", |cpu, bus| {
            cpu.add(bus, B);
            return &[4];
        }), // 0x80
        Instruction::new("ADD A,C", |cpu, bus| {
            cpu.add(bus, C);
            return &[4];
        }), // 0x81
        Instruction::new("ADD A,D", |cpu, bus| {
            cpu.add(bus, D);
            return &[4];
        }), // 0x82
        Instruction::new("ADD A,E", |cpu, bus| {
            cpu.add(bus, E);
            return &[4];
        }), // 0x83
        Instruction::new("ADD A,H", |cpu, bus| {
            cpu.add(bus, H);
            return &[4];
        }), // 0x84
        Instruction::new("ADD A,L", |cpu, bus| {
            cpu.add(bus, L);
            return &[4];
        }), // 0x85
        Instruction::new("ADD A,[HL]", |cpu, bus| {
            cpu.add(bus, AddrPtr::HL);
            return &[8];
        }), // 0x86
        Instruction::new("ADD A,A", |cpu, bus| {
            cpu.add(bus, A);
            return &[4];
        }), // 0x87
        Instruction::new("ADC A,B", |cpu, bus| {
            cpu.adc(bus, B);
            return &[4];
        }), // 0x88
        Instruction::new("ADC A,C", |cpu, bus| {
            cpu.adc(bus, C);
            return &[4];
        }), // 0x89
        Instruction::new("ADC A,D", |cpu, bus| {
            cpu.adc(bus, D);
            return &[4];
        }), // 0x8A
        Instruction::new("ADC A,E", |cpu, bus| {
            cpu.adc(bus, E);
            return &[4];
        }), // 0x8B
        Instruction::new("ADC A,H", |cpu, bus| {
            cpu.adc(bus, H);
            return &[4];
        }), // 0x8C
        Instruction::new("ADC A,L", |cpu, bus| {
            cpu.adc(bus, L);
            return &[4];
        }), // 0x8D
        Instruction::new("ADC A,[HL]", |cpu, bus| {
            cpu.adc(bus, AddrPtr::HL);
            return &[8];
        }), // 0x8E
        Instruction::new("ADC A,A", |cpu, bus| {
            cpu.adc(bus, A);
            return &[4];
        }), // 0x8F
        Instruction::new("SUB A,B", |cpu, bus| {
            cpu.sub(bus, B);
            return &[4];
        }), // 0x90
        Instruction::new("SUB A,C", |cpu, bus| {
            cpu.sub(bus, C);
            return &[4];
        }), // 0x91
        Instruction::new("SUB A,D", |cpu, bus| {
            cpu.sub(bus, D);
            return &[4];
        }), // 0x92
        Instruction::new("SUB A,E", |cpu, bus| {
            cpu.sub(bus, E);
            return &[4];
        }), // 0x93
        Instruction::new("SUB A,H", |cpu, bus| {
            cpu.sub(bus, H);
            return &[4];
        }), // 0x94
        Instruction::new("SUB A,L", |cpu, bus| {
            cpu.sub(bus, L);
            return &[4];
        }), // 0x95
        Instruction::new("SUB A,[HL]", |cpu, bus| {
            cpu.sub(bus, AddrPtr::HL);
            return &[8];
        }), // 0x96
        Instruction::new("SUB A,A", |cpu, bus| {
            cpu.sub(bus, A);
            return &[4];
        }), // 0x97
        Instruction::new("SBC A,B", |cpu, bus| {
            cpu.sbc(bus, B);
            return &[4];
        }), // 0x98
        Instruction::new("SBC A,C", |cpu, bus| {
            cpu.sbc(bus, C);
            return &[4];
        }), // 0x99
        Instruction::new("SBC A,D", |cpu, bus| {
            cpu.sbc(bus, D);
            return &[4];
        }), // 0x9A
        Instruction::new("SBC A,E", |cpu, bus| {
            cpu.sbc(bus, E);
            return &[4];
        }), // 0x9B
        Instruction::new("SBC A,H", |cpu, bus| {
            cpu.sbc(bus, H);
            return &[4];
        }), // 0x9C
        Instruction::new("SBC A,L", |cpu, bus| {
            cpu.sbc(bus, L);
            return &[4];
        }), // 0x9D
        Instruction::new("SBC A,[HL]", |cpu, bus| {
            cpu.sbc(bus, AddrPtr::HL);
            return &[8];
        }), // 0x9E
        Instruction::new("SBC A,A", |cpu, bus| {
            cpu.sbc(bus, A);
            return &[4];
        }), // 0x9F
        Instruction::new("AND A,B", |cpu, bus| {
            cpu.and(A, B);
            return &[4];
        }), // 0xA0
        Instruction::new("AND A,C", |cpu, bus| {
            cpu.and(A, C);
            return &[4];
        }), // 0xA1
        Instruction::new("AND A,D", |cpu, bus| {
            cpu.and(A, D);
            return &[4];
        }), // 0xA2
        Instruction::new("AND A,E", |cpu, bus| {
            cpu.and(A, E);
            return &[4];
        }), // 0xA3
        Instruction::new("AND A,H", |cpu, bus| {
            cpu.and(A, H);
            return &[4];
        }), // 0xA4
        Instruction::new("AND A,L", |cpu, bus| {
            cpu.and(A, L);
            return &[4];
        }), // 0xA5
        Instruction::new("AND A,[HL]", |cpu, bus| {
            cpu.and(A, [HL]);
            return &[8];
        }), // 0xA6
        Instruction::new("AND A,A", |cpu, bus| {
            cpu.and(A, A);
            return &[4];
        }), // 0xA7
        Instruction::new("XOR A,B", |cpu, bus| {
            cpu.xor(A, B);
            return &[4];
        }), // 0xA8
        Instruction::new("XOR A,C", |cpu, bus| {
            cpu.xor(A, C);
            return &[4];
        }), // 0xA9
        Instruction::new("XOR A,D", |cpu, bus| {
            cpu.xor(A, D);
            return &[4];
        }), // 0xAA
        Instruction::new("XOR A,E", |cpu, bus| {
            cpu.xor(A, E);
            return &[4];
        }), // 0xAB
        Instruction::new("XOR A,H", |cpu, bus| {
            cpu.xor(A, H);
            return &[4];
        }), // 0xAC
        Instruction::new("XOR A,L", |cpu, bus| {
            cpu.xor(A, L);
            return &[4];
        }), // 0xAD
        Instruction::new("XOR A,[HL]", |cpu, bus| {
            cpu.xor(A, [HL]);
            return &[8];
        }), // 0xAE
        Instruction::new("XOR A,A", |cpu, bus| {
            cpu.xor(A, A);
            return &[4];
        }), // 0xAF
        Instruction::new("OR A,B", |cpu, bus| {
            cpu.or(A, B);
            return &[4];
        }), // 0xB0
        Instruction::new("OR A,C", |cpu, bus| {
            cpu.or(A, C);
            return &[4];
        }), // 0xB1
        Instruction::new("OR A,D", |cpu, bus| {
            cpu.or(A, D);
            return &[4];
        }), // 0xB2
        Instruction::new("OR A,E", |cpu, bus| {
            cpu.or(A, E);
            return &[4];
        }), // 0xB3
        Instruction::new("OR A,H", |cpu, bus| {
            cpu.or(A, H);
            return &[4];
        }), // 0xB4
        Instruction::new("OR A,L", |cpu, bus| {
            cpu.or(A, L);
            return &[4];
        }), // 0xB5
        Instruction::new("OR A,[HL]", |cpu, bus| {
            cpu.or(A, [HL]);
            return &[8];
        }), // 0xB6
        Instruction::new("OR A,A", |cpu, bus| {
            cpu.or(A, A);
            return &[4];
        }), // 0xB7
        Instruction::new("CP A,B", |cpu, bus| {
            cpu.cp(A, B);
            return &[4];
        }), // 0xB8
        Instruction::new("CP A,C", |cpu, bus| {
            cpu.cp(A, C);
            return &[4];
        }), // 0xB9
        Instruction::new("CP A,D", |cpu, bus| {
            cpu.cp(A, D);
            return &[4];
        }), // 0xBA
        Instruction::new("CP A,E", |cpu, bus| {
            cpu.cp(A, E);
            return &[4];
        }), // 0xBB
        Instruction::new("CP A,H", |cpu, bus| {
            cpu.cp(A, H);
            return &[4];
        }), // 0xBC
        Instruction::new("CP A,L", |cpu, bus| {
            cpu.cp(A, L);
            return &[4];
        }), // 0xBD
        Instruction::new("CP A,[HL]", |cpu, bus| {
            cpu.cp(A, [HL]);
            return &[8];
        }), // 0xBE
        Instruction::new("CP A,A", |cpu, bus| {
            cpu.cp(A, A);
            return &[4];
        }), // 0xBF
        Instruction::new("RET NZ", |cpu, bus| {
            cpu.ret(NZ);
            return &[20, 8];
        }), // 0xC0
        Instruction::new("POP BC", |cpu, bus| {
            cpu.pop(BC);
            return &[12];
        }), // 0xC1
        Instruction::new("JP NZ,a16", |cpu, bus| {
            cpu.jp(JumpCondition::NotZero, cpu.next_u16(bus));
            return &[16, 12];
        }), // 0xC2
        Instruction::new("JP a16", |cpu, bus| {
            cpu.jp(JumpCondition::Always, cpu.next_u16(bus));
            return &[16];
        }), // 0xC3
        Instruction::new("CALL NZ,a16", |cpu, bus| {
            cpu.call(JumpCondition::NotZero, cpu.next_u16(bus));
            return &[24, 12];
        }), // 0xC4
        Instruction::new("PUSH BC", |cpu, bus| {
            cpu.push(BC);
            return &[16];
        }), // 0xC5
        Instruction::new("ADD A,n8", |cpu, bus| {
            cpu.add(A, n8);
            return &[8];
        }), // 0xC6
        Instruction::new("RST 0x00", |cpu, bus| {
            cpu.rst(0x00);
            return &[16];
        }), // 0xC7
        Instruction::new("RET Z", |cpu, bus| {
            cpu.ret(Z);
            return &[20, 8];
        }), // 0xC8
        Instruction::new("RET", |cpu, bus| {
            cpu.ret();
            return &[16];
        }), // 0xC9
        Instruction::new("JP Z,a16", |cpu, bus| {
            // cpu.jp(Z,a16);
            cpu.jp(JumpCondition::Zero, cpu.next_u16(bus));
            return &[16, 12];
        }), // 0xCA
        Instruction::new("PREFIX", |cpu, bus| {
            cpu.prefix();
            return &[4];
        }), // 0xCB
        Instruction::new("CALL Z,a16", |cpu, bus| {
            cpu.call(JumpCondition::Zero, cpu.next_u16(bus));
            return &[24, 12];
        }), // 0xCC
        Instruction::new("CALL a16", |cpu, bus| {
            cpu.call(JumpCondition::Always, cpu.next_u16(bus));
            return &[24];
        }), // 0xCD
        Instruction::new("ADC A,n8", |cpu, bus| {
            cpu.adc(A, n8);
            return &[8];
        }), // 0xCE
        Instruction::new("RST 0x08", |cpu, bus| {
            cpu.rst(0x08);
            return &[16];
        }), // 0xCF
        Instruction::new("RET NC", |cpu, bus| {
            cpu.ret(NC);
            return &[20, 8];
        }), // 0xD0
        Instruction::new("POP DE", |cpu, bus| {
            cpu.pop(DE);
            return &[12];
        }), // 0xD1
        Instruction::new("JP NC,a16", |cpu, bus| {
            cpu.jp(JumpCondition::NotCarry, cpu.next_u16(bus));
            return &[16, 12];
        }), // 0xD2
        Instruction::new("ILLEGAL_D3", |cpu, bus| {
            unreachable!("0xD3 is illegal");
            // cpu.illegal_d3();
            // return &[4];
        }), // 0xD3
        Instruction::new("CALL NC,a16", |cpu, bus| {
            cpu.call(NC, a16);
            return &[24, 12];
        }), // 0xD4
        Instruction::new("PUSH DE", |cpu, bus| {
            cpu.push(DE);
            return &[16];
        }), // 0xD5
        Instruction::new("SUB A,n8", |cpu, bus| {
            cpu.sub(A, n8);
            return &[8];
        }), // 0xD6
        Instruction::new("RST 0x10", |cpu, bus| {
            cpu.rst(0x10);
            return &[16];
        }), // 0xD7
        Instruction::new("RET C", |cpu, bus| {
            cpu.ret(C);
            return &[20, 8];
        }), // 0xD8
        Instruction::new("RETI", |cpu, bus| {
            cpu.reti();
            return &[16];
        }), // 0xD9
        Instruction::new("JP C,a16", |cpu, bus| {
            cpu.jp(JumpCondition::Carry, cpu.next_u16(bus));
            return &[16, 12];
        }), // 0xDA
        Instruction::new("ILLEGAL_DB", |cpu, bus| {
            unreachable!("0xDB is illegal");
            // cpu.illegal_db();
            // return &[4];
        }), // 0xDB
        Instruction::new("CALL C,a16", |cpu, bus| {
            cpu.call(C, a16);
            return &[24, 12];
        }), // 0xDC
        Instruction::new("ILLEGAL_DD", |cpu, bus| {
            unreachable!("0xDD is illegal");
            // cpu.illegal_dd();
            // return &[4];
        }), // 0xDD
        Instruction::new("SBC A,n8", |cpu, bus| {
            cpu.sbc(A, n8);
            return &[8];
        }), // 0xDE
        Instruction::new("RST 0x18", |cpu, bus| {
            cpu.rst(0x18);
            return &[16];
        }), // 0xDF
        Instruction::new("LDH [a8],A", |cpu, bus| {
            // cpu.ldh([a8],A);
            cpu.ld(
                bus,
                AddrPtr::ZeroPage {
                    lsb: cpu.next_byte(bus),
                },
                A,
            );
            return &[12];
        }), // 0xE0
        Instruction::new("POP HL", |cpu, bus| {
            cpu.pop(HL);
            return &[12];
        }), // 0xE1
        Instruction::new("LD [C],A", |cpu, bus| {
            cpu.ld(bus, AddrPtr::ZeroPageC, A);
            return &[8];
        }), // 0xE2
        Instruction::new("ILLEGAL_E3", |cpu, bus| {
            unreachable!("0xE3 is illegal");
            // cpu.illegal_e3();
            // return &[4];
        }), // 0xE3
        Instruction::new("ILLEGAL_E4", |cpu, bus| {
            unreachable!("0xE4 is illegal");
            // cpu.illegal_e4();
            // return &[4];
        }), // 0xE4
        Instruction::new("PUSH HL", |cpu, bus| {
            cpu.push(HL);
            return &[16];
        }), // 0xE5
        Instruction::new("AND A,n8", |cpu, bus| {
            cpu.and(A, n8);
            return &[8];
        }), // 0xE6
        Instruction::new("RST 0x20", |cpu, bus| {
            cpu.rst(0x20);
            return &[16];
        }), // 0xE7
        Instruction::new("ADD SP,e8", |cpu, bus| {
            cpu.add(SP, e8);
            return &[16];
        }), // 0xE8
        Instruction::new("JP HL", |cpu, bus| {
            cpu.jp(JumpCondition::Always, cpu.registers.hl());
            return &[4];
        }), // 0xE9
        Instruction::new("LD [a16],A", |cpu, bus| {
            cpu.ld(bus, AddrPtr::a16(cpu.next_u16(bus)), A);
            return &[16];
        }), // 0xEA
        Instruction::new("ILLEGAL_EB", |cpu, bus| {
            unreachable!("0xEB is illegal");
            // cpu.illegal_eb();
            // return &[4];
        }), // 0xEB
        Instruction::new("ILLEGAL_EC", |cpu, bus| {
            unreachable!("0xEC is illegal");
            // cpu.illegal_ec();
            // return &[4];
        }), // 0xEC
        Instruction::new("ILLEGAL_ED", |cpu, bus| {
            unreachable!("0xED is illegal");
            // cpu.illegal_ed();
            // return &[4];
        }), // 0xED
        Instruction::new("XOR A,n8", |cpu, bus| {
            cpu.xor(A, n8);
            return &[8];
        }), // 0xEE
        Instruction::new("RST 0x28", |cpu, bus| {
            cpu.rst(0x28);
            return &[16];
        }), // 0xEF
        Instruction::new("LDH A,[a8]", |cpu, bus| {
            // cpu.ldh(A,[a8]);
            cpu.ld(bus, A, AddrPtr::ZeroPage {
                lsb: cpu.next_byte(bus),
            });
            return &[12];
        }), // 0xF0
        Instruction::new("POP AF", |cpu, bus| {
            cpu.pop(AF);
            return &[12];
        }), // 0xF1
        Instruction::new("LD A,[C]", |cpu, bus| {
            cpu.ld(bus, A, AddrPtr::ZeroPageC);
            return &[8];
        }), // 0xF2
        Instruction::new("DI", |cpu, bus| {
            cpu.di();
            return &[4];
        }), // 0xF3
        Instruction::new("ILLEGAL_F4", |cpu, bus| {
            unreachable!("0xF4 is illegal");
            // cpu.illegal_f4();
            // return &[4];
        }), // 0xF4
        Instruction::new("PUSH AF", |cpu, bus| {
            cpu.push(AF);
            return &[16];
        }), // 0xF5
        Instruction::new("OR A,n8", |cpu, bus| {
            cpu.or(A, n8);
            return &[8];
        }), // 0xF6
        Instruction::new("RST 0x30", |cpu, bus| {
            cpu.rst(0x30);
            return &[16];
        }), // 0xF7
        Instruction::new("LD HL,SP+e8", |cpu, bus| {
            // cpu.ld(HL, SP + e8);
            cpu.ld_hl_spe8(bus);
            return &[12];
        }), // 0xF8
        Instruction::new("LD SP,HL", |cpu, bus| {
            cpu.ld(bus, SP, HL);
            return &[8];
        }), // 0xF9
        Instruction::new("LD A,[a16]", |cpu, bus| {
            cpu.ld(bus, A, AddrPtr::a16(cpu.next_u16(bus)));
            return &[16];
        }), // 0xFA
        Instruction::new("EI", |cpu, bus| {
            cpu.ei();
            return &[4];
        }), // 0xFB
        Instruction::new("ILLEGAL_FC", |cpu, bus| {
            unreachable!("0xFC is illegal");
            // cpu.illegal_fc();
            // return &[4];
        }), // 0xFC
        Instruction::new("ILLEGAL_FD", |cpu, bus| {
            unreachable!("0xFD is illegal");
            // cpu.illegal_fd();
            // return &[4];
        }), // 0xFD
        Instruction::new("CP A,n8", |cpu, bus| {
            cpu.cp(A, n8);
            return &[8];
        }), // 0xFE
        Instruction::new("RST 0x38", |cpu, bus| {
            cpu.rst(0x38);
            return &[16];
        }), // 0xFF
    ];
}

/// Condition wether the CPU should execute JP-like instructions
#[derive(Debug)]
pub enum JumpCondition {
    /// Jump if the Z(ero) flag is set to `0`
    NotZero,
    /// Jump if the Z(ero) flag is set to `1`
    Zero,
    /// Jump if the C(arry) flag is set to `0`
    NotCarry,
    /// Jump if the C(arry) flag is set to `1`
    Carry,
    /// Always jump
    Always,
}
