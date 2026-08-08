// Interrogation dialogue engine (docs/internal/grab-interrogate-design.md §4, slice 5): a pure-ish function over state the AI already keeps — each man's own
// witness record (game.ts Witness: what HE saw and heard), his sightings, the floor's alarm level, the net's radio check, the lockdown duties — that answers
// 'Sam has this man by the throat and leans on him: what does he say?'. Three axes pick the cell: KNOWLEDGE of Sam (none / heard / glimpsed / saw / sawHurt —
// his own perception only, never the floor's), AGITATION (0‥1 → low / mid / high, from the floor state, what he personally went through, how long he has been
// held, and his temperament), TEMPERAMENT (game.ts TEMPERAMENT: steady / nervy / hard). Each cell has STAGES — 1 the reaction, 2 the pressure, 3 the one useful
// thing he plausibly knows (the INTEL generator) or the admission / refusal that he has none, then 'x', nothing left, tapped for as long as you like — each a
// small pool sampled without repeats per man. Nothing here imports the game beyond types and three read-only values (TEMPERAMENT, rollcallState, escalationOf);
// gather() and the small readers it is built from (knowledgeOf, agitationOf, the token resolvers) are all that touch a Game — the picker works on the plain
// DialogueCtx gather returns (the QA probes synthesize those by hand to sweep the whole matrix). Nothing is written back but the man's own Talk record.
//
// The grab (player.ts startGrab / updateHold) owns 'held': while Guard.held is set its `since` IS the held clock and nothing here touches it; the sandbox
// button's bare exchange with a man nobody holds still stamps talk.heldSince itself and lets it lapse (heldSinceOf), so the probes can sweep the matrix.
//
// ============================================================================================================================================================
//   EVERY LINE OF DIALOGUE IN THIS FILE (the LINES table and the INTEL texts) IS A DRAFT PLACEHOLDER, written by the engine's author only so the cells have
//   something in them to exercise the picker. They are placeholders to be rewritten in the project's own voice (slice 6). Constraints for the rewrite are the ones
//   lintDialogue() enforces: ≤ MAX_CHARS characters after the longest token expansion, one sentence or two fragments, no line depending on another having been
//   shown (pools are sampled, not sequenced), tokens only from TOKENS below, needs only from NEEDS.
// ============================================================================================================================================================
import { v3 } from '../math/vec';
import type { Vec3 } from '../math/vec';
import type { Level } from '../scene/level';
import type { Game, Guard, Temperament, Talk } from './game';
import { TEMPERAMENT } from './game';
import { rollcallState, escalationOf } from './guards';

// ---------------------------------------------------------------- axes
export type Knowledge = 'none' | 'heard' | 'glimpsed' | 'saw' | 'sawHurt';
export const KNOWLEDGE: Knowledge[] = ['none', 'heard', 'glimpsed', 'saw', 'sawHurt'];
export type Band = 'low' | 'mid' | 'high';
export const BANDS: Band[] = ['low', 'mid', 'high'];
export const TEMPERAMENTS: Temperament[] = ['steady', 'nervy', 'hard'];
export type StageKey = 1 | 2 | 3 | 'x';
export const STAGES: StageKey[] = [1, 2, 3, 'x'];
/** agitation bands: calm below MID, rattled below HIGH, panicked from there (hard men's 'panicked' is written as cold fury, not pleading) */
export const BAND_MID = 0.3, BAND_HIGH = 0.6;
export function bandOf(a: number): Band { return a >= BAND_HIGH ? 'high' : a >= BAND_MID ? 'mid' : 'low'; }
/** hard limit for one line after token expansion (the bubble is 190 px of 11 px text: ~3 lines of it), and the reach of the sandbox gate until the grab exists */
export const MAX_CHARS = 90;
export const INTERROGATE_REACH = 2.0;
/** seconds held after which 'heldLong' lines unlock; seconds without an exchange after which the SANDBOX (a man nobody holds) treats the next press as a fresh hold;
 *  seconds an aborted choke / whip keeps a man that has since been let go rattled (held, it lasts the hold) */
export const HELD_LONG_SECS = 40, HOLD_LAPSE_SECS = 30, ABORT_RATTLE_SECS = 60;

/**
 * knowledgeOf — what this man knows of SAM, the highest level that holds, from HIS OWN record only (the floor's alarm level moves agitation and unlocks
 * colour lines; it is never knowledge — a man told 'weapons out' over the net has still never perceived anyone):
 *
 *   level      holds when (fields on Guard gd / gd.witness W / W.heard H)                              means
 *   ─────────  ──────────────────────────────────────────────────────────────────────────────────────  ─────────────────────────────────────────────────────
 *   sawHurt    W.sawAct ≠ null (guards.ts witnessKill: the victim in his open view as he dropped, or    he watched Sam shoot / drop / choke a colleague
 *              his eyes on Sam as the shot went) · or W.sawHeld ≠ null (reserved: a colleague held)
 *   saw        gd.lastSeenT ≥ 0 (a REAL sighting, sightGain > 0.25: the licence to shoot) · or          he has had Sam plainly in view at least once
 *              W.alertedBy === 'sight' (glimpses that added up to CONTACT) · or gd.shots > 0 (fired at him)
 *   glimpsed   gd.sightT ≥ 0 (a faint glimpse, sightGain > 0.05, moved his fix — never a real sighting)  a shape in the dark, about your size
 *   heard      H.shots + H.kick + H.bang + H.small > 0 (sounds that reached him through his own hearing  he knows SOMEONE is in the building tonight,
 *              model and raised him) · or W.alertedBy ∈ {sound, radio, body} · or W.sawBody / W.called-  never had eyes on who
 *              ToBody (a man is down: found by him, or called in on the net) · or W.dazzled > 0
 *   none       nothing above                                                                             never perceived Sam or any sign of him
 */
export function knowledgeOf(g: Game, gd: Guard): Knowledge {
  const W = gd.witness, H = W.heard;
  if (W.sawAct || W.sawHeld) return 'sawHurt';
  if (gd.lastSeenT >= 0 || W.alertedBy === 'sight' || gd.shots > 0) return 'saw';
  if (gd.sightT >= 0) return 'glimpsed';
  if (H.shots + H.kick + H.bang + H.small > 0 || W.alertedBy === 'sound' || W.alertedBy === 'radio' || W.alertedBy === 'body' || W.sawBody || W.calledToBody || W.dazzled > 0) return 'heard';
  return 'none';
}
const K_RANK: Record<Knowledge, number> = { none: 0, heard: 1, glimpsed: 2, saw: 3, sawHurt: 4 };
export function knowledgeRank(k: Knowledge): number { return K_RANK[k]; }

/**
 * agitationOf — 0‥1, the sum of (clamped):
 *
 *   term            value                       source
 *   ──────────────  ──────────────────────────  ────────────────────────────────────────────────────────────────────────────────────────────────
 *   floor           0.22 × escalation (0/1/2)   guards.ts escalationOf: calm / heightened / lockdown (0 under the tour)
 *   episode         +0.15                       g.alarm.episode: somebody is alert or still searching right now
 *   ownAlert        +0.20                       he is 'alert' now, or went alert within the last 60 s (W.alertT)
 *   ownEdge         +0.10                       he is 'suspicious' / 'search' now (not alert)
 *   sawAct          +0.30                       W.sawAct: watched Sam put a man down
 *   sawBody         +0.20                       W.sawBody: walked onto a body himself
 *   calledToBody    +0.12                       W.calledToBody (and did not find one himself): heard a man called in
 *   shots           +0.15                       H.shots > 0: gunfire reached him (anyone's)
 *   bangs           +0.10                       H.kick + H.bang > 0: a door going in / a stun charge
 *   dazzled         +0.15                       W.dazzled > 0: took a stun charge in the open eyes
 *   missing         +0.10                       the net has asked after a man and got nothing (any x.missedAt ≥ 0 / missingRaised)
 *   blackout        +0.08                       g.blackout.active
 *   held            nervy +0.04 per 10 s held (cap +0.15); steady / hard −0.03 per 10 s (cap −0.12): they settle, he does not (heldSinceOf: Guard.held.since)
 *   gunToHead       +0.10                       held with the pistol at his head rather than an arm round the throat (Guard.held.variant 'gun')
 *   aborted         +0.20                       Sam started to choke / pistol-whip him and eased off (talk.abortT): for the rest of the hold, or ABORT_RATTLE_SECS once let go
 *   pressed         +0.03 per press past the third (cap +0.15): the exhausted pool gets rawer the longer you lean
 *   bias            nervy +0.15 · hard −0.10 · steady 0                                                          (TEMPERAMENT)
 */
