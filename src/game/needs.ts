// Needs-driven patrol variation, TELEGRAPHED (STATUS E12 (c) — the maintainer: "i actually really like the needs driven patrol variation, especially if it's
// telegraphed like fear!"). Every man on the night staff carries two or three slow needs — a coffee at the break-room counter, a smoke outside the fire exit,
// water at the cooler, a stretch round the lobby — dealt him for the night off his callsign (which needs, how fast each fills, how far along it starts: the
// same man every run, so a probe and a player see the same Kowalski). A need's level rises only while he is CALM on his route and stands still under everything
// else (a noise he is looking into, a body, an errand, the tour, a floor that is up); when one is full AND his route has brought him to the waiting waypoint
// nearest the place AND nobody else is off his route (the floor is short-staffed: one man away at a time) AND the floor is calm — he goes. But not before he
// has TOLD you, twenty seconds out, the way fear tells you: he looks at his watch and glances the way he will go (T−20 s), says so to the colleague within
// earshot — who grumbles back — or, with nobody near, over the net so the radio strip carries it (T−12 s), rubs his neck and turns to face the way out (T−5 s),
// and walks: at his patrol pace, torch as it hangs on patrol, through the doors on the way, to the spot; does the thing for a fixed spell (25 s at the counter,
// 40 outside, 12 at the cooler; the stretch is the lap itself); walks back to the waypoint he left, says 'right, where was I', and the route resumes where it
// stopped. The whole absence is therefore a constant per man and kind (the probes print them) — a window the player can COUNT on, never luck.
// ONE MODEL: nothing here dulls him. His state stays 'patrol' throughout, so his eyes, ears, the body he might walk onto, the kicked lock he might pass, the
// whistle he might answer are exactly the patrolling man's; and anything that reaches him — awareness to the 'huh?' line, a called body, an errand handed him,
// the floor going up, the tour taking the floor, the response team — CANCELS the trip on the frame, wherever he stands (a man startled halfway to the fire door
// looks into it from there, off his route: interesting, and fair). It is not resumed: the need is put back to half and he returns to his route as the ordinary
// AI has him. `Game.tune.needs` is the switch (default on; off = the patrol byte for byte as it was — tools/qa needs-identity holds it); NEEDS the numbers, Tab ▸
// Sandbox. guards.ts calls needsPatrol from the patrol branch (his feet while a trip owns them) and game.ts needsTick once a frame (levels,
// the deal, the interrupts); character.ts has the two tells (CharacterAnimator.tell); dialogue.ts the lines (NEEDS_LINES); chronicle.ts the 'needBreak' rows.
import { allDown } from './seats';
import { SideMap } from '../math/side';
import { Vec3, clamp } from '../math/vec';
import type { Game, Guard } from './game';
import { escalationOf } from './guards';
import { hashStr } from './dialogue';
import { isDaylight } from './lighting';
import './chronicle';
import './doze';
import { claimMan, releaseMan, ownerWitness, OWN } from './owner';
import { RUST_PASS, RUST_SURFACE, crossToRust } from './crossing';

// ================================================================ the kinds and their places (TODO(LevelData): NEED_SPOTS belongs in the level file beside routes / chokepoints — like qrf.ts
// QRF_MARKS it is checked headlessly against levels/office.json: free nav cells, reachable from every route, the loop's legs walkable, out of every leaf's sweep)
export type NeedKind = 'coffee' | 'smoke' | 'water' | 'stretch';
export const NEED_KINDS: NeedKind[] = ['coffee', 'smoke', 'water', 'stretch'];
/** Where a need is answered: stand at `pos` facing `face` (Character yaw) for the kind's dwell — or, for the stretch, walk `loop` round from `pos` without
 *  stopping and come back. `name` words the panel; the timeline has its own (chronicle.ts NEED_WORDS). */
