#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct QuirkConfig {
    // Chip8 quirks
    /// Whether to reset flags register to zero on AND OR and XOR instructions
    pub flag_reset: bool,

    /// Whether to set the pointer register to the last used memory location when saving or loading registers
    pub save_load_set_pointer: bool,

    /// On draw instruction, wait for next v-blank, limiting the program to ~60 FPS
    pub display_wait: bool,

    /// Wrap sprite even when only partially over the edge
    pub partial_wrap: bool,

    // CHIP-48 and SUPER-CHIP quirks
    /// Bitshift instructions operate purely on vX rather than storing vX into vY and then shifting it
    pub alt_shift: bool,

    /// For the BNNN jump instruction, instead of v0 + NNN,
    /// select the register from the highest nibble of NNN
    pub alt_rel_jump: bool,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl QuirkConfig {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn use_preset(&mut self, preset: QuirkPresets) {
        match preset {
            QuirkPresets::Chip8 => {
                self.flag_reset = true;
                self.save_load_set_pointer = true;
                self.display_wait = true;
                self.partial_wrap = false;
                self.alt_shift = false;
                self.alt_rel_jump = false;
            }
            QuirkPresets::SuperChip => {
                self.flag_reset = false;
                self.save_load_set_pointer = false;
                self.display_wait = false;
                self.partial_wrap = false;
                self.alt_shift = true;
                self.alt_rel_jump = true;
            }
            QuirkPresets::XoChip => {
                self.flag_reset = false;
                self.save_load_set_pointer = true;
                self.display_wait = false;
                self.partial_wrap = false;
                self.alt_shift = false;
                self.alt_rel_jump = false;
            }
        }
    }
}

impl Default for QuirkConfig {
    fn default() -> Self {
        // right now, we just create this with all falses, then set them in use_preset
        let mut config = Self {
            flag_reset: false,
            save_load_set_pointer: false,
            display_wait: false,
            partial_wrap: false,
            alt_shift: false,
            alt_rel_jump: false,
        };
        config.use_preset(QuirkPresets::Chip8);
        config
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, Debug)]
pub enum QuirkPresets {
    Chip8,
    SuperChip,
    XoChip,
}
