// WHICH SIDE EACH DOOR OPENS INTO (doors.ts DOORWAY — the maintainer, 10-08: "doors could use some work too. only open one direction"). A door's leaf has one side it
// opens into (Door.swing); a level file may say which (`swing` on the door) and almost none does, so it is DEALT here, once, when the Game is built — from what
// the level already says about its rooms, by the rule the goal set: INTO THE ROOM IT SERVES, AWAY FROM THE CORRIDOR. In order, the first that speaks:
//   authored     the door's own `swing`
//   folds        a stop authored past a right angle and a bit on one side only (the break room's lot door folds flat against the outside wall): that side
//   outside      (DOORWAY.exitsOut — the maintainer, 10-08: "for doors just make it realistic") a door the level marks `exterior` with a ROOF over one face and open
//                sky over the other — a fire exit, a building's door onto its lot or yard: OUTWARD, toward the open air, as an exit door is hung. Dealt BESIDE
//                the rules below, not instead of them (SwingDeal.out, Door.outward): the switch picks between the two live. It does not speak for a door roofed
//                on both faces (a stair tower's) or on neither (a gate in a yard wall), nor against an authored `swing` or a folding stop
//   plan         a drill plan for this door (Level.breach / clearPlans): the way its `fling` throws the leaf — into the plan's room
//   room         the room-clear structure lists it as a door of one room: into that room; of two rooms (a door between them): into the smaller
//   hostages     the rescue's room door / side door: into the hostage room
//   mission      the mission's entry door: into the server room
//   lock         a keyed door: away from the face its cylinder is on (the corridor's)
//   stands open  the angle it is authored standing at
//   floor        the nav grid flooded from either face with every doorway shut: into the SMALLER floor (a room off a hall, a building off its lot)
//   wider        the side its authored stops let it open further, else +1
//   leaf room    (DOORWAY.leafWay — the maintainer, 10-27, of his squad on the compound: "feels like squad gets stuck when stacking up or going through doors") a
//                door whose leaf, standing open on the side the rules above deal it, SHUTS FLOOR OFF FROM ITS OWN DOORWAY for a man — a cot a metre inside the
//                door, and the open leaf hangs across the one strip that leads past it — opens the OTHER way, where its leaf has room to swing and leaves the way clear. Dealt BESIDE
//                the rules above like the outside rule (SwingDeal.clear, Door.clearway): the switch picks live. It does not speak against an authored `swing`,
//                a folding stop, a drill plan's fling or a leaf authored standing open, nor where the other side is no better or the leaf would swing into something there
// Pure geometry over the Level and the nav grid: no dice, nothing of the Game's state — so the deal is the same on every boot and, with DOORWAY.oneWay off,
// changes nothing at all (Door.swing is then read by nobody). `dealSwings` returns what it dealt and why (Game.doorSwings: the probes and the notes read it).
import type { Level } from '../scene/level';
import type { NavGrid, StaticCollision } from '../scene/collision';
import { BoxFlag } from '../scene/boxes';
import { DOORWAY, type Doors, type Door } from './doors';

export type SwingWhy = 'authored' | 'folds' | 'outside' | 'plan' | 'room' | 'hostages' | 'mission' | 'lock' | 'stands open' | 'floor' | 'wider' | 'leaf room';
export interface SwingDeal { door: string; swing: 1 | -1; why: SwingWhy; /** the room / floor it opens into, in words, where the rule names one */ into?: string;
  /** an outside door's deal with DOORWAY.exitsOut on (the 'outside' rule: toward the open air) — the row itself is its deal with the switch off */ out?: SwingDeal;
  /** its deal with DOORWAY.leafWay on, where the dealt side's open leaf shuts floor off from the doorway (the 'leaf room' rule: the other side) */ clear?: SwingDeal }
/** The deal as the doors stand now: an outside door's `out` row while DOORWAY.exitsOut, every other row as dealt (Game.doorSwings). */
export const swingsNow = (deals: SwingDeal[]): SwingDeal[] => deals.map(s => s.out && DOORWAY.exitsOut ? s.out : s.clear && DOORWAY.leafWay ? s.clear : s);
interface Rect { x0: number; x1: number; z0: number; z1: number }
/** the least a door must be able to open (rad) on a side for that side to be dealt by rule */
const MIN_OPEN = 0.9;

