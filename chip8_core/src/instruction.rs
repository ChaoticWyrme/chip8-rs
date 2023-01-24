use log::debug;

type Register = u8;
type Address = u16;
type OpCode = u16;

// TODO: Change From implementations into TryFrom, since they have failure conditions

/// An enum representing the type of a math instruction
///
/// We use this since the math instructions generally have the same opcode pattern:
/// `0x8XYO`, where the 8 indicates it's a math instruction, the X and Y are arguments for the instruction,
/// and the O is the operation. The X is usually the destination register, and the Y is the source register.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MathOperation {
    // Dest = Source
    Assign,
    // Dest = Dest|Source
    BitwiseOr,
    // Dest = Dest&Source
    BitwiseAnd,
    // Dest = Dest^Source
    BitwiseXor,
    // Dest += Source
    Add,
    // Dest -= Source
    Subtract,
    // Stores least signifigant bit of Source in VF then
    // bitshifts Source to the right by one
    // Source >>= 1
    BitshiftRight,
    // Dest = Source - Dest
    Difference,
    // Stores most signifigant bit of Source in VF then
    // bitshifts Source to the left by one
    // Source <<= 1
    BitshiftLeft,
    UnknownOperation(OpCode),
}

impl From<OpCode> for MathOperation {
    /// Find the type of instruction that a given opcode represents
    fn from(opcode: OpCode) -> Self {
        // the last 4 digits tell you what the math operation is
        let operation = 0x000F & opcode;
        match operation {
            0x0 => MathOperation::Assign,
            0x1 => MathOperation::BitwiseOr,
            0x2 => MathOperation::BitwiseAnd,
            0x3 => MathOperation::BitwiseXor,
            0x4 => MathOperation::Add,
            0x5 => MathOperation::Subtract,
            0x6 => MathOperation::BitshiftRight,
            0x7 => MathOperation::Difference,
            0xE => MathOperation::BitshiftLeft,
            _ => MathOperation::UnknownOperation(opcode),
        }
    }
}

/// All the different possible instructions
/// NNN: Address
/// NN: 8-bit constant
/// N: 4-bit constant
/// X and Y: 4-bit register identifier
/// PC: Program Counter
/// I: 16-bit register for memory addresses (pointer)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    /// 0x0NNN
    /// Calls a machine code routine at address NNN,
    /// Probably not neccesary
    MachineCodeCall(u16),

    /// 0x0000
    /// Halt emulator
    Halt,

    /// 0x00E0
    /// Clears the screen
    ClearDisplay,

    // 0x00EE
    // Returns from current subroutine
    Return,

    /// 0x1NNN
    /// Jump to adress NNN
    /// PC = NNN
    Goto {
        address: Address,
    },

    /// 0x2NNN
    /// Call subroutine at NNN
    Call {
        address: Address,
    },

    /// 0x3XNN
    /// Skip next instruction if VX == NN
    RegisterEqualToConst {
        register: Register,
        value: u8,
    },

    /// 0x4XNN
    /// Skip next instruction if VX != NN
    RegisterNotEqualToConst {
        register: Register,
        value: u8,
    },

    /// 0x5XY0
    /// Skip next instruction if VX == VY
    RegistersEqual(Register, Register),

    /// 0x6XNN
    /// Set VX to NN
    SetRegister {
        register: Register,
        value: u8,
    },

    /// 0x7XNN
    /// Vx += NN (no carry flag, but still wrap)
    AddConst {
        register: Register,
        value: u8,
    },

    /// 0x8XYO
    /// Do math/bitwise operation O with registers X and Y
    Math {
        source: Register,
        destination: Register,
        operation: MathOperation,
    },

    /// 0x9XY0
    /// Skip next instruction if VX != VY
    RegistersNotEqual(Register, Register),

    /// 0xANNN
    /// I = NNN
    SetPointer(u16),

    /// 0xBNNN
    /// PC = V0 + NNN
    /// Jump to address NNN + V0
    JumpRelative {
        offset: u16,
    },

    /// 0xCXNN
    /// Vx = rand()&NN
    Random {
        register: Register,
        mask: u8,
    },

    /// 0xDXYN
    /// draw(Vx, Vy, N)
    /// Draw a sprite at coordinates (Vx, Vy) with a width of 8 pixels
    /// and a height of N+1 pixels. Each row of 8 pixels is read as bit-coded
    /// starting from memory location I; I is not changed at the end of the instruction.
    /// VF is set to 1 if any screen pixels are flipped from set to unset when the sprite
    /// is drawn and to 0 otherwise. This allows for some collision detection.
    Draw {
        position: (Register, Register),
        height: u8,
    },

    /// 0xEX9E
    /// If key stored in Vx is pressed, skip the next instruction
    KeyPressed(Register),

    /// 0xEXA1
    /// If key stored in Vx is not pressed, skip the next instruction
    KeyNotPressed(Register),

    /// 0xFX07
    /// Set Vx to the value of the delay timer
    GetDelayTimer(Register),

    /// 0xFX0A
    /// Wait for a key press then store it in Vx
    /// *BLOCKING*
    WaitKeyPress(Register),

    /// 0xFX15
    /// Set the delay timer to Vx
    SetDelayTimer(Register),

    /// 0xFX18
    /// Set the sound timer to Vx
    SetSoundTimer(Register),

    /// 0xFX1E
    /// I += Vx (no carry bit)
    AddToPointer(Register),

    /// 0xFX29
    /// Set I to the location of the sprite for the character in Vx.
    /// Characters 0-F are represented by a 4x5 font
    SetPointerToLetter(Register),

    /// 0xFX33
    /// Split number in Vx into it's decimal place values.
    /// *(I+0) = hundreds place
    /// *(I+1) = tens place
    /// *(I+2) = ones place
    SplitNumber(Register),

    /// 0xFX55
    /// Store V0 to Vx (inclusive) in memory starting at address I.
    /// Doesn't modify I
    RegisterDump(Register),

    /// 0xFX65
    /// Fill V0 to Vx (inclusive) from memory starting at address I.
    /// Doesn't modify I
    RegisterLoad(Register),

    /// TODO: Change out for using TryFrom instead of this crutch
    UndefinedOperation(u16),
}

