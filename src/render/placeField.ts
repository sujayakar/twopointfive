// THE LOOK PROTOTYPE'S PLACE FIELD — which PLACE every point of the floor plan is in, as one number: the light that falls on its floor (log2), the same
// from a room's window to its far end and another on the other side of its wall. The one exposure model (render/lookexpo.ts, shaders/place_common.wgsl)
// shows every place by that number against the number of the place Sam stands in. Before it the composite asked the radiance cache at each PIXEL (lookPlaceHere,
// four cells and their sight lines): a room's own fall-off from its window was then part of "the place", and bringing each spot up by its own darkness
// flattened the room, which is what the rows this model replaces kept correcting (the maintainer, 10-31: "i worry that we're finding these cases one by one
// and not fixing them systematically").
// WHAT LIVES HERE: the map (which cells are wall: the level's tall boxes, and a door's line while its leaf is shut; drawn again when a leaf moves), the field's
// buffer and its solve (shaders/place_field.wgsl), and Sam's eye, eased on the GPU from the field under his feet. PICTURE ONLY: nothing in the simulation
// reads it, it takes no die and no wall clock, and it imports nothing from src/game.
// THE BUFFER IS READ AS A UNIFORM by the composite: the fragment stage has no sampled texture left to bind (render/passes.ts compEntries).
// NEVER SEEN ON A GPU: judged on Chromium's software adapter, from stills (tools/qa/look-sheet.cjs) and the read-back map.
import { makeBuffer, makeShader, computePipelineAsync } from '../gpu/device';
import { Box, BoxFlag } from '../scene/boxes';
import { WORLD } from '../scene/world';
import placeSrc from './shaders/place_field.wgsl' with { type: 'text' };

/** cells a metre (0.4 m cells) and the map's size: place_field.wgsl and composite.wgsl PLACE_W / PLACE_H say the same (lookexpo-check compares) */
export const PLACE_RES = 2.5;
export const PLACE_W = Math.round(WORLD.rcSize[0] * PLACE_RES), PLACE_H = Math.round(WORLD.rcSize[2] * PLACE_RES);
/** the words after the cells: Sam's eye, 1 once the field is live, the open air's low level and its middle (place_field.wgsl placeEye) */
export const PLACE_WORDS = 4;
/** the buffer's size in bytes: the cells and the words after them, rounded up to whole vec4f (place_common.wgsl PlaceBuf) */
export const PLACE_BYTES = Math.ceil((PLACE_W * PLACE_H + PLACE_WORDS) / 4) * 16;
export const PLACE = {
  reach: 8,        // m: how far a place's light is smoothed (never through a wall): a room this size or smaller has one level
  link: 0.01,      // … and what of that pull crosses a doorway's cells: two rooms through an open door stay two places, with a soft step at the threshold
  doorCells: 4,    // an opening this many cells wide or less (1.6 m) is a doorway …
  doorDepth: 3,    // … when the way through it opens out within this many cells on both sides (a passage that stays narrow is a corridor, one place along its length)
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
 *  mullion), and not a decal or a glow card. Glass counts. placeWall's own test, and render/openings.ts's plan from the floor to the ceiling. */
export function placeSolid(b: Box): boolean {
  return !b.rot && (b.flags & HIDDEN) !== HIDDEN && Math.max(b.h[0], b.h[2]) >= PLACE.minWide && !((b.flags & BoxFlag.NoShadow) !== 0 && (b.flags & BoxFlag.Glass) === 0);
}
/** which slices of the height between PLACE.maxFoot and PLACE.minTop a box fills, as bits (0 = it is no part of a wall). A wall fills them all; a window is
 *  a sill, a pane and a lintel that fill them between them, so the map adds boxes up cell by cell. Glass counts (a window is a wall to the light of a
 *  place, whole or shot out: the light that comes through it is the room's own reading). Not a tilted box, not one a blast has hidden, not a decal or a
 *  glow card, not a post. A door's leaf is the one dynamic box that counts, whole (thin, wide and a door's height: a man is none of the three), and only to say whether it
 *  shuts its door (placeMask, leafShuts). */
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
/** the map of a list of boxes: 255 = wall; a free cell is 1 (a doorway's cell) or 2 (any other), + 4 when no roof is over it (row-major, j * PLACE_W + i).
 *  Pure: the check's too.
 *  A DOORWAY is a free run of PLACE.doorCells cells or fewer between two walls, where the way through it opens out wider within PLACE.doorDepth cells on
 *  both sides: the gap in a wall, however thick the wall. A corridor is as narrow but stays narrow, so it is not one, and neither is a niche. */
