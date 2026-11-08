// A LANE THAT COST THEM (the maintainer, 10-11: "enemies do seem to run into held corridors a bit too eagerly, especially when they've already taken fire. was
// pretty easy to just hold a hallway and pick them off").
//
// What made them walk in: every way a defender is SENT somewhere in a room fight (the stock rally, Hard's press and its flankers, a squad's errand, the fight
// AI's own chase) is the shortest walk, and the only memory of a place that cost them was tactics.ts's kill-zone DOORWAY — one door frame, 25 s, never asked by
// a man on a scripted leg. A hallway the team holds from one end is neither. So five men in turn took the same hall past the same bodies.
//
// A LANE = a stretch of floor one gun covers: the wedge from where the fire came FROM, along the line to where it LANDED, as far as that gun has a line of fire
// (the level's own geometry and the door leaves as they stand) — and, within `fieldM` of that gun, everything else it has a line of fire to: men who know
// where a gun is know what it looks at, so the second and third ways in to the same guns do not each cost a man. One is opened by what a side has SUFFERED there: a man of theirs dropped by a round, or rounds
// passing a living man (`roundK` each: two near misses are a man's worth). ONE MODEL: the lane is nothing to anybody until a fit man of that side KNOWS of
// it — the man the rounds passed, or a mate with a line to where it happened, or (later) a man who comes on the bodies — and has had `callSecs` to say so on
// their net. Cut down before he could: nobody knows, and the next man walks in as before.
//
// WHAT THEY DO INSTEAD, once it is known and for `secs` after it last cost them (longer for every man lost in it):
//   · COME ANOTHER WAY: its floor is `dear` in every walk a man of that side plans (guards.ts goTo — the fight AI's hunters and the scripted legs of a press,
//     a flank or a squad's errand alike), so a way round is taken wherever the level has one.
//   · HOLD BACK: a man whose way still lies down it stops at the last floor out of its line and holds there with his gun on its mouth (followPath's gate →
//     RoomFight.laneStop), and while he is held no press and no noise AT THOSE GUNS takes him off it (`laneBars`: they would send him at the same guns by
//     the same way; a fight somewhere else may have him). Nobody who knows of the lane walks over to a fight at its guns BY HIMSELF, held or not (`laneKnownAt`
//     in the stock rally — the one-at-a-time feed); a press may still take a man who is not held: together, by its own way. A man caught standing in it off his own post while rounds are still coming down it gets out of the line, to floor the guns do not see even
//     shifted a stride. A doorway drill (tactics.ts) is not stacked on a jamb the guns see through the opening (`laneCovers`), nor run through a
//     doorway whose far side is in their line — unless the charge that goes in first will burst on those guns (`laneDoor`).
//   · GO ROUND: with two or more held at its mouths, after a breath one (two of four) is sent at where the fire comes from by a way that never shows him to
//     that gun — if the level has one; the rest keep the mouths.
//   · HOLD IT AT TWO ANGLES, WITH FIRE (`pin`): a way round that would show its man to the guns too long is taken as far as where they would first see him; he
//     sets there, and the men held on the lane take turns putting a snap burst down it from their corners at where the fire came from (`flankTick`, `pinStep`).
//   · USE WHAT THEY CARRY: a man at a mouth with a stun charge on him (Hard's kit) and the guns in a throw's reach puts it down the lane, and the men on its
//     mouths go in behind the bang while it lasts — behind any stun charge OF THEIRS that bursts on those guns with nothing solid between, the doorway
//     drill's too (`bangs`: a side knows its own cans; the holders' own sends nobody). And a man held on a lane with a FRAGMENTATION GRENADE on him (`frag`,
//     10-13) throws it at the middle of where the lane's fire came from, his arm round the corner he is held at (`fragOut`, `fragStep`): his own throw —
//     frag.ts — wants a fresh fix of his own and a line his rifle has lost, and a man held short of a lane has neither. Nobody goes in behind that one.
//
// HOW LONG: `secs` after it last cost them — or `movedSecs` after every gun that made it has cost them from somewhere well away (`movedM`), or is down: the
// guns have moved. Its floor is read again against the doors and walls as they stand, a wedge each half second (a door shut across it, a wall blown open).
// WHAT THEY KNOW OF THE GUNS is where rounds that cost them left the muzzle (the shot events' own start points, as tactics.ts reads them) — exact, where a man
// would have a rough idea; nothing is known of a gun that has not cost them.
//
// What it is NOT: no man added, no hit point, no better aim, no dice of its own (the one roll is the round's own spread, drawn by fireWeapon for a burst from a corner), nothing an enemy could not have seen or been told. Only defenders in
// a room fight (team 'hostile' units while the fight is live); never the night game, a show, or a teammate (he is under orders). `LANES.on` off = nothing here
// runs = the build by run digest; the frozen harness stand holds it off (TPS_LANES=1).
import './seats';
import { SideMap, SideSet, GameSideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import type { Game, Guard, GameEvent } from './game';
import type { Item } from './throwables';
import { fightLive } from './roomfight';
import './guards';
import './squad';
import './combat';
import './glasspen';
import './teams';
import './pressure';
import './tactics';
import './frag';
import './dialogue';
import './cues';
import './doorway';
import './gunroles';   // (called inside laneBars only)
import { releaseMan, ownerWitness, OWN } from './owner';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

export const LANES = {
  /** the whole rule. Module default OFF (the frozen stand); the page raises it (src/host/profile.ts pageSwitches; `?nolanes` holds it off) */
  on: false,
  /** how long a lane is remembered after it last cost them — × the men lost in it, up to three */
  secs: 150,
  /** how far down its line a gun's lane runs, m · the half angle of the wedge about that line · floor this near the line is in it whatever the angle, m */
  reachM: 32, halfDeg: 22, widthM: 1.1,
  /** …and everything that gun has a line of fire to within this of it, whichever way: men who know where a gun is know what it looks at (0 = the line alone), m */
  fieldM: 12,
  /** floor this near the gun itself is not part of its lane (he is on top of it: that is a fight), m */
  ownM: 1,
  /** fire that landed nearer its gun than this opens no lane, m */
  minM: 2,
  /** a round passing within `nearM` of a living man is worth this much of a man dropped (1 = a lane) */
  roundK: 0.5, nearM: 1.5,
  /** the beat before a man who saw it acts on it himself · the beat he needs to get it said · how far a mate sees it happen, m · how near a man coming on the bodies knows what they mean, m */
  reactSecs: 0.35, callSecs: 0.9, seeM: 30, bodyM: 12,
  /** the price of a lane's floor in a walk (× plain floor) */
  dear: 14,
  /** how far along his way a walking man looks for a lane, m */
  aheadM: 3.5,
  /** an enemy he has seen this close inside the last second is a fight already: no gate, no backing out, m (never less than the distance the room fight itself
   *  breaks a walk off at for a man in sight) */
  closeM: 4.5,
  /** rounds down it inside this many seconds: a man standing in it off his post gets out of the line */
  hotSecs: 8,
  /** GO ROUND: on · how many of the men held at its mouths at most · the breath before anybody is sent · the longest way round, m · no sooner again, s */
  roundMen: 2, thinkSecs: 2.5, roundMaxM: 80, roundGap: 25,
  /** …and he may first show himself to the guns only this far round from the lane's own line, deg, and only for the last this much of his way, m */
  roundSideDeg: 55, roundShowM: 8,
  /** a place to stop is out of sight of the guns as they stood AND of a gun a stride to either side of each (they shift about), m */
  shiftM: 0.9,
  /** THE CHARGE: on · a throw's reach to the guns, m · a bang this near the guns has them blind, m · how long the lane is theirs to push down behind the bang, s · who goes (this near the guns), m · how much faster than a chase they go, m/s · no sooner again, s */
  stunM: 12, bangM: 5, pushSecs: 6, pushM: 12, pushFast: 0.8, stunGap: 20,
  /** THE GUNS HAVE MOVED: once every gun that made a lane has cost them from further than `movedM` from it (or is down), it is forgotten `movedSecs` later, m · s */
  movedM: 15, movedSecs: 20,
  /** A LOOK ROUND THE CORNER (what they know of a lane is only as good as the last time it cost them): on · a lane quiet this long gets one, s · no sooner
   *  again — doubled for each look that found them still there, up to `peekMax`, s · after a look that cost the man, s · only a man held this near the lane's
   *  mouth goes, m · one of them in sight within this of where the fire came from = still held, m · how long he looks down an empty lane before he calls it
   *  clear, s · a look that showed the spots but not all the floor round them clears it only the second time running and only once it has been quiet this
   *  long, s · a lane seen still held is remembered at least this much longer, s */
  peek: true, peekQuiet: 10, peekGap: 15, peekMax: 60, peekLost: 45, peekReachM: 5, peekNearM: 10, peekLook: 0.8, peekSure: 45, peekKeep: 60,
  /** …he comes up to the corner at this pace, m/s, and looks round it with his head this far out past his feet, m (so the guns never get his chest for it) */
  peekPace: 1.3, peekLean: 0.35,
  /** HELD AT TWO ANGLES, WITH FIRE (off = a way round must hide its man to its last `roundShowM`, and the men at a lane's mouth only keep their guns on it):
   *  on · a way that comes at the guns from another side but would show its man for longer than that (up to `pinShowM`) is taken as far as where the guns
   *  would first see him: he STOPS there and says so — one more man held on the lane, from its other side. While a man is set there, the men held on the lane, either side, take turns stepping
   *  to their corners — gun already on where the fire came from — to put `pinRounds` down it at that place, one every `pinEvery` s, from where the muzzle first clears the corner, and get back within `pinSnap` s of that first line (no longer in the guns' sight than a look is — a man who stayed out a second and a half to fire was lost on 5 of 14 bursts): a burst no
   *  sooner than `pinGap` s after the last, `pinMax` of them, then `pinRest` s of quiet · a round goes wide by this (the round's own spread number: 0.24 = up
   *  to 1.2 m off the place at 10 m on each axis, on top of where his bore sits inside `pinAimDeg` — fire AT A PLACE to keep heads down, far wider than any aimed round of his; it can still hit a man who stands on it) ·
   *  his gun must be within this of the place before a round leaves, deg · his pace out, m/s */
  pinRounds: 3, pinEvery: 0.12, pinSnap: 0.6, pinGap: 4, pinMax: 4, pinRest: 20, pinSpread: 0.24, pinAimDeg: 9, pinPace: 2,
  /** the longest end of a way round the guns may see, with this on, m · nobody fires from a corner nearer the guns than this, m */
  pinShowM: 12, pinMinM: 7,
  /** …AND GO IN UNDER IT (off: measured — over six holds on The block the run-in cost the defenders 7 more men and the team none): on the first round of a
   *  mate's burst the men set on the lane's other side run in at its guns */
  pinGo: false,
  /** THROUGH GLASS (off = every line of a lane asked at the sidearm's energy, as it was): a lane remembers the heaviest gun that made it, and its floor and
   *  'has one of their guns a line to this spot' are asked at that gun's energy — a rifle's lane goes through a pane a 9 mm's does not */
  glass: true,
  /** WHERE HE STOPS (off = planted where the gate caught him, as it was): never in a door's opening or sweep, never where a gun of the lane a stride to either
   *  side of where it fired from has him — he backs to cover first, and a held man since moved onto such floor backs out again; the point he watches is never
   *  nearer than `lookM`; a man whose own look cleared the lane is not planted on it afterwards */
  lookM: 1.5,
  /** ROUNDS HE NOTICED (off = any round passing within `nearM` on the plan counts, as it was): a round past a LIVING man counts only once his own senses
   *  have it, inside `reactSecs` — it rattled him, he heard gunfire, or he was roused already; a quiet round over a calm man's head tells him nothing */
  roused: true,
  /** A GRENADE DOWN THE LANE THAT PINS HIM (the maintainer, 10-13, asked whether a defender pinned by a hallway hold should throw a frag down the corridor
   *  that pins him: "yeah, that makes sense". Off = he never does: his own throw — frag.ts — wants a fresh fix of his own and a line his rifle has lost, and a
   *  man held short of a lane has a watch point at his feet and no line at all): on · the breath after a lane is known before anybody thinks of it, s · no
   *  sooner again on a lane, s · nobody steps to a corner on it for a look or a burst, and no stun charge goes down it, until this long after it has gone
   *  off, s · how far he may walk while he gets it out (the wind-up's second, at a look's pace), m. Only a man who CARRIES one (frag.ts deals them: FRAG.defenders, one man in three, three a floor) and is held on the lane by this rule, at one of
   *  the places its fire came from, with everything frag.ts asks of any throw of his (the reach, the floor's gap between grenades, none of his own in it, no
   *  hostage by it, the shout and the second it takes him — shot in it, the grenade stays on his belt). He throws from behind the corner he is held at, his
   *  arm round it; with no arc from there he steps toward where the lane opens, no further than the first place his arm has one and never onto the
   *  lane's own floor (his arm goes into the guns' line, he does not), and goes back. */
  frag: true, fragThink: 4, fragGap: 30, fragLook: 1.5, fragStepM: 0.9,
  /** lanes a side keeps at once */
  max: 6,
};
export const LANES_DEFAULTS: typeof LANES = JSON.parse(JSON.stringify(LANES));

interface Lane {
  id: number;
  /** where the fire comes from · where it landed (the first cost) · the unit line between, on the plan */
  from: Vec3; at: Vec3; dx: number; dz: number;
  t0: number; lastT: number; until: number;
  /** what it has cost: men's worth (a near miss counts `roundK`) · men dropped in it */
  weight: number; downs: number;
  /** their side knows of it (said on their net) since */
  known: boolean; knownAt: number; lookAt: number;
  /** the muzzles that have fired down it (≤ 4, a stride or more apart): what a place to stop must be out of sight of */
  guns: Vec3[];
  /** who of the side saw it cost them, and when: a fit one acts on it himself `reactSecs` later and has it said on the net `callSecs` later · who knows it himself */
  saw: Map<Guard, number>; knowers: Set<Guard>;
  /** its floor: nav cell indices (made when it has cost a man's worth) · the wedges it was made of */
  cells: Set<number> | null; wedges: Vec3[];
  /** its floor being read again against the doors and walls as they stand now (one wedge each half second, swapped in when the last is read) */
  re: { set: Set<number>; i: number } | null;
  /** whose guns made it (character ids) · which of them have since cost the side from somewhere else */
  who: Set<number>; moved: Set<number>;
  roundAt: number; stunAt: number; pushUntil: number; can: { who: number; t: number; land: Vec3 } | null;
  /** the next look round the corner is due · looks taken that found it still held (each doubles the wait) */
  peekAt: number; peekN: number; peekPart: number;
  /** fire from its angles: the next burst is due · bursts of this volley · the id of the man who fired last (−1 none) */
  pinAt: number; pinN: number; pinBy: number;
  /** no grenade down it before this (LANES.frag; 0 with that off) */
  fragAt: number;
  /** the heaviest gun that made it, as glasspen.ts's energy (0 = not known: asked at the sidearm's, as every line was) */
  e: number;
}
/** a man out for a look: the lane · on his way to its mouth, or back · since · where he was held, what he watched from there, where he looks from · when he first had a line down it */
/** a man sent round to a lane's other side: on his way to where the guns would first see him · set there, holding it from that side · going in (`pinGo` only) · legs re-issued */
interface Flank { lane: number; phase: 'walk' | 'set' | 'go'; at: Vec3; t: number; n: number }
interface Peek { lane: number; phase: 'out' | 'back'; t: number; home: Vec3; look: Vec3; to: Vec3; lineT: number; eye?: Vec3;
  /** out to FIRE down it from his corner: rounds left of his burst · since his muzzle first had a line · the next round no sooner than · he has seen them still on it · when it last had a line */
  fire?: { n: number; t: number; next: number; saw: boolean; lineT: number };
  /** out to put a GRENADE down it: the place it is meant for, once he has shouted (null = still looking for an arc) · places asked so far · the next no sooner than · he could step to where the lane opens · he is on his way there · no floor of his own left toward it · a place had an arc but one of his own in its reach · when it should be out of his hand · how many had left his hand before */
  frag?: { at: Vec3 | null; k: number; next: number; walk: boolean; going: boolean; edge: boolean; own: boolean; until: number; had: number } }
interface State {
  lanes: Lane[]; serial: number;
  /** the floor of every lane the SIDE knows that is not being pushed down, 1 per nav cell (null = none) · bumped whenever any man's floor changes · a man's own
   *  floor where he knows a lane the net does not yet */
  mask: Uint8Array | null; ver: number; own: WeakMap<Guard, { ver: number; m: Uint8Array }>;
  seen: WeakSet<GameEvent>; banged: WeakSet<GameEvent>;
  /** per man: the lane he is held short of, a gate's last look, his post as first met, lines said */
  push: Map<Guard, number>; flank: Map<Guard, number>; flankOf: Map<Guard, Flank>; dropped: WeakSet<Guard>; failT: WeakMap<Guard, number>; skip: boolean; held: Map<Guard, { lane: number; t: number; look: Vec3; aimed: boolean; at: Vec3 }>; cans: Item[]; roll: number; due: { gd: Guard; lane: number; look: Vec3 }[]; look: WeakMap<Guard, number>; post: WeakMap<Guard, Vec3>; backT: WeakMap<Guard, number>; saidT: number; t: number; tickT: number;
  stats: { lanes: number; known: number; unknown: number; held: number; rerouted: number; backedOut: number; round: number; stuns: number; pushes: number; lapsed: number; peeks: number; peekThere: number; peekClear: number; peekLost: number; pins: number; pinRounds: number; pinLost: number; sets: number; goes: number; frags: number; fragLost: number; fragKept: number };
  /** the men out for a look round the corner (not kept across a load: he is planted where he stood, like the held) */
  peek: Map<Guard, Peek>;
  /** men whose magazine change a burst from a corner started: counted down here while they have nobody to shoot (not kept across a load) */
  reloads: Map<Guard, number>;   // (the man → his reload clock as this rule last left it: lanesTick)
  /** a man who looked for an arc for his grenade and found none: the lane, and the place it was meant for (LANES.frag; not kept across a load) */
  fragNo: WeakMap<Guard, { lane: number; at: Vec3 }>;
  /** rounds that passed a living man who had not noticed them yet (LANES.roused): counted if his senses have them inside `reactSecs`, else forgotten. Not saved */
  near: { gd: Guard; from: Vec3; at: Vec3; who: number; t: number }[];
}
const states = new GameSideMap<Game, State>('lanes.states');
ownerWitness<Game>('lane', (gd, g) => { const st = states.get(g); return !!st && (st.held.size > 0 || st.peek.size > 0) && live(g) && !!laneOf(st, gd); });
const fresh = (): State => ({ lanes: [], serial: 0, mask: null, ver: 0, own: new SideMap(), seen: new SideSet(), banged: new SideSet(), push: new Map(), flank: new Map(), flankOf: new Map(), dropped: new SideSet(), failT: new SideMap(), skip: false, held: new Map(), cans: [], roll: 0, due: [], look: new SideMap(), post: new SideMap(), backT: new SideMap(), saidT: -100, t: -1, tickT: 0,
  stats: { lanes: 0, known: 0, unknown: 0, held: 0, rerouted: 0, backedOut: 0, round: 0, stuns: 0, pushes: 0, lapsed: 0, peeks: 0, peekThere: 0, peekClear: 0, peekLost: 0, pins: 0, pinRounds: 0, pinLost: 0, sets: 0, goes: 0, frags: 0, fragLost: 0, fragKept: 0 }, peek: new Map(), reloads: new Map(), fragNo: new SideMap(), near: [] });
const S = (g: Game): State => { let st = states.get(g); if (!st) { st = fresh(); states.set(g, st); } return st; };

const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const SIDE = 'hostile' as const;
/** a man the rule is for: a defender of a live room fight, on his feet */
const mine = (g: Game, gd: Guard) => gd.team === SIDE && !!gd.unit && gd.state !== 'dead' && !gd.held;
const live = (g: Game) => LANES.on && fightLive(g) && g.aiEnabled && !g.touring;
/** WHO THE LANES HAVE (src/game/owner.ts): a man held short of a lane or out for a look round its corner, while that lane stands */
const laneOf = (st: State, gd: Guard) => { const h = st.held.get(gd) ?? st.peek.get(gd); return h && st.lanes.find(L => L.id === h.lane); };
/** every man this file had, let go of: the state goes with them */
function letGo(g: Game, st: State): void { for (const gd of st.held.keys()) releaseMan(gd, OWN.lane, g.time); for (const gd of st.peek.keys()) releaseMan(gd, OWN.lane, g.time); }
const cellAt = (g: Game, p: Vec3) => { const n = g.col.nav; const x = Math.floor(p[0] / n.cell), z = Math.floor(p[2] / n.cell); return x < 0 || z < 0 || x >= n.W || z >= n.H ? -1 : z * n.W + x; };
const inMask = (g: Game, m: Uint8Array | null, p: Vec3) => { if (!m) return false; const i = cellAt(g, p); return i >= 0 && m[i] === 1; };
const pushing = (g: Game, st: State, gd: Guard) => { const t = st.push.get(gd); if (t === undefined) return false; if (g.time < t && gd.state !== 'dead') return true; st.push.delete(gd); return false; };
/** sent round at the guns (GO ROUND): his way is still priced, but the gate and the back-out leave him to it — he is meant to come out on them */
/** the energy a lane's lines of fire are asked at: its heaviest gun's, or nothing (collision.ts then asks at the sidearm's) */
const laneE = (L: Lane): number | undefined => LANES.glass && L.e > 0 ? L.e : undefined;

/** the floor one gun at `from` covers along the line to `at`: nav cells in the wedge with a line of fire from the gun */
function wedge(g: Game, into: Set<number>, from: Vec3, at: Vec3, e?: number) {
  const L = LANES, n = g.col.nav, c = n.cell; let dx = at[0] - from[0], dz = at[2] - from[2]; const len = Math.hypot(dx, dz); if (len < 0.5) return; dx /= len; dz /= len;
  const reach = Math.max(L.reachM, len + 2), cosH = Math.cos(L.halfDeg * Math.PI / 180), field = L.fieldM;
  const x0 = Math.max(0, Math.floor((from[0] - reach) / c)), x1 = Math.min(n.W - 1, Math.floor((from[0] + reach) / c)), z0 = Math.max(0, Math.floor((from[2] - reach) / c)), z1 = Math.min(n.H - 1, Math.floor((from[2] + reach) / c));
  const muzzle: Vec3 = [from[0], 1.4, from[2]], p: Vec3 = [0, 1.0, 0];
  for (let z = z0; z <= z1; z++) for (let x = x0; x <= x1; x++) {
    const i = z * n.W + x; if (n.blocked[i] || into.has(i)) continue;
    const px = (x + 0.5) * c - from[0], pz = (z + 0.5) * c - from[2], d = Math.hypot(px, pz); if (d < L.ownM || d > reach) continue;
    if (d > field) { const along = px * dx + pz * dz; if (along <= 0) continue;
      if (along / d < cosH && Math.abs(px * dz - pz * dx) > L.widthM) continue; }
    p[0] = px + from[0]; p[2] = pz + from[2];
    if (!g.col.shotBlocked(muzzle, p, undefined, e)) into.add(i);
  }
}
function remask(g: Game, st: State) {
  const n = g.col.nav; let m: Uint8Array | null = null;
  for (const L of st.lanes) { if (!L.known || !L.cells) continue; m ??= new Uint8Array(n.W * n.H); for (const i of L.cells) m[i] = 1; }
  st.mask = m; st.ver++;
}
/** the floor of every lane THIS man knows of (the net's, and what he has seen for himself and not yet had said) */
function maskOf(g: Game, st: State, gd: Guard): Uint8Array | null {
  let own = false; for (const L of st.lanes) if (!L.known && L.cells && L.knowers.has(gd)) { own = true; break; }
  if (!own) return st.mask;
  const c = st.own.get(gd); if (c && c.ver === st.ver) return c.m;
  const m = st.mask ? st.mask.slice() : new Uint8Array(g.col.nav.W * g.col.nav.H);
  for (const L of st.lanes) if (!L.known && L.cells && L.knowers.has(gd)) for (const i of L.cells) m[i] = 1;
  st.own.set(gd, { ver: st.ver, m }); return m;
}







function priceOf(g: Game, m: Uint8Array | null, base: ((ix: number, iz: number) => number) | undefined): ((ix: number, iz: number) => number) | undefined {
  if (!m) return base; const W = g.col.nav.W, d = LANES.dear;
  return base ? (ix, iz) => Math.max(base(ix, iz), m[iz * W + ix] ? d : 1) : (ix, iz) => m[iz * W + ix] ? d : 1;
}
/** guards.ts goTo: the price of a walk for this man — `base` (the pursuit tactics', or none) with the floor of every lane his side knows made dear */
export function lanePenalty(g: Game, gd: Guard, base: ((ix: number, iz: number) => number) | undefined): ((ix: number, iz: number) => number) | undefined { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('lanePenalty', undefined, [g, gd, base]); if (__r !== RUST_PASS) return __r as any; } }
  if (!LANES.on || gd.script?.direct) return base;   // (…a walk a level's script sent the short way, GuardScript.direct: a staged advance down a road the other side has guns on)
  const st = states.get(g); if (!st || !st.lanes.length || !live(g) || !mine(g, gd) || pushing(g, st, gd)) return base;
  return priceOf(g, maskOf(g, st, gd), base);
}
/** roomfight.ts rally: the noise he would walk over to BY HIMSELF is at the guns of a lane he knows is covered, or down it — nobody goes at those alone, held or
 *  not (the one-at-a-time feed). A press may still take a man who is not held: together, by its own way, a charge first. */
