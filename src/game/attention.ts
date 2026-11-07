// A GUARD HAS ONE ATTENTION, AND WHERE IT IS CAN BE SEEN (the maintainer, 10-13: "we can try to play with some mechanics where guards' attention could be
// redirected elsewhere (e.g. apollo robbins psychology stuff) but we should make sure it's legible and fun"). Robbins's picture of it is a spotlight with one
// man behind it: put on a thing, it is not on the rest. Until this file a small noise only made a man MORE dangerous — wary, turning, sweeping the room for as
// long as his suspicion took to drain, with nothing to say where he was looking or when he would stop. Now a small thing that makes a calm man look TAKES his
// attention to the place he puts it, for a time he shows:
//   · WHAT TAKES IT: anything his ears or eyes already answer by going to look or turning his head (guards.ts hearingCheck's small things — a whistle he
//     answers, a door, a knock or a clack, a carton shot across the room, a lamp dying in his view) — nothing is heard further than it was. Never his own
//     side's noise, never a footfall (that is YOU, and it breaks a hold: below), never anything loud (a slam, glass, a shot: those rouse him as they did).
//     ONE thing is made to work as it was written to: a knock MADE TO BE LOOKED AT (GameEvent.lure — the ring airfoil's clack off a wall, launcher.ts: 'they go
//     and look at IT') that reaches a calm man at all lifts him to `drawAw`, so he does go and look — by its own level a single clack left him under the
//     'huh?' line at every distance. The ring is this game's thrown bottle. A faint knock nobody made to be looked at (a chair a door swept) is no lure.
//     And a thing another file LENDS it to for as long as it says so (attentionLend: a man at a lit screen — the dark-rooms file's — is one more holder).
//   · WHERE IT GOES: the place he puts the thing — the source through open air, the doorway or corner it came round otherwise (hearingCheck's own `heardAt`).
//   · HOW LONG: while he walks over, and then `…Secs` of looking AT it (not round the room: the old five-second sweep is what made the wait unreadable). A thing
//     he only turns his head to holds it for the turn of the head.
//   · WHAT IT COSTS HIM: outside `focusDeg` of the line from his eye to the thing, what he sees of a man builds at `sideK` of the rate, writes no fix, and a
//     footfall reaches his ear at `earK`. INSIDE it he is as sharp as ever, and so is the point-blank floor (guards.ts sightGain, under 1.8 m): GUARD_EYE and
//     the close rule's dark numbers are not touched — the hold is geometry and time, never light.
//   · WHAT BREAKS IT: `breakGain` of awareness glimpsed past the edge of it ('—wait.' and he turns to YOU), a footfall that reaches him, anything loud, a body,
//     an errand, a grab, a fight. Broken, he is the wary man he always was, with his whole eye — and what he half-saw at the edge stays with him when the
//     look ends. (A man still at his own business under a hold — a lent one, a turned head — keeps no sum: it breaks when his awareness itself gets there.)
//   · HE IS NOT A FOOL (the maintainer: "we don't want them to feel dumb"): every taking inside `memorySecs` is shorter by `wearK` and he says so; the
//     `spent`-th is no look at all — he draws and searches the place (the whistle's own 'twice' rule, whistle.ts, stands beside this and is not changed).
//     Things inside `sameSecs` of each other are one happening (a canister's bounces, a thing dragged): one taking, which follows it.
//   · HOW HE SHOWS IT (attentionMarks → ui/attentionviz.ts; show, not tell — no word, no number): a thread on the floor from him to the place and a ring there
//     that runs down like a clock once he is at it; `tellSecs` before the end he SAYS he is done ('must have been nothing' moves here from the walk back), and
//     only then does the eye widen. Nothing is drawn of a man the fog of war hides.
// ONE switch, ATTENTION.on: ON in the page (?noattention = the build as it was), OFF here and on the frozen harness stand (TPS_ATTENTION=1; a FROZEN_HOLDS row).
// With it off every export returns on its first line. No dice but the line pools (simRandom), no camera, no graphics setting; per-man state in a WeakMap with a
// snapshot / restore pair (savestate.ts). Type-only game imports but for functions called inside functions (src/game is one import cycle).
import { SideMap } from '../math/side';
import { Vec3, v3, clamp } from '../math/vec';
import type { Game, Guard } from './game';
import './teams';
import './whistle';
import './guards';
import { crossToRust } from './crossing';

