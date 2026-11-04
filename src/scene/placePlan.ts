// THE FLOOR PLAN OF A LEVEL AS IT STANDS — which cells of its floor are wall, which are a doorway's, which have a roof over them, on one grid of 0.4 m
// cells: what walls, panes and shut doors enclose is a PLACE. Plain arithmetic on the level's boxes, with no GPU, no die and no clock, so a server can run
// it and the game's own CPU light may read it (the maintainer, 11-04, of daylight indoors: the game is moving to read light measured on the CPU, and what
// a guard senses through a door must come from the same openings the picture draws). Until 11-04 this lived in render/placeField.ts, beside the GPU field
// that is solved on it; that file keeps the field, its buffer and Sam's eye, and reads the plan from here.
// WHO READS IT: render/placeField.ts (the light of a place, the look's exposure), render/wallSides.ts (which side of a wall a point is on), scene/openings.ts
// (where a place is open to the sky), scene/geoTry.ts by main.ts (a yard wall has the sky on both hands), and the checks of each.
// PLACE holds the plan's numbers AND the field's solve numbers in one table, as it always did (a tool hands one table of both: tools/qa/place-field-page.cjs
// --place); the solve's are read by render/placeField.ts alone.
import { Box, BoxFlag } from './boxes';
import { WORLD } from './world';
import { type Pt, apart, narrowest, footprint, WAY } from './wayThrough';

/** cells a metre (0.4 m cells) and the map's size: place_field.wgsl and composite.wgsl PLACE_W / PLACE_H say the same (lookexpo-check compares) */
export const PLACE_RES = 2.5;
export const PLACE_W = Math.round(WORLD.rcSize[0] * PLACE_RES), PLACE_H = Math.round(WORLD.rcSize[2] * PLACE_RES);
export const PLACE = {
  reach: 8,        // m: how far a place's light is smoothed (never through a wall): a room this size or smaller has one level
  link: 0.01,      // … and what of that pull crosses a doorway's cells: two rooms through an open door stay two places, with a soft step at the threshold
  doorCells: 4,    // an opening this many cells wide or less (1.6 m) is a doorway …
  doorDepth: 3,    // … when the way through it opens out within this many cells on both sides (a passage that stays narrow is a corridor, one place along its length)
  near: 0.5,       // stops a cell (1.25 a metre): how fast THE BRIGHTEST PLACE A STRIDE AWAY dims over floor. Slower than a doorway's own light falls off into the room, or the gain would rise inward faster than the light falls and a strip of floor a stride in would be drawn brighter than the threshold (at 1 a software draw had that strip). At a half, a room five stops under its yard is its own again four metres in
  nearDoor: 0.125, // … and across a doorway's cell: little, but never nothing (place_field.wgsl hand: with no loss there a doorway's two cells keep the brightest answer either ever held)
  omega: 1.93,     // the solve's over-relaxation
  sweeps: 4,       // whole sweeps (both colours) a drawn frame
  settle: 160,     // … and in one go when the eye is told to take its reading outright (a first frame, a restart, a still)
  dark: 0.02,      // what a probe that holds nothing counts for against one that holds light (a probe inside a crate; a black room)
  minTop: 1.9,     // m: a cell is wall when boxes fill it from maxFoot up to here, one box or several (a sill, its pane and the lintel over it)
  maxFoot: 0.45,
  minWide: 0.2,    // m: the half-length a box must have to be part of a wall (a post, a pipe and a mullion are not; a 0.5 m pier between a door and a window is, and a pane 0.6 m wide: at 0.3 the pass's kitchen was one place with its yard once its door stood wide)
  gapNear: 0.2001, // m: a free cell this near a door's line, between its jambs, is a doorway's cell whatever the map makes of the shapes round it (placeMask `gaps`). Half a cell and a hair: the very cells a shut leaf fills, so a shut door has no doorway cell beside it. At 0.3 it had a row of them on one side or both, each cut off from its room with a level of its own, and the room's exposure showed that row of floor and the wall over it as a pale block by every shut door (the maintainer on his GPU, 11-01: "weird artifacts to the left of the door in overcast too", "doesn't show up in night but does for dawn and dusk")
  roofWide: 0.45,  // m: the half-length, both ways, a box over head height must have to be a roof over its cells (weather.ts roofsOf's own: a ceiling slab; a beam and a lintel are not)
  skyShare: 0.1,   // THE OPEN AIR's level is the one this share of the cells under the open sky are darker than (the walled yard, not the snow field past the wall and not one dark nook)
  skyEase: 0.5,    // s: that level follows a change of light by this (it is never told to the picture in a step)
  y: 1.1,          // m: the height the cache's light on a face that looks up is read at: the layer a table top is in (the cache's second of four, 0.75 to 1.5 m; the third holds the cell a hanging bulb is in, and read a bulb's room several times too bright)
  open: 0.9,       // s for the eye to open (Sam walks into the dark) …
  close: 0.35,     // … and to close: faster, as an eye is
  shutNear: 0.08,  // m: a leaf whose two ends are both this near its door's line is shut, and the line is a wall from jamb to jamb (placeMask)
  leafStep: 0.04,  // rad: a leaf must have swung this far for the map to be drawn again
};
const HIDDEN = BoxFlag.NoPrimary | BoxFlag.NoShadow;