export function agitationOf(g: Game, gd: Guard): number { return agitationTerms(g, gd).a; }
function agitationTerms(g: Game, gd: Guard): { a: number; why: string[] } {
  const W = gd.witness, H = W.heard, t = temperamentOf(gd), talk = gd.talk; const why: string[] = []; let a = 0;
  const add = (v: number, label: string) => { if (Math.abs(v) < 1e-6) return; a += v; if (Math.abs(v) >= 0.005) why.push(`${label} ${v > 0 ? '+' : ''}${v.toFixed(2)}`); };
  add(0.22 * escalationOf(g), 'floor');
  if (g.alarm.episode && !g.quietUtility) add(0.15, 'episode');
  if (gd.state === 'alert' || (W.alertT >= 0 && g.time - W.alertT < 60)) add(0.2, 'ownAlert'); else if (gd.state === 'suspicious' || gd.state === 'search') add(0.1, 'ownEdge');
  if (W.sawAct) add(0.3, 'sawAct');
  if (W.sawBody) add(0.2, 'sawBody'); else if (W.calledToBody) add(0.12, 'calledToBody');
  if (H.shots > 0) add(0.15, 'shots');
  if (H.kick + H.bang > 0) add(0.1, 'bangs');
  if (W.dazzled > 0) add(0.15, 'dazzled');
  if (missingKnownTo(g, gd)) add(0.1, 'missing');
  if (g.blackout.active) add(0.08, 'blackout');
  const hs = heldSinceOf(g, gd); const held = hs >= 0 ? Math.max(0, g.time - hs) : 0;
  if (held > 0) add(t === 'nervy' ? Math.min(0.15, 0.004 * held) : -Math.min(0.12, 0.003 * held), 'held');
  if (gd.held?.variant === 'gun') add(0.1, 'gunToHead');
  if (talk.abortT >= 0 && (gd.held || g.time - talk.abortT < ABORT_RATTLE_SECS)) add(0.2, 'aborted');
  if (talk.presses > 3) add(Math.min(0.15, 0.03 * (talk.presses - 3)), 'pressed');
  add(t === 'nervy' ? 0.15 : t === 'hard' ? -0.1 : 0, 'bias');
  return { a: Math.max(0, Math.min(1, a)), why };
}
export function temperamentOf(gd: Guard): Temperament { return TEMPERAMENT[gd.callsign] ?? 'steady'; }
/** the stages at which this temperament GIVES the useful thing: steady plainly at 3; nervy early, at 2, and whatever else he has at 3; hard only once rattled */
export function givesAt(t: Temperament, band: Band): StageKey[] { return t === 'nervy' ? [2, 3] : t === 'hard' ? (band === 'low' ? [] : [3]) : [3]; }

// ---------------------------------------------------------------- needs (fact predicates by name) and tokens
/** Fact predicates a line may require (all must hold). Evaluated once per exchange into DialogueCtx.facts. */
export const NEEDS = ['floorUp', 'lockdown', 'episode', 'heardSmall', 'heardShots', 'heardBang', 'bodyFound', 'bodyCalled', 'bodyKnown', 'missingKnown', 'blackout', 'heldLong', 'rollcallLive', 'dazzled', 'sawShot', 'sawDrop', 'alertNow', 'drewOnYou', 'partnerKnown'] as const;
export type Need = typeof NEEDS[number];
/** Template tokens, resolved at pick time (a line whose token comes back null is simply not eligible). Written {name} in a line; {Name} capitalises the value.
 *    partner      his pair in a lockdown (leader / follower), else the nearest colleague he does not know to be down — by callsign
 *    victim       the man he watched Sam put down (W.sawAct) / saw held
 *    body         the body he found, else the one he heard called in
 *    missing      the first man the net has asked after and not raised (missedAt / missingRaised)
 *    room         the room HE stands in now (roomName: the clear plans' bounds + corridor / lobby / cubicles / car park)
 *    sawWhere     the room Sam was in when this man last had him in view, even faintly (gd.sightPos)
 *    heardWhere   the room the last small noise seemed to come from (H.smallWhere: the doorway it spilled through if not direct)
 *    route        his own patrol, prettified ('the corridor');  partnerRoute  {partner}'s
 *    post         the corridor junction somebody holds in this lockdown ('the east end'), else his own route
 *    door         a keyed door on his route (a locked LevelDoorDef within DOOR_ROUTE_M of his route's line) — null for the cubicle man
 *    caller       who is running the radio check right now (rollcallState.caller)
 *    checker / checked   the man walking a silent colleague's route right now (task checkOn), and whose route it is
 *    clearRoom    the room the lockdown pair is clearing / walking to now (its plan's door if the plan names no room)
 *    breakerRoom  the room the mains breaker is in (level.breaker) — where a man gets sent when the lights go
 *    checkEta     coarse time until the next radio check is due: 'any second now' / 'in a minute or so' / 'in a couple of minutes' / 'in N minutes'
 *    minutes      whole minutes since the last radio check was called ('one', 'two', …) — null before the first
 *    n            how many of them are on tonight, as a word */
export const TOKENS = ['partner', 'victim', 'body', 'missing', 'room', 'sawWhere', 'heardWhere', 'route', 'partnerRoute', 'post', 'door', 'caller', 'checker', 'checked', 'clearRoom', 'breakerRoom', 'checkEta', 'minutes', 'n'] as const;
export type Token = typeof TOKENS[number];
/** the longest value each token can take (lintDialogue expands lines with these to check MAX_CHARS); callsigns: 'Lindqvist'; clearRoom's worst is a door name */
const TOKEN_WORST: Record<Token, string> = {
  partner: 'Lindqvist', victim: 'Lindqvist', body: 'Lindqvist', missing: 'Lindqvist', caller: 'Lindqvist', checker: 'Lindqvist', checked: 'Lindqvist',
  room: "the manager's office", sawWhere: "the manager's office", heardWhere: "the manager's office", breakerRoom: "the manager's office", clearRoom: 'the break room side door',
  route: 'the lobby loop', partnerRoute: 'the lobby loop', post: 'the cubicle door', door: 'the server room door',
  checkEta: 'in a couple of minutes', minutes: 'fifteen', n: 'three',
};
/** tokens that always resolve for a living man in a level (coverage lint counts only lines whose tokens are in here or guaranteed by the cell's K) */
const TOKENS_ALWAYS: Token[] = ['room', 'route', 'post', 'breakerRoom', 'checkEta', 'n'];
/** …and the ones a knowledge level guarantees: a glimpse leaves sightPos, a sighting at least the fix or his own feet (gather's fallback); sawHurt names a victim (but may never have had Sam himself in view) */
const TOKENS_BY_K: Record<Knowledge, Token[]> = { none: [], heard: [], glimpsed: ['sawWhere'], saw: ['sawWhere'], sawHurt: ['victim'] };

