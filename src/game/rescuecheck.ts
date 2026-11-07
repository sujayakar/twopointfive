// THE RESCUE'S MEN CHECK IN (defenders answer the plan the team chose: the quiet way in on a hostage rescue).
//
// WHAT WAS THERE: a rescue's cast is its own (rescue.ts: a captor on a clock, a door man, a roamer or two) and none of the squads' radio check runs on it. A
// loud way in lets the roamer off to the noise and the captor shouts his clock; a man of theirs taken without a sound is missed by nobody, for as long as the
// team cares to take.
//
// THE RULE, while a rescue is staged or running and their side is calm (no man of the cast alert or searching):
//   · A RADIO CHECK every `every` seconds (the first after `first`): the captor — or, without him, the first man who can — says "radio check — sound off".
//     A man who can answer has answered. A man who is down and whom nobody of theirs saw fall has not.
//     A man down in the hostage room itself is never asked for: that room is the rescue's own (its clock, its hand-over), and nobody is walked into it.
//   · HE IS ASKED FOR BY HIS POST `askSecs` later ("the break room, sound off … the break room?" — a defender does not say the team's label for a mate:
//     dialogue.ts placeName of where he was posted), and `goSecs` after that ONE MAN GOES THERE: a roamer still on his own post first, else the door man
//     or a guard posted outside the hostage room — "nothing from the break room — one moving there!", at a walk, his boots filed for the team's ears.
//     NEVER the captor and never a man in the room with the hostages (rescue.ts: they are the clock), never a man the rescue has already let off his post
//     or Hard's crew has dealt with, never a man another rule has (powerout.ts powerOutKept: a lane, a press, a dig-in). Nobody free: "nothing from the
//     break room — nobody to spare. Stay on your doors!" and nobody moves.
//   · ON THE WAY AND AT THE POST he looks for any man of theirs who is down and not yet found, as any finder does (guards.ts bodySeen: in his view and lit
//     enough to make out — at the post he looks about him, and a body at his feet is found in any light). FOUND: "man down, the break room — they're
//     inside!", the body is counted and written down as any found body is, he is roused, the room fight's free man and the rescue's 'learned' one like
//     any man of theirs who sees a body, and this rule lets go of him. Nothing there after `waitSecs`: "nothing at the break room — he's gone!", and he
//     walks back to his own.
//   · THE MAN WHO WENT AND DID NOT COME BACK is missed at the next check like any man — asked for where he was going — and the post he never reached is
//     a question again.
//   · A ROAMER SAYS WHERE HE IS GOING: `wordSecs` before the rescue lets a roamer who has learned of the assault off his post (rescue.ts: `setSecs` after he
//     learned) he says so on their net — "heard that — moving to cover it!", or with nothing loud to go to "on my way — moving to the hostage door!" (not a
//     man with the team in his eye: his shouts are his own). When he goes is the rescue's, as it was; a noise that comes after he spoke is said when he
//     goes. HIS WORD IS KEPT: let go with a noise known and nothing of the team seen since he learned, his fix is that noise, as the rescue gives it to a
//     man who was calm — not the doorway its echo reached him through.
//   · A fight anywhere on their side, or a shot, a boot or a charge in his ears on the way: back to his post at a run, and the rule stands down while it lasts.
//     Men who went down while their side was roused are known down: nobody is asked for after a fight.
// THE CLOCK IS NOT TOUCHED: nothing here starts it, speeds it or stops it. A man walking to a post has learned nothing of the team (rescue.ts asks
// `OWN.walk` before it reads his walk as that); what he SEES there is his own.
// No man is added and nobody's eyes, aim or health change; the post he walks to is his mate's own, not where the team is. No dice (the beats are the cast's
// ids), no camera, no graphics setting, no wall clock. State per Game with its pair (rescueCheckSnapshot / rescueCheckRestore through savestate.ts).
// Type-only game imports but for functions used inside calls (src/game is one import cycle).
import { SideMap } from '../math/side';
import type { Game, Guard } from './game';
import { Vec3, v3 } from '../math/vec';
import './squad';
import './pressure';
import { calmSave, calmLoad, type Calm, type CalmSnap } from './powerout';
import './guards';
import './cues';
import './chronicle';
import './dialogue';
import './mission';
import './rescue';
import { ownerWitness } from './owner';
import { RUST_PASS, RUST_SURFACE } from './crossing';

/** the switch and its numbers: off in the module, raised by the page (host/profile.ts pageSwitches; ?norescuecheck = a rescue's men never miss each other,
 *  as it was) and by the shipped stand; the harness's frozen stand leaves it off (TPS_RESCUECHECK=1 turns it on). UNTUNED by play. */
export const RESCUECHECK = {
  on: false,
  /** the first check after the staging, and the gap between checks, s (where in the band: the caller's id and the check's number — no dice) */
  first: [14, 22] as [number, number], every: [24, 36] as [number, number],
  /** the breath a man is given to answer before he is asked for by name · the breath after it before a man goes */
  askSecs: 4, goSecs: 4,
  /** how long the man who went stands at an empty post before he says so and walks back */
  waitSecs: 5,
  /** legs of a walk tried again before it is given up (the room fight ends a leg at ten seconds) */
  tries: 4,
  /** how far round his feet the man at the post finds a body in any light, m */
  feetM: 2.5,
  /** a roamer's word this long before the rescue lets him off his post */
  wordSecs: 2.5,
};

