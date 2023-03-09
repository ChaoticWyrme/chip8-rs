import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import wasmPack from 'vite-plugin-wasm-pack'
import { VitePluginFonts } from 'vite-plugin-fonts';

// https://vitejs.dev/config/
export default defineConfig({
  base: './',
  plugins: [svelte(), wasmPack('../chip8_wasm'), VitePluginFonts({
    google: {
      families: ['Roboto Mono']
    }
  })],
})
