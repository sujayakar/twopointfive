// THE LOOK PROTOTYPE's DAYLIGHT AT OPENINGS (the look panel's "Daylight at openings" row, render/frame.ts lookConfig.windows; engine.ts lookOpeningLights keeps
// the lights). By day the open air is thousands of times a lamp, and it comes into a room wherever the room is open to it: a window, a door that stands open,
// a hole a charge blew in the wall. The radiance cache brings that light in too, but as a grid of probes a metre apart that hold a few
// broad directions each: it arrives spread over the room with no way it came from. So each opening is ALSO a light of its own, as a film crew would hang one:
// a one-sided disk just inside the wall, as big as the opening, as bright as what the opening looks out on, shaded per pixel like any lamp. A man by an open
// door throws a shadow into the room and the light falls off from the doorway.
// ONE RULE FINDS THEM ALL, from the level's boxes as they stand (the maintainer, 11-01, of windows and open doors as real openings: "yep let's do it. blowing
// holes in walls should work too!"). Before it a window was an upright Glass box of the level, found once, so a door and a hole were nothing and a broken wall
// changed nothing. The rule, on the renderer's floor plan (scene/placePlan.ts's cells and its test of what is a roof):
//   AN OPENING IS WHERE FLOOR UNDER A ROOF MEETS FLOOR UNDER THE SKY WITH NO WALL BETWEEN THEM, AND WALL ON BOTH HANDS.
// Glass is no wall to light. What is left of a wall round a hole is one. "Wall on both hands" is what tells a hole in a wall from an open side: a stall's
// awning on four posts and a canopy with one wall behind it are open sides, and the cache's probes carry what comes in there. A passage's mouth between its
// two walls is an opening, however wide. The opening's size is MEASURED, not read off the grid: the wall's plane across it is sampled every 5 cm from the
// floor to the roof, and a sample is open when no solid box stands in the straight way through the wall, from half a metre outside to the light's own plane
// (so a lintel, a sill, a mullion and sandbags against the outside each take what they cover; a cupboard in the room is in front of the light, and shadows it). TWO STAGES: `planOpenings` finds every opening the WALLS leave, where its light hangs and how big it is with nothing
// in it (made again only when a wall changes); `openAreas` then takes out what each door's leaf covers where it hangs now (a shut door: all of it), so a
// doorway's light stays where it is while its door swings, and only its strength moves.
// Nothing here knows a level, a door or a window by name, and it imports nothing from src/game. The check: tools/qa/openings-check.ts.
import type { Vec3 } from '../math/vec';
import { BoxFlag, type Box } from './boxes';
import { WORLD } from './world';
import { PLACE, PLACE_W, PLACE_H, PLACE_RES, placeWall, placeSolid, placeRoof, stampCells } from './placePlan';

export const LOOK_OPENINGS = {
  ground: 0.5,    // what the ground outside sends back of the light on it (snow more, a shaded yard less)
  up: 0.3,        // the sky an opening looks at: this much of the zenith's and the rest of the horizon's
  least: 0.004,   // an opening whose light is under this is not lit at all (a night's sky: it would only take a lamp's place in the list)
  seen: 0.008,    // … and a PORTAL (render/lights.ts LIGHT_PORTAL) is lit while what it looks out on × the frame's exposure is over this: by what the picture
                  // would show of a surface under that sky, whatever the hour and however big the opening
  proud: 0.05,    // m from the wall's inside face into the room (a disk inside the reveal has the wall in front of its rim)
  deep: 2,        // cells of floor a man could stand on, under the roof, found within `look` cells in and `side` to either hand of an opening, for it to lead
                  // into a place (the strip under an eave does not, nor the bed of a lorry under its canvas)
  look: 6, side: 3,
  reach: 0.5,     // m OUTSIDE the opening's line: a solid box this near out there, in the straight way through, blocks what it covers (a shutter, sandbags
                  // against the wall). INSIDE, only what stands in the wall's own depth does, as far as the light's plane: a thing in the room is in front
                  // of the light and throws its own shadow (a cupboard by a window, a leaf swung into the room)
  thin: 0.15,     // m: a box this thin across the wall is the wall's own (its inside face is where the light hangs)
  ds: 0.05,       // m between samples along the wall …
  dy: 0.1,        // … and up it
  small: 0.25,    // m²: less open than this is not worth a light (a vent's slot, a hatch)
  crack: 0.12,    // m: a slit narrower than this along the wall is shut
  play: 0.07,     // m: the play a shut leaf has in its frame, and a finger over (a 0.84 m leaf in a 0.9 m doorway has 6 cm, at one jamb or shared
                  // between the two): a gap of the narrowest way through is open by what it has over this (ajarOf)
  slit: 0.25,     // m: … and one lower than this up the wall (the gap between a shed's wall and its roof, a vent's slot: with it open no wall would be shut
                  // from the floor to the roof, and nothing would have a wall on either hand)
  jamb: 0.2,      // m of wall, shut from the floor to the roof, an opening ends at on each hand. Less than this between two open stretches is a mullion or a
                  // post: they are one opening. Open air past its end, with no such wall, makes it an open side: no light
  span: 2,        // m: an opening longer than this, and than twice its height, is lit as several lights side by side (a strip of high glass, a glazed wall)
  same: 0.6,      // m: two openings found this near each other, facing the same way, are one (a wall a little off the grid's square gives its doorway on two lines)
  most: 2.5,      // m²: what one light counts of its opening at most. THE CACHE BRINGS THE SAME DAYLIGHT IN A SECOND TIME, and the bigger the opening the more of
                  // it the probes resolve: past a big window's size the light would only double the room. It is there to give the daylight a direction
  settle: 4,      // drawn frames the walls must have stood still for before the openings are found again (a blast's own frame is the long one: not on it)
  keep: 32,       // openings a level keeps (the biggest, if it has more) …
  lit: 12,        // … and how many are lit at once: the nearest to Sam (engine.ts)
};

