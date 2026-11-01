// ONE INDEX OF EVERY SOLID THING (the maintainer, 10-31: "we should make sure all objects have the same treatment"). The breaking engines each chose their own
// things by shape and place as the level loaded (walls and cover in wallbreak.ts, furniture in furnblast.ts, the small things in destruction.ts, glass, doors,
// the loose props), and a thing none of them chose could only scorch: 443 of the pass's 653 authored boxes, everything in its yard among them.
//
// Here every authored static gets ONE row, indoors or out, made once from the level as it was authored:
//   mat   what it is made of, off its paint: the furniture's break classes (wood, board, metal, hard, soft) plus glass, snow and ground
//   size  structure (the ground, walls that carry, posts, what is overhead) · heavy (too big for a blast to move: a vehicle's bed, a tank) · furniture · small
//   hold  stands on the ground · rests on another thing · hangs (nothing under it: on a wall, from above) · lies (a finish, a rug, a film: no body)
//   via   WHICH ENGINE BREAKS IT. The engines keep their own work; they no longer choose what it is done to. A thing whose `via` is 'structure' is true
//         structure and takes a blast's marks alone; 'kept' is a thing play needs whole (the mission's thing, the breaker, a lamp that stands by itself).
//
// The rules are in ONE place, `classify` below, and are about what a thing IS, never where it stands: nothing here asks whether a roof is over it.
//
// THINGS.on (off in this module and on the frozen stand; the page raises it, ?things=0 puts it off; TPS_THINGS=1 in front of a headless tool): the furniture
// engine reads its things from here (furnblast.ts index()). Off = each engine's own choice, the build as it was.
// No top-level use of another game module's export: src/game is one import cycle.
import type { Vec3 } from '../math/vec';
import { Box, BoxFlag } from '../scene/boxes';
import { WORLD } from '../scene/world';
import type { Game } from './game';
import type { Destruction } from './destruction';
import { paletteName } from './particleFx';
import { WALLS } from './wallbreak';   // (read inside burstSize only: src/game is one import cycle)
import { snowPaintOf } from '../render/snowField';

