// SPOTTED — what Sam's SILENT partner does once a defender is suspicious, and once the pair is caught.
//
// The play note: "we also haven't tried anything with two person stealth yet, we should try this and polish"; and on the squad's goggles, a teammate follows
// Sam's lead "or staying dark while not compromised". silent.ts already gives the calm half (he walks, stays dark, holds his fire on a man who has not
// noticed). This is the half after that — the three things a silent teammate does that he did not, each with its switch, ALL behind SPOTTED.on (off = every
// function here returns on its first line and the game is the game):
//
//   SUSPICIOUS  a defender in reach has said 'huh?' (the game's own state 'suspicious': awareness over 0.3, nobody shot at yet).
//     hold      no round at a man who is only suspicious — whatever his awareness. As it was, the hold ended at awareness 0.5, well short of the 0.85 at
//               which the man himself goes alert: the team's rifle (modelled loud, suppressor or no — weapons.ts) answered a man who had heard a lock click.
//               The gun still comes onto him and stays there ('eyes on. holding.'), so the round goes the frame he goes alert. A quiet gun is not held
//               (silent.ts quietGun: the loadout's Quiet set), a floor loud has already reached holds nobody, and a hostage's last warning holds nothing.
//     low       he gets down on a knee (the eye's own crouch term: three quarters of the rate) …
//     freeze    … and stands still where he is while the man has a line to him or is near enough to hear a boot (a still shape in the half-dark fills
//               the meter at a third of a walking one's rate — guards.ts motionCore), unless Sam has walked on past `leashM`: then he follows, low.
//     slip      LIT and in the man's line (the line asked of the walls and furniture only: a shut leaf between them is about to be eased open), he takes the
//               nearest spot a couple of strides off that is out of the line — or, failing one, in the dark — and gets low there. Not at once: a moving
//               shape is what draws an eye, so he holds still for `slipWait` after the man turned, and only then moves.
//     brace     (ITS KNOB SHIPS OFF: with the sneaking follower's own rules on, the partner is already low and out of a door's line before the man drops.)
//               BEFORE the noise: a man has just been dropped (Sam's quiet round, his hands — the partner is at his shoulder and sees it). The body's weapon
//               is on the floor a beat later and every mate in earshot turns to it; for `braceSecs` each defender within `braceM` of that body who is not
//               yet roused is his business as if he had already turned — low and STILL where he is, never a step — so he is not the shape that man's eyes land on.
//     close     AT ARM'S LENGTH: a roused man who has turned to HIM inside `takeM`, nothing between them, is taken by hand — the takedown any body in this
//               game can be given (out cold, quiet), `takeSecs` with his hands on the man inside `takeReach`. Nothing shields him while he closes: a man
//               already ALERT is only gone for from contact distance (further out it is the gun, below), and never with a quiet gun in his hands.
//   CAUGHT      a defender is ALERT with his eyes on Sam or on a teammate (a real sighting inside `freshSecs`), or loud has reached this man since the order.
//     keen      he shoots on a sighting, as any roused man does (roomfight.ts: the calm floor's 'shape made out over time' is waived — nothing else is:
//               the same sight test, the same reaction beat, the same aim-settle law, the same one round a man).
//     gun       his gun goes onto the man who has them (and, suspicious, onto the man whose line he is in: it is there the frame that man goes alert) —
//               as it was he kept covering the opening cover.ts dealt him, his back to the fight.
//     step      following, with no SHOT on the man who has them (a wall between, a desk he sees over and cannot shoot over, or Sam standing in his
//               line — the file as it walks: the room fight keeps a gun down while Sam is in its lane), he takes the nearest spot
//               a couple of strides off that HAS one, off Sam's own line of fire, and fights from it — instead of standing in file behind Sam.
//     unaware   (the fire discipline that goes with a loud rifle; `holdUnaware`) before anything loud has happened, an alert or searching man who has NOT
//               got eyes on either of them, is not turned their way and is not close is left to Sam's quiet gun: the rifle stays on him and goes the moment
//               he turns, sees, or closes.
//   THE CUES    each change is SAID, once, by the man himself over the net (he is silent: guards.ts say hushes it to the radio strip), and the squad line's
//               word on the HUD says it while it lasts: 'SILENT · low', 'SILENT · contact'. Back to calm he says so too.
//
// NEVER a buff: nobody takes more to drop, nobody's aim is touched, no defender's senses are changed. He is harder to see only by what any man can do — be
// still, get low, get out of the line or out of the light — and quicker to shoot only by not waiting to make a shape out once the shape is shooting.
//
// WHAT HE KNOWS: a defender's state within `waryM` of him (the man says 'huh?' aloud and squares up to the noise — that is the cue the player has too), and,
// for 'caught', the alert the man shouts ('CONTACT!'). He does not read awareness numbers, and a man in a far room is nothing to him.
//
// Scope: the squad under Sam's orders while it is SILENT (silent.ts underSilence). A loud squad, a show, the tour, a man a drill owns: untouched. Feet are only
// ever changed for a man who is FOLLOWING (fall in); a man posted by an order (hold, cover, move there, a stack) keeps his feet and only gets low.
// State is per Guard in a WeakMap: nothing on the Game, nothing in the digest, nothing in a checkpoint.
// Probes: tools/qa/spotted-probes.ts. Tallies: tools/qa/pair-sneak.ts with SPOTTED=1 in the environment.
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)

