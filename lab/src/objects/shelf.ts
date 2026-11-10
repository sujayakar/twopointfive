// A SHELF OF PACKETS: clutter. Near, every packet; then what can be seen of them, each one piece; then one box a run; then one a tier, of the mean color.
import { type V3, add, mul, ID } from '../math';
import { boundsOf, type Piece } from '../scene/world';
import { M } from '../scene/materials';
import { Shelver, type Kind } from '../scene/stock';
import { gap } from '../scene/stand';
import { apart, type Made, type ObjectKind, type Put } from './object';
const W = 1.2, D = 0.44, HIGH = 1.62, TIERS: [number, Kind[], number][] = [[0.16, ['jug', 'bottle', 'six'], 0.30], [0.50, ['carton', 'tall', 'bottle'], 0.28], [0.82, ['can', 'jar', 'tube'], 0.24], [1.10, ['cup', 'bowl', 'yogurt'], 0.22], [1.36, ['bag', 'pouch', 'box'], 0.22]];
const FRAME: V3 = [0.92, 0.91, 0.88];
/** every packet: its pieces, the largest first */
const packets = (seed: number): { tier: number; pieces: Piece[] }[] => { const out: { tier: number; pieces: Piece[] }[] = []; TIERS.forEach(([y, kinds, tall], t) => { const w = apart(w => { const s = new Shelver(w); for (let k = 0; k < seed + t; k++) s.items.push(s.items.shift()!); s.tier([0, y, 0.02], 0, -W / 2 + 0.02, W / 2 - 0.02, D - 0.06, kinds, tall); }), by = new Map<string, Piece[]>(); for (const p of w.pieces) { const l = by.get(p.rig!); if (l) l.push(p); else by.set(p.rig!, [p]); } for (const l of by.values()) out.push({ tier: t, pieces: l.map(({ rig: _, ...p }) => (void _, { ...p, at: [p.at[0], y + (p.at[1] - y) * (1 - 0.0022 * (out.length % 5)), p.at[2]] as V3, half: p.quad ? p.half : [p.half[0], p.half[1] * (1 - 0.0022 * (out.length % 5)), p.half[2]] as V3 })) }); }); return out.filter((a, i) => !out.slice(0, i).some(b => b.tier === a.tier && gap(b.pieces[0]!, a.pieces[0]!, 0.9) < -0.004)); };
const size = (p: Piece) => p.half[0] * p.half[1] * p.half[2];
function frame(p: Put, fine: boolean) { /* (the price strips are most of its yellow: no level is without them) */
  for (const [y] of TIERS) { p.box([0, y - 0.015, 0], [W / 2 - 0.022, 0.0145, D / 2 - 0.004], M.paint!, FRAME); p.box([0, y - 0.015, D / 2 + 0.002], [W / 2 - 0.022, 0.019, 0.004], M.plastic!, [0.95, 0.80, 0.25]); }
  for (const s of [-1, 1]) p.box([s * (W / 2 - 0.01), HIGH / 2, 0], [0.0115, HIGH / 2, D / 2], M.paint!, FRAME); p.box([0, HIGH / 2, -D / 2 + 0.006], [W / 2 - 0.022, HIGH / 2 - 0.01, 0.006], M.paint!, FRAME);
  p.box([0, 0.065, 0.01], [W / 2 - 0.022, 0.062, D / 2 - 0.03], M.paint!, [0.30, 0.32, 0.34]); p.box([0, HIGH - 0.03, D / 2 - 0.02], [W / 2 - 0.022, 0.055, 0.012], M.paint!, [0.86, 0.16, 0.12], fine ? { marks: [{ face: 5, at: [0, 0], size: [0.5, 0.10], art: 'atrium', color: [1, 1, 1] }] } : {});
}
const colorOf = (p: Piece): V3 => p.tint ?? [1, 1, 1];
/** one box round some pieces, of their mean color by face. Tints are roots of what is sent back, so they are meaned as squares */
function boxOf(ps: Piece[], bright = 1, pale = 0.18): Piece { /* (what is printed on them is paler than they are) */ const lo: V3 = [9, 9, 9], hi: V3 = [-9, -9, -9]; let c: V3 = [0, 0, 0], a = 0; for (const p of ps) { const b = boundsOf(p), s = p.half[0] * p.half[1]; for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, b.mn[k]!); hi[k] = Math.max(hi[k]!, b.mx[k]!); } const t = colorOf(p); c = add(c, [t[0] ** 2 * s, t[1] ** 2 * s, t[2] ** 2 * s]); a += s; } c = mul(c, (1 - pale) * bright * bright / a); c = add(c, [0.8 * pale, 0.8 * pale, 0.78 * pale]); return { at: [(lo[0] + hi[0]) / 2, (lo[1] + hi[1]) / 2, (lo[2] + hi[2]) / 2], half: [(hi[0] - lo[0]) / 2, (hi[1] - lo[1]) / 2, (hi[2] - lo[2]) / 2], rot: ID, mat: M.print!, flags: 0, tint: [Math.sqrt(c[0]), Math.sqrt(c[1]), Math.sqrt(c[2])] }; }
/** the same product side by side and behind: a run */
function runs(ps: { tier: number; pieces: Piece[] }[]): Piece[][] { const by = new Map<string, Piece[]>(); for (const q of ps) { const main = q.pieces.reduce((a, b) => size(a) > size(b) ? a : b), k = q.tier + ':' + (main.marks?.[0]?.art ?? '') + colorOf(main).join(); const l = by.get(k); if (l) l.push(...q.pieces); else by.set(k, [...q.pieces]); } return [...by.values()]; }
export const shelf: ObjectKind = { name: 'shelf', title: 'Shelf of packets', make: (): Made => ({
  name: 'shelf', title: 'Shelf of packets', size: [1.2, 1.64, 0.46], on: 'floor', close: { at: [0.1, 1.2, 0.2], far: 2.8 }, read: 3, moves: { kind: 'still' }, crowd: 36,
  levels: [
    { shape: 'pieces', error: 0, less: '', put(p) { frame(p, true); p.from({ pieces: packets(p.seed).flatMap(q => q.pieces) }); } },
    { shape: 'pieces', error: 0.012, less: 'lids and caps that are thin, the rows behind', put(p) { frame(p, false); p.from({ pieces: packets(p.seed).filter(q => q.tier === TIERS.length - 1 || q.pieces[0]!.at[2] > -0.02).flatMap(q => q.pieces.filter(x => Math.min(...x.half) > 0.013 || x === q.pieces[0])) }); } },
    { shape: 'pieces', error: 0.035, less: 'packets: a box a run', put(p) { frame(p, false); p.from({ pieces: runs(packets(p.seed)).map(r => boxOf(r, 1.03)) }); } },
    { shape: 'pieces', error: 0.09, less: 'runs: a box a tier', put(p) { frame(p, false); const all = packets(p.seed); p.from({ pieces: TIERS.map((_, t) => boxOf(all.filter(q => q.tier === t).flatMap(q => q.pieces), 1.12, 0.25)).map(q => ({ ...q, mat: M.film! })) }); } },
  ],
}) };
