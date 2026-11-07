// WHAT THE LAST TICK LEFT TO BE DRAWN, read where Rust left it (rust/game_draw). `sections.ts` (made by tools/draw/gen.ts) has every offset as a constant;
// this says where each section lies. Nothing here knows the game, and reading a field makes no object:
//   const f = D.f, o = D.at[Section.fog]; … f[o + Fog.edge] …
import { LAYOUT, Section } from './sections';

/** the module's calls this needs */
export interface DrawFace { face_draw(h: number, want: number): number; face_draw_layout(): number }

/** every section */
export const ALL = 2 ** Section.count - 1;

/** why this module's sections are not the ones this page's offsets were made from, or '' */
export function drawRefused(X: DrawFace): string {
  if (typeof X.face_draw !== 'function') return 'the module draws nothing: it was built from another tree';
  const has = X.face_draw_layout() >>> 0;
  return has === LAYOUT ? '' : `the module's draw sections are of another stamp (${has}) than this page's (${LAYOUT})`;
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
  private lay(table: number): void {
    this.fresh();
    for (let i = 0, t = table >>> 2; i < Section.count; i++, t += 2) { this.at[i] = this.u[t] >>> 2; this.n[i] = this.u[t + 1]; }
  }
  /** before the sections are read again: where the module's memory has grown since, the views over it are made again (the rows themselves do not move) */
  fresh(): void { if (this.buffer !== this.memory.buffer) { this.buffer = this.memory.buffer; this.f = new Float32Array(this.buffer); this.u = new Uint32Array(this.buffer); } }
}