export const THINGS = {
  on: false,
  /** a finish, a rug, a film of snow: thinner than this (m) and it has no body to break */
  flatM: 0.05,
  /** OVERHEAD: a thing whose underside is this high (m) is over a man's head — a lintel, a ceiling, a roof, a duct */
  overheadM: 2.0,
  /** a thing in the way of a body reaches no higher than this (m); taller, standing on the ground, it is part of the building */
  maxTop: 2.3,
  /** A POST: no thicker than `thick` either way in plan, at least `tall` high (m) — a jamb, a stud, a pole that carries something */
  post: { thick: 0.3, tall: 1.5 },
  /** HEAVY: more than `vol` m³, or longer than `long` m (a thin thing — a board, a sheet, a fence panel under `thin` m — may run to `longThin`) */
  heavy: { vol: 0.7, long: 2.6, thin: 0.12, longThin: 4.5 },
  /** SMALL: under `vol` m³ and no side over `side` m — what sits on furniture, what a blast's far ring knocks over */
  small: { vol: 0.03, side: 0.7 },
  /** two boxes nearer than this (m) touch */
  touch: 0.04, panelVol: 0.25,
  /** EVERY THING BREAKS (the maintainer on a GPU, 10-31: "the indestructible objects"). On: a HEAVY thing is a thing too — it comes apart in the shatter
   *  ring, and one of steel or stone only in the contact ring, with the burst lying against it (the furniture engine's `core`); and a POST that stands free
   *  of the building (a stall's leg, a fence post, a pole) is a thing, with whatever light thing it carries. Off: both stay, as the index first had them.
   *  What still stays is the building (full height, overhead, its paints, a post in a wall) and what play needs whole. */
  every: true,
  /** WHAT STANDS SHOWS THE DAMAGE, AND EACH STUFF GOES ITS OWN WAY (the maintainer, 10-31: "one thing that control did well was having different deformations
   *  per material, like wood splintering, metal bending"). Three rules under one switch, on with the index:
   *    marks   a thing a burst reaches and does not break is marked where it stands, in its own stuff — steel dented, stone chipped, wood split, soft stuff
   *            torn (scars.ts: a cut record on the box; `reach` × the shatter ring, at most `most` things a burst and `perBox` marks a thing)
   *    local   a heavy thing loses the heavy PART the burst is against, not all of it: each heavy part is asked by its own distance (and what it held up goes
   *            with it); the rest stands, marked
   *    bent    steel is never sawn: a heavy steel part comes off in folded lengths of `plate` m, one no longer than that goes over whole and dented, and
   *            every plate torn off any steel thing carries its fold (breakTable.ts `bent`)
   *  The numbers are metres deep a mark may bite (a range by a hash, × how near the burst was) and the most of the side it is on it may take. A marked
   *  thing is at least `min` long and tall and `minFace` across, its top at or over `stands` (never a floor); a mark lies within `span` of the burst along
   *  the face (a cut only takes an edge or a corner off, so a long thing hit mid-length shows nothing); a crease under `fine` deep is not cut.
   *  The page: `?deform=0` puts the three off alone (they go with `?things=0` too). */
  deform: { on: true, marks: true, local: true, bent: true, reach: 1.4, most: 10, perBox: 2, min: 0.25, minFace: 0.1, stands: 0.3, span: 1.5, fine: 0.01, big: 2.5, bigAt: 0.8, plate: 1.8,
    dent: [0.03, 0.09] as [number, number], dentMost: 0.14, chip: [0.07, 0.2] as [number, number], chipMost: 0.3, chipTwo: 0.6,
    split: [0.04, 0.12] as [number, number], splitMost: 0.3, tear: [0.03, 0.07] as [number, number], tearMost: 0.25,
    chips: { dent: 3, chip: 14, split: 12, tear: 5 }, dust: 0.9 },
  /** BROKEN PIECES LIE APART (the maintainer on a GPU, 11-01: two door slabs of one thickness lying through each other, their tops flickering in one plane).
   *  On: a piece of the pool is seated `step` m × (its serial mod `levels`) over the surface it comes to rest on (debris.ts `seat`), so two pieces of one
   *  thickness that lie flat through each other never have their tops in one plane: the pieces of one break are all at their own height, two of different
   *  breaks share one 1 time in `levels`. The highest sits `step` × (`levels` − 1) up (7 mm). Nothing else of a piece changes: its size, its flight, where
   *  it lands. Acts only with the one index. The page: `?apart=0` = every piece 1.5 mm over its floor, as before. */
  apart: { on: true, step: 0.0007, levels: 11 },
  /** WHAT IS LOOSE STAYS IN THE LEVEL (the maintainer on a GPU, 11-01: "objects that fly off the domain can disappear"). The ground, the trace grid and
   *  every floor ray end at the world's rectangle (scene/world.ts WORLD.min / max): a piece past it lies on nothing and no traced ray meets it. On: the
   *  rectangle's edge is a wall to a pool piece (debris.ts), a pushed or thrown prop (props.ts) and a door off its hinges (doorblast.ts): it stops there
   *  with `back` of its speed returned, as off a wall, and comes to rest inside like any other. Acts only with the one index. The page: `?inside=0` =
   *  what flies past the edge is gone from the picture, as before. */
  inside: { on: true, back: 0.2 },
  /** OLD RUBBLE STAYS (the maintainer, 11-01: after a lot of blasts "the old clutter has gone away"; "that single system that we use in both settings rather
   *  than bolting another thing on top"). The debris pool's one list holds the pieces in motion AND the ones lying still, and its tier figure (64 / 32) was the
   *  bound on both, so the 65th piece crumbled the oldest one at rest. On: the tier figure bounds only the
   *  pieces IN MOTION (what it was written for: tumbling shadow casters), and the list may hold `keep` times it in all. A piece that comes to rest stays where
   *  it lies in the same list, with the same feet, the same wake by a blast and the same crumble when the larger bound is reached.
   *  A blast over a heap wakes pieces only up to the tier figure; the rest lie on. Acts only with the one index. The page: `?rubble=0` = the pool as it was. */
  rubble: { on: true, keep: 5 },
  /** how a heavy part is cut when its ring takes it (m a piece along its length, at most `max`, in `strips` across) and how its pieces fly and turn beside a chair's */
  heavyCut: { len: 0.9, max: 5, strips: 2, fly: 0.55, spin: 0.5 },
  /** ONE SET OF RINGS FOR EVERY BURST (the maintainer, 10-31: "we should have different layers of destruction for stuff like wall charges"). Every burst is
   *  the same five rings, in metres at size 1 (a wall charge) and scaled by the burst's size; every engine reads its reach from them and keeps none of its own:
   *    contact  the thing the burst is ON opens or goes: a wall is holed from the floor up, stone and concrete break
   *    shatter  a thing breaks into its material's pieces: a wall loses its upper half, cover its cells, a door its hinges, furniture is split and thrown
   *    throw    a thing comes apart at its joints or goes over whole, loose things fly
   *    shake    small things fall off what they rest on, loose things are nudged
   *    mark     scorch, dust and the floor's mess (the pall and the mess are sized by the burst, as they were)
   *  GLASS keeps its own graded model (glassbreak.ts: a pane's fate by its type and the blast's strength where it stands) and its own size, FRAG.glassM.
   *  The two inner rings are the wall engine's own shares of its radius (0.45 and 0.8 of WALLS.blast.radius), so a wall opens to a burst as it always did;
   *  the furniture's and the doors' reaches, which each had a scale of their own, follow the walls'.
   *  A BURST'S SIZE is its breaking radius over a wall charge's (WALLS.blast.radius): a wall charge 1, a 40 mm 1, a hand grenade 2.5 ∕ 2.2. `frag` scales
   *  every burst that is not a breaching charge (1 = by its radius alone; the lab's slider); a breaching charge under `chargeM` of radius is a door's, `door`.
   *  on (acts only with THINGS.on; the page raises both, ?rings=0 puts this one off; TPS_RINGS=1): off = every engine's own reach, as it was. */
  rings: { on: false, contact: 0.99, shatter: 1.76, throw: 4.0, shake: 6.0, mark: 9.0, door: 0.7, frag: 1, chargeM: 1.5, shakeChunks: 8 },
};
/** a burst's five rings in metres */
export interface Rings { size: number; contact: number; shatter: number; throw: number; shake: number; mark: number }
/** THINGS.inside: where a loose thing at (x, z), r across (rz along z when that differs), is held on the world's ground rectangle — null while the rule is off or it is already inside
 *  (or the thing is wider than the world). nx / nz = the way back in on each axis (0 = that axis was inside). */
