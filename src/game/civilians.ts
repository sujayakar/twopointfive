// Civilians (STATUS E12 (d) — the maintainer: "sound great, we'll need them for the campaign"): the building's own night people — Marta, who cleans the
// cubicle floor and the corridor, and Owen, working late at a desk in the cubicle farm — with rules of their own. They fight nobody and nobody fights them
// (teams.ts 'civilian': hostile to no side, no side hostile to them; they are Guard RECORDS for the plumbing that takes one — the arm lock, the ragdoll, the
// bubble, the path follower — kept in Game.civilians, never in Game.guards, so no AI side perceives, lines up or holds fire round one and every loop over the
// shift stays the shift's). ONE MODEL: what they see of Sam is the guards' own eye test (guards.ts eyeContact) reading HIS light meter through the guards' own
// sight-gain arithmetic (sightGain: light^1.5 × distance × crouch × motion across her view × the smoke between them), only less keen and, head down over a desk
// or a bin, narrower; what they hear is the guards' hearing model — the same events, the same horizons, the same propagation round the corners
// (collision.ts SoundPaths) with the suppressed pistol's own 'odd noise' reach — and what the men then hear of HER (a scream) travels the same way back.
// The loop: WORK her route (wipe the desks, empty the bins, sit typing, the cooler, the printer — a library clip laid over the whole figure, character.ts
// `act`) → a glimpse over the glimpse line: STARTLED (she stops, looks, 'hello…?', and mostly talks herself out of it) → clearly seen, or a body, or a gun
// out, or a shot / a charge / a scream in earshot: ALARMED (a frozen beat and a gasp) → FLEE at a run to the NEAREST, by path, of a living calm guard, a
// wall phone, or a way out — never past what frightened her → REPORT (to the man: what and where; he takes it as a fix — a look for 'somebody', weapon out
// and a search of the spot for an armed man or a body, radioed, the floor a step up) / PHONE (six seconds on the line, then control puts it out over the net
// and sends the nearest calm man) / out of the DOOR (gone; control rings the floor a little later) → COWER in a corner (crouched, whimpering; up and off again
// if the man with the gun comes at her). HELD from behind like a guard (player.ts: the same grab, the same arm): she can be QUESTIONED (a thing or two she knows
// — the side door's digits on their sticky note, who walks where), told to get down and stay quiet — COMPLY: let go into a corner, and she tells nobody anything
// unless a guard walks onto her — choked out or pistol-whipped (out cold: priced hard on the card) or shot (a 'civilian casualty': forty points, control's fury
// when they find her, and with tune.civilianKillFails the mission there and then). Guards REACT to her: a scream in earshot or a woman running through his
// torch turns a calm man toward where she came from; one found crouched in a corner is asked what happened and tells him; one found on the floor is a body
// like any body — called in, the floor up (a dead one: lockdown, whatever it was before). The response team coming clears them off the floor (control's line;
// they run for the doors and are gone). `Game.tune.civilians` is the switch (default on; off = nobody but the shift, today's game byte for byte); under the
// showcase tour, the breach's daylight and the attract loop they are not on the floor at all. Authored as data (levels/office.json `civilians`, scene/level.ts
// CivilianDef: her loop, the task and facing at each waypoint, her tints); the phones and the ways out are CIV_SPOTS below (TODO(LevelData), checked headless
// like qrf.ts QRF_MARKS). Lines: dialogue.ts CIV_LINES. QA: tools/qa/civilians-probes.ts (`probes.ts civilians`), soak.ts --civilians on|off.
import { manLeaves } from '../sim/men/store';
import { men } from './seats';
import { SideMap, SideSet } from '../math/side';
import { Vec3, v3, clamp, DEG } from '../math/vec';
import { plainShort } from './gear';   // (a leaf: a weapon's name as it is written into a row)
import type { Game, DownKind, DownBy } from './game';
import { Guard } from './game';
import type { PatrolRoute, CivilianDef, CivilianTask } from '../scene/level';
import type { DialoguePick } from './dialogue';
import { CIV_LINES, CIV_COUNT, HOSTAGE_LINES, SNATCH_LINES, hashStr, roomName, isOffice } from './dialogue';
import type { HostageMoment } from './dialogue';
import { WORDS, w, wg } from './words';   // (a leaf: the dialogue pass's one switch)
import { civMost, levelTooBig } from './levelfit';
import { goTo } from './guards';
import { torchLightDef } from './rigProps';
import { isDaylight } from './lighting';
import { chronicleN } from './chronicle';
import { missionLive, DOWN_CARD_SECS } from './mission';
import './glass';
import { RAG_COUNT } from './ragdoll';
import type { Character } from './character';
import { hearPath } from './weather';
import { simRandom, mulberry32, rollOn } from '../math/dice';   // the simulation's die (math/dice.ts): the Game's own when it has one, Math.random otherwise
import { RUST_PASS, RUST_SURFACE, crossToRust } from './crossing';

// ================================================================ authored spots (TODO(LevelData): the phones and the ways off the floor belong in the level file beside the
// civilians block — like qrf.ts QRF_MARKS / needs.ts NEED_SPOTS they are checked headlessly against levels/office.json: free nav cells, reachable from every civilian's marks)
/** where a frightened civilian can ring security from (stand at `pos` facing `face`), and where she can get off the floor (`pos` just outside the door: reaching it = gone) */
export interface CivSpot { name: string; pos: Vec3; face: number; }
export const CIV_SPOTS = {
  phones: [
    { name: 'the reception phone', pos: [8.9, 0, 13.8], face: 0 },              // the staff side of the reception counter (the desk phone sits on its north edge at (8.45, 1.18, 14.85)), facing south over it
    { name: 'the break-room phone', pos: [34.6, 0, 16.2], face: Math.PI / 2 },  // the wall set over the break-room counter, two steps south of the coffee machine (needs.ts has a man stand at 14.4), facing the counter
  ] as CivSpot[],
  exits: [
    { name: 'the fire exit', pos: [38.2, 0, 23.2], face: 0 },                   // out through the fire exit and away down the east strip past the dumpster (reaching it = off the floor: she is gone into the night from there, not on the doorstep)
    { name: 'the outside door', pos: [34.0, 0, 25.0], face: Math.PI / 2 },        // out through the break room's outside door and east along the alley past the van
  ] as CivSpot[],
};

/** THE PHONES AND THE WAYS OFF THIS LEVEL'S FLOOR: a level that carries its own places (its file's scenarios.quiet.phones / .wayouts — scene/scenarios.ts
 *  QuietData; a level with a quiet mission says which its floor has, and one it leaves out is simply not there: she runs for a guard, or hides) reads them;
 *  every other level reads the office's table above, as it always has. */
export function civSpots(g: Game): { phones: CivSpot[]; exits: CivSpot[] } {
  const N = g.level.scenarios?.snatch;   // (game/snatch.ts: the one man of a snatch level runs for the block's own ways out; no phone, nobody to tell)
  if (N) { let o = ownSpots.get(N); if (!o) ownSpots.set(N, o = { phones: [], exits: N.exits.map(e => ({ name: e.name, pos: e.pos, face: 0 })) }); return o; }
  const Q = g.level.scenarios?.quiet; if (!Q) return CIV_SPOTS;
  let o = ownSpots.get(Q); if (!o) ownSpots.set(Q, o = { phones: (Q.phones ?? []).map(s => ({ name: s.name, pos: s.pos, face: s.face })), exits: (Q.wayouts ?? []).map(s => ({ name: s.name, pos: s.pos, face: s.face })) });
  return o;
}
const ownSpots = new SideMap<object, { phones: CivSpot[]; exits: CivSpot[] }>('civilians.ownSpots');

// ================================================================ numbers (Tab ▸ Sandbox ▸ civilians rows read a few; the switch and killFails persist per machine)
export const CIV = {
  /** her eye against the guards' (guards.ts GUARD_EYE 16 m / 58° / near field 3.5 m of 100°): walking about; head down over a task; once frightened (looking for it) */
  eye: { range: 12, halfDeg: 50, nearM: 2.5, nearDeg: 95 },
  eyeBusy: { range: 8, halfDeg: 34, nearM: 2.0, nearDeg: 80 },
  eyeAlarmed: { range: 14, halfDeg: 62, nearM: 3.0, nearDeg: 100 },
  /** × the guards' sight gain on Sam (a civilian is not looking for anyone); × that again for a gun in his hand seen inside gunM (or him doing violence) */
  keenK: 0.6, gunK: 2.5, gunM: 10,
  /** awareness lines: over glimpseAt she stops and looks ('hello…?'); over spotAt she has SEEN him — the guards' own alert line */
  glimpseAt: 0.3, spotAt: 0.85,
  /** the startle: how long she stands looking before she talks herself out of it; a second inside this and it is no longer nothing */
  startleSecs: 4.0, startleAgainSecs: 20,
  /** bodies: seen inside this range in her cone with a clear line to the pelvis (no light asked, as the men's own body test asks none) */
  bodyM: 10,
  /** the frozen beat before she runs (dealt per civilian and fright inside the window), her run, her walk, the crouched creep into a corner */
  freeze: [0.4, 0.8] as [number, number], runSpeed: 3.2, walkSpeed: 1.05, creepSpeed: 1.2,
  /** the report: she stops this close to the man (in sight of him) and takes this long telling him; the phone call's length */
  reportM: 2.2, reportSecs: 2.6, phoneSecs: 6,
  /** target choice: metres of path a GUARD is worth over a phone (a person beats a handset), what leaving the floor costs over either (the last resort), and what a
   *  target the far side of the fright costs (she does not run past him to reach it) */
  guardBonusM: 4, exitMalusM: 6, pastThreatM: 15,
  /** cowering: the corner is looked for within this; the man with the gun inside refleeM of her (and seen) puts her up and off again, once; not told to stay and
   *  nobody come for her in loneSecs, she goes looking for a guard again; told and the floor calm again for settleSecs, back to work */
  cornerM: 4, refleeM: 6, loneSecs: 45, settleSecs: 75,
  /** 'stay down, stay quiet': minutes she stays put and tells nobody (unless a man walks onto her) */
  complyMin: 3,
  /** the men: a scream carries this far by the sound's own path (× its carry); a running / screaming woman is SEEN inside seeM in his cone if lit past seeLit on her own
   *  meter (or inside 4 m); one crouched in a corner is found inside foundM with a clear line, and asked after foundSecs */
  screamM: 16, seeM: 12, seeLit: 0.25, foundM: 3.0, foundSecs: 1.2,
  /** control rings the floor this long after a civilian gets out of the building */
  outsideCallSecs: 18,
  /** passing pleasantries with a calm man inside greetM (a clear line between them): at most one per pair per pairSecs and one on the floor per floorSecs — off = never */
  greet: true, greetM: 3.0, greetPairSecs: 240, greetFloorSecs: 70,
  /** the response team on its way: control clears them off the floor (they run for the doors) */
  evacuateOnQrf: true,
};