const SLICES = 8, FULL = (1 << SLICES) - 1;
/** is a box of the level one a plan of its walls counts at all, whatever its height: upright, drawn or shadowing, long enough one way (not a post, a pipe or a
 *  mullion), and not a decal or a glow card. Glass counts. placeWall's own test, and scene/openings.ts's plan from the floor to the ceiling. */
export function placeSolid(b: Box): boolean {
  return !b.rot && (b.flags & HIDDEN) !== HIDDEN && Math.max(b.h[0], b.h[2]) >= PLACE.minWide && !((b.flags & BoxFlag.NoShadow) !== 0 && (b.flags & BoxFlag.Glass) === 0);
}
/** which slices of the height between PLACE.maxFoot and PLACE.minTop a box fills, as bits (0 = it is no part of a wall). A wall fills them all; a window is
 *  a sill, a pane and a lintel that fill them between them, so the map adds boxes up cell by cell. Glass counts (a window is a wall to the light of a
 *  place, whole or shot out: the light that comes through it is the room's own reading). Not a tilted box, not one a blast has hidden, not a decal or a
 *  glow card, not a post. A door's leaf is the one dynamic box that counts, whole, where it hangs (thin, wide and a door's height: a man is none of the three). */
export function placeWall(b: Box): number {
  if (b.rot || (b.flags & HIDDEN) === HIDDEN) return 0;
  const y0 = b.c[1] - b.h[1], y1 = b.c[1] + b.h[1], wide = Math.max(b.h[0], b.h[2]);
  if ((b.flags & BoxFlag.Dynamic) !== 0) return y1 >= PLACE.minTop && y0 <= PLACE.maxFoot && y1 - y0 >= 1.7 && Math.min(b.h[0], b.h[2]) <= 0.08 && wide >= 0.3 ? FULL : 0;
  if (!placeSolid(b)) return 0;
  let bits = 0; const step = (PLACE.minTop - PLACE.maxFoot) / SLICES;
  for (let k = 0; k < SLICES; k++) { const y = PLACE.maxFoot + (k + 0.5) * step; if (y0 - 0.02 <= y && y <= y1 + 0.02) bits |= 1 << k; }
  return bits;
}
/** OR `bits` into the cells of one box's footprint (render-side twin of game/airSim.ts stamp: a box thinner than a cell is padded to one, so a wall never leaks) */
export function stampCells(cx: number, cz: number, hx: number, hz: number, yaw: number, each: (k: number) => void): void {
  const cell = 1 / PLACE_RES, c = Math.cos(yaw), s = Math.sin(yaw), square = Math.abs(c * s) < 0.02;
  const need = cell * (square ? 0.5 : 0.75) + 1e-4, ex = Math.max(hx, need), ez = Math.max(hz, need);
  const rx = Math.abs(c) * ex + Math.abs(s) * ez, rz = Math.abs(s) * ex + Math.abs(c) * ez, ox = WORLD.rcMin[0], oz = WORLD.rcMin[2];
  const i0 = Math.max(0, Math.floor((cx - rx - ox) * PLACE_RES)), i1 = Math.min(PLACE_W - 1, Math.floor((cx + rx - ox) * PLACE_RES));
  const j0 = Math.max(0, Math.floor((cz - rz - oz) * PLACE_RES)), j1 = Math.min(PLACE_H - 1, Math.floor((cz + rz - oz) * PLACE_RES));
  for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) {
    const px = ox + (i + 0.5) * cell - cx, pz = oz + (j + 0.5) * cell - cz, lx = c * px - s * pz, lz = s * px + c * pz;
    if (Math.abs(lx) <= ex && Math.abs(lz) <= ez) each(j * PLACE_W + i);
  }
}
function stamp(m: Uint8Array, bits: number, cx: number, cz: number, hx: number, hz: number, yaw: number): void { stampCells(cx, cz, hx, hz, yaw, k => { m[k] |= bits; }); }
/** is this box a roof over the cells it covers: a slab over head height, wide both ways, not tilted, not one a blast has hidden (a roof the camera cuts away
 *  is still a roof: it is hidden from the eye alone) */
