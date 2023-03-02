pub mod utils;

use std::convert::TryInto;
use std::ops::{Deref, DerefMut};

use chip8_core::Chip8;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const DEFAULT_ROM: &'static [u8] = include_bytes!("../chip8-test-suite.ch8");

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Default)]
#[wasm_bindgen]
pub struct Chip8Wrap(Chip8);

impl Deref for Chip8Wrap {
    type Target = Chip8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Chip8Wrap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[wasm_bindgen]
impl Chip8Wrap {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Chip8Wrap {
        Chip8Wrap(Chip8::new())
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

    pub fn get_timers(&self) -> Timers {
        Timers {
            delay_timer: self.timers.delay,
            sound_timer: self.timers.sound,
        }
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
}

#[wasm_bindgen]
pub struct Timers {
    pub delay_timer: usize,
    pub sound_timer: usize,
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}