/// Takes a value and returns a range of bytes from that value
///
/// For reference, it shifts the value according to this table
/// | Location | Size | Shift amount |
/// |----------|------|--------------|
/// | 0        | 1    | 3            |
/// | 0        | 2    | 2            |
/// | 0        | 3    | 1            |
/// | 0        | 4    | 0            |
/// | 1        | 1    | 2            |
/// | 1        | 2    | 1            |
/// | 1        | 3    | 0            |
/// | 2        | 1    | 1            |
/// | 2        | 2    | 0            |
/// | 3        | 1    | 0            |
///
/// # Panics
/// This function panics if given an invalid set of arguments.
/// Specifically, if the size is zero, or if the sum of the size and location is greater than 4,
/// since those arguments would produce invalid values.
/// Generally, the location and size arguments should be set statically at the call site,
/// so that you don't accidentally pass in invalid arguments.
///
/// # Examples
///
/// We'll use the value `0xDEAF` for the following examples:
/// ```
/// use chip8_core::instruction::{get_nibble, get_nibbles};
/// let bytes = 0xDEAF;
/// ```
///
/// - Get the last 3 nibbles of the value
/// ```
/// # use chip8_core::instruction::{get_nibble, get_nibbles};
/// # let bytes = 0xDEAF;
///
/// println!("{:X}", get_nibbles(bytes, 1, 3));
/// ```
///
/// - Get the first nibble of the value
/// ```
/// # use chip8_core::instruction::{get_nibble, get_nibbles};
/// # let bytes = 0xDEAF;
///
/// println!("{:X}", get_nibble(bytes, 0));
/// ```
pub fn get_nibbles(value: u16, location: u8, size: u8) -> u16 {
    let mask = match size {
        1 => 0xF,
        2 => 0xFF,
        3 => 0xFFF,
        4 => return value,
        0 => panic!("Can't get a value zero nibbles long"),
        _ => panic!("Can't get a value more than 4 nibbles long from a u16"),
    };

    assert!(
        location <= 3,
        "Can't get nibbles past the end of the value (location: {})",
        location
    );
    assert!(
        (location + size) <= 4,
        "A value {} nibbles long at an offset of {} goes past the end of the value",
        size,
        location
    );

    // we only want $size nibbles starting $location nibbles from the left, so we shift the value over (location - size) nibbles to the right
    let shifted_value = value >> ((4 - location - size) * 4);

    // AND the shifted value with the mask to remove the remaining bits on the left and return the extracted nibbles
    shifted_value & mask
}