/** the switch and every number of it (UNTUNED by play: nobody has sneaked past a held man on a real page yet) */
export const ATTENTION = {
  on: false,
  /** the half-angle (deg, in plan) of what he is LOOKING AT, about the line from his eye to the thing; outside it a man's sight rate is × `sideK` and a footfall at his ear × `earK` */
  focusDeg: 25, sideK: 0.3, earK: 0.5,
  /** the awareness a man glimpsed OUTSIDE the focus has to build, in sum, for the hold to break and his head to come round */
  breakGain: 0.2,
  /** only a man under this awareness can have his attention taken (over it something has him properly worried: a slam, glass, a glimpse) */
  calmAw: 0.6,
  /** a knock made to be looked at (GameEvent.lure: a ring's clack) that reaches a calm man lifts his suspicion to this: over the 'huh?' line, so he goes to look at it */
  drawAw: 0.34,
  /** his suspicion is held at least this high while his attention is on a thing he is walking to or looking at (WHISTLE.walkFloor's number and reason) */
  floor: 0.32,
  /** seconds AT the thing, by what it was: a whistle he answered, a knock / clack / thud, a door, a light gone out in his view */
  whistleSecs: 4, noiseSecs: 5, doorSecs: 4, lightSecs: 5,
  /** how long before the end he says he is done (the tell: the eye is still narrow under it), how far his chest swings while he looks at the thing (rad; the
   *  old look-about swung 1.1), and the longest walk a hold lasts through (s) */
  tellSecs: 0.8, dwellSweep: 0.3, walkCap: 15,
  /** every taking inside `memorySecs` of the last is × `wearK` as long; the `spent`-th inside it is a search, not a look */
  wearK: 0.6, memorySecs: 60, spent: 3,
  /** a small thing inside this of the last one that reached him is the same happening (a canister's bounces, a carton's thud and its scrape, a thing dragged):
   *  it moves the place his attention is on and is no new taking; and how long a man a glimpse or a step took off a lent hold (a screen) stays off it */
  sameSecs: 2.5, lentBackSecs: 6,
  /** the floor marks (the thread and the ring), and the ring's radius (m) */
  marks: true, ringM: 0.42,
  /** the thread runs the WAY HE WILL WALK to the place (attentionWay: round the walls, through the doors), not straight through them; `wayReask` = the
   *  seconds an answer of the nav grid's is kept. Picture only (on on every stand; `?straightthread` in the page = the straight thread it was). */
  way: true, wayReask: 0.5,
};

export type DrawKind = 'whistle' | 'noise' | 'door' | 'light' | 'lent';
/** 'go' = he walks to it and looks at it; 'look' = he looks at it from where he stands (a posted man, a turned head); 'lent' = another file holds it there */
export type HoldMode = 'go' | 'look' | 'lent';
export interface Hold {
  at: Vec3; kind: DrawKind; mode: HoldMode;
  /** when it was taken; seconds AT the thing this taking is worth (worn); when it ends (-1: still walking to it — or lent: for as long as it is) */
  t0: number; secs: number; until: number;
  /** what a FRESH taking of this kind is worth (the ring's whole: a worn one starts part-spent) */
  full: number;
  /** awareness glimpsed outside the focus so far; has he said he is done; which taking inside the memory this is (1 = fresh) */
  glimpse: number; told: boolean; nth: number;
  /** × a footfall at his ear, × every other small noise at it (1 but for a lent hold whose lender says less: headphones) */
  ears: number; noises: number;
  /** the way he faced as he reached it (the focus's axis once the thing is under his feet), null before */
  yaw: number | null;
}
interface Mem {
  hold: Hold | null;
  /** a small thing that reached him this frame (hearingCheck), for the tick to weigh once his state has answered it */
  cand: { at: Vec3; kind: DrawKind; t: number } | null;
  /** another file's standing claim on it, refreshed every frame it stands */
  lent: { at: Vec3; t: number; ears: number; noises: number } | null;
  /** when each taking inside the memory began; the last frame the tick ran for him; when he last said he was done; the last line used from each pool */
  takes: number[]; tick: number; toldAt: number; line: number;
  /** when a small thing last reached him; until when a lent hold may not take him back; the frame guards.ts left his 'huh?' to this file (attentionAgain) */
  heardT: number; lentOff: number; owed: number;
  /** the happening he is hearing (things inside `sameSecs` of each other) has had its taking: it gets no second one however long it goes on */
  series: boolean;
}
const memOf = new SideMap<Guard, Mem>('attention.memOf');
function mem(gd: Guard): Mem { let M = memOf.get(gd); if (!M) { M = { hold: null, cand: null, lent: null, takes: [], tick: -1e9, toldAt: -1e9, line: -1, heardT: -1e9, lentOff: -1e9, owed: -1e9, series: false }; memOf.set(gd, M); } return M; }
/** a hold nobody has ticked for this long is nobody's (a man handed to a script, grabbed, dead: updateGuard returns before the tick) */
const STALE = 0.25;
/** module-wide counters and the last few endings, for the check and the panel */
export const ATTENTION_STATS = { taken: 0, looks: 0, lent: 0, released: 0, broken: 0, spent: 0, worn: 0 };
export const ATTENTION_LOG: { t: number; who: string; what: string; kind: DrawKind; secs: number; nth: number }[] = [];

