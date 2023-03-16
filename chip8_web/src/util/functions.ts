import { formatHex } from "./format";

export function generateHexRow(memoryView: DataView, offset: number): string {
    const clampedOffset = Math.max(
        0,
        Math.min(offset, memoryView.byteLength - 1)
    );
    let row: string[] = [];

    const endOffset = Math.min(clampedOffset + 16, memoryView.byteLength);

    for (let i = clampedOffset; i < endOffset; i++) {
        row[i] = formatHex(memoryView.getUint8(offset + i), 2);
    }

    return row.join(" ");
}

export class Debouncer {
    innerFunc: Function;
    lastActivation: number; // from performance.now();
    activationInterval: number; // number of milliseconds between activations of innerFunc
    activateMultiple: boolean = false; // whether to activate multiple times if triggered after multiple activationIntervals

    /**
     * Creates an instance of Debouncer.
     * @param {Function} innerFunc
     * @param {number} [activationInterval]
     * @memberof Debouncer
     */
    constructor(innerFunc: Function, activationInterval?: number) {
        this.innerFunc = innerFunc;
        this.activationInterval = activationInterval;
    }

    /**
     * Sets activationInterval according to the number of activations per second wanted.
     *
     * @param {*} activationsPerSecond Number of activations to have every second
     * @memberof Debouncer
     */
    setPerSecond(activationsPerSecond) {
        // If it's zero, that means disable
        if (activationsPerSecond <= 0) {
            this.activationInterval = 0;
            return;
        }
        this.activationInterval = Math.floor(1000 / activationsPerSecond);
    }

    /**
     * 
     *
     * @returns true if the inner function was called, false otherwise
     * @memberof Debouncer
     */
    activate() {
        if (this.activationInterval <= 0) {
            this.innerFunc();
            return true;
        }

        // If this is the first time this function is called, just run the inner function and update the lastActivation time
        if (typeof this.lastActivation === 'undefined') {
            this.innerFunc();
            this.lastActivation = performance.now();
            return true;
        }

        // get this now, so the timing of the inner function doesn't
        // change the time before the next activation
        let now: number = performance.now();
        // milliseconds since last activation
        let delta: number = now - this.lastActivation;

        // If it has been more than activationInterval milliseconds, run the inner function
        if (delta > this.activationInterval) {
            // If activateMultiple is enabled, check if we should run the function multiple times
            if (this.activateMultiple && delta > this.activationInterval * 2) {
                for (let i = 0; i < delta / this.activationInterval; i++) {
                    this.innerFunc();
                }
            } else {
                this.innerFunc();
            }

            // Update the timestamp of the last activation
            // this should maybe subtract the remainder since the last activation, so that it remains accurate
            this.lastActivation = now;
            return true;
        }

        return false;
    }

}