export interface Opening {
  /** where the light hangs */ pos: Vec3; /** into the room */ dir: Vec3; /** the disk's radius, m */ radius: number;
  /** what the walls leave open, m² */ full: number; /** what is open now, a door's leaf taken out, m² */ area: number;
  /** … and what is open now TO A PORTAL, m²: the narrowest way through it past whatever hangs in it (ajarOf: a leaf a hand's width off its frame is open by
   *  that hand's width, where `area` has nothing yet and then pops on) */
  ajar: number;
  /** the upright rectangle that holds everything the walls leave open, and no more: its middle (`mid`, in the light's plane; `pos` is where the open part's
   *  weight lies, which is off the middle of an opening that is no plain rectangle), half its width along the wall and half its height, m (a portal's
   *  shape: what of it is wall, a ray toward it finds out) */
  mid: Vec3;
  half: [number, number];
  /** the samples the walls leave open (openAreas' own): across `axis` at L, the straight way through from `a` (outside) to `b` (the light's plane), the
   *  wall's own inside and outside faces at `in` and `out`, the footprints of the boxes that end the opening on each hand (`jambs`: across the wall and
   *  along it; null where it ends at another light's part of the same glass), from s0 along the wall, `nY` heights a place, a place's heights as bits */
  cut: { axis: 0 | 1; L: number; a: number; b: number; in: number; out: number; jambs: [readonly Pt[] | null, readonly Pt[] | null]; s0: number; nY: number; cols: Int32Array };
}

/** does a box stop light going straight through a wall: solid, drawn, and not glass (a decal, a glow card and a pane do not; a roof the camera cuts away is
 *  above every sample). Of the things that move, a door's leaf alone (placeWall's own test: a man is no part of a wall). */
const blocks = (b: Box, dynamic: boolean): boolean => (b.flags & (BoxFlag.Glass | BoxFlag.NoShadow)) === 0 && (!dynamic || placeWall(b) === 255);

type Pt = readonly [number, number];
interface Near { b: Box; c: number; sn: number; rn: number; cn: number }
/** the boxes of `from` within reach of the line at L across `axis`, between s0 and s1 along it */
function nearLine(from: readonly Box[], axis: 0 | 1, L: number, s0: number, s1: number, out: Near[]): void {
  out.length = 0;
  for (const b of from) {
    const cs = Math.abs(Math.cos(b.yaw)), sn = Math.abs(Math.sin(b.yaw)), rx = cs * b.h[0] + sn * b.h[2], rz = sn * b.h[0] + cs * b.h[2];
    const cn = axis === 0 ? b.c[0] : b.c[2], ca = axis === 0 ? b.c[2] : b.c[0], rn = axis === 0 ? rx : rz, rs = axis === 0 ? rz : rx;
    if (Math.abs(cn - L) > rn + LOOK_OPENINGS.reach || ca + rs < s0 || ca - rs > s1) continue;
    out.push({ b, c: Math.cos(b.yaw), sn: Math.sin(b.yaw), rn, cn });
  }
}
/** does the straight way through at place s, from `a` to `e` across the wall, cross this box's footprint */
function crosses({ b, c, sn }: Near, axis: 0 | 1, a: number, e: number, s: number): boolean {
  // the way through in the box's own frame (stampCells' turn), clipped to its footprint
  let t0 = 0, t1 = 1;
  const ax = (axis === 0 ? a : s) - b.c[0], az = (axis === 0 ? s : a) - b.c[2], bx = (axis === 0 ? e : s) - b.c[0], bz = (axis === 0 ? s : e) - b.c[2];
  for (let d = 0; d < 2; d++) { const p = d === 0 ? c * ax - sn * az : sn * ax + c * az, w = (d === 0 ? c * bx - sn * bz : sn * bx + c * bz) - p, h = d === 0 ? b.h[0] : b.h[2];
    if (Math.abs(w) < 1e-9) { if (Math.abs(p) > h) return false; continue; }
    const u = (-h - p) / w, v = (h - p) / w; t0 = Math.max(t0, Math.min(u, v)); t1 = Math.min(t1, Math.max(u, v)); if (t0 > t1) return false; }
  return true;
}
/** a box's footprint as corners in order, across the wall and along it */
const outline = ({ b, c, sn }: Near, axis: 0 | 1): Pt[] => [[-1, -1], [1, -1], [1, 1], [-1, 1]].map(([i, j]): Pt => { const lx = i * b.h[0], lz = j * b.h[2], x = b.c[0] + c * lx + sn * lz, z = b.c[2] - sn * lx + c * lz; return axis === 0 ? [x, z] : [z, x]; });
/** `bits` (the heights open at place s along the line, one bit a sample of `dy`) less what the boxes of `near` cover there: a box covers a height when it
 *  stands in the straight way through, from `a` to `b` across the wall, at that place */
