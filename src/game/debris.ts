// Rigid DEBRIS (the destruction lane, game/destruction.ts): the few things a firefight knocks loose that must stay GEOMETRY — a ceiling tile see-sawing down,
// a monitor housing off its stand, a mug or a run of binders off a desk — as a small POOL of tumbling boxes: ballistic flight with the throwables' swept
// sphere-vs-box contact against the statics, leaves and furniture (the support distance is the box's own extent along the surface normal, so a tile lands
// on its face and a binder on its edge), dead-ish bounces, a turn toward lying flat on each floor touch, settle, SLEEP; drawn as one dynamic shadow-casting
// box each. Bounded by construction (renderer v3 stage 2's chaos budget, docs/internal/image-quality.md §4: 64 tumbling casters cost +2 ms at high with no
// boil): the pool holds at most `cap()` chunks (settings.debrisCap: 64 ultra / high, 32 below) — a spawn over the cap evicts the oldest chunk already AT REST,
// and with none at rest it REFUSES (null) and the caller demotes the break-off to a particle burst (Control's rule). Landing is heard: the first hard touch
// plays its knock and files a 'prop' hearing event at the chunk's level (a tile 0.4, a monitor 0.5, clutter 0.25) blamed on WHOEVER CAUSED IT (Chunk.who: the
// shooter whose round knocked it off / dropped the tile, the thrower of the charge that shoved it — so a guard's own burst is his side's clumsiness to the men
// and no row on Sam's debrief, Sam's is heard and chronicled as his; NOBODY_ID when no one did: unexplained to the men, nobody's in the debrief). Deterministic: no Math.random — every wobble is hashed off the chunk's serial. Cleared by a restart / the tour's cuts with the rest of the aftermath.
import { Vec3, v3, quat } from '../math/vec';
import { Box, BoxFlag, makeBox, MKEY, wordOf } from '../scene/boxes';
import type { Game } from './game';
import type { Prop } from './props';
import { impulseKick, type ImpulseLike, type ImpulseResponse } from './sheetModel';
import { NOBODY_ID } from './consts';
import { DEBRIS } from './destructionSwitch';
import { DRESS } from './dress';
import { flightOf } from './falling';
import { FALL_PIECES, FALL_HEARD, FALL_MAXP } from '../sim/fallshome';
import type { PlateMeta } from './shards';
import type { BreakMeta } from './breakTable';
import { RUST_PASS, RUST_SURFACE } from './crossing';

export type ChunkKind = 'tile' | 'housing' | 'clutter' | 'binder' | 'chunk' | 'shard';   // (shard: a fallen cell of a glass pane — game/glassbreak.ts)
export interface Chunk {
  kind: ChunkKind; box: Box; v: Vec3; w: Vec3;
  /** at rest (asleep); seconds spent nearly still (toward sleep); age; whether its first hard landing has sounded */
  rest: boolean; stillT: number; age: number; sounded: boolean;
  /** hearing level of its landing ('prop' event) and the knock it plays */
  level: number; sfx: { name: string; rate: number; gain: number }[];
  serial: number;
  /** who is blamed for its landing (the character id whose round / charge set it going; NOBODY_ID = no one) */
  who: number;
  /** the prop it came to rest on (drops again once that prop moves) and that prop's pose key then */
  on: Prop | null; onPose: number;
  /** GLASS only (game/shards.ts — absent on every other chunk, and on a shard while that model is off): what the plate is, and its own contact numbers (the share of
   *  its slide kept on a floor / a wall touch, the share of its normal speed it comes back with) in place of the pool's 0.55 / 0.8 / 0.22 */
  glass?: PlateMeta; contact?: { keepFloor: number; keepWall: number; rest: number };
  /** CUT PIECES only (game/breakTable.ts — absent while the cut switch is off): the material row it reads and whether it has broken once already */
  mat?: BreakMeta;
  /** CUT PIECES only: a second drawn box riding this chunk's pose (the other half of a kinked metal panel: same box, its own cut record). One pool slot. */
  mate?: Box;
  /** KICKED RUBBLE only (Debris.feet below — never set while RUBBLE.on is off): set going by a BOOT, not thrown — until it sleeps again (or a blast takes it) it SLIDES
   *  on its floor (slide(), not step()): no landing, so no knock and no hearing event, it pushes no prop it meets, breaks no further, and its slot can still be
   *  taken; `footAt` = its age at the last shove, `footN` = shoves in a row (a piece a man keeps pressing into a wall is let be for a while), `footSide` = the side
   *  of his line his last kick sent it to */
  foot?: boolean; footAt?: number; footN?: number; footSide?: number;
  /** DRESSING: this chunk is picture only — it writes nothing of the game (no furniture feels it land, the floor is not told by it, no ray of the game's meets
   *  its box; whoever made it has already told the simulation what falls where). Every chunk carries it under DEBRIS.dress (destructionSwitch.ts), a glass
   *  plate under shards.ts SHARDS.byFall. Absent = a chunk as it always was. */
  dress?: boolean;
  /** m: a loose piece's own height over the floor it is seated on (edgeLook.ts's fragments: no two overlapping flakes laid flat with their tops in one plane); a pool piece has none */
  lift?: number;
}
/** the feet's books for one list of pieces: shoves made (its hash stream) and the counts. The pool keeps its own; a caller with a list of its own (edgeLook.ts)
 *  keeps its own too, so what the pool's pieces draw never depends on a picture-only list having been kicked first */