/// Get a given nibble (hexidecimal digit) from the given value
///
/// This is an alias for calling [`get_nibbles`] with a size of one.
pub fn get_nibble(value: u16, location: u8) -> u8 {
    get_nibbles(value, location, 1) as u8
}

impl From<u16> for Instruction {
    fn from(instruction: u16) -> Self {
        // get the first nibble of the opcode, which is the category of the instruction
        let category_num = get_nibble(instruction, 0);
        match category_num {
            0x0 => {
                if instruction == 0x0000 {
                    Instruction::Halt
                } else if instruction == 0x00E0 {
                    Instruction::ClearDisplay
                } else if instruction == 0x00EE {
                    Instruction::Return
                } else {
                    Instruction::MachineCodeCall(instruction)
                }
            }
            0x1 => Instruction::Goto {
                address: 0x0FFF & instruction,
            },
            0x2 => Instruction::Call {
                address: 0x0FFF & instruction,
            },
            0x3 => Instruction::RegisterEqualToConst {
                register: get_nibble(instruction, 1) as Register, //((0x0F00 & instruction) >> 8) as Register,
                value: (0x00FF & instruction) as u8,
            },
            0x4 => Instruction::RegisterNotEqualToConst {
                register: get_nibble(instruction, 1) as Register,
                value: get_nibbles(instruction, 2, 2) as u8,
            },
            0x5 => Instruction::RegistersEqual(
                get_nibble(instruction, 1) as Register,
                get_nibble(instruction, 2) as Register,
            ),
            0x6 => Instruction::SetRegister {
                register: get_nibble(instruction, 1) as Register,
                value: (instruction & 0x00FF) as u8,
            },
            0x7 => Instruction::AddConst {
                register: get_nibble(instruction, 1) as Register,
                value: (instruction & 0x00FF) as u8,
            },
            0x8 => {
                // Math instruction is 0x8XYO where
                // X = register to assign result to / operate on
                // Y = Register to get info from (ie multiply X by)
                // O = Math operation
                // so we call X destination and Y source
                let source = get_nibble(instruction, 2) as Register;
                let destination = get_nibble(instruction, 1) as Register;
                Instruction::Math {
                    source,
                    destination,
                    operation: instruction.into(),
                }
            }
            0x9 => Instruction::RegistersNotEqual(
                get_nibble(instruction, 1) as Register,
                get_nibble(instruction, 2) as Register,
            ),
            0xA => Instruction::SetPointer(instruction & 0x0FFF),
            0xB => Instruction::JumpRelative {
                offset: instruction & 0x0FFF,
            },
            0xC => Instruction::Random {
                register: get_nibble(instruction, 1) as Register,
                mask: (instruction & 0x00FF) as u8,
            },
            0xD => Instruction::Draw {
                position: (
                    get_nibble(instruction, 1) as Register,
                    get_nibble(instruction, 2) as Register,
                ),
                height: (instruction & 0x000F) as u8,
            },
            0xE => {
                let sub_instruction = (0x00FF & instruction) as u8;
                let register = get_nibble(instruction, 1) as Register;
                if sub_instruction == 0x9E {
                    Instruction::KeyPressed(register)
                } else if sub_instruction == 0xA1 {
                    Instruction::KeyNotPressed(register)
                } else {
                    Instruction::UndefinedOperation(instruction)
                }
            }
            0xF => {
                let sub_instruction = (0x00FF & instruction) as u8;
                let register = get_nibble(instruction, 1) as Register;
                match sub_instruction {
                    0x07 => Instruction::GetDelayTimer(register),
                    0x0A => Instruction::WaitKeyPress(register),
                    0x15 => Instruction::SetDelayTimer(register),
                    0x18 => Instruction::SetSoundTimer(register),
                    0x1E => Instruction::AddToPointer(register),
                    0x29 => Instruction::SetPointerToLetter(register),
                    0x33 => Instruction::SplitNumber(register),
                    0x55 => Instruction::RegisterDump(register),
                    0x65 => Instruction::RegisterLoad(register),
                    _ => Instruction::UndefinedOperation(instruction),
                }
            }
            _ => unreachable!("get_nibble returned value above 0xF"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test decoding the two instructions that don't have any arguments
    #[test]
    fn decode_no_arg_instructions() {
        assert_eq!(Instruction::from(0x00E0), Instruction::ClearDisplay);

        assert_eq!(Instruction::from(0x00EE), Instruction::Return);
    }

    #[test]
    fn decode_flow_instructions() {
        assert_eq!(
            Instruction::from(0x1321),
            Instruction::Goto { address: 0x321 },
            "Decode goto instruction with the address 0x321"
        );

        assert_eq!(
            Instruction::from(0x2321),
            Instruction::Call { address: 0x321 },
            "Decode call instruction with the address 0x321"
        );

        assert_eq!(
            Instruction::from(0xB321),
            Instruction::JumpRelative { offset: 0x321 },
            "Decode jump relative instruction with the offset 0x321"
        );
    }

    #[test]
    fn decode_bcd() {
        assert_eq!(
            Instruction::from(0xF433),
            Instruction::SplitNumber(4),
            "Decode BCD/split number instruction with register 4"
        );
    }

    /// Single digit hex for each value
    fn test_math_op(
        operation: u8,
        source: u8,
        destination: u8,
        variant: MathOperation,
        desc: &'static str,
    ) {
        let opcode =
            0x8000 + (destination as u16 * 0x100) + (source as u16 * 0x10) + operation as u16;

        assert_eq!(
            Instruction::from(opcode),
            Instruction::Math {
                source: source,
                destination,
                operation: variant
            },
            "{}",
            desc
        );
    }

    #[test]
    fn decode_math_instructions() {
        use MathOperation::*;

        test_math_op(
            0x0,
            0x2,
            0xA,
            Assign,
            "Decode assign math instruction with register 2 to register A",
        );

        test_math_op(
            0x1,
            0x0,
            0x8,
            BitwiseOr,
            "Decode bitwise OR operation from register 0 onto register 8",
        );

        test_math_op(
            0x2,
            0xA,
            0x3,
            BitwiseAnd,
            "Decode bitwise AND operation from register A onto register 2",
        );

        test_math_op(
            0x3,
            0xD,
            0x6,
            BitwiseXor,
            "Decode bitwise XOR operation from register D onto register 6",
        );

        test_math_op(
            0x4,
            0xF,
            0xE,
            Add,
            "Decode add instruction from register F onto register E",
        );

        test_math_op(
            0x5,
            0x2,
            0x5,
            Subtract,
            "Decode subtract instruction from register 2 onto register 5",
        );

        test_math_op(
            0x6,
            0x0,
            0x3,
            BitshiftRight,
            "Decode bitwise right shift on register 0x3, source is ignored",
        );
    }

    /// assert_eq! with custom message to compare values in binary, for testing bit shifting functions
    macro_rules! binary_assert_eq {
        ($a:expr, $b:expr) => {
            assert_eq!($b, $a, "\nExpected: {:b}\nFound:    {:b}", $b, $a);
        };
    }

    #[test]
    fn test_get_nibble() {
        let bytes = 0xABCD;
        binary_assert_eq!(get_nibble(bytes, 0), 0xA);
        binary_assert_eq!(get_nibble(bytes, 1), 0xB);
        binary_assert_eq!(get_nibble(bytes, 2), 0xC);
        binary_assert_eq!(get_nibble(bytes, 3), 0xD);
    }

    /// Test get_nibbles function, ignoring potential values
    #[test]
    fn test_get_nibbles() {
        let bytes = 0xABCD;
        binary_assert_eq!(get_nibbles(bytes, 0, 2), 0xAB);
        binary_assert_eq!(get_nibbles(bytes, 0, 3), 0xABC);
        binary_assert_eq!(get_nibbles(bytes, 1, 2), 0xBC);
        binary_assert_eq!(get_nibbles(bytes, 1, 3), 0xBCD);
        binary_assert_eq!(get_nibbles(bytes, 2, 2), 0xCD);
    }
}
