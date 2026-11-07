// THE PARTNER SNEAKS — how a teammate FOLLOWS while the squad is silent and nobody has noticed it.
//
// The play note (10-08): "oh we also haven't tried anything with two person stealth yet, we should try this and polish". Measured on the build before this
// (tools/qa/pairsneak.ts, rebuild/pair-stealth/partner-sneaks.md): a silent teammate told to fall in walks UPRIGHT at the walk's pace whatever Sam does, closes
// up in bursts (his slowest pace is over Sam's crouch-walk), cuts straight to his place in the file — and stands or walks in a lit doorway a defender is looking
// at because Sam, low and quick, went through it a second earlier. The dark keeps him; the lit, watched stretches are where he gives the pair away.
//
// What he does differently with SNEAK.on, while he FOLLOWS (orders.ts 'fall in'), the squad is SILENT (silent.ts), not compromised, and no defender is hunting:
//   LOW WITH SAM   Sam crouched, he is crouched — and at Sam's own pace plus what closes the gap, with no floor under it (`minPace`): he creeps when Sam
//                  creeps instead of walking up and stopping. Low, his legs are asked for no more than `lowPace`. He is low whenever he waits or crosses, too.
//   IN SAM'S STEPS he walks Sam's own trail — every crumb of it, kept `trailM` long, not the last ten metres with a bee-line to his place: the line Sam chose
//                  through the dark is the line he takes, also when he is catching up from a wait. He closes to `back` metres further back than loud.
//   LOOKS BEFORE HE STEPS  the next `aheadM` of the trail is read a few times a second: how LIT each step is (the CPU's estimate of the direct light there, plus
//                  what Sam's own meter read there over that estimate as he walked it) and which defenders have a LINE to a crouched man on it (guards.ts's
//                  own eye: 16 m, head to chest or head) — and of those, who is LOOKING that way now (his cone, `coneDeg` wider: a glance travels). A step is
//                  EXPOSED when it is lit and has a line (or is inside `nearM` of a defender's eye, lit or not: a shape at his elbow is seen in the dark).
//   WAITS          an exposed stretch ahead that somebody is looking at: he stops short of it, low and still, where he is not exposed — and says so, once, over
//                  the net (`cue`: 'holding. it's lit and he's looking.'; after `nagSecs` 'can't cross while it's lit. he's still on it.' — the cue for Sam: put the light out,
//                  or go on without him). A stretch somebody IS looking at he never walks into; one too long to cross in a go (`crossM`) with a line
//                  on it but nobody looking, he gives `patience` seconds and then takes at the next moment nobody looks (0 = never).
//                  A door swinging shut between him and the man who was looking does not un-see it: he takes him to be looking still, for `memory` seconds.
//   CROSSES        the same stretch with every eye that has a line to it turned away: he goes, low, at `crossPace`, to the first step past it that is NOT
//                  exposed (past his place in the file if need be, never inside `standNear` of Sam) and does not stop in it. And LOOKED AT, when it is cheap:
//                  what the crossing would add to each looker's awareness by the eye's own arithmetic (light^1.5 × distance × the crouch), with a margin,
//                  inside `budget` — a step of dim light at range costs a watcher nothing; a man Sam has just stirred has none to spare.
//   STANDS IN THE DARK  Sam stopped: his place is the nearest step along the trail that is NOT exposed (lit with a line, whoever is looking) — on towards Sam
//                  or back the way they came, never the lit doorway behind Sam's shoulder; with no dark step short of Sam he holds in the dark behind and
//                  says so. No such step anywhere on the trail (it runs straight at an open door): he steps ASIDE, off the trail, out of the door's line (`side`). The cover's unasked step to see through a door (cover.ts) is not taken while he sneaks.
//   CLEAR OF A DOOR (`door`) Sam at a SHUT door, about to open it, and a defender beyond it: the lit steps in that man's line through the doorway (the frame read
//                  `doorPad` wider) are exposed as if it stood open — he waits short of them, or steps back / aside out of them, BEFORE the leaf moves, says
//                  so once, and is still when it opens — and until the leaf has been at rest `doorSettle` s (a leaf half way round hides a line for a moment only).
//   CAUGHT IN THE OPEN  seen where he stands (his OWN bones and his OWN meter, whatever the read of the trail said — a light came back on, a door swung open)
//                  and looked at, or stood still in it: to the same nearest dark step, at a creep — once, not hopping about (`hideGap`).
//   SENT TO A MARK (`post`) the rescue flow's cover, a move / cover of Sam's (never a hold): he walks there low while Sam is low, at no more than `lowPace`, and is low on it —
//                  a plain posted walk is upright at the brisk pace with the gun up, across whatever is lit on the way. Posture and pace only: he does not wait.
// What it does NOT do (other parts' seams): nothing once a defender is alert or searching or the squad is compromised (every rule returns null and he is the
// build's own follower — the compromise part's); no new order, no menu row (the orders part's); nothing for a man on a stack or a door tool's marks.
// HONEST LIMITS: the light AHEAD of him is the CPU's direct-light estimate (no bounce, and no moon unless sense.ts SENSE.moon: guards.ts lightEstimateAt) plus what Sam's meter — the
// renderer's, in the page — read over it where Sam walked (`ambient`); his own meter where he STANDS is the renderer's too, and 'caught in the open' reads
// that. Headless both meters ARE the estimate, so the ambient term is untested there. He knows a defender only by the line from
// that man to the step he is about to take (he looks before he steps — he is not told who is behind a wall, and a wall is why he is safe from him);
// a defender who walks INTO view while he is mid-stretch is not predicted.
// A SKILL KNOB is left open (the play note: a partner's sneaking skill "can vary", "only the player is truly amazing at it"): SNEAK.skill scales how far he
// looks ahead and how wide he reads a glance — 1 = this competent partner; the page lists the call.
//
// OFF (the default; `?sneak` / SNEAK=on / the panel's row turn it on): every function below returns on its first line and orders.ts's follower is the build's
// own, frame for frame (the run digests agree). State is per Game in a WeakMap: nothing on the Game, in the digest or in a checkpoint. No dice.
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { SideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import type { Game, Guard } from './game';
import { squadSilent, silentCompromised } from './silent';
import { pairUp } from './pairup';
import { SPOTTED, spottedMode } from './spotted';
import { lightEstimateAt } from './guards';
import './teams';
import './doze';
import './workthings';
import './kitload';   // (what the sneaking man's card makes of a crossing's price: his shape over his pace)
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

export const SNEAK = {
  /** THE switch: off = the follower exactly as he was */
  on: false,
  /** low when Sam is low (and whenever he waits or crosses) */
  crouch: true,
  /** his legs: the least he is asked for while following (m/s; the loud file's is the walk, 1.5) · the most while low · across an exposed stretch */
  minPace: 0.5, lowPace: 1.45, crossPace: 1.6,
  /** a crouched footfall's level while he sneaks (silent's walk is 0.2, Sam's crouch 0.125; 0 = silent's own) — "only the player is truly amazing" at it */
  step: 0.14,
  /** he walks Sam's own trail (off: his place is the build's, by its own route) · metres of it kept · metres further back than the loud file he closes to */
  trailM: 60, back: 0.3,
  /** he looks before he steps (off: he never waits) · how far up the trail he reads · this lit (0‥1 on the meter's curve) is a lit step */
  aheadM: 2.4, litAt: 0.1,
  /** LIT TO WHOM (the maintainer's partner would not follow him across an open yard at night: on a GPU the open night shadow reads 14 to 27 on the meter,
   *  over `litAt`, where the software renderer reads near 0 — so on the page every step with a line was "lit" and only lines decided). With `byEye` a step
   *  at or over `litAt` with a line from a defender is SEEN only where THAT defender's eye would gain on a man there: the guards' own arithmetic (eyeRate:
   *  detectRate × light^1.5 × the distance term × the crouch) over `seenRate`, which is the guards' own floor — a rate at or under 0.05 /s moves no
   *  awareness (guards.ts). Dim ground far from an eye is ground he may stand on and cross; dim ground near one is not; nothing is seen that was not seen
   *  before. Read so: the follower's step and standing place (read), the stalk's watched cells and its priced way (the walker's map), a door frame's funnel
   *  (an eye behind a frame that would not gain on a man at the spot is no eye on it). NOT read so: a remembered stretch with no eye on it now (the flat
   *  number stands), darkfoe.ts, seenwalk.ts, spotted.ts. His kit card's shape (kitload.ts loadSeen) is in the rate, as the guards have it. The page: on
   *  (`?byeye=0`; `?seenrate=<n>`); false here and on the frozen harness stand (`TPS_BYEYE=1`; a FROZEN_HOLDS row). false = the flat number, as it was */
  byEye: false, seenRate: 0.05,
  /** a step's light is the CPU's direct-light estimate PLUS what Sam's own meter read there over that estimate when he walked it (the moon, the bounce) */
  ambient: true,
  /** … and a man on a way of his OWN (sneakWalk: the stalk to a man's back — floor nobody has walked) reads every step of it by the estimate PLUS what his own
   *  meter reads over the estimate of where he stands: all of it on open ground when he stands on open ground (the moon the estimate has not got is on the
   *  yard ahead of him as it is on him), as a crumb carries Sam's where he or the step is under a roof. OFF here and on the frozen stand
   *  (`TPS_STALKAMBIENT=1`), ON in the page (`?stalkambient=0`). Off = the estimate alone, as it was. Honest limit: it is the light of where he STANDS — from
   *  under a roof's edge, or in a wall's moon shadow, the moonlit ground ahead still reads dark until he is in it; the stalk is read again every
   *  PAIR.stalkEvery, and he stops on the first read after his meter has risen. */
  stalkAmbient: false,
  /** a defender's cone is read this many degrees wider (a glance travels) · a step whose chest is inside this of an unwary defender's eye, in his near
   *  field, is exposed, lit or not (the eye's own point-blank rule is 1.8 m, eye to chest) */
  coneDeg: 25, nearM: 2.1,
  /** an exposed stretch longer than this is not crossed while anybody has a line to it (he waits for the light, not the glance) */
  crossM: 4.5,
  /** … for this long (s), then he goes at the next moment nobody looks (0 = for as long as it takes). A stretch somebody IS looking at he never walks into. */
  patience: 8,
  /** a crossing that has not made its far side in this long is let go of (s) */
  crossSecs: 8,
  /** a looked-at stretch is crossed all the same when the most stirred looker's awareness after it — his now + `margin` × what the eye's own arithmetic
   *  adds over the crossing — stays within this (suspicion starts at 0.3) */
  budget: 0.12, margin: 1.6,
  /** a man he saw looking at a lit stretch, and can no longer see (a door swung to between them): taken to be looking still for this long (s) */
  memory: 12,
  /** his words over the net when he waits (and after `nagSecs` of waiting, and when he comes on) — at most one every `cueGap` seconds */
  nagSecs: 7, cueGap: 12,
  /** Sam stopped: he stands in the nearest unexposed spot along the trail (back within `standM` of his place, or on towards Sam but never inside
   *  `standNear` of him) rather than on a lit, seen one */
  standM: 4, standNear: 0.8,
  /** … and with no such step on the trail (a corridor that runs straight at an open door: every step Sam took is in the doorway's line), a spot OFF it,
   *  within `sideM` of where he is, that is not exposed: he steps aside out of the door's line — never into a door frame, never onto Sam or a mate.
   *  A lit spot a defender would have a line to THROUGH AN OPEN DOORWAY but for its leaf is no such spot: a leaf swings (the door's FUNNEL: the lines from
   *  his eye through the frame's whole opening) */
  sideM: 2.6,
  /** … and where he will STOP on his way to such a spot is out of sight too: he pulls up within a quarter metre of it, on the side he comes from, so a
   *  spot centimetres past the edge of a door's line left him standing in the line (the points `sideStop` and half of it short of the spot, towards
   *  him, are read as well). 0 = only the spot itself is read, as it was. */
  sideStop: 0.25,
  /** … and a SHUT door Sam is at (inside `doorM` of its frame: he is about to pick, ease or push it) is taken as open already: a lit place in the line
   *  of a defender beyond it is no place to be stood when the leaf moves — he finds his dark step, or steps aside, BEFORE it opens and is still when it does */
  door: true, doorM: 1.8, doorPad: 0.4, doorSettle: 1.0,   // (doorPad: for a step of the TRAIL the shut frame's opening is read that much wider at both jambs — the man beyond it may have moved by the time it opens; a spot off the trail has `aside`'s own margin)
  /** caught where he stands (seen by his own bones and meter, and looked at or stood still): to the nearest dark step — not again inside `hideGap` s */
  hideGap: 2.5,
  /** A FOLLOWER IS NOT LEFT STANDING ON THE WALK OF A MAN WHO COMES TO A SOUND. workthings.ts took him off the walk of the man sent to the breaker
   *  (`workComing`); the same hole stood for a man who heard something and walks to it over where the follower waits for his back. With this on, a
   *  suspicious man with no errand and no body to see to, whose route ends at the place he is going to look at and passes inside `noiseWalkM` of the
   *  follower with nothing solid between, is that man too (`sneakComing`): while Sam is going on the follower crosses after him instead of waiting short of
   *  what that man is looking at, does not keep him as a remembered looker, does not back off down the way he is coming, and is not stood still by
   *  spotted.ts's freeze. Off = the build as it was. */
  noiseWalk: false, noiseWalkM: 2.5,
  /** how often his read of the way is redone (game seconds) */
  planSecs: 0.12,
  /** the partner's sneaking skill, 1 = this one. Under 1 he reads less of the way ahead and a narrower glance (0 = he never waits). The page's call. */
  skill: 1,
};
export const SNEAK_DEFAULTS: typeof SNEAK = JSON.parse(JSON.stringify(SNEAK));
/** `?sneak` (`=0` off) sets the switch for the visit; the harness sets it by SNEAK=on */
try { const q = new URLSearchParams(location.search); if (q.has('sneak')) SNEAK.on = q.get('sneak') !== '0'; } catch { /* no page (the harness) */ }
try { const e = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.SNEAK; if (e) SNEAK.on = e !== '0' && e.toLowerCase() !== 'off'; } catch { /* no process (the page) */ }

export const SNEAK_STATS = { waits: 0, waitS: 0, crossings: 0, shade: 0, backOffs: 0, asides: 0, cues: 0, /** times a man coming to a sound was read as walking over a follower (SNEAK.noiseWalk) */ noiseWalks: 0 };
/** what he did, newest last (QA; the panel's summary) */
export const SNEAK_LOG: { t: number; who: string; what: string; at: [number, number] }[] = [];

/** his words (over the net while they are silent: silent.ts hush) — the CUE: the first says he has stopped and why, the second what would bring him on */
const LINES = {
  wait: { lit: "holding. it's lit and he's looking.", near: "holding. too close while he's looking.", long: 'holding. too much open light to cross.', open: "holding in the dark. you're stood in the light.", door: "i'm off the door. ready when you are." },
  nag: { lit: "can't cross while it's lit. he's still on it.", near: "he's still facing me. waiting for his back.", long: "it's a long lit stretch. kill the light or go on.", open: "you're stood in the light. i'll hold in the dark.", door: '' },
  go: 'moving.',
};
/** why he waits: a lit stretch somebody is looking at · a dark one at a looking defender's elbow · a lit one too long to cross with a line on it · no dark step short of Sam · the way on is in the line of a shut door Sam is about to open */
type Why = keyof typeof LINES.wait;
type Mode = 'walk' | 'wait' | 'cross' | 'stand' | 'back';
interface Crumb { p: Vec3;
  /** its light as last read (0‥1), when, and for which posture */ lit: number; t: number; low: boolean;
  /** what Sam's OWN meter read over the CPU's direct-light estimate of where he stood as he dropped it (0‥1) — the light the estimate cannot have: the moon
   *  (guards.ts lightEstimateAt skips kind 2), the bounce. Sam's meter is the renderer's on the page: one model for his plan and the eyes on him · and that
   *  estimate then: the extra is scaled down with the direct light there (a lamp put out takes its bounce with it) */
  amb: number; cpu0: number }
interface Man {
  /** the crumb he has reached (an index into State.trail + State.off) · when he last followed · when a look for the trail last found none he can walk to */ i: number; t: number; lostT: number;
  mode: Mode; planT: number;
  /** a wait: when it began, what for, and whether its second line has been said */ waitT: number; why: Why; nagged: boolean; cueT: number; /** when ONE VOICE last kept a line of his back */ voiceT: number;
  /** a back-off: when it began · when the last one ended · the crumb it ended on (he stays low till he has moved on from it; −1: none) */ backT: number; hideT: number; hidI: number;
  /** the lit stretch he last saw somebody looking at: who, which crumbs (absolute), when he last saw it so */ mem: { h: Guard; a: number; b: number; t: number } | null;
  /** a crossing: the crumbs it starts and ends on, when it was decided */ crossFrom: number; crossTo: number; crossT: number;
  /** the dark step he has taken or is backing off to (−1: none) */ stand: number;
  /** … or the spot off the trail he has stepped aside to (null: none) · when a look for one last found none */ side: Vec3 | null; sideT: number;
  /** the crumbs to this frame's goal, for sneakWay (null: none laid) */ way: Vec3[] | null;
}
/** when each door's leaf was last seen to move (read at the plan's rate, for the doors Sam is at) */
const leafT = new SideMap<object, { a: number; t: number }>('sneak.leafT');
interface State { trail: Crumb[]; off: number; t: number; men: WeakMap<Guard, Man>; list: Man[]; live: boolean; /** Sam's meter over the estimate, eased up slowly (a flash is not the light of a place) */ amb: number }
const states = new SideMap<Game, State>('sneak.states');
const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);