/** the side (Doors.side's sign) of the closed leaf a floor point is on, 0 when it is on the leaf's own line */
function sideOf(d: Door, x: number, z: number): 1 | -1 | 0 {
  const c = d.def.closedDir, ux = Math.cos(c), uz = Math.sin(c), W = d.def.width; const mx = d.def.hinge[0] + ux * W / 2, mz = d.def.hinge[1] + uz * W / 2;
  const s = (x - mx) * -uz + (z - mz) * ux; return Math.abs(s) < 0.05 ? 0 : s > 0 ? 1 : -1;
}
const mid = (r: Rect): [number, number] => [(r.x0 + r.x1) / 2, (r.z0 + r.z1) / 2];
const area = (r: Rect) => Math.abs((r.x1 - r.x0) * (r.z1 - r.z0));

/** How much of the floor just off face `s` of a door has a ROOF over it, of ROOF_PTS points (0.8 and 1.5 m out, the middle of the leaf and 0.4 m either way): a
 *  solid static whose footprint holds the point and whose underside is over a standing man's head — a ceiling slab, a canopy; never the wall the door hangs in
 *  (the points stand clear of it) and never a thing standing on the ground. Six points, so a lamp head on the wall over the door is not a roof. */
const ROOF_PTS = 6, ROOF_UNDER = 1.9;
function roofOver(level: Level, d: Door, s: 1 | -1): number {
  const c = d.def.closedDir, ux = Math.cos(c), uz = Math.sin(c), mx = d.def.hinge[0] + ux * d.def.width / 2, mz = d.def.hinge[1] + uz * d.def.width / 2; let n = 0;
  for (const off of [0.8, 1.5]) for (const al of [-0.4, 0, 0.4]) {
    const x = mx - uz * off * s + ux * al, z = mz + ux * off * s + uz * al;
    if (level.boxes.some(b => { if (b.flags & BoxFlag.NoShadow || b.c[1] - b.h[1] < ROOF_UNDER) return false;
      const dx = x - b.c[0], dz = z - b.c[2], cy = Math.cos(b.yaw), sy = Math.sin(b.yaw); return Math.abs(dx * cy - dz * sy) <= b.h[0] && Math.abs(dx * sy + dz * cy) <= b.h[2]; })) n++;
  }
  return n;
}
/** The open-air side of an outside door: the face with open sky over it when the other is under a roof (at most 2 of the 6 points covered against at least 4);
 *  0 for a door the level does not mark `exterior`, and for one roofed on both faces or on neither. */
function outsideOf(level: Level, d: Door): 1 | -1 | 0 {
  if (!d.def.exterior) return 0;
  const p = roofOver(level, d, 1), m = roofOver(level, d, -1), IN = ROOF_PTS - 2, OUT = 2;
  return p >= IN && m <= OUT ? -1 : m >= IN && p <= OUT ? 1 : 0;
}

