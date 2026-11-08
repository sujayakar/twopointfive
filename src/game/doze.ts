// DOZING IN THE DARK — a calm guard in an unlit room has a reason to be there (the maintainer, 2026-10-13: "i wonder if we even just want to have guards
// sleeping in dark rooms or something like that. like snoozing off in a chair. it should make sense why they're hanging out when they can't see.").
//
// BEFORE: a staff man whose route has him WAIT in a place with no light stood there on his feet and stared into the black for the length of the wait.
// NOW (DOZE.on): a calm staff man who comes to a wait of `minWait` to `maxWait` seconds at a waypoint where the place is dark by the calm torch's own reading
// (calmtorch.ts workLit) and the floor he faces is dark too (darkfoe.ts darkFoeBlind: a man in the dark watching a LIT door is a picket, and stays one), with a free chair within `chairM` that is in the dark too, walks to the chair, sits down in it (close.ts closeSeat: the seated man the
// close-in mode already takes 'over a chair') and nods off: head down, torch dark, a 'z' over him that grows with his breath. His wait clock RUNS the whole time,
// so the level's timetable is close to what it was (not equal: he gets up at the chair, not at his stand, and leaves a second or two late): `riseSecs` before the wait is over he stretches, gets up and walks his route on.
//
// ASLEEP his eyes are shut (he sees nobody, no body, no light going out: guards.ts canSeeChar / pointInView / bodySeen ask dozeAsleep, so every reader of a
// man's eyes has it) and his ears are dull: guards.ts hearingCheck runs for him as for anybody, its small noises at `hearK` of a waking man's. WHAT WAKES HIM:
//   noise   anything hearingCheck counts at that bar: gunfire, a charge or a door kicked in (as loud to him as to anybody), a whistle that is his to answer
//           (whistle.ts: the nearest calm man in its reach), a door or furniture noise, glass, an enemy's footfall — a crouched step never does, a walking
//           step does beside his chair, a run does across the room
//   light   `lightAt` on the meter's scale on his face for `lightSecs` from anything but his own side's lamps: a weapon light, the room's lights coming on
//   bump    his chair shoved by anybody
//   the floor going up (an alarm, a body called in over the net, the mains cut: he is up with the rest, by their rules)
// A man woken by noise, light or a bump is SUSPICIOUS from the frame he wakes (`wakeAw`), placed on what woke him, rooted `stirSecs` while he comes round — and he
// does not doze again that run: the room has an awake man in it from then on. Taken from behind in his chair he never wakes at all. Asleep he says nothing and
// calls nobody on the radio (needs.ts partnerReply, guards.ts radioCaller / spokesman); he does count in on the radio check, eyes shut (one log line).
//
// AT A SCREEN (the maintainer's steer, the same day: "working on a computer in a dark room (with their face illuminated by light) could be a reasonable
// setting. it'd also interact with their attention being on the screen (and maybe even headphones on?) so they'd be less perceptive. and some probably some
// opportunities for funny writing too."). Where the chair is at one of the level's screens (a switchable in group 'monitors', the chair on its lit side) the
// same wait can be sat out AT it: he shifts the chair (`pullM` at most, to where a man facing the screen has room for his feet — which may be a hand AWAY from it), sits FACING it, switches it
// on if it was off — its light is on his face and in the room: a light like any other, to the meter and to every eye — and is at work: awake, eyes open and
// on the screen (he sees what is in front of him, which is the screen), headphones on (hearingCheck at `noises` for the small noises, `ears` for a footfall:
// dozeEar), a '♪' over him until something has reached him, and now and then a word for what is on it (SCREEN_LINES: a bubble, no radio, no log row; no line
// twice on the floor until all have been said). A wait under `napWait` prefers the screen, a longer one the sleep — one chair can be both. He is up for what
// gets any seated man up (a noise that does reach him: the net of awareness 0.3; a shove of his chair; the alarm; the mains — the screen dies with them),
// and for the screen going dark under his eyes ('light': up, looking at it). Got up by any of those, hauled out of the chair or put down in it, he leaves the screen ON, and it
// stays on (LEFT) for `idleSecs` with nobody at it, then blanks — or until the next man who sits to it gets up at the end of his wait: he switches it off. At the end of his wait a man
// switches off what he switched on, gets up and walks on, and the chair stays where he pulled it. `DOZE.screen` false = no screens: the men doze or stand,
// as before it (a recording from before the key replays so: journal.ts). THE SEAM with the attention rule (attention.ts, another goal's):
// how held attention narrows a man's EYES is that rule's. This file lends it the screen each frame he is at work (attentionLend, `lend`): with that rule on, a man
// off to the side of the screen is taken in at its `sideK` and the floor shows the thread and the ring. The hold ends by that rule (a man at the edge of his
// eye who has his awareness to its `breakGain`, a step that reaches him) and is lent again `lentBackSecs` on if he is still sat; he stays SAT until this file's
// own 0.3. His EARS stay this file's (`ears` / `noises` through dozeEar; the lend passes 1 and 1), so the headphones count once and work with that rule's
// switch off. With it off, or `lend` off, his eyes are whole. That rule holds nobody on a loud squad floor (its own `able`), and it keeps the lend for a
// quarter second after the last call: a man got up at the end of his wait walks his first steps with the thread still on the dark screen.
//
// The switch: OFF in the module and on the harness's frozen stand (TPS_DOZE=1 raises it there); the page turns it on (host/profile.ts pageSwitches; ?nodoze = every
// man stands his wait, as it was). Off = the build as it was by run digest: every export below returns on its first line. It needs close.ts's seat (CLOSE.on).
// A recording from before the key replays with it off (journal.ts applyKnobs). CHECK: tools/qa/doze-check.ts (`probes.ts doze`; DOZE_BREAK=1 must fail).
// NEVER SEEN ON A GPU: the slump (character.ts `slump`), the chair under him, the 'z' and the '♪', the screen's light on a seated man's face and in the room,
// the chair pulled up are numbers nobody has looked at. He makes NO SOUND yet: a breath or a
// snore is the sound table's to add (dozeBreath below is its phase).
import './seats';
import { SideMap } from '../math/side';
import type { Game, Guard, LightTarget } from './game';
import type { Prop } from './props';
import type { Vec3 } from '../math/vec';
import './guards';
import './calmtorch';
import './darkfoe';
import { CLOSE, closeSeat, closeUnseat } from './close';
import './attention';
import '../math/exact';
import { LIGHT_IN } from '../sim/lightshome';