function openAt(near: readonly Near[], axis: 0 | 1, a: number, e: number, s: number, nY: number, bits: number): number {
  const K = LOOK_OPENINGS;
  for (const nr of near) {
    if (!crosses(nr, axis, a, e, s)) continue;
    const b = nr.b, v0 = Math.max(0, Math.ceil((b.c[1] - b.h[1]) / K.dy - 0.5 - 1e-6)), v1 = Math.min(nY - 1, Math.floor((b.c[1] + b.h[1]) / K.dy - 0.5 + 1e-6));
    for (let v = v0; v <= v1; v++) bits &= ~(1 << v);
  }
  return bits;
}
/** a slit narrower than `crack` along the wall is no way in for light (the hand's width between a shut leaf and its jamb) */
function shutCracks(cols: Int32Array): void {
  const n = cols.length;
  for (let q = 0; q < n;) { if (!cols[q]) { q++; continue; } let e = q; while (e < n && cols[e]) e++; if ((e - q) * LOOK_OPENINGS.ds < LOOK_OPENINGS.crack) for (let w = q; w < e; w++) cols[w] = 0; q = e; }
}
/** … and so is one lower than `slit` up it */
function shutSlits(cols: Int32Array, nY: number): void {
  const most = Math.ceil(LOOK_OPENINGS.slit / LOOK_OPENINGS.dy - 1e-6) - 1;   // (runs of this many samples or fewer)
  for (let q = 0; q < cols.length; q++) { let c = cols[q]; if (!c) continue;
    for (let v = 0; v < nY;) { if (!(c & (1 << v))) { v++; continue; } let e = v; while (e < nY && (c & (1 << e))) e++; if (e - v <= most) for (let w = v; w < e; w++) c &= ~(1 << w); v = e; }
    cols[q] = c; }
}
/** what a level's openings are found from, as one number: every box of its statics that could stand in light's way, with where it is (a frame's bar and
 *  a lintel are part of the measure, and the place map's own key leaves them out). Only what the finder reads: a box that turns wet or is given another
 *  paint is the same box. No test of any box. */
const SHAPE = BoxFlag.Glass | BoxFlag.NoShadow | BoxFlag.NoPrimary | BoxFlag.Dynamic;
export function openingsKey(statics: readonly Box[]): number {
  let key = statics.length * 31 | 0; const mix = (v: number) => { key = Math.imul(key ^ (Math.round(v * 100) | 0), 0x9e3779b1); key ^= key >>> 15; };   // (each number in its turn: a box moved is never the same key by a sum that cancels)
  for (const b of statics) { mix((b.flags & SHAPE) + (b.rot ? 0.5 : 0)); mix(b.c[0]); mix(b.c[1]); mix(b.c[2]); mix(b.h[0]); mix(b.h[1]); mix(b.h[2]); mix(b.yaw); }
  return key;
}
/** … and what openAreas reads: where each door's leaf hangs */
export function leavesKey(dynamics: readonly Box[]): number {
  let key = 7; for (const b of dynamics) if (blocks(b, true)) key = (Math.imul(key, 33) + Math.round(b.yaw * 50) * 7 + Math.round((b.c[0] * 3 + b.c[2] * 7) * 20)) | 0;
  return key;
}
const count = (cols: Int32Array): number => { let n = 0; for (let q = 0; q < cols.length; q++) { let v = cols[q]; while (v) { v &= v - 1; n++; } } return n; };

