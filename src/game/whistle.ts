// Sam's low whistle (V) — Chaos Theory's "psst": a small deliberate noise to pull ONE man off his line and over to where you were. It is a GameEvent like every
// other sound ('whistle', level WHISTLE.level, at Sam's feet, who = the player) and it reaches the guards through the SAME propagation the mixer and every other
// event use (collision.ts SoundPaths: straight through the air, round the corner along the walkable space and in through the doorway, or dully through a shut
// leaf) — guards.ts hearingCheck hands each man's copy to hearWhistle below. What it does to them:
//   · every living man within WHISTLE.horizon (straight line) whose propagated path is shorter than reachK × carry (× the alarmed floor's keener ear) HEARS it:
//     ~8 m of open air, ~5.8 m round a corner, ~2.6 m through a shut door — the numbers the probes print (tools/qa/whistle-probes.ts);
//   · of the calm men who hear it (patrol / suspicious, on nobody's errand, not choreographed, not in Sam's arm) only the NEAREST by path answers: a line from a
//     small pool, half a second to place it, then he walks over to where it came from — the doorway or corner it came through if it was routed — and looks
//     about, exactly the suspicion a creaking door gives him (awareness 0.45: he talks himself out of it on arrival), torch still in hand, nothing on the net;
//     the others who heard it turn their heads that way for a second and a half and carry on;
//   · the same man answering a SECOND whistle inside twiceWindow seconds stops being polite: a proper search of that area (the 'search' state and its plan of
//     hiding places round the point, gun out), still without a word on the radio unless the search finds something;
//   · a man already hunting (alert / search) takes it as one more fix on where you are — no line, no change of state; a man about to go loud anyway (a shot in the
//     same half second) is not talked down to 'hello?' by it.
// The player's side (whistleKey / tryWhistle): V, anywhere but down, once per `cooldown` seconds — the event, the sound meter's tick, the synthesized two-note
// voice at his head (sfx.ts 'whistle'); no HUD line, it is diegetic. The knobs (WHISTLE) sit in Tab ▸ Sandbox and are kept per machine like the aim rows.
import './seats';
import { SideMap } from '../math/side';
import { Vec3, v3 } from '../math/vec';
import type { Game, Guard, GameEvent } from './game';
import type { UpperMode } from './character';
import './teams';
import { wg } from './words';   // (a leaf: the dialogue pass's one switch)
import { crossToRust } from './crossing';

/** Every number of it (panel-tunable; the QA probes read them rather than copy them). */
export const WHISTLE = {
  /** the hearing-model level stamped on the event and shown on the SOUND meter (a crouched step on lino is ~0.2, a walk 0.33, the suppressed pistol 0.6) */
  level: 0.35,
  /** metres of OPEN AIR it carries (the propagated path must be shorter than this × the route's carry: 1 direct, 0.9 over a partition, 0.72 round a corner,
   *  0.32 through a shut door, 0.25 through the mass) — × KEEN on an alarmed floor, as every small noise */
  reachK: 8,
  /** straight-line pre-filter (m): nobody further off than this is even asked (the propagation is the dear part) */
  horizon: 12,
  /** seconds between two whistles (V inside it does nothing) */
  cooldown: 2,
  /** the same man answering a second one inside this many seconds searches the place properly instead of taking another look */
  twiceWindow: 60,
  /** what answering it leaves him with: the suspicion of a creak (under the 0.85 that is CONTACT, over the 0.3 that is 'huh?'), and the beat he takes to place it */
  awareness: 0.45, reactT: 0.5,
  /** while he is still WALKING over to one, his suspicion is held at least this high (guards.ts, the suspicious branch: it wears off at 0.06/s otherwise, and a spot
   *  eight metres off had him give up two strides short of it); arrived, it drains from here through the look round — ~3 s of torch into the corners — and he
   *  talks himself out of it ('must have been nothing') and goes back. 0 = the creak's behaviour exactly. */
  walkFloor: 0.32,
  /** the others who heard it: how long their heads stay turned toward it (s), and how far round a neck goes for it (rad) */
  glanceSecs: 1.5, glanceMax: 1.2,
  /** the voice's gain at Sam's head (sfx.ts 'whistle' peaks ~0.3: quiet — it is for the man six metres off, not the floor) */
  gain: 0.55,
};
export type WhistleKnobs = typeof WHISTLE;
/** How the man who answers it CARRIES himself on the way over while it is still only a whistle to him (guards.ts, the suspicious branch, via whistleSoft): 'torch'
 *  = the torch comes up to shoulder height and goes where he looks (the animator's Idle_Torch_Loop over the walk — in the hand that holds it with gunhand.ts
 *  GUNHAND.whistle; the clip's own raised hand is the empty one); 'none' = the calm patrol's low carry, the beam
 *  swung up onto the spot from the hip. The one-line taste switch. Either way the pistol stays on his belt until something more than a whistle reaches him. */
