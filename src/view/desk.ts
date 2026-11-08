// THE DESK, as a page has it (rust/game_run/src/desk.rs): what the cards before a game show and press. The page holds no rule of a card and no card: it keeps
// the text a press says is kept, and asks again.
import { Asks } from '../draw/asks';
import type { Commands } from '../draw/commands';
import type { Pressed } from './read';

/** the module's calls the desk is asked by */
export interface DeskFace { face_alloc(n: number): number; face_free(p: number, n: number): void; face_out_ptr(): number; face_out_len(): number; face_desk_do(p: number, n: number): number; face_desk_ask(p: number, n: number): number }
export const KIT_KEPT = 'twopointsix.kit';

export class Desk extends Asks {
  /** `keeps`: told the cards' kept text after a press that was taken */
  constructor(private readonly X: DeskFace, private readonly memory: WebAssembly.Memory, private readonly keeps: (kept: string) => void) {
    super(asked => { if (!this.handed(asked, X.face_desk_ask)) throw new Error(`the desk: ${this.words()}`); return this.out(); });
  }
  private handed(b: Uint8Array, f: (p: number, n: number) => number): boolean {
    const p = this.X.face_alloc(b.length) >>> 0; new Uint8Array(this.memory.buffer, p, b.length).set(b);
    try { return f(p, b.length) !== 0; } finally { this.X.face_free(p, b.length); }
  }
  private out(): Uint8Array { return new Uint8Array(this.memory.buffer, this.X.face_out_ptr() >>> 0, this.X.face_out_len() >>> 0).slice(); }
  private words(): string { return new TextDecoder().decode(this.out()); }
  /** what this machine keeps and what the address says, done as a start is done it */
  do(c: Commands): void { if (c.length && !this.handed(c.take(), this.X.face_desk_do)) throw new Error(`the desk: ${this.words()}`); }
  protected override told(p: Pressed): Pressed { if (p.ok) this.keeps(p.kept); return p; }
}

let standing: Desk | null = null;
/** the page's one desk */
export function desk(): Desk { if (!standing) throw new Error('the desk is asked before the page has set it'); return standing; }
export function setDesk(d: Desk): void { standing = d; }