// ---------------------------------------------------------------- what he says (placeholder lines — the writing pass is the maintainer's)
/** the second taking inside the memory (instead of 'huh?'), the one that is no look at all, the hold broken by a man at the edge of his eye, and — for a side
 *  with no 'must have been nothing' of its own — being done with it */
const WARY = ['again?', '…that\'s twice.'], WARY_FOE = ['again. stay sharp.', 'twice now.'];
const SPENT = ['no. somebody\'s doing that.', 'that\'s no accident. come out.'], SPENT_FOE = ['that\'s bait. search the place.', 'someone\'s playing us. sweep it.'];
const BROKE = ['—wait.', 'what—'];
const DONE_FOE = ['…nothing.', 'nothing there.'];
export function attentionLines(): string[] { return [...WARY, ...WARY_FOE, ...SPENT, ...SPENT_FOE, ...BROKE, ...DONE_FOE]; }

// ---------------------------------------------------------------- reading it
/** the hold on this man's attention right now, or null (always null with the switch off) */
export function attentionOf(g: Game, gd: Guard): Readonly<Hold> | null {
  if (!ATTENTION.on) return null;
  const M = memOf.get(gd); return M && M.hold && g.time - M.tick <= STALE ? M.hold : null;
}
/** the cosine of the angle at his eye between the thing that holds him and a man at `p` (the thing under his own feet: the way he faced it) */
function offCos(gd: Guard, H: Hold, p: Vec3): number {
  const c = gd.char, e = c.bones.head ?? c.pos;
  let ax = H.at[0] - e[0], az = H.at[2] - e[2]; const al = Math.hypot(ax, az);
  if (al < 1.0) { const y = H.yaw ?? c.bodyYaw; ax = Math.sin(y); az = Math.cos(y); } else { ax /= al; az /= al; }
  const px = p[0] - e[0], pz = p[2] - e[2], pl = Math.hypot(px, pz); if (pl < 1e-3) return 1;
  return (ax * px + az * pz) / pl;
}
/** is the floor point `p` outside what he is looking at: more than `focusDeg` off the line from him to the thing */
function outside(gd: Guard, H: Hold, p: Vec3): boolean { return offCos(gd, H, p) < Math.cos(ATTENTION.focusDeg * Math.PI / 180); }
/** degrees a man at `p` stands off the thing that holds this man's attention, at his eye (null: nothing holds it) — the checks' and the panel's reading of `focusDeg` */
export function attentionOffDeg(g: Game, gd: Guard, p: Vec3): number | null { const H = attentionOf(g, gd) as Hold | null; return H ? Math.acos(clamp(offCos(gd, H, p), -1, 1)) * 180 / Math.PI : null; }
/** guards.ts sightGain / sightGainOn: × the rate at which this man's eye takes in a man standing at `p` (1 for a man whose attention is his own, and inside the focus) */
export function attentionSideK(g: Game, gd: Guard, p: Vec3): number {
  const H = attentionOf(g, gd); return H && outside(gd, H as Hold, p) ? ATTENTION.sideK : 1;
}

/** Another file's standing claim on this man's attention (a lit screen he sits at): call it every update the claim stands, with the place on the floor — a claim
 *  not renewed is over at his next tick or the one after (there is no call to take one back: stop calling). A checkpoint carries it. `ears` /
 *  `noises` = × a footfall / × every other small noise at his ear while it stands (headphones: the lender's numbers). It holds only a calm man on his own
 *  business, shows as any hold does, and a real thing — a noise that makes him look, a glimpse, a step — takes it from the screen like any other. */
export function attentionLend(g: Game, gd: Guard, at: Vec3, o: { ears?: number; noises?: number } = {}): void { return crossToRust('attentionLend', undefined, [g, gd, at, o]) as any; }

// ---------------------------------------------------------------- showing it (ui/attentionviz.ts draws exactly this list; the check reads it)
/** one man's held attention as the floor shows it: a thread from his feet to the place and a ring there, `left` of it still to run (1 while he walks to it or
 *  it is lent — less than 1 from the start for a worn taking; → 0 at the thing); `ending` = he has said he is done; `worn` = not his first inside the memory */