export const WHISTLE_SOFT_UPPER: UpperMode = 'torch';

/** what the man who goes to look says (placeholder lines — the writing pass is the maintainer's), and what he says the second time */
const LOOK_LINES = ['hello?', 'someone there?', '…what was that?', 'who\'s that whistling?', 'somebody down there?'];
const TWICE_LINES = ['right — who\'s messing about?', 'that\'s twice now. come on out.', 'okay. not funny. where are you?'];
/** (WORDS) the same pool as it is said now, place for place (read where the line is picked) */
const TWICE_NOW = ['all right — who\'s messing around?', 'that\'s twice now. come on out.', 'okay. not funny. where are you?'];
/** the same two moments from a man who is NOT the night staff (words.ts; team 'hostile': an armed defender holding a place does not call 'hello?' or 'not funny' into the dark) */
const LOOK_FOE = ['…quiet. listen.', 'you hear that?', "something's there."];
const TWICE_FOE = ["that's no bird. going to look.", "someone's close. eyes up."];
/** the whistle's lines, for the QA soak (its own smell, and only ever from a man who has just answered one) */
export function whistleLines(kind: 'look' | 'twice' | 'all' = 'all'): string[] { return kind === 'look' ? [...LOOK_LINES, ...LOOK_FOE] : kind === 'twice' ? [...TWICE_LINES, ...wg([TWICE_NOW[0]], []), ...TWICE_FOE] : [...LOOK_LINES, ...LOOK_FOE, ...TWICE_LINES, ...wg([TWICE_NOW[0]], []), ...TWICE_FOE]; }

// ---------------------------------------------------------------- per-man memory (dies with the Guard object: resetGuards makes fresh men)
/** 'never' for respondedAt: far enough back that no twiceWindow the panel allows reaches it, still finite for the logs */
const NEVER = -1e9;
export interface WhistleMem {
  /** game time he last ANSWERED one (went to look / to search), NEVER if not yet; how many answered, how many of those became searches */
  respondedAt: number; responses: number; searches: number;
  /** whistles that reached him while he was already hunting (a fix, nothing said) / that somebody nearer answered (a turned head) */
  cues: number; glances: number;
  /** the turned head: until when, and toward where */
  glanceUntil: number; glanceAt: Vec3 | null;
  /** his current suspicion is a whistle's and nothing more (whistleSoft): set answering one, cleared back on patrol / hunting; softAt = when */
  soft: boolean; softAt: number;
  /** the last line he used from each pool (no saying the same one twice running) */
  lastLook: number; lastTwice: number;
}
const memOf = new SideMap<Guard, WhistleMem>('whistle.memOf');
function mem(gd: Guard): WhistleMem {
  let M = memOf.get(gd);
  if (!M) { M = { respondedAt: NEVER, responses: 0, searches: 0, cues: 0, glances: 0, glanceUntil: -1, glanceAt: null, soft: false, softAt: NEVER, lastLook: -1, lastTwice: -1 }; memOf.set(gd, M); }
  return M;
}
/** read-only peek for the panel / the QA harness (null: no whistle has ever concerned this man) */
export function whistleMem(gd: Guard): Readonly<WhistleMem> | null { return memOf.get(gd) ?? null; }

// ---------------------------------------------------------------- the player's side
export interface Blower { readyAt: number; blown: number; lastAt: number; }
const blowerOf = new SideMap<object, Blower>('whistle.blowerOf');   // keyed by the Player object: a restart's fresh Sam starts with a fresh breath
function blower(g: Game): Blower { let B = blowerOf.get(g.player); if (!B) { B = { readyAt: -1, blown: 0, lastAt: -100 }; blowerOf.set(g.player, B); } return B; }
/** module-wide counters for the QA soak (events blown, events resolved against the guards, men who answered / searched / took the cue / turned a head) */
export const WHISTLE_STATS = { blown: 0, resolved: 0, heardBy: 0, answered: 0, searched: 0, cued: 0, glanced: 0, unanswered: 0 };

/** seconds until V will sound again (0 = ready) */
export function whistleReadyIn(g: Game): number { return Math.max(0, blower(g).readyAt - g.time); }


/** Blow it if the breath is back: the event at Sam's feet (hearWhistle lifts it to mouth height for the propagation), the meter's tick, the voice at his head.
 *  False = still inside the cooldown (or down): nothing happened. */
export function tryWhistle(g: Game): boolean { return crossToRust('tryWhistle', undefined, [g]) as any; }

// ---------------------------------------------------------------- the guards' side
/** who a whistle reached and how (path length, where it seemed to come from), and which calm man is to answer it — worked out ONCE per event, the first time any
 *  man's hearingCheck meets it, against every living man on the floor at that moment */
