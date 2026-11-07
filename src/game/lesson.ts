// A LESSON: the tutorial's missions (scene/scenarioList.ts LESSON_MISSIONS; first built for the seven Training levels, taken out 2026-10-23). A level file carries a `scenarios.lesson` block
// (scene/scenarios.ts LessonData): STEPS, each a line for the HUD's objective row naming the key, and a DONE-TEST read off the game's own state. The step's line
// moves on when the player has DONE the thing — never on a clock.
//
// WHAT IT IS NOT: a script. It moves nobody, spawns nothing, ends nothing and writes nothing the simulation reads (a step done asks for a checkpoint: `advance`) — a lesson is READ-ONLY over the game, so the
// same keys on a lesson level play the same run with it on or off (tools/qa/lesson-gate.ts holds the two digests equal for every Training level). The level under it is a real one of
// its kind — a quiet mission (quiet.ts) or a room clear (roomclear.ts) — with real guards on real routes; the scenario's own objective takes the row back after
// the last step and ends the run as it always does.
//
// LIVE = the level carries the block ∧ a scenario is up on it (so the plain `?levelFile=x&solo` boot of a lesson
// level is that level's plain night, no lines) ∧ no show running. A level without the block: every entry point returns on its first line.
//
// THE TESTS (LESSON_CHECKS below): plain records `{ is: '<kind>', … }`, each asked by the Rust game (rust/game_rules/src/lesson: `passes`). Two families —
//   STATE, true while it holds:  in · near · flag · slot · pace · meter · stage · door · target · men · all · any, and by family —
//     hands: downed · body · calm   kit: reload · launcher · burst   squad: orderTo · stackOn · pairsOn · taken · following   doors: stacks   the quiet partner: mate
//   DEEDS, counted from the moment the step came up:  did { what: 'act:<verb>' | 'event:<kind>' | 'chron:<kind>' | 'order:<id>' | 'order:<id>@foe:<callsign>', n?, on?: '<door>' } ·
//     threw (a can, the frag, the launcher's grenade) · word (F, the squad's word, over how many doors) · pair (the quiet pair's log)
// A new kind is its arm in the Rust game and its row of the checker's table here.
//
// ALL OF A LESSON THAT RUNS IS THE RUST GAME'S: which step is up, whether it is done, how its line reads. What a HUD is told of it is the view's `lesson` section
// (src/view/seatview.ts). What is left here: the switch, the level file's lint against the built floor (checkLesson), and the lesson's record as the bridge fills it.
//
// (src/game is one import cycle: nothing here is built from another game module's exports at module load.)
import { ofThisMission } from './thisrun';   // (the same mission by its serial, as the Rust game asks it)
import type { Game } from './game';
import type { LessonTest, Rect } from '../scene/scenarios';
import type { Vec3 } from '../math/vec';
import { WORLD } from '../scene/world';
/** A LINE NAMES THE MOVE, NOT ITS KEY: a token in braces is written out in the build's own words — the held click or the grab's key, the knockout or the
 *  knife — so a level's sentence can be reworded freely and still say the key the build has.
 *  `{grab him}` · `{grab him from behind}` · `{knock him out}` (the blow in reach) · `{take him out}` (the silent blow from behind). `him` may be any words
 *  for the man (`{grab the near guard from behind}`, `{grab yours}`): letters and apostrophes, one space between words. A capital first letter begins a sentence. A name in braces that is none of these is
 *  left as written, and checkLesson refuses one (lessonTokenFaults, below). */