/** the price list's civilian terms live in chronicle.ts SCORE (civReported / civKO / civKilled) with the rest of the card's */

// ================================================================ ids, tints, dice
/** character ids: clear of Sam's 1, the staff's 10‥15 and the units' 20‥ (qrf.ts / breach.ts take the low twenties); ≥ 10 so a leaf she pushes is 'a door one of us
 *  opened' to the men's ears (guards.ts hearingCheck: who ≥ 10), not a creak nobody can explain */
const CIV_ID0 = 40;
// her own dice (line picks, the frozen beat) are mulberry32 (math/dice.ts) off her name — never Math.random, so a floor with her on it and unprovoked is the seeded floor without her
// (the same die with its state kept ON the record — `dice`, rolled by `rnd` = math/dice.ts rollOn, which captures nothing — so a checkpoint carries it, a loaded
//  civilian draws on exactly where she was, and a record a rollback snapshot puts into another game rolls its own die, not the first game's)
/** run `fn` with Math.random swapped for a private seeded die: the Guard / Character constructors draw from Math.random (the idle phase, the look clock) and a civilian's
 *  construction must not move the game's own sequence by one draw (every seeded QA run, every frozen identity digest, is of the floor WITHOUT her) */
function withDice<T>(seed: number, fn: () => T): T { const R = Math.random; Math.random = mulberry32(seed); try { return fn(); } finally { Math.random = R; } }

// ================================================================ the record (Guard.civ)
export type CivState = 'work' | 'startled' | 'alarmed' | 'flee' | 'report' | 'phone' | 'cower' | 'held' | 'evacuate' | 'gone';
/** what frightened her: Sam seen plainly / with a gun out (or doing violence) / a body / gunfire, a stun charge, a door kicked in / the suppressed pistol's odd report
 *  close by / another civilian's scream / a crash right beside her twice over / Sam's arm round her throat / the panel's button */
export type CivCause = 'sam' | 'gun' | 'body' | 'gunfire' | 'shot' | 'bang' | 'scream' | 'noise' | 'grabbed' | 'panel';
/** whom / what she is running to */
export interface CivTarget { kind: 'guard' | 'phone' | 'exit'; guard: Guard | null; spot: CivSpot | null; pos: Vec3; name: string; planPos: Vec3; }
export interface Civ {
  def: CivilianDef; name: string; label: string; she: string; her: string;
  state: CivState; stateAt: number;
  /** work: seconds into the task at the current waypoint; standing up out of a chair holds her feet this long; seated = pinned onto the chair (no collision, no shove);
   *  settleT = seconds spent stepping the last hand's breadth onto the mark itself (the path ends on a cell centre) before the task begins, −1 = not settling */
  taskT: number; riseT: number; seated: boolean; settleT: number;
  /** the startle: where she is looking, until when; when she last talked herself out of one */
  lookAt: Vec3 | null; lookUntil: number; shrugAt: number;
  /** the fright: what, where it was (what she tells the man / the phone: the men's fix), when, whether it means an ARMED man (a gun, a body, gunfire, the arm), and a
   *  serial per fright (the men are each told once per fright) */
  cause: CivCause | null; causePos: Vec3 | null; causeAt: number; armed: boolean; alarmN: number;
  /** eyes on Sam: when last, where; whether she has seen the gun */
  seenAt: number; seenPos: Vec3 | null; sawGun: boolean;
  /** the run: the target, when it was last re-chosen, targets that came to nothing this fright, the frozen beat's end; pinTarget = sent (the panel's phone button): no re-choosing */
  target: CivTarget | null; retargetAt: number; badTargets: Set<object>; freezeUntil: number; pinTarget: boolean;
  /** the report / the call: the man being told, the clock, whether this fright has been told to a man / rung in / put out by control */
  reportTo: Guard | null; talkT: number; reported: boolean; phoned: boolean; acked: boolean;
  /** the corner: where, told to stay (until when), whether she has got up and run from it once, seconds a man has stood by her */
  cowerAt: Vec3 | null; complied: boolean; complyUntil: number; refled: boolean; foundT: number; foundBy: Guard | null; foundLineAt: number; foundTold: boolean;
  /** a rescue's hostage told to get up and come (hostageLead — rescueend.ts): whom she follows (null: nobody — she is down where she was put), and how far behind
   *  him she stops. Not in the civilians' own snapshot: no checkpoint is reachable with a rescue staged, and rescueend.ts restore() gives the word again */
  lead: Character | null; leadGap: number;
  /** led, and sent the last steps to a spot by herself (civilianLedTo — snatchout.ts: the van's doors): she walks there in place of to the leader's heels */
  leadTo: Vec3 | null;
  /** her voice: when she last screamed (what the men and the other civilian hear), whimper clock, greeting clocks */
  screamAt: number; whimperAt: number; greetAt: number; lastLineAt: number;
  /** the men told of THIS fright already (a scream heard, her seen running), by alarm serial */
  told: WeakMap<Guard, number>;
  /** bodies she has already seen (one fright per body) */
  knownBodies: WeakSet<object>;
  /** questioned in the arm: how many things she has given */
  intelN: number;
  /** control's call after she got out: due at (−1 none) */
  outsideCallAt: number;
  /** her light meter (what a man's eye gets off her running through his torch): eased like everybody's */
  vis: number;
  /** her own dice: the mulberry state (a checkpoint keeps it) and the draw off it */
  dice: number; rnd: (this: { dice: number }) => number;
  /** what happened when (probes / the panel): 't state/cause …' */
  log: string[];
}

/** module-wide counters for the QA soak and the last things that happened (probes / the panel) */
export const CIV_STATS = { spawned: 0, startled: 0, alarmed: 0, fled: 0, reports: 0, phoned: 0, outsideCalls: 0, cowered: 0, complied: 0, ko: 0, killed: 0, found: 0, foundDown: 0, evacuated: 0, greetings: 0, refled: 0, noticed: 0, samSeen: 0 };
export const CIV_LOG: { t: number; who: string; what: string }[] = [];
export const CIV_DEBUG: { log: ((s: string) => void) | null } = { log: null };
function log(g: Game, gd: Guard, what: string) { const C = gd.civ!; C.log.push(`${g.time.toFixed(1)} ${what}`); if (C.log.length > 40) C.log.splice(0, 10); CIV_LOG.push({ t: g.time, who: C.name, what }); if (CIV_LOG.length > 200) CIV_LOG.splice(0, 50); CIV_DEBUG.log?.(`${g.time.toFixed(2)}s ${C.name}: ${what}`); }

// ================================================================ standing them up / down
/** A CIVILIAN COMES INTO A GAME HERE AND NOWHERE ELSE (the level's own as it is stood up, a rescue's hostages, the man a snatch is for). A game holds as
 *  many as one step of the doors can be handed beside every other man it may hold (levelfit.ts civMost), and one more is a level too big for the game,
 *  said by name as he would join: so "more men than a step is handed" stays arithmetic whoever adds a civilian, and when. */