export interface AttentionMark { gd: Guard; from: Vec3; at: Vec3; left: number; mode: HoldMode; kind: DrawKind; ending: boolean; worn: boolean; /** he is on his way over to it (not yet at the thing) */ walking: boolean; }
export function attentionMarks(g: Game): AttentionMark[] {
  if (!ATTENTION.on || !ATTENTION.marks || g.touring) return [];
  const out: AttentionMark[] = [];
  for (const gd of g.guards) {
    const H = attentionOf(g, gd); if (!H || gd.state === 'dead' || g.fogOfWar.hidden(gd)) continue;   // (fog of war: nothing live of a man nobody on the team sees — not his thread either)
    const left = H.mode === 'lent' ? 1 : clamp((H.until < 0 ? H.secs : H.until - g.time) / Math.max(1e-3, H.full), 0, 1);   // (against a FRESH taking's length)
    out.push({ gd, from: [gd.char.pos[0], 0.05, gd.char.pos[2]], at: [H.at[0], 0.05, H.at[2]], left, mode: H.mode, kind: H.kind, ending: H.mode !== 'lent' && H.until >= 0 && H.until - g.time <= ATTENTION.tellSecs, worn: H.nth > 1, walking: H.mode === 'go' && H.until < 0 });
  }
  return out;
}

/** THE THREAD'S WAY (ATTENTION.way; the maintainer, 10-22, of a thread drawn through a hut's wall: "weird that the line here is a straight line, not a pathed
 *  one. i'd expected pathed from squad commands"). For a man WALKING to the place: his feet, every corner of the way he will take, the place — his own nav
 *  path from where he is on it when it is the walk to this place, else the nav grid's answer (kept `wayReask` s; floorplan.ts wayOf is the squad's same
 *  picture). Straight (his feet, the place) for a man who only looks — a turned head, a lent hold, a man standing at the thing — and where the floor has no
 *  way at all. PICTURE ONLY: asked by the overlay, read by nothing in the update, and it WRITES NOTHING OF THE GAME'S — the kept answer lives with whoever
 *  draws (`kept`: the overlay's own table in ui/attentionviz.ts, a check's own Map), never in a side table here, so a game drawn and the same game not drawn
 *  hash alike; the grid's search writes nothing of the game's (its tables are the Rust module's, scratch of the one question). */
export interface AttentionWayKept { key: string; t: number; from: Vec3; pts: Vec3[] | null }
export interface AttentionWayStore { get(gd: Guard): AttentionWayKept | undefined; set(gd: Guard, w: AttentionWayKept): unknown }
export function attentionWay(g: Game, M: AttentionMark, kept: AttentionWayStore): Vec3[] {
  const gd = M.gd, p = gd.char.pos, flat = v3.distXZ, out: Vec3[] = [M.from];
  if (!ATTENTION.way || !M.walking) return [M.from, M.at];
  const add = (q: Vec3) => { if (flat(out[out.length - 1], q) > 0.05) out.push([q[0], M.from[1], q[2]]); };
  const P = gd.path, own = !!gd.pathGoal && gd.pathI < P.length && flat(gd.pathGoal, M.at) < 0.9 && flat(P[P.length - 1], gd.pathGoal) < 1.2;   // (a one-point side step to get unstuck is not his way)
  if (own) for (let i = gd.pathI; i < P.length; i++) add(P[i]);
  else {
    const key = `${M.at[0].toFixed(1)},${M.at[2].toFixed(1)}`, c = kept.get(gd), dt = c ? g.time - c.t : -1;   // (dt < 0: the clock went back on a load)
    let W = c && c.key === key && dt >= 0 && dt < ATTENTION.wayReask ? c : null;
    if (!W) kept.set(gd, W = { key, t: g.time, from: [p[0], 0, p[2]], pts: g.col.nav.findPath(p, M.at, 6000) });   // (6000: goTo's own budget — a place his walk finds is a place the thread finds)
    const pts = W.pts; if (!pts) return [M.from, M.at];
    // an answer kept from a stride ago starts behind him: from the end of the leg he is nearest on — looked for over its first two metres only, never 'the
    // corner nearest him' of the whole way (one that doubles back passes near him again further on, and the thread then cut the wall between)
    let k = 0, best = Infinity, arc = 0, a = W.from;
    for (let i = 0; i < pts.length && arc <= 2; i++) { const b = pts[i], dx = b[0] - a[0], dz = b[2] - a[2], l2 = dx * dx + dz * dz, u = l2 > 1e-9 ? clamp(((p[0] - a[0]) * dx + (p[2] - a[2]) * dz) / l2, 0, 1) : 0, d = Math.hypot(p[0] - a[0] - dx * u, p[2] - a[2] - dz * u); if (d < best - 1e-6) { best = d; k = i; } arc += Math.sqrt(l2); a = b; }
    for (let i = k < pts.length - 1 && flat(p, pts[k]) < 0.3 ? k + 1 : k; i < pts.length; i++) add(pts[i]);
  }
  add(M.at); if (out.length < 2) out.push(M.at);
  return out;
}

