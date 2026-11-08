// RAILS: a tutorial mission's frame over a lesson (lesson.ts). The maintainer, of a better tutorial: "we want progressive disclosure … and also really
// want them to be on rails and hard to get lost", "has to be very linear and obvious what to do … also disable all features that player isn't supposed to
// do", and of its checkpoints: "we should also very intentionally curate checkpoints".
//
// A level's `scenarios.lesson` block carries a `rails` record (scene/scenarios.ts RailsData) and its steps a few more keys; a level without the record is
// the lesson it always was, and every entry point here returns on its first line. With the record, and the switch on:
//   · LOCKED ACTIONS. What the player can do is a set of ACTIONS (ACTS below: `move`, `order`, `word`, `fire` …): `rails.allow` from the first step, plus
//     each step's own `allow` from the moment it comes up, less its `deny`. A key, a button or the wheel of an action not in the set is taken out of the
//     tick's input before anything reads it (Game.step, right after the record is loaded: the record itself, the tape and a replay keep what was pressed;
//     the gate is a function of the game's state, so a replay drops what the run dropped). The page's own keys (pause, the back key, Enter) are never locked.
//     The game asks for fixed key CODES and a rebind happens at the keyboard (host/keymap.ts), so the lock follows whatever key a row is on.
//   · THE SET IS DERIVED, never kept: the step that is up says it all, so a checkpoint put back or a snapshot holds nothing of it.
//   · A CUE. A step's `cue` is sent to the level's script (director.ts, its play program 'tutorial') as the signal `lesson:<cue>` the tick the step
//     comes up; the lesson test `{ is: 'cue', name }` is true once the program has raised `done:<name>`. The program is started here when the run
//     starts, and after a restart.
//     Every order the player gives reaches the program too, as the signal `order:<id>`.
//   · AUTHORED CHECKPOINTS. A step's `checkpoint` asks for one save the tick it comes up (savestate.ts Checkpoints.keep: kept in memory, whatever is on
//     the floor), and on a rails level those are the only saves: the every-step request and the floor's own triggers stand down.
//   · NOBODY IS LOST. `rails.respawn` seconds: a teammate who would have gone down is on a knee for that long and then up again (railsSpare, asked by
//     combat.ts killGuard) — by behaviour, no hit points.
//   · FOG BY THE STEP. `rails.fog` / a step's `fog`: the fog of war as the newest step that says so has it.
//   · THE HOLD IS ITS OWN LINE (a step's `noTap`, `menuLine`): while a step teaches the command key held, a tap of it gives no order and a few words come up
//     under the line; with the ring up the line is what to press in it. A step's `eye` is one more lookout for the fog while it is up (the squad's own charge
//     on a wall, which its men stand with their backs to); a step's `onWay` hands the view back to the squad once its order is given.
// ALL OF THIS IS THE RUST GAME'S (rust/game_rules/src/rails, src/lesson): the step that is up, what it locks, its line, its card. What a HUD is told of it is the
// view's `lesson` section (src/view/seatview.ts: `game.view.lesson.rails`): the prompt is the one line the page draws big (ui/overlay.ts) in place of the small
// objective row. What is left here: the knobs, the level file's lint (railsFaults), and the frame's record as the bridge fills it.
// The switch `RAILS.on` and its word `?rails=0` are knobs the Rust game does not read: a rails level is on rails. No shipped level but the tutorial group's carries the record.
// (src/game is one import cycle: nothing here is built from another game module's exports at module load.)
import type { Game, Guard } from './game';
import type { LessonData } from '../scene/scenarios';
import type { Vec3 } from '../math/vec';
import type { Input } from '../host/input';
import type { FollowCamera } from '../host/camera';
import { lessonTokenFaults } from './lesson';   // (called inside railsFaults only: src/game is one import cycle)
import { RUST_PASS, RUST_SURFACE, goneWithTick } from './crossing';