export interface FootBook { shoves: number; stats: { kicked: number; offMan: number; walled: number; gaveUp: number } }
/** what Debris.feet reads of a man: where his feet are and how he is moving (a Character is one) */
export interface FootMan { pos: Vec3; vel: Vec3 }
/** KICKED RUBBLE's numbers (the table is game/destructionSwitch.ts RUBBLE — no game import there; this is its shape) */
export interface FootKnobs { radius: number; reach: number; lead: number; slide: number; inside: number; floor: number; heavy: number; walk: number; base: number; gain: number; aside: number; vMax: number; off: number; spin: number; perFrame: number; cooldown: number; giveUp: number; giveUpSecs: number; scuff: number }
const G = 9.81;
/** the chunks' IMPULSE response (destruction.ts hands the pool the same records the GPU particles get: a charge wakes and shoves the mug it knocked off a minute ago):
 *  a rigid thing is heavy next to a leaf — a third of a sheet's push, a modest hop, capped at a brisk 3 m/s; a tenth of a metre a second wakes it (a round landing a
 *  foot off rocks it — under that it would not be seen to move); TUMBLE = rad/s of hashed spin per m/s of kick ÷ its size */
export const DEBRIS_IMP: ImpulseResponse & { TUMBLE: number } = { gain: 0.35, upK: 0.7, vMax: 3.0, wake: 0.1, TUMBLE: 0.5 };
/** hearing level + knock per kind */
const LANDING: Record<ChunkKind, { level: number; sfx: { name: string; rate: number; gain: number }[] }> = {
  tile:    { level: 0.4,  sfx: [{ name: 'propThud', rate: 0.75, gain: 0.9 }, { name: 'magDrop', rate: 0.5, gain: 0.35 }] },
  housing: { level: 0.5,  sfx: [{ name: 'propThud', rate: 0.6, gain: 1.0 }, { name: 'lightBreak', rate: 0.8, gain: 0.45 }] },
  clutter: { level: 0.25, sfx: [{ name: 'propThud', rate: 1.45, gain: 0.5 }] },
  binder:  { level: 0.25, sfx: [{ name: 'propThud', rate: 1.1, gain: 0.55 }] },
  chunk:   { level: 0.2,  sfx: [{ name: 'propThud', rate: 1.3, gain: 0.4 }] },
  shard:   { level: 0.3,  sfx: [{ name: 'glassCrack', rate: 1.9, gain: 0.3 }, { name: 'casingTink', rate: 0.6, gain: 0.35 }] },   // a piece of pane landing: a bright tinkle
};

/** 0‥1 off an integer (a hash, not a die) */
export function hash01(x: number): number { x = Math.imul(x ^ 0x9E3779B9, 0x85EBCA6B) >>> 0; x ^= x >>> 13; x = Math.imul(x, 0xC2B2AE35) >>> 0; x ^= x >>> 16; return (x >>> 8) / 16777216; }