export function laneKnownAt(g: Game, gd: Guard, to: Vec3 | null): boolean { return goneWithTick('laneKnownAt') as any; }
/** tactics.ts tryEngage: this spot is in the line of guns his side knows of — no place to stack on a doorway (the jamb the holders see through the opening) */
export function laneCovers(g: Game, gd: Guard, p: Vec3): boolean { return crossToRust('laneCovers', undefined, [g, gd, p]) as any; }

/** a magazine change this rule starts for a man behind his corner and counts down itself (lanesTick: one count a frame, whoever else counts it).
 *  The caller's to ask that he is not already changing one */
export function laneReload(g: Game, gd: Guard): void { return crossToRust('laneReload', undefined, [g, gd]) as any; }
export function lanesReset(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('lanesReset', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } } const st = states.get(g); if (st) { letGo(g, st); states.set(g, Object.assign(fresh(), { stats: st.stats })); } }
export function lanesStats(g: Game): Record<string, number> { return { ...(states.get(g)?.stats ?? fresh().stats), live: states.get(g)?.lanes.filter(L => L.known).length ?? 0 }; }
export function lanesSummary(g: Game): string {
  const st = states.get(g); if (!LANES.on) return 'off'; if (!st) return 'none yet';
  const k = st.lanes.filter(L => L.known).length, s = st.stats;
  return `${k} known of ${st.lanes.length} · held short ${s.held} · round ${s.rerouted + s.round} · backed out ${s.backedOut} · charges ${s.stuns} · looks ${s.peeks} (still held ${s.peekThere}, clear ${s.peekClear}) · set on its other side ${s.sets}, bursts from the corners ${s.pins}${s.frags ? ` · grenades down it ${s.frags}` : ''}`;
}
export function lanesKnows(g: Game, p: Vec3): boolean { const st = states.get(g); return !!st && inMask(g, st.mask, p); }

