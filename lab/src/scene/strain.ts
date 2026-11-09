// FOR ASKING WHAT COSTS WHAT: the same scene with more or fewer of one thing.
import { type V3, rng, rotY, add } from '../math';
import type { Building } from './building';
import { F, type Piece } from './world';
import { M } from './materials';
export interface Strain { /** stocked things, times */ stock?: number; /** lamps kept (the torches stay) */ lamps?: number; /** leaf clusters, times */ leaves?: number; /** small things that move, added */ moving?: number; /** panes, added in front of the store */ panes?: number; /** plain boxes, added on the ground about the middle */ boxes?: number }
export function strain(b: Building, s: Strain): Building {
  const w = b.world, q = rng(4242);
  if (s.stock !== undefined) { const ids = new Map<string, Piece[]>(); for (const p of w.pieces) if (p.rig?.startsWith('stock:')) { let l = ids.get(p.rig); if (!l) ids.set(p.rig, l = []); l.push(p); } const all = [...ids.values()], gone = new Set<Piece>(), more: Piece[] = []; if (s.stock < 1) all.forEach((l, i) => { if ((i * 0.61803399) % 1 >= s.stock!) for (const p of l) gone.add(p); }); for (let k = 1; k < s.stock; k++) for (const l of all) { const d: V3 = [(q() - 0.5) * 0.05, 0.004 * k, (q() - 0.5) * 0.05]; for (const p of l) more.push({ ...p, at: add(p.at, d) }); } w.pieces = w.pieces.filter(p => !gone.has(p)).concat(more); }
  if (s.lamps !== undefined) { const fixed = w.lamps.map((l, i) => [l, i] as const).filter(([l]) => l.kind !== 'spot' && l.name !== 'fire'); fixed.forEach(([l], k) => { if ((k * 0.61803399) % 1 >= s.lamps! / fixed.length) { l.on = 0; l.radiance = [0, 0, 0]; } }); }
  if (s.leaves !== undefined) { const base = w.instances.slice(); w.instances = s.leaves < 1 ? base.filter((_, i) => (i * 0.61803399) % 1 < s.leaves!) : base.slice(); for (let k = 1; k < s.leaves; k++) for (const i of base) w.instances.push({ ...i, at: add(i.at, [(q() - 0.5) * i.scale, (q() - 0.5) * i.scale * 0.6, (q() - 0.5) * i.scale]), phase: q() * 6.28 }); }
  for (let i = 0; i < (s.moving ?? 0); i++) w.add({ at: [-5 + 10 * q(), 0.2 + 2.0 * q(), -9 + 10 * q()], half: [0.05, 0.05, 0.05], rot: rotY(q() * 6), mat: M.print!, tint: [q(), q(), q()], flags: F.MOVES | F.CUT });
  for (let i = 0; i < (s.boxes ?? 0); i++) w.add({ at: [-6 + 12 * q(), 0.1 + 2.2 * q() * q(), -9.5 + 14 * q()], half: [0.03 + 0.05 * q(), 0.03 + 0.05 * q(), 0.03 + 0.05 * q()], rot: rotY(q() * 6), mat: M.print!, tint: [q(), q(), q()], flags: F.CUT });
  for (let i = 0; i < (s.panes ?? 0); i++) w.add({ at: [-5 + 10 * q(), 1.2, -8 + 8 * q()], half: [0.4, 0.6, 0.004], rot: rotY(q() * 6), mat: M.glass!, tint: [0.97, 0.98, 0.98], flags: F.GLASS | F.CUT });
  return b;
}