export const RAILS = { on: false,
  /** the cursor this near (px) the step's mark on the screen is ON the mark's place, whatever stands between the eye and it (orders.ts targetUnder) */
  snapPx: 40,
  /** a step's `onWay`: a man of the squad under an order whose mark is within this many metres of the step's own is on his way to it (0: never) */
  wayM: 6,
  /** a step's `onWay.run`: those men RUN the leg ("the guys should sprint back to exfil when they're done": the mission's own order, one more reason of
   *  teampace.ts — a hurt man and a quiet one still walk). false: they walk it, as it was */
  wayRun: true,
  /** a step's `after`: seconds its line stands in for the step's own once the order it asks for has gone out and its men stand where it sent them (while one
   *  is still walking the line stays) — then the step asks again: the men could not do it, or it was the wrong place, and a line that only ever says 'they
   *  are on it' is a player left waiting. 0: never shown */
  afterSecs: 8,
  /** …and however long they walk, the step asks again this many seconds after the order went out (men who cannot get there never arrive) */
  afterMax: 45,
  /** the big line goes to the bottom edge from over the player and his teammates where they stand under it (ui/railclear.ts; picture only) */
  lineClear: true,
  /** a step's mark is not drawn where a ring lies over it, the squad's or his own (ui/overlay.ts asks ui/cmdring.ts `covers`; picture only): the mark
   *  is on the spot a ring opens on, so its circle lay over the ring's middle and the digits of its inner wedges — and with the ring up there the
   *  place is found, and the line says what to press (`?markclear=0`; a knob of the frame's table like lineClear, and the frame is off on the
   *  frozen stand) */
  markClear: true,
  /** a step's own eye is never the one the fog of war has no place for: with the places full (four men up and the seat's lookout) it takes the last
   *  (fogofwar.ts update; before, it was the one dropped, and the squad's own wall charge went off unseen) */
  eyeFirst: true,
  /** THE FRAME'S EYES ARE THE PLAYER'S VIEW, NOT THE SQUAD'S KNOWLEDGE (fogofwar.ts update): the seat's lookout and a step's eye light the floor they look at
   *  and put a man there in the picture — drawn, as a man the squad is shooting at is — and are no answer to the fog's seen() or sighting(), which the
   *  squad's own choices read (the entry on a door: autopilot.ts contactBehind; the rescue's flow; the plan's marks; a charge's known men). The team's
   *  own eyes answer those exactly as on a level with no frame, the one a step's eye takes the picture's last place from included. false: a frame's eye is
   *  one more of the team's (as it was: with a step's lookout in a hall the squad then entered, the stack at its door threw its last can at a man nobody
   *  of the team had seen, and went in three seconds after the pair at the back) */
  eyeView: true,
  /** A CARD LANDS A BEAT AFTER WHAT WAS ON SCREEN ("we show the modals too quickly. like it should happen a beat after something happens on screen … my squad
   *  covered the gate but then seeing them shoot the bad guys got interrupted by the modal"): a card pauses the world the tick its step comes up, and a
   *  step comes up the tick the deed before it is done — the middle of whatever that deed set off. So THE STEP BEFORE A CARD SETTLES: its deed done, the
   *  lesson stays on it (lesson.ts tick asks `settles`) for `cardBeatSecs` seconds. Then the next step comes up with its card, as it always did: its
   *  keys, its cue, its checkpoint and its mark with it, nothing ahead of the card (so all of them are that beat later than the deed: THIS MOVES TICKS).
   *  Meanwhile the done step's line is what there is to watch — its own line where the step only waited for the level's script, else what its men are
   *  doing (its `after` line, when that asks for no key) — or nothing at all. What he does in the beat is the done step's: the next step counts from
   *  the tick it comes up, as every step does. The beat is the step's whether or not the page then shows the card (one read already, ?cards=0, a run
   *  with no page). TWO WORDS ARE THE CARD'S OWN: `explain.quiet` — the deed before this card sets off a fight he
   *  is there to watch (his squad sent at a gate, into a room), so the card also waits while that fight is on: a round of his own or his squad's, a
   *  blast, a door kicked in or a man going down in the last `cardQuiet` seconds, never past `cardMax` seconds from the deed; and `explain.now` — a card
   *  that must cut in, no beat at all. The fight's wait is asked for and not the rule: where he is the one shooting and the next step is his next
   *  move in that fight, a card held for his own rounds held its keys and its cue with it. OFF here and on the frozen stand, where the next step comes
   *  up the tick the deed is done, as it was (TPS_CARDBEAT=1 raises it there, with the frame); ON in the page (profile.ts; ?cardbeat=0) */
  cardBeat: false, cardBeatSecs: 1.2, cardQuiet: 2, cardMax: 9,
  /** TO THE ENEMY A SPARED MAN IS A MAN DOWN (a recorded play of the first mission, 10-24: a teammate took the round that would have killed him, went on
   *  his knee for the level's `respawn` seconds, and the defender who had shot him kept firing into him the whole time). While he is on his knee nobody
   *  of theirs is on him: the man whose foe he was comes off him as off a man he has dropped (guards.ts acquireFoe: the next enemy in his sight, else
   *  his wary look round), and nobody takes him for a new one (acquireFoe; a moving defender's stop to fight, roomfight.ts; the response's eye, qrf.ts;
   *  a colleague's man, tactics.ts; the men a defender counts for a grenade, frag.ts). The defender says nothing of it and a burst he had begun ends.
   *  Up again he is seen and fought like anybody. THIS MOVES FIGHTS on a mission with `respawn`. OFF here and on the
   *  frozen stand (TPS_SPAREDOWN=1 raises it there, with the frame); ON in the page (profile.ts; ?sparedown=0) */
  spareDown: false,
  /** A MISSION MAY TAKE THE FLAT RING (the maintainer, 10-23, of the first mission's wall charge — the ring, [5], and then a second ring of one row
   *  under a line that asked for [3]: "weird after hitting 5"). A tutorial mission's ring was the menu's pages, because its lines name their digits;
   *  one whose level says `rails.flatRing` has the ring the rest of the game has (cmdring.ts CMDRING.flat: every order of the thing on one ring, with
   *  the level's `rails.orders` its only rows), and the step's line while that ring is up is its `flatLine`. false: the pages on every tutorial
   *  mission, as built. Off here and on the frozen stand (`TPS_RAILFLAT=1`, which acts only with the frame and the flat tree up: `TPS_RAILS=1`
   *  `TPS_QRING=1 TPS_QONLY=1 TPS_RINGFLAT=1`); the page raises it (`?railflat=0` off) */
  flatRing: false,
  /** seconds a spared teammate is given when the level names none */
  spareSecs: 6 };