export interface NeedSpot { kind: NeedKind; name: string; pos: Vec3; face: number; loop?: Vec3[]; }
export const NEED_SPOTS: Record<NeedKind, NeedSpot> = {
  coffee: { kind: 'coffee', name: 'the break-room counter', pos: [34.6, 0, 14.4], face: Math.PI / 2 },             // on the mat at the counter, facing the machine (the counter's face is x 35.15; the coffee machine stands at z 14.3)
  smoke: { kind: 'smoke', name: 'outside the fire exit', pos: [37.6, 0, 22.4], face: 1.1 },                       // out through the fire exit (its leaf sprung shut behind him) onto the east strip, a stride south of the door's sweep, facing out over the lot
  water: { kind: 'water', name: 'the water cooler', pos: [26.9, 0, 22.8], face: Math.PI / 4 },                    // the cooler in the cubicle farm's south-east corner (27.5, 23.4): a step short of it, facing it
  stretch: { kind: 'stretch', name: 'the lobby', pos: [8.4, 0, 17.0], face: 0, loop: [[10.8, 0, 18.6], [9.6, 0, 21.4], [7.4, 0, 22.8], [5.4, 0, 18.4], [8.4, 0, 17.0]] },   // in under the arch to the open floor before reception, once round the seating, back
};

/** WHERE A NEED IS ANSWERED ON THIS LEVEL: a level that carries its own places (its file's scenarios.quiet.needs — scene/scenarios.ts QuietData; any level with a
 *  quiet mission says which errands its floor has, a kind it leaves out is not dealt there) reads them; every other level reads the office's table above, as
 *  it always has. null: no such place on this floor. */
export function needSpot(g: Game, kind: NeedKind): NeedSpot | null {
  const Q = g.level.scenarios?.quiet; if (!Q) return NEED_SPOTS[kind] ?? null;
  const s = Q.needs?.[kind]; return s ? ownSpot(s, kind) : null;
}
const owned = new SideMap<object, NeedSpot>('needs.owned');
function ownSpot(s: { name: string; pos: Vec3; face: number; loop?: Vec3[] }, kind: NeedKind): NeedSpot { let o = owned.get(s); if (!o) owned.set(s, o = { kind, name: s.name, pos: s.pos, face: s.face, ...(s.loop ? { loop: s.loop } : {}) }); return o; }

// ================================================================ numbers 
export const NEEDS = {
  /** × every man's fill rate (1 = as dealt: a coffee fills in 6–9 min of calm patrol, a smoke in 8–12, water in 6–10, the stretch in 5–7) */
  rateK: 1,
  /** men off their routes at once, floor-wide (telegraphing counts as off: the slot is his from the watch check on) */
  maxAway: 1,
  /** the telegraph, seconds before he walks: the whole lead (the watch check + glance open it), when he says it, when he rubs his neck and turns to go */
  announceLead: 20, sayLead: 12, stretchLead: 5,
  /** a colleague his raised voice reaches is told to his face and answers `replyAfter` s later: this many metres of open air along the SOUND'S PATH (collision.ts
   *  SoundPaths — × its carry: 0.72 of it round a corner and in at the door, 0.32 through a shut leaf; the whistle's model); nobody that near and it goes over the net */
  partnerM: 14, replyAfter: 1.5,
  /** the dwell per kind (s): standing at the counter / outside / at the cooler; the stretch's is the lap itself (0 = no standing about) */
  dwell: { coffee: 25, smoke: 40, water: 12, stretch: 0 } as Record<NeedKind, number>,
  /** a trip only ever starts from a waypoint whose authored wait is at least this (a place he stands at anyway: legible), and — fromNearest — only from the one of
   *  those nearest the spot by path (so the absence per man and kind is one number); false = any waiting waypoint, the moment the need is full */
  minWait: 2, fromNearest: true,
  /** a kind whose place is nearer than this (m, by path) to the waypoint he would leave from is not dealt him at all: his route covers it already (Novak waits beside the cooler) */
  minTripM: 4,
  /** what a need is put back to when a trip is broken off, and what every need of his is capped at once he has been through an alarm (adrenaline: nobody wants a
   *  coffee straight after a manhunt) */
  afterCancel: 0.5, afterAlert: 0.3,
  /** seconds after coming back before HE may go again on any need, and the least gap between any two departures floor-wide */
  restSecs: 90, floorGapSecs: 30,
  /** the walk there and back: × the patrol pace (1 = exactly the route's walk); seconds of no headway on a leg before the trip is given up where he stands */
  paceK: 1, stuckSecs: 6,
};
/** per-kind fill windows (minutes of calm patrol from empty to full, dealt per man inside the range) and how far along a fresh man may start (0.3‥0.7 of full: the
 *  first man is over his line two to four minutes into a quiet run, and goes when his route next brings him to that need's waypoint) */
