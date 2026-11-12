// WHAT THE LAST TICK LEFT TO BE DRAWN, read where Rust left it (rust/game_draw). `sections.ts` (written by the program `half`, rust/game_start/src/bin/half) has every offset as a constant;
// this says where each section lies. Nothing here knows the game, and reading a field makes no object:
//   const f = D.f, o = D.at[Section.fog]; … f[o + Fog.edge] …
import type { Face, GameNo } from './face';
import { LAYOUT, Section } from './sections';

/** why what crosses is not, to this module, what this page's half was written from, or '' */
export function drawRefused(face: Face): string {
  if (!face.has('stamp')) return 'the module is of another tree: it answers no stamp';
  const has = face.stamp();
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

  constructor(private face: Face, private want: number) {}

  /** A TICK HAS RUN: its sections are made, and lie here until the next */
  take(h: GameNo): void { this.lay(this.face.draw(h, this.want)); }
  /** …one after the first of several a host steps before it draws a frame: what moves is blended over them all */
  takeOver(h: GameNo): void { this.lay(this.face.drawOver(h, this.want)); }
  /** A FRAME IS DRAWN, `a` of the way from the tick before to the last, looking at `focus`: the rows that move stand there until the next call */
  frame(h: GameNo, a: number, focus: ArrayLike<number>): void { this.lay(this.face.frame(h, a, focus)); }
  private lay(table: number): void {
    this.fresh();
    for (let i = 0, t = table >>> 2; i < Section.count; i++, t += 2) { this.at[i] = this.u[t] >>> 2; this.n[i] = this.u[t + 1]; }
  }
  /** a section's rows handed to the GPU as they lie: `count` rows from `from` (or every one), to the same place in the buffer */
  send(queue: GPUQueue, buffer: GPUBuffer, section: number, row: number, from = 0, count = this.n[section] - from, to = from * row * 4): void {
    if (count > 0) queue.writeBuffer(buffer, to, this.buffer as ArrayBuffer, (this.at[section] + from * row) * 4, count * row * 4);
  }
  /** before the sections are read again: where the module's memory has grown since, the views over it are made again (the rows themselves do not move) */
  fresh(): void { if (this.buffer !== this.face.memory.buffer) { this.buffer = this.face.memory.buffer; this.f = new Float32Array(this.buffer); this.u = new Uint32Array(this.buffer); } }
}