/** every opening the WALLS of a level leave, as it stands: a door's leaf is no part of this, shut or swinging (`area` = `full` here; openAreas takes the
 *  leaves out). Pure, and the heavy half: made again only when a wall changes. */
export function planOpenings(statics: readonly Box[]): Opening[] {
  const W = PLACE_W, H = PLACE_H, N = W * H, K = LOOK_OPENINGS, cell = 1 / PLACE_RES, ox = WORLD.rcMin[0], oz = WORLD.rcMin[2], top = WORLD.ceilingY, SL = 8, step = top / SL;
  // THE PLAN: which eighths of the height from the floor to the ceiling something that stops light fills in each cell, which of them glass fills, and how
  // high the roof over the cell is (scene/placePlan.ts's own plan is cut between a man's shin and his head, for which room a point is in: a strip of glass
  // over a door's height is nothing to it, and it is a window)
  const solid = new Uint8Array(N), glass = new Uint8Array(N), under = new Uint8Array(N).fill(255), roofY = new Float32Array(N).fill(Infinity);
  const bitsOf = (y0: number, y1: number): number => { let m = 0; for (let i = 0; i < SL; i++) { const y = (i + 0.5) * step; if (y0 - 0.02 <= y && y <= y1 + 0.02) m |= 1 << i; } return m; };
  const STAND = bitsOf(PLACE.maxFoot, PLACE.minTop);   // (the heights a standing man takes)
  for (const b of statics) {
    const y0 = b.c[1] - b.h[1], y1 = b.c[1] + b.h[1], pane = (b.flags & BoxFlag.Glass) !== 0;
    if (placeRoof(b)) stampCells(b.c[0], b.c[2], b.h[0], b.h[2], b.yaw, k => { if (y0 < roofY[k]) roofY[k] = y0; });
    if (!placeSolid(b)) continue;   // (the place map's own test: not a post, a pipe, a decal or a glow card)
    const bits = bitsOf(y0, y1), to = pane ? glass : solid; if (bits) stampCells(b.c[0], b.c[2], b.h[0], b.h[2], b.yaw, k => { to[k] |= bits; });
  }
  for (let k = 0; k < N; k++) if (roofY[k] < Infinity) { let m = 0; for (let i = 0; i < SL; i++) if ((i + 0.5) * step < roofY[k]) m |= 1 << i; under[k] = m; }
  /** the heights light can pass a cell at: all that nothing solid fills, under its roof */
  const open = (k: number): number => ~solid[k] & under[k];
  const roofed = (k: number): boolean => roofY[k] < Infinity;
  // 1. the edges: two cells side by side, one under a roof and one not, light passing both at a height they share, and floor under the roof behind
  interface Edge { axis: 0 | 1; line: number; sign: 1 | -1; at: number; top: number }
  const edges: Edge[] = [];
  // does the first cell under the roof lead into a place: a short flood under the roof, cell to cell where light passes both at a height they share, no
  // further in than `look` cells and `side` to either hand, finds `deep` cells a man could stand on that are IN from the edge (the strip under an eave has
  // none: a wall is behind it)
  const seen = new Uint8Array(N), stack: number[] = [], touched: number[] = [];
  const leadsIn = (i: number, j: number, di: number, dj: number): boolean => {
    let n = 0; stack.length = 0; stack.push(j * W + i); seen[j * W + i] = 1; touched.push(j * W + i);
    while (stack.length && n < K.deep) { const c = stack.pop()!, cx = c % W, cz = (c - cx) / W;
      for (let d = 0; d < 4; d++) { const x = cx + (d === 0 ? 1 : d === 1 ? -1 : 0), z = cz + (d === 2 ? 1 : d === 3 ? -1 : 0); if (x < 0 || z < 0 || x >= W || z >= H) continue;
        const k = z * W + x, inn = (x - i) * di + (z - j) * dj; if (seen[k] || inn < 0 || inn >= K.look || Math.abs((x - i) * dj - (z - j) * di) > K.side || !roofed(k) || !(open(k) & open(c))) continue;
        seen[k] = 1; touched.push(k); stack.push(k); if (inn >= 1 && ((solid[k] | glass[k]) & STAND) === 0) n++; } }
    for (const k of touched) seen[k] = 0; touched.length = 0;
    return n >= K.deep; };
  for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) {
    const k = j * W + i, a = open(k); if (!a) continue;
    for (const axis of [0, 1] as const) {
      const x = i + (axis === 0 ? 1 : 0), z = j + (axis === 1 ? 1 : 0); if (x >= W || z >= H) continue;
      const t = z * W + x, b = open(t); if (!(a & b) || roofed(k) === roofed(t)) continue;
      const sign = roofed(t) ? 1 : -1, di = axis === 0 ? sign : 0, dj = axis === 1 ? sign : 0;
      if (leadsIn(roofed(t) ? x : i, roofed(t) ? z : j, di, dj)) edges.push({ axis, line: axis === 0 ? i + 1 : j + 1, sign, at: axis === 0 ? j : i, top: Math.min(top, roofY[roofed(t) ? t : k]) });
    }
  }
  // 2. edges in one line, side by side, are one stretch of the roof's edge to look along
  edges.sort((p, q) => p.axis - q.axis || p.line - q.line || p.sign - q.sign || p.at - q.at);
  interface Run { axis: 0 | 1; line: number; sign: 1 | -1; a: number; b: number; top: number }
  const runs: Run[] = [];
  for (let e = 0; e < edges.length;) {
    let f = e, lid = edges[e].top; while (f + 1 < edges.length && edges[f + 1].axis === edges[e].axis && edges[f + 1].line === edges[e].line && edges[f + 1].sign === edges[e].sign && edges[f + 1].at === edges[f].at + 1) { f++; lid = Math.min(lid, edges[f].top); }
    const E = edges[e]; runs.push({ axis: E.axis, line: E.line, sign: E.sign, a: E.at, b: E.at + (f - e + 1), top: lid });
    e = f + 1;
  }
  // 3. measure each: sample the wall's plane across it against the boxes that stand in the straight way through
  const out: Opening[] = [], near: Near[] = [], J = Math.round(K.jamb / K.ds), pad = 2 * cell;   // (the grid rounds a jamb to a cell: the samples find it, and the wall past it)
  const blockers: Box[] = []; for (const b of statics) if (blocks(b, false) && b.c[1] + b.h[1] > 0.02 && b.c[1] - b.h[1] < top) blockers.push(b);
  for (const r of runs) {
    const o0 = r.axis === 0 ? ox : oz, oS = r.axis === 0 ? oz : ox, L = o0 + r.line * cell;   // the line's place across the wall, m
    const c0 = oS + r.a * cell, c1 = oS + r.b * cell, s0 = c0 - pad, s1 = c1 + pad;
    nearLine(blockers, r.axis, L, s0, s1, near);
    // the wall's own boxes say where its inside face is (thin across the wall, on the line)
    let face = 0, back = Infinity; for (const { b, rn, cn } of near) if (rn <= K.thin && Math.abs(cn - L) <= 0.31 && b.h[1] >= 0.2) { face = Math.max(face, r.sign * (cn - L) + rn); back = Math.min(back, r.sign * (cn - L) - rn); }
    if (back > face) back = face;   // (… and where its outside face is: no such box, no depth)
    // each place along the wall: the heights that are open there, from the floor to the roof, as bits (one a sample)
    const nS = Math.round((s1 - s0) / K.ds), nY = Math.max(1, Math.min(30, Math.floor(r.top / K.dy + 1e-6))), all = (1 << nY) - 1, cols = new Int32Array(nS);
    const hang = Math.min(face, 0.45) + K.proud, wa = L - r.sign * K.reach, wb = L + r.sign * (hang - 0.01);   // (from outside to just short of where the light hangs)
    for (let q = 0; q < nS; q++) cols[q] = openAt(near, r.axis, wa, wb, s0 + (q + 0.5) * K.ds, nY, all);
    // the box that ends an opening on one hand, at the opening's mid height: its footprint is that jamb (ajarOf). THE WALL'S, not a crate's that stands
    // against it: the tallest of the boxes the wall's own depth is crossed by there (where no box is thin enough to say the wall's depth, of all the way)
    const dIn = L + r.sign * Math.min(face, 0.45), dOut = L + r.sign * Math.max(back, -0.45), deep = Math.abs(dIn - dOut) > 0.02;
    const jamb = (s: number, y: number): Pt[] | null => { let best: Near | null = null;
      for (const nr of near) if (Math.abs(y - nr.b.c[1]) <= nr.b.h[1] && crosses(nr, r.axis, deep ? dOut : wa, deep ? dIn : wb, s) && (!best || nr.b.c[1] + nr.b.h[1] > best.b.c[1] + best.b.h[1] + 1e-6)) best = nr;
      return best ? outline(best, r.axis) : null; };
    shutSlits(cols, nY); shutCracks(cols);
    // the open stretches: one ends where the wall is shut from the floor to the roof for `jamb` or more (less is a mullion or a post, and the stretch goes on)
    for (let q = 0; q < nS;) {
      if (!cols[q]) { q++; continue; }
      let last = q, e = q + 1; while (e < nS) { if (cols[e]) { last = e++; continue; } let g = e; while (g < nS && !cols[g]) g++; if (g - e >= J || g >= nS) break; e = g; }
      const a = q, b = last; q = last + 1;
      if (a < J || b > nS - 1 - J) continue;   // (open to the end of what was looked at, on one hand: an open side, or a neighbour's opening that its own stretch measures)
      if (s0 + (b + 1) * K.ds < c0 - 0.5 * cell || s0 + a * K.ds > c1 + 0.5 * cell) continue;   // (not this stretch's own: its neighbour's)
      let loY = Infinity, hiY = -Infinity; for (let w = a; w <= b; w++) for (let v = 0; v < nY; v++) if (cols[w] & (1 << v)) { loY = Math.min(loY, v); hiY = Math.max(hiY, v); }
      const wide = (b - a + 1) * K.ds, tall = (hiY - loY + 1) * K.dy, parts = Math.max(1, Math.round(wide / Math.max(2 * tall, K.span)));
      for (let p = 0; p < parts; p++) {
        const qa = a + Math.round((b - a + 1) * p / parts), qb = a + Math.round((b - a + 1) * (p + 1) / parts) - 1; if (qb < qa) continue;
        let cells = 0, sumS = 0, sumY = 0, loS = Infinity, hiS = -Infinity, lo = Infinity, hi = -Infinity;
        for (let w = qa; w <= qb; w++) { const s = s0 + (w + 0.5) * K.ds;
          for (let v = 0; v < nY; v++) if (cols[w] & (1 << v)) { const y = (v + 0.5) * K.dy; cells++; sumS += s; sumY += y; loS = Math.min(loS, s); hiS = Math.max(hiS, s); lo = Math.min(lo, y); hi = Math.max(hi, y); } }
        const full = cells * K.ds * K.dy; if (full < K.small) continue;
        const s = sumS / cells, y = sumY / cells, n = L + r.sign * hang, hw = (hiS - loS + K.ds) * 0.5, hh = (hi - lo + K.dy) * 0.5;
        out.push({ pos: r.axis === 0 ? [n, y, s] : [s, y, n], dir: r.axis === 0 ? [r.sign, 0, 0] : [0, 0, r.sign], radius: Math.max(0.1, Math.min(Math.sqrt(full / Math.PI), Math.min(hw, hh) + 0.02)), full, area: full, ajar: full, mid: r.axis === 0 ? [n, (lo + hi) / 2, (loS + hiS) / 2] : [(loS + hiS) / 2, (lo + hi) / 2, n], half: [hw, hh],
          cut: { axis: r.axis, L, a: wa, b: wb, in: dIn, out: dOut, jambs: [jamb(s0 + (qa - 0.5) * K.ds, (lo + hi) / 2), jamb(s0 + (qb + 1.5) * K.ds, (lo + hi) / 2)], s0: s0 + qa * K.ds, nY, cols: cols.slice(qa, qb + 1) } });
      }
    }
  }
  // the biggest first; one found twice (on two lines of the grid, by a wall a little off its square) is kept once, the bigger
  out.sort((p, q) => q.full - p.full || p.pos[0] - q.pos[0] || p.pos[2] - q.pos[2]);
  const kept: Opening[] = [];
  // one found twice (a wall a little off the grid's square gives its doorway on two lines, one behind the other) is kept once: the same way, within `same`
  // through the wall and up it, and side by side by less than the smaller disk. Two slots side by side with a pier between them are two
  const twice = (k: Opening, o: Opening) => { const dx = k.pos[0] - o.pos[0], dz = k.pos[2] - o.pos[2];
    return k.dir[0] * o.dir[0] + k.dir[2] * o.dir[2] > 0.9 && Math.abs(dx * o.dir[0] + dz * o.dir[2]) < K.same && Math.abs(k.pos[1] - o.pos[1]) < K.same && Math.abs(dx * o.dir[2] - dz * o.dir[0]) < Math.max(0.15, Math.min(k.radius, o.radius)); };
  for (const o of out) if (!kept.some(k => twice(k, o))) kept.push(o);
  return kept.slice(0, K.keep);
}

