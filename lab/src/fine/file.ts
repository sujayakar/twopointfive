// SHAPES IN A FILE: a line of JSON (names, levels, how many words each), then their words, one after another. Nothing in a shape points outside it, so reading is cutting the file up.
import type { Packed } from './template';
export function bake(shapes: Packed[]): Uint8Array<ArrayBuffer> {
  const head = new TextEncoder().encode(JSON.stringify(shapes.map(s => ({ name: s.name, kind: s.kind, levels: s.levels, words: s.data.length })))), at = 8 + ((head.length + 3) & ~3), out = new Uint8Array(at + shapes.reduce((a, s) => a + s.data.byteLength, 0)), u = new Uint32Array(out.buffer, 0, 2); u[0] = 0x454e4946; u[1] = head.length; out.set(head, 8);
  let o = at; for (const s of shapes) { out.set(new Uint8Array(s.data.buffer, s.data.byteOffset, s.data.byteLength), o); o += s.data.byteLength; } return out;
}
export function unbake(file: ArrayBuffer): Packed[] {
  const u = new Uint32Array(file, 0, 2); if (u[0] !== 0x454e4946) throw new Error('not a file of shapes'); const head = JSON.parse(new TextDecoder().decode(new Uint8Array(file, 8, u[1]!))) as { name: string; kind: number; levels: Packed['levels']; words: number }[]; let o = 8 + ((u[1]! + 3) & ~3);
  return head.map(h => { const data = new Uint32Array(file, o, h.words); o += h.words * 4; return { name: h.name, kind: h.kind, levels: h.levels, data }; });
}
export const squeeze = async (b: Uint8Array<ArrayBuffer>) => new Uint8Array(await new Response(new Blob([b]).stream().pipeThrough(new CompressionStream('gzip'))).arrayBuffer());
export const unsqueeze = async (b: Uint8Array<ArrayBuffer>) => new Response(new Blob([b]).stream().pipeThrough(new DecompressionStream('gzip'))).arrayBuffer();
