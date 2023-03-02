// Cribbed some ideas from https://github.com/YuzuJS/setImmediate/
// a setImmediate polyfill, but it didn't work with the module system

let registry = {};

export function registerImmediate(callback): string {
    let tag = '$setImmediate$' + Math.random() + '$';

    let handler = event => {
        if (event.source === window && typeof event.data === 'string' && event.data.startsWith(tag)) {
            callback();
        }
    };
    globalThis.addEventListener('message', handler);

    registry[tag] = handler;

    return tag;
}

export function clearImmediate(tag) {
    let handler = registry[tag];
    globalThis.removeEventListener('message', handler);
}

export function runImmediate(tag) {
    globalThis.postMessage(tag);
}