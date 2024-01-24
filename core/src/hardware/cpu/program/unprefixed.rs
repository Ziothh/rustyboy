use super::Instruction;
use super::super::memory::{
    Reg8::*,
    Reg16::*,
};

fn t() {
}

impl Instruction {
    const UNPREFIXED: [Self; u8::MAX as usize + 1] = [
        Instruction::new("NOP", |cpu, bus| {
            cpu.nop();
            return &[4];
        }),
        Instruction::new("LD BC,n16", |cpu, bus| {
            cpu.ld(BC, cpu.next_u16(bus));
            return &[12];
        }),
        Instruction::new("LD [BC],A", |cpu, bus| {
            cpu.ld([BC],A);
            return &[8];
        }),
        Instruction::new("INC BC", |cpu, bus| {
            cpu.inc(BC);
            return &[8];
        }),
        Instruction::new("INC B", |cpu, bus| {
            cpu.inc(B);
            return &[4];
        }),
        Instruction::new("DEC B", |cpu, bus| {
            cpu.dec(B);
            return &[4];
        }),
        Instruction::new("LD B,n8", |cpu, bus| {
            cpu.ld(B,n8);
            return &[8];
        }),
        Instruction::new("RLCA", |cpu, bus| {
            cpu.rlca();
            return &[4];
        }),
        Instruction::new("LD [a16],SP", |cpu, bus| {
            cpu.ld([a16],SP);
            return &[20];
        }),
        Instruction::new("ADD HL,BC", |cpu, bus| {
            cpu.add(HL,BC);
            return &[8];
        }),
        Instruction::new("LD A,[BC]", |cpu, bus| {
            cpu.ld(A,[BC]);
            return &[8];
        }),
        Instruction::new("DEC BC", |cpu, bus| {
            cpu.dec(BC);
            return &[8];
        }),
        Instruction::new("INC C", |cpu, bus| {
            cpu.inc(C);
            return &[4];
        }),
        Instruction::new("DEC C", |cpu, bus| {
            cpu.dec(C);
            return &[4];
        }),
        Instruction::new("LD C,n8", |cpu, bus| {
            cpu.ld(C,n8);
            return &[8];
        }),
        Instruction::new("RRCA", |cpu, bus| {
            cpu.rrca();
            return &[4];
        }),
        Instruction::new("STOP n8", |cpu, bus| {
            cpu.stop(n8);
            return &[4];
        }),
        Instruction::new("LD DE,n16", |cpu, bus| {
            cpu.ld(DE,n16);
            return &[12];
        }),
        Instruction::new("LD [DE],A", |cpu, bus| {
            cpu.ld([DE],A);
            return &[8];
        }),
        Instruction::new("INC DE", |cpu, bus| {
            cpu.inc(DE);
            return &[8];
        }),
        Instruction::new("INC D", |cpu, bus| {
            cpu.inc(D);
            return &[4];
        }),
        Instruction::new("DEC D", |cpu, bus| {
            cpu.dec(D);
            return &[4];
        }),
        Instruction::new("LD D,n8", |cpu, bus| {
            cpu.ld(D,n8);
            return &[8];
        }),
        Instruction::new("RLA", |cpu, bus| {
            cpu.rla();
            return &[4];
        }),
        Instruction::new("JR e8", |cpu, bus| {
            cpu.jr(e8);
            return &[12];
        }),
        Instruction::new("ADD HL,DE", |cpu, bus| {
            cpu.add(HL,DE);
            return &[8];
        }),
        Instruction::new("LD A,[DE]", |cpu, bus| {
            cpu.ld(A,[DE]);
            return &[8];
        }),
        Instruction::new("DEC DE", |cpu, bus| {
            cpu.dec(DE);
            return &[8];
        }),
        Instruction::new("INC E", |cpu, bus| {
            cpu.inc(E);
            return &[4];
        }),
        Instruction::new("DEC E", |cpu, bus| {
            cpu.dec(E);
            return &[4];
        }),
        Instruction::new("LD E,n8", |cpu, bus| {
            cpu.ld(E,n8);
            return &[8];
        }),
        Instruction::new("RRA", |cpu, bus| {
            cpu.rra();
            return &[4];
        }),
        Instruction::new("JR NZ,e8", |cpu, bus| {
            cpu.jr(NZ,e8);
            return &[12, 8];
        }),
        Instruction::new("LD HL,n16", |cpu, bus| {
            cpu.ld(HL,n16);
            return &[12];
        }),
        Instruction::new("LD [HL+],A", |cpu, bus| {
            cpu.ld([HL+],A);
            return &[8];
        }),
        Instruction::new("INC HL", |cpu, bus| {
            cpu.inc(HL);
            return &[8];
        }),
        Instruction::new("INC H", |cpu, bus| {
            cpu.inc(H);
            return &[4];
        }),
        Instruction::new("DEC H", |cpu, bus| {
            cpu.dec(H);
            return &[4];
        }),
        Instruction::new("LD H,n8", |cpu, bus| {
            cpu.ld(H,n8);
            return &[8];
        }),
        Instruction::new("DAA", |cpu, bus| {
            cpu.daa();
            return &[4];
        }),
        Instruction::new("JR Z,e8", |cpu, bus| {
            cpu.jr(Z,e8);
            return &[12, 8];
        }),
        Instruction::new("ADD HL,HL", |cpu, bus| {
            cpu.add(HL,HL);
            return &[8];
        }),
        Instruction::new("LD A,[HL+]", |cpu, bus| {
            cpu.ld(A,[HL+]);
            return &[8];
        }),
        Instruction::new("DEC HL", |cpu, bus| {
            cpu.dec(HL);
            return &[8];
        }),
        Instruction::new("INC L", |cpu, bus| {
            cpu.inc(L);
            return &[4];
        }),
        Instruction::new("DEC L", |cpu, bus| {
            cpu.dec(L);
            return &[4];
        }),
        Instruction::new("LD L,n8", |cpu, bus| {
            cpu.ld(L,n8);
            return &[8];
        }),
        Instruction::new("CPL", |cpu, bus| {
            cpu.cpl();
            return &[4];
        }),
        Instruction::new("JR NC,e8", |cpu, bus| {
            cpu.jr(NC,e8);
            return &[12, 8];
        }),
        Instruction::new("LD SP,n16", |cpu, bus| {
            cpu.ld(SP,n16);
            return &[12];
        }),
        Instruction::new("LD [HL-],A", |cpu, bus| {
            cpu.ld([HL-],A);
            return &[8];
        }),
        Instruction::new("INC SP", |cpu, bus| {
            cpu.inc(SP);
            return &[8];
        }),
        Instruction::new("INC [HL]", |cpu, bus| {
            cpu.inc([HL]);
            return &[12];
        }),
        Instruction::new("DEC [HL]", |cpu, bus| {
            cpu.dec([HL]);
            return &[12];
        }),
        Instruction::new("LD [HL],n8", |cpu, bus| {
            cpu.ld([HL],n8);
            return &[12];
        }),
        Instruction::new("SCF", |cpu, bus| {
            cpu.scf();
            return &[4];
        }),
        Instruction::new("JR C,e8", |cpu, bus| {
            cpu.jr(C,e8);
            return &[12, 8];
        }),
        Instruction::new("ADD HL,SP", |cpu, bus| {
            cpu.add(HL,SP);
            return &[8];
        }),
        Instruction::new("LD A,[HL-]", |cpu, bus| {
            cpu.ld(A,[HL-]);
            return &[8];
        }),
        Instruction::new("DEC SP", |cpu, bus| {
            cpu.dec(SP);
            return &[8];
        }),
        Instruction::new("INC A", |cpu, bus| {
            cpu.inc(A);
            return &[4];
        }),
        Instruction::new("DEC A", |cpu, bus| {
            cpu.dec(A);
            return &[4];
        }),
        Instruction::new("LD A,n8", |cpu, bus| {
            cpu.ld(A,n8);
            return &[8];
        }),
        Instruction::new("CCF", |cpu, bus| {
            cpu.ccf();
            return &[4];
        }),
        Instruction::new("LD B,B", |cpu, bus| {
            cpu.ld(B,B);
            return &[4];
        }),
        Instruction::new("LD B,C", |cpu, bus| {
            cpu.ld(B,C);
            return &[4];
        }),
        Instruction::new("LD B,D", |cpu, bus| {
            cpu.ld(B,D);
            return &[4];
        }),
        Instruction::new("LD B,E", |cpu, bus| {
            cpu.ld(B,E);
            return &[4];
        }),
        Instruction::new("LD B,H", |cpu, bus| {
            cpu.ld(B,H);
            return &[4];
        }),
        Instruction::new("LD B,L", |cpu, bus| {
            cpu.ld(B,L);
            return &[4];
        }),
        Instruction::new("LD B,[HL]", |cpu, bus| {
            cpu.ld(B,[HL]);
            return &[8];
        }),
        Instruction::new("LD B,A", |cpu, bus| {
            cpu.ld(B,A);
            return &[4];
        }),
        Instruction::new("LD C,B", |cpu, bus| {
            cpu.ld(C,B);
            return &[4];
        }),
        Instruction::new("LD C,C", |cpu, bus| {
            cpu.ld(C,C);
            return &[4];
        }),
        Instruction::new("LD C,D", |cpu, bus| {
            cpu.ld(C,D);
            return &[4];
        }),
        Instruction::new("LD C,E", |cpu, bus| {
            cpu.ld(C,E);
            return &[4];
        }),
        Instruction::new("LD C,H", |cpu, bus| {
            cpu.ld(C,H);
            return &[4];
        }),
        Instruction::new("LD C,L", |cpu, bus| {
            cpu.ld(C,L);
            return &[4];
        }),
        Instruction::new("LD C,[HL]", |cpu, bus| {
            cpu.ld(C,[HL]);
            return &[8];
        }),
        Instruction::new("LD C,A", |cpu, bus| {
            cpu.ld(C,A);
            return &[4];
        }),
        Instruction::new("LD D,B", |cpu, bus| {
            cpu.ld(D,B);
            return &[4];
        }),
        Instruction::new("LD D,C", |cpu, bus| {
            cpu.ld(D,C);
            return &[4];
        }),
        Instruction::new("LD D,D", |cpu, bus| {
            cpu.ld(D,D);
            return &[4];
        }),
        Instruction::new("LD D,E", |cpu, bus| {
            cpu.ld(D,E);
            return &[4];
        }),
        Instruction::new("LD D,H", |cpu, bus| {
            cpu.ld(D,H);
            return &[4];
        }),
        Instruction::new("LD D,L", |cpu, bus| {
            cpu.ld(D,L);
            return &[4];
        }),
        Instruction::new("LD D,[HL]", |cpu, bus| {
            cpu.ld(D,[HL]);
            return &[8];
        }),
        Instruction::new("LD D,A", |cpu, bus| {
            cpu.ld(D,A);
            return &[4];
        }),
        Instruction::new("LD E,B", |cpu, bus| {
            cpu.ld(E,B);
            return &[4];
        }),
        Instruction::new("LD E,C", |cpu, bus| {
            cpu.ld(E,C);
            return &[4];
        }),
        Instruction::new("LD E,D", |cpu, bus| {
            cpu.ld(E,D);
            return &[4];
        }),
        Instruction::new("LD E,E", |cpu, bus| {
            cpu.ld(E,E);
            return &[4];
        }),
        Instruction::new("LD E,H", |cpu, bus| {
            cpu.ld(E,H);
            return &[4];
        }),
        Instruction::new("LD E,L", |cpu, bus| {
            cpu.ld(E,L);
            return &[4];
        }),
        Instruction::new("LD E,[HL]", |cpu, bus| {
            cpu.ld(E,[HL]);
            return &[8];
        }),
        Instruction::new("LD E,A", |cpu, bus| {
            cpu.ld(E,A);
            return &[4];
        }),
        Instruction::new("LD H,B", |cpu, bus| {
            cpu.ld(H,B);
            return &[4];
        }),
        Instruction::new("LD H,C", |cpu, bus| {
            cpu.ld(H,C);
            return &[4];
        }),
        Instruction::new("LD H,D", |cpu, bus| {
            cpu.ld(H,D);
            return &[4];
        }),
        Instruction::new("LD H,E", |cpu, bus| {
            cpu.ld(H,E);
            return &[4];
        }),
        Instruction::new("LD H,H", |cpu, bus| {
            cpu.ld(H,H);
            return &[4];
        }),
        Instruction::new("LD H,L", |cpu, bus| {
            cpu.ld(H,L);
            return &[4];
        }),
        Instruction::new("LD H,[HL]", |cpu, bus| {
            cpu.ld(H,[HL]);
            return &[8];
        }),
        Instruction::new("LD H,A", |cpu, bus| {
            cpu.ld(H,A);
            return &[4];
        }),
        Instruction::new("LD L,B", |cpu, bus| {
            cpu.ld(L,B);
            return &[4];
        }),
        Instruction::new("LD L,C", |cpu, bus| {
            cpu.ld(L,C);
            return &[4];
        }),
        Instruction::new("LD L,D", |cpu, bus| {
            cpu.ld(L,D);
            return &[4];
        }),
        Instruction::new("LD L,E", |cpu, bus| {
            cpu.ld(L,E);
            return &[4];
        }),
        Instruction::new("LD L,H", |cpu, bus| {
            cpu.ld(L,H);
            return &[4];
        }),
        Instruction::new("LD L,L", |cpu, bus| {
            cpu.ld(L,L);
            return &[4];
        }),
        Instruction::new("LD L,[HL]", |cpu, bus| {
            cpu.ld(L,[HL]);
            return &[8];
        }),
        Instruction::new("LD L,A", |cpu, bus| {
            cpu.ld(L,A);
            return &[4];
        }),
        Instruction::new("LD [HL],B", |cpu, bus| {
            cpu.ld([HL],B);
            return &[8];
        }),
        Instruction::new("LD [HL],C", |cpu, bus| {
            cpu.ld([HL],C);
            return &[8];
        }),
        Instruction::new("LD [HL],D", |cpu, bus| {
            cpu.ld([HL],D);
            return &[8];
        }),
        Instruction::new("LD [HL],E", |cpu, bus| {
            cpu.ld([HL],E);
            return &[8];
        }),
        Instruction::new("LD [HL],H", |cpu, bus| {
            cpu.ld([HL],H);
            return &[8];
        }),
        Instruction::new("LD [HL],L", |cpu, bus| {
            cpu.ld([HL],L);
            return &[8];
        }),
        Instruction::new("HALT", |cpu, bus| {
            cpu.halt();
            return &[4];
        }),
        Instruction::new("LD [HL],A", |cpu, bus| {
            cpu.ld([HL],A);
            return &[8];
        }),
        Instruction::new("LD A,B", |cpu, bus| {
            cpu.ld(A,B);
            return &[4];
        }),
        Instruction::new("LD A,C", |cpu, bus| {
            cpu.ld(A,C);
            return &[4];
        }),
        Instruction::new("LD A,D", |cpu, bus| {
            cpu.ld(A,D);
            return &[4];
        }),
        Instruction::new("LD A,E", |cpu, bus| {
            cpu.ld(A,E);
            return &[4];
        }),
        Instruction::new("LD A,H", |cpu, bus| {
            cpu.ld(A,H);
            return &[4];
        }),
        Instruction::new("LD A,L", |cpu, bus| {
            cpu.ld(A,L);
            return &[4];
        }),
        Instruction::new("LD A,[HL]", |cpu, bus| {
            cpu.ld(A,[HL]);
            return &[8];
        }),
        Instruction::new("LD A,A", |cpu, bus| {
            cpu.ld(A,A);
            return &[4];
        }),
        Instruction::new("ADD A,B", |cpu, bus| {
            cpu.add(A,B);
            return &[4];
        }),
        Instruction::new("ADD A,C", |cpu, bus| {
            cpu.add(A,C);
            return &[4];
        }),
        Instruction::new("ADD A,D", |cpu, bus| {
            cpu.add(A,D);
            return &[4];
        }),
        Instruction::new("ADD A,E", |cpu, bus| {
            cpu.add(A,E);
            return &[4];
        }),
        Instruction::new("ADD A,H", |cpu, bus| {
            cpu.add(A,H);
            return &[4];
        }),
        Instruction::new("ADD A,L", |cpu, bus| {
            cpu.add(A,L);
            return &[4];
        }),
        Instruction::new("ADD A,[HL]", |cpu, bus| {
            cpu.add(A,[HL]);
            return &[8];
        }),
        Instruction::new("ADD A,A", |cpu, bus| {
            cpu.add(A,A);
            return &[4];
        }),
        Instruction::new("ADC A,B", |cpu, bus| {
            cpu.adc(A,B);
            return &[4];
        }),
        Instruction::new("ADC A,C", |cpu, bus| {
            cpu.adc(A,C);
            return &[4];
        }),
        Instruction::new("ADC A,D", |cpu, bus| {
            cpu.adc(A,D);
            return &[4];
        }),
        Instruction::new("ADC A,E", |cpu, bus| {
            cpu.adc(A,E);
            return &[4];
        }),
        Instruction::new("ADC A,H", |cpu, bus| {
            cpu.adc(A,H);
            return &[4];
        }),
        Instruction::new("ADC A,L", |cpu, bus| {
            cpu.adc(A,L);
            return &[4];
        }),
        Instruction::new("ADC A,[HL]", |cpu, bus| {
            cpu.adc(A,[HL]);
            return &[8];
        }),
        Instruction::new("ADC A,A", |cpu, bus| {
            cpu.adc(A,A);
            return &[4];
        }),
        Instruction::new("SUB A,B", |cpu, bus| {
            cpu.sub(A,B);
            return &[4];
        }),
        Instruction::new("SUB A,C", |cpu, bus| {
            cpu.sub(A,C);
            return &[4];
        }),
        Instruction::new("SUB A,D", |cpu, bus| {
            cpu.sub(A,D);
            return &[4];
        }),
        Instruction::new("SUB A,E", |cpu, bus| {
            cpu.sub(A,E);
            return &[4];
        }),
        Instruction::new("SUB A,H", |cpu, bus| {
            cpu.sub(A,H);
            return &[4];
        }),
        Instruction::new("SUB A,L", |cpu, bus| {
            cpu.sub(A,L);
            return &[4];
        }),
        Instruction::new("SUB A,[HL]", |cpu, bus| {
            cpu.sub(A,[HL]);
            return &[8];
        }),
        Instruction::new("SUB A,A", |cpu, bus| {
            cpu.sub(A,A);
            return &[4];
        }),
        Instruction::new("SBC A,B", |cpu, bus| {
            cpu.sbc(A,B);
            return &[4];
        }),
        Instruction::new("SBC A,C", |cpu, bus| {
            cpu.sbc(A,C);
            return &[4];
        }),
        Instruction::new("SBC A,D", |cpu, bus| {
            cpu.sbc(A,D);
            return &[4];
        }),
        Instruction::new("SBC A,E", |cpu, bus| {
            cpu.sbc(A,E);
            return &[4];
        }),
        Instruction::new("SBC A,H", |cpu, bus| {
            cpu.sbc(A,H);
            return &[4];
        }),
        Instruction::new("SBC A,L", |cpu, bus| {
            cpu.sbc(A,L);
            return &[4];
        }),
        Instruction::new("SBC A,[HL]", |cpu, bus| {
            cpu.sbc(A,[HL]);
            return &[8];
        }),
        Instruction::new("SBC A,A", |cpu, bus| {
            cpu.sbc(A,A);
            return &[4];
        }),
        Instruction::new("AND A,B", |cpu, bus| {
            cpu.and(A,B);
            return &[4];
        }),
        Instruction::new("AND A,C", |cpu, bus| {
            cpu.and(A,C);
            return &[4];
        }),
        Instruction::new("AND A,D", |cpu, bus| {
            cpu.and(A,D);
            return &[4];
        }),
        Instruction::new("AND A,E", |cpu, bus| {
            cpu.and(A,E);
            return &[4];
        }),
        Instruction::new("AND A,H", |cpu, bus| {
            cpu.and(A,H);
            return &[4];
        }),
        Instruction::new("AND A,L", |cpu, bus| {
            cpu.and(A,L);
            return &[4];
        }),
        Instruction::new("AND A,[HL]", |cpu, bus| {
            cpu.and(A,[HL]);
            return &[8];
        }),
        Instruction::new("AND A,A", |cpu, bus| {
            cpu.and(A,A);
            return &[4];
        }),
        Instruction::new("XOR A,B", |cpu, bus| {
            cpu.xor(A,B);
            return &[4];
        }),
        Instruction::new("XOR A,C", |cpu, bus| {
            cpu.xor(A,C);
            return &[4];
        }),
        Instruction::new("XOR A,D", |cpu, bus| {
            cpu.xor(A,D);
            return &[4];
        }),
        Instruction::new("XOR A,E", |cpu, bus| {
            cpu.xor(A,E);
            return &[4];
        }),
        Instruction::new("XOR A,H", |cpu, bus| {
            cpu.xor(A,H);
            return &[4];
        }),
        Instruction::new("XOR A,L", |cpu, bus| {
            cpu.xor(A,L);
            return &[4];
        }),
        Instruction::new("XOR A,[HL]", |cpu, bus| {
            cpu.xor(A,[HL]);
            return &[8];
        }),
        Instruction::new("XOR A,A", |cpu, bus| {
            cpu.xor(A,A);
            return &[4];
        }),
        Instruction::new("OR A,B", |cpu, bus| {
            cpu.or(A,B);
            return &[4];
        }),
        Instruction::new("OR A,C", |cpu, bus| {
            cpu.or(A,C);
            return &[4];
        }),
        Instruction::new("OR A,D", |cpu, bus| {
            cpu.or(A,D);
            return &[4];
        }),
        Instruction::new("OR A,E", |cpu, bus| {
            cpu.or(A,E);
            return &[4];
        }),
        Instruction::new("OR A,H", |cpu, bus| {
            cpu.or(A,H);
            return &[4];
        }),
        Instruction::new("OR A,L", |cpu, bus| {
            cpu.or(A,L);
            return &[4];
        }),
        Instruction::new("OR A,[HL]", |cpu, bus| {
            cpu.or(A,[HL]);
            return &[8];
        }),
        Instruction::new("OR A,A", |cpu, bus| {
            cpu.or(A,A);
            return &[4];
        }),
        Instruction::new("CP A,B", |cpu, bus| {
            cpu.cp(A,B);
            return &[4];
        }),
        Instruction::new("CP A,C", |cpu, bus| {
            cpu.cp(A,C);
            return &[4];
        }),
        Instruction::new("CP A,D", |cpu, bus| {
            cpu.cp(A,D);
            return &[4];
        }),
        Instruction::new("CP A,E", |cpu, bus| {
            cpu.cp(A,E);
            return &[4];
        }),
        Instruction::new("CP A,H", |cpu, bus| {
            cpu.cp(A,H);
            return &[4];
        }),
        Instruction::new("CP A,L", |cpu, bus| {
            cpu.cp(A,L);
            return &[4];
        }),
        Instruction::new("CP A,[HL]", |cpu, bus| {
            cpu.cp(A,[HL]);
            return &[8];
        }),
        Instruction::new("CP A,A", |cpu, bus| {
            cpu.cp(A,A);
            return &[4];
        }),
        Instruction::new("RET NZ", |cpu, bus| {
            cpu.ret(NZ);
            return &[20, 8];
        }),
        Instruction::new("POP BC", |cpu, bus| {
            cpu.pop(BC);
            return &[12];
        }),
        Instruction::new("JP NZ,a16", |cpu, bus| {
            cpu.jp(NZ,a16);
            return &[16, 12];
        }),
        Instruction::new("JP a16", |cpu, bus| {
            cpu.jp(a16);
            return &[16];
        }),
        Instruction::new("CALL NZ,a16", |cpu, bus| {
            cpu.call(NZ,a16);
            return &[24, 12];
        }),
        Instruction::new("PUSH BC", |cpu, bus| {
            cpu.push(BC);
            return &[16];
        }),
        Instruction::new("ADD A,n8", |cpu, bus| {
            cpu.add(A,n8);
            return &[8];
        }),
        Instruction::new("RST 0x00", |cpu, bus| {
            cpu.rst(0x00);
            return &[16];
        }),
        Instruction::new("RET Z", |cpu, bus| {
            cpu.ret(Z);
            return &[20, 8];
        }),
        Instruction::new("RET", |cpu, bus| {
            cpu.ret();
            return &[16];
        }),
        Instruction::new("JP Z,a16", |cpu, bus| {
            cpu.jp(Z,a16);
            return &[16, 12];
        }),
        Instruction::new("PREFIX", |cpu, bus| {
            cpu.prefix();
            return &[4];
        }),
        Instruction::new("CALL Z,a16", |cpu, bus| {
            cpu.call(Z,a16);
            return &[24, 12];
        }),
        Instruction::new("CALL a16", |cpu, bus| {
            cpu.call(a16);
            return &[24];
        }),
        Instruction::new("ADC A,n8", |cpu, bus| {
            cpu.adc(A,n8);
            return &[8];
        }),
        Instruction::new("RST 0x08", |cpu, bus| {
            cpu.rst(0x08);
            return &[16];
        }),
        Instruction::new("RET NC", |cpu, bus| {
            cpu.ret(NC);
            return &[20, 8];
        }),
        Instruction::new("POP DE", |cpu, bus| {
            cpu.pop(DE);
            return &[12];
        }),
        Instruction::new("JP NC,a16", |cpu, bus| {
            cpu.jp(NC,a16);
            return &[16, 12];
        }),
        Instruction::new("ILLEGAL_D3", |cpu, bus| {
            cpu.illegal_d3();
            return &[4];
        }),
        Instruction::new("CALL NC,a16", |cpu, bus| {
            cpu.call(NC,a16);
            return &[24, 12];
        }),
        Instruction::new("PUSH DE", |cpu, bus| {
            cpu.push(DE);
            return &[16];
        }),
        Instruction::new("SUB A,n8", |cpu, bus| {
            cpu.sub(A,n8);
            return &[8];
        }),
        Instruction::new("RST 0x10", |cpu, bus| {
            cpu.rst(0x10);
            return &[16];
        }),
        Instruction::new("RET C", |cpu, bus| {
            cpu.ret(C);
            return &[20, 8];
        }),
        Instruction::new("RETI", |cpu, bus| {
            cpu.reti();
            return &[16];
        }),
        Instruction::new("JP C,a16", |cpu, bus| {
            cpu.jp(C,a16);
            return &[16, 12];
        }),
        Instruction::new("ILLEGAL_DB", |cpu, bus| {
            cpu.illegal_db();
            return &[4];
        }),
        Instruction::new("CALL C,a16", |cpu, bus| {
            cpu.call(C,a16);
            return &[24, 12];
        }),
        Instruction::new("ILLEGAL_DD", |cpu, bus| {
            cpu.illegal_dd();
            return &[4];
        }),
        Instruction::new("SBC A,n8", |cpu, bus| {
            cpu.sbc(A,n8);
            return &[8];
        }),
        Instruction::new("RST 0x18", |cpu, bus| {
            cpu.rst(0x18);
            return &[16];
        }),
        Instruction::new("LDH [a8],A", |cpu, bus| {
            cpu.ldh([a8],A);
            return &[12];
        }),
        Instruction::new("POP HL", |cpu, bus| {
            cpu.pop(HL);
            return &[12];
        }),
        Instruction::new("LD [C],A", |cpu, bus| {
            cpu.ld([C],A);
            return &[8];
        }),
        Instruction::new("ILLEGAL_E3", |cpu, bus| {
            cpu.illegal_e3();
            return &[4];
        }),
        Instruction::new("ILLEGAL_E4", |cpu, bus| {
            cpu.illegal_e4();
            return &[4];
        }),
        Instruction::new("PUSH HL", |cpu, bus| {
            cpu.push(HL);
            return &[16];
        }),
        Instruction::new("AND A,n8", |cpu, bus| {
            cpu.and(A,n8);
            return &[8];
        }),
        Instruction::new("RST 0x20", |cpu, bus| {
            cpu.rst(0x20);
            return &[16];
        }),
        Instruction::new("ADD SP,e8", |cpu, bus| {
            cpu.add(SP,e8);
            return &[16];
        }),
        Instruction::new("JP HL", |cpu, bus| {
            cpu.jp(HL);
            return &[4];
        }),
        Instruction::new("LD [a16],A", |cpu, bus| {
            cpu.ld([a16],A);
            return &[16];
        }),
        Instruction::new("ILLEGAL_EB", |cpu, bus| {
            cpu.illegal_eb();
            return &[4];
        }),
        Instruction::new("ILLEGAL_EC", |cpu, bus| {
            cpu.illegal_ec();
            return &[4];
        }),
        Instruction::new("ILLEGAL_ED", |cpu, bus| {
            cpu.illegal_ed();
            return &[4];
        }),
        Instruction::new("XOR A,n8", |cpu, bus| {
            cpu.xor(A,n8);
            return &[8];
        }),
        Instruction::new("RST 0x28", |cpu, bus| {
            cpu.rst(0x28);
            return &[16];
        }),
        Instruction::new("LDH A,[a8]", |cpu, bus| {
            cpu.ldh(A,[a8]);
            return &[12];
        }),
        Instruction::new("POP AF", |cpu, bus| {
            cpu.pop(AF);
            return &[12];
        }),
        Instruction::new("LD A,[C]", |cpu, bus| {
            cpu.ld(A,[C]);
            return &[8];
        }),
        Instruction::new("DI", |cpu, bus| {
            cpu.di();
            return &[4];
        }),
        Instruction::new("ILLEGAL_F4", |cpu, bus| {
            cpu.illegal_f4();
            return &[4];
        }),
        Instruction::new("PUSH AF", |cpu, bus| {
            cpu.push(AF);
            return &[16];
        }),
        Instruction::new("OR A,n8", |cpu, bus| {
            cpu.or(A,n8);
            return &[8];
        }),
        Instruction::new("RST 0x30", |cpu, bus| {
            cpu.rst(0x30);
            return &[16];
        }),
        Instruction::new("LD HL,SP+e8", |cpu, bus| {
            cpu.ld(HL,SP+e8);
            return &[12];
        }),
        Instruction::new("LD SP,HL", |cpu, bus| {
            cpu.ld(SP,HL);
            return &[8];
        }),
        Instruction::new("LD A,[a16]", |cpu, bus| {
            cpu.ld(A,[a16]);
            return &[16];
        }),
        Instruction::new("EI", |cpu, bus| {
            cpu.ei();
            return &[4];
        }),
        Instruction::new("ILLEGAL_FC", |cpu, bus| {
            cpu.illegal_fc();
            return &[4];
        }),
        Instruction::new("ILLEGAL_FD", |cpu, bus| {
            cpu.illegal_fd();
            return &[4];
        }),
        Instruction::new("CP A,n8", |cpu, bus| {
            cpu.cp(A,n8);
            return &[8];
        }),
        Instruction::new("RST 0x38", |cpu, bus| {
            cpu.rst(0x38);
            return &[16];
        })
    ];
}
