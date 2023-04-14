import { writable } from 'svelte/store';

export const cyclesPerFrame = writable(20);
export const frameTime = writable(1000 / 60);

export const running = writable(true);

// TODO: Create FPS tracker here