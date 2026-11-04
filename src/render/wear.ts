// The TRAFFIC MAP the detail layer's floor terms read (renderer v3 phase 4; render/shaders/detail.wgsl, bound by the g-buffer at its group 1 binding 6):
// two bytes per texel over the world's grid footprint at WEAR_RES texels a metre —
//   r = TRAFFIC 0‥255: where the shift walks. Every patrol route's legs traced over the nav grid (the paths the men actually take, waypoint to waypoint and
//       back round), stamped with a walking lane's width, plus every door's threshold (everybody funnels through a doorway) — the polish there wears matte;
//   g = WALL PROXIMITY 0‥255: how close the texel is to something standing on the floor (walls, cabinets, counters, plinths — the statics whose foot is at
//       floor level), 255 at contact → 0 at GRIME_REACH: the band a mop does not reach.
//       ON THE LOOK'S PAGE (`longFoot`) it is the band where stones gather, and that is along a WALL: each face of a standing box lays its band × how long the
//       straight line of faces is that it is a piece of (faceRuns below). A wall's face runs for metres and has the band in full; a table's leg, a post, a
//       crate, a crate pushed against a wall have none. A band is a strip along something; round a thing put down it is a halo (the maintainer, of the look's
//       gravel that comes up in this band, 11-03: "circles on ground near table legs are weird"; his shots of a dressed yard on 11-04 show the same ring at
//       every crate and bench foot).
// A pure function of the level + its nav grid (nothing random: the same level draws the same wear); built once at boot by game.ts. The viewer / editor build
// none, so their g-buffer keeps its 1×1 zero and the floor terms do nothing there.
import type { Level } from '../scene/level';
import type { NavGrid } from '../scene/collision';
import { WORLD } from '../scene/world';
import { BoxFlag, type Box } from '../scene/boxes';

export const WEAR_RES = 8;          // texels per metre (12.5 cm: a lane's edge is soft over twice that anyway)
const LANE_SIGMA = 0.34;            // a walking lane's Gaussian half-width (m): two feet and some wander
const DOOR_SIGMA = 0.55;            // a threshold's blob (m)
const DOOR_AMP = 1.4;               // … and its weight against one route pass
const TRAFFIC_K = 1.1;              // saturation: traffic = 1 − exp(−K × passes) — two overlapping routes read as well worn, not twice as worn
const GRIME_REACH = 0.16;           // wall proximity falls to 0 this far from the foot of the wall (m)
const RUN_LO = 1.25, RUN_HI = 2.5;  // m of straight face (`longFoot`): under LO no band (a crate, a pallet, two crates side by side), from HI all of it (a hut's wall, a container)
const RUN_MEET = 0.03;              // m: two faces meet end to end within this
const RUN_STEP = 0.12;              // m: a face this far proud of another, or back from it, is still on its line (a wall's pillar, a plinth course, a board leant flat on it)
const RUN_TURN = 0.9986;            // … and faces the same way within 3° (the cosine)

export interface WearMap { data: Uint8Array; w: number; h: number }

