pub mod display;
pub mod instruction;
pub mod keypad;
pub mod time;

use std::fmt::Display;

use instruction::Instruction;
use keypad::{Key, Keypad};
use rand::Rng;
use time::Timers;

use thiserror::Error;

use num_traits::FromPrimitive;

#[derive(Error, Debug)]
pub enum DecodingError {
    #[error("Opcode ({opcode:?}) doesn't match a known one")]
    InvalidOpcode { opcode: u16 },
    #[error("No math operation matching the given opcode ({opcode:?})")]
    InvalidMathOperation { opcode: u16 },
    #[error("The {operation:?} operation cannot run because {reason:?}.")]
    InvalidState { operation: String, reason: String },
}

pub struct Chip8 {
    //rom: [u8; 0x1000],
    pub memory: [u8; 0x1000],
    pub registers: [u8; 16],
    pub pointer: u16,
    pub pc: u16,
    pub stack: Vec<u16>,
    pub timers: Timers,
    pub display: display::Display,
    pub keypad: Keypad,
    pub running: bool,
    key_wait_register: Option<usize>,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            memory: [0_u8; 0x1000],
            registers: Default::default(),
            pointer: Default::default(),
            pc: Default::default(),
            stack: Default::default(),
            timers: Default::default(),
            display: Default::default(),
            keypad: Default::default(),
            running: true,
            key_wait_register: None,
        }
    }
}