export class Debris {
  list: Chunk[] = [];
  private serial = 1; private frame = 0;
  /** running totals for the panel / QA: spawned, refused at the cap (demoted by the caller), evicted at rest, landings heard */
  stats = { spawned: 0, refused: 0, evicted: 0, landed: 0, peakLive: 0, peakMoving: 0, kicked: 0, told: 0 };
  constructor(private g: Game) {}

  /** the pool's size on this tier (ui/quality.ts Caps.debris → settings.debrisCap; 64 without one) */
  cap(): number { const c = (this.g.engine.settings as { debrisCap?: number }).debrisCap; return Math.max(4, Math.floor(c ?? 64)); }
  get moving(): number { let n = 0; for (const c of this.list) if (!c.rest && !c.foot) n++; return n; }   // (thrown and not yet down: a piece a boot slid is not part of the mayhem)

  /** A chunk of `kind` = the box `box` (taken over as is: centre, half extents, yaw / rot, albedo) thrown at `v` (m/s) turning at `w` (rad/s), its landing blamed on
   *  `who`. null when the pool is full of chunks still moving — the caller demotes it to particles. */
  spawn(kind: ChunkKind, box: Box, v: Vec3, w: Vec3, who: number = NOBODY_ID, level = 0): Chunk | null {
    if (kind === 'shard' && this.glass) { const r = this.glass.take(box, v, w, who); if (r !== undefined) return r; }   // glass after it lets go is game/shards.ts's (undefined = not live: as it always was)
    if (!DEBRIS.dress) return this.spawnRaw(kind, box, v, w, who);
    if (this.fling) v = v3.add(v, this.fling);
    this.thrown(kind, box, v, level, who);   // the game's half, decided before the pool is asked: whether it takes the piece changes nothing of it
    return this.spawnRaw(kind, box, v, w, who, true);
  }
  /** THE SIMULATION'S HALF OF A PIECE THAT IS THROWN (DEBRIS.dress — destructionSwitch.ts): its one landing, put in the game's falls. When and where =
   *  falling.ts flightOf, the simulation's answer for a thing that is thrown (the arc of a body let go at the piece's middle with its speed, stopped short
   *  of the first static or pushable thing it meets, down to what is under it there); nothing if it would land slower than DEBRIS.minSpeed. It lands at its row's
   *  hearing level, and raises the dust its row says a piece that hard a landing breaks again with (`breaks.lands`: a ceiling tile's puff). Every number
   *  here is the simulation's own (the box and the speed the caller's rule made, the statics): no chunk is read. */
  private thrown(kind: ChunkKind, box: Box, v: Vec3, level: number, who: number) {
    const g = this.g, c = box.c, f = flightOf(this.solids, c, v, G);
    const down = Math.sqrt(v[1] * v[1] + 2 * G * Math.max(0, c[1] - f.pos[1]));
    if (!(down > DEBRIS.minSpeed)) return;
    const L = this.breaks?.lands?.(kind, box, down);
    if (g.falls.schedule(g.time + f.t, f.pos[0], f.pos[1], f.pos[2], '', 0, 1, 0, who, FALL_HEARD, level || L?.level || LANDING[kind].level, FALL_MAXP, FALL_PIECES, L?.dust ?? 0)) this.stats.told++;   // (a list already full: the piece is not heard — FallStore.schedule — and not counted as told)
  }
  /** what a thrown piece can meet, for the simulation's answer: the statics and the pushable things, as a chunk's own flight asks them */
  private solids = { raycast: (ro: Vec3, rd: Vec3, tmax: number): { t: number } | null => { const st = this.g.col.raycast(ro, rd, tmax), ph = this.g.props.raycast(ro, rd, st ? st.t : tmax); return ph ?? st; } };
  /** (DEBRIS.dress) a speed added to every piece thrown through spawn() while it is set: a caller that used to throw a thing by another rule's road and then
   *  shove the chunks that appeared (furnblast.ts: a blast on the lane's own destructible) says the shove BEFORE, so the simulation's landing has it too */
  fling: Vec3 | null = null;
  /** GLASS HOOKS (game/shards.ts sets them; null = this pool exactly as it was): `take` is offered every 'shard' spawn first (a Chunk / null = it placed the fragment
   *  itself, undefined = not its business), `landed` a glass plate's first hard touch (true = it made the sound and filed the hearing event), `rested` a glass plate
   *  going to sleep, `gone` (optional) a glass plate the POOL has just dropped — evicted at rest to make room, or retired by age. Only chunks carrying `glass` reach the last three. */
  glass: { take(box: Box, v: Vec3, w: Vec3, who: number): Chunk | null | undefined; landed(c: Chunk, speed: number): boolean; rested(c: Chunk): void; gone?(c: Chunk): void;
    /** (optional) are the glass plates dressing now (shards.ts SHARDS.byFall)? */
    dress?(): boolean } | null = null;
  /** THE BREAK TABLE'S HOOKS (game/breakTable.ts sets them; each is nothing while the cut switch is off — null or off = this pool exactly as it was): every chunk
   *  born, a piece's first hard touch, an impulse's kick on a piece, a resting piece evicted for its slot (the ONE crumble-on-evict rule: glass plates included). */
  breaks: { born(c: Chunk): void; landed(c: Chunk, speed: number): void; kicked(c: Chunk, kick: number): void; evicted(c: Chunk): void;
    /** (optional) for the simulation's landing of a piece of this kind and paint coming down at `speed` m/s: the hearing level of its material's row (0 = its
     *  kind's) and the dust it raises if that is hard enough to break it again (a puff's scale; 0 = none) */
    lands?(kind: ChunkKind, box: Box, speed: number): { level: number; dust: number };
    /** (optional) WHICH piece gives up its slot, among those `ok` allows: an index into the list, −1 = none. Absent = the oldest, as ever. */
    pick?(list: readonly Chunk[], ok: (c: Chunk) => boolean): number } | null = null;
  /** spawn() without the glass door (what spawn() always did; shards.ts places its plates through this) */
  spawnRaw(kind: ChunkKind, box: Box, v: Vec3, w: Vec3, who: number = NOBODY_ID, told = false): Chunk | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Debris.spawnRaw', this, [kind, box, v, w, who, told]); if (__r !== RUST_PASS) return __r as any; } }
    const dress = DEBRIS.dress && (told || kind !== 'shard' || !this.glass || !!this.glass.dress?.());   // (is this piece dressing? every piece under DEBRIS.dress whose landing the simulation has — `told`: spawn()'s — and the pool's own; but a glass plate that still lays its litter where it rests, SHARDS.byFall off, is the game's)
    if (dress && !DRESS.debris) { this.stats.refused++; return null; }   // (dress.ts: a host that draws nothing holds no pool, where the pool is dressing)
    while (this.list.length >= this.cap()) {   // (while: the cap can drop mid-run — the auto ladder hopping a preset — and the pool must come back under it, not hover one over)
      const i = this.breaks?.pick ? this.breaks.pick(this.list, evictable) : this.list.findIndex(evictable);
      if (i < 0) { this.stats.refused++; return null; }
      const gone = this.list.splice(i, 1)[0]; this.stats.evicted++; this.breaks?.evicted(gone);
    }
    const serial = this.serial++;
    const b = makeBox({ c: v3.copy(box.c), h: v3.copy(box.h), yaw: box.yaw, rot: box.rot ? [box.rot[0], box.rot[1], box.rot[2], box.rot[3]] : quat.yaw(box.yaw), albedo: v3.copy(box.albedo), ...wordOf(box), flags: BoxFlag.Dynamic, name: 'debris', mkey: MKEY.debris + (serial & 65535) });
    const L = LANDING[kind];
    const c: Chunk = { kind, box: b, v: v3.copy(v), w: v3.copy(w), rest: false, stillT: 0, age: 0, sounded: false, level: L.level, sfx: L.sfx, serial, who, on: null, onPose: 0 };
    if (dress) c.dress = true;   // (nothing of the game feels it: no push on what it lands on, no word to the floor, no ray of the game's meets it)
    this.list.push(c); this.stats.spawned++; this.breaks?.born(c);
    return c;
  }
  clear() { if (DEBRIS.dress || this.g.falls.waitingIn(FALL_PIECES)) this.g.falls.clear(false, FALL_PIECES); this.fling = null; this.list.length = 0; this.impulses = 0; this.serial0 = this.serial; this.shoves = 0; this.scuffAt = -1; this.feetStats = { kicked: 0, offMan: 0, walled: 0, gaveUp: 0 }; } serial0 = 1;   // (+ the impulses' hash stream and the feet's; `serial` runs on so motion keys stay unique across restarts — a restarted encounter's wobbles differ by it, two fresh games agree; `serial0` = the serial at the last clear, read by game/breakTable.ts alone)
  boxes(out: Box[]) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Debris.boxes', this, [out]); if (__r !== RUST_PASS) return __r as any; } } for (const c of this.list) { if (c.dress) c.box.dress = true; out.push(c.box); const m = c.mate; if (m) { m.c = c.box.c; m.yaw = c.box.yaw; m.rot = c.box.rot; m.dress = c.box.dress; out.push(m); } } }
  /** An IMPULSE over the pool (the same record the GPU particles get, game/sheetModel.ts impulseKick: strength × DEBRIS_IMP.gain × (1 − d/r) along radial + up +
   *  bias): every chunk within reach kicked at least DEBRIS_IMP.wake is woken if it rests, shoved, capped at vMax and set turning by a hashed wobble ∝ the kick ÷ its
   *  size; `blocked(p)` (the caller's line-of-sight test, or none) leaves a chunk behind a partition alone; `who` (when given) takes the blame for its next landing.
   *  Returns how many it moved. Deterministic (hash01 off the chunk's serial and the pool's impulse count). */
  impulse(imp: ImpulseLike, blocked?: (p: Vec3) => boolean, who?: number): number {
    if (!this.list.length) return 0;
    let n = 0; const k = ++this.impulses;
    for (const c of this.list) {
      if (c.age <= 0) continue;   // knocked off this very frame BY the event whose impulse this is: it already has its shove (the velocity policy's), not a second one
      const kick = impulseKick(imp, c.box.c, DEBRIS_IMP); if (!kick || kick.s < DEBRIS_IMP.wake) continue;
      if (blocked && v3.dist(c.box.c, imp.pos) > 0.5 && blocked(c.box.c)) continue;
      if (c.rest) { c.rest = false; c.on = null; c.stillT = 0; }
      if (c.foot) c.foot = false;   // (a blast has it now: thrown, heard and breakable like any other)
      let v = v3.add(c.v, kick.dv); const sp = v3.len(v); if (sp > DEBRIS_IMP.vMax) v = v3.scale(v, DEBRIS_IMP.vMax / sp); c.v = v;
      const h = (j: number) => hash01(c.serial * 977 + k * 131 + j) - 0.5;
      const tw = DEBRIS_IMP.TUMBLE * kick.s / Math.max(0.08, boundR(c.box));
      c.w = v3.add(c.w, [h(1) * tw, h(2) * tw * 0.5, h(3) * tw]);
      c.sounded = false; if (who !== undefined) c.who = who; this.breaks?.kicked(c, kick.s);   // (thrown again: its next hard landing is a fresh knock to hear, on whoever threw the charge)
      n++;
    }
    this.stats.kicked += n;
    return n;
  }
  /** impulses applied so far (hash stream) */
  private impulses = 0;

  private footMu = 0.3;
  /** shoves made so far (the feet's own hash stream — not the impulses') and when the last scuff played */
  private shoves = 0; private scuffAt = -1;
  /** KICKED RUBBLE's counts (its own object: `stats` is as it was) — kicked ahead of a walking man, pushed off a man it lay in, tries a wall stopped, pieces given up on */
  feetStats = { kicked: 0, offMan: 0, walled: 0, gaveUp: 0 };


  /** panel / QA line */
  summary(): string { const s = this.stats; return `debris ${this.list.length}/${this.cap()} (${this.moving} moving) · spawned ${s.spawned} · refused ${s.refused} · evicted ${s.evicted} · landings ${s.landed} · peak ${s.peakLive}/${s.peakMoving} moving`; }
}

/** a slot the pool may take: a piece at rest — or one only a boot has set sliding (Chunk.foot: never set while RUBBLE.on is off) */
const evictable = (c: Chunk) => c.rest || c.foot === true;
function boundR(b: Box): number { return Math.hypot(b.h[0], b.h[1], b.h[2]); }