/** `longFoot`: the wall band × how long the face is (the head of this file; the look's page asks for it, every other page has the band as it always was) */
export function buildWearMap(level: Level, nav: NavGrid | null, longFoot = false): WearMap {
  const w = Math.round(WORLD.gridW * WORLD.gridCell * WEAR_RES), h = Math.round(WORLD.gridH * WORLD.gridCell * WEAR_RES);
  const ox = WORLD.min[0], oz = WORLD.min[2];
  const T = new Float32Array(w * h);
  // ---- traffic: Gaussian stamps along polylines ----
  const stamp = (x: number, z: number, sigma: number, amp: number) => {
    const r = sigma * 2.5, s2 = 2 * sigma * sigma;
    const x0 = Math.max(0, Math.floor((x - r - ox) * WEAR_RES)), x1 = Math.min(w - 1, Math.ceil((x + r - ox) * WEAR_RES));
    const z0 = Math.max(0, Math.floor((z - r - oz) * WEAR_RES)), z1 = Math.min(h - 1, Math.ceil((z + r - oz) * WEAR_RES));
    for (let tz = z0; tz <= z1; tz++) {
      const pz = oz + (tz + 0.5) / WEAR_RES; const dz = pz - z;
      for (let tx = x0; tx <= x1; tx++) { const px = ox + (tx + 0.5) / WEAR_RES; const dx = px - x; const d2 = dx * dx + dz * dz; if (d2 <= r * r) T[tz * w + tx] += amp * Math.exp(-d2 / s2); }
    }
  };
  /** stamp a segment a→b every ~10 cm; `amp` is per metre walked so a leg's weight does not depend on the step */
  const walk = (ax: number, az: number, bx: number, bz: number, amp: number) => {
    const len = Math.hypot(bx - ax, bz - az); const n = Math.max(1, Math.ceil(len / 0.1)); const per = amp * (len / n) / (LANE_SIGMA * 2.5);
    for (let i = 0; i <= n; i++) { const t = i / n; stamp(ax + (bx - ax) * t, az + (bz - az) * t, LANE_SIGMA, per); }
  };
  for (const route of level.routes) {
    const pts = route.points; if (pts.length < 2) continue;
    for (let i = 0; i < pts.length; i++) {
      const a = pts[i], b = pts[(i + 1) % pts.length];   // the loop closes: the last waypoint walks back to the first
      const path = nav?.findPath(a, b, 6000) ?? null;
      const poly = path && path.length >= 2 ? path : [a, b];
      for (let k = 0; k + 1 < poly.length; k++) walk(poly[k][0], poly[k][2], poly[k + 1][0], poly[k + 1][2], 1);
      // a man stands and turns at each waypoint for his wait: a worn patch there
      stamp(a[0], a[2], LANE_SIGMA * 1.3, 0.35 + Math.min(1, (route.wait[i] ?? 0) / 20));
    }
  }
  for (const d of level.doors) {   // thresholds: the frame centre (doors.ts frameCentre), both faces
    const cx = d.hinge[0] + Math.cos(d.closedDir) * d.width * 0.5, cz = d.hinge[1] + Math.sin(d.closedDir) * d.width * 0.5;
    stamp(cx, cz, DOOR_SIGMA, DOOR_AMP);
  }
  const prox = longFoot ? bandByFaces(level.boxes, w, h, ox, oz) : bandByOccupancy(level.boxes, w, h, ox, oz);
  const data = new Uint8Array(w * h * 2);
  for (let i = 0; i < w * h; i++) {
    const traffic = 1 - Math.exp(-TRAFFIC_K * T[i]);
    data[i * 2] = Math.max(0, Math.min(255, Math.round(traffic * 255)));
    data[i * 2 + 1] = Math.max(0, Math.min(255, Math.round(prox[i] * 255)));
  }
  return { data, w, h };
}

/** a static whose foot is on the floor: not a glow card or a decal, not a slab or a rug, not something up on a wall */
const standsOn = (b: Box) => (b.flags & BoxFlag.NoShadow) === 0 && b.c[1] - b.h[1] <= 0.06 && b.c[1] + b.h[1] >= 0.25;

/** THE BAND AS EVERY PAGE HAS IT: 1 where anything stands, falling to 0 at GRIME_REACH from it. Not to be changed: the build's picture is this to the byte
 *  (`probes.ts materials-wear` holds its sum on the office). */
function bandByOccupancy(boxes: readonly Box[], w: number, h: number, ox: number, oz: number): Float64Array {
  // ---- wall proximity: occupancy of statics standing on the floor, then a 3-4 chamfer distance transform ----
  const occ = new Uint8Array(w * h);
  for (const b of boxes) {
    if (b.flags & BoxFlag.NoShadow) continue;                            // glow cards, decals
    const bottom = b.c[1] - b.h[1], top = b.c[1] + b.h[1];
    if (bottom > 0.06 || top < 0.25) continue;                           // not standing on the floor / a floor slab or a rug itself
    const c = Math.cos(b.yaw), s = Math.sin(b.yaw);
    const ex = Math.abs(c) * b.h[0] + Math.abs(s) * b.h[2], ez = Math.abs(s) * b.h[0] + Math.abs(c) * b.h[2];
    const x0 = Math.max(0, Math.floor((b.c[0] - ex - ox) * WEAR_RES)), x1 = Math.min(w - 1, Math.ceil((b.c[0] + ex - ox) * WEAR_RES));
    const z0 = Math.max(0, Math.floor((b.c[2] - ez - oz) * WEAR_RES)), z1 = Math.min(h - 1, Math.ceil((b.c[2] + ez - oz) * WEAR_RES));
    for (let tz = z0; tz <= z1; tz++) {
      const pz = oz + (tz + 0.5) / WEAR_RES - b.c[2];
      for (let tx = x0; tx <= x1; tx++) {
        const px = ox + (tx + 0.5) / WEAR_RES - b.c[0];
        const lx = c * px - s * pz, lz = s * px + c * pz;                 // world → box local (Box.yaw convention: local +x → (cos, 0, −sin))
        if (Math.abs(lx) <= b.h[0] + 0.5 / WEAR_RES && Math.abs(lz) <= b.h[2] + 0.5 / WEAR_RES) occ[tz * w + tx] = 1;
      }
    }
  }
  const INF = 1e6; const D = new Float32Array(w * h);
  for (let i = 0; i < w * h; i++) D[i] = occ[i] ? 0 : INF;
  const d1 = 1, d2 = Math.SQRT2;
  for (let z = 0; z < h; z++) for (let x = 0; x < w; x++) {              // forward pass
    const i = z * w + x; let v = D[i];
    if (x > 0) v = Math.min(v, D[i - 1] + d1); if (z > 0) { v = Math.min(v, D[i - w] + d1); if (x > 0) v = Math.min(v, D[i - w - 1] + d2); if (x < w - 1) v = Math.min(v, D[i - w + 1] + d2); }
    D[i] = v;
  }
  for (let z = h - 1; z >= 0; z--) for (let x = w - 1; x >= 0; x--) {    // backward pass
    const i = z * w + x; let v = D[i];
    if (x < w - 1) v = Math.min(v, D[i + 1] + d1); if (z < h - 1) { v = Math.min(v, D[i + w] + d1); if (x < w - 1) v = Math.min(v, D[i + w + 1] + d2); if (x > 0) v = Math.min(v, D[i + w - 1] + d2); }
    D[i] = v;
  }
  const prox = new Float64Array(w * h);
  for (let i = 0; i < w * h; i++) {
    const distM = Math.max(0, D[i] / WEAR_RES - 0.5 / WEAR_RES);          // texel centre → metres from the occupied texel's edge
    prox[i] = occ[i] ? 1 : Math.max(0, 1 - distM / GRIME_REACH);
  }
  return prox;
}

