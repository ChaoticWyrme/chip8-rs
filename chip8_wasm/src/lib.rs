pub mod utils;

use std::convert::TryInto;
use std::ops::{Deref, DerefMut};

use chip8_core::quirks::QuirkConfig;
use chip8_core::time::Timers;
pub use chip8_core::Chip8;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const DEFAULT_ROM: &[u8] = include_bytes!("../chip8-test-suite.ch8");

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Default)]
#[wasm_bindgen(js_name = "Chip8")]
pub struct WasmChip8(Chip8);

impl Deref for WasmChip8 {
    type Target = Chip8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WasmChip8 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[wasm_bindgen(js_class = "Chip8")]
impl WasmChip8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmChip8 {
        WasmChip8(Chip8::new())
    }

    pub fn reset(&mut self) {
        self.0.reset();
    }

    pub fn load_default(&mut self) {
        self.memory[0x200..0x200 + DEFAULT_ROM.len()].copy_from_slice(DEFAULT_ROM);
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        if rom.len() >= self.memory.len() - 0x200 {
            log::warn!("Rom too large, there may be errors");
        } else {
            self.memory[0x200..0x200 + rom.len()].copy_from_slice(rom);
        }
    }

    pub fn get_display_pointer(&self) -> *const bool {
        self.display.pixels.as_ptr()
    }

    pub fn get_ram_pointer(&self) -> *const u8 {
        self.memory.as_ptr()
    }

    pub fn tick(&mut self) {
        self.run_next().unwrap();
    }

    pub fn render_text(&self) -> String {
        self.display.to_string()
    }

    pub fn key_down(&mut self, key: u8) {
        if key > 0xf {
            // Return early for invalid input
            return;
        }

        self.press_key(key.try_into().expect("Already checked range"));
    }

    pub fn key_up(&mut self, key: u8) {
        if key > 0xf {
            // Return early for invalid input
            return;
        }

        self.release_key(key.try_into().expect("Already checked range"));
    }

    pub fn exec_instruction(&mut self, opcode: u16) {
        let instruction = opcode.into();
        log::debug!("Executing {:?}", instruction);
        self.handle_instruction(instruction).unwrap();
    }

    pub fn set_log_level(&self, level_str: &str) {
        let level = match level_str.trim() {
            "debug" => log::LevelFilter::Debug,
            "error" => log::LevelFilter::Error,
            "trace" => log::LevelFilter::Trace,
            "warn" => log::LevelFilter::Warn,
            "info" => log::LevelFilter::Warn,
            _ => log::LevelFilter::Debug,
        };
        log::set_max_level(level);
    }

    pub fn get_wait_register(&self) -> String {
        format!("{:X?}", self.key_wait_register)
    }

    pub fn change_quirk(&mut self, quirk_name: &str, new_val: bool) {
        match quirk_name {
            "flag_reset" => self.quirks.flag_reset = new_val,
            "save_load_set_pointer" => self.quirks.save_load_set_pointer = new_val,
            "display_wait" => self.quirks.display_wait = new_val,
            "partial_wrap" => self.quirks.partial_wrap = new_val,
            "alt_shift" => self.quirks.alt_shift = new_val,
            "alt_rel_jump" => self.quirks.alt_rel_jump = new_val,
            _ => log::error!("Invalid quirk name queried {}", quirk_name),
        };
    }

    pub fn opcode_to_instruction_string(opcode: u16) -> String {
        let instr = chip8_core::instruction::Instruction::from(opcode);
        instr.to_string()
    }

    pub fn current_instruction(&self) -> u16 {
        self.0.get_u16(self.pc as usize)
    }

    pub fn get_instruction(&self, index: usize) -> u16 {
        self.get_u16(index)
    }

    #[wasm_bindgen(getter)]
    pub fn quirks(&self) -> QuirkConfig {
        self.quirks.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_quirks(&mut self, quirks: QuirkConfig) {
        self.quirks = quirks
    }

    #[wasm_bindgen(getter)]
    pub fn timers(&self) -> Timers {
        self.timers.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn running(&self) -> bool {
        self.running
    }

    #[wasm_bindgen(setter)]
    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    #[wasm_bindgen(getter)]
    pub fn program_counter(&self) -> u16 {
        self.pc
    }

    #[wasm_bindgen(setter)]
    pub fn set_program_counter(&mut self, pc: u16) {
        self.pc = pc
    }
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}