impl Display for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Chip8 internals:
        ROM: Opaque,
        Memory: Opaque,"
        )?;
        writeln!(f, "Registers: \n{}", self.registers_to_string())?;

        Ok(())
    }
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            // rom: [0u8; 0x1000],
            memory: [0u8; 0x1000],
            registers: [0_u8; 16],
            pointer: 0,
            pc: 0,
            stack: vec![0u16; 0],
            timers: Timers::new(),
            display: display::Display::default(),
            keypad: Keypad::default(),
            running: true,
            key_wait_register: None,
        }
    }

    pub fn is_key_waiting(&self) -> bool {
        self.key_wait_register.is_some()
    }

    fn registers_to_string(&self) -> String {
        let mut strings = Vec::new();
        for i in 0..self.registers.len() {
            strings.push(format!("R{:#x}: {:#x}", i, self.registers[i]));
        }
        strings.join(", ")
    }

    #[allow(dead_code, unused_variables)]
    pub fn handle_instruction(
        &mut self,
        instruction: instruction::Instruction,
    ) -> Result<(), DecodingError> {
        match instruction {
            Instruction::MachineCodeCall(_) => unimplemented!(),
            // TODO: IMPLEMENT DISPLAY
            Instruction::ClearDisplay => unimplemented!(),
            Instruction::Return => {
                if self.stack.len() == 0 {
                    self.running = false;
                    return Err(DecodingError::InvalidState {
                        operation: "Return".to_owned(),
                        reason: "there is no return point on the call stack".to_owned(),
                    });
                }
                let return_point = self
                    .stack
                    .pop()
                    .expect("Empty call stack at return statement");
                self.pc = return_point;
            }
            Instruction::Goto { address } => self.pc = address,
            Instruction::Call { address } => {
                self.stack.push(self.pc);
                self.pc = address;
            }
            Instruction::RegisterEqualToConst { register, value } => {
                // ugh, register has to be usize to index into an array
                if self.registers[register as usize] == value {
                    self.next_instruction();
                }
            }
            Instruction::RegisterNotEqualToConst { register, value } => {
                if self.registers[register as usize] != value {
                    self.next_instruction();
                }
            }
            Instruction::RegistersEqual(register1, register2) => {
                if self.registers[register1 as usize] == self.registers[register2 as usize] {
                    self.next_instruction();
                }
            }
            Instruction::SetRegister { register, value } => {
                self.registers[register as usize] = value;
            }
            Instruction::AddConst { register, value } => {
                self.registers[register as usize] += value;
            }
            Instruction::Math {
                source,
                destination,
                operation,
            } => {
                self.handle_math(source, destination, operation)?;
            }
            Instruction::RegistersNotEqual(register1, register2) => {
                if self.registers[register1 as usize] != self.registers[register2 as usize] {
                    self.next_instruction();
                }
            }
            Instruction::SetPointer(address) => self.pointer = address,
            Instruction::JumpRelative { offset } => {
                self.pc = offset + self.registers[0] as u16;
            }
            Instruction::Random { register, mask } => {
                let rand: u8 = rand::thread_rng().gen();
                self.registers[register as usize] = rand & mask
            }
            Instruction::Draw { position, height } => {
                self.display.draw_sprite(
                    position.0,
                    position.1,
                    height,
                    &self.memory[self.pointer as usize..self.memory.len()],
                );
            }
            Instruction::KeyPressed(register) => {
                let key = self.registers[register as usize];
                if self.keypad.is_key_pressed(
                    Key::from_u8(key).expect("Register contains value not in keypad range (0-15)"),
                ) {
                    self.step_instructions(1);
                }
            }
            Instruction::KeyNotPressed(register) => {
                let key = self.registers[register as usize];
                if !self.keypad.is_key_pressed(
                    Key::from_u8(key).expect("Register contains value not in keypad range (0-15)"),
                ) {
                    self.step_instructions(1);
                }
            }
            Instruction::GetDelayTimer(register) => {
                self.registers[register as usize] = self.timers.delay as u8
            }
            Instruction::WaitKeyPress(register) => {
                self.key_wait_register = Some(register as usize);
                // probably should have a callback for this
            }
            Instruction::SetDelayTimer(register) => {
                self.timers.delay = self.registers[register as usize].into();
            }
            Instruction::SetSoundTimer(register) => {
                self.timers.sound = self.registers[register as usize].into();
            }
            Instruction::AddToPointer(register) => {
                self.pointer += self.registers[register as usize] as u16;
            }
            Instruction::SetPointerToLetter(register) => todo!("Need to find chip8 font"),
            Instruction::SplitNumber(register) => {
                let value = self.registers[register as usize];
                let digits = [
                    value / 100 % 10, // hundreds digit
                    value / 10 % 10,  // tens digit
                    value % 10,       // ones digit
                ];

                self.memory[self.pointer as usize + 0] = digits[0];
                self.memory[self.pointer as usize + 1] = digits[1];
                self.memory[self.pointer as usize + 2] = digits[2];
            }
            Instruction::RegisterDump(register) => {
                for i in 0..register as usize {
                    self.memory[self.pointer as usize + i] = self.registers[i]
                }
            }
            Instruction::RegisterLoad(register) => {
                for i in 0..register as usize {
                    self.registers[i] = self.memory[self.pointer as usize + i];
                }
            }
            Instruction::UndefinedOperation(opcode) => {
                return Err(DecodingError::InvalidOpcode { opcode });
            }
        };
        Ok(())
    }

    fn next_instruction(&mut self) {
        self.pc += 2;
    }

    fn step_instructions(&mut self, steps: u16) {
        self.pc += steps * 2;
    }

    /// Handles a Instruction::Math
    fn handle_math(
        &mut self,
        source: u8,
        destination: u8,
        operation: instruction::MathOperation,
    ) -> Result<(), DecodingError> {
        use instruction::MathOperation::*;
        let source_val = self.registers[source as usize];
        let dest_val = self.registers[destination as usize];

        match operation {
            Assign => {
                self.registers[destination as usize] = source_val;
            }
            BitwiseOr => {
                self.registers[destination as usize] = source_val | dest_val;
            }
            BitwiseAnd => {
                self.registers[destination as usize] = source_val & dest_val;
            }
            BitwiseXor => {
                self.registers[destination as usize] = source_val ^ dest_val;
            }
            Add => {
                self.registers[destination as usize] += source_val;
                // dest_val = destination value before operation
                // compare to see if the result of addition is smaller than the original value
                self.set_carry(dest_val > self.registers[destination as usize]);
            }
            Subtract => {
                self.registers[destination as usize] -= dest_val;
                // check if operation carried
                // dest_val = cached value from before operation
                self.set_carry(dest_val < self.registers[destination as usize]);
            }
            BitshiftRight => {
                self.registers[0xF] = dest_val & 0b00000001;
                self.registers[destination as usize] = dest_val >> 1;
            }
            Difference => {
                self.registers[destination as usize] = source_val - dest_val;
            }
            BitshiftLeft => {
                self.registers[0xF] = dest_val & 0b10000000;
                self.registers[destination as usize] = dest_val << 1;
            }
            UnknownOperation(opcode) => {
                return Err(DecodingError::InvalidMathOperation { opcode });
            }
        }

        Ok(())
    }

    fn set_carry(&mut self, carry: bool) {
        self.registers[0xf] = carry as u8;
    }

    pub fn get_carry(&self) -> bool {
        self.registers[0xF] == 1
    }

    fn get_instruction_at_pc(&self) -> Instruction {
        use byteorder::ByteOrder;
        let instruction_data: u16 = byteorder::BigEndian::read_u16(
            &self.memory[(self.pc as usize)..((self.pc + 2) as usize)],
        );
        println!("Instruction: {:#x}", instruction_data);
        instruction_data.into()
    }

    pub fn run_next(&mut self) -> Result<(), DecodingError> {
        self.timers.do_ticks();
        let instruction = self.get_instruction_at_pc();
        self.handle_instruction(instruction)?;
        self.next_instruction();
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), DecodingError> {
        while self.running {
            self.run_next()?;
        }
        Ok(())
    }
}

/// A display frontend
pub trait Frontend {
    fn render_display(screen: display::Display);
    fn play_tone();
    fn stop_tone();
    fn is_key_pressed(key: char);
    fn wait_for_key(key: char);
}

struct MockFrontend;

impl Frontend for MockFrontend {
    fn render_display(screen: display::Display) {}

    fn play_tone() {
        todo!()
    }

    fn stop_tone() {
        todo!()
    }

    fn is_key_pressed(key: char) {
        todo!()
    }

    fn wait_for_key(key: char) {
        todo!()
    }
}
