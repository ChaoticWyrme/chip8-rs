pub struct Display {
    pub pixels: Vec<Vec<bool>>,
    width: usize,
    height: usize,
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
            pixels: vec![vec![false; width]; height],
            width,
            height,
        }
    }

    /// Flips pixel at (x, y) to it's opposite state
    /// # Arguments
    /// * `x` - The x position of the pixel to flip
    /// * `y` - The y position of the pixel to flip
    /// # Returns
    /// Returns the new value of the pixel
    fn flip_pixel(&mut self, x: usize, y: usize) -> bool {
        self.pixels[y][x] = !self.pixels[y][x];
        return self.pixels[y][x];
    }

    /// Draws a sprite from memory to the screen
    /// # Arguments
    /// * `height` - The height of the sprite 1-16
    /// * `memory` - A slice of the memory containing the sprite data, should be
    /// # Returns
    /// Returns true if a bit is flipped from on to off, false otherwise.
    pub fn draw_sprite(&mut self, pos_x: u8, pos_y: u8, height: u8, memory: &[u8]) -> bool {
        let height = height as usize;
        let width: usize = 8;
        let pos_x = pos_x as usize;
        let pos_y = pos_y as usize;
        let mut collide_check = false;

        for y in pos_y..(pos_y + height) {
            let row: u8 = memory[y];
            let mut mask: u8 = 0b10000000;
            for x in pos_x..(pos_x + width) {
                if (row & mask) != 0 {
                    let result = self.flip_pixel(x, y);
                    // if a bit is flipped from on to off, this function should return true
                    if !result {
                        collide_check = true;
                    }
                }
                mask = mask >> 1;
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
}

impl Default for Display {
    fn default() -> Self {
        Display::new(64, 32)
    }
}
