type Register = u8;
type Address = u16;
type OpCode = u16;

// TODO: Change From implementations into TryFrom, since they have failure conditions

/// get_bytes(value, start, length)
/// gets length bytes starting "start" bytes from the left
macro_rules! get_bytes {
    ($value:ident, $location:literal, 1) => {
        (0xF000 >> $location) & $value
    };
    ($value:ident, $location:literal, 2) => {
        (0xFF00 >> $location) & $value
    };
    ($value:ident, $location:literal, 3) => {
        (0xFFF0 >> $location) & $value
    };
    ($value:ident, 1, $size:literal) => {
        get_bytes!($value, 4, $size)
    };
    ($value:ident, 2, $size:literal <= 2) => {
        get_bytes!($value, 8, $size)
    };
    ($value:ident, 3, 1) => {
        get_bytes!($value, 12, 1)
    };
    ($value:ident, $location:literal) => {
        get_bytes!($value, $location, 1)
    };
}

#[derive(Debug)]
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
    UnknownOperation(OpCode)
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
pub enum Instruction {

    /// 0x0NNN
    /// Calls a machine code routine at address NNN,
    /// Probably not neccesary
    MachineCodeCall(u16),

    /// 0x00E0
    /// Clears the screen
    ClearDisplay,

    // 0x00EE
    // Returns from current subroutine
    Return,

    /// 0x1NNN
    /// Jump to adress NNN
    /// PC = NNN
    Goto { address: Address },
    
    /// 0x2NNN
    /// Call subroutine at NNN
    Call { address: Address },

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
        value: u8
    },

    /// 0x5XY0
    /// Skip next instruction if VX == VY
    RegistersEqual(Register, Register),

    /// 0x6XNN
    /// Set VX to NN
    SetRegister {
        register: Register,
        value: u8
    },

    /// 0x7XNN
    /// Vx += NN (no carry flag)
    AddConst {
        register: Register,
        value: u8
    },

    /// 0x8XYO
    /// Do math/bitwise operation O with registers X and Y
    Math {
        source: Register,
        destination: Register,
        operation: MathOperation
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
    JumpRelative { offset: u16 },

    /// 0xCXNN
    /// Vx = rand()&NN
    Random {
        register: Register,
        mask: u8
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
        height: u8
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
    UndefinedOperation(u16)
}

impl From<u16> for Instruction {
    fn from(instruction: u16) -> Self {
        // zero out all bits after the first 2
        // and cast to u8, so it's easy to compare
        let category_num: u8 = ((instruction & 0xF000) >> 12) as u8;
        match category_num {
            0x0 => {
                if instruction == 0x00E0 {
                    Instruction::ClearDisplay
                } else if instruction == 0x00EE {
                    Instruction::Return
                } else {
                    Instruction::MachineCodeCall(0x0FFF & instruction)
                }
            },
            0x1 => Instruction::Goto {
                address: 0x0FFF & instruction
            },
            0x2 => Instruction::Call {
                address: 0x0FFF & instruction
            },
            0x3 => Instruction::RegisterEqualToConst {
                register: get_bytes!(instruction, 1) as Register,//((0x0F00 & instruction) >> 8) as Register,
                value: (0x00FF & instruction) as u8
            },
            0x4 => Instruction::RegisterNotEqualToConst {
                register:  get_bytes!(instruction, 1) as Register,
                value: get_bytes!(instruction, 2, 2) as u8
            },
            0x5 => Instruction::RegistersEqual(
                get_bytes!(instruction, 1) as Register,
                get_bytes!(instruction, 2) as Register
            ),
            0x6 => Instruction::SetRegister {
                register: get_bytes!(instruction, 1) as Register,
                value: (instruction & 0x00FF) as u8
            },
            0x7 => Instruction::AddConst {
                register: get_bytes!(instruction, 1) as Register,
                value: (instruction & 0x00FF) as u8
            },
            0x8 => {
                let source = get_bytes!(instruction, 1) as Register;
                let destination = get_bytes!(instruction, 2) as Register;
                Instruction::Math {
                    source,
                    destination,
                    operation: instruction.into(),
                }
            },
            0x9 => Instruction::RegistersNotEqual(
                get_bytes!(instruction, 1) as Register,
                get_bytes!(instruction, 2) as Register,
            ),
            0xA => Instruction::SetPointer(get_bytes!(instruction, 1, 3)),
            0xB => Instruction::JumpRelative {
                offset: instruction & 0x0FFF,
            },
            0xC => Instruction::Random {
                register: get_bytes!(instruction, 1) as Register,
                mask: (instruction & 0x00FF) as u8,
            },
            0xD => Instruction::Draw {
                position: (
                    get_bytes!(instruction, 1) as Register,
                    get_bytes!(instruction, 2) as Register
                ),
                height: (instruction & 0x000F) as u8,
            },
            0xE => {
                let sub_instruction = (0x00FF & instruction) as u8;
                let register = get_bytes!(instruction, 1) as Register;
                if sub_instruction == 0x9E {
                    Instruction::KeyPressed(register)
                } else if sub_instruction == 0xA1 {
                    Instruction::KeyNotPressed(register)
                } else {
                    Instruction::UndefinedOperation(instruction)
                }
            },
            0xF => {
                let sub_instruction = (0x00FF & instruction) as u8;
                let register = get_bytes!(instruction, 1) as Register;
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
                    _ => Instruction::UndefinedOperation(instruction)
                }
            }
            _ => Instruction::UndefinedOperation(instruction),
        }
    }
}