function join(g: Game, gd: Guard): Guard {
  const most = civMost(); if (g.civilians.length >= most) throw new Error(levelTooBig(g.level, `more than ${most} civilians at once (its own, a rescue's hostages, the man a snatch is for), and a game holds no more men than one step of its doors can be handed (game/levelfit.ts civMost)`));
  g.civilians.push(gd); return gd;
}
/** Fresh civilians on their first marks (game.ts: at construction after the shift, and resetCivilians). Nothing with the switch off or none authored. */
export function spawnCivilians(g: Game) {
  g.civilians = [];
  if (!g.tune.civilians || g.nightBreach?.active || g.rescue?.active) return;   // (the night breach playtest clears the floor of everybody but its cast — nightbreach.ts)
  const defs = g.level.civilians ?? [];
  defs.forEach((def, i) => { join(g, makeCivilian(g, def, i)); CIV_STATS.spawned++; });
}
/** restartEncounter / resetGuards: whoever Sam has hold of or is hauling let go of, then fresh civilians (or none, the switch off since) */
export function resetCivilians(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('resetCivilians', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  for (const s of g.seats) { const pl = s.view.player; if (!pl) continue;
    if (pl.holding?.g.civ) { const gd = pl.holding.g; gd.held = null; gd.char.anim.heldPose = null; pl.holding = null; pl.char.anim.holdPose = null; }
    if (pl.dragging?.civ) s.view.toggleDrag();
    if (pl.takedown?.g.civ) pl.takedown = null;
  }
  for (const cv of g.civilians) { forgetMeter(g, cv); manLeaves(cv.char, cv); }
  const F = floorOf(g); F.evacTold = false; F.greetAt = -1e9; F.pending.length = 0;   // (a word due from a man of the old shift is nobody's now)
  spawnCivilians(g);
}
// ================================================================ save / load — the checkpoints' slice (game/savestate.ts saveState / applyState call these two)
/** One civilian as plain data — everything the next frame of hers hangs on: where she is and how she is moving (the Guard record's share: place, gait, the mark she
 *  is on and its wait, the walk under way with its watchdogs, her ears' marker in the event ids, the arm-lock's talk record, her bubble); how she went down if she
 *  did (and the body's particles, laid down again asleep like the men's — savestate.ts RagSnap); and the Civ record whole: the task's clocks, the startle, the
 *  fright (what / where / when / armed / its serial), whom she is running to (a man by CALLSIGN, a phone / a way out by name, the plan's points), whom she has told
 *  and how far the telling got, the corner and the promise, who has stood by her, her voice's clocks, the men already told of this fright and the bodies she
 *  knows of (by callsign), what she has given up in the arm, control's call due, her meter, her dice. Absolute game times throughout (`CivSnap.time` rides
 *  along so a restore under another clock can shift them). A civilian HELD at the save is filed held: on the first frame after the load she comes off an arm that
 *  is no longer round her exactly as off a cancelled hold (released: the fright of her life, from where Sam stands). */
export interface CivSnapOne {
  id: string;
  pos: Vec3; vel: Vec3; yaw: number; aimYaw: number; crouch: number; animPhase: number; speed: number; stepPhase: number; lookPhase: number;
  wp: number; wait: number; awareness: number; heardUpTo: number;
  path: Vec3[]; pathI: number; pathGoal: Vec3 | null; repathT: number; lastPos: Vec3; stuckT: number; stuckN: number; stallRef: Vec3 | null; stallT: number;
  talk: { stage: number; presses: number; lastT: number; abortT: number }; bubble: { text: string; t: number; dur: number; radio: boolean } | null;
  down: { kind: DownKind; by: DownBy; at: number; found: boolean; rag: { x: number[]; rot: [number, number, number, number]; sleeping: boolean; own?: boolean } | null } | null;   // (own: savestate.ts RagSnap — the trunk's rest shape is these particles' own on a load)
  state: CivState; stateAt: number; taskT: number; riseT: number; settleT: number; seated: boolean; lookAt: Vec3 | null; lookUntil: number; shrugAt: number;
  cause: CivCause | null; causePos: Vec3 | null; causeAt: number; armed: boolean; alarmN: number; seenAt: number; seenPos: Vec3 | null; sawGun: boolean;
  target: { kind: 'guard' | 'phone' | 'exit'; guard: string | null; spot: string | null; pos: Vec3; planPos: Vec3 } | null; retargetAt: number; badTargets: string[]; freezeUntil: number; pinTarget: boolean;
  reportTo: string | null; talkT: number; reported: boolean; phoned: boolean; acked: boolean;
  cowerAt: Vec3 | null; complied: boolean; complyUntil: number; refled: boolean; foundT: number; foundBy: string | null; foundLineAt: number; foundTold: boolean;
  screamAt: number; whimperAt: number; greetAt: number; lastLineAt: number; told: [string, number][]; knownBodies: string[]; intelN: number; outsideCallAt: number; vis: number; dice: number;
}
/** the queue's entries: `cv` / `o` by callsign */
export interface CivPendingSnap { at: number; kind: PendKind; cv: string; o: string | null; key?: string; armed?: boolean; pos?: Vec3; lvl?: number; dead?: boolean; }
export interface CivSnap { time: number; list: CivSnapOne[]; floor: { evacTold: boolean; greetAt: number; pending: CivPendingSnap[] }; }
export function civiliansSnapshot(g: Game): CivSnap {
  const cp = (p: Vec3 | null | undefined): Vec3 | null => p ? v3.copy(p) : null;
  const list: CivSnapOne[] = g.civilians.map(gd => {
    const C = gd.civ!, c = gd.char, dead = gd.state === 'dead', rag = c.ragdoll;
    const told: [string, number][] = []; for (const o of g.guards) { const n = C.told.get(o); if (n !== undefined) told.push([o.callsign, n]); }
    const knownBodies: string[] = []; for (const o of [...g.guards, ...g.civilians]) if (o !== gd && C.knownBodies.has(o)) knownBodies.push(o.callsign);
    return {
      id: C.def.name,
      pos: v3.copy(c.pos), vel: dead ? [0, 0, 0] : v3.copy(c.vel), yaw: c.bodyYaw, aimYaw: c.aimYaw, crouch: c.anim.crouchTarget, animPhase: c.anim.phase, speed: gd.speed, stepPhase: gd.stepPhase, lookPhase: gd.lookPhase,
      wp: gd.wp, wait: gd.wait, awareness: gd.awareness, heardUpTo: gd.heardUpTo,
      path: gd.path.map(p => v3.copy(p)), pathI: gd.pathI, pathGoal: cp(gd.pathGoal), repathT: gd.repathT, lastPos: v3.copy(gd.lastPos), stuckT: gd.stuckT, stuckN: gd.stuckN, stallRef: cp(gd.stallRef), stallT: gd.stallT,
      talk: { stage: gd.talk.stage, presses: gd.talk.presses, lastT: gd.talk.lastT, abortT: gd.talk.abortT }, bubble: gd.bubble ? { ...gd.bubble } : null,
      down: dead ? { kind: gd.downKind ?? 'struck', by: gd.downBy ?? 'world', at: gd.diedAt, found: gd.found, rag: rag ? { x: Array.from(rag.x, x => Math.round(x * 1e4) / 1e4), rot: [rag.rot[0], rag.rot[1], rag.rot[2], rag.rot[3]], sleeping: rag.sleeping, own: true } : null } : null,
      state: C.state, stateAt: C.stateAt, taskT: C.taskT, riseT: C.riseT, settleT: C.settleT, seated: C.seated, lookAt: cp(C.lookAt), lookUntil: C.lookUntil, shrugAt: C.shrugAt,
      cause: C.cause, causePos: cp(C.causePos), causeAt: C.causeAt, armed: C.armed, alarmN: C.alarmN, seenAt: C.seenAt, seenPos: cp(C.seenPos), sawGun: C.sawGun,
      target: C.target ? { kind: C.target.kind, guard: C.target.guard?.callsign ?? null, spot: C.target.spot?.name ?? null, pos: v3.copy(C.target.pos), planPos: v3.copy(C.target.planPos) } : null, retargetAt: C.retargetAt,
      badTargets: [...C.badTargets].map(x => (x as Guard).callsign ?? (x as CivSpot).name).filter(Boolean), freezeUntil: C.freezeUntil, pinTarget: C.pinTarget,
      reportTo: C.reportTo?.callsign ?? null, talkT: C.talkT, reported: C.reported, phoned: C.phoned, acked: C.acked,
      cowerAt: cp(C.cowerAt), complied: C.complied, complyUntil: C.complyUntil, refled: C.refled, foundT: C.foundT, foundBy: C.foundBy?.callsign ?? null, foundLineAt: C.foundLineAt, foundTold: C.foundTold,
      screamAt: C.screamAt, whimperAt: C.whimperAt, greetAt: C.greetAt, lastLineAt: C.lastLineAt, told, knownBodies, intelN: C.intelN, outsideCallAt: C.outsideCallAt, vis: C.vis, dice: C.dice,
    };
  });
  const F = floorOf(g);
  return { time: g.time, list, floor: { evacTold: F.evacTold, greetAt: F.greetAt, pending: F.pending.map(P => ({ at: P.at, kind: P.kind, cv: P.cv.callsign, o: P.o?.callsign ?? null, key: P.key, armed: P.armed, pos: P.pos ? v3.copy(P.pos) : undefined, lvl: P.lvl, dead: P.dead })) } };
}
/** Put the civilians back as `snap` has them ONTO THE FRESH ROSTER a restart has just stood up (savestate.ts applyState runs after restartEncounter → resetGuards
 *  → resetCivilians: no second reset here), each entry applied by name — clocks shifted by (now − snap.time), men found again by callsign (one gone = that
 *  target / teller dropped: she re-chooses next frame), a downed one laid down where she lay, asleep at the file's particles (no score, no line: the chronicle
 *  carries the original row; found stays found), the queue of words due put back. `snap` null (a file from before this slice, or the switch off at the save):
 *  the fresh ones stay on their marks — but their EARS are set right either way: a fresh civilian's marker was stamped off the abandoned future's event count,
 *  which the load has just wound back (review G3.1: deaf for minutes after every load). Entries for civilians this level no longer has are ignored. */
export function civiliansRestore(g: Game, snap: CivSnap | null | undefined) {
  for (const cv of g.civilians) { cv.heardUpTo = Math.min(cv.heardUpTo, g.eventSeq); cv.civ!.stateAt = Math.min(cv.civ!.stateAt, g.time); }   // (both stamped at construction off the abandoned future: the event count and the clock have since gone BACK)
  if (!snap || !Array.isArray(snap.list)) return;
  const fin = (x: number | undefined | null, d = 0) => typeof x === 'number' && Number.isFinite(x) ? x : d;
  const dtc = g.time - fin(snap.time, g.time); const sh = (t: number | undefined | null, d = -1e9) => { const v = fin(t, d); return v > -1e8 && v < 1e8 ? v + dtc : v; };
  const cp = (p: Vec3 | null | undefined): Vec3 | null => p && p.length >= 3 ? [fin(p[0]), fin(p[1]), fin(p[2])] : null;
  const man = (cs: string | null | undefined) => (cs && g.guards.find(x => x.callsign === cs && x.state !== 'dead')) || null;
  const anyone = (cs: string | null | undefined) => (cs && (g.guards.find(x => x.callsign === cs) ?? g.civilians.find(x => x.callsign === cs))) || null;
  const spotOf = (kind: string, name: string | null | undefined) => (kind === 'phone' ? civSpots(g).phones : kind === 'exit' ? civSpots(g).exits : [...civSpots(g).phones, ...civSpots(g).exits]).find(x => x.name === name) ?? null;
  for (const S of snap.list) {
    const gd = g.civilians.find(x => x.civ!.def.name === S.id); if (!gd) continue; const C = gd.civ!, c = gd.char;
    // the Guard record's share
    c.pos = cp(S.pos) ?? c.pos; c.vel = cp(S.vel) ?? [0, 0, 0]; c.bodyYaw = fin(S.yaw); c.aimYaw = fin(S.aimYaw, c.bodyYaw); c.aimPitch = 0; c.anim.crouchTarget = fin(S.crouch); c.anim.phase = fin(S.animPhase); gd.speed = fin(S.speed); gd.stepPhase = fin(S.stepPhase); gd.lookPhase = fin(S.lookPhase);
    gd.wp = (S.wp | 0) % Math.max(1, gd.route.points.length); gd.wait = fin(S.wait); gd.awareness = clamp(fin(S.awareness), 0, 1); gd.heardUpTo = Math.min(fin(S.heardUpTo, g.eventSeq), g.eventSeq);
    gd.path = (S.path ?? []).map(p => cp(p) ?? [0, 0, 0]); gd.pathI = Math.max(0, S.pathI | 0);   // (as saved, NOT clamped to the path: a walk that ended and had its path cleared leaves pathI past the empty list — inert, every reader asks pathI < path.length — and clamping it made load ≠ the game stepped on, and save(load(s)) ≠ s)
    gd.pathGoal = cp(S.pathGoal); gd.repathT = fin(S.repathT); gd.lastPos = cp(S.lastPos) ?? v3.copy(c.pos); gd.stuckT = fin(S.stuckT); gd.stuckN = S.stuckN | 0; gd.stallRef = cp(S.stallRef); gd.stallT = fin(S.stallT);
    if (S.talk) { gd.talk.stage = S.talk.stage | 0; gd.talk.presses = S.talk.presses | 0; gd.talk.lastT = fin(S.talk.lastT, -1); gd.talk.abortT = fin(S.talk.abortT, -1); gd.talk.heldSince = -1; }
    gd.bubble = S.bubble ? { text: String(S.bubble.text), t: sh(S.bubble.t), dur: fin(S.bubble.dur), radio: !!S.bubble.radio } : null;
    gd.held = null; c.anim.heldPose = null;
    if (S.down) {   // the body: settled where she lay (savestate.ts lays the men down the same way)
      gd.state = 'dead'; gd.speed = 0; gd.diedAt = sh(S.down.at); gd.downKind = S.down.kind; gd.downBy = S.down.by; gd.found = !!S.down.found; c.anim.act = null; C.target = null; C.reportTo = null;
      c.update(0); c.ragdollize(g.col, [Math.sin(c.bodyYaw), 0, Math.cos(c.bodyYaw)], true);
      const R = S.down.rag; if (R && R.x?.length === RAG_COUNT * 3 && c.ragdoll) { c.ragdoll.settleAt(R.x, R.rot, R.own === true); if (R.sleeping === false) c.ragdoll.wake(); }
      c.vel = [0, 0, 0]; c.anim.speed = 0; c.update(0);
      log(g, gd, `restored — down (${gd.downKind}${gd.found ? ', found' : ''})`);
      continue;
    }
    // the Civ record
    C.state = S.state; C.stateAt = sh(S.stateAt); C.taskT = fin(S.taskT); C.riseT = fin(S.riseT); C.settleT = fin(S.settleT, -1); C.seated = !!S.seated; C.lookAt = cp(S.lookAt); C.lookUntil = sh(S.lookUntil, -1); C.shrugAt = sh(S.shrugAt);
    C.cause = S.cause ?? null; C.causePos = cp(S.causePos); C.causeAt = sh(S.causeAt, -1); C.armed = !!S.armed; C.alarmN = S.alarmN | 0; C.seenAt = sh(S.seenAt); C.seenPos = cp(S.seenPos); C.sawGun = !!S.sawGun;
    C.target = null;
    if (S.target) {
      if (S.target.kind === 'guard') { const o = man(S.target.guard); if (o) C.target = { kind: 'guard', guard: o, spot: null, pos: cp(S.target.pos) ?? v3.copy(o.char.pos), name: o.callsign, planPos: cp(S.target.planPos) ?? v3.copy(o.char.pos) }; }
      else { const spot = spotOf(S.target.kind, S.target.spot); if (spot) C.target = { kind: S.target.kind, guard: null, spot, pos: v3.copy(spot.pos), name: spot.name, planPos: v3.copy(spot.pos) }; }
      if (!C.target) { gd.path = []; gd.pathGoal = null; }   // (her man is gone: flee() re-chooses on the next frame)
    }
    C.retargetAt = sh(S.retargetAt); C.freezeUntil = sh(S.freezeUntil, -1); C.pinTarget = !!S.pinTarget && !!C.target;
    C.badTargets.clear(); for (const n of S.badTargets ?? []) { const x = man(n) ?? spotOf('', n); if (x) C.badTargets.add(x); }
    C.reportTo = man(S.reportTo); if (C.state === 'report' && !C.reportTo) { C.state = 'flee'; C.stateAt = g.time; c.anim.act = null; }
    C.talkT = fin(S.talkT); C.reported = !!S.reported; C.phoned = !!S.phoned; C.acked = !!S.acked;
    C.cowerAt = cp(S.cowerAt); C.complied = !!S.complied; C.complyUntil = sh(S.complyUntil, -1); C.refled = !!S.refled; C.foundT = fin(S.foundT); C.foundBy = man(S.foundBy); C.foundLineAt = S.foundLineAt < 0 ? -1 : sh(S.foundLineAt, -1); C.foundTold = !!S.foundTold;
    C.screamAt = sh(S.screamAt); C.whimperAt = sh(S.whimperAt); C.greetAt = sh(S.greetAt); C.lastLineAt = sh(S.lastLineAt);
    C.told = new SideMap(); for (const [cs, n] of S.told ?? []) { const o = man(cs); if (o) C.told.set(o, n | 0); }
    C.knownBodies = new SideSet(); for (const cs of S.knownBodies ?? []) { const o = anyone(cs); if (o) C.knownBodies.add(o); }
    C.intelN = S.intelN | 0; C.outsideCallAt = S.outsideCallAt < 0 ? -1 : sh(S.outsideCallAt, -1); C.vis = fin(S.vis); if (Number.isFinite(S.dice)) C.dice = S.dice | 0;
    // the figure: the task's clip back on her at once where she was in it (a woman at her desk loads seated, not standing up into the chair over a second)
    const K = C.state === 'work' && gd.wait > 0 ? TASK_CLIP[taskAt(gd)] : null;
    c.anim.act = K ? { clip: K.clip, loop: K.loop, rate: K.rate, in: 0, out: K.out } : (C.state === 'report' || C.state === 'phone') ? { clip: 'Idle_Talking_Loop', loop: true, rate: 1.2, in: 0, out: 0.4 } : null;
    c.update(0);
    log(g, gd, `restored — ${C.state}${C.cause ? ` [${C.cause}${C.armed ? ', armed' : ''}]` : ''}${C.target ? ` → ${C.target.name}` : ''}`);
  }
  // the floor's memory and the queue of words due (an entry whose civilian is gone is dropped; a man gone = the doing runs with nobody excepted, as it would had he died meanwhile)
  const F = floorOf(g), SF = snap.floor;
  if (SF) {
    F.evacTold = !!SF.evacTold; F.greetAt = sh(SF.greetAt); F.pending.length = 0;
    for (const P of SF.pending ?? []) { const cv = g.civilians.find(x => x.callsign === P.cv); if (!cv || !KINDS.has(P.kind)) continue; F.pending.push({ at: sh(P.at, g.time), kind: P.kind, cv, o: anyone(P.o) ?? null, key: P.key, armed: P.armed, pos: cp(P.pos) ?? undefined, lvl: P.lvl, dead: P.dead }); }
  }
}
const KINDS: ReadonlySet<PendKind> = new Set<PendKind>(['greetReply', 'reportRadio', 'escalate', 'outsideCall', 'complyLine', 'foundReport', 'bodyEsc', 'outrage']);
/** savestate.ts Checkpoints.gate — why no AUTOSAVE may be written on the civilians' account (null = they are no objection): one who has glimpsed something and is
 *  still deciding, one frightened and not yet done with it (frozen, running, telling a man, on the phone — a checkpoint a stride ahead of the floor going up is a
 *  death loop, exactly as with a man about to see him), one hiding from HIM with nobody told yet (she saw Sam, or his gun, or had his arm round her throat, and
 *  was not told to stay down: the next man by her corner hears all of it), or a word of theirs still on its way (a guard's radio call a beat after her report,
 *  the floor step after that, control's call after she got out of the door). A civilian told to stay down, or lying unfound, is the player's own settled doing —
 *  no objection, as an unfound body is none. */
export function civilianCompromise(g: Game): string | null {
  if (!civiliansOnFloor(g)) return null;
  for (const cv of g.civilians) {
    if (cv.state === 'dead' || cv.held) continue; const C = cv.civ!;
    if (C.state === 'startled') return `${C.name} has glimpsed something`;
    if (C.state === 'alarmed' || C.state === 'flee' || C.state === 'report' || C.state === 'phone') return `${C.name} is ${C.state === 'alarmed' ? 'frightened' : C.state === 'flee' ? `running for ${C.target?.name ?? 'help'}` : C.state === 'report' ? `telling ${C.reportTo?.callsign ?? 'a guard'}` : 'on the phone to security'}`;
    if (C.state === 'cower' && !C.reported && !C.complied && (C.cause === 'sam' || C.cause === 'gun' || C.cause === 'grabbed')) return `${C.name} is hiding from you and nobody knows yet`;
  }
  const P = floorOf(g).pending.find(p => p.kind !== 'greetReply' && p.kind !== 'complyLine');
  if (P) return `${P.cv.callsign}'s ${P.kind === 'outsideCall' ? 'news is about to reach control' : P.kind === 'reportRadio' ? 'report is about to go over the net' : 'word is about to move the floor'}`;
  return null;
}
function makeCivilian(g: Game, def: CivilianDef, i: number): Guard {
  const id = CIV_ID0 + i;
  const route: PatrolRoute = { name: def.name, points: def.points.map(p => v3.copy(p)), wait: [...def.wait], keys: [] };   // her loop in a route's shape (Guard.route / wp / wait run it); nothing on her belt
  const fl = torchLightDef(`civlight${id}`, id, 0, 10, 20); fl.enabled = false; fl.intensity = 0;                            // a light record for the plumbing that reads one (the grab asks was it lit): never in the engine, never lit
  const p0 = route.points[0], p1 = route.points[1 % route.points.length];
  const yaw = def.face?.[0] ?? (route.points.length > 1 ? v3.yawTo(p0, p1) : 0);
  const gd = withDice(hashStr(`civ:${def.name}:${i}`), () => new Guard(g.men, g.ch, id, route, -1, fl, v3.copy(def.tint), { tint2: def.tint2 ? v3.copy(def.tint2) : [0.06, 0.06, 0.08], yaw, name: def.name, team: 'civilian' }));
  gd.foe = null; gd.drawn = false; gd.heardUpTo = g.eventSeq;   // (deaf to what was in the air before she existed)
  const worker = def.kind === 'worker';
  gd.civ = {
    def, name: def.name, label: worker ? 'late worker' : 'cleaner', she: worker ? 'he' : 'she', her: worker ? 'him' : 'her',
    state: 'work', stateAt: g.time, taskT: 0, riseT: 0, seated: false, settleT: -1, lookAt: null, lookUntil: -1, shrugAt: -1e9,
    cause: null, causePos: null, causeAt: -1, armed: false, alarmN: 0, seenAt: -1e9, seenPos: null, sawGun: false,
    target: null, retargetAt: -1, badTargets: new Set(), freezeUntil: -1, pinTarget: false,
    reportTo: null, talkT: 0, reported: false, phoned: false, acked: false,
    cowerAt: null, complied: false, complyUntil: -1, refled: false, foundT: 0, foundBy: null, foundLineAt: -1, foundTold: false,
    lead: null, leadGap: 0, leadTo: null,
    screamAt: -1e9, whimperAt: -1e9, greetAt: -1e9, lastLineAt: -1e9, told: new SideMap(), knownBodies: new SideSet(), intelN: 0, outsideCallAt: -1, vis: 0,
    dice: hashStr(`civdice:${def.name}`) | 0, rnd: rollOn, log: [],   // (the sequence mulberry32(dice) gives)
  };
  gd.wait = route.wait[0] ?? 0;   // she starts AT her first mark, doing what she does there
  gd.char.update(0);
  return gd;
}
/** A HOSTAGE (game/rescue.ts — the hostage rescue playtest; nobody else calls it): a civilian of the usual make put on the floor at `pos` facing `face`, down and
 *  told to stay down — the complied cower with no end to it, so gunfire, a charge going off beside her and the men coming through the door leave her where she
 *  is (toAlarmed's own rule for a civilian who was told). On Game.civilians like the cleaner, so the rounds, the pose and the body are the civilians' own. */
export function spawnHostage(g: Game, def: { name: string; kind: CivilianDef['kind']; tint: Vec3; tint2?: Vec3; pos: Vec3; face: number }): Guard { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('spawnHostage', undefined, [g, def]); if (__r !== RUST_PASS) return __r as any; } }
  const gd = makeCivilian(g, { name: def.name, kind: def.kind, tint: def.tint, tint2: def.tint2, points: [v3.copy(def.pos)], wait: [0], face: [def.face] }, g.civilians.length);
  join(g, gd);
  toCower(g, gd, true); gd.civ!.complyUntil = Infinity; gd.civ!.cause = 'gun'; gd.civ!.armed = true;
  if (WORDS.on) gd.civ!.label = 'hostage';   // (what a row or a message calls her: no cleaner, no late worker — the dialogue pass)
  return gd;
}
/** THE MAN A SNATCH IS FOR (game/snatch.ts — nobody else calls these four): a civilian of the usual make at his own marks, working them like the late worker
 *  until something frightens him; then the civilians' own fright, run and corner are his (civSpots hands him the level's ways out). */
