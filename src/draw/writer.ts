// BYTES BEING WRITTEN for the module: numbers little end first, a yes or no as a byte, a text, bytes and a list as how many and then each (rust/game_start/src/host/field.rs reads them). What is written stands
// until it is taken.
const enc = new TextEncoder();

export class Writer {
  protected b = new Uint8Array(256); protected d = new DataView(this.b.buffer); protected n = 0;
  get length(): number { return this.n; }
  /** room for `k` bytes more: where they begin (the buffer may be another after it, so it is asked for before the buffer is) */
  protected room(k: number): number { const at = this.n; this.n += k; if (this.n > this.b.length) { const b = new Uint8Array(Math.max(this.n, this.b.length * 2)); b.set(this.b.subarray(0, at)); this.b = b; this.d = new DataView(b.buffer); } return at; }
  protected yes(v: boolean): void { const at = this.room(1); this.b[at] = v ? 1 : 0; }
  protected u32(v: number): void { const at = this.room(4); this.d.setUint32(at, v >>> 0, true); }
  protected i32(v: number): void { const at = this.room(4); this.d.setInt32(at, v | 0, true); }
  protected f64(v: number): void { const at = this.room(8); this.d.setFloat64(at, v, true); }
  protected three(v: ArrayLike<number>): void { const at = this.room(24); for (let i = 0; i < 3; i++) this.d.setFloat64(at + i * 8, v[i], true); }
  protected doubles(v: ArrayLike<number>): void { this.u32(v.length); const at = this.room(v.length * 8); for (let i = 0; i < v.length; i++) this.d.setFloat64(at + i * 8, v[i], true); }
  protected text(s: string): void { const most = s.length * 3, at = this.room(4 + most), n = enc.encodeInto(s, this.b.subarray(at + 4, at + 4 + most)).written; this.d.setUint32(at, n, true); this.n = at + 4 + n; }
  protected bytes(b: Uint8Array): void { this.u32(b.length); this.put(b); }
  /** a row, as it lies */
  protected laid(row: Float64Array): void { this.put(new Uint8Array(row.buffer, row.byteOffset, row.byteLength)); }
  /** what was written once before, again */
  put(b: Uint8Array): void { const at = this.room(b.length); this.b.set(b, at); }
  /** what was written, which is then nothing (the bytes stand until the next write) */
  take(): Uint8Array { const r = this.b.subarray(0, this.n); this.n = 0; return r; }
}
