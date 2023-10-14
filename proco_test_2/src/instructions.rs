#![allow(non_snake_case)]
#![allow(dead_code)]

//! This module defines a set of assembly instructions, each encapsulated as a separate function.
//! <br>
//! <br>
//! __________
//! Instructions will be documented as follow :
//! 
//! A brief description of the instruction's function
//! 
//! **Syntax**<br>
//! The syntax of this instruction
//! 
//! **Allowed Addressing Modes**<br>
//! A list of the allowed addressing modes for both source and destination.
//! 
//! **Operation Performed**<br>
//! A precise description of the operation performed. "S" represents the source, and "D" represents the destination.
//!
//! **Flags**<br>
//! The affected flags and how they are affected.
pub mod instructions {

    // May gonna be removed
    enum Operand {
        Register(u8),               //   Rn
        IndirectRegister(u8),       //  (Rn)
        IndirectAutoIncrement(u8),  //  (Rn)+
        IndirectAutoDecrement(u8),  // -(Rn)
        Immediate(u16),             //   #x
        Memory(u16),                //   @x
    }

    /// Copy the contents of a register or memory area into a register or memory area
    /// 
    /// The `MOVE()` function in Rust corresponds to the MOVE instruction in a processor architecture.
    /// This instruction is used to copy data from a source operand to a destination operand. There are
    /// three variations of this instruction: MOVE, MOVE.l, and MOVE.h.
    ///
    /// - `MOVE` copies 16 bits of data from the source to the destination.
    /// - `MOVE.l` copies the 8 least significant bits (LSB) of the source to the destination.
    /// - `MOVE.h` copies the 8 most significant bits (MSB) of the source to the destination.
    ///
    /// In the case of `MOVE.l` and `MOVE.h`, if the destination is a register, the 8 bits are copied into
    /// the 8 LSB of the destination register, while the 8 MSB remain unchanged. However, if the destination
    /// is a memory address, only the byte at the specified address will be modified.
    ///
    /// It is crucial to exercise caution when using `MOVE.l` or `MOVE.h` to avoid unintended consequences.
    ///
    /// Syntax:
    /// ```
    /// MOVE[.l/.h] S, D
    /// ```
    ///
    /// Allowed Addressing Modes:
    /// - Source (S): Rn, (Rn), (Rn)+, -(Rn), #x, @x
    /// - Destination (D): Rn, (Rn), (Rn)+, -(Rn), @x
    ///
    /// Note: There can only be one immediate value or address among the operands.
    ///
    /// Operation Performed:
    /// ```
    /// D = S
    /// ```
    ///
    /// Flags Affected:
    /// - Carry Flag (C) = 0
    /// - Negative Flag (N) = S15 (or S7 if MOVE.l is used)
    /// - Zero Flag (Z) = 1 if S equals 0, 0 otherwise. (Caution: For MOVE.l or MOVE.h, consider only the
    ///   8 LSB or MSB, respectively, when evaluating S.)
    pub fn MOVE() { }

    /// That function push a value onto the stack
    pub fn PUSH() { }

    /// That function pop a value from the stack
    pub fn POP()    { }

    /// Performs addition operation.
    pub fn ADD() { }

    /// Compares two values and sets flags based on the result.
    pub fn CMP() { }

    /// Subtracts one value from another.
    pub fn SUB() { }

    /// Performs a logical shift left operation.
    pub fn LSL() { }

    /// Performs a logical shift right operation.
    pub fn LSR() { }

    /// Performs a bitwise AND operation.
    pub fn AND() { }

    /// Performs a bitwise OR operation.
    pub fn OR() { }

    /// Performs a bitwise XOR operation.
    pub fn XOR() { }

    /// Performs a bitwise NOT operation.
    pub fn NOT() { }

    /// Branches to a target if two values are equal.
    pub fn BEQ() { }

    /// Branches to a target if two values are not equal.
    pub fn BNE() { }

    

    /// Branches to a target if one value is less than or equal to another.
    pub fn BLE() { }


    /// Branches to a target if the carry flag is clear.
    pub fn BCC() { }

    /// Branches to a target if one value is greater than another.
    pub fn BGT() { }

    /// Branches to a target if one value is greater than or equal to another.
    pub fn BGE() { }

    /// Unconditionally branches to a target.
    pub fn BRA() { }

    pub fn BCS() { }

    /// Branches to a target if one value is less than another.
    pub fn BLT() { }
    
    /// Jumps to a target if the carry flag is clear.
    pub fn JCC() { }

    /// Jumps to a target if one value is greater than another.
    pub fn JGT() { }

    /// Jumps to a target if the carry flag is set.
    pub fn JCS() { }

    /// Jumps to a target if one value is less than another.
    pub fn JLT() { }

    /// Jumps to a target if two values are equal.
    pub fn JEQ() { }

    /// Jumps to a target if two values are not equal.
    pub fn JNE() { }

    /// Jumps to a target if one value is less than or equal to another.
    pub fn JLE() { }

    /// Jumps to a target if one value is greater than or equal to another.
    pub fn JGE() { }

    /// Unconditionally jumps to a target.
    pub fn JMP() { }

    /// Branches to a subroutine.
    pub fn BSR() { }

    /// Jumps to a subroutine.
    pub fn JSR() { }

    /// Returns from a subroutine.
    pub fn RTS() { }

    /// Generates a software trap.
    pub fn TRAP() { }

    /// Returns from an exception.
    pub fn RTE() { }

}