export type PlaceGap = readonly [number, number, number, number];
/** `gaps` = the level's DOORS as it knows them, each the line of its opening on the floor plan (hinge x, z to the far jamb x, z): the free cells on that line
 *  are doorway cells outright. The shapes alone could not say (the maintainer on his GPU, 11-01: "easy to get into a state where the exposure is off, like
 *  moving between these two rooms"): a leaf hanging at some angle beside its own opening made the gap a niche or a bend, and the two places it joins one. */
/** does leaf `b` SHUT the door whose opening is the line `gap`: it lies in that line, both its ends within PLACE.shutNear of it, its middle between the jambs */
export function leafShuts(b: Box, gap: PlaceGap): boolean {
  if (placeWall(b) !== FULL) return false;
  const [ax, az, bx, bz] = gap, dx = bx - ax, dz = bz - az, len = Math.hypot(dx, dz); if (!(len > 1e-3)) return false; const ux = dx / len, uz = dz / len;
  const c = Math.cos(b.yaw), sn = Math.sin(b.yaw), long = b.h[0] >= b.h[2], hl = long ? b.h[0] : b.h[2], lx = long ? c : sn, lz = long ? -sn : c;
  const off = (x: number, z: number) => Math.abs((x - ax) * uz - (z - az) * ux), t = (b.c[0] - ax) * ux + (b.c[2] - az) * uz;
  return t > 0 && t < len && off(b.c[0] + lx * hl, b.c[2] + lz * hl) <= PLACE.shutNear && off(b.c[0] - lx * hl, b.c[2] - lz * hl) <= PLACE.shutNear;
}
/** A DOOR IS A WALL EXACTLY WHEN ITS LEAF IS SHUT (11-03). A leaf was stamped into the map wherever it hung, as a wall. Shut, that is the door's own line,
 *  which the gap rule below shuts from jamb to jamb anyway. Open or part way through its swing it is a board standing in a room: it encloses nothing, both its
 *  sides are one place, and the cells it crossed were walls with no side to take a level from, shown by whatever level the cells round them held, whole
 *  (the maintainer on his GPU, the smoke lab's shed at dusk: "there's an odd blocky artifact near doors when they open and close"; on the software adapter
 *  a leaf half open had a pale square 0.8 m across at its foot). So a leaf that is not shut is not in the map at all, and the floor under it is read as the
 *  free floor it is. `dynamics` = the leaves, asked only whether they shut their doors. */
