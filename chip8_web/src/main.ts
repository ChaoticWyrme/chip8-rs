import './app.css'
import App from './App.svelte'
import init from 'chip8_wasm'

const main = async () => {
  const bindings = await init();
  bindings.set_panic_hook();
  const app = new App({
    target: document.getElementById('app'),
    props: {
      'bindings': bindings,
    }

  });
}

main();