export function placeRoof(b: Box): boolean {
  return !b.rot && (b.flags & HIDDEN) !== HIDDEN && (b.flags & BoxFlag.Dynamic) === 0 && b.c[1] - b.h[1] >= PLACE.minTop && Math.min(b.h[0], b.h[2]) >= PLACE.roofWide;
}
/** a leaf as the plan asks after it: where it is, how far it reaches, its footprint */
export interface PlaceLeaf { c: readonly [number, number]; r: number; o: Pt[] }
/** the leaves among a list of moving boxes (placeWall's own test of what a leaf is: thin, wide and a door's height) */
export function leafOutlines(dynamics: readonly Box[]): PlaceLeaf[] {
  const out: PlaceLeaf[] = [];
  for (const b of dynamics) if (placeWall(b) === FULL) out.push({ c: [b.c[0], b.c[2]], r: Math.hypot(b.h[0], b.h[2]), o: footprint(b.c[0], b.c[2], b.h[0], b.h[2], b.yaw) });
  return out;
}
/** is a box of the level something a door's leaf shuts against: upright, drawn or shadowing, not a decal or a glow card, and standing from the floor to a
 *  door's head (PLACE.maxFoot ‥ PLACE.minTop), however narrow. A pier counts where placeWall wants a wall's width; a crate, a shelf under a door's height
 *  and a lintel do not. Glass counts. */
export function placeJamb(b: Box): boolean {
  return !b.rot && (b.flags & HIDDEN) !== HIDDEN && (b.flags & BoxFlag.Dynamic) === 0 && !((b.flags & BoxFlag.NoShadow) !== 0 && (b.flags & BoxFlag.Glass) === 0) && b.c[1] - b.h[1] <= PLACE.maxFoot && b.c[1] + b.h[1] >= PLACE.minTop;
}
/** EACH DOOR'S JAMBS: the footprints of the level's own boxes a leaf shuts against (placeJamb) that stand at either end of its line (within the play of
 *  a shut leaf, WAY.play, of the end: a leaf is a hand narrower than its doorway). Nothing here knows what a jamb is: it is the wall that is there. One
 *  list a door, both ends together. */