// ---------------------------------------------------------------- the line table — DRAFT PLACEHOLDERS (see the header)
export interface Line {
  id: string;
  k: Knowledge | '*';          // knowledge cell ('*' = floor colour: any K, gated by needs) — the one axis the picker never widens
  a?: Band | 'mid+';           // agitation band (omitted = any; 'mid+' = mid or high)
  t?: Temperament;             // temperament (omitted = any)
  s: StageKey;                 // 1 reaction · 2 pressure · 3 nothing-left / refusal (INTEL takes the giving stages first) · 'x' exhausted
  text: string; needs?: Need[]; w?: number;
}
export const LINES: Line[] = [
  // ---- none: never knew you were there — startled, knows nothing (anchor case 1)
  { id: 'n1s', k: 'none', t: 'steady', s: 1, text: "Easy— easy. I'm not moving. Hands are where you can see them." },
  { id: 'n1n', k: 'none', t: 'nervy', s: 1, text: "Wh— who— okay. Okay okay okay. Don't. Please don't." },
  { id: 'n1h', k: 'none', t: 'hard', s: 1, text: "…That's a bad idea, friend. Mostly for you." },
  { id: 'n1a', k: 'none', s: 1, text: 'Where did you— there was nobody there. I checked. There was nobody.' },
  { id: 'n1f', k: 'none', a: 'mid+', s: 1, needs: ['floorUp'], text: "So it's real. They said weapons out and I thought someone sat on his radio." },
  { id: 'n2s', k: 'none', t: 'steady', s: 2, text: "I don't know you. I walk {route} and I sign the sheet. That's the whole job." },
  { id: 'n2n', k: 'none', t: 'nervy', s: 2, text: "I haven't seen your face. I don't want to. I'm looking at the floor— see?" },
  { id: 'n2h', k: 'none', t: 'hard', s: 2, text: 'You want the tour, reception opens at eight.' },
  { id: 'n2a', k: 'none', s: 2, text: "Whatever you're here for, it isn't me. I'm nights. I'm nobody." },
  { id: 'n3a', k: 'none', s: 3, text: "That's it. That's all there is. I do nights and I go home." },
  { id: 'n3n', k: 'none', t: 'nervy', s: 3, text: "I don't KNOW anything— I fill the machine and I walk. That's me. That's all of me." },
  { id: 'n3h', k: 'none', t: 'hard', a: 'low', s: 3, w: 2, text: 'Nothing. You get nothing, and you knew that when you grabbed me.' },
  { id: 'nxa', k: 'none', s: 'x', text: 'I told you. I just do nights.' },
  { id: 'nxb', k: 'none', s: 'x', text: "Same as before. I don't know. I still don't know." },
  { id: 'nxn', k: 'none', t: 'nervy', s: 'x', text: 'Please. My arm. I answered you.' },
  { id: 'nxh', k: 'none', t: 'hard', s: 'x', text: 'Ask again. Same answer.' },
  // ---- heard: knows someone is in the building, never saw who
  { id: 'h1s', k: 'heard', t: 'steady', s: 1, needs: ['heardSmall'], text: 'So that was you, by {heardWhere}. I wrote it off as the building settling.' },
  { id: 'h1n', k: 'heard', t: 'nervy', s: 1, text: 'I knew it. I KNEW I heard something. Nobody ever believes me.' },
  { id: 'h1h', k: 'heard', t: 'hard', s: 1, text: "Should've trusted my ears and come looking for you." },
  { id: 'h1a', k: 'heard', s: 1, text: "Something's been off all night. So it's a someone. Fine." },
  { id: 'h1g', k: 'heard', s: 1, needs: ['heardShots'], w: 2, text: "Those were shots. That was you? …Who's down. Tell me who's down." },
  { id: 'h1b', k: 'heard', s: 1, needs: ['bodyKnown'], w: 2, text: "They found {body}. That was you. And now it's my turn, is it." },
  { id: 'h2s', k: 'heard', t: 'steady', s: 2, needs: ['partnerKnown'], text: '{Partner} is going to love this. I told him it was the vending machine.' },
  { id: 'h2n', k: 'heard', t: 'nervy', s: 2, text: "I called it in as nothing. I SAID nothing. They'll put that on me." },
  { id: 'h2h', k: 'heard', t: 'hard', s: 2, text: "You're loud, whoever you are. Loud gets found." },
  { id: 'h2a', k: 'heard', s: 2, text: "I never saw you. Noises, that's all — and I'm not paid to chase noises." },
  { id: 'h3a', k: 'heard', s: 3, text: "I heard things. I didn't see things. That's the truth and it's all of it." },
  { id: 'h3h', k: 'heard', t: 'hard', a: 'low', s: 3, w: 2, text: "You've had your answer. It was no." },
  { id: 'hxa', k: 'heard', s: 'x', text: 'Noises. All I ever had was noises.' },
  { id: 'hxb', k: 'heard', s: 'x', text: "I've said it. Ears, not eyes." },
  // ---- glimpsed: a shape in the dark
  { id: 'g1s', k: 'glimpsed', t: 'steady', s: 1, text: 'Something moved, over in {sawWhere}. I told myself it was the light.' },
  { id: 'g1n', k: 'glimpsed', t: 'nervy', s: 1, text: 'You were right there. I looked RIGHT at you. Oh no. No no no.' },
  { id: 'g1h', k: 'glimpsed', t: 'hard', s: 1, text: 'Next time the torch stays on you.' },
  { id: 'g1a', k: 'glimpsed', s: 1, text: "I saw— I don't know what I saw. You, I suppose. In {sawWhere}." },
  { id: 'g2a', k: 'glimpsed', s: 2, text: "A shape. That's what goes in the book. A shape, about your size." },
  { id: 'g2n', k: 'glimpsed', t: 'nervy', a: 'mid+', s: 2, text: 'I nearly called it. Thumb on the button. I nearly— why didn\'t I.' },
  { id: 'g2h', k: 'glimpsed', t: 'hard', s: 2, text: 'Half a second sooner with the light and we\'d be talking the other way round.' },
  { id: 'g3a', k: 'glimpsed', s: 3, text: "Half a second of you in the dark. That's everything I've got." },
  { id: 'g3h', k: 'glimpsed', t: 'hard', a: 'low', s: 3, w: 2, text: 'No. Work it out yourself.' },
  { id: 'gxa', k: 'glimpsed', s: 'x', text: 'A shape. I keep telling you. A shape.' },
  { id: 'gxb', k: 'glimpsed', s: 'x', text: "That's twice you've asked and twice it's the same." },
  // ---- saw: had you plainly / hunted you
  { id: 's1s', k: 'saw', t: 'steady', s: 1, text: 'Everyone on this floor is looking for you. You do know that.' },
  { id: 's1n', k: 'saw', t: 'nervy', s: 1, text: "It's you. From {sawWhere}. Okay. Okay— I'm not doing anything. Look. Nothing." },
  { id: 's1p', k: 'saw', t: 'nervy', a: 'high', s: 1, w: 2, text: "They're coming— all of them— you heard the radio same as me—" },
  { id: 's1h', k: 'saw', t: 'hard', s: 1, text: "You. From {sawWhere}. You don't walk out of this building." },
  { id: 's1a', k: 'saw', s: 1, text: 'I had you. In {sawWhere}. I had you and I lost you.' },
  { id: 's1d', k: 'saw', s: 1, needs: ['drewOnYou'], w: 2, text: 'I put rounds your way and you still got behind me. …Fine. Fine.' },
  { id: 's2s', k: 'saw', t: 'steady', s: 2, text: 'Let go now and I count to sixty before I touch the radio. That offer is real.' },
  { id: 's2n', k: 'saw', t: 'nervy', s: 2, text: "What do you want— just say it— say it and I'll say yes—" },
  { id: 's2h', k: 'saw', t: 'hard', s: 2, text: "Squeeze all you like. I've had worse off my brother." },
  { id: 's2a', k: 'saw', s: 2, text: "You've got maybe a minute before someone walks round that corner. Use it." },
  { id: 's3a', k: 'saw', s: 3, text: "That's everything. Do what you're going to do." },
  { id: 's3h', k: 'saw', t: 'hard', a: 'low', s: 3, w: 2, text: 'From me? Nothing. Not a word.' },
  { id: 'sxa', k: 'saw', s: 'x', text: "I've said it all. Twice." },
  { id: 'sxb', k: 'saw', s: 'x', text: 'Still here. Still nothing new.' },
  // ---- sawHurt: watched you shoot / drop a colleague — agitated, hostile, names it (anchor case 2)
  { id: 'k1s', k: 'sawHurt', t: 'steady', s: 1, needs: ['sawShot'], text: "You shot {victim}. I watched you do it. Don't talk to me like we're talking." },
  { id: 'k1n', k: 'sawHurt', t: 'nervy', s: 1, needs: ['sawShot'], text: "{Victim}— is he— you didn't have to— he'd have let you walk past—" },
  { id: 'k1h', k: 'sawHurt', t: 'hard', s: 1, needs: ['sawShot'], text: '{Victim} had a name. Remember it. Somebody is going to ask you for it.' },
  { id: 'k1a', k: 'sawHurt', s: 1, needs: ['sawShot'], text: 'I saw you shoot {victim}. I saw it. Whatever you say next, I saw it.' },
  { id: 'k1d', k: 'sawHurt', s: 1, needs: ['sawDrop'], text: "Is {victim} breathing? …Tell me he's breathing." },
  { id: 'k1e', k: 'sawHurt', t: 'hard', s: 1, needs: ['sawDrop'], text: 'I saw what you did to {victim}. Try it from the front some time.' },
  { id: 'k1g', k: 'sawHurt', s: 1, w: 0.6, text: 'I saw what you did to {victim}. Get your arm off me.' },   // (the one s1 line that names no particular deed: covers the reserved 'grab' / sawHeld kinds; weighted under the specific ones)
  { id: 'k2s', k: 'sawHurt', t: 'steady', s: 2, text: "What do you want from me? You've shown me what you are." },
  { id: 'k2n', k: 'sawHurt', t: 'nervy', s: 2, text: 'Okay! Okay. Anything. Just— not like {victim}. Please. Not like that.' },
  { id: 'k2h', k: 'sawHurt', t: 'hard', s: 2, needs: ['sawShot'], text: "Every man on this floor heard that. They're not coming to talk." },
  { id: 'k2i', k: 'sawHurt', t: 'hard', s: 2, text: 'You put {victim} down in front of me and now you want a chat. No.' },
  { id: 'k2a', k: 'sawHurt', s: 2, text: '{Victim} first, then me — that the plan? Then get on with it.' },
  { id: 'k3a', k: 'sawHurt', s: 3, text: "You've had all I've got. The rest you took off {victim}." },
  { id: 'k3h', k: 'sawHurt', t: 'hard', s: 3, w: 2, text: 'No.' },
  { id: 'kxa', k: 'sawHurt', t: 'nervy', s: 'x', text: 'Please. I answered. Please.' },
  { id: 'kxb', k: 'sawHurt', s: 'x', text: "Nothing else. There's nothing else. Ask {victim}— oh. Right." },
  { id: 'kxs', k: 'sawHurt', s: 'x', text: "I've nothing left to tell you and no reason left to lie to you." },
  { id: 'kxh', k: 'sawHurt', t: 'hard', s: 'x', text: 'Still no.' },
  // ---- floor colour: any knowledge, the needs do the gating
  { id: 'f2m', k: '*', t: 'steady', s: 2, needs: ['missingKnown'], w: 1.2, text: "{Missing} hasn't answered his radio. That was you as well, wasn't it." },
  { id: 'f2l', k: '*', t: 'hard', s: 2, needs: ['lockdown'], text: "Hear that? Pairs and doors. You're in a box now." },
  { id: 'f2k', k: '*', s: 2, needs: ['lockdown'], text: "It's locked down. Nobody walks alone and every door gets pulled to. Good luck." },
  { id: 'f2b', k: '*', t: 'hard', s: 2, needs: ['bodyCalled'], text: "I heard them call {body} in. I'm telling you nothing." },
  { id: 'f2c', k: '*', t: 'steady', s: 2, needs: ['bodyCalled'], text: "After {body} they said weapons out. Should've listened harder." },
  { id: 'f2f', k: '*', s: 2, needs: ['bodyFound'], text: 'I stood over {body} not long ago. I know exactly what you are.' },
  { id: 'f2o', k: '*', s: 2, needs: ['blackout'], text: 'The lights too? …Course it was you.' },
  { id: 'f2d', k: '*', t: 'nervy', s: 2, needs: ['dazzled'], text: "I still can't see straight. That flash— that was you too, wasn't it—" },
  { id: 'f2r', k: '*', s: 2, needs: ['rollcallLive'], w: 2, text: "That's the check going now. I don't answer it, they come down {route}." },
  { id: 'fxl', k: '*', t: 'steady', s: 'x', needs: ['heldLong'], text: "My arm's gone dead. Whatever this is, finish it." },
  { id: 'fxn', k: '*', t: 'nervy', s: 'x', needs: ['heldLong'], text: "I can't feel my hand. Please. I've been good. I've answered everything." },
  { id: 'fxe', k: '*', a: 'high', s: 'x', text: "You're running out of floor. You know that. I can hear it in you." },
];

