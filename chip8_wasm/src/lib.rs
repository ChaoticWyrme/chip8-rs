pub mod utils;

use std::ops::{Deref, DerefMut};

use chip8_core::Chip8;
use once_cell::unsync::Lazy;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut STATE: Lazy<Chip8> = Lazy::new(|| Chip8::new());

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
        let mut emu = Chip8Wrap(Chip8::new());
        emu.memory[0x200..0x200 + IBM_LOGO.len()].copy_from_slice(IBM_LOGO);
        emu
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
}

#[wasm_bindgen]
pub fn init_emu() {
    unsafe { STATE.memory[0x200..0x200 + IBM_LOGO.len()].copy_from_slice(IBM_LOGO) }
}

#[wasm_bindgen]
pub fn get_display_pointer() -> *const bool {
    unsafe { STATE.display.pixels.as_ptr() }
}

#[wasm_bindgen]
pub fn tick() {
    unsafe {
        STATE.run_next().unwrap();
    }
}

#[wasm_bindgen]
pub fn render_text() -> String {
    unsafe { STATE.display.to_string() }
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}
