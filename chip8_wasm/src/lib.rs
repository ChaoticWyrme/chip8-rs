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

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const IBM_LOGO: &'static [u8] = include_bytes!("../../roms/chip8-roms/programs/IBM Logo.ch8");

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

    pub fn load_ibm(&mut self) {
        self.memory[0x200..0x200 + IBM_LOGO.len()].copy_from_slice(IBM_LOGO);
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        if rom.len() >= self.memory.len() - 0x200 {
            log!("Rom too large, there may be errors");
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
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}
