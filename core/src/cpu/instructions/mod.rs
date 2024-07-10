use super::memory::{Address, Reg16, Reg8};

mod decode;

mod execute;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    // [Arithmetic operations]
    /// Adds to the 8-bit `A` register, the carry flag and a value (based on ArithmeticTarget),
    /// and stores the result back into the `A` register.
    ///
    /// # Example
    /// ```text
    /// // Example: ADC B
    /// if opcode == 0x88:
    ///   result, carry_per_bit = A + flags.C + B
    ///   A = result
    ///   flags.Z = 1 if result == 0 else 0
    ///   flags.N = 0
    ///   flags.H = 1 if carry_per_bit[3] else 0
    ///   flags.C = 1 if carry_per_bit[7] else 0
    /// ```
    ADC(ArithmeticTarget),
    /// Adds to the 8-bit `A` register, a value (based on ArithmeticTarget),
    /// and stores the result back into the `A` register.
    ///
    /// # Example
    /// ```text
    /// // Example: ADD (HL)
    /// if opcode == 0x86:
    ///   data = read(HL)
    ///   result, carry_per_bit = A + data
    ///   A = result
    ///   flags.Z = 1 if result == 0 else 0
    ///   flags.N = 0
    ///   flags.H = 1 if carry_per_bit[3] else 0
    ///   flags.C = 1 if carry_per_bit[7] else 0
    /// ```
    ADD(ArithmeticTarget),
    /// Add to the combined `HL` register the data is inside of a combined 16-bit register
    ADD16(Reg16),
    /// Add to the combined 16-bit `HL` register the data is inside of the Stack Pointer
    ADD16_SP,
    /// Add a signed `u8` at `mem[PC++]` to the `Stack Pointer`
    ADD_SP_i8(i8),

    /// # `SUB`: Subtract
    /// Subtracts from the 8-bit `A` register, the 8-bit `value`, and stores the result back into the `A` register
    ///
    /// ## Opcode
    /// r:    `0b10010xxx`/various
    /// (HL): `0b10010110`/`0x96`
    /// n:    `0b11010110`/`0xD6 + n`
    ///
    /// ## Flags:
    /// Z N H C
    /// Z 1 H C
    ///
    /// ## Pseudocode: SUB B
    /// ```text
    /// result, carry_per_bit = A - B
    /// A = result
    /// flags.Z = 1 if result == 0 else 0
    /// flags.N = 1
    /// flags.H = 1 if carry_per_bit[3] else 0
    /// flags.C = 1 if carry_per_bit[7] else 0
    /// ```
    SUB(ArithmeticTarget),

    // [Bitwise operations]
    /// # **AND**
    /// TODO
    AND(ArithmeticTarget),
    /// # **OR**
    /// Performs a bitwise OR operation between the 8-bit `A` register and the 8-bit `source`, and stores the result back
    /// into the `A` register.
    ///
    /// ## Opcode
    /// 0b10110xxx/various
    ///
    /// ## Flags
    /// Z N H C
    /// Z 0 0 0
    ///
    /// ## Pseudocode: OR B
    /// ```text
    /// result = A | B
    /// A = result
    /// flags.Z = 1 if result == 0 else 0
    /// flags.N = 0
    /// flags.H = 0
    /// flags.C = 0
    /// ```
    OR { source: ArithmeticTarget },
    /// **XOR**: Bitwise XOR
    /// Performs a bitwise XOR operation between the 8-bit `A` register and the 8-bit `value`, and stores the result
    /// back into the `A` register.
    ///
    /// ## Opcode
    /// r:    `0b10101xxx`/various
    /// (HL): `0b10101110`/`0xAE`
    /// n:    `0b11101110`/`0xEE + n`
    ///
    /// ## Pseudocode: XOR B
    /// ```text
    /// result = A ^ B
    /// A = result
    /// flags.Z = 1 if result == 0 else 0
    /// flags.N = 0
    /// flags.H = 0
    /// flags.C = 0
    /// ```
    XOR(ArithmeticTarget),

    /// # **SBC**: Subtract with carry
    /// Subtracts from the 8-bit `A` register, the `carry flag` and the 8-bit `value`, and stores the result back into the `A`
    /// register.
    ///
    /// ## Opcode
    /// `0b10011xxx`/various r
    /// `0b10011110`/`0x9E` (HL)
    ///
    /// ## Flags:
    /// Z N H C
    /// Z 1 H C
    ///
    /// ## Pseudocode: SBC B
    /// ```text
    /// result, carry_per_bit = A - flags.C - B
    /// A = result
    /// flags.Z = 1 if result == 0 else 0
    /// flags.N = 1
    /// flags.H = 1 if carry_per_bit[3] else 0
    /// flags.C = 1 if carry_per_bit[7] else 0
    /// ```
    SBC(ArithmeticTarget),

    // [Call]
    /// Conditional function call to the absolute address specified by the 16-bit operand `address`.
    CALL {
        /// The memory address, read from the program at [PC++, PC++]
        address: u16,
        /// The condition wether the function should be executed (= jumped to).
        condition: JumpCondition,
    },
    /// # **RST n**: Restart / Call function (implied)
    /// Unconditional function call to the absolute fixed address defined by the opcode.
    ///
    /// ## Opcode
    /// `0b11xxx111`/various
    ///
    /// ## Pseudocode
    /// ```text
    /// n = rst_address(opcode)
    /// SP--
    /// write(SP--, msb(PC))
    /// write(SP, lsb(PC))
    /// PC = unsigned_16(lsb=n, msb=0x00)
    /// ```
    RST(u16),

    /// # Complement carry flag (CCF)
    /// Flips the `C`(arry) flag, and clears the `N` and `H` flags.
    ///
    /// ## Pseudocode
    /// ```text
    /// flags.N = 0
    /// flags.H = 0
    /// flags.C = !flags.C
    /// ```
    CCF,
    /// # Complement accumulator (CPL)
    /// Flips all the bits in the 8-bit A register, and sets the N and H flags.
    ///
    /// ## Pseudocode
    /// ```text
    /// A = !A
    /// flags.N = 1
    /// flags.H = 1
    /// ```
    CPL,

    /// # Compare (CP)
    ///
    /// ## Register
    /// Subtracts from the 8-bit `A` register, the 8-bit register r,
    /// and updates flags based on the result.
    ///
    /// This instruction is basically identical to SUB r, but does not update the A register.
    ///
    /// ## Indirect (HL)
    /// Subtracts from the 8-bit A register,
    /// data from the absolute address specified by the 16-bit register HL,
    /// and updates flags based on the result.
    ///
    /// This instruction is basically identical to SUB (HL),
    /// but does not update the A register.
    ///
    /// ## Immediate
    ///
    /// Subtracts from the 8-bit A register,
    /// the immediate data n, and updates flags based on the result.
    ///
    /// This instruction is basically identical to SUB n, but does not update the A register
    CP(ArithmeticTarget),

    /// Decimal adjust accumulator
    DAA,

    /// # Decrement
    ///
    /// ## `r`
    /// Decrement the 8-bit register `r`
    ///
    /// ## Indirect HL
    /// Decrements data at the absolute address specified by the 16-bit register `HL`.
    DEC(ArithmeticTarget),
    /// Decrement the immediate value inside of a combined 16-bit register
    DEC16(Reg16),
    /// Decrement the Stack Pointer
    DEC16_SP,

    /// # Decrement
    ///
    /// ## `r`
    /// Increment the 8-bit register `r`
    ///
    /// ## Indirect HL
    /// Increments data at the absolute address specified by the 16-bit register `HL`.
    INC(ArithmeticTarget),
    /// Increment the immediate value inside of a combined 16-bit register
    INC16(Reg16),
    /// Increment the Stack Pointer
    INC16_SP,

    /// Jump
    JP {
        target: JumpAddress,
        condition: JumpCondition,
    },

    /// Jump Relative
    JR {
        /// The address to jump to, relative to the `program_counter`
        target: i8,
        condition: JumpCondition,
    },

    /// # Load (LD & LDH)
    /// Load the value from the `source` into the `destination`.
    LD {
        destination: LoadTarget,
        source: LoadTarget,
    },
    LD16 {
        /// The immediate 16-bit value.
        /// `u16::from_nibles(lsb=mem[PC++], msb=mem[PC++])`
        value: u16,
        destination: Reg16,
    },
    /// Load into the address at the immediate address nn, the value of the Stack Pointer
    ///
    /// ```text
    /// nn = u16::from_nibles(lsb=mem[PC++], msb=mem[PC++])
    /// mem[nn] = PC
    /// ```
    LD_nn_SP(u16),
    /// Add the offset `i8` to the Stack Pointer and load that into memory at `HL`.
    /// `mem[HL] = SP + e`
    LD_HL_SP_e(i8),
    /// Load the value of the 16-bit combined register `HL` into the Stack Pointer
    LD_SP_HL,

    // [Stack instructions]
    /// # **POP r**: Pop from stack
    /// Pops to the 16-bit register `rr`, data from the stack memory.
    /// This instruction does not do calculations that affect flags, but POP AF completely replaces the F register
    /// value, so all flags are changed based on the 8-bit data that is read from memory
    ///
    /// ## Opcode
    /// `0b11xx0001`/various
    ///
    /// ## Pseudocode: POP BC
    /// ```text
    /// BC = unsigned_16(lsb=read(SP++), msb=read(SP++))
    /// ```
    POP(Reg16),
    /// # **PUSH rr**: Push to stack
    /// Push to the stack memory, data from the 16-bit register `rr`.
    ///
    /// ## Opcode
    /// `0b11xx0101`/various
    ///
    /// ## Pseudocode: PUSH BC
    /// ```text
    /// SP--
    /// write(SP--, msb(BC))
    /// write(SP, lsb(BC))
    /// ```
    ///
    /// ## Flags
    /// Z N H C
    /// \- - - -
    PUSH(Reg16),
    /// # **RET**: Return from function
    ///
    ///
    /// ## Pseudocode
    /// ```text
    /// if (condition):
    ///   PC = unsigned_16(lsb=read(SP++), msb=read(SP++)    
    /// ```
    /// ## Flags:
    /// Z N H C
    /// \- - - -
    RET(JumpCondition),
    /// # **RETI**: Return from interrupt handler
    /// Unconditional return from a function. Also enables interrupts by setting `IME=1`.
    ///
    /// ## Opcode
    /// `0b11011001`/`0xD9`
    ///
    /// ## Pseudocode
    /// ```text
    /// PC = unsigned_16(lsb=read(SP++), msb=read(SP++))
    /// IME = 1
    /// ```
    RETI,

    // [Rotate]
    /// # **RL r**: Rotate left through carry
    ///
    /// ## Flags:
    /// Z N H C
    /// 0 0 0 C
    ///
    /// ## Pseudocode
    /// ```text
    /// // TODO
    /// ```
    RL(ArithmeticTarget),
    /// # **RL r**: Rotate left
    ///
    /// ## Flags:
    /// Z N H C
    /// 0 0 0 C
    ///
    /// ## Pseudocode
    /// ```text
    /// // TODO
    /// ```
    RLC(ArithmeticTarget),
    /// # **RL r**: Rotate right through carry
    ///
    /// ## Flags:
    /// Z N H C
    /// 0 0 0 C
    ///
    /// ## Example
    /// ```text
    /// // TODO
    /// ```
    RR(ArithmeticTarget),
    /// # **RL r**: Rotate right
    ///
    /// ## Flags:
    /// Z N H C
    /// 0 0 0 C
    ///
    /// ## Pseudocode
    /// ```text
    /// // TODO
    /// ```
    RRC(ArithmeticTarget),

    // [Flags]
    /// # **SCF**: Set carry flag
    /// Sets the `carry` flag, and clears the `N` and `H` flags.
    ///
    /// ## Opcode
    /// `0b00110111`/`0x37`
    ///
    /// ## Flags:
    /// Z N H C
    /// - 0 0 1
    ///
    /// ## Pseudocode
    /// ```text
    /// flags.N = 0
    /// flags.H = 0
    /// flags.C = 1
    /// ```
    SCF,

    // [Misc]
    /// # **NOP**: No operation
    /// Doesn't do anything, just takes 1 mcycle (4 cycles)
    NOP,
    /// # **STOP**: Stop system and main clocks
    STOP,
    /// # **HALT**: Halt system clock
    HALT,

    /// # Disable interrupts
    /// Disables interrupt handling by setting `IME=0` and cancelling any scheduled effects of the EI instruction if any.
    ///
    /// ## Opcode
    /// `0b11110011`/`0xF3`
    DI,
    /// # Enable interrupts
    /// Schedules interrupt handling to be enabled after the next machine cycle.
    ///
    /// ## Opcode
    /// `0b11111011`/`0xFB`
    EI,

    /// Opcode prefix byte `0xCB` has been read while not in prefixed mode.
    ///
    /// Set instruction parsing to prefixed mode so that the next instruction will be parsed from
    /// the prefixed opcode table instead of the default table.
    PREFIX,
    /// The opcode (depending on prefix) does NOT have an instruction that relates to it.
    /// It is a "Undefined instruction".
    /// { bytes: 1, cycles: 4 }
    UNDEFINED,
    // /// Load values from memory
    // LD(LoadType),
    // /// Load halfword
    // LDH(LoadHalfwordType),

    // [Function calls]
    // Call a function
    // CALL(JumpTest),
    // Return from current function
    // RET(JumpTest),
    // [idk]
    // ADD(ArithmeticTarget),
    // INC(IncDecTarget),
    // /// Jump
    // JP(JumpTest),

    // ADDHL (add to HL) - just like ADD except that the target is added to the HL register
    // ADC (add with carry) - just like ADD except that the value of the carry flag is also added to the number
    // SUB (subtract) - subtract the value stored in a specific register with the value in the A register
    // SBC (subtract with carry) - just like ADD except that the value of the carry flag is also subtracted from the number
    // AND (logical and) - do a bitwise and on the value in a specific register and the value in the A register
    // OR (logical or) - do a bitwise or on the value in a specific register and the value in the A register
    // XOR (logical xor) - do a bitwise xor on the value in a specific register and the value in the A register
    // CP (compare) - just like SUB except the result of the subtraction is not stored back into A
    // INC (increment) - increment the value in a specific register by 1
    // DEC (decrement) - decrement the value in a specific register by 1
    // CCF (complement carry flag) - toggle the value of the carry flag
    // SCF (set carry flag) - set the carry flag to true
    // RRA (rotate right A register) - bit rotate A register right through the carry flag
    // RLA (rotate left A register) - bit rotate A register left through the carry flag
    // RRCA (rotate right A register) - bit rotate A register right (not through the carry flag)
    // RRLA (rotate left A register) - bit rotate A register left (not through the carry flag)
    // CPL (complement) - toggle every bit of the A register
    // BIT (bit test) - test to see if a specific bit of a specific register is set
    // RESET (bit reset) - set a specific bit of a specific register to 0
    // SET (bit set) - set a specific bit of a specific register to 1
    // SRL (shift right logical) - bit shift a specific register right by 1
    // RR (rotate right) - bit rotate a specific register right by 1 through the carry flag
    // RL (rotate left) - bit rotate a specific register left by 1 through the carry flag
    // RRC (rorate right) - bit rotate a specific register right by 1 (not through the carry flag)
    // RLC (rorate left) - bit rotate a specific register left by 1 (not through the carry flag)
    // SRA (shift right arithmetic) - arithmetic shift a specific register right by 1
    // SLA (shift left arithmetic) - arithmetic shift a specific register left by 1
    // SWAP (swap nibbles) - switch upper and lower nibble of a specific register
}

