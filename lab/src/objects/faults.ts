// FAULTS, planted on purpose: for each kind of thing the bench says it sees, an object that has it. `caught` is the mark that must miss it.
import { type V3, mul } from '../math';
import { F, type Instance, type Piece } from '../scene/world';
import { M } from '../scene/materials';
import { Put, type Level, type Made } from './object';
export interface Fault { name: string; of: string; what: string; caught: string; plant(m: Made): Made }
type Parts = { pieces: Piece[]; instances: Instance[] };
/** what a level puts, changed afterwards */
const after = (l: Level, f: (q: Parts, p: Put) => void): Level => ({ ...l, put(p) { const a = p.w.pieces.length, b = p.w.moving.length, c = p.w.instances.length, live = l.put(p); f({ pieces: [...p.w.pieces.slice(a), ...p.w.moving.slice(b)], instances: p.w.instances.slice(c) }, p); return live; } });
const level = (m: Made, k: number, f: (q: Parts, p: Put) => void): Made => ({ ...m, levels: m.levels.map((l, i) => i === k ? after(l, f) : l) });
const tint = (q: Parts, f: (t: V3) => V3) => { for (const p of q.pieces) p.tint = f(p.tint ?? [1, 1, 1]); for (const i of q.instances) i.tint = f(i.tint); };
const grow = (q: Parts, p: Put, k: number) => { for (const x of q.pieces) { x.at = [0, 1, 2].map(a => p.at[a]! + (x.at[a]! - p.at[a]!) * k) as unknown as V3; x.half = mul(x.half, k); if (x.quad) x.quad = [x.quad[0] / k ** 2, x.quad[1] / k ** 2, x.quad[2] / k ** 2, x.quad[3] / k]; if (x.planes) x.planes = x.planes.map(([n, d]) => [n, d * k]); } };
export const FAULTS: Fault[] = [
  { name: 'dark', of: 'chair', what: 'the second level is a tenth darker', caught: 'swap.bright', plant: m => level(m, 1, q => tint(q, t => mul(t, 0.94))) },
  { name: 'hue', of: 'chair', what: 'the second level is a little redder', caught: 'swap.color', plant: m => level(m, 1, q => tint(q, t => [t[0] * 1.08, t[1] * 0.99, t[2] * 0.90])) },
  { name: 'fat', of: 'chair', what: 'the third level is 8 % larger', caught: 'swap.outline', plant: m => level(m, 2, (q, p) => grow(q, p, 1.08)) },
  { name: 'liar', of: 'chair', what: 'the third level says it is off by 8 mm', caught: 'swap.outline', plant: m => ({ ...m, levels: m.levels.map((l, i) => i === 1 ? { ...l, error: 0.004 } : i === 2 ? { ...l, error: 0.008 } : l) }) },
  { name: 'legless', of: 'chair', what: 'the third level has lost a leg', caught: 'swap.outline', plant: m => level(m, 2, (q, p) => { const i = p.w.pieces.indexOf(q.pieces[0]!); p.w.pieces.splice(i, 1); }) },
  { name: 'no shadow', of: 'chair', what: 'the second level casts none', caught: 'swap.shadow', plant: m => level(m, 1, q => { for (const p of q.pieces) p.flags |= F.NOSHADOW; }) },
  { name: 'shut back', of: 'chair', what: 'the third level\'s back is a board', caught: 'swap.cover', plant: m => level(m, 2, (_, p) => { p.box([0, 0.665, -0.244], [0.16, 0.15, 0.005], M.print!, [0.74, 0.53, 0.36]); }) },
  { name: 'red under', of: 'chair', what: 'the second level is red beneath', caught: 'wide.bounce', plant: m => level(m, 1, (_, p) => { p.box([0, 0.36, 0], [0.20, 0.06, 0.19], M.paint!, [1, 0.08, 0.05]); }) },
  { name: 'dull', of: 'car', what: 'the second level\'s paint is matte', caught: 'swap.shine', plant: m => level(m, 1, q => { for (const p of q.pieces) if (p.mat === M.car || p.mat === M['car glass'] || p.mat === M.chrome || p.mat === M.gloss) p.mat = M.crumb!; }) },
  { name: 'aside', of: 'chair', what: 'the second level stands 3 cm to the side', caught: 'swap.pop', plant: m => level(m, 1, q => { for (const p of q.pieces) p.at = [p.at[0] + 0.03, p.at[1], p.at[2]]; }) },
  { name: 'hairs', of: 'chair', what: 'the finest level has a hundred hairs that glow', caught: 'calm.sparkle', plant: m => level(m, 0, (_, p) => { for (let i = 0; i < 100; i++) { const x = -0.2 + (i % 20) * 0.021, y = 0.5 + Math.floor(i / 20) * 0.07; p.bar([x, y, 0.1], [x + 0.03, y + 0.065, 0.16], 0.0012, 0.0011, M.glow!, [1, 1, 1], { flags: F.GLOW, seed: 250 }); } }) },
  { name: 'ember', of: 'chair', what: 'a part glows and is no lamp', caught: 'look.stops', plant: m => level(m, 0, (_, p) => { p.box([0, 0.25, 0], [0.12, 0.02, 0.12], M.glow!, [1, 0.8, 0.5], { flags: F.GLOW, seed: 900 }); }) },
  { name: 'no gain', of: 'chair', what: 'the second level has as many parts as the first', caught: 'cost.parts', plant: m => ({ ...m, levels: m.levels.map((l, i) => i === 1 ? { ...m.levels[0]!, error: l.error } : l) }) },
  { name: 'heavy', of: 'chair', what: 'the coarsest level is four thousand blocks', caught: 'cost.gain', plant: m => level(m, 2, (_, p) => { for (let i = 0; i < 4000; i++) p.box([-0.19 + (i % 20) * 0.02, 0.03 + Math.floor(i / 400) * 0.04, -0.19 + (Math.floor(i / 20) % 20) * 0.02], [0.009, 0.018, 0.009], M.wood!, [1.1, 1, 0.9]); }) },
  { name: 'lifted', of: 'chair', what: 'it stands 2 cm over the floor', caught: 'rule.foot', plant: m => level(m, 0, q => { for (const p of q.pieces) p.at = [p.at[0], p.at[1] + 0.02, p.at[2]]; }) },
  { name: 'loose part', of: 'chair', what: 'a part hangs in the air', caught: 'rule.floats', plant: m => level(m, 0, (_, p) => { p.ball([0, 1.1, 0], [0.03, 0.03, 0.03], M.brass!, [1, 1, 1]); }) },
  { name: 'one plane', of: 'chair', what: 'two faces share a plane', caught: 'rule.faces', plant: m => level(m, 0, (_, p) => { p.box([0, SEATY, 0], [0.10, 0.0181, 0.10], M.paint!, [1, 1, 1]); p.box([0.03, SEATY, 0.02], [0.10, 0.0181, 0.10], M.paint!, [0, 0, 1]); }) },
  { name: 'orphan', of: 'chair', what: 'a piece is no thing\'s', caught: 'rule.thing', plant: m => level(m, 0, q => { q.pieces[0]!.thing = undefined; }) },
  { name: 'dice', of: 'chair', what: 'it is made by dice that no one seeded', caught: 'rule.same', plant: m => level(m, 0, q => { for (const p of q.pieces) p.at = [p.at[0] + 0.001 * Math.random(), p.at[1], p.at[2]]; }) },
  { name: 'wrong size', of: 'chair', what: 'it says it is half as high', caught: 'rule.size', plant: m => ({ ...m, size: [m.size[0], m.size[1] / 2, m.size[2]] }) },
  { name: 'backward', of: 'chair', what: 'a coarser level says it is off by less', caught: 'rule.order', plant: m => ({ ...m, levels: m.levels.map((l, i) => i === 2 ? { ...l, error: 0.006 } : l) }) },
  { name: 'small print', of: 'shelf', what: 'what is printed is a fifth as large', caught: 'print.kept', plant: m => level(m, 0, q => { for (const p of q.pieces) p.marks = p.marks?.map(k => ({ ...k, size: [k.size[0] / 5, k.size[1] / 5] })); }) },
  { name: 'ghost', of: 'chair', what: 'it is a plate on the floor, painted as the floor is', caught: 'seen.edge', plant: m => ({ ...m, size: [0.5, 0.01, 0.5], levels: m.levels.map((l, i) => ({ ...l, put(p: Put) { for (let x = 0; x < 9 >> i; x++) p.box([(x - 4) * 0.05, 0.005, 0], [0.0248, 0.005, 0.25], M['lamp shade']!, [0.74, 0.75, 0.77]); } })) }) },
  { name: 'lead', of: 'chair', what: 'it says it floats, and is of lead', caught: 'live.floats', plant: m => ({ ...m, moves: { kind: 'loose', dense: 9000, floats: true } }) },
  { name: 'one leg short', of: 'chair', what: 'its weight is outside its legs', caught: 'live.upright', plant: m => level(m, 0, (_, p) => { p.box([0, 0.80, 0.75], [0.18, 0.06, 0.45], M.steel!, [1, 1, 1]); }) },
  { name: 'stiff', of: 'ficus', what: 'the wind does not move it', caught: 'live.sways', plant: m => ({ ...m, levels: m.levels.map(l => after(l, q => { for (const i of q.instances) i.sway = 0; })) }) },
  { name: 'skates', of: 'person', what: 'its feet slide', caught: 'live.feet', plant: m => ({ ...m, levels: m.levels.map(l => ({ ...l, put(p: Put) { const live = l.put(p)!; let k = 0; return { step: (dt: number, t: number, a: never) => { live.step!(dt, t, a); k++; }, feet: () => live.feet!().map(f => ({ ...f, at: [f.at[0] + k * 0.004, f.at[1], f.at[2]] as V3 })) }; } })) as Level[] }) },
  { name: 'frozen', of: 'moped', what: 'the coarser levels do not move', caught: 'live.together', plant: m => ({ ...m, levels: m.levels.map((l, i) => i ? { ...l, put(p: Put) { const live = l.put(p); return live ? { ...live, step: () => {} } : live; } } : l) }) },
  { name: 'trim', of: 'chair', what: 'a small plate is let into a large one, flush', caught: 'rule.faces', plant: m => level(m, 0, (_, p) => { p.box([0, 0.60, 0.5], [0.20, 0.20, 0.20], M.paint!, [1, 1, 1]); p.box([0.133, 0.78, 0.633], [0.02, 0.02, 0.02], M.paint!, [1, 0, 0]); }) },
  { name: 'clear of it', of: 'chair', what: 'a ball hangs off a corner, its box touching', caught: 'rule.floats', plant: m => level(m, 0, (_, p) => { p.ball([0.21 + 0.095, 0.452 + 0.095, 0.208 + 0.095], [0.1, 0.1, 0.1], M.brass!, [1, 1, 1]); }) },
  { name: 'comes apart', of: 'person', what: 'a part of it stays where it was', caught: 'live.held', plant: m => ({ ...m, levels: m.levels.map((l, i) => i ? l : { ...l, put(p: Put) { const a = p.w.moving.length, live = l.put(p)!, one = p.w.moving[a + 24]!, at = one.at; return { ...live, step: (dt: number, t: number, air: never) => { live.step!(dt, t, air); one.at = at; } }; } }) as Level[] }) },
  { name: 'off the rig', of: 'person', what: 'the third level is left behind', caught: 'live.together', plant: m => ({ ...m, levels: m.levels.map((l, i) => i === 2 ? { ...l, put(p: Put) { const live = l.put(p)!; return { ...live, step: () => {} }; } } : l) }) },
];
const SEATY = 0.434;