const FILL_MIN: Record<NeedKind, [number, number]> = { coffee: [6, 9], smoke: [8, 12], water: [6, 10], stretch: [5, 7] };
const LEVEL0 = [0.3, 0.7];
/** levels keep filling past full up to this (which of two full needs is the more pressing), and the calm a man must have to fill / to schedule (under the 0.3 'huh?') */
const LEVEL_CAP = 1.5, CALM_AWARENESS = 0.3;

// ================================================================ per-man memory (dies with the Guard object: resetGuards / restartEncounter make fresh men with fresh needs)
export interface Need { kind: NeedKind; level: number; fillMin: number; wp: number; tripM: number; }
export type TripPhase = 'telegraph' | 'going' | 'dwell' | 'returning';
export interface Trip {
  kind: NeedKind; spot: NeedSpot; phase: TripPhase; forced: boolean;
  /** game times: scheduled (the watch check), due to walk, walked, reached the spot, due to start back, back on the waypoint (−1 = not yet) */
  t0: number; departAt: number; leftAt: number; arrivedAt: number; dwellUntil: number; backAt: number;
  /** the waypoint he leaves from and comes back to (route.points[wp]), and the yaw of the way out (the first leg of the path: what he glances along and turns to) */
  wp: number; outYaw: number;
  /** the telegraph's beats done: watch check / said it (and to whom, or over the net) / the partner's answer due & done / neck rub */
  watched: boolean; told: boolean; partner: Guard | null; radio: boolean; replyAt: number; replied: boolean; necked: boolean;
  /** the stretch: index of the loop point being walked to (−1 = not on the loop) */
  loopI: number;
  /** the line he used (QA) */
  line: string;
  /** the floor's last-departure stamp before this trip took it (given back if the telegraph is broken off: nobody walked) */
  gapBefore: number;
}
export interface NeedMem { needs: Need[]; trip: Trip | null; lastBackAt: number; trips: number; cancels: number; shaken: boolean; dealt: boolean; }
const memOf = new SideMap<Guard, NeedMem>('needs.memOf');
ownerWitness('needs', gd => !!memOf.get(gd)?.trip);
function mem(gd: Guard): NeedMem { let M = memOf.get(gd); if (!M) { M = { needs: [], trip: null, lastBackAt: -1e9, trips: 0, cancels: 0, shaken: false, dealt: false }; memOf.set(gd, M); } return M; }
/** read-only peek for the panel / the probes (null: the needs have never touched this man — the switch off, a unit, a corpse from before) */
export function needsMem(gd: Guard): Readonly<NeedMem> | null { return memOf.get(gd) ?? null; }
/** the trip this man is on (telegraphing, away, coming back), or null */
export function needsTrip(gd: Guard): Readonly<Trip> | null { return memOf.get(gd)?.trip ?? null; }
/** is he busy with a need — telegraphing one or off his route on one? (dialogue.ts chatty: no small talk with a man who has just told you he is off) */

/** per-Game: when anybody last walked off (the floor-wide gap) */
const floorOf = new SideMap<object, { lastDepartAt: number }>('needs.floorOf');
function floor(g: Game) { let F = floorOf.get(g); if (!F) { F = { lastDepartAt: -1e9 }; floorOf.set(g, F); } return F; }