/** The nav cells either face of every door opens on, with every doorway shut: per door, the floor (cell count) reachable from just off each face. */
function floors(doors: Doors, nav: NavGrid): Map<Door, { plus: number; minus: number }> {
  const W = nav.W, H = nav.H, cs = nav.cell, wall = nav.blocked.slice();
  for (const d of doors.list) {   // every closed leaf as a wall: the cells its line (and a hand's width either end: the jambs) runs through
    const c = d.def.closedDir, ux = Math.cos(c), uz = Math.sin(c);
    for (let t = -0.3; t <= d.def.width + 0.3; t += cs / 4) for (const o of [-cs * 0.3, 0, cs * 0.3]) {
      const x = Math.floor((d.def.hinge[0] + ux * t - uz * o) / cs), z = Math.floor((d.def.hinge[1] + uz * t + ux * o) / cs);
      if (x >= 0 && z >= 0 && x < W && z < H) wall[z * W + x] = 1;
    }
  }
  const comp = new Int32Array(W * H).fill(-1), sizes: number[] = [];
  const flood = (start: number): number => {
    if (comp[start] >= 0) return comp[start];
    const id = sizes.length; let n = 0; const q = [start]; comp[start] = id;
    while (q.length) { const i = q.pop()!; n++; const x = i % W, z = (i / W) | 0;
      if (x > 0 && !wall[i - 1] && comp[i - 1] < 0) { comp[i - 1] = id; q.push(i - 1); } if (x < W - 1 && !wall[i + 1] && comp[i + 1] < 0) { comp[i + 1] = id; q.push(i + 1); }
      if (z > 0 && !wall[i - W] && comp[i - W] < 0) { comp[i - W] = id; q.push(i - W); } if (z < H - 1 && !wall[i + W] && comp[i + W] < 0) { comp[i + W] = id; q.push(i + W); } }
    sizes.push(n); return id;
  };
  const out = new Map<Door, { plus: number; minus: number }>();
  for (const d of doors.list) {
    const c = d.def.closedDir, ux = Math.cos(c), uz = Math.sin(c), mx = d.def.hinge[0] + ux * d.def.width / 2, mz = d.def.hinge[1] + uz * d.def.width / 2;
    const from = (s: number): number => {   // the first free cell out from the leaf's middle on that face
      for (const off of [0.75, 1.0, 1.25, 0.5, 1.5]) for (const al of [0, 0.3, -0.3]) {
        const x = Math.floor((mx - uz * off * s + ux * al) / cs), z = Math.floor((mz + ux * off * s + uz * al) / cs);
        if (x >= 0 && z >= 0 && x < W && z < H && !wall[z * W + x]) return flood(z * W + x);
      }
      return -1;
    };
    const a = from(1), b = from(-1);
    out.set(d, a >= 0 && b >= 0 && a === b ? { plus: 0, minus: 0 } : { plus: a < 0 ? 0 : sizes[a], minus: b < 0 ? 0 : sizes[b] });   // (one floor both sides — an opening beside it joins them: it says nothing)
  }
  return out;
}

/** DOORWAY.leafWay — a man's radius (a little under the 0.34 the nav grid is baked with and a door's contact asks: a gap he brushes through, nudging the
 *  leaf, is a way. At 0.34 the barracks' n3 and the bazaar's pharm_back would turn too) · the floor grid's step and its reach either way from the doorway (m) · the angle a pulled or pushed leaf comes to rest at (it
 *  coasts past square) · the floor (m²) an open leaf must shut off from its doorway before the door is hung the other way · the band of floor round a standing
 *  leaf (m) that, all free, is a way round it · how far off the door's own wall that band starts */
const LEAF_MAN = 0.3, LEAF_STEP = 0.1, LEAF_WIN = 4, LEAF_REST = 1.84, LEAF_CUT = 1.0, LEAF_RING = 0.25, LEAF_WALL = 0.5;
/** How much floor on face `s` of a door (m², within LEAF_WIN of it) a man can walk to the doorway from with no leaf there and can NOT with the leaf standing open
 *  on that face: the floor its open leaf shuts off. Asked square to the frame and at the angle a leaf rests at (each no wider than its authored stop) and the
 *  LESSER taken — a leaf that leaves a way at either is one a man can put where it leaves one. Statics only (the level's boxes, lived things among them),
 *  between knee and shoulder as the nav grid asks it. Cheap for nearly every door: a leaf with free floor all the way round it shuts nothing off, and only
 *  one without is flooded. The other doors of the level count as open, so the figure is all the floor that is reached only past this leaf, a next room with no
 *  other way in among it. −1 = no floor a man can stand on at the doorway on that face. */