/** the sneak is live for the squad this frame: the switch, the silent order standing and unbroken, Sam on his feet, and no defender hunting (one read a frame) */
const liveAt = new SideMap<Game, { t: number; v: boolean }>('sneak.liveAt');
export function sneakLive(g: Game): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('sneakLive', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  if (!SNEAK.on) return false;
  let L = liveAt.get(g); if (L && L.t === g.time) return L.v; if (!L) liveAt.set(g, L = { t: 0, v: false });
  // (pairup.ts 'hunted', with spotted.ts on: a defender hunting somewhere is not the end of it — the partner sneaks on until the PAIR is caught, which is
  //  spotted.ts's call: an alert man with his eyes on one of them, or loud in their ears. Its mode is last frame's here — it ticks after the first ask.)
  L.t = g.time; L.v = squadSilent(g) && !silentCompromised(g) && !g.player.down && (pairUp('hunted') && SPOTTED.on
    ? !g.orders.squad().some(gd => gd.state !== 'dead' && spottedMode(g, gd) === 'caught')
    : !g.guards.some(h => (h.state === 'alert' || h.state === 'search') && h.team !== 'operative' && h.team !== 'civilian'));
  return L.v;
}
const mate = (gd: Guard) => gd.team === 'operative' && !!gd.unit;
/** this teammate is under the sneak this frame (orders.ts: no unasked cover step while he is) */
export function sneaking(g: Game, gd: Guard): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('sneaking', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } } return SNEAK.on && mate(gd) && sneakLive(g); }
/** he is low because Sam is (orders.ts: also while he steps out of Sam's way, which is the order's own move) */
export function sneakLow(g: Game, gd: Guard): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('sneakLow', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } } return SNEAK.on && SNEAK.crouch && g.player.crouch && sneaking(g, gd); }
/** is this man walking over where the teammate stands — the man sent to the breaker (workthings.ts workComing), or, under SNEAK.noiseWalk, any suspicious man
 *  on his way to a sound: no errand, no body to see to, not held to his post or in Sam's arm, none of them in his view this moment, still on the walk,
 *  his route's end the place he means to look at, and what is left of that route inside `noiseWalkM` of the teammate with nothing solid between (where the man stands, the corners still ahead, the place itself) */
