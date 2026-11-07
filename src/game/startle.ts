// ENEMIES STARTLE WHEN THEY FIRST SEE YOU (the maintainer: 'we should also have startle animations when unaware enemies see you for the first time. a quick
// beat and then they pull out their gun or run').
//   · WHO: a man hostile to the player who was CALM (on his round, at his post: state 'patrol') no more than `fresh` seconds ago and whose own EYES are on
//     a man now that the fight AI has him alert — whether his eyes took him there (straight from calm, or through the 'huh?' a plain sight passes through)
//     or a shot did, a moment ahead of the man who fired it coming through the door. Never a man who has been looking for somebody for longer than that
//     (suspicious, searching, roused and waiting): he is not surprised. Never a man blinded or folded (that is its own state), a choreographed one, a
//     civilian (civilians.ts has her own fright), a man of the player's side. Once a man.
//   · THE BEAT (`beat` seconds): he JOLTS (character.ts startleFrom: reared back off the thing, head snapped round to it, arms flung out), his feet stay where
//     they are and whatever was in his hand stays in it, DOWN — the torch still the torch, a rifle at the low ready. No round leaves him: the beat is a floor
//     under the fight AI's own reaction clock (Guard.reactT), laid again every frame of it (guards.ts, beside the blinded man's), which already holds every
//     round, and where that clock is longer (the room fight's surprise) the start plays at its head and the gun comes up inside what is left of it. The
//     'CONTACT!' his first sight would have put over his head waits for the end of it: he shouts as he draws, and a man dropped inside it has called nothing.
//   · THEN HE DRAWS, OR RUNS. Draws: the gun comes up, and for `draw` seconds of that no round leaves it; then the fight as it was. RUNS (`run`, and only where the nerve rule is on: morale.ts): a GREEN man
//     (trainingOf: by his squad's key, the difficulty's mix, the night staff's temperament) who is ALONE (that file's own test: no living man of his side near
//     him in his view) with the man he saw inside `runM` — standing to draw there is losing. A man on a post of the room fight's goes the way that file sends
//     a man whose nerve has gone (out by a door nothing has come at, to hold it from the far side — else to cover: roomfight.ts bolt, with that file's own
//     shout); any other man turns and runs, bent low and his hands down, for the cover the suppression model picks off that man's line (coverFrom), holds it
//     the few seconds a man shifted under fire holds his, and fights from there. Nowhere to go: he draws.
//   · giveStartle is the START alone, for anybody who has one to give (a door opened into a man): the jolt, his gun down, and — for a man the fight AI has
//     looking or fighting (suspicious, alert) — his feet still and no round for it; nothing decided after it. A man still on his round walks on under the
//     jolt: rousing him is the caller's. It asks no switch of this file's — the caller's own rule is what is on or off.
// By behaviour, never by hit points; nothing is written or drawn for the player but the man himself. This file draws no die and reads no wall clock: the
// clocks are game seconds. The switch is off in the module and on the frozen stand: with it off (and nobody calling giveStartle) no man has a record here and
// every seam returns on its first lines. Like its neighbours it stands aside for a show on the floor and the utility stages. Type-only game imports but for
// the functions below, used inside calls only (src/game is one import cycle).
import { SideMap } from '../math/side';
import type { Game, Guard, GuardState, GuardScript } from './game';
import { Vec3 } from '../math/vec';
import './teams';
import './morale';
import './suppression';
import './guards';
import { claimMan, releaseMan, ownerWitness, OWN } from './owner';
import { crossToRust, goneWithTick } from './crossing';

/** the switch and its numbers: off in the module, raised by the page (host/profile.ts pageSwitches; ?startle=0 = a man who sees you goes for his gun at once,
 *  as he did). `beat` s of the start; `draw` s after it for the gun to come up before a round can leave it (a man who draws: his feet still, his gun rising); `fresh` s since he last stood calm inside which a sighting still catches him unaware; `late` s past the end of a beat
 *  after which draw-or-run is no longer asked (something else had him meanwhile: he is in the fight by now); `run`: a green man alone, caught inside `runM`
 *  m and not inside `nearM` (at arm's length nobody turns his back: he draws), runs for it instead of drawing; `clearSecs`: how long a man running off a post is let run before the man he ran from can stop him to fight; `runSecs`:
 *  the longest a run for cover lasts, and `stuckSecs` gaining under `stuckM` m ends it where he is */
export const STARTLE = { on: false, beat: 0.5, draw: 0.3, fresh: 1.0, late: 0.25, runM: 8, nearM: 2.5, clearSecs: 1.5, runSecs: 4, stuckSecs: 0.75, stuckM: 0.3 };
/** QA: men startled by what they saw · starts given by a caller (giveStartle, all of them) · of the first: drew after the beat · ran */
export const STARTLE_STATS = { seen: 0, given: 0, drew: 0, ran: 0 };