export function leafCut(col: StaticCollision, d: Door, s: 1 | -1): number {
  const stop = s > 0 ? d.maxAuth : -d.minAuth; if (stop < 0.9) return 0;
  const c = d.def.closedDir, W = d.def.width, ux = Math.cos(c), uz = Math.sin(c), nx = -uz * s, nz = ux * s, [hx, hz] = d.def.hinge, cx = hx + ux * W / 2, cz = hz + uz * W / 2;
  const NA = Math.round(2 * LEAF_WIN / LEAF_STEP) + 1, NP = Math.round(LEAF_WIN / LEAF_STEP), N = NA * NP, known = new Int8Array(N).fill(-1), q: [number, number, number] = [0, 0, 0];
  const alOf = (k: number) => -LEAF_WIN + (k % NA) * LEAF_STEP, peOf = (k: number) => (Math.floor(k / NA) + 0.5) * LEAF_STEP;
  const free = (k: number): boolean => { if (known[k] < 0) { const al = alOf(k), pe = peOf(k), x = cx + ux * al + nx * pe, z = cz + uz * al + nz * pe; q[0] = x; q[1] = 0; q[2] = z;
      known[k] = col.collideCircle(q, LEAF_MAN, 0.25, 1.5, 1) && Math.hypot(q[0] - x, q[2] - z) > 0.005 ? 0 : 1; } return known[k] === 1; };
  let start = -1, near = 1e9;   // the doorway's own floor on this face: the free spot nearest the middle of the frame
  for (let k = 0; k < NA * Math.min(NP, 9); k++) { const al = Math.abs(alOf(k)), pe = peOf(k); if (al < W / 2 - LEAF_MAN + 0.06 && al + pe < near && free(k)) { near = al + pe; start = k; } }
  if (start < 0) return -1;
  const flood = (leaf: Float32Array | null): Uint8Array | null => { const R = LEAF_MAN + d.thick / 2; if (leaf && leaf[start] < R) return null; const seen = new Uint8Array(N), st = [start]; seen[start] = 1;
    while (st.length) { const k = st.pop()!, i = k % NA;
      for (const n of [i > 0 ? k - 1 : -1, i < NA - 1 ? k + 1 : -1, k - NA, k + NA]) if (n >= 0 && n < N && !seen[n] && !(leaf && leaf[n] < R) && free(n)) { seen[n] = 1; st.push(n); } }
    return seen; };
  let open: Uint8Array | null = null, best = Infinity;
  for (const th of stop < Math.PI / 2 ? [stop] : [Math.PI / 2, Math.min(stop, LEAF_REST)]) {
    const lx = Math.cos(th) * ux + Math.sin(th) * nx, lz = Math.cos(th) * uz + Math.sin(th) * nz, leaf = new Float32Array(N), R = LEAF_MAN + d.thick / 2; let round = true;
    for (let k = 0; k < N; k++) { const al = alOf(k), pe = peOf(k), px = cx + ux * al + nx * pe - hx, pz = cz + uz * al + nz * pe - hz, t = Math.max(0, Math.min(W, px * lx + pz * lz));
      leaf[k] = Math.hypot(px - lx * t, pz - lz * t); if (round && leaf[k] >= R && leaf[k] < R + LEAF_RING && pe >= LEAF_WALL && !free(k)) round = false; }
    if (round) return 0;   // free floor all the way round the standing leaf: a man walks round it
    open ??= flood(null)!; const shut = flood(leaf); let n = 0; for (let k = 0; k < N; k++) if (open[k] && leaf[k] >= R && !(shut && shut[k])) n++;
    best = Math.min(best, n * LEAF_STEP * LEAF_STEP); if (best === 0) return 0;
  }
  return best;
}
/** Whether a leaf swung open on face `s`, as far as it rests, meets nothing of the level on its way (the same statics, at three spots along the leaf) */
export function leafSwings(col: StaticCollision, d: Door, s: 1 | -1): boolean {
  const stop = Math.min(s > 0 ? d.maxAuth : -d.minAuth, LEAF_REST), c = d.def.closedDir, W = d.def.width, [hx, hz] = d.def.hinge, q: [number, number, number] = [0, 0, 0];
  for (let a = 0.5; a <= stop + 1e-9; a += 0.1) for (const t of [0.35, 0.7, 1]) { const x = hx + Math.cos(c + s * a) * W * t, z = hz + Math.sin(c + s * a) * W * t; q[0] = x; q[1] = 0; q[2] = z;
    if (col.collideCircle(q, 0.03, 0.25, 1.5, 1) && Math.hypot(q[0] - x, q[2] - z) > 0.005) return false; }
  return true;
}

