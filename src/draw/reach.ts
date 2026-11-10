// THE LIGHTS' REACH, kept by the picture (render/reachKeep.ts): told where the level stands, where a wall has changed, and how big a sun is.
import type { Engine } from '../engine';
import { buildInteriorCells, type ReachDoor } from '../render/reach';
import type { RtLight } from '../render/lights';
import type { Drawn } from './drawn';
import { Coming, Doorway, Section } from './sections';
import type { DrawWorld } from './world';

/** frames a blast's walls wait before they are read (the blast's own frame is the long one) */
const HOLD = 1;

export function doorways(D: Drawn): ReachDoor[] {
  const out: ReachDoor[] = [];
  for (let i = 0; i < D.n[Section.doorways]; i++) { const o = D.at[Section.doorways] + i * Doorway.row; out.push({ hinge: [D.f[o + Doorway.hinge], D.f[o + Doorway.hinge + 1]], closedDir: D.f[o + Doorway.closedDir], width: D.f[o + Doorway.width] }); }
  return out;
}

export class DrawReach {
  private suns = new Map<RtLight, number>();
  constructor(private E: Pick<Engine, 'reach' | 'lights'>, private W: DrawWorld, private D: Drawn, doors: ReachDoor[]) { E.reach.boot(doors, buildInteriorCells(W.statics)); W.walls.length = 0; }
  /** before a frame is drawn, its statics and lights as they stand */
  from(): void {
    const K = this.E.reach, W = this.W.walls;
    for (const r of W) K.changed(r, HOLD);
    W.length = 0;
    for (const l of this.E.lights.lights) if (l.kind === 2 && this.suns.get(l) !== l.radius) { this.suns.set(l, l.radius); if (l.radius > 0) K.sun(l, l.radius); else K.dir(l); }
    // (the one that is next: after those that stand, so the one that has come takes the mask that was made for it)
    const o = this.D.at[Section.coming]; if (this.D.n[Section.coming]) { const f = this.D.f; K.coming([f[o + Coming.dir], f[o + Coming.dir + 1], f[o + Coming.dir + 2]], f[o + Coming.radius]); } else K.coming(null);
  }
}