type Phase = 'out' | 'there' | 'back';
interface Out { gd: Guard; about: Guard; goal: Vec3; from: Vec3; phase: Phase; thereT: number; tries: number; stepT: number; heard0: number; calm: Calm }
export interface RescueCheckEvent { t: number; what: 'check' | 'asks' | 'goes' | 'nobody' | 'there' | 'found' | 'empty' | 'home' | 'abort' | 'lost' | 'word' | 'kept'; who: string; why: string }
interface State { nextAt: number; n: number; stage: 'idle' | 'called' | 'asked'; t0: number; caller: Guard | null; about: Guard | null; known: Set<Guard>;
  /** where a man who went out was going (he is looked for there, not on the post he left) */ where: Map<Guard, Vec3>;
  /** what a roamer has said of where he is going · the roamers whose letting-go this rule has seen */ worded: Map<Guard, 'noise' | 'door'>; kept: Set<Guard>; out: Out | null; events: RescueCheckEvent[] }
const STATES = new SideMap<Game, State>('rescuecheck.STATES');
ownerWitness<Game>('walk', (gd, g) => walking(g, gd), true);

const alive = (gd: Guard) => gd.state !== 'dead' && gd.char.alive && !gd.fled;
const roused = (gd: Guard) => gd.state === 'alert' || gd.state === 'search';

/** rescue.ts defenders(): this man is out on the check's errand and has noticed nothing — his walk is not word of the team (a thing he hears on the way is:
 *  the rescue's own test, a man no longer on patrol or with something in his ears, reads him then as it reads any man) */
const walking = (g: Game, gd: Guard) => RESCUECHECK.on && STATES.get(g)?.out?.gd === gd && !roused(gd) && gd.state !== 'suspicious' && gd.awareness < 0.3;
/** the check's and the debugger's timeline */
export function rescueCheckTimeline(g: Game): RescueCheckEvent[] { return STATES.get(g)?.events ?? []; }
/** a staging, a restart (game.ts resetGuards) */
export function rescueCheckReset(g: Game) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('rescueCheckReset', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } } STATES.delete(g); }




// ---------------------------------------------------------------- the state's pair (new module state the game depends on rides a snapshot)
export type RescueCheckSnap = { nextAt: number; n: number; stage: State['stage']; t0: number; caller: number; about: number; known: number[]; where: [number, Vec3][]; worded: [number, 'noise' | 'door'][]; kept: number[];
  out: { i: number; about: number; goal: Vec3; from: Vec3; phase: Phase; thereT: number; tries: number; stepT: number; heard0: number; calm: CalmSnap } | null };
export function rescueCheckSnapshot(g: Game): RescueCheckSnap | undefined {
  const st = STATES.get(g); if (!RESCUECHECK.on || !st) return undefined;
  const i = (gd: Guard | null) => gd ? g.guards.indexOf(gd) : -1, M = st.out;
  return { nextAt: st.nextAt, n: st.n, stage: st.stage, t0: st.t0, caller: i(st.caller), about: i(st.about), known: [...st.known].map(i).filter(k => k >= 0), where: [...st.where].map(([m, p]): [number, Vec3] => [i(m), v3.copy(p)]).filter(e => e[0] >= 0), worded: [...st.worded].map(([m, w]): [number, 'noise' | 'door'] => [i(m), w]).filter(e => e[0] >= 0), kept: [...st.kept].map(i).filter(k => k >= 0),   // (the timeline is the check's and the debugger's: not kept)
    out: M ? { i: i(M.gd), about: i(M.about), goal: v3.copy(M.goal), from: v3.copy(M.from), phase: M.phase, thereT: M.thereT, tries: M.tries, stepT: M.stepT, heard0: M.heard0, calm: calmSave(M.calm) } : null };
}
export function rescueCheckRestore(g: Game, s: RescueCheckSnap | null | undefined) {
  if (!s) { STATES.delete(g); return; }
  const at = (k: number) => g.guards[k] ?? null, o = s.out, gd = o ? at(o.i) : null, about = o ? at(o.about) : null;
  STATES.set(g, { nextAt: s.nextAt, n: s.n, stage: s.stage, t0: s.t0, caller: at(s.caller), about: at(s.about), known: new Set(s.known.map(at).filter((x): x is Guard => !!x)), where: new Map(s.where.filter(e => at(e[0])).map(([k, p]): [Guard, Vec3] => [at(k)!, v3.copy(p)])), worded: new Map(s.worded.filter(e => at(e[0])).map(([k, w]): [Guard, 'noise' | 'door'] => [at(k)!, w])), kept: new Set(s.kept.map(at).filter((x): x is Guard => !!x)), events: [],
    out: o && gd && about ? { gd, about, goal: v3.copy(o.goal), from: v3.copy(o.from), phase: o.phase, thereT: o.thereT, tries: o.tries, stepT: o.stepT, heard0: o.heard0, calm: calmLoad(g, o.calm) } : null });
}
