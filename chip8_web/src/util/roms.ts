import { QuirkPresets } from 'chip8_wasm';

const roms: Rom[] = [
    { title: 'Test Suite', filename: 'chip8-test-suite.ch8', quirks: QuirkPresets.Chip8 },
    { title: 'WIT SCDS Logo', filename: 'wit-scds.ch8', quirks: QuirkPresets.Chip8 },
    { title: 'Minimal Snake', url: 'https://johnearnest.github.io/chip8Archive/roms/snek.ch8' },
    { title: 'Super Pong', url: 'https://johnearnest.github.io/chip8Archive/roms/superpong.ch8' }
];

export default roms;

export interface Rom {
    title: string,
    filename?: string,
    url?: string,
    quirks?: QuirkPresets
};