export function placeMask(statics: readonly Box[], dynamics: readonly Box[], out = new Uint8Array(PLACE_W * PLACE_H), gaps: readonly PlaceGap[] = []): Uint8Array {
  const W = PLACE_W, H = PLACE_H; out.fill(0);
  for (const b of statics) { const bits = placeWall(b); if (bits) stamp(out, bits, b.c[0], b.c[2], b.h[0], b.h[2], b.yaw); }
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
  const door = new Uint8Array(W * H);
  for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) { const k = j * W + i; if (out[k] === 255) continue;
    if ((wx[k] <= PLACE.doorCells && opens(wx, i, j, 0, 1, wx[k]) && opens(wx, i, j, 0, -1, wx[k])) || (wz[k] <= PLACE.doorCells && opens(wz, i, j, 1, 0, wz[k]) && opens(wz, i, j, -1, 0, wz[k]))) door[k] = 1; }
  for (const [ax, az, bx, bz] of gaps) {
    const dx = bx - ax, dz = bz - az, len = Math.hypot(dx, dz); if (!(len > 1e-3)) continue; const ux = dx / len, uz = dz / len, pad = PLACE.gapNear, cell = 1 / PLACE_RES, ox = WORLD.rcMin[0], oz = WORLD.rcMin[2];
    // A SHUT DOOR IS A WALL FROM JAMB TO JAMB. A leaf is a hand narrower than its opening, and where that crack lay over a cell's centre the map had a free
    // cell in the wall's own line beside the shut leaf: a doorway by its shape, cut off from both rooms, holding a level of its own, and shown as a pale block
    // on the floor and the wall there (the maintainer on his GPU, 11-01: "weird artifacts to the left of the door in overcast too", "doesn't show up in night
    // but does for dawn and dusk"; at night no place has a gain). A leaf that lies in its own line (both its ends within PLACE.shutNear of it) shuts the line.
    const shut = dynamics.some(b => leafShuts(b, [ax, az, bx, bz]));
    const i0 = Math.max(0, Math.floor((Math.min(ax, bx) - pad - ox) * PLACE_RES)), i1 = Math.min(W - 1, Math.floor((Math.max(ax, bx) + pad - ox) * PLACE_RES));
    const j0 = Math.max(0, Math.floor((Math.min(az, bz) - pad - oz) * PLACE_RES)), j1 = Math.min(H - 1, Math.floor((Math.max(az, bz) + pad - oz) * PLACE_RES));
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) { const px = ox + (i + 0.5) * cell - ax, pz = oz + (j + 0.5) * cell - az, t = px * ux + pz * uz;
      if (t >= -cell * 0.5 && t <= len + cell * 0.5 && Math.abs(px * uz - pz * ux) <= pad) { if (shut) out[j * W + i] = 255; else door[j * W + i] = 1; } }
  }
  for (let k = 0; k < W * H; k++) if (out[k] !== 255) out[k] = (door[k] ? 1 : 2) | (roof[k] ? 0 : 4);
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
  update(statics: readonly Box[], dynamics: readonly Box[], gaps: readonly PlaceGap[] = []): boolean {
    const key = placeKey(statics, dynamics, gaps); if (key === this.key) return false;
    this.key = key; placeMask(statics, dynamics, this.mask, gaps); this.drawn++; return true;
  }
}

const U_BYTES = 64;
export class PlaceField {
  /** what the composite binds as a uniform: PLACE_W × PLACE_H levels (log2; −1000 = wall), then PLACE_WORDS words: the eye's level, 1 once the field is live,
   *  THE OPEN AIR's LOW level on this map (PLACE.skyShare: the level a tenth of the cells under the open sky are darker than; a walled yard's and not open
   *  country's) and its MIDDLE (the level half of them are darker than) */
  readonly buffer: GPUBuffer;
  ready = false;
  readonly whenReady: Promise<void>;
  /** maps drawn and sweeps encoded since it was built (a tool's) */
  drawn = 0; swept = 0;
  readonly mask = new Uint8Array(PLACE_W * PLACE_H);
  private readonly device: GPUDevice;
  private readonly maskTex: GPUTexture; private readonly ubo: GPUBuffer[] = [];
  private sweep: GPUComputePipeline | null = null; private eye: GPUComputePipeline | null = null; private readonly bind: GPUBindGroup[] = [];
  private readonly own = new PlaceMap(); private sentMap: PlaceMap | null = null; private sentDrawn = -1; private dead = false; private fresh = true;
  private readonly bgl: GPUBindGroupLayout; private readonly maskView: GPUTextureView; private dice: GPUTextureView | null = null;
  private readonly u = new ArrayBuffer(U_BYTES); private readonly uf = new Float32Array(this.u); private readonly uu = new Uint32Array(this.u);