const TOKENS: [RegExp, (who: string, held: boolean, bare: boolean) => string][] = [
  [/^grab (.+) from behind$/, (who, held) => held ? `hold left click on ${who} from behind to grab him` : `press [Space] behind ${who} to grab him`],
  [/^grab (.+)$/, (who, held) => held ? `hold left click on ${who} to grab him` : `press [Space] to grab ${who}`],
  [/^knock (.+) out$/, (who, _h, bare) => bare ? `right-click to knock ${who} out` : `right-click to knock ${who} out, or left-click to use the knife`],   // (the click as a verb is hyphened; "hold left click on him" above is the button being held, and is not)
  [/^take (.+) out$/, (who, _h, bare) => bare ? `right-click to knock ${who} out` : `left-click to knife ${who}`],
];
/** the four, as a level writes them of 'him' */
export const LESSON_TOKENS: readonly string[] = ['grab him', 'grab him from behind', 'knock him out', 'take him out'];
const TOKEN = /\{([A-Za-z][A-Za-z' ]*)\}/g, lower = (name: string) => name.charAt(0).toLowerCase() + name.slice(1);
/** is this name (the words between the braces, either capital) one the writer writes out? */
export const lessonToken = (name: string): boolean => { const n = lower(name); if (!/^[A-Za-z]+(?: [A-Za-z']+)*$/.test(name)) return false;   // (the letters the writer-out reads, one space between words, none at an end)
  const hit = TOKENS.find(([re]) => re.test(n)); return !!hit && !/(^| )from( |$)/.test(hit[0].exec(n)![1]); };   // (…and a 'from' left in the man's words is a 'from behind' misspelt)
export const tokenLine = (line: string, held: boolean, bare: boolean): string => line.indexOf('{') < 0 ? line : line.replace(TOKEN, (m, name: string) => {
  const n = lower(name); if (!lessonToken(name)) return m; for (const [re, say] of TOKENS) { const hit = re.exec(n); if (!hit) continue;
    const s = say(hit[1], held, bare); return n !== name ? s.charAt(0).toUpperCase() + s.slice(1) : s; }   // (the capital the author wrote, nothing guessed)
  return m; });
/** what checkLesson asks of a line with a brace in it: every name in braces is a token (a misspelt one would be shown, braces and all), and written out the
 *  longest of the four ways the two keys can stand it still fits the row — the cap is of what the player reads, not of what the level wrote */
export function lessonTokenFaults(w: string, line: string, cap: number, row = 'the objective row', always = false): string[] {
  if (line.indexOf('{') < 0 && line.indexOf('}') < 0) return [];
  const out: string[] = [], rest = line.replace(/\{([^{}]*)\}/g, (m, name: string) => { if (!lessonToken(name)) out.push(`${w} has {${name}}, which is no move's token (${LESSON_TOKENS.join(', ')})`); return ''; });
  if (/[{}]/.test(rest)) out.push(`${w} has a brace that closes nothing or is never closed`);
  const long = Math.max(...[0, 1, 2, 3].map(k => tokenLine(line, !!(k & 1), !!(k & 2)).length));
  if (!out.length && (always || line.length <= cap) && long > cap) out.push(`${w} is ${long} characters with its tokens written out: ${row} holds about ${cap}`);   // (`row`: what the cap is of, for the message · `always`: a field with no cap of its own as written — without it a line already over the cap as written is its own check's to refuse, and is not said twice)
  return out;
}
import { railsFaults } from './rails';   // (called inside checkLesson only)

/** the switch's knob (`?nolesson`): the Rust game does not read it, a lesson level always has its lesson */
export const LESSON = { on: true };
export const LESSON_STATS = { started: 0, steps: 0, finished: 0 };

/** what is wrong with one kind's record on this floor (null: nothing) */
type TestCheck = (g: Game, t: LessonTest) => string | null;

const num = (v: unknown): v is number => typeof v === 'number' && Number.isFinite(v);
const str = (v: unknown): v is string => typeof v === 'string' && v.length > 0;
const rect = (v: unknown): v is Rect => !!v && typeof v === 'object' && num((v as Rect).x0) && num((v as Rect).x1) && num((v as Rect).z0) && num((v as Rect).z1) && (v as Rect).x0 < (v as Rect).x1 && (v as Rect).z0 < (v as Rect).z1;
const vec = (v: unknown): v is Vec3 => Array.isArray(v) && v.length === 3 && v.every(num);
/** what a `pair` test may count (the first words of pairorders.ts PAIR_LOG rows) */
const PAIR_WHATS = ['mark', 'now', 'set', 'go', 'go:F', 'go:hands', 'down'];

/** the flags a `flag` test can read off the player (and the team's posture) */
const FLAGS: readonly string[] = ['crouch', 'sprint', 'wall', 'peek', 'lean', 'nv', 'light', 'holstered', 'gun', 'ringsOut', 'shellsOut', 'auto', 'setup', 'setupShut', 'dragging', 'dark', 'chargeSet', 'chargeClear', 'wallBlown', 'holding', 'picking', 'coding', 'menu', 'ownRing', 'silent'];

// ---- the test kinds' checks. KEEP each family's rows together and a blank line between families: parts add rows side by side ----
const areaCheck: TestCheck = (g, t) => !rect(t.area) ? 'area: a rect { x0, x1, z0, z1 }' : freeIn(g, t.area) ? null : 'area: no free floor inside it';
const noFields: TestCheck = () => null;
export const LESSON_CHECKS: Record<string, TestCheck> = {
  // -- where he is
  in: areaCheck,
  near: (_g, t) => !vec(t.at) ? 'at: a Vec3' : t.r !== undefined && !num(t.r) ? 'r: metres' : null,

  // -- what he is doing / carrying
  flag: (_g, t) => FLAGS.includes(t.flag as string) ? null : `flag: one of ${FLAGS.join(', ')}`,
  slot: (_g, t) => ['pistol', 'rifle', 'smoke', 'flash', 'frag'].includes(t.slot as string) ? null : 'slot: pistol, rifle, smoke, flash or frag',
  pace: (_g, t) => bandCheck(t),
  meter: (_g, t) => t.which !== 'light' && t.which !== 'noise' ? "which: 'light' or 'noise'" : bandCheck(t),

  // -- the world
  stage: (_g, t) => STAGES.includes(t.stage as string) && t.stage !== 'infiltrate' ? null : 'stage: drive (the place reached), exfil (the thing taken) or done',
  door: (g, t) => !g.doors.byName(t.door as string) ? `door: no door named '${String(t.door)}'` : ['open', 'shut', 'unlocked', 'broken', 'charged'].includes(t.state as string) ? null : 'state: open, shut, unlocked, broken or charged',
  target: (g, t) => !g.targets.some(y => y.name === t.name) ? `name: no light or device named '${String(t.name)}'` : typeof t.on !== 'boolean' ? 'on: true or false' : null,
  men: (_g, t) => t.down === undefined || (num(t.down) && t.down >= 1) ? null : 'down: how many, ≥ 1',

  // -- deeds, counted since the step came up
  // (an order may name the DOOR it was given on: `on: '<door name>'` — "stack up on the first hut's door" is not passed by a stack on another — or, after an `@` in `what`, the MAN or the THING it was given on, in the words orders.ts logs as its target: a callsign is the deal's, so a man is named only on a mission whose men are placed the same every run)
  did: (g, t) => { const m = str(t.what) ? /^(?:(?:act|event|chron|order):[\w.-]+|order:[\w.-]+@(foe:[\w.'-]+(?: [\w.'-]+)*|thing:(?:breaker|objective|device)))$/.exec(t.what) : null, foe = m?.[1]?.startsWith('foe:') ? m[1].slice(4) : null;
    return !m ? "what: 'act:<verb>', 'event:<kind>', 'chron:<kind>', 'order:<id>', 'order:<id>@foe:<callsign>' (an order given ON that man) or 'order:<id>@thing:breaker|objective|device' (a door is `on`)"
    : foe !== null && g.guards.some(x => x.team !== 'operative' && x.state !== 'dead') && !g.guards.some(x => x.team !== 'operative' && x.callsign === foe) ? `what: no man called '${foe}' on this floor`
    : t.on === undefined ? null : m[1] ? 'on: not with a target in `what`' : !(t.what as string).startsWith('order:') ? 'on: only with an order' : !str(t.on) || !g.doors.byName(t.on as string) ? `on: no door '${t.on}'` : null; },

  // -- (the hands' rows)
  downed: (_g, t) => !HOWS.includes(t.how as string) ? `how: one of ${HOWS.join(', ')}` : t.n === undefined || (num(t.n) && t.n >= 1) ? null : 'n: how many, ≥ 1',
  body: areaCheck,
  calm: (g, t) => t.route === undefined || g.level.routes.some(r => r.name === t.route) ? null : `route: no patrol route named '${String(t.route)}'`,

  // -- (the guns' and the kit's rows)
  loaded: noFields,
  reload: noFields,
  launcher: (_g, t) => !['ring', 'he40', 'none'].includes(t.on as string) ? "on: 'ring', 'he40' or 'none'" : t.fired !== undefined && typeof t.fired !== 'boolean' ? 'fired: true or false' : t.fired && t.on === 'none' ? 'fired: nothing is fired with no attachment on' : null,
  burst: (_g, t) => t.n === undefined || (num(t.n) && t.n >= 2) ? null : 'n: how many rounds in the one string, ≥ 2',
  threw: (_g, t) => t.area !== undefined && !rect(t.area) ? 'area: a rect { x0, x1, z0, z1 }' : t.area !== undefined && (t.quick || t.kind === 'he40') ? 'area: a thrown can or frag (not the quick throw, not the 40 mm)' : t.kind !== undefined && !['smoke', 'flash', 'frag', 'he40'].includes(t.kind as string) ? "kind: 'smoke', 'flash', 'frag' or 'he40'" : t.quick !== undefined && typeof t.quick !== 'boolean' ? 'quick: true or false' : t.quick && (t.kind === 'frag' || t.kind === 'he40') ? 'quick: G throws only smoke and the stun can' : null,

  // -- (the squad's rows: orders, door tools, two pairs)
  orderTo: (_g, t) => t.who === 'all' || t.who === 'red' || t.who === 'blue' ? null : "who: 'all', 'red' or 'blue'",
  stackOn: (g, t) => !g.doors.byName(t.door as string) ? `door: no door named '${String(t.door)}'` : t.who === undefined || t.who === 'all' || t.who === 'red' || t.who === 'blue' ? null : "who: 'all', 'red' or 'blue'",
  pairsOn: (_g, t) => t.doors === undefined || (num(t.doors) && t.doors >= 1) ? null : 'doors: how many different doors, ≥ 1',
  word: (_g, t) => t.doors === undefined || (num(t.doors) && t.doors >= 1) ? null : 'doors: how many doors go on it, ≥ 1',
  taken: (g, t) => g.level.scenarios?.roomClear?.rooms.some(r => r.id === t.room) ? null : `room: no room '${String(t.room)}' in the level's roomClear block`,
  following: noFields,

  // -- (the quiet partner's rows)
  mate: (g, t) => t.area === undefined && t.at === undefined && t.by === undefined && t.away === undefined ? 'one of area (a rect), at (a Vec3, + r), by (metres from the player), away (metres)'
    : t.area !== undefined && !rect(t.area) ? 'area: a rect { x0, x1, z0, z1 }' : t.area !== undefined && !freeIn(g, t.area as Rect) ? 'area: no free floor inside it' : t.at !== undefined && !vec(t.at) ? 'at: a Vec3'
    : [t.r, t.by, t.away].some(v => v !== undefined && !(num(v) && v > 0)) ? 'r / by / away: metres, > 0' : null,
  pair: (_g, t) => !PAIR_WHATS.includes(t.what as string) ? `what: one of ${PAIR_WHATS.join(', ')}` : t.n === undefined || (num(t.n) && t.n >= 1) ? null : 'n: how many, ≥ 1',

  // -- several at once
  // ---- a rails level's (rails.ts) ----
  cue: (_g, t) => str(t.name) ? null : 'name: the beat the level\'s script ends with c.signal(\'done:<name>\')',
  view: (_g, t) => t.turned === undefined && t.zoomed === undefined && t.panned === undefined ? 'turned (degrees), zoomed (a ratio over 1) and / or panned (metres)' : (t.turned !== undefined && !num(t.turned)) || (t.panned !== undefined && !num(t.panned)) || (t.zoomed !== undefined && !(num(t.zoomed) && t.zoomed > 1)) ? 'turned, panned: numbers · zoomed: a ratio over 1' : null,
  squad: (g, t) => !rect(t.area) ? 'area: a rect { x0, x1, z0, z1 }' : !freeIn(g, t.area as Rect) ? 'area: no free floor inside it' : t.n === undefined || (num(t.n) && t.n >= 1) ? null : 'n: how many, ≥ 1',
  stacks: (g, t) => !Array.isArray(t.doors) || !t.doors.length ? 'doors: a list of door names' : (t.doors as string[]).find(d => !g.doors.byName(d)) !== undefined ? `doors: no door named '${String((t.doors as string[]).find(d => !g.doors.byName(d)))}'` : t.n === undefined || (num(t.n) && t.n >= 1) ? null : 'n: how many men a door, ≥ 1',
  watching: (_g, t) => !vec(t.at) ? 'at: a Vec3' : t.r !== undefined && !(num(t.r) && t.r > 0) ? 'r: metres, > 0' : t.n === undefined || (num(t.n) && t.n >= 1) ? null : 'n: how many men, ≥ 1',
  all: (g, t) => ofCheck(g, t),
  any: (g, t) => ofCheck(g, t),
};
const STAGES: string[] = ['infiltrate', 'drive', 'exfil', 'done'];
const HOWS: string[] = ['takedown', 'choke', 'whip', 'ring', 'knife'];
function bandCheck(t: LessonTest): string | null { return (t.below === undefined && t.above === undefined) || (t.below !== undefined && !num(t.below)) || (t.above !== undefined && !num(t.above)) ? 'below and / or above: numbers' : null; }
function ofCheck(g: Game, t: LessonTest): string | null { if (!Array.isArray(t.of) || !t.of.length) return 'of: a non-empty list of tests'; for (const u of t.of as LessonTest[]) { const why = testFault(g, u); if (why) return why; } return null; }
function freeIn(g: Game, R: Rect): boolean { const nav = g.col.nav; for (let x = R.x0 + 0.2; x < R.x1; x += 0.3) for (let z = R.z0 + 0.2; z < R.z1; z += 0.3) if (!nav.isBlocked(x, z)) return true; return false; }
/** what is wrong with one test record on this floor (null: nothing) */
export function testFault(g: Game, t: LessonTest): string | null {
  if (!t || typeof t !== 'object' || !str(t.is)) return "a test is a record { is: '<kind>', … }";
  const K = LESSON_CHECKS[t.is]; if (!K) return `no test kind '${t.is}' (${Object.keys(LESSON_CHECKS).join(', ')})`;
  const why = K(g, t); return why ? `${t.is} — ${why}` : null;
}

/** a checkpoint's slice of a lesson */
export interface LessonSnap { i: number; done: [string, number][] }

/** THE LESSON'S RECORD on the page's copy, as the bridge fills it from the Rust game. Nothing here decides anything: what a HUD is told is `game.view.lesson`. */
export class Lesson {
  /** the step that is up (= steps.length once every step is done) */
  i = 0;
  /** game time each step was done at, by id — the scripted run and the probes read it */
  doneAt = new Map<string, number>();
  log: string[] = [];
  seen: unknown = null;
  run: object | null = null;   // the Mission this pass belongs to
  constructor(private g: Game) {}

  /** a checkpoint's slice (savestate.ts): which step is up and when the ones before it were done. The deeds a step has counted are NOT filed: a loaded step
   *  counts from the load */
  snapshot(): LessonSnap { const mine = ofThisMission(this.g, this.run); return { i: mine ? this.i : 0, done: mine ? [...this.doneAt] : [] }; }   // (a save that lands on a fresh Mission before its first tick files step 1, never the last run's step)
  /** a checkpoint loaded (savestate.ts, after the mission slice): the file's place among the steps — a file without the slice (made with the lesson off)
   *  leaves the lesson stood down for that run: its world is mid-way, and step 1 over it could ask for a deed already done */
  restore(s: LessonSnap | undefined) {
    const of = this.g.level.scenarios?.lesson?.steps?.length; if (!of) return;   // (the level's word, not the view's: a file loads with no Rust game standing to ask)
    this.run = this.g.mission; this.seen = null; this.doneAt.clear(); this.i = of;
    if (s && Number.isInteger(s.i) && s.i >= 0 && s.i <= of) { this.i = s.i; for (const [id, t] of s.done ?? []) if (typeof id === 'string' && Number.isFinite(t)) this.doneAt.set(id, t); }
  }
}

// ---------------------------------------------------------------- the checker (tools/qa/lesson-check.ts; the probes): the block against the BUILT floor
/** Everything the level file's validator cannot say (scene/levelData.ts checks shapes): each test's kind and fields, its doors / lights / areas on this floor,
 *  a marker that is in the world. `g` is a Game stood up on the level. Returns the faults, each a line an author can act on. */
export function checkLesson(g: Game): string[] {
  const L = g.level.scenarios?.lesson, out: string[] = [];
  if (!L) return ['the level carries no scenarios.lesson block'];
  out.push(...railsFaults(L));   // (rails.ts: the frame's own keys — action names, cues)
  if (!g.level.scenarios?.quiet && !g.level.scenarios?.roomClear && !g.level.scenarios?.rescue && !g.level.scenarios?.nightBreach) out.push('the level carries a lesson and no scenario to run it over (scenarios.quiet / roomClear / rescue / nightBreach)');
  L.steps.forEach((T, i) => { const w = `steps[${i}] '${T.id}'`;
    const why = testFault(g, T.done); if (why) out.push(`${w}.done: ${why}`);
    if (T.line.length > 96) out.push(`${w}.line is ${T.line.length} characters: the objective row holds about 96`);
    if (T.setupLine && T.setupLine.length > 96) out.push(`${w}.setupLine is ${T.setupLine.length} characters: the objective row holds about 96`);
    if (T.blowLine && T.blowLine.length > 96) out.push(`${w}.blowLine is ${T.blowLine.length} characters: the objective row holds about 96`);
    for (const k of ['line', 'setupLine', 'blowLine', 'ringLine'] as const) { const l = T[k]; if (l) out.push(...lessonTokenFaults(`${w}.${k}`, l, 96)); }
    if (T.at && !(T.at[0] > WORLD.min[0] && T.at[0] < WORLD.max[0] && T.at[2] > WORLD.min[2] && T.at[2] < WORLD.max[2])) out.push(`${w}.at (${T.at[0]}, ${T.at[2]}) is outside the world`); });
  return out;
}