interface Heard { d: number; at: Vec3; direct: boolean; carry: number; }
interface Resolution { hearers: Map<Guard, Heard>; responder: Guard | null; answeredBy: Guard | null; t: number; }
const resOf = new SideMap<GameEvent, Resolution>('whistle.resOf');
/** the last few resolutions, newest last (QA: 'only one man answers', 'through the door nobody heard it') */
export const WHISTLE_LOG: { t: number; pos: Vec3; hearers: string[]; responder: string | null; answered: string | null; search: boolean }[] = [];





/** Is this man's suspicion (still) only a whistle's: he answered one, is walking over or looking about, and nothing since has sharpened it — no footfall, no
 *  glimpse, no slam (any of those lifts his awareness past what the whistle left him), no CONTACT since. guards.ts handCarry keeps the torch in his hand for it. */
export function whistleSoft(gd: Guard): boolean {
  const M = memOf.get(gd);
  return !!M && M.soft && gd.state === 'suspicious' && gd.awareness <= WHISTLE.awareness + 0.02 && gd.witness.alertT < M.softAt;
}



// ---------------------------------------------------------------- checkpoints (game/savestate.ts): Sam's breath, and every man's memory of the whistles by roster index
export interface WhistleSnap { blower: Blower; men: (WhistleMem | null)[]; }
export function whistleSnapshot(g: Game): WhistleSnap {
  return { blower: { ...blower(g) }, men: g.guards.map(gd => { const M = memOf.get(gd); return M ? { ...M, glanceAt: M.glanceAt ? v3.copy(M.glanceAt) : null } : null; }) };
}
/** …onto the FRESH Player (the blower is keyed by that object) and the fresh roster. The whistle EVENTS themselves are not put back (savestate.ts applyState drops
 *  kind 'whistle' from the restored half second): `resolve` above answers each event object once for the whole floor, and everybody in earshot heard it the frame
 *  it sounded — a fresh copy of it would be a second whistle nobody blew (review G3.8). */
export function whistleRestore(g: Game, s: WhistleSnap | null | undefined) {
  if (!s) return;
  const B = blower(g); const b = s.blower; if (b) { B.readyAt = Number.isFinite(b.readyAt) ? b.readyAt : -1; B.blown = b.blown | 0; B.lastAt = Number.isFinite(b.lastAt) ? b.lastAt : -100; }
  s.men?.forEach((M, i) => {
    const gd = g.guards[i]; if (!gd || !M) return;
    const m = mem(gd);
    m.respondedAt = Number.isFinite(M.respondedAt) ? M.respondedAt : NEVER; m.responses = M.responses | 0; m.searches = M.searches | 0; m.cues = M.cues | 0; m.glances = M.glances | 0;
    m.glanceUntil = Number.isFinite(M.glanceUntil) ? M.glanceUntil : -1; m.glanceAt = M.glanceAt ? v3.copy(M.glanceAt) : null; m.soft = !!M.soft; m.softAt = Number.isFinite(M.softAt) ? M.softAt : NEVER; m.lastLook = M.lastLook ?? -1; m.lastTwice = M.lastTwice ?? -1;
  });
}

// ---------------------------------------------------------------- panel / QA
/** 'whistle ▸ ready · 3 blown · Novak answered 4 s ago (×2, 1 search) · Reyes turned his head' — or what the last one did */
export function whistleSummary(g: Game): string {
  const parts: string[] = [];
  const B = blower(g); const rd = whistleReadyIn(g);
  parts.push(rd > 0 ? `again in ${rd.toFixed(1)} s` : 'ready (V)'); parts.push(`${B.blown} blown`);
  for (const gd of g.guards) {
    const M = memOf.get(gd); if (!M || gd.state === 'dead') continue;
    const bits: string[] = [];
    if (M.responses) bits.push(`answered ${(g.time - M.respondedAt).toFixed(0)} s ago ×${M.responses}${M.searches ? ` (${M.searches} search${M.searches > 1 ? 'es' : ''})` : ''}${whistleSoft(gd) ? ', torch' : ''}`);
    if (M.cues) bits.push(`${M.cues} cue${M.cues > 1 ? 's' : ''}`);
    if (M.glances) bits.push(`${M.glances} glance${M.glances > 1 ? 's' : ''}${g.time < M.glanceUntil ? ' (looking)' : ''}`);
    if (bits.length) parts.push(`${gd.callsign}: ${bits.join(', ')}`);
  }
  const L = WHISTLE_LOG[WHISTLE_LOG.length - 1];
  if (L) parts.push(`last: heard by ${L.hearers.length ? L.hearers.join('/') : 'nobody'}${L.answered ? ` — ${L.answered} ${L.search ? 'searches' : 'looks'}` : L.responder ? ` — ${L.responder} to answer` : ''}`);
  return 'whistle ▸ ' + parts.join(' · ');
}
