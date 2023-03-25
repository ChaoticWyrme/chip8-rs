import { writable } from 'svelte/store';

export const cyclesPerFrame = writable(20);
export const frameTime = writable(1000 / 60);

// TODO: Create FPS tracker here