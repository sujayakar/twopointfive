// BOTH SIDES REACT WHEN THE LEVEL BREAKS (the destruction goal's AI part; notes: rebuild/destruction/ai-react.md).
//
// The destruction engine (wallbreak.ts) keeps the WORLD true the moment a wall or a piece of cover goes: the picture, rounds, eyes, sound, feet and the nav grid
// all take the same change, and one row per change is filed in its feed (`g.destr.walls.onChange`). What it cannot keep true is what the AI built ONCE per
// level and what a man decided a moment ago: the room he thinks he is in, the openings his team watches, the mark he is standing on. This module reads the
// feed and keeps those true — by each man's own senses, on both sides, with nothing scripted:
//
//   (a) COVER GONE. A man standing within a few strides of a change asks one question: from where the trouble is (the man he last saw, where the rounds come
//       from, whoever made the hole, the far side of a hole beside him), is my chest open NOW along a line that crosses what just went (WALLREACT.everyGone:
//       the newest thing first, then each thing that went before it while he was owed the answer; off, the newest row's alone)? If so he moves:
//       a defender holding a room takes the room fight's own cover move and plants again where he ends up; a man in the fight AI takes its shift-to-cover; a
//       teammate on a mark has his mark moved to the nearest spot out of that line (a stack slides along its wall). A man who cannot move this second (blind,
//       held, eyes on a man at arm's length) is asked again for a few seconds.
//   (b) A NEW OPENING. A breach (a port: the wall's upper knocked out; a hole: floor to top) is NOTICED by the men who hear it open (the sound model's own path
//       length, a blast far, a wall shot through near), who see it (a line to it, in front of them) or who made it — fog of war stands: nobody else knows.
//       The team shares what one of them knows (they are on a net); a defender tells the men in earshot a breath later. Known to the team it becomes one of
//       the squad's OPENINGS (cover.ts): the unasked cover deals it to a free gun like any doorway, and the entry points count it as a way a round can come
//       (entrypoints.ts — a point of an entry is not left in its line while what is beyond is unknown). Known to a defender it is weighed against the door he
//       watches (roomfight.ts applyWatch → wallReactLook): hearing it go, a man seen through it and rounds through it are evidence, a hole into his room is
//       worth a gun by itself; his gun, his eyes and his fix go onto it.
//   (c) A WAY THROUGH. The nav grid is already re-baked, so every walk planned from now on may go through a hole. What was flooded before is refreshed: the
//       room fight's rooms on either side of a walk-through hole become ONE room for everybody (the same Room object for the men who held it — their evidence
//       is carried over door by door), so nobody counts a mate through a hole as 'in the next room', breaks off to 'the other side' of a wall that is gone, or
//       holds a room as sealed with a hole in it. The team's cleared-rooms book (cleared.ts) needs nothing: a hole's floor is no room's, exactly like a
//       doorway — what is beyond it stays UNCLEARED until one of the team stands there. A flanking plan under way (tactics.ts) is dealt again once when one of
//       its men learns of a hole a man fits through.
//
// SWITCH: WALLREACT.on (default on) — and nothing here runs without THE ONE SWITCH of the destruction goal (destructionSwitch.ts), because no row is ever
// filed without it. Off = the engine's build by digest: every hook returns on its first line. Everything is this module's own state (a WeakMap by Game), reset
// with the encounter. Deterministic: no dice of its own (a man's bark goes through g.say like any other).
//
// NOT here (other pieces'): who decides to shoot at cover or through a wall, who places a charge, doors off their hinges, glass.
import { isManId, menUp, teamActive, teamMen, orderOf } from './seats';
import { SideMap } from '../math/side';
import { v3, type Vec3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import type { Opening } from './cover';
import type { Assignment } from './orders';
import type { LevelChange } from './wallbreak';
import type { Team } from './teams';
import { isHostile } from './teams';
import './suppression';
import './squad';
import './guards';
import './digin';
import { coverfireOn } from './coverfire';
import './wallcharge';
import { WORDS } from './words';
import './owner';

export const WALLREACT = {
  /** the switch (and only ever live under the destruction goal's own) */
  on: true,
  // ---- noticing a breach
  /** a man sees a breach he has a line to inside this, in a cone `fovDeg` wide about where he faces (the guards' own eye cone is 116°) */
  seeM: 18, fovDeg: 116,
  /** path length (the sound model's) a wall going is heard at: by a blast · by fire · a collapse */
  hearBlastM: 26, hearFireM: 10, hearFallM: 14,
  /** a defender who learned of one by his own senses tells his side's men within this much sound path, this many seconds later */
  tellM: 12, tellSecs: 1.2,
  // ---- cover gone
  /** a man this near what changed asks whether he is still covered */
  nearM: 2.6,
  /** … and is asked again for this long while he cannot move (blind, held, a man at arm's length) */
  retrySecs: 5,
  /** EVERY THING THAT WENT: one blast can file two rows beside a man — the crate he kneels behind, then the furniture round it — and the one question kept for
   *  him was the newest row's, so he was asked about the cupboard, never about his crate ('nothing beyond it to hide from'), and knelt on in the open. On: the
   *  question he is still owed keeps each thing that went before the newest too — its rect, its breach, whose doing it was — and where the newest has nothing
   *  to say to him the one before it is asked, each exactly as a row alone is asked (so a newest row that moves him moves him as it always did). No more than
   *  `goneMax` things a question: past that the one furthest from him is let go. OFF here and on the frozen stand (`TPS_EVERYGONE=1`), ON in the page (`?everygone=0` = the newest row alone, as it was); a
   *  recording from before the key replays with it off. */
  everyGone: false, goneMax: 6,
  /** ONE BLAST, ONE FLOOD: a blast that opens two walls files two rows, and the first was flooded with the second hole already in the floor — its fill ran on
   *  through that hole into a room the row never touched and stopped there as 'the same room' (roomfight.ts roomAt), a room whose door list was still its old
   *  one; the defenders' evidence was carried over onto that short list and what they knew of every other door was dropped (the second row's flood put the
   *  doors back, not the evidence). On: the rows of one frame are flooded as one — every room any of them touches, once. OFF here and on the frozen stand
   *  (`TPS_BLASTFLOOD=1`), ON in the page (`?blastflood=0` = a flood a row, as it was); a recording from before the key replays with it off. */
  blastFlood: false,
  /** a mark is not moved further than this */
  stepM: 3.5,
  /** a holding man with nothing nearer runs this far for a corner */
  runM: 9,
  /** the far side of a hole beside a man counts as where trouble is from this far beyond it */
  beyondM: 2.5,
  // ---- watching a breach (defenders)
  /** evidence: hearing or seeing it open · per second with an enemy in sight through it · per second with rounds coming through it · forgotten per second */
  evOpen: 1.6, evSeen: 1.5, evFire: 1.0, forget: 0.12,
  /** what a breach is worth with no evidence at all: one a man fits through · a port · less per mate already on it · kept once on it */
  holeBase: 0.45, portBase: 0.2, crowd: 0.35, sticky: 0.15,
  /** … and for this long after a man learns of one it outweighs any door: he looks at what just opened */
  freshSecs: 3, fresh: 3,
  /** … inside this, with a line to it */
  watchM: 16,
  /** barks */
  say: true,
};
export const WALLREACT_STATS = { rows: 0, breaches: 0, merged: 0, learnedOps: 0, learnedMen: 0, told: 0, movedDef: 0, movedAI: 0, movedOp: 0, noCover: 0, watches: 0, replans: 0, openings: 0 };
export const WALLREACT_LOG: string[] = [];
/** a tool's hook (tools/qa/wallreact-play.ts blows its holes from here): called at the top of every tick whatever the switch says, so an on / off pair of runs
 *  gets the same blasts */
/** a probe's ear: why each candidate spot was or was not taken */
export const WALLREACT_DEBUG: { log: ((s: string) => void) | null } = { log: null };
export const WALLREACT_TEST: { tick: ((g: Game) => void) | null } = { tick: null };

/** One opening in a wall: a port (eyes and rounds) or a hole (feet too). `u` runs along the wall, `n` through it; `lo`‥`hi` along `u` from `c0`. */
export interface Breach {
  key: string; wall: number; c0: [number, number]; u: [number, number]; n: [number, number]; lo: number; hi: number;
  /** its middle, chest high · half its width */
  at: Vec3; half: number;
  /** grown into another breach of its wall and out of the book: that one (WALLREACT.everyGone: a thing a question keeps is about the breach that is left) */
  into?: Breach;
  walk: boolean; t: number; cause: LevelChange['cause']; who: number;
  /** who blew the charge that made it a blown wall, where that was not its first opener (`who` stays the first: other readers key on it) — read only for the line they say */
  blastBy?: number;
  /** the team knows it (since when) · the defenders who know it (since when) */
  ops: number; men: Map<Guard, number>;
  /** somebody has said so, per side · the men owed the news, by whom, and when */
  said: Set<Team>; tell: { by: Guard; at: number }[];
  /** the squad's opening for it (cover.ts), once the team knows */
  opening: Opening | null;
}
interface Mind { ev: number; line: boolean; lineT: number; t: number }
/** one thing that went, as a question holds it: where, the breach it made (a wall's) and whose doing it was */
interface Gone { rect: [number, number, number, number]; b: Breach | null; who: number }
interface Pend extends Gone { gd: Guard; /** WALLREACT.everyGone: what went before this while he was still owed the question, newest first */ also?: Gone[]; until: number; next: number }
// (roomfight.ts keeps these to itself: the shapes this module reads and re-floods, mirrored)
interface Entry { door: Door; at: Vec3; out: [number, number] }
interface Room { id: number; cells: Set<number>; entries: Entry[] }
interface Def { gd: Guard; room: Room | null; ev: number[]; watch: number; want: number; glanceTo: number; glanceUntil: number; mode: 'hold' | 'move' | 'free'; moveKind: 'fallback' | 'cover'; moveT: number; cameBy: Door | null; stallAt: Vec3; stallT: number; movedAt: number; station: Vec3; pt: Vec3 }
interface FightBooks { rooms: Room[]; roomOfCell: Map<number, Room>; defs: Def[]; roomAt(p: Vec3): Room | null; stats: { tookCover: number }; log: { t: number; who: string; what: string }[] }

const flat = v3.distXZ;
const EYE = 1.55, CHEST = 1.1;
const note = (g: Game, s: string) => { WALLREACT_LOG.push(`${g.time.toFixed(2)} ${s}`); if (WALLREACT_LOG.length > 400) WALLREACT_LOG.splice(0, 200); };
const rectDist = (p: Vec3, r: [number, number, number, number]) => Math.hypot(Math.max(r[0] - p[0], 0, p[0] - r[2]), Math.max(r[1] - p[2], 0, p[2] - r[3]));

export class WallReact {
  breaches: Breach[] = [];
  private queue: LevelChange[] = []; private stale = false;
  private pend: Pend[] = [];
  /** WALLREACT.blastFlood: the floor all of this frame's rows opened, until the first of them has flooded it (then empty); null = a flood a row */
  private floodAll: number[] | null = null;
  private minds = new Map<Guard, Map<Breach, Mind>>();
  /** who rests his gun on which breach right now (defenders) */
  watching = new Map<Guard, Breach>();
  private last = -1; private lastEv = -1; private lastSweep = -1;
  private movedT = new SideMap<Guard, number>();
  private unsub: (() => void) | null = null;
  constructor(private g: Game) {
    const W = g.destr?.walls; if (!W) return;
    this.unsub = W.onChange(row => { if (row.kind === 'reset') this.stale = true; if (WALLREACT.on) this.queue.push(row); });
  }
  private get fight(): FightBooks { return this.g.fight as unknown as FightBooks; }
  private defOf(gd: Guard): Def | null { return this.fight.defs.find(D => D.gd === gd) ?? null; }

  /** a restart (Game.resetGuards): nothing known, nobody owed a move. The breaches themselves go when the walls are put back (the feed's 'reset' row). */
  reset() { this.pend.length = 0; this.minds.clear(); this.watching.clear(); this.last = this.lastEv = this.lastSweep = -1; for (const b of this.breaches) { b.ops = -1; b.men.clear(); b.said.clear(); b.tell.length = 0; } this.dropOpenings(); }


  // ================================================================ the feed
  private take(row: LevelChange) {
    if (row.kind === 'reset' || row.kind === 'door') return;   // (a door off its hinges is the doors' own piece)
    WALLREACT_STATS.rows++;
    const b = row.kind === 'cover' ? null : this.upsert(row);
    if (row.opened.length) { if (!this.floodAll) this.reflood(row.opened); else if (this.floodAll.length) { this.reflood(this.floodAll); this.floodAll = []; } }
    if (b) this.heard(b, row);
    const g = this.g;
    const asked = new Set<Guard>();   // (a teammate standing on his mark is beside it by his feet AND by his mark: one question)
    const ask = (gd: Guard) => { if (asked.has(gd)) return; asked.add(gd);
      const i = this.pend.findIndex(x => x.gd === gd), old = i >= 0 && WALLREACT.everyGone ? this.pend[i] : null;
      const P: Pend = { gd, rect: row.rect, b, who: row.who, until: g.time + WALLREACT.retrySecs, next: g.time };
      if (old) { let ob = old.b; while (ob?.into) ob = ob.into; P.also = this.kept(orderOf(g, gd)?.post ?? gd.char.pos, [{ rect: old.rect, b: ob, who: old.who }, ...(old.also ?? [])]); }   // (everyGone: what went before this row is still his to be asked about, when the newest thing has nothing to say to him)
      if (i >= 0) this.pend[i] = P; else this.pend.push(P); };   // (one question a man. With everyGone off it is about the newest thing to go and nothing else; on, the things that went before it while he was owed the answer wait behind it, no more than goneMax in all)
    const own = (gd: Guard) => row.cause === 'fire' && gd.char.id === row.who && coverfireOn(g, gd) !== null;   // (the man whose aimed fire opened it — coverfire.ts, he is chewing it for the line: it is not his cover that went)
    for (const gd of g.guards) if (gd.state !== 'dead' && !own(gd) && rectDist(gd.char.pos, row.rect) <= WALLREACT.nearM) ask(gd);
    for (const A of this.posts()) if (!own(A.gd) && rectDist(A.A.post!, row.rect) <= WALLREACT.nearM) ask(A.gd);   // (a man still walking to a mark beside it)
  }
  /** the things a question keeps behind its newest (newest first): at most goneMax - 1 of them — past that the one FURTHEST from where the man is asked (his
   *  feet; a teammate's mark) is let go (what hid him stood beside him), the oldest of two as far */
  private kept(at: Vec3, list: Gone[]): Gone[] {
    const n = WALLREACT.goneMax - 1; if (n <= 0) return [];
    while (list.length > n) { let k = 0, far = -1; list.forEach((x, i) => { const d = rectDist(at, x.rect); if (d >= far) { far = d; k = i; } }); list.splice(k, 1); }
    return list;
  }
  /** once a second: a man who has SINCE planted himself in front of a breach he knows of (the room fight's own moves, a search, a new mark — none of them
   *  know of holes) is asked exactly as the men who stood there when it opened were */
  private sweep() {
    const g = this.g, K = WALLREACT;
    const ask = (gd: Guard, at: Vec3, b: Breach) => {
      if (this.pend.some(P => P.gd === gd)) return;
      const d = Math.abs((at[0] - b.at[0]) * b.n[0] + (at[2] - b.at[2]) * b.n[1]), l = Math.abs((at[0] - b.at[0]) * b.u[0] + (at[2] - b.at[2]) * b.u[1]);
      if (d > K.nearM || l > b.half + 0.6) return;   // (in front of it, not merely near its wall)
      if (!this.beyond(gd, b, at) || !this.openThrough(b, at, gd.team === 'operative' ? 1.2 : 0.9)) return;   // (… and open through it to floor that is not his own side's: the next breach is looked at, not this one asked about for ever)
      const ax = b.at[0] - b.u[0] * b.half, az = b.at[2] - b.u[1] * b.half, bx = b.at[0] + b.u[0] * b.half, bz = b.at[2] + b.u[1] * b.half;
      this.pend.push({ gd, rect: [Math.min(ax, bx) - 0.1, Math.min(az, bz) - 0.1, Math.max(ax, bx) + 0.1, Math.max(az, bz) + 0.1], b, who: 0, until: g.time + 1.5, next: g.time });
    };
    for (const b of this.breaches) {
      for (const gd of b.men.keys()) { if (gd.state === 'dead') continue; const D = this.defOf(gd); if (D && D.mode === 'hold' && g.time - (D.movedAt ?? -1e9) > K.retrySecs) ask(gd, gd.char.pos, b); }
      if (b.ops >= 0) for (const A of this.posts()) if (A.A.arrived) ask(A.gd, A.A.post!, b);
    }
  }
  /** the breach a wall's row belongs to: a new one, or the one on that wall it widens */
  private upsert(row: LevelChange): Breach | null {
    const g = this.g, W = g.destr.walls.all().find(x => x.id === row.id); if (!W) return null;
    const bx = W.box, cs = Math.cos(bx.yaw ?? 0), sn = Math.sin(bx.yaw ?? 0), alongX = bx.h[0] >= bx.h[2];
    const u: [number, number] = alongX ? [cs, -sn] : [sn, cs], n: [number, number] = alongX ? [sn, cs] : [cs, -sn], c0: [number, number] = [bx.c[0], bx.c[2]], L = alongX ? bx.h[0] : bx.h[2];
    let lo = Infinity, hi = -Infinity;
    for (const [x, z] of [[row.rect[0], row.rect[1]], [row.rect[2], row.rect[1]], [row.rect[0], row.rect[3]], [row.rect[2], row.rect[3]]]) { const s = (x - c0[0]) * u[0] + (z - c0[1]) * u[1]; lo = Math.min(lo, s); hi = Math.max(hi, s); }
    lo = Math.max(-L, lo); hi = Math.min(L, hi);   // (the rect is the fresh columns' world bounds: its run along the wall, clipped to the wall)
    if (!(hi > lo)) return null;
    let b = this.breaches.find(x => x.wall === row.id && lo <= x.hi + 0.3 && hi >= x.lo - 0.3) ?? null;
    const walk = row.kind === 'hole' && row.walkable;   // (floor freed by a shelf falling off a wall that only lost its upper is no way through)
    if (b) { b.lo = Math.min(b.lo, lo); b.hi = Math.max(b.hi, hi); if (row.cause === 'blast') { b.cause = 'blast'; b.blastBy = row.who; }   // (a charge blown into a gap rounds had already cut: it is a blown wall from here on — the line they say, and the blind man's stamp below)
      if (walk && !b.walk) { b.walk = true; b.ops = -1; b.men.clear(); b.said.clear(); b.tell.length = 0; } }   // (a port widened into a way through is news again: learned afresh, plans dealt before it are older than it)
    else {
      b = { key: `breach:${row.id}:${row.serial}`, wall: row.id, c0, u, n, lo, hi, at: [0, CHEST, 0], half: 0, walk, t: g.time, cause: row.cause, who: row.who, ops: -1, men: new Map(), said: new Set(), tell: [], opening: null };
      this.breaches.push(b); WALLREACT_STATS.breaches++;
    }
    // (two breaches of one wall grown into each other: one)
    for (let i = this.breaches.length - 1; i >= 0; i--) { const o = this.breaches[i]; if (o === b || o.wall !== b.wall || o.lo > b.hi + 0.3 || o.hi < b.lo - 0.3) continue;
      b.lo = Math.min(b.lo, o.lo); b.hi = Math.max(b.hi, o.hi); b.walk ||= o.walk; if (o.ops >= 0 && b.ops < 0) b.ops = o.ops; for (const [m, t] of o.men) if (!b.men.has(m)) b.men.set(m, t);
      for (const M of this.minds.values()) { const e = M.get(o); if (e) { M.delete(o); const k = M.get(b); if (k) k.ev = Math.max(k.ev, e.ev); else M.set(b, e); } }
      for (const [m, w] of this.watching) if (w === o) this.watching.set(m, b);
      o.into = b; for (const P of this.pend) if (P.also) for (const X of P.also) if (X.b === o) X.b = b;   // (everyGone: an older thing a question keeps is about the breach its own grew into; the newest thing's is left as it was asked)
      if (o.opening) this.dropOpening(o.opening); this.breaches.splice(i, 1); }
    const mid = (b.lo + b.hi) / 2; b.at[0] = c0[0] + u[0] * mid; b.at[2] = c0[1] + u[1] * mid; b.half = (b.hi - b.lo) / 2;
    if (b.opening) { b.opening.at[0] = b.at[0]; b.opening.at[2] = b.at[2]; b.opening.half = b.half; }
    note(g, `${row.kind} ${row.cause} wall #${row.id} → ${b.key} at (${b.at[0].toFixed(1)}, ${b.at[2].toFixed(1)}) ${(b.half * 2).toFixed(1)} m ${b.walk ? 'WALK' : 'port'}`);
    return b;
  }

  /** floor opened: the rooms it touches are flooded again — two rooms joined by a hole are ONE (the lower-numbered Room object lives on, the other becomes
   *  the same cells and entries), and each defender's evidence is carried over by door */
  private reflood(opened: readonly number[]) {
    const g = this.g, F = this.fight, nav = g.col.nav, W = nav.W, cs = nav.cell;
    const touched = new Set<Room>();
    for (const c of opened) for (const n of [c, c - 1, c + 1, c - W, c + W]) { const r = F.roomOfCell.get(n); if (r) touched.add(r); }
    if (!touched.size) return;
    const olds = [...touched].sort((a, b) => a.id - b.id), before = new Map(olds.map(r => [r, { cells: [...r.cells], entries: [...r.entries] }]));
    for (const r of olds) for (const c of r.cells) if (F.roomOfCell.get(c) === r) F.roomOfCell.delete(c);
    const now = new Map<Room, Room>();
    // (dig-in, the press and the last stand hold INDICES into a room's doors: a room's own doors keep their places, in the same Entry objects, and the doors
    //  it gains come after them — for the room that lives on and, in its own order, for the Room object it absorbed)
    const same = (a: Entry, b: Entry) => a.door === b.door && flat(a.at, b.at) < 0.3;
    const merged = (old: Entry[], fresh: Entry[]) => [...old.filter(o => fresh.some(f => same(f, o))), ...fresh.filter(f => !old.some(o => same(o, f)))];
    for (const r of olds) {
      if (now.has(r)) continue;
      const seed = before.get(r)!.cells.find(c => nav.blocked[c] !== 1) ?? -1;
      if (seed < 0) { now.set(r, r); continue; }
      const n0 = F.rooms.length, nr = F.roomAt([((seed % W) + 0.5) * cs, 0, (((seed / W) | 0) + 0.5) * cs]);
      if (!nr) { now.set(r, r); continue; }
      let into = nr;
      if (F.rooms.length > n0 && F.rooms[F.rooms.length - 1] === nr) {   // (a fresh flood: it lives on as the Room the men already hold — the same object, the same Set and list, so whoever else keeps a hand on it reads the new room)
        F.rooms.pop(); r.cells.clear(); for (const c of nr.cells) { r.cells.add(c); F.roomOfCell.set(c, r); } const E = merged(before.get(r)!.entries, nr.entries); r.entries.length = 0; r.entries.push(...E); into = r; }
      now.set(r, into);
      for (const o of olds) { if (now.has(o)) continue; const c0 = before.get(o)!.cells.find(c => nav.blocked[c] !== 1); if (c0 !== undefined && into.cells.has(c0)) { o.cells = into.cells; o.entries = merged(before.get(o)!.entries, into.entries); now.set(o, into); WALLREACT_STATS.merged++; } }
    }
    for (const D of F.defs) {
      if (!D.room || !touched.has(D.room)) continue;
      const was = before.get(D.room)!.entries, room = now.get(D.room) ?? D.room, E = room.entries;
      const idx = (i: number) => i >= 0 && was[i] ? E.findIndex(e => same(e, was[i])) : -1;
      const ev = E.map(e => { const j = was.findIndex(x => same(x, e)); return j >= 0 ? D.ev[j] ?? 0 : 0; });
      D.watch = idx(D.watch); D.want = idx(D.want); D.glanceTo = idx(D.glanceTo); D.ev = ev; D.room = room;
    }
    note(g, `rooms ${olds.map(r => r.id).join('+')} flooded again → ${[...new Set(now.values())].map(r => `${r.id} (${r.cells.size} cells, ${r.entries.length} doors)`).join(', ')}`);
  }

  // ================================================================ (b) noticing
  private head(gd: Guard | 'sam'): Vec3 { const p = gd === 'sam' ? this.g.player.char.pos : gd.char.pos; return [p[0], EYE, p[2]]; }
  private faceOf(b: Breach, from: Vec3, y = 1.3): Vec3 { const s = (from[0] - b.at[0]) * b.n[0] + (from[2] - b.at[2]) * b.n[1] >= 0 ? 1 : -1; return [b.at[0] + b.n[0] * s * 0.3, y, b.at[2] + b.n[1] * s * 0.3]; }
  private opsKnow(b: Breach, by: string) { if (b.ops >= 0) return; b.ops = this.g.time; WALLREACT_STATS.learnedOps++; note(this.g, `the team knows ${b.key} (${by})`); }
  private manKnows(b: Breach, gd: Guard, how: 'heard' | 'saw' | 'made' | 'told') {
    if (b.men.has(gd)) return; const g = this.g, K = WALLREACT; b.men.set(gd, g.time);
    let M = this.minds.get(gd); if (!M) this.minds.set(gd, M = new Map());
    M.set(b, { ev: how === 'told' ? K.evOpen * 0.5 : how === 'made' ? 0 : K.evOpen, line: false, lineT: -1, t: how === 'made' ? -1e9 : b.cause === 'blast' && g.seats.some(s => (s.view.wallCharge?.stats.blown ?? 0) > 0) ? Math.max(g.time, gd.dazzledUntil) : g.time });   // (a man flash-blind as a blast opened it — the wall charge's stun — has it fresh from when his eyes are back: the room fight asks him nothing until then. Only once a wall charge has gone: before one, the stamp is the moment itself, as it always was)
    if (how === 'told') { WALLREACT_STATS.told++; return; }
    WALLREACT_STATS.learnedMen++; note(g, `${gd.callsign} ${how} ${b.key}`);
    if (how === 'made') return;
    b.tell.push({ by: gd, at: g.time + K.tellSecs });
    if (K.say && !b.said.has(gd.team) && gd.state !== 'patrol') { b.said.add(gd.team);   // (a man the noise has not stirred does not call it)
      // (the dialogue pass, words.ts: 'they' did it only if they did — a hole his OWN side's rounds or grenade made is called as a hole, and a port nobody shot — a blast's, a collapse's — is not 'shooting')
      const by = b.cause === 'blast' ? b.blastBy ?? b.who : b.who;   // (a gap one side's rounds cut and the other side then blew is THEIR blown wall, not his own side's hole)
      const own = WORDS.on && (by !== undefined && isManId(by) ? !isHostile(gd.team, 'player') : g.guards.some(x => x.char.id === by && x.team === gd.team));
      g.say(gd, own ? (b.walk ? "the wall's down — watch it!" : 'hole in the wall — watch it!')
        : b.walk ? (b.cause === 'blast' ? "they've blown the wall!" : "the wall's down — they can come through!") : WORDS.on && b.cause !== 'fire' ? 'hole in the wall — watch it!' : "they're shooting through the wall!"); }
  }
  /** the wall going, as a noise: who hears it open knows where */
  private heard(b: Breach, row: LevelChange) {
    const g = this.g, K = WALLREACT, R = row.cause === 'blast' ? K.hearBlastM : row.cause === 'fire' ? K.hearFireM : K.hearFallM, src: Vec3 = [row.pos[0], 1.3, row.pos[2]];
    const hears = (head: Vec3) => flat(head, src) < R && g.col.sound.propagate(src, head).pathLen < R;
    const ours = row.who !== undefined && isManId(row.who);
    if (ours || menUp(g).some(pl => hears([pl.char.pos[0], EYE, pl.char.pos[2]]))) this.opsKnow(b, ours ? 'the player made it' : 'the player heard it');
    for (const gd of g.guards) {
      if (gd.state === 'dead') continue; const made = gd.char.id === row.who;
      if (!made && !hears(this.head(gd))) continue;
      if (gd.team === 'operative') this.opsKnow(b, `${gd.callsign} ${made ? 'made' : 'heard'} it`); else if (gd.team !== 'civilian') this.manKnows(b, gd, made ? 'made' : 'heard');
    }
  }

  // ================================================================ (b) the squad's book
  private dropOpening(o: Opening) { const L = this.g.cover?.openings; if (!L) return; const i = L.indexOf(o); if (i >= 0) L.splice(i, 1); }
  private dropOpenings() { for (const b of this.breaches) if (b.opening) { this.dropOpening(b.opening); b.opening = null; } }

  /** does the plan line p → q go through the breach */
  private through(b: Breach, p: Vec3, q: Vec3): boolean {
    const sp = (p[0] - b.at[0]) * b.n[0] + (p[2] - b.at[2]) * b.n[1], sq = (q[0] - b.at[0]) * b.n[0] + (q[2] - b.at[2]) * b.n[1]; if (sp * sq >= 0) return false;
    const t = sp / (sp - sq), x = p[0] + (q[0] - p[0]) * t, z = p[2] + (q[2] - p[2]) * t;
    return Math.abs((x - b.at[0]) * b.u[0] + (z - b.at[2]) * b.u[1]) <= b.half + 0.35;
  }
  /** roomfight.ts applyWatch: `pt` is where his gun and his fix are about to rest (a door of his room, or straight ahead) — a breach he knows, has a line to and
   *  has more against than that door, takes it */
  look(gd: Guard, pt: Vec3) {
    const M = this.minds.get(gd); if (!M || !M.size) return;
    const g = this.g, K = WALLREACT, D = this.defOf(gd); if (!D || D.mode !== 'hold') { this.watching.delete(gd); return; }
    if (g.time < D.glanceUntil) return;   // (his wandering eye is his own)
    const p = gd.char.pos, had = this.watching.get(gd) ?? null; let best: Breach | null = null, bs = (D.watch >= 0 ? D.ev[D.watch] ?? 0 : 0) + 0.05;
    for (const [b, m] of M) {
      if (flat(p, b.at) > K.watchM) continue;
      if (g.time - m.lineT > 0.5 || g.time < m.lineT) { m.lineT = g.time; m.line = !g.col.segmentBlocked([p[0], EYE, p[2]], this.faceOf(b, p)); }
      if (!m.line) continue;
      let mates = 0; for (const [o, w] of this.watching) if (o !== gd && w === b && o.state !== 'dead') mates++;
      const s = m.ev + (b.walk ? K.holeBase : K.portBase) - K.crowd * mates + (b === had ? K.sticky : 0) + (g.time - m.t < K.freshSecs && g.time >= m.t ? K.fresh : 0);   // (a wall that has just come open in front of him is the newest thing in the room, whatever the same bang did at a door)
      if (s > bs) { bs = s; best = b; }
    }
    if (!best) { this.watching.delete(gd); return; }
    if (best !== had) { this.watching.set(gd, best); WALLREACT_STATS.watches++; this.fight.log.push({ t: g.time, who: gd.callsign, what: `watch → the ${best.walk ? 'hole' : 'port'} at (${best.at[0].toFixed(1)}, ${best.at[2].toFixed(1)})` }); }
    pt[0] = best.at[0]; pt[2] = best.at[2];
  }

  // ================================================================ (a) cover gone
  private posts(): { gd: Guard; A: Assignment }[] {
    const g = this.g, out: { gd: Guard; A: Assignment }[] = []; if (!teamActive(g)) return out;
    for (const gd of teamMen(g)) { if (gd.state === 'dead') continue; const A = orderOf(g, gd); if (A && A.post) out.push({ gd, A }); }
    return out;
  }
  /** the floor just beyond a breach from where a man stands — null when it is no danger to him: no floor there, his own side's men on it, or (the team) a room
   *  their book has cleared */
  private beyond(gd: Guard, b: Breach, at: Vec3): Vec3 | null {
    const g = this.g, K = WALLREACT;
    const s = (at[0] - b.at[0]) * b.n[0] + (at[2] - b.at[2]) * b.n[1] >= 0 ? 1 : -1; let far: Vec3 | null = null;
    for (const d of [K.beyondM, K.beyondM * 0.7, K.beyondM * 0.45]) { const q: Vec3 = [b.at[0] - b.n[0] * s * d, 0, b.at[2] - b.n[1] * s * d]; if (!g.col.nav.isBlocked(q[0], q[2])) { far = q; break; } }   // (a narrow passage beyond it: nearer the hole)
    if (!far) return null;
    const F = far, over = (p: Vec3) => ((p[0] - b.at[0]) * b.n[0] + (p[2] - b.at[2]) * b.n[1]) * s < 0 && flat(p, F) < 2.5;   // (beyond the wall, by that floor)
    if (gd.team === 'operative') { if (g.cleared.roomAt(far) !== 0 && g.cleared.knownAt(far)) return null; if (g.orders.squad().some(m => m !== gd && m.state !== 'dead' && over(m.char.pos)) || (!g.player.down && over(g.player.char.pos))) return null; }   // (his own side's ground beyond it)
    else for (const m of g.guards) if (m !== gd && m.state !== 'dead' && m.team === gd.team && over(m.char.pos)) return null;
    return far;
  }
  /** is a chest at `h` over `p` open THROUGH the breach to the floor just beyond it (three points across its width) */
  private openThrough(b: Breach, p: Vec3, h: number): boolean {
    const g = this.g, d = (p[0] - b.at[0]) * b.n[0] + (p[2] - b.at[2]) * b.n[1]; if (Math.abs(d) > 8) return false; const s = d >= 0 ? 1 : -1;
    for (const k of [0, 0.8, -0.8]) { const q: Vec3 = [b.at[0] + b.u[0] * b.half * k - b.n[0] * s * 0.9, 1.15, b.at[2] + b.u[1] * b.half * k - b.n[1] * s * 0.9]; if (this.through(b, q, p) && !g.col.segmentBlocked(q, [p[0], h, p[2]])) return true; }
    return false;
  }


}

const states = new SideMap<Game, WallReact>('wallreact.states');
/** this game's (made on first asking: by then the destruction engine stands) */
export function wallReactOf(g: Game): WallReact { let s = states.get(g); if (!s) states.set(g, s = new WallReact(g)); return s; }
/** game.ts resetGuards */
export function wallReactReset(g: Game) { states.get(g)?.reset(); }