  constructor(device: GPUDevice, dice: GPUTextureView) {
    this.device = device; const B = GPUBufferUsage, C = GPUShaderStage.COMPUTE;
    this.buffer = makeBuffer(device, PLACE_BYTES, B.STORAGE | B.UNIFORM | B.COPY_DST | B.COPY_SRC, 'place field');
    this.maskTex = device.createTexture({ label: 'place map', size: [PLACE_W, PLACE_H], format: 'r8uint', usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST });
    const bgl = device.createBindGroupLayout({ label: 'place', entries: [
      { binding: 0, visibility: C, buffer: { type: 'uniform' } }, { binding: 1, visibility: C, texture: { sampleType: 'float', viewDimension: '3d' } },
      { binding: 3, visibility: C, texture: { sampleType: 'uint' } }, { binding: 4, visibility: C, buffer: { type: 'storage' } }] });
    const layout = device.createPipelineLayout({ label: 'place', bindGroupLayouts: [bgl] }), module = makeShader(device, placeSrc, 'place_field');
    this.whenReady = Promise.all([
      computePipelineAsync(device, { label: 'place sweep', layout, compute: { module, entryPoint: 'placeSweep' } }).then(p => { this.sweep = p; }),
      computePipelineAsync(device, { label: 'place eye', layout, compute: { module, entryPoint: 'placeEye' } }).then(p => { this.eye = p; }),
    ]).then(() => {
      // a compile the device refused comes back as an INVALID pipeline (gpu/device.ts computePipelineAsync), and setting one on the frame's encoder would void
      // the whole frame. So each is set once on an encoder of its own, under an error scope: an error there leaves the field off (the kernel's read by the
      // cache answers, as it does before this) and the frame is never handed either.
      const scoped = typeof device.pushErrorScope === 'function' && typeof device.popErrorScope === 'function' && typeof GPUValidationError !== 'undefined';
      if (!scoped) { this.ready = true; return; }
      device.pushErrorScope('validation');
      try { const enc = device.createCommandEncoder({ label: 'place pipelines tried' }), pass = enc.beginComputePass(); pass.setPipeline(this.sweep!); pass.setPipeline(this.eye!); pass.end(); device.queue.submit([enc.finish()]); } catch { /* the scope says */ }
      return Promise.resolve(device.popErrorScope()).then(err => {
        if (err instanceof GPUValidationError) { this.sweep = this.eye = null; console.warn('place field: not built on this device', err.message); } else this.ready = true;
      }, () => { /* a lost device: its own handler speaks */ });
    });
    this.bgl = bgl; this.maskView = this.maskTex.createView();
    for (let k = 0; k < 2; k++) this.ubo.push(makeBuffer(device, U_BYTES, B.UNIFORM | B.COPY_DST, `place uniforms ${k}`));   // (one uniform block a colour: both sweeps of a frame are encoded before either block could be written again)
    this.bindTo(dice);
    this.clear();
  }

  /** the cache's dice volume the field reads: a rebuilt cache (another preset) has another, and the two groups are made again for it */
  private bindTo(dice: GPUTextureView): void {
    if (this.dice === dice) return; this.dice = dice; this.bind.length = 0;
    for (let k = 0; k < 2; k++) this.bind.push(this.device.createBindGroup({ label: `place ${k}`, layout: this.bgl, entries: [
      { binding: 0, resource: { buffer: this.ubo[k] } }, { binding: 1, resource: dice }, { binding: 3, resource: this.maskView }, { binding: 4, resource: { buffer: this.buffer } }] }));
  }

  /** the map follows the level as it stands: drawn again only when a wall, a pane's box or a leaf has changed. True when it was drawn. */
  setBoxes(statics: readonly Box[], dynamics: readonly Box[], gaps: readonly PlaceGap[] = []): boolean { this.own.update(statics, dynamics, gaps); return this.setMap(this.own); }
  /** … from a map the caller keeps (the engine's, which the wall sides read too). True when it was sent. */
  setMap(map: PlaceMap): boolean {
    if (this.dead || (map === this.sentMap && map.drawn === this.sentDrawn)) return false;
    this.sentMap = map; this.sentDrawn = map.drawn; this.mask.set(map.mask); this.drawn++;
    this.device.queue.writeTexture({ texture: this.maskTex }, this.mask, { bytesPerRow: PLACE_W }, [PLACE_W, PLACE_H]); return true;
  }

