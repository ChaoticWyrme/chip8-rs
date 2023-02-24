import { get_display_pointer, tick, render_text } from 'chip8_wasm';
import type { InitOutput } from 'chip8_wasm/chip8_wasm'

let bindings: InitOutput = globalThis.bindings;

function init() {
    bindings.init_emu();
}

let canvas = document.querySelector('#main-canvas');
const CANVAS_WIDTH = 1000

function mainLoop() {
    canvas.textContent = render_text();
    tick();

    requestAnimationFrame(mainLoop);
}

mainLoop()

export { }