export function heldInside(x: number, z: number, r: number, rz = r): { x: number; z: number; nx: number; nz: number } | null {
  if (!THINGS.on || !THINGS.inside.on) return null;
  const x0 = WORLD.min[0] + r, x1 = WORLD.max[0] - r, z0 = WORLD.min[2] + rz, z1 = WORLD.max[2] - rz; if (x0 > x1 || z0 > z1) return null;
  const nx = x < x0 ? 1 : x > x1 ? -1 : 0, nz = z < z0 ? 1 : z > z1 ? -1 : 0; if (!nx && !nz) return null;
  return { x: nx > 0 ? x0 : nx < 0 ? x1 : x, z: nz > 0 ? z0 : nz < 0 ? z1 : z, nx, nz };
}
export const ringsOn = (): boolean => THINGS.on && THINGS.rings.on;
/** how big a burst is (1 = a wall charge), from what its caller says of it: `chaos: false` is a breaching charge that brings its own bang */
export function burstSize(o: { radius?: number; power?: number; chaos?: boolean }): number {
  const K = THINGS.rings, R0 = WALLS.blast.radius, R = (o.radius ?? R0) * (o.power ?? 1);
  return o.chaos === false ? (R >= K.chargeM ? R / R0 : K.door) : K.frag * R / R0;
}
export function ringsOf(size: number): Rings { const K = THINGS.rings; return { size, contact: K.contact * size, shatter: K.shatter * size, throw: K.throw * size, shake: K.shake * size, mark: K.mark * size }; }

