import './app.css'
import Chip8App from './Chip8.svelte';
import init from 'chip8_wasm'
import { Chip8 } from 'chip8_wasm';

const main = async () => {
  const bindings = await init();
  bindings.set_panic_hook();

  let context = new Map();
  context.set('memory', bindings.memory);
  context.set('emu', new Chip8());

  const app = new Chip8App({
    target: document.getElementById('app'),
    context,
  });
}

main();