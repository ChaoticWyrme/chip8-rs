#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

// We duplicate display so that we can have wasm_bindgen(skip) on pixels
// Without this, we need https://github.com/rust-lang/rust/issues/82679 (cfg_eval macro) to be stabilized

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct Display {
    #[wasm_bindgen(skip)]
    pub pixels: Vec<bool>,
    width: usize,
    height: usize,
}

#[cfg(not(feature = "wasm"))]
pub struct Display {
    pub pixels: Vec<bool>,
    width: usize,
    height: usize,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl Display {
    #[wasm_bindgen(getter)]
    pub fn pixels(&self) -> Vec<u8> {
        self.pixels.iter().map(|&val| val as u8).collect()
    }
}

impl Display {
    // Could definitely use a more efficient bitpacking algorithm here.
    // Make collision detection easy
    // Just AND slice of pixels and slice of sprite and test if greater than 1
    // (all 1s are where old frame already on and new sprite is flipping)
    // Then just XOR the two slices for the new values that will invert any bits where the sprite is one
    // We can use bitvec, with bitfields
    pub fn new(width: usize, height: usize) -> Self {
        debug_assert_eq!(width % 8, 0, "Width must be a multiple of 8");
        Display {
            pixels: vec![false; width * height],
            width,
            height,
        }
    }

    fn get_offset(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    /// Flips pixel at (x, y) to it's opposite state
    /// # Arguments
    /// * `x` - The x position of the pixel to flip
    /// * `y` - The y position of the pixel to flip
    /// # Returns
    /// Returns the new value of the pixel
    fn flip_pixel(&mut self, x: usize, y: usize) -> bool {
        let offset = self.get_offset(x, y);
        // log::info!(
        //     "Offset: {offset} = x({x}) + (y({y}) * width({})",
        //     self.width
        // );
        self.pixels[offset] = !self.pixels[offset];
        self.pixels[offset]
    }

    /// Draws a sprite from memory to the screen
    /// # Arguments
    /// * `pos_x` - The position of the sprite on the x-axis. Wraps if greater than self.width
    /// * `pos_y` - The position of the sprite on the y-axis. Wraps if greater than self.height
    /// * `sprite_height` - The height of the sprite 1-16. Certain modes can have 0 mean a 16x16 sprite, otherwise width is 8.
    /// * `memory` - A slice of the memory containing the sprite data, should be
    /// * `wrap_sprite` - Whether the sprite should wrap partially
    /// # Returns
    /// Returns true if a bit is flipped from on to off, false otherwise.
    pub fn draw_sprite(
        &mut self,
        pos_x: u8,
        pos_y: u8,
        sprite_height: u8,
        memory: &[u8],
        wrap_sprite: bool,
    ) -> bool {
        let sprite_height = sprite_height as usize;
        let sprite_width: usize = 8;
        let pos_x = pos_x as usize % self.width;
        let pos_y = pos_y as usize % self.height;
        let mut collide_check = false;

        for (row_index, mut y) in (pos_y..(pos_y + sprite_height)).enumerate() {
            let row: u8 = memory[row_index];
            let mut mask: u8 = 0b10000000;
            for mut x in pos_x..(pos_x + sprite_width) {
                if (row & mask) != 0 {
                    // modulo coordinates, so that it wraps around the screen
                    if wrap_sprite {
                        x %= self.width;
                        y %= self.height;
                    }

                    if x < self.width && y < self.height {
                        let result = self.flip_pixel(x, y);
                        // if a bit is flipped from on to off, this function should return true
                        if !result {
                            collide_check = true;
                        }
                    } else {
                        break;
                    }
                }
                mask >>= 1;
            }
        }

        collide_check
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn clear(&mut self) {
        // set every pixel to false
        self.pixels.iter_mut().for_each(|pixel| *pixel = false);
    }
}

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.pixels.as_slice().chunks(self.width) {
            for &pixel in line {
                let symbol = if pixel { '█' } else { '░' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for Display {
    fn default() -> Self {
        Display::new(64, 32)
    }
}