export function doorJambs(statics: readonly Box[], gaps: readonly PlaceGap[]): Pt[][][] {
  const out: Pt[][][] = gaps.map(() => []), play = WAY.play;
  for (const b of statics) { if (!placeJamb(b)) continue; const r = Math.hypot(b.h[0], b.h[2]) + play; let o: Pt[] | null = null;
    gaps.forEach((g, n) => { for (let e = 0; e < 4; e += 2) { const x = g[e], z = g[e + 1]; if (Math.abs(x - b.c[0]) > r || Math.abs(z - b.c[2]) > r) continue;
      o ??= footprint(b.c[0], b.c[2], b.h[0], b.h[2], b.yaw); if (apart(o, [[x, z], [x, z]]) <= play) { out[n].push(o); break; } } }); }
  return out;
}
/** HOW OPEN A DOORWAY IS, 0‥1: the narrowest way through it past whatever hangs in it, over the way through with nothing hung (scene/wayThrough.ts: the
 *  shortest chain of gaps from one end of the door's line to the other over its jambs and the leaves near it, each gap less WAY.play: the measure a portal's
 *  open share is taken by, scene/openings.ts ajarOf). So one leaf is open by how far its free edge stands from the jamb it shuts on, the wall's own
 *  depth counted, and a leaf swung clear leaves the doorway whole. 1 with no leaf near. (Two leaves that meet in one doorway are two doors here, each
 *  measured to the point they meet at: no level has such a pair.) */
export function doorwayOpen(gap: PlaceGap, leaves: readonly PlaceLeaf[], jambs: readonly Pt[][] = []): number {
  const [ax, az, bx, bz] = gap, len = Math.hypot(bx - ax, bz - az); if (!(len > 1e-3)) return 1;
  const mx = (ax + bx) / 2, mz = (az + bz) / 2, hung: Pt[][] = [];
  for (const l of leaves) if (Math.hypot(l.c[0] - mx, l.c[1] - mz) <= len * 0.5 + l.r + 0.3) hung.push(l.o);
  if (!hung.length) return 1;
  const A: Pt[] = [[ax, az], [ax, az]], B: Pt[] = [[bx, bz], [bx, bz]], play = WAY.play, whole = Math.min(len - play, narrowest(A, B, jambs, play));
  return Math.max(0, Math.min(1, narrowest(A, B, [...jambs, ...hung], play) / Math.max(whole, 1e-3)));
}
/** A DOORWAY'S OPENNESS AS THE MAP CARRIES IT: DOOR_STEPS steps of a DOOR_PER_STOP-th of a stop under wide open (so the top step is a doorway whole,
 *  the least is about a thirty-second of it, and a step is the same small change of brightness wherever it falls); 0 = shut, the line is a wall.
 *  place_field.wgsl's constants of the same names and doorShut read them (lookexpo-check compares). */
export const DOOR_STEPS = 31, DOOR_PER_STOP = 6;
export function doorStep(open: number): number { return open > 0 ? Math.max(0, Math.min(DOOR_STEPS, Math.round(DOOR_STEPS + DOOR_PER_STOP * Math.log2(open)))) : 0; }
/** every door's step as the level stands (`jambs`: doorJambs of the same statics and gaps, where the caller keeps them) */
export function doorSteps(statics: readonly Box[], dynamics: readonly Box[], gaps: readonly PlaceGap[], jambs: readonly Pt[][][] = doorJambs(statics, gaps)): number[] {
  const leaves = leafOutlines(dynamics); return gaps.map((g, n) => doorStep(doorwayOpen(g, leaves, jambs[n])));
}
/** the map of a list of boxes: 255 = wall; a free cell is 1 (a doorway's cell) or 2 (any other), + 4 when no roof is over it (row-major, j * PLACE_W + i).
 *  BY DEGREES (`steps` given: each door's doorSteps, the look panel's "Doors open by degrees" row): A LEAF IS NEVER A WALL WHERE IT HANGS, and a door's
 *  cells carry HOW OPEN IT IS in the bits above (× 8: its step, 1‥DOOR_STEPS; 0 there = a doorway by its shape alone, wide open). A door at step 0 is
 *  shut: a wall from jamb to jamb.
 *  THE CALLER'S PART: `steps` has one entry a gap, and `gaps` names every door whose leaf is among `dynamics` (a leaf with no gap is then nowhere in
 *  the map, its doorway open by its shape; a gap with no step is shut).
 *  Without it a leaf is stamped as wall cells where it stands, as it was: a leaf swinging through 54 degrees walled off the cell inside its own door for a
 *  moment, and the place's bound to the yard's level came and went in a frame (the maintainer's videos of 11-04: "some odd brightness swinging").
 *  Pure: the check's too.
 *  A DOORWAY is a free run of PLACE.doorCells cells or fewer between two walls, where the way through it opens out wider within PLACE.doorDepth cells on
 *  both sides: the gap in a wall, however thick the wall. A corridor is as narrow but stays narrow, so it is not one, and neither is a niche. */