import { SideMap } from '../math/side';
import type { Vec3 } from '../math/vec';
import type { Game, Guard } from './game';
import './teams';
import { underSilence, squadSilent } from './silent';
import { lightEstimateAt } from './guards';
import { PAIRUP, keptOn } from './pairup';
import './combat';
import './sneak';
import './takedown';   // the partner's takedown as a move, and the man it lands on (takedown.ts): poses only
import { crossToRust, goneWithTick } from './crossing';

export const SPOTTED = {
  /** THE switch (off = the build as it was; ?spotted on the URL, SPOTTED=1 in the harness's environment, Tab ▸ Sandbox ▸ Spotted) */
  on: false,
  // ---- suspicious
  /** no round at a man who is only suspicious */
  hold: true,
  /** on a knee while a suspicious man has a line to him or is in earshot */
  low: true,
  /** … and still (following only), while Sam is within `leashM` */
  freeze: true, leashM: 7,
  /** lit (the meter at `litAt` or over) and in his line: the nearest spot within `slipM` out of the line, or dark */
  slip: true, slipM: 3.2, litAt: 0.3,
  /** … but a moving shape draws the eye: he holds still first, and moves off the line only once a man has been turned his way this long (s; 0 = at once) */
  slipWait: 1.5,
  /** how far off a suspicious man is his business (m, straight) · and inside which a boot is (a silent step carries about 3.6 m) */
  waryM: 14, earM: 5,
  /** he stays down this long after the last such man has settled or dropped (s) */
  lingerSecs: 1.5,
  /** (off as shipped) a man dropped: for `braceSecs` every unroused defender within `braceM` of the body is his business as if he had already turned to the noise */
  brace: false, braceSecs: 3, braceM: 7,
  /** a roused man who has turned to him inside `takeM` is taken by hand: `takeSecs` with his hands on him inside `takeReach` (m) */
  close: true, takeM: 2.2, takeReach: 1.05, takeSecs: 0.6,
  // ---- caught
  /** an alert man's sighting of one of them counts for this long (s) · within this of the teammate (m) · and 'caught' lasts this long past the last one (s) */
  freshSecs: 2.5, caughtM: 30, lastSecs: 6,
  /** he shoots on a sighting while caught */
  keen: true,
  /** his gun goes onto the man: the one who has them (caught), the one whose line he is in (suspicious) */
  gun: true,
  /** following with no line to the man who has them: the nearest spot within `stepM` that has one */
  step: true, stepM: 3.2,
  /** before anything loud: an alert / searching man with no eyes on them, not turned their way (`faceDeg`) and not within `closeM` is not fired on */
  holdUnaware: true, faceDeg: 35, closeM: 6,
  /** the lines and the HUD's word */
  cue: true,
};
export const SPOTTED_DEFAULTS = { ...SPOTTED };
export type SpottedMode = 'calm' | 'wary' | 'caught';
interface Man {
  mode: SpottedMode; since: number;
  /** the suspicious men that are his business this frame · the alert man who has them */
  wary: Guard[]; by: Guard | null;
  /** the nearest suspicious man with a line to him (his gun's) */
  eyes: Guard | null;
  waryT: number; caughtT: number;
  /** where his feet are sent while this lasts (one object for its length: the script runner keeps a path while the goal stands) · what it is */
  spot: Vec3 | null; what: 'freeze' | 'slip' | 'step' | null; askT: number;
  /** Sam went on past the leash: he follows until he is back at his shoulder */
  exposed: boolean; trail: boolean; saidT: number;
  /** a suspicious man is among `wary` (not only men braced for) · that has been said this spell */
  real: boolean; told: boolean;
  /** the man his hands are going onto: since when they are on him (−1: closing), when it began, and where his feet are sent (one object, moved with the man) */
  take: { foe: Guard; t0: number; born: number; spot: Vec3; asked?: boolean } | null;
  /** a man his hands did not reach in time is left alone until then · since when a man has really been turned (−1: none) · the last tick that read him */
  takeNo: { foe: Guard; until: number } | null; realT: number; tickT: number;
}
const men = new SideMap<Guard, Man>('spotted.men');
export const SPOTTED_STATS = { wary: 0, caught: 0, calm: 0, slips: 0, freezes: 0, steps: 0, held: 0, heldUnaware: 0, keen: 0, cues: 0, braces: 0, takes: 0, taken: 0 };
/** what changed, newest last (QA; the panel's summary) */
export const SPOTTED_LOG: { t: number; who: string; to: SpottedMode; what: string | null; of: string | null }[] = [];

const flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const NONE: never[] = [];
export { shotLine as spottedShotLine };
/** the meter's reading for a man knelt at `q` (the CPU's direct light — what a teammate's own meter falls back on) */
const litAt = (g: Game, q: ArrayLike<number>) => g.visibilityOf(lightEstimateAt(g, [q[0], 0.9, q[2]]));
const live = (g: Game, gd: Guard) => SPOTTED.on && underSilence(g, gd) && gd.state !== 'dead';

// ------------------------------------------------------------ what the rest of the game asks (each a return on its first line with the switch off)
/** his mode this frame (calm with the switch off, the squad loud, or nobody ever asked) */
export function spottedMode(g: Game, gd: Guard): SpottedMode { return live(g, gd) ? men.get(gd)?.mode ?? 'calm' : 'calm'; }
/** roomfight.ts's fire gate, beside silent.ts's own: this round is held */
export function spottedHoldsRound(g: Game, gd: Guard, h: Guard): boolean { return crossToRust('spottedHoldsRound', undefined, [g, gd, h]) as any; }
/** roomfight.ts's eyes: caught, he shoots on a sighting */
export function spottedKeen(g: Game, gd: Guard): boolean { return goneWithTick('spottedKeen') as any; }
/** orders.ts, as it poses him: where his feet go and how he carries himself while this lasts — null: as ordered. `follower`: he is on 'fall in';
 *  `aside`: he is stepping out of Sam's way this frame (that step is never taken from him). A man posted by an order only ever gets low. */
export function spottedFeet(g: Game, gd: Guard, follower: boolean, aside: boolean): { goal: Vec3 | null; speed: number | null; crouch: boolean; watch: Vec3 | null } | null { return goneWithTick('spottedFeet') as any; }