/** how far apart two outlines are in plan (each a segment or a convex outline, as corners in order): 0 where they touch, cross or one holds the other */
function apart(A: readonly Pt[], B: readonly Pt[]): number {
  const edges = (P: readonly Pt[]) => P.length === 2 ? 1 : P.length, side = (a: Pt, b: Pt, p: Pt) => (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
  const toSeg = (p: Pt, a: Pt, b: Pt) => { const ex = b[0] - a[0], ey = b[1] - a[1], l2 = ex * ex + ey * ey, t = l2 > 0 ? Math.max(0, Math.min(1, ((p[0] - a[0]) * ex + (p[1] - a[1]) * ey) / l2)) : 0; return Math.hypot(p[0] - a[0] - ex * t, p[1] - a[1] - ey * t); };
  const holds = (P: readonly Pt[], p: Pt) => { if (P.length < 3) return false; let pos = 0, neg = 0; for (let i = 0; i < P.length; i++) { const k = side(P[i], P[(i + 1) % P.length], p); if (k > 0) pos++; else if (k < 0) neg++; } return (pos === 0) !== (neg === 0); };   // (all on one side; an outline with no area holds nothing)
  if (holds(B, A[0]) || holds(A, B[0])) return 0;
  let d = Infinity;
  for (let i = 0; i < edges(A); i++) { const a0 = A[i], a1 = A[(i + 1) % A.length];
    for (let j = 0; j < edges(B); j++) { const b0 = B[j], b1 = B[(j + 1) % B.length];
      if (side(a0, a1, b0) * side(a0, a1, b1) < 0 && side(b0, b1, a0) * side(b0, b1, a1) < 0) return 0;   // (they cross)
      d = Math.min(d, toSeg(a0, b0, b1), toSeg(a1, b0, b1), toSeg(b0, a0, a1), toSeg(b1, a0, a1)); } }
  return d;
}
/** WHAT IS OPEN OF AN OPENING THAT THINGS HANG IN, TO A PORTAL (`ajar`): the narrowest way through it in plan, a height at a time. Its two jambs (the
 *  footprints of the boxes that end it, `cut.jambs`) and the boxes near it (a leaf, a pair of leaves, whatever `blocks` says stops light) are a chain
 *  across the doorway, and the light that gets through is what the gaps of the SHORTEST chain from jamb to jamb leave: the cut that lets least through.
 *  Each gap less the `play` a shut leaf has at a jamb. So a leaf a hand's width off its frame is open by that hand's width, whichever way it swings and
 *  however thick the wall, a pair of leaves by the gap between them, and nothing here knows what a door is. What a leaf leaves over its head is a slit,
 *  as in a wall: a height nothing hangs at, lower than `slit`, is as open as the height under it. (`area`, the disk's, counts only what the straight way
 *  through finds open: nothing until a leaf has swung clear of the wall's depth, a quarter of the way round.) */
function ajarOf(o: Opening, near: readonly Near[]): number {
  const K = LOOK_OPENINGS, c = o.cut, at = (n: number, s: number): Pt => [n, s], outlines = near.map(nr => outline(nr, c.axis)), last = c.cols.length - 1;
  const wide = new Float64Array(c.nY), hung = new Uint8Array(c.nY);   // a height's open width, m; and whether anything hangs at it
  for (let v = 0; v < c.nY; v++) {
    let q0 = -1, q1 = -1, n = 0; for (let q = 0; q <= last; q++) if (c.cols[q] & (1 << v)) { if (q0 < 0) q0 = q; q1 = q; n++; }
    if (q0 < 0) { hung[v] = 2; continue; }   // (wall at this height)
    const y = (v + 0.5) * K.dy, here: (readonly Pt[])[] = []; for (let i = 0; i < near.length; i++) if (Math.abs(y - near[i].b.c[1]) <= near[i].b.h[1]) here.push(outlines[i]);
    if (!here.length) { wide[v] = n * K.ds; continue; }
    // the chain: node 0 and 1 the jambs (the boxes that end the opening; where it ends at no box, the wall's own depth at the last sample), then
    // the boxes that hang in it; the shortest way from 0 to 1 over the gaps between them
    const sLo = c.s0 + q0 * K.ds, sHi = c.s0 + (q1 + 1) * K.ds;
    const nodes: (readonly Pt[])[] = [(q0 === 0 && c.jambs[0]) || [at(c.out, sLo), at(c.in, sLo)], (q1 === last && c.jambs[1]) || [at(c.out, sHi), at(c.in, sHi)], ...here], N = nodes.length;
    // (jamb to jamb with nothing between is the doorway itself, and no chain. Each gap gives up the `play`: a gap narrower than that is no gap)
    const dist = new Array<number>(N).fill(Infinity), done = new Array<boolean>(N).fill(false); dist[0] = 0;
    for (let k = 0; k < N; k++) { let u = -1; for (let i = 0; i < N; i++) if (!done[i] && (u < 0 || dist[i] < dist[u])) u = i; done[u] = true;
      for (let w = 0; w < N; w++) if (!done[w] && u + w !== 1) dist[w] = Math.min(dist[w], dist[u] + Math.max(0, apart(nodes[u], nodes[w]) - K.play)); }
    wide[v] = Math.min(n * K.ds, dist[1]); hung[v] = 1;
  }
  // a slit over a leaf's head or under its foot
  const most = Math.ceil(K.slit / K.dy - 1e-6) - 1;
  for (let v = 0; v < c.nY;) { if (hung[v]) { v++; continue; } let e = v; while (e < c.nY && !hung[e]) e++;
    if (e - v <= most) { const to = v > 0 && hung[v - 1] === 1 ? wide[v - 1] : e < c.nY && hung[e] === 1 ? wide[e] : -1; if (to >= 0) for (let w = v; w < e; w++) wide[w] = Math.min(wide[w], to); }
    v = e; }
  let sum = 0; for (let v = 0; v < c.nY; v++) sum += wide[v];
  return sum * K.dy;
}

/** what is open of each opening NOW: its `full` less what a door's leaf covers where it hangs (the leaf with its true shape: a shut one covers all of its
 *  doorway, one swung out a part; one swung into the room covers only what of it is still in the wall's depth, and shadows the rest itself). Writes `area`. The light half: made again when a leaf swings. */
export function openAreas(list: readonly Opening[], dynamics: readonly Box[]): void {
  const K = LOOK_OPENINGS, leaves: Box[] = [], near: Near[] = []; for (const b of dynamics) if (blocks(b, true)) leaves.push(b);
  for (const o of list) {
    const c = o.cut; nearLine(leaves, c.axis, c.L, c.s0, c.s0 + c.cols.length * K.ds, near);
    if (!near.length) { o.area = o.ajar = o.full; continue; }
    const cols = c.cols.slice(); for (let q = 0; q < cols.length; q++) if (cols[q]) cols[q] = openAt(near, c.axis, c.a, c.b, c.s0 + (q + 0.5) * K.ds, c.nY, cols[q]);
    shutSlits(cols, c.nY); shutCracks(cols); o.area = count(cols) * K.ds * K.dy;   // (what a leaf leaves over its head or under its foot is a slit, as in a wall)
    if (o.area < K.small) o.area = 0;
    o.ajar = ajarOf(o, near);
  }
}

/** WHAT AN OPENING LOOKS OUT ON, as one radiance (linear RGB, the picture's units): half of what it sees is the sky, low down (common.wgsl skyRadiance's
 *  ramp at the `up` mix), and half the ground, lit by the whole sky (π × (0.947 zenith + 0.053 horizon), as engine.ts lookShade has it) and by the sun that
 *  is up, and sending back `ground` of it. At every hour: a night's sky is a sky. A PORTAL's record is this × what is open of it (render/lights.ts
 *  LIGHT_PORTAL): how bright the opening is to a pass whose ray stops short of it, and what the picture's direct light reads the ground back out of
 *  (common.wgsl portalGround: twice this less the low sky), putting the sky a point really looks out on in the sky half's place. */
export function openingLooks(zen: ArrayLike<number>, hor: ArrayLike<number>, skyK: number, sun: { intensity: number; dir: ArrayLike<number>; color: ArrayLike<number> } | undefined): Vec3 {
  const K = LOOK_OPENINGS, r: Vec3 = [0, 0, 0], sunE = sun && sun.dir[1] < 0 ? sun.intensity * -sun.dir[1] : 0;
  for (let i = 0; i < 3; i++) {
    const sky = skyK * (K.up * zen[i] + (1 - K.up) * hor[i]), groundE = Math.PI * skyK * (0.947 * zen[i] + 0.053 * hor[i]) + sunE * (sun ? sun.color[i] : 0);
    r[i] = 0.5 * (sky + K.ground * groundE / Math.PI);
  }
  return r;
}
/** … and A DISK's: the same × `amount`, BY DAY ONLY. It fades out as the sky's own mean radiance falls from 0.12 to 0.04 (every hour of the day is over
 *  that, a night's sky is under it: a night's rooms are the game's, as dark as they were, and an opening that would give next to nothing takes no place
 *  in the light list). */
export function openingRadiance(zen: ArrayLike<number>, hor: ArrayLike<number>, skyK: number, sun: { intensity: number; dir: ArrayLike<number>; color: ArrayLike<number> } | undefined, amount: number): Vec3 {
  const up = skyK * (0.2126 * (0.8 * zen[0] + 0.2 * hor[0]) + 0.7152 * (0.8 * zen[1] + 0.2 * hor[1]) + 0.0722 * (0.8 * zen[2] + 0.2 * hor[2])), t = Math.min(1, Math.max(0, (up - 0.04) / 0.08)), day = t * t * (3 - 2 * t);
  const r = openingLooks(zen, hor, skyK, sun), k = Math.max(0, amount) * day; r[0] *= k; r[1] *= k; r[2] *= k;
  return r;
}