/** the switch and its numbers. `minWait` s: the shortest wait he sits one out for · `maxWait` s: the longest (a man stood somewhere longer than this is POSTED
 *  there: a picket, not a man pausing on his round) · `chairM`: how far from his waypoint the chair may stand (straight; the walk to
 *  it may be `chairM` × 1.6) · `nodSecs`: seated this long his eyes close · `riseSecs`: he is up this long before the wait ends · `hearK`: his ears asleep, as a
 *  share of a waking man's (the small noises: a step, a door, furniture, glass, a silenced round) · `lightAt` / `lightSecs`: light on his face, on the meter's 0‥1 scale, for this
 *  long · `askSecs`: how often the light is read · `wakeAw`: the awareness he wakes with (0.3 = suspicious, 0.85 = alert) · `bumpAw`: the same for a shoved chair
 *  · `stirSecs`: rooted while he comes round · `breathSecs`: one breath (the 'z' grows over it). UNTUNED by eye and by hand: first numbers. */
export const DOZE = { on: false, minWait: 20, maxWait: 90, chairM: 4.5, nodSecs: 2.5, riseSecs: 2.0, hearK: 0.6, lightAt: 0.3, lightSecs: 0.8, askSecs: 0.25, wakeAw: 0.55, bumpAw: 0.7, stirSecs: 1.3, breathSecs: 3.6,
  /** AT A SCREEN (below): `screen` = the rule's own half-switch (false = every such man dozes or stands, as before it) · `screenM`: the chair is this near the
   *  screen, on its lit side, and `pullM` is how far he will pull a chair up to it · `napWait`: a wait this long sleeps in a chair before it sits to a screen, a shorter one the other way round · `settleSecs`: seated
   *  this long he is at work · `ears` / `noises`: headphones on — a footfall and every other small noise at his
   *  ear, as shares · `sayFirst` / `sayEvery` [from, to] s: his first mutter at the screen, and the gap to each one after · `idleSecs`: a screen a man was got up from, or hauled away from, stays lit this long with nobody at it, then blanks ·
   *  `lend`: at work his attention is lent to the screen (attention.ts: the narrowed eye, the thread, the ring) — off = his eyes whole, as before the wiring */
  screen: true, screenM: 1.5, pullM: 0.5, napWait: 40, settleSecs: 1.2, ears: 0.5, noises: 0.6, sayFirst: [5, 11], sayEvery: [16, 30], idleSecs: 60, lend: true };

