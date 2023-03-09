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