export function spawnTarget(g: Game, def: { name: string; pos: Vec3; face: number; tint: Vec3; tint2?: Vec3; points?: Vec3[]; wait?: number[] }): Guard {
  const pts = def.points?.length ? def.points : [def.pos];
  const gd = makeCivilian(g, { name: def.name, kind: 'worker', tint: def.tint, tint2: def.tint2, points: pts.map(p => v3.copy(p)), wait: pts.map((_, i) => def.wait?.[i] ?? 30), face: pts.map(() => def.face) }, g.civilians.length);
  join(g, gd);
  gd.civ!.label = 'man you came for';   // (no article: its readers write 'the ' in front of the label)
  return gd;
}
/** tied where he is (the pose is updateOne's, asked of g.snatch): the promise with no end, whatever state the tie found him in */
export function civilianTie(g: Game, gd: Guard) {
  const C = gd.civ!; if (gd.state === 'dead' || C.state === 'gone') return;
  if (C.state !== 'cower') toCower(g, gd, true);
  C.complied = true; C.complyUntil = Infinity;
}
/** A HOSTAGE'S VOICE (the dialogue pass — words.ts; the words are dialogue.ts HOSTAGE_LINES): rescue.ts names the moment and she says a line of it, off her own
 *  die, never the one over her head now; `other` = the hostage the line is about (a line that names one is left out without). A bubble (and, unseen, a log row for the moments that happen once) and nothing else: no
 *  scream's stamp, no event — nobody's ears are moved by it (the cry under the loud moments is the mixer's alone). False: she is down / held / has no such line. */