export const RAILS_STATS = { dropped: 0, cues: 0, starts: 0, kept: 0, spared: 0 };

/** an action → the key codes it owns (the game's own codes: host/keymap.ts moves keys, never codes), its mouse buttons (bits of Input.buttons) and the wheel */
interface Act { keys?: string[]; buttons?: number; wheel?: boolean }
const ACTS: Record<string, Act> = {
  move: { keys: ['KeyW', 'KeyA', 'KeyS', 'KeyD'] },
  sprint: { keys: ['ShiftLeft', 'ShiftRight'] }, crouch: { keys: ['KeyC'] }, wall: { keys: ['KeyQ'] }, pace: { wheel: true },
  aim: {}, fire: { buttons: 1 }, alt: { buttons: 4 }, reload: { keys: ['KeyR'] }, mode: { keys: ['KeyZ'] },
  'slot:pistol': { keys: ['Digit1'] }, 'slot:rifle': { keys: ['Digit2'] }, 'slot:smoke': { keys: ['Digit3'] }, 'slot:flash': { keys: ['Digit4'] }, 'slot:frag': { keys: ['Digit5'] },
  // (codes, as everywhere here: the hands' own slot is 'Digit0' whatever key the board gives it — handslot.ts; the whistle's code has no key on the page
  //  while the player's own ring is on — handring.ts: there it is the ring's last wedge, and the ring is `use`'s, worked with its digits and a click)
  // (`stalk` has no key: it is the held left click that walks him up behind a man — stalk.ts, the gate's own line for it. THE ONE ACTION THAT IS OPEN
  //  UNLESS A STEP DENIES IT — `shuts`, below — because it rides with `use` wherever a mission names the grab's row, and a level that never says the
  //  word plays as it did. A step denies it where a man must not be gone for by hand, and keeps the interaction key)
  stalk: {},
  hands: { keys: ['KeyE', 'Digit0'] }, use: { keys: ['Space', 'ArrowUp', 'ArrowDown'] }, whistle: { keys: ['KeyV'] }, light: { keys: ['KeyL'] }, nvg: { keys: ['KeyN'] },
  order: { keys: ['KeyB'] }, ring: {},
  // (`ringPart`: THE WHEEL FOR A PART ONLY. The command key kept down is the ring with Shift down, for the men a Shift tap would have sent; with no Shift the key is seen
  //  let go, so the press is a tap, which on a `noTap` step gives nothing and shows the note. So a step can teach the part's wheel with everyone's shut. Needs `goal`: railsFaults)
  ringPart: {},
  menu: { keys: ['KeyX'], buttons: 4 }, who: { keys: ['KeyT'] }, word: { keys: ['KeyF'] }, tools: { keys: ['KeyG', 'KeyH'] },
  // (Shift with the command key ADDS A GOAL — planmode.ts planGoal: a part of the squad takes it and the rest go on as they were. No key of its own: the
  //  pair is the command key's and his sprint's. A lesson that lists `goal` on a step has goals from that step — planmode.ts goalsOn asks `locks('goal')` —
  //  and until then Shift with the key is the plain key, as on every lesson that never lists it. With the goal rule itself off (CMDRING.goals) the pair is
  //  the plain key on that step too. A lesson that opens `goal` gives `sprint` as well: railsFaults)
  goal: {},
  // `twice`: the squad's key tapped twice is 'fall in' (cmdring.ts `twice`; orders.ts twiceOn asks `locks('twice')`). No lesson lists it today, so on
  //  a tutorial mission a double press is two presses: it cannot undo the order a step has just asked for
  twice: {},
  camera: {}, zoom: {},
  // (ownblow.ts: a charge his own hands set, set off from his own list — no key of its own: the row is `use`'s, and a mission that names this action or
  //  the squad's word has that row from the step that opens either)
  blow: {},
  // (a mission that hands him two loads and has him swap before he fires: the setup's load row with the right button still shut. No key of its own: the
  //  row's digit is the row's — `gate`, below — and off the setup the swap is the rifle's own key)
  load: {},
};
/** the most a step's card may say (its `explain.text`): three short sentences */
const CARD_CHARS = 260;
export const RAILS_ACTIONS: readonly string[] = Object.keys(ACTS);

