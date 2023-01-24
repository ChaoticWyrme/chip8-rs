/// Constants containing the binary sprites for the letters
pub mod letters {
    pub const LETTER_0: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0];
    pub const LETTER_1: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
    pub const LETTER_2: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
    pub const LETTER_3: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
    pub const LETTER_4: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
    pub const LETTER_5: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
    pub const LETTER_6: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
    pub const LETTER_7: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
    pub const LETTER_8: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
    pub const LETTER_9: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
    pub const LETTER_A: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
    pub const LETTER_B: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
    pub const LETTER_C: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
    pub const LETTER_D: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
    pub const LETTER_E: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
    pub const LETTER_F: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];
}

pub fn load_font(memory: &mut [u8]) {
    use letters::*;

    memory[0..5].copy_from_slice(&LETTER_0);
    memory[5..10].copy_from_slice(&LETTER_1);
    memory[10..15].copy_from_slice(&LETTER_2);
    memory[15..20].copy_from_slice(&LETTER_3);
    memory[20..25].copy_from_slice(&LETTER_4);
    memory[25..30].copy_from_slice(&LETTER_5);
    memory[30..35].copy_from_slice(&LETTER_6);
    memory[35..40].copy_from_slice(&LETTER_7);
    memory[40..45].copy_from_slice(&LETTER_8);
    memory[45..50].copy_from_slice(&LETTER_9);
    memory[50..55].copy_from_slice(&LETTER_A);
    memory[55..60].copy_from_slice(&LETTER_B);
    memory[60..65].copy_from_slice(&LETTER_C);
    memory[65..70].copy_from_slice(&LETTER_D);
    memory[70..75].copy_from_slice(&LETTER_E);
    memory[75..80].copy_from_slice(&LETTER_F);
}

pub fn get_letter_address(letter: u8) -> u16 {
    5 * letter as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_font() {
        let mut mem = [0_u8; 0x1000];
        load_font(&mut mem);

        assert_eq!(mem[0..5], letters::LETTER_0);
        assert_eq!(mem[75..80], letters::LETTER_F);
    }
}