// ------------------------------------------------------------ the tick
function set(g: Game, gd: Guard, M: Man, to: SpottedMode, of: Guard | null) {
  if (M.mode === to) return;
  M.mode = to; M.since = g.time; M.spot = null; M.what = null; M.askT = -1; M.trail = false; M.eyes = null; M.told = false; M.realT = -1; SPOTTED_STATS[to]++;
  SPOTTED_LOG.push({ t: g.time, who: gd.callsign, to, what: null, of: of?.callsign ?? null }); if (SPOTTED_LOG.length > 200) SPOTTED_LOG.splice(0, 100);
}
/** candidate spots round `p`, nearest first: on the floor, a straight walk from where he stands, off every doorway and off Sam */
function* ring(g: Game, p: Vec3, maxM: number): Generator<Vec3> {
  const nav = g.col.nav, sam = g.player.char.pos;
  for (let r = 0.8; r <= maxM + 1e-6; r += 0.8) for (let k = 0; k < 12; k++) {
    const a = k * Math.PI / 6 + r * 0.7, q: Vec3 = [p[0] + Math.sin(a) * r, 0, p[2] + Math.cos(a) * r];
    if (nav.isBlocked(q[0], q[2]) || !nav.walkable(p, q) || flat(q, sam) < 0.9) continue;
    if (g.doors.list.some(d => flat(q, d.frameCentre) < 1.0)) continue;
    yield q;
  }
}
/** between Sam's gun and the man it may be on (a lane a hand wider than a body) */
function inSamsLine(g: Game, q: Vec3, h: Guard): boolean {
  const a = g.player.char.pos, b = h.char.pos, dx = b[0] - a[0], dz = b[2] - a[2], L = Math.hypot(dx, dz); if (L < 0.5) return false;
  const al = ((q[0] - a[0]) * dx + (q[2] - a[2]) * dz) / L, side = Math.abs((q[0] - a[0]) * dz - (q[2] - a[2]) * dx) / L;
  return al > 0 && al < L + 1 && side < 0.7;
}
/** a round from a man stood at `q` to a body at `hp` (chest or head height `y`), stopped a body's breadth short of it as guards.ts clearShot stops */
function shotLine(g: Game, q: ArrayLike<number>, hp: ArrayLike<number>, y: number): boolean {
  const dx = hp[0] - q[0], dz = hp[2] - q[2], L = Math.hypot(dx, dz); if (L < 0.5) return true; const k = (L - 0.3) / L;
  return !g.col.shotBlocked([q[0], 1.35, q[2]], [q[0] + dx * k, 1.35 + (y - 1.35) * k, q[2] + dz * k]);
}
/** CAUGHT: following with no SHOT on the man who has them (a wall between them, or a desk he can see over and not shoot over) → the nearest spot that has
 *  one, off Sam's own line of fire and never nearer the man than a few strides */
function cover(g: Game, gd: Guard, M: Man) {
  const K = SPOTTED, h = M.by, p = gd.char.pos; if (!K.step || !h || h.state === 'dead') { M.spot = null; M.what = null; return; }
  // a shot from q: nothing solid on the line, and nobody of ours standing in it (the room fight's own lane test — with Sam between him and the man his gun
  // stays down, so a line THROUGH Sam is no line)
  const hp = h.char.pos, chest: Vec3 = [hp[0], 1.2, hp[2]], masked = (q: ArrayLike<number>) => !!g.fight?.friendInLine([q[0], 1.4, q[2]], chest, gd);
  const shot = (q: ArrayLike<number>) => (shotLine(g, q, hp, 1.2) || shotLine(g, q, hp, 1.6)) && !masked(q);
  if (M.what === 'step' && M.spot && shot(M.spot) && !inSamsLine(g, M.spot, h)) return;   // (on his way to it, or on it)
  if (shot(p)) { M.spot = null; M.what = null; return; }
  let best: Vec3 | null = null, bs = Infinity;
  for (const q of ring(g, p, K.stepM)) { if (!shot(q) || inSamsLine(g, q, h) || flat(q, hp) < 2.5) continue;
    const cost = flat(p, q) + (litAt(g, q) >= K.litAt ? 1.5 : 0); if (cost < bs) { bs = cost; best = q; } }
  if (best) SPOTTED_STATS.steps++;
  M.spot = best; M.what = best ? 'step' : null;
}
/** ASKED FOR (an order layer whose stalk on `foe` has just been blown — the man roused with this teammate a pace or two from his back): go on and take him
 *  by hand, turned to him or not. The same hands, the same limits (inside `takeM`, an alert man only from contact distance, nothing solid between, not with
 *  the floor loud in his ears or a quiet gun in his hands). True: his hands are going on (the caller leaves his feet to spottedFeet). Nothing in this file
 *  calls it. */
export function spottedTake(g: Game, gd: Guard, foe: Guard): boolean { return crossToRust('spottedTake', undefined, [g, gd, foe]) as any; }
/** his hands are ON this man (closing does not count): one test for whoever asks what a man in a teammate's hands can see (pairorders.ts pairGripped, under
 *  pairup.ts's 'hands' rule — nothing in this file reads it, and alone the part is as it was) */