/** A ROWS LIST THAT TAKES AN ACT BACK OUT OF HIS HANDS (`rails.rows`): a list names the verbs a mission teaches, and every other row is withheld — the act that
 *  puts a listed one back among them. He opens a door with the taught row and nothing in reach shuts it (the maintainer on the stealth mission: "nothing
 *  in reach to close the door?"). Each pair here is [an act, the act that undoes it]; `rowsWithheld` says which a list names without its other half. Notes,
 *  not faults: a mission may mean it (a door that must stand open), and a row added beside a taught one can take Space's tap from it — the list is its
 *  author's to mend. tools/qa/lesson-check.ts prints them */
const UNDO: readonly (readonly [string, string, string])[] = [['push', 'pull', 'a door he opens cannot be shut'], ['ease', 'pull', 'a door he eases open cannot be shut'],
  ['drag', 'drop', 'a body he drags cannot be let go'], ['lightsOff', 'lightsOn', 'a room\'s lights he puts out cannot be put on'], ['breakerOff', 'breakerOn', 'the mains he cuts cannot be put back']];
export function rowsWithheld(L: LessonData | null | undefined): string[] {
  const R = L?.rails?.rows; if (!Array.isArray(R)) return [];
  return UNDO.filter(([a, b]) => R.includes(a) && !R.includes(b)).map(([a, b, why]) => `rails.rows names '${a}' and not '${b}': ${why}`);
}
/** what is wrong with a lesson block's rails keys (lesson.ts checkLesson adds these to its own) */
export function railsFaults(L: LessonData): string[] {
  const out: string[] = [], R = L.rails; if (!R) { L.steps.forEach((T, i) => { if (T.allow || T.deny || T.cue || T.checkpoint || T.fog !== undefined || T.noTap || T.eye || T.onWay || T.explain || T.toward || T.after) out.push(`steps[${i}] '${T.id}' carries rails keys and the lesson has no rails record`); }); return out; }
  const bad = (w: string, a: unknown) => { if (a === undefined) return; if (!Array.isArray(a)) { out.push(`${w}: a list of actions`); return; } for (const x of a) if (!ACTS[x as string]) out.push(`${w}: no action '${String(x)}' (${RAILS_ACTIONS.join(', ')})`); };
  bad('rails.allow', R.allow);
  if (Array.isArray(R.allow) && R.allow.includes('stalk')) out.push("rails.allow: 'stalk' is open unless a step denies it, so naming it in the frame's list means nothing (a step's deny takes it, a later step's allow gives it back)");
  const cues = new Set<string>(), open = new Set<string>(Array.isArray(R.allow) ? R.allow : []); let goalSaid = false, partSaid = false;
  L.steps.forEach((T, i) => { const w = `steps[${i}] '${T.id}'`; bad(`${w}.allow`, T.allow); bad(`${w}.deny`, T.deny);
    // (`goal` owns no key: Shift is `sprint`'s, and the gate takes it out of the tick on a step that does not let him run — so a goal with no sprint is a pair nobody can press)
    if (Array.isArray(T.allow)) for (const a of T.allow) open.add(a); if (Array.isArray(T.deny)) for (const a of T.deny) open.delete(a);
    if (open.has('goal') && !open.has('sprint') && !goalSaid) { goalSaid = true; out.push(`${w}: 'goal' is open here and 'sprint' is not — goal needs sprint: Shift is his sprint's key`); }
    if (open.has('ringPart') && !open.has('goal') && !partSaid) { partSaid = true; out.push(`${w}: 'ringPart' is open here and 'goal' is not — the wheel for a part is for the men a goal would send`); }
    if (T.explain && (T.explain.text.length > CARD_CHARS || (T.explain.title?.length ?? 0) > 40)) out.push(`${w}.explain: a card is a heading of at most 40 characters and at most ${CARD_CHARS} of text (it is ${T.explain.title?.length ?? 0} and ${T.explain.text.length}): a card nobody reads teaches nothing`);
    // (a card's words and the frame's other lines are written out by lessonLine as a step's line is: a move's token in one is held the same way, by what the
    //  player reads — lesson.ts lessonTokenFaults — and a card's TITLE is shown as written, so a brace in it would be shown too)
    if (T.explain) { out.push(...lessonTokenFaults(`${w}.explain.text`, T.explain.text, CARD_CHARS)); if (/[{}]/.test(T.explain.title ?? '')) out.push(`${w}.explain.title has a brace: a card's title is shown as written, a move's token goes in its text`); }
    for (const [k, l] of [['noTap', T.noTap], ['menuLine', T.menuLine], ['pageLine', T.pageLine], ['flatLine', T.flatLine], ['after.line', T.after?.line], ['after.word', T.after?.word]] as const) if (typeof l === 'string') out.push(...lessonTokenFaults(`${w}.${k}`, l, 96));
    // (a mission on the flat ring: a line that names the pages' digits has its one-ring line beside it, and a one-ring line is for such a mission only)
    if (R.flatRing && T.menuLine !== undefined && T.flatLine === undefined) out.push(`${w}.menuLine names the menu's pages and rails.flatRing gives this mission one ring: the step wants a flatLine too`);
    if (!R.flatRing && T.flatLine !== undefined) out.push(`${w}.flatLine: the lesson's rails.flatRing is not set, so no ring here is flat`);
    if (T.cue !== undefined) { if (cues.has(T.cue)) out.push(`${w}.cue '${T.cue}' is another step's too: a cue starts one beat`); cues.add(T.cue); } });
  if (R.rows !== undefined && !(Array.isArray(R.rows) && R.rows.length > 0 && R.rows.every(v => typeof v === 'string' && !!v))) out.push('rails.rows: a list of verbs, at least one (absent: every row)');
  if (R.waits !== undefined) { const Wt = R.waits as unknown as Record<string, unknown> | null;
    if (!Wt || typeof Wt !== 'object' || Array.isArray(Wt) || Object.keys(Wt).some(k => k !== 'low') || (Wt.low !== undefined && typeof Wt.low !== 'boolean')) out.push('rails.waits: { low?: true } (the squad holds where it starts until a step allows `order`)');
    else if ((R.allow ?? []).includes('order') || !L.steps.some(T => T.allow?.includes('order'))) out.push("rails.waits: the wait ends at the first step that allows 'order', so one step has to and rails.allow must not (they would wait for ever, or not at all)"); }
  if (R.orders !== undefined && !(Array.isArray(R.orders) && R.orders.length > 0 && R.orders.every(x => typeof x === 'string'))) out.push('rails.orders: a list of order ids (at least one)');
  return out;
}