// ---------------------------------------------------------------- INTEL: the one useful thing, gated by what he plausibly knows — DRAFT PLACEHOLDER texts too
/** Kinds of useful thing, in the generator's order of preference: what is happening NOW that he can know first, then the standing facts. A future mission hook
 *  reads DialoguePick.kind. Never: anything about Sam he did not perceive, door codes (there are none), the drive before it is pulled. */
export const INTEL_KINDS = ['checkOn', 'clearing', 'rollcallLive', 'blackout', 'door', 'rollcall', 'lockdownPosts', 'routes', 'missing', 'drive'] as const;
export type IntelKind = typeof INTEL_KINDS[number];
export interface IntelItem { kind: IntelKind; text: string; known: boolean; live: boolean; }
/** the texts, by kind and by who says them: 't:k' (temperament × knowledge) → 't' → '*:k' → '*' — plain facts; intelLead() puts the grudging '…Fine. ' / '…All right. '
 *  in front where the man would not volunteer it, and a knowledge-keyed variant lets the writing colour a fact by what he saw (the nervy witness ties the check to the victim) */
type IntelKey = Temperament | '*' | `${Temperament}:${Knowledge}` | `*:${Knowledge}`;
const INTEL_TEXT: Record<IntelKind, Partial<Record<IntelKey, string>>> = {
  checkOn:       { '*': "{Checker}'s walking {checked}'s route — he went quiet. He'll come past here.", nervy: "{Checker}'s out looking for {checked}— he went quiet— he'll walk right into us—" },
  clearing:      { '*': "They're clearing rooms off your last position. {ClearRoom} first." },
  rollcallLive:  { '*': "That's a check now. No answer from a man and {caller} sends someone round." },
  blackout:      { '*': 'Lights die, one of us goes to the breaker in {breakerRoom} — armed.' },
  door:          { '*': "{Door} is on a key. We all carry one. Mine's on my belt.", hard: '{Door} is keyed. We all carry one. Take mine and get out.' },
  rollcall:      { '*': 'Radio check {checkEta}. Miss it, they send a man down your route.', nervy: "A check— {checkEta}— I don't answer, somebody comes looking. That's all.", 'nervy:sawHurt': "The check's {checkEta}— {victim} won't answer it— then they ALL come—" },
  lockdownPosts: { '*': "We're paired now. One man holds {post}. Nobody walks alone." },
  routes:        { '*': "{N} of us tonight. I've got {route}, {partner} has {partnerRoute}.", nervy: "It's {n} of us— me on {route}, {partner} on {partnerRoute}— that's all I've got—" },
  missing:       { '*': 'Nobody can raise {missing}. His route turns up empty, this all locks down.' },
  drive:         { '*': "Your rack dropped off the board. Somebody's already been sent to look at it." },
};
/** the text a man of temperament t and knowledge k gives for `kind` (the most specific variant written) */
export function intelText(kind: IntelKind, t: Temperament, k: Knowledge): string { const T = INTEL_TEXT[kind]; return T[`${t}:${k}`] ?? T[t] ?? T[`*:${k}`] ?? T['*'] ?? ''; }
/** the checker's own words when HE is the man walking the silent colleague's route (gather swaps it in) */
const CHECKON_SELF = "I was sent round {checked}'s route — he'd gone quiet. Now I know why.";
/** what goes in front of a given fact: a hard man never volunteers ('…Fine. ' — and only once rattled, givesAt); a steady one who has SEEN what Sam is gives it
 *  through his teeth ('…All right. '); a nervy one just spills it */
export function intelLead(t: Temperament, k: Knowledge): string { return t === 'hard' ? '…Fine. ' : t === 'steady' && (k === 'saw' || k === 'sawHurt') ? '…All right. ' : ''; }