export type DozeWhy = 'noise' | 'light' | 'bump' | 'alarm' | 'net' | 'time' | 'taken';
type Phase = 'no' | 'going' | 'down' | 'asleep' | 'work' | 'stir' | 'up' | 'spent';
/** one man's record. `no` = this wait was looked at and has no chair for him (asked once a wait) · `going` = on his way to the chair · `down` = seated, eyes
 *  open · `asleep` · `work` = at his screen, awake · `stir` = woken, coming round · `up` = risen at the end of his wait, a moment clear of the chair · `spent` = woken once: awake for the run */
interface Doze { phase: Phase; wp: number; chair: Prop | null; mon: LightTarget | null; monWas: boolean; say: number; to: Vec3; at: Vec3; face: number; cx: number; cz: number; cyaw: number; t: number; seen: number; until: number; lit: number; ask: number; push: number; why: DozeWhy | null; wokeAt: number; }
const GAMES = new SideMap<Game, Map<Guard, Doze>>('doze.GAMES');
const REC = new SideMap<Guard, Doze>('doze.REC');
const SAID = new SideMap<Game, number[]>('doze.SAID');
/** how far, and which way, a man will pull a chair up: every 5 cm offset inside `pullM`, nearest first (built on first use) */
let PULLS: [number, number][] | null = null, PULLS_M = -1, PULLS_E = -1;   // (PULLS_E: sorted by lengths, so by the maths of the day — math/exact.ts)
const mapOf = (g: Game) => { let m = GAMES.get(g); if (!m) { m = new Map(); GAMES.set(g, m); } return m; };
export const DOZE_STATS = { sat: 0, slept: 0, worked: 0, lines: 0, rose: 0, woke: { noise: 0, light: 0, bump: 0, alarm: 0, net: 0, taken: 0 } };
/** QA: every wake, in order */
export const DOZE_LOG: { t: number; who: string; why: DozeWhy; secs: number }[] = [];

function put(g: Game, gd: Guard, D: Doze) { mapOf(g).set(gd, D); REC.set(gd, D); }
function drop(g: Game, gd: Guard) { mapOf(g).delete(gd); REC.delete(gd); gd.char.anim.slump = 0; }
/** the man at a screen is taken off the books (the switch off, the roster changed, a reset): what he switched on goes off with him */
function dark(g: Game, D: Doze) { if (D.phase === 'work' && D.mon && !D.monWas) g.lightHome.in(LIGHT_IN.on, D.mon, 0); }
/** the screens a man was got up from, or hauled away from, and LEFT LIT (he switched it on; nobody has switched it off), each with the game time it goes
 *  dark by itself (`idleSecs`: a screen nobody touches blanks) — sooner if the next man who sits to it gets up at the end of his wait: he switches it off
 *  as if it were his own. In the checkpoint. */
const LEFT = new SideMap<Game, Map<LightTarget, number>>('doze.LEFT');
function leftOff(g: Game) { const L = LEFT.get(g); if (!L) return; for (const t of L.keys()) g.lightHome.in(LIGHT_IN.on, t, 0); LEFT.delete(g); }
const blank = (phase: Phase, wp: number, t: number): Doze => ({ phase, wp, chair: null, mon: null, monWas: false, say: -1, to: [0, 0, 0], at: [0, 0, 0], face: 0, cx: 0, cz: 0, cyaw: 0, t: 0, seen: t, until: 0, lit: -1, ask: 0, push: 0, why: null, wokeAt: -1 });