/** `stone` = masonry and concrete: it breaks only in a burst's contact ring (furnblast.ts) */
export type ThingMat = 'wood' | 'board' | 'metal' | 'hard' | 'soft' | 'stone' | 'glass' | 'snow';
export type ThingSize = 'structure' | 'heavy' | 'furniture' | 'small';
export type ThingHold = 'stands' | 'rests' | 'hangs' | 'lies';
export type ThingVia = 'walls' | 'cover' | 'furniture' | 'item' | 'glass' | 'kept' | 'structure' | 'unseen';
export interface ThingBounds { x0: number; x1: number; y0: number; y1: number; z0: number; z1: number }
export interface Thing {
  /** the static's index in the level's boxes */ i: number;
  mat: ThingMat; size: ThingSize; hold: ThingHold; via: ThingVia;
  /** what it rests on (a static's index), -1 = the ground or nothing */ on: number;
  /** drawn only: no ray, round or foot meets it */ ghost: boolean;
  /** why it is structure or kept, in a word (the census prints it) */ why: string;
  a: ThingBounds;
}

/** paints that are finishes of the building: a box in one is structure unless its size and place say it is a thing (bedding in a carpet paint, a low kerb) */
const BUILDING = new Set(['carpet', 'carpetBlue', 'carpetRed', 'lino', 'stone', 'asphalt', 'grass', 'wall', 'wallDark', 'concrete', 'ceiling', 'white', 'partRed', 'partBlue', 'partTan']);
/** paints that are a window's own: its frame is the building's however small */
const GLAZING = new Set(['glassFrame', 'glass', 'glassExt']);
const GROUND = new Set(['carpet', 'carpetBlue', 'carpetRed', 'lino', 'stone', 'asphalt', 'grass']);
const STONE = new Set(['stone', 'asphalt', 'concrete', 'wall', 'wallDark']);

export function boundsOf(b: Box): ThingBounds {
  const cs = Math.abs(Math.cos(b.yaw)), sn = Math.abs(Math.sin(b.yaw)); const ex = cs * b.h[0] + sn * b.h[2], ez = sn * b.h[0] + cs * b.h[2];
  return { x0: b.c[0] - ex, x1: b.c[0] + ex, y0: b.c[1] - b.h[1], y1: b.c[1] + b.h[1], z0: b.c[2] - ez, z1: b.c[2] + ez };
}
/** what a box is made of, off its paint (an unnamed paint on a thing is steel: a cabinet, a locker, a machine) */
export function matOfPaint(b: Box): ThingMat {
  if (b.flags & BoxFlag.Glass) return 'glass';
  if (snowPaintOf(b) !== null) return 'snow';
  const name = paletteName(b.albedo); if (!name) return 'metal';
  if (name === 'wood' || name === 'woodDark') return 'wood'; if (name === 'cardboard') return 'board'; if (name === 'metal' || name === 'red' || GLAZING.has(name)) return 'metal';
  if (name === 'chair' || name === 'plant' || name === 'carpet' || name === 'carpetBlue' || name === 'carpetRed') return 'soft';
  return STONE.has(name) ? 'stone' : 'hard';
}

export class Things {
  private rows: Thing[] | null = null;
  constructor(private g: Game, private destr: Destruction) {}

  /** every authored static's row, in index order (made on the first ask, from the flags the level was authored with: the same answer whenever it is made) */
  all(): Thing[] { return this.rows ?? (this.rows = this.make()); }
  at(i: number): Thing | undefined { return this.all()[i]; }
  /** the rows are made again at the next ask (the debugger's `every` switch: the rules themselves changed) */
  forget() { this.rows = null; }

