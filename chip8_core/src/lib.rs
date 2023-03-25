pub mod display;
pub mod font;
pub mod instruction;
pub mod keypad;
pub mod quirks;
pub mod time;

use byteorder::ByteOrder;
use instruction::Instruction;
use keypad::{Key, Keypad};
use rand::Rng;
use time::Timers;

use thiserror::Error;

use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Error, Debug)]
pub enum DecodingError {
    #[error("Opcode ({opcode:X?}) doesn't match a known one")]
    InvalidOpcode { opcode: u16 },
    #[error("No math operation matching the given opcode ({opcode:X?})")]
    InvalidMathOperation { opcode: u16 },
    #[error("The {operation:?} operation cannot run because {reason:?}.")]
    InvalidState { operation: String, reason: String },
}

/// The VM state
pub struct Chip8 {
    //rom: [u8; 0x1000],
    /// Programs are loaded starting at 200
    /// 0x000 to 0x1FF are reserved for CHIP-8 interpreter
    /// Last 352 bytes are reserved for "variables and display refresh"
    /// Thus, programs have 0x200 to 0xE8F
    pub memory: [u8; 0x1000],
    // 16 valid registers, V0 to VF
    pub registers: [u8; 16],
    /// The P register
    pub pointer: u16,
    /// The program counter, should start at 200 by default.
    /// Increment by 2 per instruction, as instructions are 2 bytes long.
    pub pc: u16,
    pub stack: Vec<u16>,
    pub timers: Timers,
    pub display: display::Display,
    pub keypad: Keypad,
    pub running: bool,
    pub key_wait_register: Option<usize>,
    pub quirks: quirks::QuirkConfig,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            memory: [0_u8; 0x1000],
            registers: [0_u8; 16],
            pointer: 0,
            pc: 200,
            stack: Default::default(),
            timers: Default::default(),
            display: Default::default(),
            keypad: Default::default(),
            running: true,
            key_wait_register: None,
            quirks: Default::default(),
        }
    }
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            // rom: [0u8; 0x1000],
            memory: [0u8; 0x1000],
            registers: [0_u8; 16],
            pointer: 0,
            pc: 0x200,
            stack: vec![0u16; 0],
            timers: Timers::new(),
            display: display::Display::default(),
            keypad: Keypad::default(),
            running: true,
            key_wait_register: None,
            quirks: Default::default(),
        };
        font::load_font(&mut chip8.memory);
        chip8
    }

    pub fn reset(&mut self) {
        self.memory = [0u8; 0x1000];
        self.registers = Default::default();
        self.pointer = 0;
        self.pc = 0x200;
        self.timers = Default::default();
        self.display = Default::default();
        self.keypad = Default::default();
        self.key_wait_register = None;
    }

    pub fn is_key_waiting(&self) -> bool {
        self.key_wait_register.is_some()
    }

    pub fn press_key(&mut self, key: Key) {
        self.keypad.set_key(key, keypad::KeyState::Pressed);
        if let Some(wait_register) = self.key_wait_register {
            let key_val = key
                .to_u8()
                .expect("This shouldn't fail, it's an enum with a u8 value");
            log::info!("Key wait done with key: {:X}", key_val);
            self.registers[wait_register] = key_val;
            self.key_wait_register = None;
        }
    }

    pub fn release_key(&mut self, key: Key) {
        self.keypad.set_key(key, keypad::KeyState::NotPressed);
    }

    fn registers_to_string(&self) -> String {
        let mut strings = Vec::new();
        for i in 0..self.registers.len() {
            strings.push(format!("R{:X}: {:#x}", i, self.registers[i]));
        }
        strings.join(", ")
    }

    #[allow(dead_code, unused_variables)]
    pub fn handle_instruction(
        &mut self,
        instruction: instruction::Instruction,
    ) -> Result<(), DecodingError> {
        log::trace!("Executing instruction {:X?}", instruction);
        match instruction {
            Instruction::MachineCodeCall(opcode) => unimplemented!("Machine Code {:X}", opcode),
            Instruction::Halt => self.running = false,
            Instruction::ClearDisplay => self.display.clear(),
            Instruction::Return => {
                if self.stack.is_empty() {
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
                // wrapping add, but don't set carry flag
                self.registers[register as usize] =
                    self.registers[register as usize].wrapping_add(value);
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
                // if alternate relative jump is enabled, then use the first nibble of the offset as the register
                // otherwise, use register v0
                let register: usize = if self.quirks.alt_rel_jump {
                    // nibble 1 since offset is a u16 with only 3 bytes on the right,
                    // so from the left the first nibble is the second one. Thus, nibble location 1
                    instruction::get_nibble(offset, 1).into()
                } else {
                    0
                };
                self.pc = offset + self.registers[register] as u16;
            }
            Instruction::Random { register, mask } => {
                let rand: u8 = rand::thread_rng().gen();
                self.registers[register as usize] = rand & mask
            }
            Instruction::Draw { position, height } => {
                let mem_start = self.pointer as usize;
                // 8 bytes per row
                let mem_end = self.pointer as usize + (8 * height as usize);
                self.display.draw_sprite(
                    self.registers[position.0 as usize],
                    self.registers[position.1 as usize],
                    height,
                    &self.memory[mem_start..mem_end],
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
                println!("Is key {:x} (from register {}) pressed?", key, register);
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
                log::info!("Waiting on register {:X}", register);
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
            Instruction::SetPointerToLetter(register) => {
                self.pointer = font::get_letter_address(self.registers[register as usize])
            }
            Instruction::SplitNumber(register) => {
                let value = self.registers[register as usize];
                let digits = [
                    value / 100 % 10, // hundreds digit
                    value / 10 % 10,  // tens digit
                    value % 10,       // ones digit
                ];

                self.memory[self.pointer as usize] = digits[0];
                self.memory[self.pointer as usize + 1] = digits[1];
                self.memory[self.pointer as usize + 2] = digits[2];
            }
            Instruction::RegisterDump(register) => {
                for i in 0..=register as usize {
                    self.memory[self.pointer as usize + i] = self.registers[i]
                }
                if self.quirks.save_load_set_pointer {
                    self.pointer += (register as u16) + 1;
                }
            }
            Instruction::RegisterLoad(register) => {
                for i in 0..=register as usize {
                    self.registers[i] = self.memory[self.pointer as usize + i];
                }
                if self.quirks.save_load_set_pointer {
                    self.pointer += (register as u16) + 1;
                }
            }
            Instruction::UndefinedOperation(opcode) => {
                return Err(DecodingError::InvalidOpcode { opcode });
            }
        };
        Ok(())
    }

    pub fn next_instruction(&mut self) {
        self.pc += 2;
    }

    pub fn back_instruction(&mut self) {
        self.pc -= 2;
    }

    pub fn step_instructions(&mut self, steps: u16) {
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
                if self.quirks.flag_reset {
                    self.set_carry(false);
                }
            }
            BitwiseAnd => {
                self.registers[destination as usize] = source_val & dest_val;
                if self.quirks.flag_reset {
                    self.set_carry(false);
                }
            }
            BitwiseXor => {
                self.registers[destination as usize] = source_val ^ dest_val;
                if self.quirks.flag_reset {
                    self.set_carry(false);
                }
            }
            Add => {
                self.registers[destination as usize] = dest_val.wrapping_add(source_val);
                // dest_val = destination value before operation
                // compare to see if the result of addition is smaller than the original value
                self.set_carry(dest_val > self.registers[destination as usize]);
            }
            Subtract => {
                self.registers[destination as usize] = dest_val.wrapping_sub(source_val);
                // check if operation borrowed
                // for some reason the boolean on this operation is the opposite of addition
                // dest_val = cached value from before operation
                self.set_carry(dest_val > self.registers[destination as usize]);
            }
            Difference => {
                self.registers[destination as usize] = source_val.wrapping_sub(dest_val);
                self.set_carry(source_val > self.registers[destination as usize]);
            }
            /* For bitshift instructions, if alternate shift is enabled,
             * then bitshift source_val and store it in destination register,
             * otherwise use dest_val
             */
            BitshiftRight => {
                // calculate result depending on quirks
                let (carry, result) = if self.quirks.alt_shift {
                    (dest_val & 0b0000_0001 != 0, dest_val >> 1)
                } else {
                    (source_val & 0b0000_0001 != 0, source_val >> 1)
                };
                self.registers[destination as usize] = result;
                self.set_carry(carry);
            }
            BitshiftLeft => {
                // calculate result depending on quirks
                let (carry, result) = if self.quirks.alt_shift {
                    (dest_val & 0b0000_0001 != 0, dest_val << 1)
                } else {
                    (source_val & 0b0000_0001 != 0, source_val << 1)
                };
                self.registers[destination as usize] = result;
                self.set_carry(carry);
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

    /// Gets a u16 from the two u8s at index and the following item
    pub fn get_u16(&self, index: usize) -> u16 {
        byteorder::BE::read_u16(&self.memory[index..=(index + 1)])
    }

    fn get_instruction_at_pc(&self) -> Instruction {
        let instruction_data: u16 = self.get_u16(self.pc as usize);
        // println!("Instruction: {:#x}", instruction_data);
        instruction_data.into()
    }

    pub fn run_next(&mut self) -> Result<(), DecodingError> {
        self.timers.do_ticks();
        if !self.is_key_waiting() {
            let instruction = self.get_instruction_at_pc();
            self.next_instruction();
            self.handle_instruction(instruction)?;
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), DecodingError> {
        let mut skip_debug = false;
        while self.running {
            self.run_next()?;
            println!();
            println!("{}", self.display);
            println!("Instruction: {:X?}", self.get_instruction_at_pc());

            if self.is_key_waiting() {
                loop {
                    print!("Enter key: ");
                    let mut key = String::new();
                    std::io::stdin().read_line(&mut key).unwrap();
                    if let Ok(val) = u8::from_str_radix(key.trim(), 16) {
                        if val <= 0xf {
                            self.registers[self.key_wait_register.unwrap()] = val;
                            self.key_wait_register = None;
                            break;
                        }
                    }
                    print!("Error parsing, is this a single hex character?\nTry again: ");
                }
            }

            let mut user_input = String::new();
            loop {
                if skip_debug {
                    break;
                }
                std::io::stdin()
                    .read_line(&mut user_input)
                    .expect("Error reading from stdin");

                if user_input.starts_with("instruction") {
                    let suffix = user_input.split_once("instruction").unwrap().1.trim();
                    let mut address = self.pc;
                    if suffix.is_empty() {
                        // print current pc instruction
                        address = self.pc;
                    } else if suffix.starts_with('-') {
                        let offset =
                            u16::from_str_radix(suffix.split_once('-').unwrap().1, 16).unwrap();
                        address -= 2 * offset;
                    } else if suffix.starts_with('+') {
                        let offset =
                            u16::from_str_radix(suffix.split_once('-').unwrap().1, 16).unwrap();
                        address += 2 * offset;
                    }
                    user_input = format!("memory {:#x}", address);
                }

                if user_input.starts_with("registers") {
                    println!("Registers: \n{}", self.registers_to_string())
                } else if user_input.starts_with("pc") {
                    println!("Program counter: {:#6X}", self.pc);
                } else if user_input.starts_with("memory 0x") {
                    let hex_str = user_input.split("0x").nth(1).unwrap().trim();
                    let address = usize::from_str_radix(hex_str, 16).unwrap();
                    let mem = &self.memory[address..address + 2];

                    println!(
                        "Memory at address: {:#X}: {:#X}",
                        address,
                        byteorder::BigEndian::read_u16(mem)
                    );
                } else if user_input.starts_with("set 0x") {
                    let raw_input = user_input.split_once("0x").unwrap().1;
                    let (address_str, val_str) = raw_input.split_once("0x").unwrap();
                    let address =
                        usize::from_str_radix(address_str.trim(), 16).expect("Invalid usize hex");
                    let val = u8::from_str_radix(val_str.trim(), 16).expect("Invalid u8 hex");
                    self.memory[address] = val;
                    println!("Set address {:X} to {:X}", address, val);
                } else if user_input.starts_with("skip") {
                    skip_debug = true;
                    println!("Skipping debug until halt");
                    break;
                } else if user_input.starts_with("pointer") {
                    println!("Pointer: {:X}", self.pointer);
                } else {
                    break;
                }
                user_input.clear();
            }
        }
        println!("Halted");
        Ok(())
    }
}

impl std::fmt::Display for Chip8 {
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

#[cfg(test)]
mod tests {
    // TODO: Fix math tests, since I fixed subtle bugs in implementation
    // Should I move these tests to the integration tests?
    // use super::*;

    mod math_operations {
        use super::super::*;

        /// assert_eq! with custom message to compare values in binary, for testing bit shifting functions
        macro_rules! binary_assert_eq {
            ($a:expr, $b:expr) => {
                assert_eq!($b, $a, "\nExpected: {:b}\nFound:    {:b}", $b, $a);
            };
        }

        fn init_vm(opcode: u16) -> Chip8 {
            let mut vm = Chip8::new();
            // program counter starts at 0, so this'll be the first instruction
            let opcode_bytes = opcode.to_be_bytes();
            vm.memory[0] = opcode_bytes[0];
            vm.memory[1] = opcode_bytes[1];

            vm
        }

        #[test]
        fn assign() {
            let mut vm = init_vm(0x8120);

            // destination register, will be set to register 2 value
            vm.registers[1] = 4;
            // source register
            vm.registers[2] = 0xA;

            vm.run_next().expect("Decoding error on test instruction");

            assert_eq!(
                vm.registers[1], 0xA,
                "Value of register two not copied to register one"
            );
        }

        #[test]
        fn bitwise_or() {
            let mut vm = init_vm(0x8121);

            // destination register, will be OR'd with register 2
            vm.registers[1] = 0b101;
            vm.registers[2] = 0b110;
            //result should = 0b111

            vm.run_next().expect("Decoding error on test instruction");

            binary_assert_eq!(vm.registers[1], 0b111);
        }

        #[test]
        fn bitwise_and() {
            let mut vm = init_vm(0x8142);

            // destination register, will be AND'd with register 4
            vm.registers[1] = 0b1101;
            vm.registers[4] = 0b0111;
            //result should = 0b0101

            vm.run_next().expect("Decoding error on test instruction");

            binary_assert_eq!(vm.registers[1], 0b0101);
        }

        #[test]
        fn bitwise_xor() {
            let mut vm = init_vm(0x8123);

            // destination register, will be XOR'd with register 2
            vm.registers[1] = 0b1101;
            vm.registers[2] = 0b0110;
            //result should = 0b1011

            vm.run_next().expect("Decoding error on test instruction");

            binary_assert_eq!(vm.registers[1], 0b1011);
        }

        #[test]
        fn add_no_carry() {
            // test add with no carry
            let mut vm = init_vm(0x8124);

            // destination register, will be summed with register 2
            vm.registers[1] = 34;
            vm.registers[2] = 13;

            vm.run_next().expect("Decoding error on test instruction");

            assert_eq!(vm.registers[1], 34 + 13);
            assert!(!vm.get_carry(), "Unexpected positive carry flag");
        }

        #[test]
        fn add_carry() {
            // test carry flag is set and the addition wraps correctly
            let mut vm = init_vm(0x8124);

            vm.registers[1] = 254;
            vm.registers[2] = 30;

            vm.run_next().expect("Decoding error on test instruction");

            assert_eq!(vm.registers[1], 254_u8.wrapping_add(30), "Wrapping add");
            assert!(vm.get_carry(), "Unexpected non-one carry flag");
        }

        #[test]
        fn subtract_no_carry() {
            // test V1 -= V2 with no carry
            let mut vm = init_vm(0x8125);

            vm.registers[1] = 54;
            vm.registers[2] = 23;

            vm.run_next().expect("Decoding error on test instruction");

            assert_eq!(vm.registers[1], 54 - 23);
        }

        #[test]
        fn subtract_carry() {
            // test V1 -= V2 with carry
            let mut vm = init_vm(0x8125);

            // destination register, will be OR'd with register 2
            vm.registers[1] = 54;
            vm.registers[2] = 64;

            vm.run_next().expect("Decoding error on test instruction");

            assert_eq!(vm.registers[1], 54_u8.wrapping_sub(64));
            assert!(vm.get_carry())
        }

        #[test]
        fn bitshift_right() {
            // Test right bitshift: Store least signifigant bit in VF, then shift V1 to the right 1
            let mut vm = init_vm(0x8126);

            // source register is ignored
            vm.registers[1] = 0b1011;

            vm.run_next().expect("Decoding error on test instruction");

            binary_assert_eq!(vm.registers[1], 0b101);
            assert_eq!(vm.registers[0xF], 1, "Carrying right shift");

            vm.back_instruction();

            vm.registers[1] = 0b1100;

            vm.run_next().expect("Decoding error on test instruction");

            binary_assert_eq!(vm.registers[1], 0b110);
            assert_eq!(vm.registers[0xF], 0, "Non-carrying right shift");
        }

        #[test]
        fn bitshift_left() {
            // Test left bitshift: Store most signifigant bit in VF, then shift V1 to the left 1
            let mut vm = init_vm(0x812E);

            // source register is ignored
            vm.registers[1] = 0b1101_1011;

            vm.run_next().expect("Decoding error on test instruction");

            binary_assert_eq!(vm.registers[1], 0b101_10110);
            assert_eq!(vm.registers[0xF], 1, "Carrying left shift");

            vm.back_instruction();

            vm.registers[1] = 0b0101_0111;

            vm.run_next().expect("Decoding error on test instruction");

            binary_assert_eq!(vm.registers[1], 0b101_01110);
            assert_eq!(vm.registers[0xF], 0, "Non-carrying left shift");
        }

        #[ignore]
        #[test]
        fn difference() {
            // TODO: Finish this test
            // Test V1 = V2 - V1
            let mut vm = init_vm(0x8127);

            vm.registers[1] = 2;
            vm.registers[2] = 0b110;

            vm.run_next().expect("Decoding error on test instruction");

            assert_eq!(vm.registers[1], 0b111);
        }

        #[test]
        fn unknown_operation() {}
    }
}