// ---------------------------------------------------------------- checkpoints (savestate.ts): every man's hold and memory by roster index, clocks as seconds from now
export type AttentionSnap = ({ hold: (Omit<Hold, 't0' | 'until'> & { age: number; left: number }) | null; takes: number[]; told: number; line?: number; heard?: number; lentOff?: number; series?: boolean; lent?: { at: Vec3; age: number; ears: number; noises: number } | null; tick?: number } | null)[];
export function attentionSnapshot(g: Game): AttentionSnap | undefined {
  if (!ATTENTION.on) return undefined;
  const men: AttentionSnap = g.guards.map(gd => {
    const M = memOf.get(gd), L = M && M.lent && g.time - M.lent.t <= STALE ? M.lent : null; if (!M || (!M.hold && !L && !M.takes.length && M.line < 0 && g.time >= M.lentOff)) return null;
    const H = attentionOf(g, gd);   // (a LENT hold is carried too, with the lend under it and the tick's own age: a loaded man at his screen has the eye the carried-on man has from the first frame, whichever of the lender and the tick runs first)
    return { hold: H ? { ...H, at: v3.copy(H.at), age: g.time - H.t0, left: H.until < 0 ? -1 : H.until - g.time } : null, takes: M.takes.map(t => g.time - t), told: g.time - M.toldAt, line: M.line, heard: g.time - M.heardT, lentOff: M.lentOff - g.time, series: M.series,
      lent: L ? { at: v3.copy(L.at), age: g.time - L.t, ears: L.ears, noises: L.noises } : null, tick: Math.min(1e6, g.time - M.tick) };
  });
  return men.some(m => m) ? men : undefined;
}
export function attentionRestore(g: Game, s: AttentionSnap | null | undefined) {
  g.guards.forEach((gd, i) => {
    const S = s?.[i]; if (!S) { memOf.delete(gd); return; }
    const M = mem(gd), now = g.time, h = S.hold;
    M.takes = (S.takes ?? []).filter(Number.isFinite).map(a => now - a); M.toldAt = Number.isFinite(S.told) ? now - S.told : -1e9; M.tick = Number.isFinite(S.tick) ? now - S.tick! : now; M.cand = null; M.owed = -1e9;
    const l = S.lent; M.lent = l && Array.isArray(l.at) ? { at: v3.copy(l.at), t: now - (l.age || 0), ears: l.ears ?? ATTENTION.earK, noises: l.noises ?? 1 } : null;
    M.line = Number.isFinite(S.line) ? S.line! | 0 : -1; M.heardT = Number.isFinite(S.heard) ? now - S.heard! : -1e9; M.lentOff = Number.isFinite(S.lentOff) ? now + S.lentOff! : -1e9; M.series = !!S.series;
    M.hold = h ? { at: v3.copy(h.at), kind: h.kind, mode: h.mode, t0: now - (h.age || 0), secs: h.secs, full: h.full || h.secs, until: h.left < 0 ? -1 : now + h.left, glimpse: h.glimpse || 0, told: !!h.told, nth: h.nth | 0 || 1, ears: h.ears ?? ATTENTION.earK, noises: h.noises ?? 1, yaw: h.yaw ?? null } : null;
  });
}

// ---------------------------------------------------------------- panel / QA
/** 'attention ▸ Novak on a noise (go, 3.2 s left) · 4 taken, 1 broken' */
export function attentionSummary(g: Game): string {
  if (!ATTENTION.on) return 'attention ▸ off';
  const parts: string[] = [];
  for (const gd of g.guards) { const H = attentionOf(g, gd); if (H) parts.push(`${gd.callsign} on a ${H.kind} (${H.mode}${H.until >= 0 ? `, ${Math.max(0, H.until - g.time).toFixed(1)} s left` : ''}${H.nth > 1 ? `, taking ${H.nth}` : ''})`); }
  const S = ATTENTION_STATS; parts.push(`${S.taken} taken, ${S.looks} looks, ${S.released} let go, ${S.broken} broken, ${S.spent} spent`);
  return 'attention ▸ ' + parts.join(' · ');
}