export function sneakComing(g: Game, gd: Guard, h: Guard): boolean { return crossToRust('sneakComing', undefined, [g, gd, h]) as any; }
/** what he is doing, for a HUD's line (null: nothing to say — he is walking with Sam) */
export function sneakWord(g: Game, gd: Guard): string | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('sneakWord', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } }
  if (!SNEAK.on) return null; const M = states.get(g)?.men.get(gd); if (!M || !sneaking(g, gd)) return null;
  return M.mode === 'wait' ? (M.why === 'door' ? 'clear of the door' : M.why === 'lit' || M.why === 'near' ? 'waiting on eyes' : 'waiting on the light') : M.mode === 'cross' ? 'crossing' : M.mode === 'back' ? 'backing off' : null;
}

const cpuLit = (g: Game, a: Vec3, b: Vec3, own?: (o: number) => boolean, steady = true) => g.visibilityOf(Math.max(lightEstimateAt(g, a, own, steady), lightEstimateAt(g, b, own, steady)));


// ------------------------------------------------------------ the same eye, for a man who is NOT following (pairup.ts 'oneEye': pairorders.ts's stalk)
const cellLit = new SideMap<Game, Map<number, [number, number]>>('sneak.cellLit');
const keptLines = new SideMap<Guard, { x: number; z: number; t: number; low: boolean; map: Map<number, number> }>('sneak.keptLines');
export interface SneakWalk {
  /** a man on this floor would be LOOKED AT: lit (the sneak's bar) or at an elbow, with a line, inside a defender's cone as the sneak widens it — `wider` more
   *  degrees for a route's pricing (a way is planned round where a head may be in a second, not only where it is) */
  watched(x: number, z: number, wider?: boolean): boolean;
  /** what walking `pts` at `pace` would leave on the most stirred watcher: his awareness now + margin × Σ rate × time, as a crossing is priced (crossCost) —
   *  0: nobody would see a step of it (a step counts as the follower counts one: lit by the sneak's bar, or at an elbow). Steps within `skipM` of `skip` are not asked (the man's own back). Compare with SNEAK.budget. */
  cost(pts: Vec3[], pace: number, skip?: Vec3, skipM?: number): number;
  /** the watcher `cost` found worst (null: none) */
  by: Guard | null;
}
/** The sneak's eye over any walk by `gd`: every living defender hostile to him and not dazzled, each as guards.ts builds his eye (and, where `habit` gives
 *  the way he MOSTLY looks, a second time looking that way — the worse of the two counts, never both). */