export function spottedGripped(g: Game, h: Guard): boolean { if (!SPOTTED.on) return false;
  // (bounded like the order's own grip: hands nobody is tending any more — he was shot with them on, the squad went loud under them — do not blind the man for ever)
  for (const gd of g.guards) { if (gd.state === 'dead' || !underSilence(g, gd)) continue; const T = men.get(gd)?.take; if (T && T.foe === h && T.t0 >= 0 && g.time >= T.t0 && g.time - T.t0 < SPOTTED.takeSecs + 0.5) return true; }
  return false; }
/** the men dropped in the last `braceSecs`: where each fell and when (per Game; the first tick only learns who is already down) */
const drops = new SideMap<Game, { down: WeakSet<Guard>; at: { pos: Vec3; t: number }[]; seen: boolean; t: number }>('spotted.drops');
export function spottedSummary(g: Game): string {
  if (!SPOTTED.on) return 'off';
  const sq = g.orders.squad().filter(gd => gd.state !== 'dead'); if (!sq.length) return 'on · no squad';
  if (!squadSilent(g)) return 'on · the squad is loud (it only acts on a silent squad)';
  return sq.map(gd => { const M = men.get(gd); return `${gd.callsign} ${M?.mode ?? 'calm'}${M?.what ? ' (' + M.what + ')' : ''}`; }).join(' · ');
}
/** what a man is doing (QA: the probes and the tally read it) */
export function spottedOf(gd: Guard): { mode: SpottedMode; what: string | null; exposed: boolean; spot: Vec3 | null; by: string | null; wary: string[] } | null {
  const M = men.get(gd); return M ? { mode: M.mode, what: M.what, exposed: M.exposed, spot: M.spot, by: M.by?.callsign ?? null, wary: M.wary.map(h => h.callsign) } : null;
}

// ------------------------------------------------------------ the switch: per machine, the URL, the harness's environment
const LS = 'twopointsix.spotted';
const KEPT = ['on', 'hold', 'low', 'freeze', 'slip', 'brace', 'close', 'keen', 'gun', 'step', 'holdUnaware', 'cue'] as const;   // (the panel's rows)
export function saveSpotted() { try { const o: Record<string, boolean | number> = {}; for (const k of KEPT) if (SPOTTED[k] !== SPOTTED_DEFAULTS[k] && !(k === 'on' && PAIRUP.on)) o[k] = SPOTTED[k];   /* (on by pairup.ts's switch is that switch's to keep, not this row's: what the store held stands) */ if (PAIRUP.on) { const was = keptOn(LS); if (was !== undefined && was !== SPOTTED_DEFAULTS.on) o.on = was; } if (Object.keys(o).length) globalThis.localStorage?.setItem(LS, JSON.stringify(o)); else globalThis.localStorage?.removeItem(LS); } catch { /* no store */ } }
try { const o = JSON.parse(globalThis.localStorage?.getItem(LS) ?? 'null'); if (o) for (const k of KEPT) if (typeof o[k] === typeof SPOTTED[k]) (SPOTTED as Record<string, unknown>)[k] = o[k]; } catch { /* no store, or not ours */ }
try { const q = new URLSearchParams(location.search); if (q.has('spotted')) SPOTTED.on = q.get('spotted') !== '0'; if (q.has('nospotted')) SPOTTED.on = false; } catch { /* no page (the harness) */ }
try { const e = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.SPOTTED; if (e) { SPOTTED.on = e !== '0' && e.toLowerCase() !== 'off';
  for (const w of e.toLowerCase().split(/[\s,+]+/)) { const m = w.match(/^(no)?([a-z]+)$/); const k = m && KEPT.find(x => x.toLowerCase() === m[2]); if (k && k !== 'on') (SPOTTED as Record<string, unknown>)[k] = !m![1];
    const n = w.match(/^([a-z]+)=([\d.]+)$/), nk = n && Object.keys(SPOTTED).find(x => x.toLowerCase() === n[1] && typeof (SPOTTED as Record<string, unknown>)[x] === 'number'); if (nk) (SPOTTED as Record<string, unknown>)[nk] = Number(n![2]); } } } catch { /* no process (the page) */ }
