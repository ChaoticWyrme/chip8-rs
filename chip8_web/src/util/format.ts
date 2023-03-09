export function formatHex(val: number, padding = 2): string {
    return val.toString(16).padStart(padding, "0").toUpperCase();
}