export function sneakWalk(g: Game, gd: Guard, low: boolean, habit?: (h: Guard) => [number, number] | null): SneakWalk { return goneWithTick('sneakWalk') as any; }

// ------------------------------------------------------------ the follower's seam (orders.ts tick)
/** Sam's trail, a crumb every 0.4 m (once a frame, whoever asks first) */
function track(g: Game, S: State) {
  if (S.t === g.time) return; const dt = g.time - S.t; S.t = g.time;
  const c = g.player.char, p = c.pos, T = S.trail, last = T[T.length - 1];
  if (dt < 0 || dt > 1 || (last && flat(last.p, p) > 3)) { S.off += T.length; T.length = 0; S.amb = 0; }   // (a load, a gap in the asking — nobody was sneaking —, a teleport: the old trail leads nowhere he walked)
  let cpu = 0;
  if (SNEAK.ambient) { cpu = cpuLit(g, c.bones.chest ?? [p[0], 1.2, p[2]], c.bones.head ?? [p[0], 1.6, p[2]], o => o === c.id, false);
    S.amb = Math.min(Math.max(0, g.player.visibility - cpu - 0.05), S.amb + 0.5 * Math.max(0, Math.min(dt, 0.1))); }   // (down at once, up at half a meter a second: a lightning stroke, a flicker, the meter's own lag are not the light of a place)
  else S.amb = 0;
  if (!T.length || flat(T[T.length - 1].p, p) >= 0.4) T.push({ p: [p[0], 0, p[2]], t: -1e9, lit: 0, low: true, amb: S.amb, cpu0: cpu });
  // kept `trailM` long — but never cut from under a man who is on it (a waiting man's crumb is his place: he waits as long as it takes), short of four times that
  const max = Math.ceil(SNEAK.trailM / 0.4);
  if (T.length > max) { let keep = T.length; S.list = S.list.filter(M => g.time - M.t < 5 && g.time >= M.t); for (const M of S.list) keep = Math.min(keep, M.i - S.off - 2);
    const cut = Math.max(T.length - 4 * max, Math.min(T.length - max, keep)); if (cut > 0) { T.splice(0, cut); S.off += cut; } }
}

export function sneakSummary(g: Game): string { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('sneakSummary', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  if (!SNEAK.on) return 'off';
  if (!sneakLive(g)) return 'on · not live (the squad is loud, found out, or a defender is hunting)';
  const S = states.get(g), men = g.orders.men().map(gd => `${gd.callsign}: ${S?.men.get(gd)?.mode ?? 'not following'}`);
  return `on · ${men.join(' · ') || 'no squad'} · trail ${(S ? S.trail.length * 0.4 : 0).toFixed(0)} m · waits ${SNEAK_STATS.waits} crossings ${SNEAK_STATS.crossings} shade ${SNEAK_STATS.shade} back-offs ${SNEAK_STATS.backOffs}`;
}