export type PlaceGap = readonly [number, number, number, number];
/** `gaps` = the level's DOORS as it knows them, each the line of its opening on the floor plan (hinge x, z to the far jamb x, z): the free cells on that line
 *  are doorway cells outright. The shapes alone could not say (the maintainer on his GPU, 11-01: "easy to get into a state where the exposure is off, like
 *  moving between these two rooms"): a leaf hanging at some angle beside its own opening made the gap a niche or a bend, and the two places it joins one. */
export function placeMask(statics: readonly Box[], dynamics: readonly Box[], out = new Uint8Array(PLACE_W * PLACE_H), gaps: readonly PlaceGap[] = [], steps: readonly number[] | null = null): Uint8Array {
  const W = PLACE_W, H = PLACE_H; out.fill(0);
  for (const b of statics) { const bits = placeWall(b); if (bits) stamp(out, bits, b.c[0], b.c[2], b.h[0], b.h[2], b.yaw); }
  if (!steps) for (const b of dynamics) { const bits = placeWall(b); if (bits) stamp(out, bits, b.c[0], b.c[2], b.h[0], b.h[2], b.yaw); }
  // the cells on a door's line, between its jambs (PLACE.gapNear)
  const onLine = (g: PlaceGap, each: (k: number) => void): void => { const [ax, az, bx, bz] = g, dx = bx - ax, dz = bz - az, len = Math.hypot(dx, dz); if (!(len > 1e-3)) return;
    const ux = dx / len, uz = dz / len, pad = PLACE.gapNear, cell = 1 / PLACE_RES, ox = WORLD.rcMin[0], oz = WORLD.rcMin[2];
    const i0 = Math.max(0, Math.floor((Math.min(ax, bx) - pad - ox) * PLACE_RES)), i1 = Math.min(W - 1, Math.floor((Math.max(ax, bx) + pad - ox) * PLACE_RES));
    const j0 = Math.max(0, Math.floor((Math.min(az, bz) - pad - oz) * PLACE_RES)), j1 = Math.min(H - 1, Math.floor((Math.max(az, bz) + pad - oz) * PLACE_RES));
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) { const px = ox + (i + 0.5) * cell - ax, pz = oz + (j + 0.5) * cell - az, t = px * ux + pz * uz;
      if (t >= -cell * 0.5 && t <= len + cell * 0.5 && Math.abs(px * uz - pz * ux) <= pad) each(j * W + i); } };
  // BY DEGREES: a shut door's line is a wall BEFORE the shapes are read (as a leaf stamped in its frame was)
  if (steps) gaps.forEach((g, n) => { if (!steps[n]) onLine(g, k => { out[k] = FULL; }); });
  for (let k = 0; k < W * H; k++) out[k] = out[k] === FULL ? 255 : 2;
  const roof = new Uint8Array(W * H); for (const b of statics) if (placeRoof(b)) stamp(roof, 1, b.c[0], b.c[2], b.h[0], b.h[2], b.yaw);
  // how wide the free run through each cell is, across x and across z (a run that reaches the map's edge is open: 255)
  const wx = new Uint8Array(W * H), wz = new Uint8Array(W * H);
  for (let j = 0; j < H; j++) for (let i = 0; i < W;) { if (out[j * W + i] === 255) { i++; continue; } let e = i; while (e < W && out[j * W + e] !== 255) e++; const n = i === 0 || e === W ? 255 : Math.min(254, e - i); for (let k = i; k < e; k++) wx[j * W + k] = n; i = e; }
  for (let i = 0; i < W; i++) for (let j = 0; j < H;) { if (out[j * W + i] === 255) { j++; continue; } let e = j; while (e < H && out[e * W + i] !== 255) e++; const n = j === 0 || e === H ? 255 : Math.min(254, e - j); for (let k = j; k < e; k++) wz[k * W + i] = n; j = e; }
  // does the way out of cell (i, j) along (di, dj) open out wider than `w` within doorDepth cells (the map's edge is open; a wall first is a niche)
  const opens = (wide: Uint8Array, i: number, j: number, di: number, dj: number, w: number): boolean => {
    for (let k = 1; k <= PLACE.doorDepth; k++) { const x = i + di * k, z = j + dj * k; if (x < 0 || z < 0 || x >= W || z >= H) return true; if (out[z * W + x] === 255) return false; if (wide[z * W + x] > w) return true; }
    return false; };
  const door = new Uint8Array(W * H), deg = steps ? new Uint8Array(W * H) : null;
  for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) { const k = j * W + i; if (out[k] === 255) continue;
    if ((wx[k] <= PLACE.doorCells && opens(wx, i, j, 0, 1, wx[k]) && opens(wx, i, j, 0, -1, wx[k])) || (wz[k] <= PLACE.doorCells && opens(wz, i, j, 1, 0, wz[k]) && opens(wz, i, j, -1, 0, wz[k]))) door[k] = 1; }
  gaps.forEach((g, n) => {
    const [ax, az, bx, bz] = g, dx = bx - ax, dz = bz - az, len = Math.hypot(dx, dz); if (!(len > 1e-3)) return; const ux = dx / len, uz = dz / len;
    // A SHUT DOOR IS A WALL FROM JAMB TO JAMB. A leaf is a hand narrower than its opening, and where that crack lay over a cell's centre the map had a free
    // cell in the wall's own line beside the shut leaf: a doorway by its shape, cut off from both rooms, holding a level of its own, and shown as a pale block
    // on the floor and the wall there (the maintainer on his GPU, 11-01: "weird artifacts to the left of the door in overcast too", "doesn't show up in night
    // but does for dawn and dusk"; at night no place has a gain). A leaf that lies in its own line (both its ends within PLACE.shutNear of it) shuts the line.
    // BY DEGREES shut is what the way through says, the same measure that says how open (and its line is a wall already, above).
    if (deg) { if (steps![n]) onLine(g, k => { door[k] = 1; deg[k] = steps![n]; }); return; }
    let shut = false;
    for (const b of dynamics) { if (placeWall(b) !== FULL) continue; const c = Math.cos(b.yaw), sn = Math.sin(b.yaw), long = b.h[0] >= b.h[2], hl = long ? b.h[0] : b.h[2], lx = long ? c : sn, lz = long ? -sn : c;
      const off = (x: number, z: number) => Math.abs((x - ax) * uz - (z - az) * ux), t = (b.c[0] - ax) * ux + (b.c[2] - az) * uz;
      if (t > 0 && t < len && off(b.c[0] + lx * hl, b.c[2] + lz * hl) <= PLACE.shutNear && off(b.c[0] - lx * hl, b.c[2] - lz * hl) <= PLACE.shutNear) { shut = true; break; } }
    onLine(g, k => { if (shut) out[k] = 255; else door[k] = 1; });
  });
  for (let k = 0; k < W * H; k++) if (out[k] !== 255) out[k] = (door[k] ? 1 : 2) | (roof[k] ? 0 : 4) | (deg && door[k] ? deg[k] << 3 : 0);
  return out;
}
/** "is this spot free floor under the open sky?" on the plan of a list of boxes as they stand, for a rule that asks before the world is filled (scene/geoTry.ts:
 *  a yard wall is a wall with the sky on both hands). Off the map is the open air. */
