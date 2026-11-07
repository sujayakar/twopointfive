// THE RECORD OF A RUN: every call into the Rust game, as bytes, in order, so a run can be played again with no TypeScript game and no tool standing beside it. What a tool did between ticks is in it as what reached the game: the scan's writes, the crossings, the host's half, a caller's die. A host that writes the game's memory itself (a tool's write to the block) says so with `mem`. Nothing else can change the game.
//   TPS_GAME_RECORD=<file> tools/blitz/rust.sh rust bun run <any tool>       a host that loads the face records to that file
//   bun run tools/blitz/replay.ts <file> [--wasm FILE]                       plays it and holds the end to the end recorded (the die's count, every part's hash, the whole export's); exit 0 equal · 1 parted · 2 no such file
//   RECORD_BREAK=1 in front of the replay drops one step in fifty: it must part
// A function that only reads (the pointers and lengths of what was answered, a part's hash) is not written. Each entry: its name's number, then each argument as a number or as the bytes a pointer and a length name.
import { openSync, writeSync, closeSync, readFileSync } from 'node:fs';

/** which arguments are a pointer and its length, by the face's own signatures */
export const POINTERS: Record<string, [number, number][]> = { face_new: [[0, 1], [2, 3]], face_new_at: [[0, 1]], face_keep_given: [[0, 1], [2, 3]], face_import_into: [[1, 2]], face_lesson: [[1, 2]], face_host: [[1, 2]], face_host_half: [[1, 2]], face_hand: [[2, 3], [4, 5]], face_write: [[1, 2]], face_hostwrite: [[2, 3]], face_give: [[2, 3]], face_call: [[3, 4]], face_ask: [[4, 5]], face_checkpoint: [[1, 2]], face_probe_results: [[1, 2]], face_saves_in: [[1, 2]], face_loaded: [[1, 2]], face_part: [[0, 1]] };
/** nothing a call of these does reaches the game */
export const READERS = /^face_(alloc|free|out_ptr|out_len|panic_ptr|panic_len|panic_clear|mem|parts|part_name|part_addr|part_len|part_hashes|die_n|time|rows|out_n|slot|kept_n|kept_state|kept_block|export|reserve|puppet_warm|draw|draw_layout|draw_look|frame)$/;
export const ROLL = 0xfffe, END = 0xffff, MEM = 0xfffc;
class Writer { private b = Buffer.alloc(1 << 20); private n = 0; constructor(private fd: number) {}
  private room(k: number) { if (this.n + k > this.b.length) { this.flush(); if (k > this.b.length) this.b = Buffer.alloc(k * 2); } }
  flush() { if (this.n) { writeSync(this.fd, this.b, 0, this.n); this.n = 0; } }
  u16(v: number) { this.room(2); this.b.writeUInt16LE(v, this.n); this.n += 2; } u32(v: number) { this.room(4); this.b.writeUInt32LE(v, this.n); this.n += 4; } f64(v: number) { this.room(8); this.b.writeDoubleLE(v, this.n); this.n += 8; }
  bytes(u: Uint8Array) { this.u32(u.length); this.room(u.length); this.b.set(u, this.n); this.n += u.length; }
  str(s: string) { this.bytes(new TextEncoder().encode(s)); } close() { this.flush(); closeSync(this.fd); } }
export class Reader { p = 0; dv: DataView; constructor(readonly b: Uint8Array) { this.dv = new DataView(b.buffer, b.byteOffset, b.byteLength); } get more() { return this.p < this.b.length; }
  u16() { const v = this.dv.getUint16(this.p, true); this.p += 2; return v; } u32() { const v = this.dv.getUint32(this.p, true); this.p += 4; return v; } f64() { const v = this.dv.getFloat64(this.p, true); this.p += 8; return v; }
  bytes() { const L = this.u32(), u = this.b.subarray(this.p, this.p + L); this.p += L; return u; } str() { return new TextDecoder().decode(this.bytes()); } }
export const readRecord = (file: string) => new Reader(new Uint8Array(readFileSync(file)));

/** the end of a run, as the record holds it and as a replay reads it */
export function endOf(X: any, h: number, slot: number): { die: number; hashes: number[] } { const n = X.face_parts() >>> 0, p = X.face_part_hashes(slot) >>> 0, H = Array.from(new Uint32Array(X.memory.buffer, p, n)); if (!h) return { die: 0, hashes: H };
  // (…and every other field of the game: the whole export's hash, the block's parts being only some of it)
  X.face_export(h); const q = X.face_out_ptr() >>> 0, m = X.face_out_len() >>> 0, all = Bun.hash(new Uint8Array(X.memory.buffer, q, m)); H.push(Number(BigInt(all) & 0xffffffffn), Number(BigInt(all) >> 32n)); return { die: X.face_die_n(h), hashes: H }; }

/** the face's exports with every call written down. `lastHandle` is told each game made, so that the end can be read */
export function recorded(X: any, file: string): { X: any; roll(v: number): void; mem(at: number, b: Uint8Array): void } {
  const W = new Writer(openSync(file, 'w')), names = new Map<string, number>(); let h = 0, slot = 0, ended = false; const u8 = () => new Uint8Array(X.memory.buffer);
  const id = (n: string) => { let i = names.get(n); if (i === undefined) { i = names.size; names.set(n, i); W.u16(0xfffd); W.str(n); } return i; };
  const out: any = {}; for (const k of Object.keys(X)) { const f = X[k]; if (typeof f !== 'function' || !k.startsWith('face_') || READERS.test(k)) { out[k] = f; continue; }
    const P = POINTERS[k] ?? []; out[k] = (...a: number[]) => { const i = id(k); W.u16(i); W.u16(a.length); for (let j = 0; j < a.length; j++) { const pair = P.find(x => x[0] === j); if (pair) { W.u16(1); W.bytes(u8().subarray(a[j] >>> 0, (a[j] >>> 0) + (a[pair[1]] >>> 0))); j++; W.u16(2); continue; } W.u16(0); W.f64(a[j]); }
      const r = f(...a); if ((k === 'face_new' || k === 'face_new_at') && r) { h = r; slot = X.face_slot(r) >>> 0; } return r; }; }
  process.on('exit', () => { if (ended) return; ended = true; try { const e = endOf(X, h, slot); W.u16(END); W.f64(e.die); W.u32(e.hashes.length); for (const v of e.hashes) W.u32(v); } catch { W.u16(END); W.f64(-1); W.u32(0); } W.close(); });
  return { X: out, roll: v => { W.u16(ROLL); W.f64(v); }, mem: (at, b) => { W.u16(MEM); W.f64(at); W.bytes(b); } }; }
