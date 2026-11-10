// WHAT CAN BE SAID OF AN OBJECT WITHOUT A PICTURE: does it stand, is it whole, are its levels in order, does it live as it says. No GPU: seconds.
import { type V3, type Rot, add, sub } from '../math';
import { World, F, PIECE_FLOATS, boundsOf, type Piece } from '../scene/world';
import { M } from '../scene/materials';
import { gap } from '../scene/stand';
import { faceFights, settle } from '../scene/faces';
import { Solid } from '../scene/solid';
import { Body, Bodies } from '../scene/rigid';
import { Put, type Live, type Made } from './object';
import { extent, fightSeen, leavesOf, touch } from './shape';
import { walk } from './person';
export interface Found { id: string; what: string; value: number; unit: string; level?: number; note?: string }
export interface Count { /** of blocks of cells */ cells: number; pieces: number; moving: number; leaves: number; clusters: number; bytes: number }
export interface Built { w: World; pieces: Piece[]; live: Live | void }
export function build(m: Made, level: number, seed = 1): Built { const w = new World(), live = m.levels[level]!.put(new Put(w, m.name, [0, 0, 0], 0, seed)); return { w, pieces: [...w.pieces, ...w.moving], live }; }
/** what it takes on the GPU: pieces, the tree over them (eight orders of what stands, one of what moves), planes and curves and marks, leaves and their trees */
export function count(b: Built): Count {
  const { w } = b, rows = [...w.pieces, ...w.moving].reduce((n, p) => n + (p.planes?.length ?? 0) + 2 * (p.curves?.length ?? 0) + 3 * (p.marks?.length ?? 0), 0), leaves = w.instances.reduce((n, i) => n + w.clusters[i.cluster]!.leaves.length, 0), kinds = w.clusters.reduce((n, c) => n + c.leaves.length, 0);
  const vox = new Set([...w.pieces, ...w.moving].flatMap(p => p.vox ? [p.vox] : [])); let vb = 0, cells = 0; for (const v of vox) { vb += v.byteLength; const f = new Float32Array(v.buffer, v.byteOffset, 4), n = f[0]! * f[1]! * f[2]!; for (let i = 0; i < n; i++) if (v[4 + i]) cells++; }
  return { cells, pieces: w.pieces.length, moving: w.moving.length, leaves, clusters: w.instances.length, bytes: (w.pieces.length + w.moving.length) * PIECE_FLOATS * 4 + w.pieces.length * 32 * 8 + w.moving.length * 64 + rows * 16 + kinds * (32 + 32 * 8 * 0.67) + w.instances.length * (64 + 48 + 96 + 64) + vb };
}
const plain = (p: Piece) => !p.quad && !p.planes?.length && !p.curves?.length;
/** what hangs together with what touches the floor: the rest floats */
function afloat(b: Built, NEAR = 0.006): Piece[] {
  const ps = b.pieces.filter(p => p.at[1] > -50), bs = ps.map(boundsOf), up = ps.map((_, i) => i), find = (i: number): number => { while (up[i] !== i) { up[i] = up[up[i]!]!; i = up[i]!; } return i; }, order = ps.map((_, i) => i).sort((a, c) => bs[a]!.mn[0] - bs[c]!.mn[0]);
  for (let x = 0; x < order.length; x++) for (let y = x + 1; y < order.length; y++) { const i = order[x]!, j = order[y]!; if (bs[j]!.mn[0] > bs[i]!.mx[0] + NEAR) break; if (find(i) === find(j)) continue; let far = false; for (let k = 1; k < 3; k++) if (bs[i]!.mn[k]! > bs[j]!.mx[k]! + NEAR || bs[j]!.mn[k]! > bs[i]!.mx[k]! + NEAR) { far = true; break; } if (!far && gap(ps[i]!, ps[j]!) < NEAR && (plain(ps[i]!) && plain(ps[j]!) || touch(ps[i]!, ps[j]!, NEAR))) up[find(i)] = find(j); }
  const ground = new Set<number>(); ps.forEach((_, i) => { if (bs[i]!.mn[1] < NEAR) ground.add(find(i)); }); return ps.filter((_, i) => !ground.has(find(i)));
}
/** how far it truly reaches, leaves too */
export function bounds(b: Built): { lo: V3; hi: V3 } { const lo: V3 = [9e9, 9e9, 9e9], hi: V3 = [-9e9, -9e9, -9e9]; for (const q of b.pieces.filter(p => p.at[1] > -50).map(extent)) for (let a = 0; a < 3; a++) { lo[a] = Math.min(lo[a]!, q.mn[a]!); hi[a] = Math.max(hi[a]!, q.mx[a]!); } for (const i of leavesOf(b.w)) for (let a = 0; a < 3; a++) { lo[a] = Math.min(lo[a]!, i.at[a]! - i.r * 0.7); hi[a] = Math.max(hi[a]!, i.at[a]! + i.r * 0.7); } return { lo, hi }; }
const fnv = (s: string) => { let h = 2166136261; for (let i = 0; i < s.length; i++) h = Math.imul(h ^ s.charCodeAt(i), 16777619); return h >>> 0; };
const print = (b: Built) => fnv(JSON.stringify([b.pieces.map(p => [p.at, p.rot, p.half, p.mat, p.tint, p.flags, p.quad, p.planes, p.curves, p.marks, p.bend, p.seed, p.lamp]), b.w.instances.map(i => [i.cluster, i.at, i.rot, i.scale, i.tint, i.sway, i.phase]), b.w.clusters.map(c => c.leaves)]));
export function rules(m: Made): { found: Found[]; counts: Count[] } {
  const found: Found[] = [], say = (id: string, what: string, value: number, unit: string, level?: number, note?: string) => { found.push({ id, what, value, unit, ...(level === undefined ? {} : { level }), ...(note ? { note } : {}) }); }, counts: Count[] = [];
  const at = (p: Piece) => p.at.map(v => v.toFixed(2)).join(' ');
  m.levels.forEach((l, k) => {
    const b = build(m, k), c = count(b); counts.push(c); const here = b.pieces.filter(p => p.at[1] > -50);
    const ends = here.map(extent); if (m.on === 'floor') say('rule.foot', 'how far its foot is from the floor, beyond its error', Math.max(0, Math.abs(Math.min(...ends.map(e => e.mn[1]))) - l.error) * 1000, 'mm', k);
    const f = afloat(b); say('rule.floats', 'pieces that hang on nothing', f.length, '', k, f[0] ? `the first at ${at(f[0])}` : undefined);
    const fights = faceFights(here).filter(f => fightSeen(here, f.a, f.b)); say('rule.faces', 'pairs of faces in one plane', fights.length, '', k, fights[0] ? `the first at ${fights[0].a.at.map(v => v.toFixed(2)).join(' ')}` : undefined);
    { const c = b.w.pieces.map(p => ({ ...p })); settle(c); say('rule.parted', 'a level parts faces in one plane, seen or not: how far it moves one', 1000 * Math.max(0, ...c.map((p, i) => Math.max(...[0, 1, 2].map(a => 2 * Math.abs(p.half[a]! - b.w.pieces[i]!.half[a]!))))), 'mm', k); }
    say('rule.tint', 'pieces whose tint is cut at 1, or is no number', b.pieces.filter(p => p.tint && !p.tint.every(t => t >= 0 && t <= 1.001)).length, '', k);
    say('rule.thing', 'pieces that are no thing\'s', b.pieces.filter(p => !p.thing).length, '', k);
    say('rule.same', 'made twice, it is the same (0)', print(b) === print(build(m, k)) ? 0 : 1, '', k);
    if (k === 0) { const { lo, hi } = bounds(b), is = [0, 1, 2].map(a => hi[a]! - lo[a]!), off = Math.max(...is.map((v, a) => Math.abs(v - m.size[a]!) / Math.max(m.size[a]!, 0.15 * Math.max(...m.size)))); say('rule.size', 'how far it is from the size it says', off * 100, '%', 0, `it is ${is.map(v => v.toFixed(2)).join(' × ')}`); }
    if (k === 0 && b.w.instances.length) { let far = 0; for (const i of b.w.instances) { const s = sub(i.at, [i.rot[1][0] * 0.95 * i.scale, i.rot[1][1] * 0.95 * i.scale, i.rot[1][2] * 0.95 * i.scale]); let d = 9; for (const p of here) { const r = sub(s, p.at), e = [0, 1, 2].map(a => Math.max(0, Math.abs(r[0] * p.rot[a]![0] + r[1] * p.rot[a]![1] + r[2] * p.rot[a]![2]) - p.half[a]!)); d = Math.min(d, Math.hypot(...e)); } if (d > 0.15) far++; } say('rule.stems', 'sprays with no branch within 15 cm of their stem', 100 * far / b.w.instances.length, '%', k); }
  });
  m.levels.forEach((l, k) => { if (k) { const a = counts[k - 1]!, c = counts[k]!, n = (q: Count) => q.pieces + q.moving + q.leaves + q.cells; say('rule.order', 'its error over that of the level before (more than 1)', m.levels[k - 1]!.error > 0 ? l.error / m.levels[k - 1]!.error : l.error > 0 ? 9 : 0, '×', k); say('cost.parts', 'parts, over those of the level before', n(c) / n(a), '×', k, `${n(c)} of ${n(a)}`); } else if (l.error !== 0) say('rule.order', 'the finest is off by nothing', 0, '×', 0); });
  return { found, counts };
}
const still = { at: () => [0, 0, 0] as V3 } as never;
/** does it live as it says */
export function alive(m: Made): Found[] {
  const found: Found[] = [], say = (id: string, what: string, value: number, unit: string, note?: string) => { found.push({ id, what, value, unit, ...(note ? { note } : {}) }); }, mv = m.moves;
  if (mv.kind === 'loose') {
    const make = (lift: number, lean: number, pool: boolean) => { const b = build(m, 0), w = new World(); w.box([-20, -1.2, -20], [20, 0, 20], M.concrete!); for (const p of b.pieces) p.flags |= F.MOVES; const solid = new Solid(w, []).settle(), bodies = new Bodies(solid, pool ? [{ rect: [-4, -4, 4, 4], top: 2.5, bed: 0 }] : [], []), body = new Body(b.pieces, [0, 0, 0], [[1, 0, 0], [0, 1, 0], [0, 0, 1]], { dense: mv.dense, low: mv.low, rough: mv.rough }), home: V3 = [...body.x]; bodies.add(body); if (lift || lean) { const q: [number, number, number, number] = [Math.sin(lean / 2), 0, Math.sin(lean * 0.3), 1]; const l = Math.hypot(...q); body.q = q.map(v => v / l) as never; body.put(add(home, [0, lift, 0]), [0, 0, 0], [0, 0, 0]); } return { body, bodies, home, b }; };
    const run = (s: ReturnType<typeof make>, n: number, each?: (t: number) => void) => { for (let t = 0; t < n; t++) { s.bodies.step(1 / 60, t / 60, still); each?.(t); } return s; }, low = (b: Body) => Math.min(...b.parts.map(q => { const y = boundsOf(q.p).mn[1]; return y > -0.004 ? y : extent(q.p).mn[1]; })), tilt = (b: Body) => Math.acos(Math.max(-1, Math.min(1, b.r[1][1]))) * 57.3, hash = (b: Body) => fnv([...b.x, ...b.q].join());
    { const s = run(make(0, 0, false), 300); say('live.rests', 'left standing, how far it moved', Math.hypot(...sub(s.body.x, s.home)) * 100, 'cm', `tilt ${tilt(s.body).toFixed(1)}°`); say('live.upright', 'left standing, how far it leans', tilt(s.body), '°'); }
    { let worst = 0; const s = make(0.5, 0.35, false); run(s, 600, t => { if (t % 3 === 0) worst = Math.min(worst, low(s.body)); }); say('live.lands', 'dropped, it is at rest after ten seconds (0)', s.body.asleep ? 0 : 1, ''); say('live.sinks', 'dropped, how deep it goes into the floor', -worst * 100, 'cm', `at rest ${(-Math.min(0, low(s.body)) * 100).toFixed(1)}`); const t = run(make(0.5, 0.35, false), 600); say('live.repeats', 'the same run twice (0)', hash(s.body) === hash(t.body) ? 0 : 1, ''); }
    { const s = make(0, 0, false); run(s, 60); const big = s.body.now.reduce((a, c) => c.half[0] * c.half[1] * c.half[2] > a.half[0] * a.half[1] * a.half[2] ? c : a); s.bodies.strike([big.at[0] - 3, big.at[1], big.at[2]], [1, 0, 0], 6, s.body.m * 1.4); run(s, 400); const d = Math.hypot(s.body.x[0] - s.home[0], s.body.x[2] - s.home[2]); say('live.pushed', 'pushed, how far it goes', d * 100, 'cm', `${s.body.m.toFixed(1)} kg, tilt ${tilt(s.body).toFixed(0)}°`); say('live.weight', 'its weight', s.body.m, 'kg'); }
    { let sum = 0; const s = make(1.2, 0.1, true); run(s, 1200, t => { if (t >= 900) sum += low(s.body) / 300; }); const up = sum > 0.3; say('live.floats', 'in water it does as it says (0)', up === mv.floats ? 0 : 1, '', `its lowest is ${sum.toFixed(2)} m over the bed, under 2.5 m of water`); }
  }
  if (mv.kind === 'walks') {
    walk.on = true; try {
      const go = () => { const b = build(m, 0), live = b.live!; let slide = 0, far = 0, under = 0, was = live.feet!(), x0 = b.pieces[0]!.at, sum = ''; for (let t = 0; t < 600; t++) { live.step!(1 / 60, t / 60, still); const now = live.feet!(); now.forEach((f, i) => { if (f.down && was[i]!.down) slide = Math.max(slide, Math.hypot(f.at[0] - was[i]!.at[0], f.at[2] - was[i]!.at[2])); under = Math.min(under, f.at[1]); }); was = now.map(f => ({ at: [...f.at] as V3, down: f.down })); far = Math.max(far, Math.hypot(b.pieces[0]!.at[0] - x0[0], b.pieces[0]!.at[2] - x0[2])); if (t % 100 === 99) sum += b.pieces.map(p => p.at.join()).join(); } return { slide, far, under, hash: fnv(sum) }; }, a = go(), b = go();
      say('live.walks', 'in ten seconds it has gone', a.far, 'm'); say('live.sinks', 'walking, how deep a foot goes into the floor', -a.under * 100, 'cm'); say('live.feet', 'a foot that is down slides at most, a tick', a.slide * 1000, 'mm'); say('live.repeats', 'the same run twice (0)', a.hash === b.hash ? 0 : 1, '');
      held();
      // coarser levels hang on the same rig: their middles go where the finest's does
      together();
    } finally { walk.on = false; }
  }
  if (mv.kind === 'works') {
    const go = () => { const b = build(m, 0), live = b.live; if (!live?.step) return null; const was = b.pieces.map(p => [...p.at] as V3), r0 = b.pieces.map(p => p.rot); let most = 0, sum = ''; for (let t = 0; t < 600; t++) { live.step(1 / 60, t / 60, still); b.pieces.forEach((p, i) => { const far = Math.max(...p.half); most = Math.max(most, Math.hypot(...sub(p.at, was[i]!)), far * Math.max(...[0, 1, 2].map(a => Math.hypot(...sub(p.rot[a]!, r0[i]![a]!))))); }); if (t % 100 === 99) sum += b.pieces.map(p => p.at.join() + p.rot.join()).join(); } return { most, hash: fnv(sum) }; }, a = go(), b = go();
    say('live.works', 'in ten seconds a part of it goes or turns by', a ? a.most * 100 : 0, 'cm'); say('live.repeats', 'the same run twice (0)', a && b && a.hash === b.hash ? 0 : 1, ''); held(); together();
  }
  return found;
  function held() { const b = build(m, 0); let worst = 0, when = 0; for (let t = 0; t < 600; t++) { b.live?.step?.(1 / 60, t / 60, still); if (t % 60 === 30) { const n = afloat(b, 0.02).length; if (n > worst) { worst = n; when = t / 60; } } } say('live.held', 'moving, pieces that come away from the rest', worst, '', worst ? `at ${when.toFixed(1)} s` : undefined); }
  /** does each part of the finest that goes anywhere have a part of the level beside it that goes with it? By how far it strays, for nine tenths of them by size */
  function together() {
    const track = (k: number) => { const b = build(m, k), at: { at: V3; rot: Rot }[][] = []; for (let t = 0; t < 240; t++) { if (t % 12 === 0) at.push(b.pieces.map(p => ({ at: [...p.at] as V3, rot: p.rot }))); b.live?.step?.(1 / 60, t / 60, still); } return { ps: build(m, k).pieces, at }; }, fine = track(0), inside = (f: { at: V3; rot: Rot }, x: V3): V3 => { const e = sub(x, f.at); return [0, 1, 2].map(a => f.rot[a]![0] * e[0] + f.rot[a]![1] * e[1] + f.rot[a]![2] * e[2]) as V3; };
    m.levels.forEach((l, k) => { if (!k) return; const c = track(k), strays: [number, number][] = []; fine.ps.forEach((p, i) => { const r = Math.max(...p.half); if (r < Math.max(0.02, 2 * l.error)) return; /* (its two ends, so that what only turns is seen to) */ const axis = p.half.indexOf(r), ends = [1, -1].map(s => fine.at.map(f => add(f[i]!.at, [f[i]!.rot[axis]![0] * r * s, f[i]!.rot[axis]![1] * r * s, f[i]!.rot[axis]![2] * r * s]))); const gone = Math.max(...ends.map(e => Math.max(...e.map(x => Math.hypot(...sub(x, e[0]!)))))); if (gone < 0.03) return;
        let best = gone; const b0 = boundsOf(p); c.ps.forEach((q, j) => { const b1 = boundsOf(q), far = Math.max(...[0, 1, 2].map(a => Math.max(b1.mn[a]! - b0.mx[a]!, b0.mn[a]! - b1.mx[a]!))); if (far > 0.03 + l.error) return; let w = 0; for (const e of ends) { const x0 = inside(c.at[0]![j]!, e[0]!); for (let t = 1; t < e.length; t++) w = Math.max(w, Math.hypot(...sub(inside(c.at[t]![j]!, e[t]!), x0))); } best = Math.min(best, w); }); strays.push([best, r]); });
      strays.sort((a, b) => a[0] - b[0]); const all = strays.reduce((n, x) => n + x[1], 0); let sum = 0, v = 0; for (const [d, r] of strays) { sum += r; v = d; if (sum >= 0.9 * all) break; } found.push({ id: 'live.together', what: 'moving, how far its parts stray from the finest\'s', value: v * 100, unit: 'cm', level: k, note: `${strays.length} parts of the finest go anywhere` }); });
  }
}
