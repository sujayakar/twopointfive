// WHAT THE LAST TICK LEFT TO BE DRAWN, read where Rust left it (rust/game_draw). `sections.ts` (written by the program `half`, rust/game_start/src/bin/half) has every offset as a constant;
// this says where each section lies. Nothing here knows the game, and reading a field makes no object:
//   const f = D.f, o = D.at[Section.fog]; … f[o + Fog.edge] …
import { LAYOUT, Section } from './sections';

/** the module's calls this needs */
export interface DrawFace { face_draw(h: number, want: number): number; face_draw_over(h: number, want: number): number; face_frame(h: number, a: number, x: number, y: number, z: number): number; face_draw_layout(): number }

/** why what crosses is not, to this module, what this page's half was written from, or '' */
export function drawRefused(X: DrawFace): string {
  if (typeof X.face_draw !== 'function') return 'the module draws nothing: it was built from another tree';
  const has = X.face_draw_layout() >>> 0;
  return has === LAYOUT ? '' : `what the module draws, tells and takes is of another stamp (${has}) than this page's (${LAYOUT})`;
}

export class Drawn {
  /** the module's whole memory, as floats and as words */
  f = new Float32Array(0);
  u = new Uint32Array(0);
  /** each section's first row, in words, and how many rows it has */
  readonly at = new Uint32Array(Section.count);
  readonly n = new Uint32Array(Section.count);
  private buffer: ArrayBufferLike | null = null;

  constructor(private X: DrawFace, private memory: WebAssembly.Memory, private want: number) {}

  /** A TICK HAS RUN: its sections are made, and lie here until the next */
  take(h: number): void { this.lay(this.X.face_draw(h, this.want) >>> 0); }
  /** …one after the first of several a host steps before it draws a frame: what moves is blended over them all */
  takeOver(h: number): void { this.lay(this.X.face_draw_over(h, this.want) >>> 0); }
  /** A FRAME IS DRAWN, `a` of the way from the tick before to the last, looking at `focus`: the rows that move stand there until the next call */
  frame(h: number, a: number, focus: ArrayLike<number>): void { this.lay(this.X.face_frame(h, a, focus[0], focus[1], focus[2]) >>> 0); }
  private lay(table: number): void {
    this.fresh();
    for (let i = 0, t = table >>> 2; i < Section.count; i++, t += 2) { this.at[i] = this.u[t] >>> 2; this.n[i] = this.u[t + 1]; }
  }
  /** a section's rows handed to the GPU as they lie: `count` rows from `from` (or every one), to the same place in the buffer */
  send(queue: GPUQueue, buffer: GPUBuffer, section: number, row: number, from = 0, count = this.n[section] - from, to = from * row * 4): void {
    if (count > 0) queue.writeBuffer(buffer, to, this.buffer as ArrayBuffer, (this.at[section] + from * row) * 4, count * row * 4);
  }
  /** before the sections are read again: where the module's memory has grown since, the views over it are made again (the rows themselves do not move) */
  fresh(): void { if (this.buffer !== this.memory.buffer) { this.buffer = this.memory.buffer; this.f = new Float32Array(this.buffer); this.u = new Uint32Array(this.buffer); } }
}