  /** this frame's sweeps and the eye, encoded on `enc` AFTER the radiance cache's own pass (`dice` = its volume's view as it is this frame). `sam` = where Sam stands (world x, z; null = nobody: the eye
   *  keeps what it has); `snap` = the eye takes its reading outright, and the field is solved to rest first (a first frame, a restart, a still). */
  step(enc: GPUCommandEncoder, dice: GPUTextureView, dt: number, sam: readonly [number, number] | null, snap: boolean): void {
    if (this.dead || !this.sweep || !this.eye) return;
    this.bindTo(dice);
    const P = PLACE, first = this.fresh || snap; this.fresh = false;
    const kappa = (P.reach * PLACE_RES) ** 2, ease = (s: number) => (dt > 0 ? 1 - Math.exp(-Math.min(dt, 0.25) / Math.max(s, 1e-3)) : 0);
    for (let k = 0; k < 2; k++) {
      const f = this.uf, u = this.uu; u[0] = k; u[1] = first ? 1 : 0; f[2] = kappa; f[3] = P.omega;
      f[4] = sam ? (sam[0] - WORLD.rcMin[0]) * PLACE_RES : -1; f[5] = sam ? (sam[1] - WORLD.rcMin[2]) * PLACE_RES : -1; f[6] = ease(P.open); f[7] = ease(P.close);
      f[8] = Math.min(0.95, Math.max(0.05, (P.y - WORLD.rcMin[1]) / WORLD.rcSize[1])); f[9] = P.dark; f[10] = P.link; f[11] = ease(P.skyEase); f[12] = P.skyShare;
      this.device.queue.writeBuffer(this.ubo[k], 0, this.u);
    }
    const n = first ? P.settle : P.sweeps, gx = Math.ceil(PLACE_W / 8), gy = Math.ceil(PLACE_H / 8);
    const pass = enc.beginComputePass({ label: 'place' }); pass.setPipeline(this.sweep);
    for (let i = 0; i < n; i++) for (let k = 0; k < 2; k++) { pass.setBindGroup(0, this.bind[k]); pass.dispatchWorkgroups(gx, gy); }
    pass.setPipeline(this.eye); pass.setBindGroup(0, this.bind[0]); pass.dispatchWorkgroups(1);   // (every frame: it also keeps the live word and the brightest place)
    pass.end(); this.swept += n;
  }

  /** every cell without an answer and the eye without a reading: how the field starts. (A restart needs none of it: the snap that follows solves to rest from whatever is there.) */
  private clear(): void {
    const f = new Float32Array(PLACE_BYTES / 4).fill(-1000); f[PLACE_W * PLACE_H + 1] = 0;
    this.device.queue.writeBuffer(this.buffer, 0, f); this.fresh = true;
  }

  destroy(): void { if (this.dead) return; this.dead = true; this.buffer.destroy(); this.maskTex.destroy(); for (const b of this.ubo) b.destroy(); }

  /** the field read back (a tool's): PLACE_W × PLACE_H levels, then the eye, the live word and the brightest place */
  async readBack(): Promise<Float32Array> {
    if (this.dead) return new Float32Array(PLACE_BYTES / 4);
    const buf = this.device.createBuffer({ label: 'place read-back', size: PLACE_BYTES, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    const enc = this.device.createCommandEncoder(); enc.copyBufferToBuffer(this.buffer, 0, buf, 0, PLACE_BYTES); this.device.queue.submit([enc.finish()]);
    await buf.mapAsync(GPUMapMode.READ); const res = new Float32Array(buf.getMappedRange().slice(0)); buf.unmap(); buf.destroy(); return res;
  }
}