/** module-wide counters for the QA soak, and the last few things that happened (probes / the panel) */
export const NEEDS_STATS = { dealt: 0, scheduled: 0, forced: 0, departed: 0, arrived: 0, completed: 0, cancelled: 0, radioed: 0, told: 0, replies: 0, stuck: 0 };
export const NEEDS_LOG: { t: number; who: string; what: string }[] = [];
export const NEEDS_DEBUG: { log: ((s: string) => void) | null } = { log: null };
function log(g: Game, gd: Guard, what: string) { NEEDS_LOG.push({ t: g.time, who: gd.callsign, what }); if (NEEDS_LOG.length > 200) NEEDS_LOG.splice(0, 50); NEEDS_DEBUG.log?.(`${g.time.toFixed(2)}s ${gd.callsign}: ${what}`); }

// ================================================================ the deal: which needs a man has, off his callsign and his route
const frac = (s: string) => (hashStr(s) % 100000) / 100000;
function pathMetres(g: Game, from: Vec3, to: Vec3): number {
  const p = g.col.nav.findPath(from, to, 6000); if (!p) return Infinity;
  let s = 0, prev = from; for (const q of p) { s += Math.hypot(q[0] - prev[0], q[2] - prev[2]); prev = q; } return s;
}
/** the waypoint of his route a trip of this kind starts from: among those with an authored wait ≥ minWait, the nearest to the spot by path (fromNearest), with its
 *  distance; wp −1 = his route has no such waypoint / the spot cannot be reached from any */
function departurePoint(g: Game, gd: Guard, spot: NeedSpot): { wp: number; m: number } {
  const R = gd.route; let best = -1, bm = Infinity;
  for (let i = 0; i < R.points.length; i++) { if ((R.wait[i] ?? 0) < NEEDS.minWait) continue; const m = pathMetres(g, R.points[i], spot.pos); if (m < bm) { bm = m; best = i; } }
  return { wp: best, m: bm };
}
/** Deal this man his needs (once): every kind ranked by a hash of his callsign, kinds his route already covers (or cannot reach) dropped, the first two or three
 *  kept — each with its fill time inside the kind's window and a starting level, both off the callsign too. Staff only (a unit has no night to get through). */
function deal(g: Game, gd: Guard, M: NeedMem) {
  M.dealt = true;
  if (gd.team !== 'guard' || gd.unit || gd.routeI < 0) return;
  const cs = gd.callsign;
  const ranked = NEED_KINDS.map(k => ({ k, r: hashStr(`${cs}:${k}:rank`) })).sort((a, b) => a.r - b.r).map(x => x.k);
  const want = 2 + (hashStr(`${cs}:count`) % 2);
  for (const k of ranked) {
    if (M.needs.length >= want) break;
    const sp = needSpot(g, k); if (!sp) continue;   // (this floor has no such place)
    const dp = departurePoint(g, gd, sp);
    if (dp.wp < 0 || !Number.isFinite(dp.m) || dp.m < NEEDS.minTripM) continue;
    const [lo, hi] = FILL_MIN[k];
    M.needs.push({ kind: k, level: LEVEL0[0] + frac(`${cs}:${k}:level0`) * (LEVEL0[1] - LEVEL0[0]), fillMin: lo + frac(`${cs}:${k}:fill`) * (hi - lo), wp: dp.wp, tripM: dp.m });
  }
  NEEDS_STATS.dealt++;
  log(g, gd, `dealt ${M.needs.map(n => `${n.kind} (fills in ${n.fillMin.toFixed(1)} min, at ${n.level.toFixed(2)}, from wp${n.wp} ${n.tripM.toFixed(0)} m)`).join(', ') || 'nothing'}`);
}

// ================================================================ gates
/** the run is one the needs may play in at all: somebody home in their heads, not the tour / the attract loop / a puppet beat, not the breach's daylight, the
 *  response team not on its way or on the floor, Sam on his feet */
