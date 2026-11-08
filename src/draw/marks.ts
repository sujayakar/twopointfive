// THE MARKS ON THE WORLD, handed to the GPU where Rust left them (rust/game_draw marks.rs writes the buffer render/shaders/decals.wgsl reads): the renderer's table, sent when a word of it moved. The
// list is the game's: a load, a rollback or another seat is another buffer, and nothing here keeps a mark.
import { Decals } from '../render/decals';
import type { Drawn } from './drawn';
import { DecalHead, Section, Word } from './sections';

export class DrawDecals extends Decals {
  private owed = true;
  constructor(device: GPUDevice, private D: Drawn) { super(device); this.queue = device.queue; }
  private queue: GPUQueue;
  override get count(): number { const D = this.D; return D.n[Section.decalHead] ? D.u[D.at[Section.decalHead] + DecalHead.count] : 0; }
  /** a frame is drawn: `game.frame` has left the rows where this frame has them */
  override frame(): void {
    const D = this.D; if (!D.n[Section.decalHead]) return;
    if (D.u[D.at[Section.decalHead] + DecalHead.moved]) this.owed = true;
    const o = D.at[Section.decalHead], s = this.stats; s.count = D.u[o + DecalHead.drawn]; s.listLen = D.u[o + DecalHead.listed]; s.dropped = D.u[o + DecalHead.dropped];
    if (this.owed && this.enabled) { D.send(this.queue, this.buf, Section.decals, Word.row); this.owed = false; s.uploads++; }
  }
}