export function skyPlan(statics: readonly Box[]): (x: number, z: number) => boolean {
  const m = placeMask(statics, []);
  return (x, z) => { const i = Math.floor((x - WORLD.rcMin[0]) * PLACE_RES), j = Math.floor((z - WORLD.rcMin[2]) * PLACE_RES); if (i < 0 || j < 0 || i >= PLACE_W || j >= PLACE_H) return true; const v = m[j * PLACE_W + i]; return v !== 255 && (v & 4) !== 0; };
}
/** a number that changes when the map would: every box that is part of a wall, and each leaf's swing */
export function maskKey(statics: readonly Box[], dynamics: readonly Box[]): number {
  let key = statics.length * 31 | 0;
  for (const b of statics) { const bits = placeWall(b) || (placeRoof(b) ? 256 : 0); if (!bits) continue; key = (Math.imul(key, 33) + bits + Math.round((b.c[0] + b.h[0] * 3 + b.c[2] * 7 + b.h[2] * 13 + b.yaw * 23) * 50)) | 0; }
  for (const b of dynamics) { if (!placeWall(b)) continue; key = (Math.imul(key, 33) + Math.round(b.yaw / PLACE.leafStep) * 7 + Math.round((b.c[0] * 3 + b.c[2] * 7) * 10)) | 0; }
  return key;
}