export function needsLive(g: Game): boolean {
  return g.tune.needs && g.aiEnabled && !g.quietUtility && !g.touring && !g.puppet && !isDaylight(g) && (!g.qrf || g.qrf.state === 'idle') && !allDown(g);
}
/** the floor is one a man would leave his route on: calm (level 0), no alarm episode running, the mains on */
function floorCalm(g: Game): boolean { return escalationOf(g) === 0 && !g.alarm.episode && !g.blackout.active; }
/** HE is calm and free: patrolling, nothing on his mind or in his hands — nobody's script, errand, body, lockdown role or arm */
function manCalm(g: Game, gd: Guard, awareness: number): boolean {
  return gd.state === 'patrol' && gd.awareness < awareness && !gd.held && !gd.script && !gd.task && !gd.bodyDuty && !gd.hold && !gd.pinned && !gd.leader && !gd.post && g.guards.includes(gd);
}
/** why a trip under way must be broken off NOW, or null (the same tests, named for the log / the timeline) */
function interruptOf(g: Game, gd: Guard): string | null {
  if (!g.tune.needs) return 'switched off';
  if (gd.state === 'dead') return 'down';
  if (!g.aiEnabled) return 'AI off'; if (g.quietUtility || g.touring || g.puppet) return 'the tour'; if (isDaylight(g)) return 'daylight'; if (g.qrf && g.qrf.state !== 'idle') return 'the response team'; if (g.player.down) return 'target down';
  if (escalationOf(g) > 0) return 'the floor went up'; if (g.alarm.episode) return 'an alarm'; if (g.blackout.active) return 'the lights went';
  if (gd.held) return 'grabbed'; if (gd.bodyDuty) return 'a body'; if (gd.task) return `an errand (${gd.task.kind})`; if (gd.script) return 'orders'; if (gd.hold || gd.pinned) return 'held by a script'; if (gd.leader || gd.post) return 'lockdown duty';
  if (gd.state !== 'patrol') return gd.state === 'alert' ? 'contact' : `went ${gd.state}`;
  if (gd.awareness >= CALM_AWARENESS) return 'heard something';
  if (!g.guards.includes(gd)) return 'reset';
  return null;
}


// ================================================================ a trip: scheduled → (telegraph) → going → dwell → returning → back
/** Put this man on a trip of `kind` from where he stands, the telegraph starting now (the watch check this frame). False if the place cannot be reached from here. */
function scheduleTrip(g: Game, gd: Guard, M: NeedMem, kind: NeedKind, forced: boolean): boolean {
  const spot = needSpot(g, kind); const c = gd.char;
  if (!spot) { const n = M.needs.find(x => x.kind === kind); if (n) n.level = 0; log(g, gd, `${kind}: this floor has no place for it — dropped`); return false; }
  const path = g.col.nav.findPath(c.pos, spot.pos, 6000);
  if (!path || !path.length) { const n = M.needs.find(x => x.kind === kind); if (n) n.level = 0; log(g, gd, `${kind}: no way to ${spot.name} from here — dropped`); return false; }
  // the way OUT: the bearing of the path's first stretch (a few metres along it, so a first node at his feet does not point him at the floor)
  let q = path[0]; for (const p of path) { q = p; if (Math.hypot(p[0] - c.pos[0], p[2] - c.pos[2]) > 2.5) break; }
  const outYaw = Math.atan2(q[0] - c.pos[0], q[2] - c.pos[2]);
  const now = g.time; const F = floor(g);
  claimMan(gd, OWN.needs, g.time); M.trip = { kind, spot, phase: 'telegraph', forced, t0: now, departAt: now + NEEDS.announceLead, leftAt: -1, arrivedAt: -1, dwellUntil: -1, backAt: -1, wp: gd.wp, outYaw,
    watched: false, told: false, partner: null, radio: false, replyAt: -1, replied: false, necked: false, loopI: -1, line: '', gapBefore: F.lastDepartAt };
  F.lastDepartAt = Math.max(F.lastDepartAt, now + NEEDS.announceLead);   // the floor's gap counts from when he WILL walk (so a second man let go by maxAway ≥ 2 still leaves floorGapSecs after him); given back if the telegraph comes to nothing
  NEEDS_STATS.scheduled++; if (forced) NEEDS_STATS.forced++;
  log(g, gd, `${forced ? 'SENT' : 'schedules'} ${kind} → ${spot.name} from wp${gd.wp} (${(pathLen(c.pos, path)).toFixed(0)} m); walks at +${NEEDS.announceLead} s`);
  return true;
}
const pathLen = (from: Vec3, p: Vec3[]) => { let s = 0, prev = from; for (const q of p) { s += Math.hypot(q[0] - prev[0], q[2] - prev[2]); prev = q; } return s; };