/** THE FRAME'S RECORD on the page's copy, as the bridge fills it from the Rust game. Nothing here decides anything: what a HUD is told is `game.view.lesson`. */
export class Rails {
  run: unknown = null; answered = -1; ordersSeen = 0;
  /** teammates spared a down: the game's clock each is back on his feet at, by character id */
  spared = new Map<number, number>();
  tapT = -1e9; moved = false; wayNv = -1; gaveStep = -1; gave = 0; gaveT = -1e9; gave0 = -1e9; actT = -1e9; running = false; stepLook: unknown = null;
  /** ticks left of a run's opening, in which the view may be set back under the step */
  fresh = 0;
  log: string[] = [];
  constructor(private g: Game) {}

  /** A MISSION THAT TEACHES THE WALL keeps a walk into one from taking it until that line (rust: rails_locks_wall) */
  locksWall(): boolean { return goneWithTick('Rails.locksWall') as any; }
  /** this man runs the leg he is on (rust: rails_runs) */
  runs(gd: Guard): boolean { return goneWithTick('Rails.runs') as any; }
  /** THE GATE: whatever the step has not unlocked is taken out of the tick's input (rust: rails_gate) */
  gate(input: Input, cam: FollowCamera, dt: number): void { return goneWithTick('Rails.gate') as any; }
  /** once a tick (rust: rails_tick) */
  tick(dt: number): void { return goneWithTick('Rails.tick') as any; }


  /** is this man on his knee right now, spared */
  isSpared(gd: Guard): boolean { const t = this.spared.get(gd.char.id); return t !== undefined && this.g.time < t; }
}
