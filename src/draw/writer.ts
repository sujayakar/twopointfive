// BYTES BEING WRITTEN for the module: numbers little end first, a text as its length and its bytes. What is written stands until it is taken.
const enc = new TextEncoder();

export class Writer {
  private b = new Uint8Array(256); private d = new DataView(this.b.buffer); private n = 0;
  get length(): number { return this.n; }
  /** room for `k` bytes more: where they begin (the buffer may be another after it, so it is asked for before the buffer is) */
  private room(k: number): number { const at = this.n; this.n += k; if (this.n > this.b.length) { const b = new Uint8Array(Math.max(this.n, this.b.length * 2)); b.set(this.b.subarray(0, at)); this.b = b; this.d = new DataView(b.buffer); } return at; }
  protected u32(v: number): void { const at = this.room(4); this.d.setUint32(at, v >>> 0, true); }
  protected i32(v: number): void { const at = this.room(4); this.d.setInt32(at, v | 0, true); }
  protected f64(v: number): void { const at = this.room(8); this.d.setFloat64(at, v, true); }
  protected text(s: string): void { const t = enc.encode(s); this.u32(t.length); const at = this.room(t.length); this.b.set(t, at); }
  /** what was written, which is then nothing (the bytes stand until the next write) */
  take(): Uint8Array { const r = this.b.subarray(0, this.n); this.n = 0; return r; }
}