// ---------------------------------------------------------------- checkpoints (a room fight refuses one today — a unit on the floor — so this rides only a forced file)
export interface LanesSave { serial: number; lanes: { id: number; from: number[]; at: number[]; t0: number; lastT: number; until: number; weight: number; downs: number; known: boolean; knownAt: number; guns: number[][]; wedges: number[][]; roundAt: number; stunAt: number; who?: number[]; peekAt?: number; peekN?: number; peekPart?: number; fragAt?: number; e?: number }[] }
export function lanesSnapshot(g: Game): LanesSave | undefined {
  const st = states.get(g); if (!st || !st.lanes.length) return undefined;
  return { serial: st.serial, lanes: st.lanes.map(L => ({ id: L.id, from: [...L.from], at: [...L.at], t0: L.t0, lastT: L.lastT, until: L.until, weight: L.weight, downs: L.downs, known: L.known, knownAt: L.knownAt, guns: L.guns.map(w => [...w]), wedges: L.wedges.map(w => [...w]), roundAt: L.roundAt, stunAt: L.stunAt, who: [...L.who], peekAt: L.peekAt, peekN: L.peekN, peekPart: L.peekPart, fragAt: L.fragAt || undefined, e: L.e || undefined })) };
}
export function lanesRestore(g: Game, s: LanesSave | undefined) {
  { const was = states.get(g); if (was) letGo(g, was); } states.delete(g); const st = S(g); st.skip = true; st.t = g.time;   // (the rounds and bangs a load puts back among the events were read before the file was made: not twice)
  if (!s || !s.lanes?.length) return;
  st.serial = s.serial;
  for (const r of s.lanes) {
    const from = r.from as Vec3, at = r.at as Vec3, len = flat(from, at) || 1;
    const L: Lane = { id: r.id, from, at, dx: (at[0] - from[0]) / len, dz: (at[2] - from[2]) / len, t0: r.t0, lastT: r.lastT, until: r.until, weight: r.weight, downs: r.downs, known: r.known, knownAt: r.knownAt, lookAt: 0, guns: (r.guns ?? [r.from]).map(w => w as Vec3), saw: new Map(), knowers: new Set(), cells: null, wedges: r.wedges.map(w => w as Vec3), re: null, who: new Set(r.who ?? []), moved: new Set(), roundAt: r.roundAt, stunAt: r.stunAt, pushUntil: -1, can: null, peekAt: r.peekAt ?? 0, peekN: r.peekN ?? 0, peekPart: r.peekPart ?? 0, pinAt: 0, pinN: 0, pinBy: -1, fragAt: r.fragAt ?? 0, e: r.e ?? 0 };
    if (L.weight >= 1) { L.cells = new Set(); for (let i = 0; i + 1 < L.wedges.length; i += 2) wedge(g, L.cells, L.wedges[i], L.wedges[i + 1], laneE(L)); }
    st.lanes.push(L);
  }
  remask(g, st);   // (the men held at its mouths are not kept: each is planted where he stood — the room fight's own holder — and the gate stops him again if he is sent)
}