// ================================================================ the panel's button / the probes: send a man now
/** 'send Kowalski for a coffee now': the full telegraph from this frame wherever he stands (his slot or the floor's gap notwithstanding), then the trip. Returns
 *  a note for the panel: why not, if not. */
export function needsSend(g: Game, gd: Guard | null | undefined, kind: NeedKind = 'coffee'): string { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('needsSend', undefined, [g, gd, kind]); if (__r !== RUST_PASS) return __r as any; } }
  if (!gd) return 'no such man';
  if (!g.tune.needs) return 'needs are switched off';
  if (gd.state === 'dead') return `${gd.callsign} is down`;
  if (gd.team !== 'guard' || gd.unit) return `${gd.callsign} is not on the night staff`;
  if (!needsLive(g)) return 'not now (the tour / daylight / the response team / the AI off / the player down)';
  if (!floorCalm(g)) return 'not now — the floor is up (or in the dark)';
  const why = manCalm(g, gd, CALM_AWARENESS) ? null : interruptOf(g, gd) ?? 'busy';
  if (why) return `${gd.callsign} would not go now: ${why}`;
  const M = mem(gd); if (!M.dealt) deal(g, gd, M);
  if (M.trip) return `${gd.callsign} is already ${M.trip.phase === 'telegraph' ? 'about to go' : 'away'} (${M.trip.kind})`;
  if (!scheduleTrip(g, gd, M, kind, true)) return `${gd.callsign} cannot get to ${needSpot(g, kind)?.name ?? `a place for ${kind} (this floor has none)`} from where he stands`;
  return `${gd.callsign}: ${kind} — watch him (walks in ${NEEDS.announceLead} s)`;
}
/** the probes: put a man's need at a level (deals him first); false if he does not carry that kind */
export function needsSetLevel(g: Game, gd: Guard, kind: NeedKind, level: number): boolean { return crossToRust('needsSetLevel', undefined, [g, gd, kind, level]) as any; }

// ================================================================ checkpoints (game/savestate.ts): every staff man's needs + the trip under way, and the floor's gap clock
/** one man's slice, by roster index (null = the needs have never touched him: a corpse from before, the switch off) */
export interface NeedsManSnap { needs: Need[]; trip: (Omit<Trip, 'spot' | 'partner'> & { partner: number }) | null; lastBackAt: number; trips: number; cancels: number; shaken: boolean; dealt: boolean; }
export interface NeedsSnap { men: (NeedsManSnap | null)[]; lastDepartAt: number; }
export function needsSnapshot(g: Game): NeedsSnap {
  const men = g.guards.map((gd): NeedsManSnap | null => {
    const M = memOf.get(gd); if (!M) return null;
    const T = M.trip;
    return { needs: M.needs.map(n => ({ ...n })), lastBackAt: M.lastBackAt, trips: M.trips, cancels: M.cancels, shaken: M.shaken, dealt: M.dealt,
      trip: T ? { kind: T.kind, phase: T.phase, forced: T.forced, t0: T.t0, departAt: T.departAt, leftAt: T.leftAt, arrivedAt: T.arrivedAt, dwellUntil: T.dwellUntil, backAt: T.backAt, wp: T.wp, outYaw: T.outYaw,
        watched: T.watched, told: T.told, partner: T.partner ? g.guards.indexOf(T.partner) : -1, radio: T.radio, replyAt: T.replyAt, replied: T.replied, necked: T.necked, loopI: T.loopI, line: T.line, gapBefore: T.gapBefore } : null };
  });
  return { men, lastDepartAt: floorOf.get(g)?.lastDepartAt ?? -1e9 };
}
/** …and back onto the (fresh) roster: a trip comes back in the phase it was in — the man telegraphing goes on to walk at his departAt, the man at the counter
 *  finishes his coffee, the man walking there or back plans his path afresh next frame (goTo) from where he stands */
