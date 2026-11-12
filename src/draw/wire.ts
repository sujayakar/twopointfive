// HOW A CALL GOES OVER AND COMES BACK (rust/game_face/src/lib.rs): a call's arguments are written into the module's memory as one run of bytes; what it tells is read where the module left it, with no copy.
import { In } from '../view/read';
import { Writer } from './writer';

/** the exports that carry the bytes */
export interface Carrier { memory: WebAssembly.Memory; face_alloc(n: number): number; face_free(p: number, n: number): void; face_out_ptr(): number; face_out_len(): number; face_panic_ptr(): number; face_panic_len(): number; face_panic_clear(): void }
/** what a host does about a call: before one, and after one that came back; and what is thrown where one did not (a panic's words, and the call) */
export interface Host { before?(call: string): void; after?(): void; trapped(words: string, call: string): Error }

/** bytes the module keeps for arguments: a start's are more, and are given back */
const FIRST = 4096, MOST = 262144;
const DEC = new TextDecoder();
export const nothing = (): void => { /* an answer that is only that it was done */ };
const plain: Host = { trapped: (words, call) => new Error(`the game stopped in ${call}: ${words}`) };

export class Wire<X extends Carrier> extends Writer {
  /** the module's exports, each held to what its host does about a call */
  protected readonly X: X;
  readonly memory: WebAssembly.Memory;
  /** where the arguments lie */
  protected at = 0; private had = 0;

  constructor(exports: WebAssembly.Exports, host: Host = plain) {
    super();
    const raw = exports as unknown as X; this.memory = raw.memory;
    const X: Record<string, unknown> = {};
    for (const [name, f] of Object.entries(exports)) X[name] = typeof f !== 'function' || !name.startsWith('face_') ? f : (a: number, b: number, c: number): number => {
      host.before?.(name);
      try { const n = (f as (a: number, b: number, c: number) => number)(a, b, c); host.after?.(); return n; } catch (e) {
        // (a panic, or the stack run out)
        if (!(e instanceof WebAssembly.RuntimeError) && !(e instanceof RangeError)) throw e;
        this.n = 0; let words = ''; try { words = DEC.decode(this.lying(raw.face_panic_ptr(), raw.face_panic_len())); raw.face_panic_clear(); } catch { /* the trap's own words, then */ }
        throw host.trapped(words || e.message, name); } };
    this.X = X as unknown as X; this.b = new Uint8Array(0);
  }
  /** the module has a call of that name (one of other sources may not) */
  has(call: string): boolean { return typeof (this.X as unknown as Record<string, unknown>)[`face_${call}`] === 'function'; }
  private lying(p: number, n: number): Uint8Array<ArrayBuffer> { return new Uint8Array(this.memory.buffer as ArrayBuffer, p >>> 0, n >>> 0); }

  protected override room(k: number): number {
    const at = this.n, need = at + k;
    if (need > this.had || (at === 0 && this.had > MOST)) {
      const had = this.had, size = Math.max(need, at ? had * 2 : FIRST), was = this.at, to = this.X.face_alloc(size) >>> 0;
      if (at) new Uint8Array(this.memory.buffer).copyWithin(to, was, was + at);
      if (had) this.X.face_free(was, had);
      this.at = to; this.had = size;
    }
    // (a memory that grew has let go of every view of it)
    if (this.b.buffer !== this.memory.buffer || this.b.byteOffset !== this.at || this.b.length !== this.had) { this.b = this.lying(this.at, this.had); this.d = new DataView(this.memory.buffer, this.at, this.had); }
    this.n = need; return at;
  }
  /** how many bytes the arguments are; the next ones are written over them */
  protected sent(): number { const n = this.n; this.n = 0; return n; }
  /** what a call told, as `read` has it; a call that would not says why */
  protected told<T>(done: number, read: (I: In) => T): T {
    const b = this.lying(this.X.face_out_ptr(), this.X.face_out_len());
    if (!done) throw new Error(DEC.decode(b));
    const I = new In(b), r = read(I);
    if (I.p !== b.length) throw new Error(`an answer is ${b.length} bytes and ${I.p} were read`);
    return r;
  }
}