export function hostageSay(g: Game, gd: Guard, moment: HostageMoment, other: Guard | null = null): boolean {
  const C = gd.civ; if (!C || gd.state === 'dead' || gd.held || C.state === 'gone') return false;
  const name = other?.civ?.name ?? null, up = gd.bubble?.text;
  const pool = HOSTAGE_LINES[moment].filter(t => name !== null || !t.includes('{other}')).map(t => t.replace(/\{other\}/g, name ?? '')).filter(t => t !== up);
  if (!pool.length) return false;
  civSay(g, gd, pick(gd, pool), 0, moment === 'wait' || moment === 'threat' || moment === 'alone');   // (what she keeps saying all mission is over her head only: unseen, it would fill the log from rooms away)
  if (moment === 'aimed' || moment === 'shot' || moment === 'entry') g.audio.play('heldProtest', gd.char.bones.head ?? v3.add(gd.char.pos, [0, 0.4, 0]), 0.5, { rate: C.def.kind === 'worker' ? 1.15 : 1.5 });   // (the one voice buffer the game has for a cry: scream()'s, lower)
  return true;
}
function forgetMeter(g: Game, gd: Guard) { const id = gd.char.id; g.lightForget(`civC${id}`); g.lightForget(`civH${id}`); }

// ================================================================ gates
/** are the civilians on the floor at all right now: not under the showcase tour / the attract loop's tour flag / a puppet beat, not in the breach's daylight */
export function civiliansOnFloor(g: Game): boolean { return g.civilians.length > 0 && !g.quietUtility && !g.touring && !isDaylight(g); }
/** the ones to draw and collide with this frame (on the floor, not gone out of the building) */
export function civiliansShown(g: Game): Guard[] { return civiliansOnFloor(g) ? g.civilians.filter(cv => cv.civ!.state !== 'gone') : NONE; }
const NONE: Guard[] = [];
/** the living ones a round can find and Sam cannot walk through (shown, up, not the one in his arm) */
export function civilianBodies(g: Game): Guard[] { const P = men(g); return civiliansShown(g).filter(cv => cv.state !== 'dead' && cv.char.alive && !P.some(pl => pl.holding?.g === cv)); }
/** per-Game floor-wide memory: control's evacuation line said; the last greeting anywhere; the words / doings due a beat from now */
const floors = new SideMap<object, { evacTold: boolean; greetAt: number; pending: Pending[] }>('civilians.floors');
function floorOf(g: Game) { let F = floors.get(g); if (!F) { F = { evacTold: false, greetAt: -1e9, pending: [] }; floors.set(g, F); } return F; }
/** A word or a doing due at game time `at` — a guard's answer a beat after her line, his radio call after her report and the floor step after that, control's call
 *  after she got out of the door — as DATA, not a closure, so a checkpoint carries the queue (civiliansSnapshot: `cv` by name, `o` by callsign): whose it is
 *  (cv), the man in it (o), and the few facts the doing needs. Dropped by a reset (resetCivilians empties the queue). */
type PendKind = 'greetReply' | 'reportRadio' | 'escalate' | 'outsideCall' | 'complyLine' | 'foundReport' | 'bodyEsc' | 'outrage';
interface Pending { at: number; kind: PendKind; cv: Guard; o: Guard | null; key?: string; armed?: boolean; pos?: Vec3; lvl?: number; dead?: boolean; }

// ================================================================ her voice
/** she says it: a bubble over her head (ui/bubbles.ts draws the civilians' too), and the log under her name when she is off screen — like guards.ts say, in her name */
function civSay(g: Game, gd: Guard, text: string, hold = 0, unheard = false) {
  if (g.snatch?.isMan(gd)) { const own = g.snatch.voice(gd, text); if (own === null) return; text = own; }   // (a snatch's man is not the office's late worker: his own lines by what he is doing, or nothing — game/snatch.ts voice)
  const C = gd.civ!; const t = fill(g, gd, text);
  gd.bubble = { text: t, t: g.time, dur: Math.min(4.5, Math.max(2, 1.5 + t.length * 0.055 + hold)), radio: false };
  C.lastLineAt = g.time;
  if (!unheard && (g.fogOfWar.hidden(gd) || !onScreenHead(g, gd))) g.msg(`${C.name}: ${t}`);   // (fog of war: a hidden man's line is heard, not shown over him)
}
function onScreenHead(g: Game, gd: Guard): boolean {   // (on a player's screen: with two, on either's)
  const p = gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.7, 0]);
  if (g.seats.length > 1) { for (const s of g.seats) if (inView(s.view.viewProj, p)) return true; return false; }
  return inView(g.viewProj, p);
}
function inView(vp: Float32Array | number[] | null, p: Vec3): boolean {
  if (!vp) return false;
  const w = vp[3] * p[0] + vp[7] * p[1] + vp[11] * p[2] + vp[15]; if (w <= 0) return false;
  const x = (vp[0] * p[0] + vp[4] * p[1] + vp[8] * p[2] + vp[12]) / w, y = (vp[1] * p[0] + vp[5] * p[1] + vp[9] * p[2] + vp[13]) / w;
  return x > -0.92 && x < 0.92 && y > -0.9 && y < 0.9;
}
function pick(gd: Guard, pool: string[]): string { const C = gd.civ!; return pool[Math.floor(C.rnd() * pool.length) % pool.length]; }
/** words.ts: this is not the office's floor — the lines that name its fourth floor, its building security, its stairs and lift give way to CIV_LINES.own (false with the
 *  switch off: every line as it was). A pool is swapped or thinned BEFORE the one roll `pick` makes, so her dice run the same either way. */