interface St {
  /** when he last stood calm, and whether the hand torch was in his hand then */
  calmT: number; calmTorch: boolean;
  /** his suspicion began with his own eyes on a man (the frame he left calm) */
  bySight: boolean;
  /** when he was last startled (−1: never), until when the beat holds him (−1: not held), and where the thing was */
  at: number; until: number; toward: Vec3;
  /** the gun is held down ahead of the start (a plain sight on its way to contact) */
  pre: boolean;
  /** the torch stays in his hand while he is held */
  torch: boolean;
  /** after the beat he draws or runs (a first sight: not a start somebody else gave him) */
  choose: boolean;
  /** his 'CONTACT!' was taken off him at the start: a man who draws shouts it as the gun comes up */
  call: boolean;
  /** the gun is coming up after a start (a man who drew): no round before this (−1: none) */
  drawUntil: number;
  /** RUNNING for cover (a man on no post): where to and since when; where he was `refT` ago (a run that gains nothing ends); the order his body follows */
  run: Vec3 | null; runAt: number; ref: Vec3 | null; refT: number; script: GuardScript | null;
}
const stateOf = new SideMap<Guard, St>('startle.stateOf');
ownerWitness('startle', gd => !!stateOf.get(gd)?.run);
const blank = (): St => ({ calmT: -100, calmTorch: false, bySight: false, at: -1, until: -1, toward: [0, 0, 0], pre: false, torch: false, choose: false, call: false, drawUntil: -1, run: null, runAt: -1, ref: null, refT: -1, script: null });

/** THE START, for any caller: he jolts toward `at`, and for `secs` his gun stays down and — a man looking or fighting — his feet are still and no round
 *  leaves him (startleLeft floors the fight AI's reaction clock each frame). `torch`: the hand torch was in his hand and stays there for it (left out: asked
 *  of him as he stands). False = not given: a dead or held man, a civilian, a man of the runner's, or one still in a start. */
export function giveStartle(g: Game, gd: Guard, at: Vec3, secs = STARTLE.beat, torch?: boolean): boolean { return crossToRust('giveStartle', undefined, [g, gd, at, secs, torch]) as any; }

/** guards.ts updateGuard, right after the state transitions (`prev`: his state as the frame found it; `seen`: his eyes are on his man this frame): the end of
 *  a beat, the calm he last stood in, and the first sight itself */
export function startleSight(g: Game, gd: Guard, prev: GuardState, seen: boolean): void { return crossToRust('startleSight', undefined, [g, gd, prev, seen]) as any; }

/** guards.ts updateGuard, at the pose write: his gun is held down this frame (in the start, a plain sight on its way to one, or the frame he turns to run) */
export function startleGunDown(g: Game, gd: Guard): boolean { return goneWithTick('startleGunDown') as any; }
/** guards.ts handCarry: the hand torch is still what he carries (he had it when the start took him). A man grabbed inside it is a man grabbed with his
 *  torch, as on his round: it is what the arm knocks out of his hand, and his until the first frame he is his own again */
export function startleTorch(gd: Guard): boolean { const S = stateOf.get(gd); return !!S && S.torch && (gd.state === 'alert' || gd.state === 'suspicious') && !gd.script; }
/** is he in a start right now (anybody's: QA, and a caller that would not give two) */
export function startled(g: Game, gd: Guard): boolean { const S = stateOf.get(gd); return !!S && g.time < S.until; }
/** is he running for cover off a start (QA; a post man's run is the room fight's own move) */
export function startleRunning(gd: Guard): boolean { return !!stateOf.get(gd)?.run; }


// ---------------------------------------------------------------- checkpoints (game/savestate.ts): each roster man's record by roster index
export interface StartleSnap { men: (number[] | null)[] }
/** nothing (no key in the file) while no man has a record */
export function startleSnapshot(g: Game): StartleSnap | undefined {
  const men = g.guards.map(gd => { const S = stateOf.get(gd); return S ? [S.calmT, S.calmTorch ? 1 : 0, S.bySight ? 1 : 0, S.at, S.until, S.toward[0], S.toward[2], S.pre ? 1 : 0, S.torch ? 1 : 0, S.choose ? 1 : 0, S.call ? 1 : 0, S.drawUntil,
    ...(S.run ? [S.run[0], S.run[2], S.runAt] : [])] : null; });
  return men.some(m => m) ? { men } : undefined;
}
/** …onto the fresh roster (a file without the slice: nobody has one). The jolt itself is not replayed: a man loaded inside his beat stands it out, and one
 *  loaded on the run runs on */
export function startleRestore(g: Game, s: StartleSnap | null | undefined) {
  const f = (x: unknown, d = 0) => typeof x === 'number' && Number.isFinite(x) ? x : d;
  g.guards.forEach((gd, i) => { const M = s?.men?.[i];
    releaseMan(gd, OWN.startle, g.time); if (!M) { stateOf.delete(gd); return; }
    if (M.length >= 15) claimMan(gd, OWN.startle, g.time);
    stateOf.set(gd, { ...blank(), calmT: f(M[0], -100), calmTorch: !!M[1], bySight: !!M[2], at: f(M[3], -1), until: f(M[4], -1), toward: [f(M[5]), 0, f(M[6])], pre: !!M[7], torch: !!M[8], choose: !!M[9], call: !!M[10], drawUntil: f(M[11], -1),
      ...(M.length >= 15 ? { run: [f(M[12]), 0, f(M[13])] as Vec3, runAt: f(M[14], -1) } : null) }); });
}