// ---------------------------------------------------------------- names: rooms, routes, doors, numbers
type Rect = { name: string; x0: number; x1: number; z0: number; z1: number };
const roomRects = new WeakMap<Level, { rooms: Rect[]; building: Rect }>();
/** the office level's open areas, which no clear plan bounds (level.ts: corridor z 10‥12.2, lobby x 4‥12, cubicle farm x 12‥28, break room x 28‥36, all z 12.2‥24; walls x 4‥36, z 4‥24) — used only when the level IS that office (its three route names); any other level gets its clear-plan rooms plus inside / outside */
const OFFICE_AREAS: Rect[] = [{ name: 'the corridor', x0: 4, x1: 36, z0: 10, z1: 12.2 }, { name: 'the lobby', x0: 4, x1: 12, z0: 12.2, z1: 24 }, { name: 'the cubicles', x0: 12, x1: 28, z0: 12.2, z1: 24 }, { name: 'the break room', x0: 28, x1: 36, z0: 12.2, z1: 24 }];
const OFFICE_WALLS: Rect = { name: 'inside', x0: 4, x1: 36, z0: 4, z1: 24 };
/** The room a floor point is in, as the men would say it: the clear plans' rooms (level.ts bounds) and the mission's server room, then — on the office level — the
 *  corridor, the lobby, the cubicle floor, the break room; anywhere else inside the walls 'the back offices', outside them 'the car park'. (The design's roomName
 *  helper, kept here rather than in level.ts, which this slice does not touch; another level gets its own clear-plan rooms and a footprint from its routes.) */