const away = (g: Game) => WORDS.on && !isOffice(g.level);
/** a pool without its shouts for 'security' where there is no such desk to shout for */
const noSecurity = (g: Game, pool: string[]) => away(g) ? pool.filter(t => !/security/i.test(t)) : pool;
/** {room} {name} {label} {she} {her} {cs} filled off her record (cs = the man in `reportTo` / `foundBy`) */
function fill(g: Game, gd: Guard, text: string, guard?: Guard | null): string {
  const C = gd.civ!; const room = roomName(g.level, C.causePos ?? gd.char.pos); const man = guard ?? C.reportTo ?? C.foundBy;
  const other = g.civilians.filter(o => o !== gd && o.civ).sort((a, b) => b.civ!.screamAt - a.civ!.screamAt)[0]?.civ?.name ?? 'hey';   // ({other}: the civilian who screamed last — the one she is calling out to)
  return text.replace(/\{room\}/g, room).replace(/\{Room\}/g, room[0].toUpperCase() + room.slice(1)).replace(/\{other\}/g, other).replace(/\{name\}/g, C.name).replace(/\{label\}/g, C.label).replace(/\{she\}/g, C.she).replace(/\{her\}/g, C.her).replace(/\{cs\}/g, man?.callsign ?? 'control');
}
/** the SCREAM (or the shout for security): the sound of it, and the stamp the men and the other civilian hear it by */
function scream(g: Game, gd: Guard, shout: string | null) {
  const C = gd.civ!; C.screamAt = g.time;
  const head = gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.6, 0]);
  g.audio.play('heldProtest', head, 0.7, { rate: C.def.kind === 'worker' ? 1.15 : 1.5 });   // (the cry the held man's protest is made of, higher and out loud — the one voice buffer the game has for it)
  if (shout) civSay(g, gd, shout);
  hearScream(g, gd);
}


function setState(g: Game, gd: Guard, s: CivState) { const C = gd.civ!; if (C.state !== s) log(g, gd, `→ ${s}${s === 'flee' && C.target ? ` (${C.target.name})` : ''}${C.cause && (s === 'alarmed') ? ` [${C.cause}${C.armed ? ', armed' : ''}]` : ''}`); C.state = s; C.stateAt = g.time; gd.stallRef = null; }


// ================================================================ perception
/** her eye this frame: head down over a desk / a bin / a screen (a task running at her mark), scanning once frightened, else the walking-about eye */
function eyeOf(g: Game, gd: Guard) {
  const C = gd.civ!;
  if (C.state !== 'work' && C.state !== 'held') return CIV.eyeAlarmed;
  const task = taskAt(gd); return C.state === 'work' && gd.wait > 0 && task !== 'stand' ? CIV.eyeBusy : CIV.eye;
}

// ================================================================ WORK: the loop and its tasks
function taskAt(gd: Guard): CivilianTask { return gd.civ!.def.tasks?.[gd.wp] ?? 'stand'; }
/** the sitting clip (Sitting_Idle_Loop) carries the pelvis this far BEHIND the figure's root (measured, tools/qa clip probe): a mark authored on the chair seat is sat on
 *  by easing the root that far forward along the task's facing while the clip holds him */
export const SIT_ADVANCE = 0.33;
/** game.ts updateProps: a civilian heading for / in his chair is no contact for the furniture (he IS the chair's occupant: it must not skate off ahead of him) */
export function civilianSeated(gd: Guard): boolean { const C = gd.civ; if (!C || C.state !== 'work') return false; if (C.seated) return true; if (taskAt(gd) !== 'sit') return false; const m = gd.route.points[gd.wp % gd.route.points.length]; return v3.distXZ(m, gd.char.pos) < 1.6; }
/** the whole-figure clip a task plays while its wait runs (character.ts `act`), and how long it takes to get up out of it before she may walk */
const TASK_CLIP: Record<CivilianTask, { clip: string; loop?: boolean; rate?: number; in?: number; out?: number; rise?: number } | null> = {
  stand: null,
  wipe: { clip: 'Interact', loop: true, rate: 0.85, in: 0.4, out: 0.4 },          // the reach-and-work of it, over and over along the desk
  bin: { clip: 'Fixing_Kneeling', loop: true, rate: 0.8, in: 0.6, out: 0.7, rise: 0.8 },
  sit: { clip: 'Sitting_Idle_Loop', loop: true, rate: 1, in: 0.7, out: 0.8, rise: 1.0 },
  cooler: { clip: 'Interact', loop: true, rate: 0.6, in: 0.4, out: 0.4 },
  printer: { clip: 'Interact', loop: true, rate: 0.7, in: 0.4, out: 0.4 },
  shelf: { clip: 'PickUp_Table', loop: true, rate: 0.7, in: 0.4, out: 0.5 },
};
/** leaving a mark: the clip comes off (its own fade) and a seated / kneeling woman takes a moment to be on her feet */
function endTask(gd: Guard) { const C = gd.civ!; const K = TASK_CLIP[taskAt(gd)]; gd.char.anim.act = null; C.riseT = C.seated || gd.char.anim.actShown > 0.5 ? K?.rise ?? 0 : 0; C.taskT = 0; C.settleT = -1; if (C.seated) { const f = C.def.face?.[gd.wp] ?? gd.char.bodyYaw; gd.char.pos[0] -= Math.sin(f) * SIT_ADVANCE; gd.char.pos[2] -= Math.cos(f) * SIT_ADVANCE; gd.char.update(0); } C.seated = false; }   // (out of the chair: back onto the mark he stood up from — the seat is a prop his feet cannot stand in)


// ================================================================ ALARMED: the frozen beat, then the run
/** The fright. From any calm-ish state (work, startled, cower for a fresh cause, report / phone interrupted by worse): what and where become what she will tell; an
 *  'armed' cause upgrades a plain one; the men will each be told of THIS fright once (alarmN). She freezes for her beat with a gasp — a scream the floor can hear —
 *  then picks whom to run to. Already running: the cause is folded in (where / armed) and she keeps running. */
export function toAlarmed(g: Game, gd: Guard, cause: CivCause, at: Vec3, armed: boolean) {
  const C = gd.civ!, c = gd.char;
  if (gd.state === 'dead' || gd.held || C.state === 'gone' || C.state === 'evacuate') return;
  const fresh = C.state === 'work' || C.state === 'startled' || C.state === 'cower' || (C.state === 'report' && armed && !C.armed) || (C.state === 'phone' && armed && !C.armed && cause !== 'scream');
  // fold the news in whatever she is doing
  const rank = (k: CivCause | null) => k === null ? -1 : ({ noise: 0, scream: 1, panel: 1, shot: 2, sam: 2, bang: 3, gunfire: 3, body: 4, gun: 5, grabbed: 6 } as Record<CivCause, number>)[k];
  if (rank(cause) >= rank(C.cause) || !C.causePos) { C.cause = cause; C.causePos = v3.copy(at); }
  C.armed = C.armed || armed; C.causeAt = g.time;
  if (!fresh) return;
  if (C.state === 'cower' && C.complied && g.time < C.complyUntil && cause !== 'grabbed') return;   // told to stay down: she stays down (a fresh horror is folded into what she will say if anyone ever asks)
  if (C.state === 'cower' && (C.refled || cause === 'noise' || cause === 'scream')) return;         // a corner is left once, and only for the man himself / a shot / a body — not for another crash
  const wasCower = C.state === 'cower';
  endTask(gd); C.riseT = 0; c.anim.act = null; gd.path = []; gd.pathGoal = null; C.lookAt = null;
  C.alarmN++; C.reported = false; C.acked = false; C.phoned = false; C.reportTo = null; C.badTargets.clear(); C.target = null; C.foundT = 0; C.pinTarget = false;   // (phoned: one call per FRIGHT, not per night — review #5)
  if (wasCower) { C.refled = true; CIV_STATS.refled++; } else C.refled = false;   // (a corner is left once per fright; a fresh fright from her work starts clean — review #8)
  const [f0, f1] = CIV.freeze; C.freezeUntil = g.time + (wasCower ? 0.25 : f0 + C.rnd() * (f1 - f0));
  CIV_STATS.alarmed++;
  if (cause === 'sam' || cause === 'gun' || cause === 'grabbed') { CIV_STATS.samSeen++; chronicleN(g, 'civilianSpotted', 'civSeen', { who: C.name, cause, armed: C.armed }, c.pos); }   // the debrief's row and count: SHE saw him (a body / a shot she only heard is not a sighting of Sam) — the count with the run live whether or not the capped timeline takes the row (review U9.2)
  gd.awareness = Math.max(gd.awareness, cause === 'sam' || cause === 'gun' || cause === 'grabbed' ? 1 : 0.5);
  setState(g, gd, 'alarmed');   // (before the scream: what the other civilian hears of it finds her already frightened, not one to frighten again)
  scream(g, gd, pick(gd, CIV_LINES.gasp[cause] ?? CIV_LINES.gasp.sam));
}

// ================================================================ FLEE: whom to, and the run
function pathMetres(g: Game, from: Vec3, to: Vec3): { m: number; path: Vec3[] | null } {
  const p = g.col.nav.findPath(from, to, 6000); if (!p || !p.length) return { m: Infinity, path: null };
  let s = 0, prev = from; for (const q of p) { s += Math.hypot(q[0] - prev[0], q[2] - prev[2]); prev = q; } return { m: s, path: p };
}
function toFlee(g: Game, gd: Guard, T: CivTarget, evac: boolean) {
  const C = gd.civ!;
  C.target = T; C.retargetAt = g.time + 1.0; gd.path = []; gd.pathGoal = null; gd.stallRef = null; gd.char.anim.act = null; gd.char.anim.crouchTarget = 0;
  goTo(g, gd, T.pos, true);
  const again = C.state === 'flee' || C.state === 'evacuate';   // (re-chosen mid-run: no second shout, no second count)
  setState(g, gd, evac ? 'evacuate' : 'flee');   // (before the shout: what comes back of it through the other civilian's scream finds her already running)
  if (!again) { if (!evac) { CIV_STATS.fled++; scream(g, gd, pick(gd, noSecurity(g, CIV_LINES.flee))); } else civSay(g, gd, pick(gd, CIV_LINES.evac)); }
}