/** a number that changes when a leaf has moved at all (to a quarter of a hundredth of a radian, half a centimetre): PlaceMap's, by degrees, for "are the steps still the steps" */
function leafKey(dynamics: readonly Box[]): number {
  let key = 17; for (const b of dynamics) { if (placeWall(b) !== FULL) continue; key = (Math.imul(key, 33) + Math.round(b.yaw * 400) * 7 + Math.round((b.c[0] * 3 + b.c[2] * 7) * 200)) | 0; }
  return key;
}
/** … and each door's line (the map's whole key: PlaceMap's, and the part of wallSides.ts sidesKey that is not a leaf) */
export function placeKey(statics: readonly Box[], dynamics: readonly Box[], gaps: readonly PlaceGap[]): number {
  let key = maskKey(statics, dynamics); for (const g of gaps) key = (Math.imul(key, 33) + Math.round((g[0] * 3 + g[1] * 7 + g[2] * 13 + g[3] * 23) * 50)) | 0; return key;
}
/** the map of the level as it stands, drawn again only when its key moves. ONE a frame, whoever reads it (the field below, render/wallSides.ts): each
 *  kept its own, keyed and drawn twice. */
export class PlaceMap {
  readonly mask = new Uint8Array(PLACE_W * PLACE_H);
  /** times it was drawn */
  drawn = 0;
  private key = NaN;
  private wallKey = NaN; private jambs: Pt[][][] = []; private leafKey = NaN; private steps: number[] = [];
  /** `byDegrees`: no leaf is a wall and each door's line says how open it is (placeMask `steps`). The map is then its walls, its doors' lines and their
   *  steps, so it is drawn again when one of those moves and not for a leaf's swing between two steps. The jambs are found again only with the walls,
   *  and the steps worked out again only when a leaf has moved (a level at rest costs its two keys). */
  update(statics: readonly Box[], dynamics: readonly Box[], gaps: readonly PlaceGap[] = [], byDegrees = false): boolean {
    let key = placeKey(statics, byDegrees ? [] : dynamics, gaps), steps: number[] | null = null;
    if (byDegrees) { if (key !== this.wallKey) { this.wallKey = key; this.jambs = doorJambs(statics, gaps); this.leafKey = NaN; }
      const lk = leafKey(dynamics); if (lk !== this.leafKey) { this.leafKey = lk; this.steps = doorSteps(statics, dynamics, gaps, this.jambs); }
      steps = this.steps; key = (Math.imul(key, 33) + 7) | 0; for (const s of steps) key = (Math.imul(key, 33) + s) | 0; }
    if (key === this.key) return false;
    this.key = key; placeMask(statics, dynamics, this.mask, gaps, steps); this.drawn++; return true;
  }
}
