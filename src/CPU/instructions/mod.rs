use super::memory::{Reg8, Reg16};

mod decode;

mod execute;


#[allow(non_camel_case_types)]
pub enum Instruction {
    /// Adds to the 8-bit `A` register, the carry flag and a value (based on ArithmeticTarget), 
    /// and stores the result back into the `A` register.
    /// 
    /// # Example
    /// ```ignore
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
    /// ```ignore
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

    AND(ArithmeticTarget),

    /// Conditional function call to the absolute address specified by the 16-bit operand `address`.
    CALL {
        /// The memory address, read from the program at [PC++, PC++]
        address: u16,
        /// The condition wether the function should be executed (= jumped to).
        condition: JumpCondition,
    },

    /// # Complement carry flag (CCF)
    /// Flips the `C`(arry) flag, and clears the `N` and `H` flags.
    ///
    /// ## Pseudocode
    /// ```ignore
    /// flags.N = 0
    /// flags.H = 0
    /// flags.C = !flags.C
    /// ```
    CCF,
    /// # Complement accumulator (CPL)
    /// Flips all the bits in the 8-bit A register, and sets the N and H flags.
    ///
    /// ## Pseudocode
    /// ```ignore
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

    /// # Disable interrupts
    /// Disables interrupt handling by setting IME=0 and cancelling any scheduled effects of the EI instruction if any.
    DI,

    /// # Enable interrupts
    /// Schedules interrupt handling to be enabled after the next machine cycle.
    EI,

    /// Halt system clock
    HALT,

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
pub enum ArithmeticTarget {
    /// Add to the `A` register the data is inside of a 8-bit register
    Reg8(Reg8),
    /// The HL register contains the memory address of the 8-bit value
    Indirect,
    /// The value is equal to `mem[PC++]`
    Immediate { value: u8 },

    // /// Add to the combined `HL` register the data is inside of a combined 16-bit register
    // Reg16(Reg16),
    // /// The stack pointer is incremented by a signed 8-bit value at `mem[PC++]`
    // SignedU8ToSP { value: i8 },
    // /// Add to the combined 16-bit `HL` register the data is inside of the Stack Pointer
    // StackPointer,
}


/// Condition wether the CPU should execute JP-like instructions
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
