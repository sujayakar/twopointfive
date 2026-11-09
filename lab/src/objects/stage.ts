// THE STAGE: a grey floor, a white wall behind and to the right, the sky, a lamp over it and a beam from the side. One object, a crowd of it, or nothing.
import { type V3, norm, sub, rotY, apply } from '../math';
import { World, type Piece } from '../scene/world';
import { M } from '../scene/materials';
import type { Building, Mover } from '../scene/building';
import type { Place } from '../scene/place';
import { Solid } from '../scene/solid';
import { Bodies } from '../scene/rigid';
import { Player } from '../scene/player';
import { settle, freeEdges } from '../scene/faces';
import { settleCut } from '../scene/stand';
import { Put, type Live, type Made } from './object';
export interface Stand { /** which level stands there (−1: none) */ level: number; /** so many of them, on a grid */ crowd?: number; yaw?: number; /** without what is printed on it */ plain?: boolean; /** the wall behind it (there, if not said) */ wall?: boolean }
export const LAMP = 0, BEAM = 1;
export interface Stage { building: Building; own: Set<Piece>; lives: Live[]; /** where the crowd stands: half its width */ wide: number }
export function makeStage(m: Made, s: Stand): Stage {
  const w = new World(), [sx, sy, sz] = m.size, big = Math.max(sx, sy, sz), lives: Live[] = [], hung: Mover[] = [], crowd = s.crowd ?? 0;
  w.box([-70, -1.2, -70], [70, 0, 70], M['lamp shade']!, { tint: [0.74, 0.75, 0.77] });
  if (!crowd && s.wall !== false) w.box([-sx / 2 - 0.35 * sy - 0.1, 0, -sz / 2 - 0.25 - 0.2 * big - 0.2], [sx / 2 + 1.1 * big, 1.25 * sy + 0.3, -sz / 2 - 0.25 - 0.2 * big], M['lamp shade']!, { tint: [1.02, 1.02, 1.02] });
  w.lamp({ kind: 'rect', at: [0.35 * big, sy + 1.1, 0.5 * big], u: [0.16, 0, 0], v: [0, 0, 0.16], n: [0, -1, 0], radiance: [190, 150, 100], name: 'bench', on: 1 });
  { const at: V3 = [2.2 + sx / 2, 1.35, 2.4 + sz / 2]; w.lamp({ kind: 'spot', at, u: [0.025, 0, 0], v: norm(sub([0, sy * 0.45, 0], at)), radiance: [38000, 35800, 31000], cos: 0.915, name: 'beam', on: 1 }); }
  const own = new Set<Piece>(), step = 1.35 * Math.max(sx, sz) + 0.25, n = Math.ceil(Math.sqrt(Math.max(1, crowd)));
  if (s.level >= 0) for (let i = 0; i < Math.max(1, crowd); i++) {
    const at: V3 = crowd ? [((i % n) - (n - 1) / 2) * step, 0, (Math.floor(i / n) - (n - 1) / 2) * step] : [0, 0, 0], a = w.pieces.length, b = w.moving.length, live = m.levels[s.level]!.put(new Put(w, m.name, at, (s.yaw ?? 0) + (crowd ? i * 2.39996 : 0), 1 + i % 7));
    for (const p of [...w.pieces.slice(a), ...w.moving.slice(b)]) { own.add(p); if (s.plain) p.marks = undefined; } if (live) { lives.push(live); if (live.step) hung.push({ step: (dt, t, air) => live.step!(dt, t, air) }); }
  }
  const solid = new Solid(w, []), bodies = new Bodies(solid, [], []), player = new Player(w, solid, [30, 0.05, 30], []);
  settleCut(w, PLACE); settle(w.pieces); solid.settle(); for (const [p, mask] of freeEdges(w.pieces)) p.flags |= mask << 16; for (const p of w.moving) if (!p.quad && !p.planes?.length && !p.bend) p.flags |= 0xfff << 16;
  return { own, lives, wide: n * step / 2, building: { world: w, doors: [], people: [], levels: [{ name: 'stage', cut: 1e9, look: [0, 0, 0] }], pool: [0, 0, -1, -1], poolTop: -99, poolDeep: 0, place: PLACE, hung, bodies, solid, player } };
}
const PLACE: Place = { name: 'bench', title: 'BENCH', rooms: [[-1, -60, -1, 1, -58, 1]], openFloor: -99, sunAt: [0, 2, 0], sunSize: 16, start: { hour: 12.6, look: [0, 0, 0], yaw: 0, pitch: 0.6, dist: 10, level: 0, wet: 0, rain: 0, steam: 0 }, floors: [0], pan: [10, 10], legs: [], groups: [], quiet: { hour: 23, groups: {} }, fires: [], smokes: [], judged: [] };
void rotY; void apply;