const seated = (D: Doze) => D.phase === 'down' || D.phase === 'asleep' || D.phase === 'work';
/** WHAT HE SAYS TO HIS SCREEN (a bubble over him, nothing on the net, no log row): the night shift's paperwork. `dozeLines()` hands them to the words lint. */
const SCREEN_LINES = [
  'password expired. of course it has.',
  'reply all. he replied all.',
  '…and it\'s frozen. wonderful.',
  'who formats a duty roster like this.',
  'no. no, I saved that.',
  'page two of two. there is no page two.',
  'the overtime form wants a form.',
  'mandatory fun. who writes these.',
  'loading… still loading.',
  'printer offline. it\'s right there.',
  'one more hand. then the rounds.',
  'unread, unread, unread. all him.',
];
export function dozeLines(): string[] { return [...SCREEN_LINES]; }


/** out of the chair, whatever had him in it */
function up(g: Game, gd: Guard) { closeUnseat(g, gd); gd.char.anim.slump = 0; }   // (nothing without a seat)




/** is he asleep this frame (eyes shut: guards.ts witnessKill, the torch, the HUD) */
export function dozeAsleep(gd: Guard): boolean { return REC.get(gd)?.phase === 'asleep'; }
/** his torch is dark: sat down to it, or asleep (guards.ts poseFlashlight) */
export function dozeDark(gd: Guard): boolean { const D = REC.get(gd); return !!D && seated(D); }
/** for the neighbours: what he is at in his chair — 'asleep', at his 'screen' (awake, his mind on it), or null */
export function dozeAt(gd: Guard): 'asleep' | 'screen' | null { const p = REC.get(gd)?.phase; return p === 'asleep' ? 'asleep' : p === 'work' ? 'screen' : null; }
/** for the neighbours: was he woken out of a doze inside the last `secs` seconds, and by what (null = no). A man shaken awake in the dark is the one man a
 *  rule about suspicious lamps may want to treat apart. */
export function dozeWoke(g: Game, gd: Guard, secs = 6): DozeWhy | null { const D = REC.get(gd); return D && D.why && D.why !== 'time' && D.wokeAt >= 0 && g.time - D.wokeAt < secs ? D.why : null; }
/** QA / the panel: what the rule has of a man */
export function dozeOf(gd: Guard): Readonly<{ phase: Phase; why: DozeWhy | null; secs: number; wp: number }> | null { const D = REC.get(gd); return D ? { phase: D.phase, why: D.why, secs: D.t, wp: D.wp } : null; }
/** 'dozing ▸ Novak asleep 41 s · Reyes woken (noise)' */
export function dozeSummary(g: Game): string {
  if (!DOZE.on) return 'dozing ▸ off (every man stands his wait)';
  const rows: string[] = []; for (const [gd, D] of mapOf(g)) { if (D.phase === 'no') continue; rows.push(`${gd.callsign} ${D.phase === 'asleep' ? `asleep ${D.t.toFixed(0)} s` : D.phase === 'work' ? `at his screen ${D.t.toFixed(0)} s` : D.phase === 'spent' || D.phase === 'stir' ? `woken (${D.why})` : D.phase === 'going' ? 'to his chair' : D.phase === 'down' ? 'sitting down' : 'getting up'}`); }
  return `dozing ▸ ${rows.length ? rows.join(' · ') : 'nobody'}${CLOSE.on ? '' : ' · needs the close-in seat (CLOSE.on)'}`;
}
/** everybody up, the records gone (resetGuards / restartEncounter: the men are new objects) */
export function dozeReset(g: Game, load = false) { SAID.delete(g); if (load) LEFT.delete(g); else leftOff(g); const M = GAMES.get(g); if (!M) return; for (const [gd, D] of [...M]) { if (!load) dark(g, D); if (seated(D)) up(g, gd); drop(g, gd); } }   // (`load`: a checkpoint is being put back — the screens are the file's to set)