  /** the flags static `i` was authored with, whoever has hidden it since */
  private flags0(i: number): number { return this.destr.furn.flags0(i) ?? this.destr.flags0(i) ?? this.g.col.boxes[i].flags; }
  private make(): Thing[] {
    const g = this.g, boxes = g.col.boxes, walls = this.destr.walls, n = Math.min(boxes.length, walls.authored), K = THINGS;
    const lights = new Set<number>(); for (const t of g.targets) for (const bi of t.boxes) lights.add(bi);
    for (const l of g.level.lights) { const fb = (l as { fixtureBox?: number }).fixtureBox; if (fb !== undefined && fb >= 0) lights.add(fb); }
    const breaker = new Set<number>(g.level.breaker?.boxes ?? []);
    const fronts: Vec3[] = []; if (g.level.mission?.rack?.front) fronts.push(g.level.mission.rack.front); for (const sp of g.level.scenarios?.quiet?.spots ?? []) if (sp.front) fronts.push(sp.front);
    const out: Thing[] = [];
    for (let i = 0; i < n; i++) {
      const b = boxes[i], a = boundsOf(b); const f0 = this.flags0(i);
      out.push({ i, mat: matOfPaint(b), size: 'structure', hold: 'lies', via: 'structure', on: -1, ghost: (f0 & BoxFlag.NoShadow) !== 0, why: '', a });
    }
    // what each thing is, alone
    for (const t of out) this.classify(t, boxes[t.i], this.flags0(t.i), lights, breaker, fronts);
    // A POST THAT STANDS FREE IS A THING (THINGS.every): one no taller than a thing may be, that no part of the building touches — a wall, a full-height
    // block, another post that reaches the ceiling. A jamb, a stud in a wall, a pilaster stay the building's; and a free post that carries something that
    // stays (a lamp, a heavy canopy) is kept by the carry rule below, as any thing is.
    if (K.every) { const e = K.touch, built = out.filter(o => !o.ghost && (o.via === 'walls' || o.why === 'full height' || o.why === 'the building' || (o.why === 'a post' && o.a.y1 > K.maxTop)));
      for (const t of out) { if (t.why !== 'a post' || t.a.y1 > K.maxTop) continue; const a = t.a;
        if (built.some(o => o !== t && o.a.x0 <= a.x1 + e && o.a.x1 >= a.x0 - e && o.a.z0 <= a.z1 + e && o.a.z1 >= a.z0 - e && o.a.y0 <= a.y1 + e && o.a.y1 >= a.y0 + 0.3)) continue;   // (touching it above the floor they both stand on: a slab under a post is no wall)
        const b = boxes[t.i], vol = b.h[0] * b.h[1] * b.h[2] * 8; t.via = 'furniture'; t.size = vol <= K.small.vol && (a.y1 - a.y0) <= K.small.side ? 'small' : 'furniture'; t.why = ''; } }
    // how each is held: on the ground, on another thing, or by nothing under it. A sweep along x keeps the pair test small.
    const order = out.filter(t => t.via !== 'unseen').sort((p, q) => p.a.x0 - q.a.x0 || p.i - q.i); const e = K.touch;
    for (const t of out) {
      if (t.via === 'unseen' || t.hold === 'lies') continue;
      if (t.a.y0 <= e) { t.hold = 'stands'; continue; }
      let best = -1, top = -1e9;
      for (const o of order) {
        if (o.a.x0 > t.a.x1) break; if (o === t || o.ghost || Math.abs(o.a.y1 - t.a.y0) > e || o.a.x1 < t.a.x0 + 0.01 || o.a.z1 < t.a.z0 + 0.01 || o.a.z0 > t.a.z1 - 0.01 || o.a.x0 > t.a.x1 - 0.01) continue;
        if (o.a.y1 > top || (o.a.y1 === top && o.i < best)) { top = o.a.y1; best = o.i; }
      }
      if (best >= 0) { t.hold = 'rests'; t.on = best; } else t.hold = t.a.y0 <= 0.25 ? 'stands' : 'hangs';   // (on legs too short to draw, on a film of snow: it stands)
    }
    // WHAT RESTS ON A PIECE OF FURNITURE IS THAT PIECE'S, whatever it would be called standing alone: a light's own box (the furniture engine puts the light
    // out), lying snow (thrown as a spray), a post (a stove's flue), a board over a man's head (the top of a rack), a block in a building paint. Structure
    // stands on the ground or on structure. Never a heavy thing (a soffit that happens to meet a cabinet's top: the rule below then keeps the cabinet), a
    // window's frame, or a thing play needs whole. Asked until nothing changes: the shelf on the upright, then the box on the shelf.
    const rider = new Set(['light', 'lying snow', 'a post', 'overhead', 'the building', 'full height']);
    for (let pass = 0, more = true; more && pass < 8; pass++) {
      more = false;
      for (const t of out) {
        if (!rider.has(t.why) || t.hold !== 'rests' || out[t.on].via !== 'furniture') continue;
        const b = boxes[t.i]; if (b.h[0] * b.h[1] * b.h[2] * 8 > K.heavy.vol) continue;
        t.via = 'furniture'; t.size = t.why === 'light' || t.why === 'lying snow' ? 'small' : 'furniture'; t.why = ''; more = true;
      }
    }
    // WHAT CARRIES SOMETHING THAT STAYS, STAYS: the wall under a window, a machine's plinth, a lorry's frame, the feet under a crate of real cover. Asked of
    // everything resting on it (a thing may stand on several), until nothing changes (a plate under a plinth under a tank).
    const stays = out.filter(o => !o.ghost && o.hold !== 'lies' && o.via !== 'furniture' && o.via !== 'item' && o.via !== 'unseen' && o.a.y0 > e);
    for (let pass = 0, more = true; more && pass < 8; pass++) {
      more = false;
      for (const t of out) {
        if (t.via !== 'furniture' || t.ghost) continue;
        if (!stays.some(o => Math.abs(o.a.y0 - t.a.y1) <= e && o.a.x0 < t.a.x1 - 0.01 && o.a.x1 > t.a.x0 + 0.01 && o.a.z0 < t.a.z1 - 0.01 && o.a.z1 > t.a.z0 + 0.01)) continue;
        t.via = 'structure'; t.why = 'carries what stays'; if (t.a.y0 > e) stays.push(t); more = true;
      }
    }
    return out;
  }