/** HOW LONG THE STRAIGHT LINE OF FACES IS THAT EACH FACE OF EACH BOX IS A PIECE OF (m; four a box: its +x, −x, +z, −z sides; 0 where the box does not stand on the
 *  floor). A wall is boxes in line, and what the floor sees of it is their faces: the piece between two windows, the sill under each, the pillar at the corner
 *  a hand proud of them. Asked of one face: every standing box's face that looks the same way (RUN_TURN) and lies within RUN_STEP of this face's own plane is on
 *  its line; laid end to end along it (RUN_MEET), the stretch this face is in is its run. Asked FACE BY FACE and never as groups: a crate at the end of a wall
 *  is on its pillar's line and the pillar on the wall's, and the crate is still not on the wall's. Exact from the boxes, whatever their yaw; a square box has
 *  no long way to be mistaken about. ONLY WHAT STANDS: a lintel does not carry a line over a doorway, so a piece of wall under RUN_LO between two doorways has
 *  no band. */
export function faceRuns(boxes: readonly Box[]): Float32Array {
  const out = new Float32Array(boxes.length * 4);
  // a face: a point in its middle, the way it looks (n), the way it runs (t), half its length, and where its number goes
  const F: { x: number; z: number; nx: number; nz: number; tx: number; tz: number; a: number; at: number }[] = [];
  boxes.forEach((b, i) => { if (!standsOn(b)) return;
    const c = Math.cos(b.yaw), s = Math.sin(b.yaw);   // (Box.yaw: local +x → (cos, 0, −sin), local +z → (sin, 0, cos))
    for (let k = 0; k < 4; k++) { const sg = k & 1 ? -1 : 1, onX = k < 2, nx = sg * (onX ? c : s), nz = sg * (onX ? -s : c), d = onX ? b.h[0] : b.h[2];
      F.push({ x: b.c[0] + nx * d, z: b.c[2] + nz * d, nx, nz, tx: onX ? s : c, tz: onX ? c : -s, a: onX ? b.h[2] : b.h[0], at: i * 4 + k }); } });
  // faces that look exactly one way are kept together, in the order of their planes: a face's line is then a few neighbours in its own list (a level's walls
  // look four ways and no more), plus every face of the lists within RUN_TURN of it
  type Face = typeof F[0]; const lists = new Map<number, { deg: number; d: number[]; f: Face[] }>();
  for (const f of F) { const deg = Math.atan2(f.nz, f.nx) * 180 / Math.PI, key = Math.round(deg * 1000); let l = lists.get(key); if (!l) lists.set(key, l = { deg, d: [], f: [] }); l.f.push(f); }
  const all = [...lists.values()], turn = Math.acos(RUN_TURN) * 180 / Math.PI + 0.002;
  for (const l of all) { const nx = l.f[0].nx, nz = l.f[0].nz; l.f.sort((a, b) => (a.x * nx + a.z * nz) - (b.x * nx + b.z * nz)); l.d = l.f.map(f => f.x * nx + f.z * nz); }
  const span: number[] = [];
  for (const own of all) for (let fi = 0; fi < own.f.length; fi++) { const f = own.f[fi];
    span.length = 0;
    const take = (g: Face) => {
      if (g === f || f.nx * g.nx + f.nz * g.nz < RUN_TURN) return;
      const dx = g.x - f.x, dz = g.z - f.z, off = dx * f.nx + dz * f.nz, lean = (g.tx * f.nx + g.tz * f.nz) * g.a;   // (how far g's middle is off f's plane, and its ends more)
      if (Math.abs(off) + Math.abs(lean) > RUN_STEP) return;
      const mid = dx * f.tx + dz * f.tz, len = Math.abs(g.tx * f.tx + g.tz * f.tz) * g.a; span.push(mid - len, mid + len);
    };
    for (let k = fi - 1; k >= 0 && own.d[fi] - own.d[k] <= RUN_STEP + 0.01; k--) take(own.f[k]);
    for (let k = fi + 1; k < own.f.length && own.d[k] - own.d[fi] <= RUN_STEP + 0.01; k++) take(own.f[k]);
    for (const l of all) { if (l === own) continue; const dd = Math.abs(l.deg - own.deg); if (Math.min(dd, 360 - dd) <= turn) for (const g of l.f) take(g); }
    // grow this face's own stretch by every stretch that touches it, until none does
    let lo = -f.a, hi = f.a;
    for (let grew = true; grew;) { grew = false;
      for (let k = 0; k < span.length; k += 2) { const s0 = span[k], s1 = span[k + 1]; if (s0 > hi + RUN_MEET || s1 < lo - RUN_MEET || (s0 >= lo && s1 <= hi)) continue; lo = Math.min(lo, s0); hi = Math.max(hi, s1); grew = true; } }
    out[f.at] = hi - lo;
  }
  return out;
}