// ================================================================ COWER: the corner
/** the corner: the free nav cell within cornerM of her with the most wall about it (blocked orthogonal neighbours), reachable in a walkable line, away from the fright
 *  for choice; her own cell if nothing better */
function cornerSpot(g: Game, gd: Guard): Vec3 {
  const C = gd.civ!, nav = g.col.nav, cs = nav.cell, from = gd.char.pos, threat = C.causePos;
  const R = Math.ceil(CIV.cornerM / cs); const cx = Math.floor(from[0] / cs), cz = Math.floor(from[2] / cs);
  let best: Vec3 = v3.copy(from), bs = -Infinity;
  for (let dz = -R; dz <= R; dz++) for (let dx = -R; dx <= R; dx++) {
    const x = cx + dx, z = cz + dz; if (x < 1 || z < 1 || x >= nav.W - 1 || z >= nav.H - 1 || nav.blocked[nav.idx(x, z)]) continue;
    const q: Vec3 = [(x + 0.5) * cs, 0, (z + 0.5) * cs]; const d = v3.distXZ(from, q); if (d > CIV.cornerM) continue;
    const walls = nav.blocked[nav.idx(x + 1, z)] + nav.blocked[nav.idx(x - 1, z)] + nav.blocked[nav.idx(x, z + 1)] + nav.blocked[nav.idx(x, z - 1)];
    if (!walls) continue;
    let s = walls * 3 - d * 0.6; if (threat) s += Math.min(6, v3.distXZ(threat, q)) * 0.5;
    if (s <= bs) continue;
    if (!nav.walkable(from, q)) continue;
    bs = s; best = q;
  }
  return best;
}
function toCower(g: Game, gd: Guard, complied: boolean) {
  const C = gd.civ!, c = gd.char;
  C.complied = complied; if (complied) C.complyUntil = g.time + CIV.complyMin * 60;
  C.cowerAt = complied ? v3.copy(c.pos) : cornerSpot(g, gd); C.foundT = 0; C.foundBy = null; C.foundLineAt = -1; C.foundTold = false; C.whimperAt = g.time + 2 + C.rnd() * 4;
  gd.path = []; gd.pathGoal = null; c.anim.act = null; C.target = null;
  CIV_STATS.cowered++; if (complied) CIV_STATS.complied++;
  setState(g, gd, 'cower');
}
/** A RESCUE'S HOSTAGE ON HER FEET (rescueend.ts; and a snatch's man once tied — snatchout.ts): she follows `lead` on the civilians' own legs, stopping `gap` behind him, or
 *  (null) goes down again where she stands. Her state stays the told cower it was (nothing frightens her off it, nobody grabs her, she reports to nobody):
 *  the one branch in updateOne walks her instead of keeping her in the corner. */
export function hostageLead(g: Game, gd: Guard, lead: Character | null, gap: number) {
  const C = gd.civ; if (!C || gd.state === 'dead' || C.state !== 'cower') return;
  C.lead = lead; C.leadGap = gap; C.leadTo = null; gd.path = []; gd.pathGoal = null; gd.stallRef = null;
  if (!lead) C.cowerAt = v3.copy(gd.char.pos);   // (down where she stands: cower() scuttles nobody back to the room)
}
/** a led civilian is sent the last steps to `to` by herself (snatchout.ts: the van's doors), or (null) follows her leader again */
export function civilianLedTo(gd: Guard, to: Vec3 | null) { const C = gd.civ; if (!C || !C.lead) return; C.leadTo = to ? v3.copy(to) : null; gd.path = []; gd.pathGoal = null; gd.stallRef = null; }

// ================================================================ HELD: the arm (player.ts has the hold; these are her rows' doings)
/** player.ts startGrab, the frame the arm goes round her: whatever she was doing stops (the fright itself waits until she is out of it — she cannot scream) */
export function civilianGrabbed(g: Game, gd: Guard) {
  const C = gd.civ!; endTask(gd); C.riseT = 0; gd.char.anim.act = null; gd.char.anim.crouchTarget = 0; C.target = null; C.reportTo = null; C.lookAt = null;
  C.cause = 'grabbed'; C.causePos = v3.copy(g.player.char.pos); C.armed = true; C.causeAt = g.time; C.complied = false;
  log(g, gd, 'grabbed');
}
/** Her answers in the arm (Game.interrogate for a civilian): the first press gets fear; then the thing or two she actually knows, in order (the cleaner: the side door's
 *  digits off their sticky note — Sam LEARNS them, devices.ts learnCode, as off a guard — and who walks where; the worker: where the men come from, who sits where,
 *  the camera); then nothing, however he leans. A DialoguePick in the men's shape so the hold's plumbing reads it unchanged. */
export function civilianAnswer(g: Game, gd: Guard, rnd: () => number = simRandom): DialoguePick | null {
  void rnd;
  const C = gd.civ!; const T = gd.talk; T.stage++; T.presses++; T.lastT = g.time;
  const worker = C.def.kind === 'worker';
  const base = { kind: null as DialoguePick['kind'], cell: `civ:${C.def.kind}`, k: 'saw' as const, band: 'high' as const, t: 'nervy' as const, lineK: '*' as const, widened: '' as const, after: null, script: null, shape: null };
  if (g.snatch?.isMan(gd)) return T.stage <= 1 ? { ...base, text: pick(gd, SNATCH_LINES.held), id: 'civ:fear', stage: 1, stageN: T.stage } : { ...base, text: pick(gd, SNATCH_LINES.dry), id: 'civ:dry', stage: 'x', stageN: T.stage };   // (a snatch's man: his own two lines, and he has nothing to tell)
  if (T.stage <= 1) return { ...base, text: pick(gd, worker ? CIV_LINES.heldFearWorker : away(g) ? CIV_LINES.heldFear.map(t => /CLEAN here/.test(t) ? CIV_LINES.own.heldFear : t) : CIV_LINES.heldFear), id: 'civ:fear', stage: 1, stageN: T.stage };
  const own = away(g), men = own ? g.guards.filter(x => !x.unit && x.team === 'guard').length : 0;   // (words.ts: off the office what she knows is the keypad's digits, where there is one, and how many walk the place — the roster as she saw it come on, whoever is down since; nobody on it: not that)
  const pool = own ? [...(g.devices?.padNear(gd.char.pos) ? [CIV_LINES.own.intelPad] : []), ...(men ? wg(CIV_LINES.guide.own.intel, CIV_LINES.own.intel) : [])] : CIV_LINES.intel[C.def.kind] ?? [];
  if (C.intelN < pool.length) {
    let text = pool[C.intelN]; C.intelN++;
    if (own) { text = text.replace('{n}', CIV_COUNT[Math.min(men, CIV_COUNT.length - 1)]); if (C.intelN >= pool.length) C.intelN = Math.max(C.intelN, CIV_LINES.intel[C.def.kind]?.length ?? 0); }   // (told out: the count as player.ts's row reads 'told you everything' off the office's table)
    let kind: DialoguePick['kind'] = null;
    if (own && text.includes('{code}')) { text = text.replace('{code}', spellWords(g.devices!.padNear(gd.char.pos)!.def.code)); kind = 'code'; }
    if (/seven three nine one/i.test(text)) { const K = g.devices?.padNear(gd.char.pos); if (K) { text = text.replace(/Seven three nine one/i, spellWords(K.def.code)); kind = 'code'; } }   // the level's own digits, and Sam has them (game.ts interrogate → learnCode on kind 'code')
    return { ...base, kind, text, id: `civ:intel:${C.intelN}`, stage: T.stage === 2 ? 2 : 3, stageN: T.stage };
  }
  if (own) C.intelN = Math.max(C.intelN, CIV_LINES.intel[C.def.kind]?.length ?? 0);
  return { ...base, text: pick(gd, CIV_LINES.dry), id: 'civ:dry', stage: 'x', stageN: T.stage };
}
const DIGIT_WORDS = ['oh', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine'];
function spellWords(code: string): string { const w = code.split('').map(d => DIGIT_WORDS[Number(d)] ?? d).join(' '); return w[0].toUpperCase() + w.slice(1); }
/** The 'stay down, stay quiet' row (player.ts updateHold, verb 'comply' on a held civilian): Sam's word over Sam, hers, and she is let go where she stands — down into a
 *  crouch, told: for CIV.complyMin minutes she stays there and reports NOTHING on her own (a man walking onto her still gets it out of her). The one way anybody
 *  comes out of Sam's arm on their feet by his choice; a civilian only. The debrief's row + count. */
export function civilianComply(g: Game): boolean { return crossToRust('civilianComply', undefined, [g]) as any; }
/** combat.ts killGuard's first line hands a civilian here: `how` she went (shot = dead; struck / choked = out cold, breathing), by whose hand. Her state is 'dead' either
 *  way (every 'is she out of the picture' test the hold and the drag make keeps working); by the PLAYER's hand it goes on his card — a knockout priced hard, a death a
 *  'civilian casualty' (chronicle.ts SCORE), and with tune.civilianKillFails the mission fails where he stands. Nothing falls from her hands. The men find her as they
 *  find anybody (guardsNotice). */
export function civilianDown(g: Game, gd: Guard, dir: Vec3, quiet: boolean, how: DownKind, by: DownBy, line?: string) {
  const C = gd.civ!, c = gd.char;
  if (gd.state === 'dead') return;
  if (gd.bubble && g.time - gd.bubble.t < 0.8) g.msg(`${C.name}: ${gd.bubble.text}`);
  gd.state = 'dead'; gd.speed = 0; gd.diedAt = g.time; gd.downKind = how; gd.downBy = by; gd.script = null;
  if (gd.held) { gd.held = null; c.anim.heldPose = null; }
  c.anim.act = null; C.target = null; C.reportTo = null;
  c.ragdollize(g.col, dir, quiet);
  forgetMeter(g, gd);
  const dead = how === 'shot';
  const mark = !!g.snatch?.isMan(gd);   // (a snatch's man: out cold by Sam's hands is how he is taken, not a civilian hurt — snatch.ts words both ways he goes down)
  const hostage = WORDS.on && !!g.rescue?.isHostage(gd);   // (a hostage down is the rescue's to word — rescue.ts outcome: who shot her, and what it ends — not 'the cleaner … they will find her')
  if (by === 'player') {
    if (dead) { CIV_STATS.killed++; chronicleN(g, 'civilianKilled', 'civKilled', { who: C.name, label: C.label, weapon: g.player.gunInHand() ? plainShort(g.player.gunInHand()!.def) : 'a round' }, c.pos); }
    else if (!mark) { CIV_STATS.ko++; chronicleN(g, 'civilianKO', 'civKO', { who: C.name, label: C.label, method: how === 'choked' ? 'choke' : /whip/.test(line ?? '') ? 'whip' : /^ring/.test(line ?? '') ? 'ring' : /behind the wall when it went$/.test(line ?? '') ? 'charge' : 'takedown' }, c.pos); }
    if (dead && g.tune.civilianKillFails && missionLive(g)) { const m = g.mission; m.stage = 'failed'; m.driveT = -1; m.downCardAt = g.missionInTour ? -1 : g.time + DOWN_CARD_SECS; g.msg(`MISSION FAILED — you killed ${C.name}, the ${C.label}. press Enter to restart`); }
    else if (!hostage && !mark) g.msg(line ?? (dead ? `CIVILIAN CASUALTY — ${C.name}, the ${C.label}, is dead. that is on your record, and they will find ${C.her}` : how === 'choked' ? `${C.name} choked out — breathing. a civilian: that costs, and they may find ${C.her}` : `${C.name} out cold — breathing. a civilian: that costs, and they may find ${C.her}`));
  } else if (!hostage && !mark) g.msg(line ?? `${C.name}, the ${C.label}, is down`);
  log(g, gd, `DOWN — ${how} by ${by}`);
  // the other civilian, if it happened in front of her
  const at = c.bones.chest ?? v3.add(c.pos, [0, 1.1, 0]);
  for (const o of g.civilians) if (o !== gd && o.state !== 'dead' && !o.held && o.civ!.state !== 'gone' && civPointInView(g, o, at, CIV.bodyM)) { o.civ!.knownBodies.add(gd); toAlarmed(g, o, 'body', v3.copy(c.pos), true); }
}
function civPointInView(g: Game, gd: Guard, p: Vec3, range: number): boolean {
  const c = gd.char; const eye = c.bones.head ?? v3.add(c.pos, [0, 1.65, 0]); const look = c.chestDir; const E = eyeOf(g, gd);
  const to = v3.sub(p, eye); const L = v3.len(to); if (L > range) return false; if (Math.hypot(to[0], to[2]) < 0.3) return true;
  if (v3.dot([look[0], 0, look[2]], v3.normalize([to[0], 0, to[2]])) < Math.cos(Math.max(45, E.halfDeg) * DEG)) return false;
  return !g.col.segmentBlocked(eye, p);
}

/** her scream / shout, to the OTHER civilian's ears: the same propagation, the same reach as to the men's — what frightened the one frightens the other */
function hearScream(g: Game, gd: Guard) {
  const C = gd.civ!, mouth = gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.6, 0]);
  for (const o of g.civilians) {
    if (o === gd || o.state === 'dead' || o.held) continue; const OC = o.civ!; if (OC.state !== 'work' && OC.state !== 'startled' && OC.state !== 'cower') continue;
    if (v3.dist(o.char.pos, gd.char.pos) > CIV.screamM + 2) continue;
    const pr = hearPath(g, mouth, o.char.bones.head ?? v3.add(o.char.pos, [0, 1.6, 0]));
    if (pr.pathLen < CIV.screamM * pr.carry) toAlarmed(g, o, 'scream', v3.copy(C.causePos ?? gd.char.pos), C.armed);
  }
}