export function roomName(level: Level, p: Vec3): string {
  let R = roomRects.get(level);
  if (!R) {
    const rooms: Rect[] = [];
    for (const cp of level.clearPlans ?? []) if (cp.bounds && cp.room && !rooms.some(r => r.name.endsWith(cp.room!))) rooms.push({ name: /^the /.test(cp.room) ? cp.room : `the ${cp.room}`, ...cp.bounds });
    const S = level.mission.serverRoom; if (!rooms.some(r => /server/.test(r.name))) rooms.push({ name: 'the server room', ...S });
    const office = ['corridor', 'cubicles', 'lobby_break'].every(n => level.routes.some(r => r.name === n));
    let building = OFFICE_WALLS;
    if (office) rooms.push(...OFFICE_AREAS);
    else { building = { name: 'inside', x0: Infinity, x1: -Infinity, z0: Infinity, z1: -Infinity }; for (const r of level.routes) for (const q of r.points) { building.x0 = Math.min(building.x0, q[0] - 3); building.x1 = Math.max(building.x1, q[0] + 3); building.z0 = Math.min(building.z0, q[2] - 3); building.z1 = Math.max(building.z1, q[2] + 3); } }   // (the men patrol indoors: their routes' extent, padded, stands in for the walls)
    R = { rooms, building }; roomRects.set(level, R);
  }
  for (const r of R.rooms) if (p[0] >= r.x0 && p[0] <= r.x1 && p[2] >= r.z0 && p[2] <= r.z1) return r.name;
  const B = R.building; if (p[0] >= B.x0 && p[0] <= B.x1 && p[2] >= B.z0 && p[2] <= B.z1) return 'the back offices';   // inside the walls but in no named rectangle (a partition's thickness): near enough
  return 'the car park';
}
const ROUTE_NAMES: Record<string, string> = { corridor: 'the corridor', cubicles: 'the cubicles', lobby_break: 'the lobby loop' };
export function routeName(level: Level, routeI: number): string { const n = level.routes[routeI]?.name ?? 'route'; return ROUTE_NAMES[n] ?? `the ${n.replace(/_/g, ' ')}`; }
const DOOR_NAMES: Record<string, string> = { server: 'the server room door', manager: "the manager's door", storage: 'the storage door', conference: 'the conference door', breakroom_n: 'the break room door', breakroom_w: 'the break room side door', breakroom_ext: 'the outside door', fire_exit: 'the fire exit', server_manager: 'the office side door' };
function doorName(n: string): string { return DOOR_NAMES[n] ?? `the ${n.replace(/_/g, ' ')} door`; }
const NUM_WORDS = ['none', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine', 'ten', 'eleven', 'twelve', 'thirteen', 'fourteen', 'fifteen'];
function numWord(n: number): string { return NUM_WORDS[n] ?? String(n); }
/** metres within which a keyed door's frame counts as ON a route's line (server / manager / storage stand ~1.1 m off the corridor line; the cubicle loop never comes within 3 m of one) */
const DOOR_ROUTE_M = 2.5;
function distToRoute(pts: Vec3[], p: Vec3): number {
  let best = Infinity;
  for (let i = 0; i < pts.length; i++) {
    const a = pts[i], b = pts[(i + 1) % pts.length]; const abx = b[0] - a[0], abz = b[2] - a[2]; const L2 = abx * abx + abz * abz;
    const t = L2 > 1e-6 ? Math.max(0, Math.min(1, ((p[0] - a[0]) * abx + (p[2] - a[2]) * abz) / L2)) : 0;
    best = Math.min(best, Math.hypot(a[0] + abx * t - p[0], a[2] + abz * t - p[2]));
  }
  return best;
}
/** the keyed door nearest this man's route line, within DOOR_ROUTE_M — by its authored lock (a keep kicked out since is still 'the keyed door' to him) */
export function keyedDoorOnRoute(g: Game, gd: Guard): string | null {
  const pts = g.level.routes[gd.routeI]?.points; if (!pts?.length) return null;
  let best: string | null = null, bd = DOOR_ROUTE_M;
  for (const d of g.doors.list) { if (!d.def.locked) continue; const dd = distToRoute(pts, d.frameCentre); if (dd < bd) { bd = dd; best = d.def.name; } }
  return best ? doorName(best) : null;
}
/** coarse words for the seconds until the next radio check (deliberately vague: he knows the rhythm, not the clock) */
export function etaWords(secs: number): string { return secs <= 20 ? 'any second now' : secs <= 75 ? 'in a minute or so' : secs <= 150 ? 'in a couple of minutes' : secs <= 900 ? `in ${numWord(Math.round(secs / 60))} minutes` : 'before long'; }   // (past a quarter of an hour nobody on nights is counting: 'before long')

// ---------------------------------------------------------------- the context: everything the picker needs, read off the game once per exchange
export interface DialogueCtx {
  who: string; k: Knowledge; a: number; band: Band; t: Temperament;
  why: string[];                                   // the agitation terms, for the panel / probes
  facts: Partial<Record<Need, boolean>>;
  tokens: Partial<Record<Token, string | null>>;
  intel: IntelItem[];                              // in INTEL_KINDS order, known / live evaluated
  time: number;
  recent: Map<string, number> | null;              // line id → game time anybody last said it (floor-wide staleness), null for synthetic contexts
}
const recentByGame = new WeakMap<object, Map<string, number>>();
/** seconds within which a line another man said counts as stale for everybody (× RECENT_W) */
const RECENT_SECS = 90, RECENT_W = 0.25;

/** does a fact stamped on the net at game time T reach a man held since `heldSince` (heldSinceOf; -1 = free): yes if he was not held then (a forearm across the throat is not when you follow the chatter in your ear) */
function heardOnNet(heldSince: number, T: number): boolean { return T >= 0 && (heldSince < 0 || T <= heldSince + 0.5); }
/** the first colleague the net has asked after by name and not raised (guards.ts updateRollcall / partnerWatch stamp missedAt; the route walk sets missingRaised) — as far as THIS man followed it */
function missingKnownTo(g: Game, gd: Guard): Guard | null { const hs = heldSinceOf(g, gd); return g.guards.find(x => x !== gd && (x.missedAt >= 0 || x.missingRaised) && heardOnNet(hs, x.missedAt >= 0 ? x.missedAt : 0)) ?? null; }
/** the men this man KNOWS to be down (he watched it, found him, heard him called in, or heard the net report him off his route) — everyone else is, to him, at his post */
function knownDown(g: Game, gd: Guard): Set<string> {
  const W = gd.witness; const s = new Set<string>();
  if (W.sawAct) s.add(W.sawAct.victim); if (W.sawBody) s.add(W.sawBody.victim); if (W.calledToBody) s.add(W.calledToBody.victim); if (W.sawHeld) s.add(W.sawHeld.who);
  for (const x of g.guards) if (x !== gd && x.missingRaised) s.add(x.callsign);
  return s;
}
function partnerOf(g: Game, gd: Guard): Guard | null {
  const down = knownDown(g, gd);   // dead or alive to the world, a man he does not KNOW to be down is, as far as he is concerned, still on his route — and one he watched drop is nobody's partner any more
  if (gd.leader && gd.leader !== gd && !down.has(gd.leader.callsign)) return gd.leader;
  const follower = g.guards.find(x => x !== gd && x.leader === gd && !down.has(x.callsign)); if (follower) return follower;
  let best: Guard | null = null, bd = Infinity;
  for (const x of g.guards) { if (x === gd || down.has(x.callsign)) continue; const d = v3.distXZ(x.char.pos, gd.char.pos); if (d < bd) { bd = d; best = x; } }
  return best;
}
/** since when this man counts as HELD for the engine's purposes: the hold's own clock while Sam has him (Guard.held.since — the grab owns it, presses never move it),
 *  else the sandbox stand-in for a man nobody holds — talk.heldSince while that exchange is live (the last press within HOLD_LAPSE_SECS, or none yet since the
 *  stamp) — else -1 */
export function heldSinceOf(g: Game, gd: Guard): number {
  if (gd.held) return gd.held.since;
  const talk = gd.talk; return talk.heldSince >= 0 && (talk.lastT < talk.heldSince || g.time - talk.lastT <= HOLD_LAPSE_SECS) ? talk.heldSince : -1;
}

/** Read the game into a DialogueCtx for this man (the only function past the axes that touches Game). */
export function gather(g: Game, gd: Guard): DialogueCtx {
  const W = gd.witness, H = W.heard, talk = gd.talk, L = g.level, R = rollcallState(g), lvl = escalationOf(g);
  const k = knowledgeOf(g, gd); const { a, why } = agitationTerms(g, gd); const band = bandOf(a); const t = temperamentOf(gd);
  const hs = heldSinceOf(g, gd); const held = hs >= 0 ? g.time - hs : 0;
  // tokens
  const partner = partnerOf(g, gd);
  const missingMan = missingKnownTo(g, gd);
  const checker = g.guards.find(x => x.state !== 'dead' && x.task?.kind === 'checkOn') ?? null;
  const checkTask = checker?.task?.kind === 'checkOn' ? checker.task : null;
  const postMan = g.guards.find(x => x.state !== 'dead' && x.post) ?? null;
  const nextAt = R.nextAt >= 0 ? R.nextAt : g.time + (lvl >= 1 ? 45 : 90);
  const tokens: Partial<Record<Token, string | null>> = {
    partner: partner?.callsign ?? null,
    victim: W.sawAct?.victim ?? W.sawHeld?.who ?? null,
    body: W.sawBody?.victim ?? W.calledToBody?.victim ?? null,
    missing: missingMan?.callsign ?? null,
    room: roomName(L, gd.char.pos),
    sawWhere: gd.sightT >= 0 ? roomName(L, gd.sightPos) : gd.lastSeenT >= 0 || W.alertedBy === 'sight' ? roomName(L, gd.lastKnown ?? gd.char.pos) : null,   // (a scripted man's sighting or a bump — squad.ts perceive — stamps lastSeenT and the fix without sightPos: where he had him is the fix, else where he stands)
    heardWhere: H.small > 0 && H.smallWhere ? roomName(L, H.smallWhere) : null,
    breakerRoom: roomName(L, L.breaker.pos),
    route: routeName(L, gd.routeI),
    partnerRoute: partner ? routeName(L, partner.routeI) : null,
    post: postMan?.post ? `the ${postMan.post.name}` : routeName(L, gd.routeI),
    door: keyedDoorOnRoute(g, gd),
    caller: R.caller && R.caller !== gd ? R.caller.callsign : null,   // (himself calling it while held cannot happen: a held man is off the caller rota once the grab lands)
    checker: checker && checker !== gd ? checker.callsign : null,      // (the checker himself gets CHECKON_SELF, which names only {checked})
    checked: checkTask ? checkTask.who.callsign : null,
    clearRoom: g.clearing && g.clearing.stage !== 'done' && g.clearing.cur ? (g.clearing.cur.room ? (/^the /.test(g.clearing.cur.room) ? g.clearing.cur.room : `the ${g.clearing.cur.room}`) : doorName(g.clearing.cur.door)) : null,
    checkEta: R.stage !== 'idle' ? 'any second now' : etaWords(nextAt - g.time),
    minutes: R.checks > 0 ? numWord(Math.max(1, Math.round((g.time - R.t0) / 60))) : null,
    n: numWord(g.guards.length),
  };
  // facts
  const facts: Partial<Record<Need, boolean>> = {
    floorUp: lvl >= 1, lockdown: lvl >= 2, episode: g.alarm.episode && !g.quietUtility,
    heardSmall: H.small > 0 && !!H.smallWhere, heardShots: H.shots > 0, heardBang: H.kick + H.bang > 0,
    bodyFound: !!W.sawBody, bodyCalled: !!W.calledToBody && !W.sawBody, bodyKnown: !!(W.sawBody || W.calledToBody),
    missingKnown: !!missingMan, blackout: g.blackout.active, heldLong: held >= HELD_LONG_SECS,
    rollcallLive: R.stage !== 'idle', dazzled: W.dazzled > 0,
    sawShot: W.sawAct?.kind === 'shot', sawDrop: !!W.sawAct && W.sawAct.kind !== 'shot' && W.sawAct.kind !== 'grab',
    alertNow: gd.state === 'alert', drewOnYou: gd.shots > 0, partnerKnown: !!partner,
  };
  // intel: live = true of the world now; known = he could plausibly know it (routine, or it went over the net while he was free to listen)
  const txt = (kind: IntelKind) => intelText(kind, t, k);
  const intel: IntelItem[] = [
    { kind: 'checkOn', text: checker === gd ? CHECKON_SELF : txt('checkOn'), live: !!checkTask && checkTask.who !== gd, known: !!checkTask && (checker === gd || heardOnNet(hs, checkTask.t0)) },
    { kind: 'clearing', text: txt('clearing'), live: lvl >= 2 && !!g.clearing && g.clearing.stage !== 'done' && g.clearing.a !== gd && g.clearing.b !== gd, known: heardOnNet(hs, g.alarm.raisedAt) },   // (the order that dealt the clear went out with the raise: guards.ts raiseEscalation → planClears)
    { kind: 'rollcallLive', text: txt('rollcallLive'), live: R.stage !== 'idle' && !!R.caller && R.caller !== gd, known: true },
    { kind: 'blackout', text: txt('blackout'), live: g.blackout.active, known: true },
    { kind: 'door', text: txt('door'), live: !!tokens.door, known: true },
    { kind: 'rollcall', text: txt('rollcall'), live: !g.quietUtility && g.aiEnabled, known: true },   // (no checks run under the tour or with the AI off: then it is not true that one is due)
    { kind: 'lockdownPosts', text: txt('lockdownPosts'), live: lvl >= 2 && !!postMan, known: heardOnNet(hs, g.alarm.raisedAt) || postMan === gd },
    { kind: 'routes', text: txt('routes'), live: !!partner, known: true },
    { kind: 'missing', text: txt('missing'), live: !!missingMan && lvl < 2, known: !!missingMan },
    { kind: 'drive', text: txt('drive'), live: g.mission.pulled && g.mission.sweepAt < 0, known: true },   // (the rack check went out over the radio — mission.ts sendRackCheck — but Mission keeps no time for it, so this one net fact is not hold-gated: flavour, last in line; a Mission.sweptAt would gate it like the errand)
  ];
  let recent = recentByGame.get(g); if (!recent) { recent = new Map(); recentByGame.set(g, recent); }
  return { who: gd.callsign, k, a, band, t, why, facts, tokens, intel, time: g.time, recent };
}

// ---------------------------------------------------------------- the picker (pure over a DialogueCtx + the man's Talk state)
export interface DialoguePick {
  text: string; id: string;                        // the resolved line and its id ('intel:<kind>' for a generated one)
  kind: IntelKind | null;                          // the useful thing given, if this exchange gave one (a mission hook may consume it)
  stage: StageKey; stageN: number;                 // the stage this exchange was (1‥3, then 'x'; stageN counts on: 4, 5, …)
  cell: string; k: Knowledge; band: Band; t: Temperament;   // the cell it was picked for
  lineK: Knowledge | '*';                          // the picked line's own knowledge key (never above k — the QA soak asserts it)
  widened: '' | 't' | 'a' | 'repeat';              // how far the pool had to be widened to find anything ('' = exact cell)
}
const TOKEN_RE = /\{([A-Za-z]+)\}/g;
/** the tokens a text uses, as Token keys (lower-cased first letter) */
export function tokensIn(text: string): string[] { const out: string[] = []; for (const m of text.matchAll(TOKEN_RE)) out.push(m[1][0].toLowerCase() + m[1].slice(1)); return out; }
/** expand {tokens} from the table ({Name} capitalises); null if any comes back empty */
export function expand(text: string, tokens: Partial<Record<string, string | null>>): string | null {
  let ok = true;
  const out = text.replace(TOKEN_RE, (_m, raw: string) => { const key = raw[0].toLowerCase() + raw.slice(1); const v = tokens[key]; if (!v) { ok = false; return ''; } return raw[0] !== key[0] ? v[0].toUpperCase() + v.slice(1) : v; });
  return ok ? out : null;
}
function stageKeyOf(n: number): StageKey { return n <= 1 ? 1 : n === 2 ? 2 : n === 3 ? 3 : 'x'; }
function lineMatches(l: Line, k: Knowledge, band: Band, t: Temperament, s: StageKey, useT: boolean, useA: boolean): boolean {
  if (l.s !== s) return false;
  if (l.k !== '*' && l.k !== k) return false;
  if (useT && l.t && l.t !== t) return false;
  if (useA && l.a) { if (l.a === 'mid+') { if (band === 'low') return false; } else if (l.a !== band) return false; }
  return true;
}
function needsHold(l: Line, facts: Partial<Record<Need, boolean>>): boolean { return !l.needs || l.needs.every(n => !!facts[n]); }
/** The first useful thing that is true now, known to him, not yet given, and whose tokens resolve — or null. */
export function intelFor(ctx: DialogueCtx, talk: Talk): { kind: IntelKind; text: string } | null {
  for (const it of ctx.intel) {
    if (!it.live || !it.known || talk.given.has(it.kind)) continue;
    const text = expand(intelLead(ctx.t, ctx.k) + it.text, ctx.tokens); if (!text) continue;
    return { kind: it.kind, text };
  }
  return null;
}
/** One exchange: advance the man's stage, pick the line for (his cell × that stage), mark it said. `rnd` decides among the pool (seeded by the caller for
 *  replayable soaks / probes). Never returns a line keyed to a knowledge level other than his own; a dry cell widens temperament, then agitation, at the staged
 *  presses, and at 'nothing left' repeats his own brush-offs (least-said first) before borrowing anyone else's. */
export function pickLine(ctx: DialogueCtx, talk: Talk, rnd: () => number): DialoguePick {
  const stageN = talk.stage + 1; const s = stageKeyOf(stageN);
  talk.stage = stageN; talk.presses++; talk.lastT = ctx.time;
  const cell = `${ctx.k}·${ctx.band}·${ctx.t}`;
  const base = { stage: s, stageN, cell, k: ctx.k, band: ctx.band, t: ctx.t };
  // the giving stages: the INTEL generator first (a late press still gets the one useful thing if he never gave one and now would)
  const gives = givesAt(ctx.t, ctx.band);
  if (gives.includes(s) || (s === 'x' && talk.given.size === 0 && gives.length > 0 && stageN === 4)) {
    const it = intelFor(ctx, talk);
    if (it) { talk.given.add(it.kind); const id = `intel:${it.kind}`; talk.said.add(id); talk.last = { id, cell, stage: String(s), kind: it.kind }; return { ...base, text: it.text, id, kind: it.kind, lineK: ctx.k, widened: '' }; }
  }
  // the pool for this cell and stage: exact, else any temperament, else any agitation too — never another man's knowledge
  const eligible = (useT: boolean, useA: boolean, allowSaid: boolean) => {
    const out: { l: Line; text: string; w: number }[] = [];
    for (const l of LINES) {
      if (!lineMatches(l, ctx.k, ctx.band, ctx.t, s, useT, useA) || !needsHold(l, ctx.facts)) continue;
      if (!allowSaid && talk.said.has(l.id)) continue;
      const text = expand(l.text, ctx.tokens); if (!text) continue;
      let w = (l.w ?? 1) * (l.needs?.length ? 1.6 : 1) * (l.t === ctx.t ? 1.5 : 1) * (l.a ? 1.3 : 1) * (l.k !== '*' ? 1.15 : 1);   // specific beats generic
      const rt = ctx.recent?.get(l.id); if (rt !== undefined && ctx.time - rt < RECENT_SECS) w *= RECENT_W;
      if (allowSaid) w /= 1 + (talk.saidN.get(l.id) ?? 0) * 3;
      out.push({ l, text, w });
    }
    return out;
  };
  // dry-cell policy: at the staged presses widen temperament, then agitation, before ever repeating (lintDialogue guarantees those cells are never dry anyway); at
  // 'nothing left' repeat his OWN brush-offs (least-said first) before borrowing another temperament's voice or another band's heat
  const chain: [boolean, boolean, boolean, DialoguePick['widened']][] = s === 'x'
    ? [[true, true, false, ''], [true, true, true, 'repeat'], [false, true, true, 't'], [false, false, true, 'a']]
    : [[true, true, false, ''], [false, true, false, 't'], [false, false, false, 'a'], [false, false, true, 'repeat']];
  let widened: DialoguePick['widened'] = ''; let pool: { l: Line; text: string; w: number }[] = [];
  for (const [useT, useA, allowSaid, tag] of chain) {
    pool = eligible(useT, useA, allowSaid); widened = tag;
    if (allowSaid && pool.length > 1 && talk.last) { const fresh = pool.filter(p => p.l.id !== talk.last!.id); if (fresh.length) pool = fresh; }   // repeating, but never the very line he said last if there is any other
    if (pool.length) break;
  }
  if (!pool.length) {   // nothing at all for this K and stage (lintDialogue guarantees this cannot happen for the shipped table; a synthetic context with no tokens could get here)
    const text = '…'; talk.last = { id: '-', cell, stage: String(s), kind: null };
    return { ...base, text, id: '-', kind: null, lineK: ctx.k, widened: 'repeat' };
  }
  let total = 0; for (const p of pool) total += p.w;
  let r = rnd() * total; let chosen = pool[pool.length - 1];
  for (const p of pool) { r -= p.w; if (r <= 0) { chosen = p; break; } }
  const { l, text } = chosen;
  talk.said.add(l.id); talk.saidN.set(l.id, (talk.saidN.get(l.id) ?? 0) + 1); ctx.recent?.set(l.id, ctx.time);
  talk.last = { id: l.id, cell, stage: String(s), kind: null };
  return { ...base, text, id: l.id, kind: null, lineK: l.k, widened };
}

/** The whole exchange for a real man: gather → pick. Mutates gd.talk (stage, said, given); says nothing itself (Game.interrogate does the bubble). Null for a
 *  man who cannot talk (down — dead or out cold — or no longer on the roster). A man Sam holds is on the hold's clock (heldSinceOf) and nothing here touches it;
 *  the sandbox's bare exchange with a man nobody holds stamps talk.heldSince on the first press, or after HOLD_LAPSE_SECS without one. */
export function interrogate(g: Game, gd: Guard, rnd: () => number = Math.random): DialoguePick | null {
  if (gd.state === 'dead' || !g.guards.includes(gd)) return null;
  const talk = gd.talk;
  if (!gd.held && (talk.heldSince < 0 || (talk.lastT >= 0 && g.time - talk.lastT > HOLD_LAPSE_SECS))) talk.heldSince = g.time;
  return pickLine(gather(g, gd), talk, rnd);
}

/** Sam's side of the NEXT exchange with this man — the interaction panel's primary row while holding him (design §1.4; slice 1 builds the row): the question
 *  changes per stage and greys once he has nothing left. DRAFT strings like the rest. */
export function promptFor(gd: Guard): { text: string; spent: boolean } {
  const next = stageKeyOf(gd.talk.stage + 1);
  return next === 1 ? { text: 'make him talk', spent: false } : next === 2 ? { text: 'lean on him', spent: false } : next === 3 ? { text: 'ask what he knows', spent: false } : { text: "he's got nothing left", spent: true };
}
/** One line for the panel / probes: who, temperament, his cell now with the agitation terms, the stage the NEXT press would be, what he has said, the useful thing he would give next. */
export function dialogueSummary(g: Game, gd: Guard): string {
  const ctx = gather(g, gd), talk = gd.talk; const next = stageKeyOf(talk.stage + 1);
  const it = ctx.intel.find(i => i.live && i.known && !talk.given.has(i.kind));
  return `${gd.callsign} (${ctx.t}${gd.state === 'dead' ? ', down' : ''}) · cell ${ctx.k}·${ctx.band}·${ctx.t} · A ${ctx.a.toFixed(2)} [${ctx.why.join(', ') || '—'}] · next press: stage ${next}${givesAt(ctx.t, ctx.band).includes(next) ? ' (gives)' : ''} · said ${talk.said.size}${talk.given.size ? ` · gave ${[...talk.given].join('/')}` : ''} · next intel: ${it ? it.kind : '—'}${talk.last ? ` · last: ${talk.last.id} @${talk.last.cell} s${talk.last.stage}` : ''}`;
}

// ---------------------------------------------------------------- QA: a synthetic context for sweeping the matrix without a world, and the table lint
/** A hand-made context for cell (k, band, t): every token filled with a plausible value (unless overridden), the facts a knowledge level implies switched on. */
export function synthCtx(k: Knowledge, band: Band, t: Temperament, over: { facts?: Partial<Record<Need, boolean>>; tokens?: Partial<Record<Token, string | null>>; intelLive?: IntelKind[] } = {}): DialogueCtx {
  const a = band === 'high' ? 0.75 : band === 'mid' ? 0.45 : 0.1;
  const tokens: Partial<Record<Token, string | null>> = { partner: 'Novak', victim: k === 'sawHurt' ? 'Kowalski' : null, body: null, missing: null, room: 'the corridor', sawWhere: k === 'glimpsed' || k === 'saw' || k === 'sawHurt' ? 'the server room' : null, heardWhere: k === 'heard' ? 'the cubicles' : null, route: 'the corridor', partnerRoute: 'the cubicles', post: 'the east end', door: 'the server room door', caller: 'Reyes', checker: null, checked: null, clearRoom: null, breakerRoom: 'the storage room', checkEta: 'in a minute or so', minutes: null, n: 'three', ...over.tokens };
  const facts: Partial<Record<Need, boolean>> = { heardSmall: k === 'heard', sawShot: k === 'sawHurt', partnerKnown: !!tokens.partner, ...over.facts };
  const live = new Set<IntelKind>(over.intelLive ?? ['door', 'rollcall', 'routes']);
  const intel: IntelItem[] = INTEL_KINDS.map(kind => ({ kind, text: intelText(kind, t, k), live: live.has(kind), known: true }));
  return { who: 'synthetic', k, a, band, t, why: [], facts, tokens, intel, time: 0, recent: null };
}

/** Dev lint over the table (the QA probe runs it; empty = clean): unknown tokens / needs, over MAX_CHARS after the worst-case expansion, duplicate ids or texts,
 *  and coverage — every (K, band, T, stage) cell has at least one line it can always reach (no needs, tokens guaranteed for that K), stages 1 and 2 with a line
 *  written FOR that knowledge level (not just floor colour), every INTEL text within the limit with the hard prefix on. */
export function lintDialogue(): string[] {
  const problems: string[] = [];
  const ids = new Set<string>(), texts = new Set<string>();
  const worst = (text: string) => expand(text, TOKEN_WORST) ?? text;
  for (const l of LINES) {
    if (ids.has(l.id)) problems.push(`duplicate id ${l.id}`); ids.add(l.id);
    const key = l.text.trim().toLowerCase(); if (texts.has(key)) problems.push(`duplicate text (${l.id}): "${l.text}"`); texts.add(key);
    for (const tk of tokensIn(l.text)) if (!(TOKENS as readonly string[]).includes(tk)) problems.push(`${l.id}: unknown token {${tk}}`);
    for (const n of l.needs ?? []) if (!(NEEDS as readonly string[]).includes(n)) problems.push(`${l.id}: unknown need '${n}'`);
    const len = worst(l.text).length; if (len > MAX_CHARS) problems.push(`${l.id}: ${len} chars after expansion (> ${MAX_CHARS}): "${worst(l.text)}"`);
    if (!l.text.trim()) problems.push(`${l.id}: empty`);
    if (l.w !== undefined && !(l.w > 0)) problems.push(`${l.id}: weight ${l.w} (must be > 0 — the weighted pick degenerates otherwise)`);
    if (l.k !== '*' && !KNOWLEDGE.includes(l.k)) problems.push(`${l.id}: bad k ${l.k}`);
    if (!STAGES.includes(l.s)) problems.push(`${l.id}: bad stage ${String(l.s)}`);
  }
  const leadWorst = (ts: Temperament[], ks: Knowledge[]) => ts.reduce((m, t) => Math.max(m, ...ks.map(k => intelLead(t, k).length)), 0);   // the longest lead-in any of these men can put in front
  for (const kind of INTEL_KINDS) for (const [tk, text] of [...Object.entries(INTEL_TEXT[kind]), ...(kind === 'checkOn' ? [['self', CHECKON_SELF]] : [])] as [string, string][]) {
    if (!text) continue;
    for (const t2 of tokensIn(text)) if (!(TOKENS as readonly string[]).includes(t2)) problems.push(`intel ${kind}/${tk}: unknown token {${t2}}`);
    const [tPart, kPart] = tk.split(':') as [string, string | undefined];
    if (tPart !== '*' && tPart !== 'self' && !(TEMPERAMENTS as string[]).includes(tPart)) problems.push(`intel ${kind}: bad key '${tk}'`);
    if (kPart !== undefined && !(KNOWLEDGE as string[]).includes(kPart)) problems.push(`intel ${kind}: bad knowledge in key '${tk}'`);
    const users = tPart === '*' || tPart === 'self' ? TEMPERAMENTS.filter(t => tPart === 'self' || !INTEL_TEXT[kind][t]) : [tPart as Temperament];   // who can end up saying this variant (a '*' text is shadowed for men with their own), and knowing what
    const ks = kPart !== undefined ? [kPart as Knowledge] : KNOWLEDGE;
    const len = worst(text).length + leadWorst(users, ks); if (len > MAX_CHARS) problems.push(`intel ${kind}/${tk}: ${len} chars after expansion with the lead-in (> ${MAX_CHARS}): "${worst(text)}"`);
  }
  // coverage: reachable without needs and with only the guaranteed tokens
  for (const k of KNOWLEDGE) for (const band of BANDS) for (const t of TEMPERAMENTS) for (const s of STAGES) {
    const sure = new Set<string>([...TOKENS_ALWAYS, ...TOKENS_BY_K[k]]);
    const reach = (own: boolean) => LINES.some(l => lineMatches(l, k, band, t, s, false, false) && !l.needs?.length && tokensIn(l.text).every(x => sure.has(x)) && (!own || l.k === k));
    if (!reach(false)) problems.push(`cell ${k}·${band}·${t} stage ${String(s)}: no line always reachable`);
    else if ((s === 1 || s === 2) && !reach(true)) problems.push(`cell ${k}·${band}·${t} stage ${String(s)}: only floor-colour lines, none written for '${k}'`);
  }
  return problems;
}
/** Coverage matrix for the report / probe: lines per (K × stage), counting a line for every K it can serve ('*' lines under 'any'). */
export function coverageMatrix(): { rows: { k: string; counts: Record<string, number>; total: number }[]; total: number; intel: number } {
  const rows = [...KNOWLEDGE, '*' as const].map(k => { const counts: Record<string, number> = {}; let total = 0; for (const s of STAGES) { const n = LINES.filter(l => l.k === k && l.s === s).length; counts[String(s)] = n; total += n; } return { k: k === '*' ? 'any (needs-gated)' : k, counts, total }; });
  let intel = 0; for (const kind of INTEL_KINDS) intel += Object.values(INTEL_TEXT[kind]).filter(Boolean).length;
  return { rows, total: LINES.length, intel: intel + 1 };   // (+1: the checker's own variant of checkOn, inline in gather)
}