/** THE BAND ON THE LOOK'S PAGE: each standing box lays its own, exact from its own rectangle (no occupancy, no transform: a turned wall has no steps), × how long
 *  the face is that the texel is nearest to; a texel keeps the most any box gives it, so a crate against a wall takes nothing from the wall's band beside it.
 *  A texel whose middle is within half a texel of the box is its foot and the band falls off from there, as the occupancy above has it. */
function bandByFaces(boxes: readonly Box[], w: number, h: number, ox: number, oz: number): Float64Array {
  const prox = new Float64Array(w * h), run = faceRuns(boxes), half = 0.5 / WEAR_RES, pad = GRIME_REACH + half;
  const weigh = (m: number) => { const t = Math.min(1, Math.max(0, (m - RUN_LO) / (RUN_HI - RUN_LO))); return t * t * (3 - 2 * t); };
  boxes.forEach((b, bi) => {
    if (!standsOn(b)) return;
    const k = [weigh(run[bi * 4]), weigh(run[bi * 4 + 1]), weigh(run[bi * 4 + 2]), weigh(run[bi * 4 + 3])]; if (Math.max(k[0], k[1], k[2], k[3]) <= 0) return;
    const c = Math.cos(b.yaw), s = Math.sin(b.yaw);
    const ex = Math.abs(c) * b.h[0] + Math.abs(s) * b.h[2] + pad, ez = Math.abs(s) * b.h[0] + Math.abs(c) * b.h[2] + pad;
    const x0 = Math.max(0, Math.floor((b.c[0] - ex - ox) * WEAR_RES)), x1 = Math.min(w - 1, Math.ceil((b.c[0] + ex - ox) * WEAR_RES));
    const z0 = Math.max(0, Math.floor((b.c[2] - ez - oz) * WEAR_RES)), z1 = Math.min(h - 1, Math.ceil((b.c[2] + ez - oz) * WEAR_RES));
    for (let tz = z0; tz <= z1; tz++) {
      const pz = oz + (tz + 0.5) / WEAR_RES - b.c[2];
      for (let tx = x0; tx <= x1; tx++) {
        const px = ox + (tx + 0.5) / WEAR_RES - b.c[0];
        const lx = c * px - s * pz, lz = s * px + c * pz, dx = Math.abs(lx) - b.h[0], dz = Math.abs(lz) - b.h[2];   // world → box local; metres out from each pair of sides
        const face = dx >= dz ? (lx >= 0 ? 0 : 1) : (lz >= 0 ? 2 : 3);                                             // the side the texel is nearest to
        const v = k[face] * Math.max(0, 1 - Math.max(0, Math.hypot(Math.max(dx, 0), Math.max(dz, 0)) - half) / GRIME_REACH);
        if (v > prox[tz * w + tx]) prox[tz * w + tx] = v;
      }
    }
  });
  return prox;
}