// ================================================================ persistence (one key per machine: the switch and the fail rule)
const LS_KEY = 'twopointsix.civilians';
export function saveCivTune(g: Game) { try { localStorage.setItem(LS_KEY, JSON.stringify({ on: g.tune.civilians, killFails: g.tune.civilianKillFails, greet: CIV.greet })); } catch { /* no storage */ } }
export function loadCivTune(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('loadCivTune', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  try { const j = JSON.parse(localStorage.getItem(LS_KEY) ?? 'null'); if (j && typeof j === 'object') { if (typeof j.on === 'boolean') g.tune.civilians = j.on; if (typeof j.killFails === 'boolean') g.tune.civilianKillFails = j.killFails; if (typeof j.greet === 'boolean') CIV.greet = j.greet; } } catch { /* nothing stored, or not readable */ }   // (no early return: a FRESH profile has no record and must still read the URL flag below)
  try { if (typeof location !== 'undefined' && new URLSearchParams(location.search).has('nocivilians')) g.tune.civilians = false; } catch { /* no location (the harness) */ }   // ?nocivilians: nobody but the shift from boot — the frame and the floor byte for byte as before them — whatever the stored switch says
}
/** the switch flipped live (Tab panel / console): off = they leave the floor this frame (a held / dragged one let go of first); on = fresh ones on their marks */
export function setCivilians(g: Game, on: boolean) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('setCivilians', undefined, [g, on]); if (__r !== RUST_PASS) return __r as any; } } g.tune.civilians = on; saveCivTune(g); resetCivilians(g); }

// ================================================================ panel / QA
/** 'civilians ▸ Marta WORK wipe @wp3 (14 s) aw 0.00 | Owen FLEE → Novak [gun, armed] aw 1.00' */
export function civiliansSummary(g: Game): string {
  if (!g.tune.civilians) return 'civilians ▸ off';
  if (!g.civilians.length) return 'civilians ▸ none authored on this level';
  if (!civiliansOnFloor(g)) return 'civilians ▸ off the floor (the tour / daylight)';
  return 'civilians ▸ ' + g.civilians.map(cv => {
    const C = cv.civ!;
    if (cv.state === 'dead') return `${C.name} DOWN (${cv.downKind}${cv.found ? ', found' : ''})`;
    const bits = [`${C.name} ${C.state.toUpperCase()}`];
    if (C.state === 'work') bits.push(cv.wait > 0 ? `${taskAt(cv)} @wp${cv.wp} (${Math.ceil(cv.wait)} s)` : `→ wp${cv.wp}`);
    if (C.target) bits.push(`→ ${C.target.name}`);
    if (C.reportTo && (C.state === 'report' || C.reported)) bits.push(`told ${C.reportTo.callsign}`);
    if (C.cause) bits.push(`[${C.cause}${C.armed ? ', armed' : ''}${C.reported ? ', reported' : ''}${C.phoned ? ', phoned' : ''}]`);
    if (C.state === 'cower' && C.complied) bits.push(`complied ${Math.max(0, Math.ceil((C.complyUntil - g.time) / 60))} min`);
    bits.push(`aw ${cv.awareness.toFixed(2)}`);
    return bits.join(' ');
  }).join(' | ');
}
/** the panel's / the probes' buttons: frighten one now (as if she had seen him where he stands), or send her to ring it in from the nearest phone */
export function civPanic(g: Game, gd: Guard | undefined, what: 'panic' | 'phone' = 'panic'): string { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('civPanic', undefined, [g, gd, what]); if (__r !== RUST_PASS) return __r as any; } }
  if (!gd?.civ) return 'no such civilian';
  if (!civiliansOnFloor(g)) return 'not now (the tour / daylight)';
  if (gd.state === 'dead') return `${gd.civ.name} is down`; if (gd.held) return `${gd.civ.name} is in your arm`;
  const C = gd.civ; const pp = v3.copy(g.player.char.pos);
  if (what === 'phone') {
    C.cause = 'panel'; C.causePos = pp; C.causeAt = g.time; C.alarmN++; C.reported = false; C.phoned = false; C.badTargets.clear(); endTask(gd); C.riseT = 0; gd.char.anim.act = null;
    let best: CivSpot | null = null, bm = Infinity; for (const s of civSpots(g).phones) { const m = pathMetres(g, gd.char.pos, s.pos).m; if (m < bm) { bm = m; best = s; } }
    if (!best) return `${C.name} cannot reach a phone from here`;
    toFlee(g, gd, { kind: 'phone', guard: null, spot: best, pos: v3.copy(best.pos), name: best.name, planPos: v3.copy(best.pos) }, false); C.pinTarget = true;
    return `${C.name} runs for ${best.name}`;
  }
  if (C.state !== 'work' && C.state !== 'startled' && C.state !== 'cower') return `${C.name} is already ${C.state}`;
  toAlarmed(g, gd, 'panel', pp, false);
  return `${C.name} panics (as if she had seen you where you stand)`;
}
/** headless check of CIV_SPOTS and every civilian's marks against the level (free nav cells; every spot reachable from every civilian's first mark) — the probes print it */
export function checkCivSpots(g: Game): string[] { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('checkCivSpots', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const nav = g.col.nav; const errs: string[] = [];
  const spots = [...civSpots(g).phones, ...civSpots(g).exits];
  for (const s of spots) if (nav.isBlocked(s.pos[0], s.pos[2])) errs.push(`${s.name} (${s.pos[0]}, ${s.pos[2]}) is not a free nav cell`);
  for (const def of g.level.civilians ?? []) {
    def.points.forEach((p, i) => { if (nav.isBlocked(p[0], p[2])) errs.push(`${def.name} mark ${i} (${p[0]}, ${p[2]}) is not a free nav cell`); });
    for (const s of spots) if (!nav.findPath(def.points[0], s.pos, 6000)) errs.push(`${def.name} cannot reach ${s.name} from her first mark`);
    for (let i = 1; i < def.points.length; i++) if (!nav.findPath(def.points[i - 1], def.points[i], 6000)) errs.push(`${def.name}: no path mark ${i - 1} → ${i}`);
  }
  return errs;
}