// ---- a checkpoint carries who sits where, who sleeps, who was woken and whose wait has no chair (no key with the switch off or nobody on a record)
export interface DozeSnap { men: { i: number; phase: Phase; wp: number; chair: number; mon?: number; monWas?: boolean; say?: number; to?: number[]; at: number[]; face: number; cx: number; cz: number; cyaw: number; t: number; until: number; lit: number; ask?: number; why: DozeWhy | null; wokeAt: number }[]; said?: number[]; left?: number[][] }
export function dozeSnapshot(g: Game): DozeSnap | null {
  const M = GAMES.get(g), said = SAID.get(g), left = [...LEFT.get(g) ?? []].map(([t, at]) => [g.targets.indexOf(t), at - g.time]).filter(e => e[0] >= 0); if (!DOZE.on || !M && !left.length || M && !M.size && !said?.length && !left.length) return null;
  const men: DozeSnap['men'] = [];
  for (const [gd, D] of M ?? []) { const i = g.guards.indexOf(gd); if (i < 0) continue;
    men.push({ i, phase: D.phase, wp: D.wp, chair: D.chair ? g.props.props.indexOf(D.chair) : -1, ...(D.mon ? { mon: g.targets.indexOf(D.mon), monWas: D.monWas, ...(D.say >= 0 ? { say: D.say - g.time } : null) } : null), ...(D.phase === 'going' ? { to: [D.to[0], D.to[1], D.to[2]] } : null), at: [D.at[0], D.at[1], D.at[2]], face: D.face, cx: D.cx, cz: D.cz, cyaw: D.cyaw, t: D.t, until: D.until - g.time, lit: D.lit, ask: D.ask, why: D.why, wokeAt: D.wokeAt < 0 ? -1 : g.time - D.wokeAt }); }
  return men.length || said?.length || left.length ? { men, ...(said?.length ? { said: [...said] } : null), ...(left.length ? { left } : null) } : null;
}
export function dozeRestore(g: Game, s: DozeSnap | null | undefined) {
  dozeReset(g, true); if (!s || !DOZE.on) return;
  if (s.said?.length) SAID.set(g, [...s.said]);
  if (s.left?.length) { const L = new Map<LightTarget, number>(); for (const [i, secs] of s.left) { const t = g.targets[i]; if (t) L.set(t, g.time + secs); } if (L.size) LEFT.set(g, L); }
  for (const m of s.men) { const gd = g.guards[m.i]; if (!gd || gd.state === 'dead') continue;
    const D = blank(m.phase, m.wp, g.time); D.chair = g.props.props[m.chair] ?? null; D.mon = m.mon !== undefined ? g.targets[m.mon] ?? null : null; D.monWas = !!m.monWas; D.say = m.say === undefined ? -1 : Math.max(0, g.time + m.say); D.at = [m.at[0], m.at[1], m.at[2]]; D.face = m.face; D.to = m.to ? [m.to[0], m.to[1], m.to[2]] : [D.at[0] + Math.sin(D.face) * 0.33, 0, D.at[2] + Math.cos(D.face) * 0.33];   /* (`say` is seconds from the save, and may be past due: a bubble was in its way. A file from before `to`: the floor in front of the chair, as it was then) */ D.cx = m.cx; D.cz = m.cz; D.cyaw = m.cyaw; D.t = m.t; D.until = g.time + m.until; D.lit = m.lit; D.ask = m.ask ?? 0; D.why = m.why; D.wokeAt = m.wokeAt < 0 ? -1 : g.time - m.wokeAt;
    if ((D.phase === 'going' || seated(D) || D.phase === 'up') && !D.chair) continue;
    if (D.phase === 'work' && !D.mon) continue;
    if (seated(D)) { gd.hold = false; if (!closeSeat(g, gd, D.at, D.face)) continue; D.push = NaN; gd.char.anim.slump = D.phase === 'asleep' ? 1 : D.phase === 'work' ? 0.3 : D.mon ? 0.15 : 0.35; }
    put(g, gd, D); }
}