export function needsRestore(g: Game, s: NeedsSnap | null | undefined) {
  if (!s) return;
  floor(g).lastDepartAt = Number.isFinite(s.lastDepartAt) ? s.lastDepartAt : -1e9;
  s.men?.forEach((S, i) => {
    const gd = g.guards[i]; if (!gd || !S) return;
    const M = mem(gd);
    M.needs = (S.needs ?? []).filter(n => NEED_KINDS.includes(n.kind)).map(n => ({ kind: n.kind, level: clamp(Number(n.level) || 0, 0, LEVEL_CAP), fillMin: Math.max(0.1, Number(n.fillMin) || 1), wp: n.wp | 0, tripM: Number(n.tripM) || 0 }));
    M.lastBackAt = Number.isFinite(S.lastBackAt) ? S.lastBackAt : -1e9; M.trips = S.trips | 0; M.cancels = S.cancels | 0; M.shaken = !!S.shaken; M.dealt = !!S.dealt;
    const T = S.trip;
    M.trip = T && needSpot(g, T.kind) && gd.state !== 'dead' ? { kind: T.kind, spot: needSpot(g, T.kind)!, phase: T.phase, forced: !!T.forced, t0: T.t0, departAt: T.departAt, leftAt: T.leftAt, arrivedAt: T.arrivedAt, dwellUntil: T.dwellUntil, backAt: T.backAt, wp: T.wp | 0, outYaw: T.outYaw,
      watched: !!T.watched, told: !!T.told, partner: T.partner >= 0 ? g.guards[T.partner] ?? null : null, radio: !!T.radio, replyAt: T.replyAt, replied: !!T.replied, necked: !!T.necked, loopI: T.loopI | 0, line: T.line ?? '', gapBefore: T.gapBefore } : null;
    if (M.trip) claimMan(gd, OWN.needs, g.time); else releaseMan(gd, OWN.needs, g.time);
  });
}

// ================================================================ panel / QA
/** 'needs ▸ Kowalski coffee 0.82 · smoke 0.40 | Novak AWAY coffee (dwell, 12 s left) | Reyes stretch 1.00 — waits for wp6 · slot 1/1' */
export function needsSummary(g: Game): string {
  if (!g.tune.needs) return 'needs ▸ off';
  const parts: string[] = []; let away = 0;
  for (const gd of g.guards) {
    const M = memOf.get(gd); if (!M || gd.state === 'dead' || !M.needs.length) continue;
    const T = M.trip;
    if (T) { away++;
      const s = T.phase === 'telegraph' ? `about to go — ${T.kind} in ${Math.max(0, T.departAt - g.time).toFixed(0)} s${T.told ? (T.radio ? ' (radioed)' : ` (told ${T.partner?.callsign ?? '?'})`) : ''}` : T.phase === 'going' ? `walking to ${T.spot.name}` : T.phase === 'dwell' ? `at ${T.spot.name}, ${Math.max(0, T.dwellUntil - g.time).toFixed(0)} s left` : `coming back to wp${T.wp}`;
      parts.push(`${gd.callsign} AWAY ${s}${T.forced ? ' [sent]' : ''}`);
    } else {
      const full = M.needs.filter(n => n.level >= 1).sort((a, b) => b.level - a.level)[0];
      parts.push(`${gd.callsign} ${M.needs.map(n => `${n.kind} ${n.level.toFixed(2)}`).join(' ')}${full ? ` — waits for wp${full.wp}` : ''}${g.time - M.lastBackAt < NEEDS.restSecs ? ` (rests ${Math.ceil(NEEDS.restSecs - (g.time - M.lastBackAt))} s)` : ''}`);
    }
  }
  const gap = NEEDS.floorGapSecs - (g.time - floor(g).lastDepartAt);
  return `needs ▸ ${needsLive(g) && floorCalm(g) ? '' : 'FROZEN · '}${parts.join(' | ') || 'nobody dealt yet'} · slot ${away}/${NEEDS.maxAway}${gap > 0 ? ` · floor gap ${Math.ceil(gap)} s` : ''}`;
}