/** Deal every door of the level its side (Door.setSwing) and say why. `col`: the level's statics, for the 'leaf room' rule (absent: that rule says nothing). */
export function dealSwings(level: Level, doors: Doors, nav: NavGrid | null, col: StaticCollision | null = null): SwingDeal[] {
  const S = level.scenarios, rooms = S?.roomClear?.rooms ?? [], plans = [...(level.breach ? [level.breach] : []), ...(level.clearPlans ?? [])];
  let fl: ReturnType<typeof floors> | null | undefined;   // (flooded on the first door that gets as far as asking)
  const toward = (d: Door, r: Rect | undefined): 1 | -1 | 0 => r ? sideOf(d, ...mid(r)) : 0;
  return doors.list.map((d): SwingDeal => {
    const name = d.def.name, opens = (s: number) => (s > 0 ? d.maxAuth : -d.minAuth) >= MIN_OPEN;
    const deal = (swing: 1 | -1 | 0, why: SwingWhy, into?: string): SwingDeal | null => swing && (why === 'authored' || opens(swing) || !opens(-swing)) ? { door: name, swing, why, ...(into ? { into } : {}) } : null;   // (never a side its authored stop will not let it open to — a wall behind it — while the other will)
    const tries: (() => SwingDeal | null)[] = [
      () => deal(d.def.swing ?? 0, 'authored'),
      () => deal(d.maxAuth > 2.2 && -d.minAuth <= 2.2 ? 1 : -d.minAuth > 2.2 && d.maxAuth <= 2.2 ? -1 : 0, 'folds'),
      () => { const P = plans.filter(p => p.door === name && p.fling); return P.length && P.every(p => Math.sign(p.fling) === Math.sign(P[0].fling)) ? deal(Math.sign(P[0].fling) as 1 | -1, 'plan', P[0].room) : null; },   // (plans from both faces that throw it opposite ways say nothing: the room rule decides)
      () => { const mine = rooms.filter(r => r.doors.some(x => x.id === name)).sort((a, b) => area(a.bounds) - area(b.bounds) || (a.id < b.id ? -1 : 1));
        for (const r of mine) { const s = deal(toward(d, r.bounds), 'room', r.name); if (s) return s; } return null; },
      () => { const R = S?.rescue; if (!R) return null; const bounds = R.room.bounds ?? plans.find(p => p.door === R.room.door)?.bounds;
        return R.room.door === name || R.side?.door === name ? deal(toward(d, bounds), 'hostages', R.room.name) : null; },
      () => level.mission?.entryDoor === name ? deal(toward(d, level.mission.serverRoom), 'mission', 'server room') : null,
      () => d.def.locked ? deal(-d.keySide as 1 | -1, 'lock') : null,
      () => deal(Math.sign(d.def.angle ?? 0) as 1 | -1 | 0, 'stands open'),
      () => { if (fl === undefined) fl = nav ? floors(doors, nav) : null; const f = fl?.get(d); return f && f.plus > 0 && f.minus > 0 && f.plus !== f.minus ? deal(f.plus < f.minus ? 1 : -1, 'floor', `${Math.min(f.plus, f.minus)} cells against ${Math.max(f.plus, f.minus)}`) : null; },
    ];
    let got: SwingDeal | null = null; for (const t of tries) if ((got = t())) break;
    got ??= { door: name, swing: d.maxAuth >= -d.minAuth ? 1 : -1, why: 'wider' };
    const out = got.why !== 'authored' && got.why !== 'folds' ? deal(outsideOf(level, d), 'outside', 'the open air') : null;   // (an authored side and a folding stop speak first: for those the switch changes nothing)
    if (out) got.out = out;
    let clear: SwingDeal | null = null;
    if (col && got.why !== 'authored' && got.why !== 'folds' && got.why !== 'plan' && got.why !== 'stands open' && !d.def.angle && opens(-got.swing)) {
      const cut = leafCut(col, d, got.swing), other = cut >= LEAF_CUT ? leafCut(col, d, -got.swing as 1 | -1) : -1;   // (other < 0: nowhere to stand in the doorway on that side)
      if (cut >= LEAF_CUT && other >= 0 && other < LEAF_CUT && other * 2 <= cut && leafSwings(col, d, -got.swing as 1 | -1)) clear = { door: name, swing: -got.swing as 1 | -1, why: 'leaf room', into: `${cut.toFixed(1)} m² of floor shut off by its leaf the other way` };
    }
    if (clear) got.clear = clear;
    d.setSwing(got.swing, out ? out.swing : 0, clear ? clear.swing : 0); return got;
  });
}