  /** WHAT ONE BOX IS. The order is the rule: the first line that fits names it. */
  private classify(t: Thing, b: Box, f0: number, lights: Set<number>, breaker: Set<number>, fronts: Vec3[]) {
    const K = THINGS, a = t.a, walls = this.destr.walls;
    const set = (via: ThingVia, size: ThingSize, why = '') => { t.via = via; t.size = size; t.why = why; };
    const sx = a.x1 - a.x0, sz = a.z1 - a.z0, tall = a.y1 - a.y0, long = Math.max(b.h[0], b.h[2]) * 2, thin = Math.min(b.h[0], b.h[1], b.h[2]) * 2, vol = b.h[0] * b.h[1] * b.h[2] * 8;
    const sized: ThingSize = vol > K.heavy.vol || long > (thin <= K.heavy.thin ? K.heavy.longThin : K.heavy.long) ? 'heavy' : vol <= K.small.vol && Math.max(long, tall) <= K.small.side ? 'small' : 'furniture';
    t.hold = 'stands';   // (settled by make() once every row is named; 'lies' below is final)
    // not in the level a body meets
    if (f0 & (BoxFlag.NoPrimary | BoxFlag.Dynamic)) return set('unseen', 'small', 'not drawn');
    if (b.c[0] < WORLD.min[0] || b.c[0] > WORLD.max[0] || b.c[2] < WORLD.min[2] || b.c[2] > WORLD.max[2]) return set('structure', 'structure', 'outside the level');
    // the engines that already know their own
    if (f0 & BoxFlag.Glass) return set('glass', sized);
    const br = walls.breakableAt(t.i);
    if (br && br.kind === 'cover' && sized !== 'heavy' && a.y1 <= K.maxTop && (t.mat === 'wood' || t.mat === 'board') && (Math.min(b.h[0], b.h[2]) * 2 <= K.heavy.thin || vol <= K.panelVol))
      return set('furniture', sized, 'a panel');   // (a desk's panel, a small carton: cover the wall engine cuts by the round AND a part of the piece it belongs to, which razes it — a crate that is real cover stays the wall engine's alone)
    if (br) return set(br.kind === 'wall' ? 'walls' : 'cover', br.kind === 'wall' ? 'structure' : sized);
    if (this.destr.itemOf(t.i)) return set('item', 'small');
    // what play needs whole
    if (breaker.has(t.i)) return set('kept', sized, 'the breaker');
    if (lights.has(t.i) || b.emissive[0] + b.emissive[1] + b.emissive[2] > 0) return set('kept', sized, 'light');
    // the building
    const name = paletteName(b.albedo);
    if (t.mat === 'snow') { if (a.y0 <= 0.25) t.hold = 'lies'; return set('structure', 'structure', 'lying snow'); }   // (snow lying ON a thing is that thing's: make())
    if (tall < K.flatM && a.y0 <= 0.25) { t.hold = 'lies'; return set('structure', 'structure', 'a finish'); }
    if (name && GLAZING.has(name)) return set('structure', 'structure', 'a window\'s frame');
    if (a.y0 >= K.overheadM) return set('structure', 'structure', 'overhead');
    if (a.y0 <= 0.25 && a.y1 > K.maxTop) return set('structure', 'structure', Math.max(sx, sz) <= K.post.thick ? 'a post' : 'full height');
    if (Math.max(sx, sz) <= K.post.thick && tall >= K.post.tall && !t.ghost) return set('structure', 'structure', 'a post');
    if (name && BUILDING.has(name) && (sized === 'heavy' || (GROUND.has(name) && tall < 0.06))) return set('structure', 'structure', 'the building');
    // a thing
    const front = fronts.some(f => f[0] > a.x0 - 0.3 && f[0] < a.x1 + 0.3 && f[2] > a.z0 - 0.3 && f[2] < a.z1 + 0.3);
    if (sized === 'heavy') {
      if (!K.every) return set('structure', 'heavy', 'too heavy to move');
      if (t.ghost) return set('structure', 'heavy', 'a finish');   // (a long drawn-only sheet — a dado, a stripe down a wall: the building's dress, with no body to break)
      if (front) return set('kept', sized, 'the mission\'s thing');
      return set('furniture', 'heavy');   // (a thing: the furniture engine breaks it in the ring its weight and stuff ask for — furnblast.ts `core`)
    }
    if (front) return set('kept', sized, 'the mission\'s thing');
    set('furniture', sized);
  }

  /** counts for the panel, the census and the checks: by engine, and of the things no engine breaks, by why */
  census(): { n: number; via: Record<string, number>; why: Record<string, number>; mat: Record<string, number>; hold: Record<string, number>; size: Record<string, number> } {
    const via: Record<string, number> = {}, why: Record<string, number> = {}, mat: Record<string, number> = {}, hold: Record<string, number> = {}, size: Record<string, number> = {};
    const add = (m: Record<string, number>, k: string) => { m[k] = (m[k] ?? 0) + 1; };
    for (const t of this.all()) { add(via, t.via); if (t.via === 'structure' || t.via === 'kept') add(why, t.why || t.via); if (t.via !== 'unseen' && t.via !== 'structure') { add(mat, t.mat); add(hold, t.hold); add(size, t.size); } }
    return { n: this.all().length, via, why, mat, hold, size };
  }
}
