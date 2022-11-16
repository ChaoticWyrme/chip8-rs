use chip8_core::*;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut system = Chip8::new();
    // system.execute();

    let rom_path: PathBuf = std::env::args()
        .nth(1)
        .expect("Rom path should be first arg")
        .into();
    // let program = fs::read("./roms/test_opcode.ch8").unwrap();
    let program = fs::read(rom_path).unwrap();
    system.memory[0x200..0x200 + program.len()].copy_from_slice(program.as_slice());
    system.pc = 0x200;
    system.run().unwrap();
}