/// NOTE
///
/// Immediate is only available for the following instructions:
///  - ADD & ADC
#[derive(Debug)]
pub enum ArithmeticTarget {
    /// Add to the `A` register the data is inside of a 8-bit register
    ///
    /// Indication: `{REG_NAME} (immediate)`
    Reg8(Reg8),
    /// The `HL` register contains the **memory address** of the 8-bit value
    ///
    /// Indication: `HL`
    Indirect,
    /// The value is equal to `mem[PC++]`
    ///
    /// Indication: `n8 (immediate)`
    Immediate { value: u8 },
    // /// Add to the combined `HL` register the data is inside of a combined 16-bit register
    // Reg16(Reg16),
    // /// The stack pointer is incremented by a signed 8-bit value at `mem[PC++]`
    // SignedU8ToSP { value: i8 },
    // /// Add to the combined 16-bit `HL` register the data is inside of the Stack Pointer
    // StackPointer,
}

/// Condition wether the CPU should execute JP-like instructions
#[derive(Debug)]
pub enum JumpCondition {
    /// Jump if the Z(ero) flag is set to `0`
    NotZero,
    /// Jump if the Z(ero) flag is set to `!`
    Zero,
    /// Jump if the C(arry) flag is set to `0`
    NotCarry,
    /// Jump if the C(arry) flag is set to `1`
    Carry,
    /// Always jump
    Always,
}

#[derive(Debug)]
pub enum JumpAddress {
    /// The absolute address, stored in `u16::from_nibbles(lsb=mem[PC++], msb=mem[PC++])`
    Immediate(u16),
    /// The absolute address is stored in the combined 16-bit `HL` register
    HL,
}

/// 8-bit load target
#[derive(Debug)]
pub enum LoadTarget {
    /// Direct from/into 8-bit register
    Reg8(Reg8),
    /// Direct from/into memory address
    Address(Address),
}
