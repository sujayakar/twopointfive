// The SESSION JOURNAL (the maintainer, 08-13: "could we make a record mode that journals all of the reads from the GPU for replay later? then we could have those
// scenarios for a strict mode QA of the deterministic stuff"). The simulation is deterministic GIVEN ITS INPUTS; this files the inputs. A recording is:
//
//   frame 0   a CHECKPOINT (savestate.ts saveState, kind 'harness') of the moment F10 went down — and the running game re-entered through loadState on the spot,
//             so the recorded game and any replay start from the same bytes by the same road (the checkpoints' identity claim, probes.ts checkpoint-identity:
//             load-into-the-same-game ≡ load-into-a-fresh-game); smoke in the air is dropped as on any load, a conversation is cut short, a searcher re-deals;
//   seed      Math.random ← mulberry32(seed) from that frame to the stop (the game's dice: barks, spread, reaction jitter — math/dice.ts simDie); the mixer and
//             the smoke solver draw the SYSTEM die (dice.ts sysRandom) so nothing but the simulation takes from the seeded stream — headless there is neither;
//   env       what the simulation reads off the ENGINE rather than the Game (the debris cap of the quality tier, the emissive scale, the torch bezel radius, whether
//             GPU particles are attached, the marking lights, the canister row) and the construction-time dice a checkpoint does not hold (the fixtures' restrike
//             stagger) — a replay stands its engine stand-in up to match;
//   knobs     Game.tune, the sandbox flags and every module knob table (AIM, NEEDS, WHISTLE …: kept per machine, so the maintainer's are not the harness's);
//   per frame dt exactly as Game.update got it · the canvas size · the CAMERA as the update read it (look-at, yaw, pitch, distance, aspect, fov: an input like the
//             mouse — the aim ray, WASD's basis, who is 'on screen' for a bubble all hang on it — so a follow-camera tweak invalidates no recording) · the input
//             (keys held, keys pressed, buttons, clicks, cursor, wheel) · every SENSOR value delivered (sensors.ts: the light meters, the smoke sight lines — the
//             GPU's part, journaled as consumed) — each stream delta-coded, so a still scene costs a dt a frame;
//   digests   digestGame (digest.ts) after every JOURNAL_DIGEST_EVERY-th update and after the last: a replay compares its own and names the field that differs.
//
// tools/qa/replay.ts plays a file back on the headless harness (fresh Game → loadState → seed → the frames fed through the synthetic input path with the sensors in
// replay mode) and exits 0 only if every digest agreed and no sensor read diverged: a REFACTOR must replay every session in tools/qa/sessions/ identically; an
// intended gameplay change is expected to diverge (re-record). This module is UI-free (main.ts owns F10, the HUD tag and the delivery: ui/sessionrec.ts); the
// recorder runs headless too (tools/qa/journal-probes.ts records its own fixture).
//
// NOT captured, by decision: the Tab panel's live edits mid-recording (hands off it while REC is up), module tables not in KNOB_TABLES, a level other than the
// one the replay builds (the file's fingerprint refuses it), the smoke solver's CPU-side facts (inSmokeDomain / budget — read only to decide whether to emit MORE
// smoke, never by the simulation), a lightning stroke's light alive across frame 0 (a fifth of a second, once a storm-minute).
import type { Game } from './game';
import { reportError } from '../cloud/report';   // (the reporter: a leaf that imports nothing, reads no game state, rolls no die. A page's callback that threw is said, and that is all)
import { DESTRUCTION } from './destructionSwitch';
import { LIVED } from '../scene/lived';
import { SHAPES } from '../scene/shapes';
import { cutConfig } from '../scene/cut';
import type { Input } from './input';
import type { FollowCamera } from './camera';
import { saveState, loadState, canSave, levelFingerprint } from './savestate';
import type { SaveState, TapeSlots } from './savestate';
import type { SensorJournal } from './sensors';
import { digestGame } from './digest';
import { simDie } from '../math/dice';
import type { SimDie } from '../math/dice';
import type { GameDigest } from './digest';
import { envAsBuilt } from './env';   // (a leaf: it imports nothing, so it moves nobody's place in the order)
import { knobsNamed, assignDeep, JOURNAL_KNOBS } from './knobs';   // THE list of knob tables (knobs.ts): it imports the tables' modules in the order this file always did
import { WALLS, WALLS_DEFAULTS } from './wallbreak';
import { DOORTOOLS } from './doortools';
import { SNEAK } from './sneak';
import { AIR_SIM } from './airSim';
import { WALLSNAP } from './wallsnap';
import { STALK } from './stalk';
import { TAPSTEP } from './tapstep';
import { STICKY } from './stickycam';
import { BIGSLOT } from './kit';
import { SQUADTHROW } from './squadthrow';
import { CMDRING } from './cmdring';
import { HOLD } from './character';
import { MORALE } from './morale';
import { FIRE, FIRE_LEAN_WAS, FIRE_STEPS } from './fire';
import { POSEMATH } from '../anim/skeleton';
import { SENSE } from './sense';
import { DOZE } from './doze';
import { AIMWEIGHT } from './aimweight';
import { TEAMPACE } from './teampace';
import { RAILS } from './rails';
import { AIMCOST } from './aimcost';
import { MARKSIT } from '../render/decals';
import { PLAN_PACE } from '../render/planPace';
import { REACH_PACE } from '../render/reach';
import { COATFINISH, SOLIDGLASS } from '../scene/stuff';
import { WALLSKIN } from './wallskin';
import { AUTOCAM, autoTuneOld } from '../host/autocam';
import { PRESSURE } from './pressure';
import { DOORSHOT } from './doorblast';
import { LIGHT_IN } from '../sim/lightshome';

export const JOURNAL_VERSION = 1;
/** …and what a file says when a replay cannot do without what came after v1: a slot's text filed at the frame an update read it (`slots.loads`), a last frame
 *  no replay can follow (`meta.lost`). A build from before them refuses such a file by its version instead of replaying it into a break of its own; every other
 *  file still says 1 and is read by any build. */
export const JOURNAL_VERSION_SLOTS = 2;
/** a digest after every this-many-th update (and after the last) */
export const JOURNAL_DIGEST_EVERY = 120;
/** a recording asked for while it cannot start (Sam mid-action, a can in the air, a body falling) waits this many frames for the moment, then gives up with the reason */
export const JOURNAL_PENDING_FRAMES = 60 * 8;
/** the longest recording (frames): twenty minutes at sixty */
export const JOURNAL_MAX_FRAMES = 60 * 60 * 20;

/** the module knob tables a recording carries and a replay applies (kept per machine or flipped on the panel: the maintainer's values are not the harness's) */
/** the knob tables a recording captures — LAZY: journal.ts sits on the game.ts ↔ guards.ts import cycle, and the browser bundle hoists module bindings, so an eager object
 *  literal here captured `undefined` for tables whose module had not initialised yet (the recorder threw on its first page frame; headless import order hid it) */
export function knobTables(): Record<string, object> { return knobsNamed(JOURNAL_KNOBS); }
// (the tables themselves, and why each is in a recording, are knobs.ts's now — GLASSBREAK: rounds, sound and men pass through its holes, gameplay, while the dust table is
//  presentation and stays out; DOORSTAY: whether any leaf has a closer is gameplay for everyone who walks, looks or listens through a doorway; HANDLE: where a man stands to
//  open a door; DOORMANNERS: who waits for whom at a frame, both sides of a squad map)

// ================================================================================================================================ the file
/** one frame's input as filed: k = the keys held (the whole set, only when it changed), p = keys pressed this frame, c = mouse buttons clicked this frame (bits),
 *  b = buttons held (on change), m = the cursor in CSS px (on change), w = wheel travel (when non-zero) — absent = unchanged (k / b / m) or nothing (p / c / w) */
export interface InputRec { k?: string[]; p?: string[]; c?: number; b?: number; m?: [number, number]; w?: number; }
/** the engine-side facts the simulation reads (see the head) + the fixtures' restrike stagger by target name */
export interface EnvSnap { debrisCap: number; emissiveScale: number; flashlightRadius: number; markings: boolean; particlesOn: boolean;
  /** recorded under the game page's HUD (main.ts): its speech bubbles null an expired `bubble` after each update (hudExpireBubbles) — a replay of such a file does the same; a headless recording has no HUD */
  hud: boolean;
  /** HostApi.glassFracture where the recording's engine said either way (the page: were its shaders compiled with the fracture's cracks; native: false): glass.ts files a pane's star decal — which the digest counts — only where the renderer does NOT draw the fracture's own cracks, so a replay must stand on the same answer. Absent = the engine did not say (the headless stub; every file from before the new glass) */
  glassFracture?: boolean;
  /** scene/cut.ts cutConfig.on as the recording ran (absent = off): rayBox / lineBox answer a box's CUT solid only under it — the harness raises it before its BoxWorld exists */
  cutBoxes?: boolean;
  canister: { canisterDuration: number; canisterDensity: number; canisterRadius: number; canisterTemp: number; canisterSpeed: number } | null; stagger: Record<string, number>;
  /** game/destructionSwitch.ts as the recording ran (absent in files from before it = off): the level's dressing, the roof rule, the loud rules under a scenario */
  destruction?: { on: boolean };
  /** scene/lived.ts as the recording ran (absent = off): the level was loaded with its lived-in rooms' furniture, so a replay loads it the same way */
  lived?: boolean; }
export interface KnobsSnap { tune: Record<string, unknown>; flags: { godMode: boolean; aiEnabled: boolean; infiniteAmmo: boolean; playerInvisible: boolean; fn2000: boolean; autosave: boolean; fog?: boolean }; tables: Record<string, unknown>; }
/** the whole session (JSON): see the head for what each block is */
export interface SessionFile {
  v: number; game: 'twopointsix'; recordedAt: string; commit?: string; note?: string;
  seed: number; level: SaveState['level']; save: SaveState; env: EnvSnap; knobs: KnobsSnap;
  frames: number;
  /** dt per frame, run-length coded: [count, dt] pairs */
  dt: [number, number][];
  /** the canvas: [f, clientWidth, clientHeight, width, height] on change */
  view: number[][];
  /** the camera as Game.update read it: [f, mask, values…] on change — mask bits 0‥7 = look-at x / y / z, yaw, pitch, distance, aspect, fov */
  cam: number[][];
  /** [f, InputRec] for every frame on which anything was pressed / clicked / moved / changed */
  input: [number, InputRec][];
  sensors: SensorJournal;
  /** the checkpoint store as the recording found it at its frame 0 (the index rows, the autosave inside its regret window) and, as [f, key, text], each slot
   *  of those that an update read — SessionRecorder.slotRead. Absent: nothing was stored (or a file from before the block) */
  slots?: TapeSlots & { loads: [number, string, string][] };
  /** [f, digest] after the update of frame f */
  digests: [number, GameDigest][];
  meta: { seconds: number; stopped: string; bytesPerFrame?: number; userAgent?: string; /** finished between two frames: the closing digest is of the state after the HUD's write of the last one (a replay of a page recording makes that write before it compares) */ between?: boolean; /** ended by a load no replay can make (SessionRecorder.cut, `lost`): the last frame is that load's, filed whole, and a replay plays to the frame before it */ lost?: boolean };
}
export type JournalState = 'idle' | 'pending' | 'recording';
type View = { clientWidth: number; clientHeight: number; width: number; height: number };

// ================================================================================================================================ env / knobs (both directions)
export function envSnapshot(g: Game): EnvSnap {
  const S = g.engine.settings as unknown as Record<string, unknown>; const num = (k: string, d: number) => typeof S[k] === 'number' && Number.isFinite(S[k] as number) ? S[k] as number : d;
  const E = g.engine as unknown as { particles?: unknown; glassFracture?: unknown; settings: { gpuParticles?: boolean } };
  const P = (g.smoke as { params?: Record<string, number> }).params;
  const stagger: Record<string, number> = {}; for (const t of g.targets) stagger[t.name] = t.stagger;
  return { debrisCap: num('debrisCap', 64), emissiveScale: num('emissiveScale', 1), flashlightRadius: num('flashlightRadius', 0.07), markings: S.markings !== false, particlesOn: !!(E.particles && E.settings.gpuParticles), hud: false, ...(typeof E.glassFracture === 'boolean' ? { glassFracture: E.glassFracture } : {}),
    canister: P ? { canisterDuration: P.canisterDuration, canisterDensity: P.canisterDensity, canisterRadius: P.canisterRadius, canisterTemp: P.canisterTemp, canisterSpeed: P.canisterSpeed } : null, stagger, ...(DESTRUCTION.on ? { destruction: { on: true } } : {}), ...(LIVED.on ? { lived: true } : {}), ...(cutConfig.on ? { cutBoxes: true } : {}) };
}
/** the settings half of an env, onto an engine's settings BEFORE the Game is built on it (the marking lights are registered in the constructor) */
export function applyEnvSettings(settings: Record<string, unknown>, env: EnvSnap): void { settings.debrisCap = env.debrisCap; settings.emissiveScale = env.emissiveScale; settings.flashlightRadius = env.flashlightRadius; settings.markings = env.markings; }
/** the game half of an env, AFTER the checkpoint is loaded: the fixtures' restrike stagger */
export function applyEnvGame(g: Game, env: EnvSnap): void { for (const t of g.targets) { const s = env.stagger[t.name]; if (typeof s === 'number' && Number.isFinite(s)) g.lightHome.in(LIGHT_IN.stagger, t, s); } }
export function knobsSnapshot(g: Game): KnobsSnap {
  const tables: Record<string, unknown> = {}; for (const [k, T] of Object.entries(knobTables())) tables[k] = JSON.parse(JSON.stringify(T));
  return { tune: JSON.parse(JSON.stringify(g.tune)), flags: { godMode: g.godMode, aiEnabled: g.aiEnabled, infiniteAmmo: g.infiniteAmmo, playerInvisible: g.playerInvisible, fn2000: g.fn2000, autosave: !!g.saves?.enabled, fog: !!g.fogOfWar?.on }, tables };   // (autosave: the controller writes chronicle rows — state, even if the digest leaves them out)
}
export function applyKnobs(g: Game, k: KnobsSnap): void {
  assignDeep(g.tune as unknown as Record<string, unknown>, k.tune);
  if (typeof k.tune.qrfSquad !== 'boolean') g.tune.qrfSquad = false;   // (a recording from before the response team took a room by the squad's own code — no such key in its tune: its element ran its own drill)
  g.godMode = !!k.flags.godMode; g.aiEnabled = k.flags.aiEnabled !== false; g.infiniteAmmo = !!k.flags.infiniteAmmo; g.playerInvisible = !!k.flags.playerInvisible; if (typeof k.flags.fn2000 === 'boolean' && g.fn2000 !== k.flags.fn2000) g.setLoadout(k.flags.fn2000);
  if (g.fogOfWar) g.fogOfWar.on = !!k.flags.fog;   // (the fog of war asks the sensors and bends say() / the reticle: a recording made under it replays under it — a file from before it has no flag = off)
  if (g.saves && typeof k.flags.autosave === 'boolean') g.saves.enabled = k.flags.autosave;   // (the field, not setEnabled: nothing to persist on a replay's machine)
  if (!k.tables.ENV) envAsBuilt();   // (a recording from before the game had numbers of its own for the scene's exposure and the tube's gain, env.ts: it was made on the picture's defaults, which are the build's — whatever a recipe or a --set left in the table)
  if ((k.tables.HARD as { sweep?: boolean } | undefined)?.sweep !== true) PRESSURE.sweep = false;   // (a recording from before hard's defenders went to look — it carries no HARD row, or one with no such key — replays with every man holding where the noise left him. The switch lives on HARD: PRESSURE is hard's copy of it and no recording carries that table, so the copy is only ever taken DOWN here; a recording that has the row on replays on the difficulty of the page it is played on, as every HARD row does)
  for (const [name, T] of Object.entries(knobTables())) { const src = k.tables[name]; if (src && typeof src === 'object') assignDeep(T as Record<string, unknown>, src as Record<string, unknown>); }
  if (typeof (k.tables.MOVES as { snap?: number } | undefined)?.snap !== 'number') { const M = knobTables().MOVES as { snap?: number } | undefined; if (M) M.snap = 0; }   // (a recording from before the pistol's carry and aim traded — its MOVES row has no such key — replays with the two layers on their old clocks)
  if (typeof (k.tables.MOVES as { steady?: number } | undefined)?.steady !== 'number') { const M = knobTables().MOVES as { steady?: number } | undefined; if (M) M.steady = 0; }   // (likewise a recording from before the crouch walk was carried level replays with the cycle as it was: rounds leave the drawn muzzle)
  if (typeof (k.tables.MOVES as { steadyRun?: number } | undefined)?.steadyRun !== 'number') { const M = knobTables().MOVES as { steadyRun?: number } | undefined; if (M) M.steadyRun = 0; }   // (…and from before the rifle sprint's side roll was taken out — no such key in its MOVES row — replays with the trunk rolling as it was recorded: his head and chest bones are what eyes look for)
  if (!k.tables.WALLSNAP) WALLSNAP.on = false;   // (a file from before the wall hold's feel pass: its takes ran the old clock and its rifle was slung for the wall)
  else { const ws = k.tables.WALLSNAP as Partial<typeof WALLSNAP>; if (ws.alongDeg === undefined) WALLSNAP.alongDeg = 0; if (ws.press === undefined) WALLSNAP.press = false; if (ws.leanGun === undefined) WALLSNAP.leanGun = 0; }   // (…and one from before 10-10's two rules: the stick's along / away as it was, the rifle drawn on the wall waiting for the step off)
  if (typeof (k.tables.CMDRING as { hint?: boolean } | undefined)?.hint !== 'boolean') CMDRING.hint = false;   // (… and one from before its hint: nothing drawn at the cursor before the key)
  if (typeof (k.tables.CMDRING as { holdSecs?: number } | undefined)?.holdSecs !== 'number') CMDRING.holdSecs = 0;   // (… and from before a tap opened no ring: the ring went up as the key went down)
  for (const key of ['goalHalf', 'twice'] as const) if (typeof (k.tables.CMDRING as Record<string, unknown> | undefined)?.[key] !== 'boolean') CMDRING[key] = false;   // (… and from before the first goal sent half the squad and the key tapped twice was 'fall in': its first goal was everyone's, and its second tap the key's order again)
  if (typeof (k.tables.HOLD as { strip?: { fall?: number } } | undefined)?.strip?.fall !== 'number') HOLD.strip.fall = 0;   // (a file from before the stripped gun's fall was timed: thrown down at the fixed push, wherever that landed it)
  if (!k.tables.BIGSLOT) BIGSLOT.on = false;   // (a file from before the big slot: the shotgun was a man's gun, a wall charge was pouches, nobody carried a hammer)
  if (!k.tables.SQUADTHROW) SQUADTHROW.on = false;   // (a file from before teammates threw a frag or a stun canister on order: the floor's menu had no such row)
  if (!k.tables.TAPSTEP) TAPSTEP.on = false;   // (a file from before the steps: a crouch walked)
  if (k.tables.STALK) { const T = k.tables.STALK as Partial<typeof STALK>;
    if (T.drag === undefined) STALK.drag = false;   // (…and from before the mouse could steer the creep: the hold walks him in)
    if (T.orbitSnap === undefined) STALK.orbitSnap = 0;   // (…and from before a key nearly across him was taken as across him)
    if (T.wideLead === undefined) STALK.wideLead = 0;   // (…and from before the left hand led the right: both as far ahead)
    if (T.widePole === undefined) STALK.widePole = [0.75, 0.1, -0.65];   // (…and with the elbows out to the side, as the arms first went out)
    if (T.joltFace === undefined) STALK.joltFace = 180; }   // (a file from before he shrank back off the man who turned round on him)
  if (k.tables.STICKY && (k.tables.STICKY as Partial<typeof STICKY>).live === undefined) STICKY.live = STICKY.max;   // (a file from before only the newest camera sent a picture: every camera out was an eye)
  if (k.tables.RAILS) { const R = k.tables.RAILS as Record<string, unknown>; if (R.snapPx === undefined) RAILS.snapPx = 0; if (R.keep === undefined) RAILS.keep = 0; if (R.keepDeep === undefined) { RAILS.keepDeep = 1; RAILS.keepUp = 0; } if (R.wayM === undefined) RAILS.wayM = 0; if (R.afterSecs === undefined) RAILS.afterSecs = 0; }   // (a tutorial run recorded before the ring was the place and the mark was kept in the picture)
  if ((k.tables.TEAMPACE as { doorBrisk?: boolean } | undefined)?.doorBrisk === undefined) TEAMPACE.doorBrisk = false;   // (… and one from before nobody sprinted to a stack: its men ran to cover on the way to a door as on any leg)
  if ((k.tables.TEAMPACE as { pacing?: boolean } | undefined)?.pacing === undefined) TEAMPACE.pacing = false;   // (… and one from before a teammate paced himself on the player: its men walked every leg at the flat share of his walk)
  if (k.tables.AIMWEIGHT) if ((k.tables.AIMWEIGHT as Partial<typeof AIMWEIGHT>).launchWait === undefined) AIMWEIGHT.launchWait = 0;   // (a file from before the player's aim had weight: its clicks were aimed with the gun eased straight onto the cursor — and one from before a launcher's load waited on the swing: it left the frame the button came up)
  if ((k.tables.AIMCOST as Partial<typeof AIMCOST> | undefined)?.flankAt === undefined) AIMCOST.flankAt = 1e9;   // (…and one from before the standoff's flank round waited for still feet: it went the frame the line was his)
  if (!k.tables.MARKSIT) MARKSIT.on = false;   // (a file with no table for the word for marks that sit in the surface: its blast marks are drawn as before the rule. A picture only — and that is also a tape of 11-04 and 11-05, when the page raised the word by hand with no table to record it)
  if (!k.tables.PLAN_PACE) PLAN_PACE.on = false;   // (a file with no table for the planners' pace: each plan of the picture's was made whole on its frame. A picture only)
  if (!k.tables.REACH_PACE) REACH_PACE.whole = false;   // (a file with no table for the reach masks' pace: the shares, which is what the page plays with no word)
  if (!k.tables.COATFINISH) COATFINISH.on = false;   // (a file with no table for a named coat's finish: its vehicles' bodies have the finish their level files give them, as before the rule. A picture only)
  if (!k.tables.SOLIDGLASS) SOLIDGLASS.on = false;   // (a file with no table for solid glass's dark: its vehicles' windows are the grey their level files give them, as before the rule. A picture only)
  if (!k.tables.WALLSKIN) WALLSKIN.on = false;   // (a file from before the paint on a wall went with the wall: its holed walls' bands were drawn whole)
  if ((k.tables.AUTOCAM as Partial<typeof AUTOCAM> | undefined)?.tune === undefined) autoTuneOld((k.tables.AUTOCAM ?? {}) as Partial<typeof AUTOCAM>);   // (…and one from before its numbers were the player's: the switch off, and the numbers it has no word for the module's own)
  if ((k.tables.MORALE as Partial<typeof MORALE> | undefined)?.training === undefined) { MORALE.training = 'none'; MORALE.nerve = 'normal'; MORALE.fanStep = 0; }   // (…and one from before a man was green or steady: every man the first slice's — broken alone in the dark only, folded when he was, and scrabbling straight back with no fan to find him a way (fanStep 0 shuts it). A recording that carries the keys keeps its own: assignDeep takes a string row as it takes a number)
  if ((k.tables.MORALE as Partial<typeof MORALE> | undefined)?.darkRead === undefined) MORALE.darkRead = 'place';   // (…and one from before the dark had a second reading: where he stands, as it was read)
  { const F0 = k.tables.FIRE as (Partial<typeof FIRE> & { look?: Partial<typeof FIRE.look> }) | undefined;   // (… one from before a fire's lamp was a fire's: the level's strength and colour; and from before the look's newer numbers: the picture's memory unchanged, the flame hardly leaning)
    if (F0?.bright === undefined) { FIRE.bright = 1; FIRE.warm = [1, 1, 1]; }
    if (F0?.reach === undefined) FIRE.reach = 1;
    if (F0?.look?.react === undefined) { FIRE.look.react = 0; FIRE.look.lean = FIRE_LEAN_WAS; }
    if (F0?.look?.steps === undefined) FIRE.look.steps = FIRE_STEPS; }   // (… and from before the flame's sample count was a number of the table: the count it was, the picture only)
  if ((k.tables.FIRE as Partial<typeof FIRE> | undefined)?.gas === undefined) FIRE.gas.on = false;   // (… one from before a fire's flame could be the solver's gas: the flame of noise)
  if ((k.tables.FIRE as Partial<typeof FIRE> | undefined)?.look === undefined) { FIRE.look.shimmer = 0; FIRE.look.plume = 0; FIRE.embers = 0; }   // (… one from before the flame's look had numbers: no shimmer over it, no smoke drawn off its tips, no particle embers — the picture only)
  if ((k.tables.FIRE as Partial<typeof FIRE> | undefined)?.size === undefined) FIRE.size = 0;   // (… and one from before its lamp was made as big as the flame: the size the level gave it — a meter's shadow ray stops that much short of the lamp)
  if (k.tables.DOZE) { const d = k.tables.DOZE as Partial<typeof DOZE>; if (d.screen !== undefined && d.idleSecs === undefined) DOZE.idleSecs = 1e9; }   // (a file from before a screen left lit blanked: it stayed lit)
  if (!k.tables.SENSE) SENSE.cpu = false;   // (a file from before the senses could run on the CPU: it read the renderer's values, and they are in it)
  if ((k.tables.SENSE as Partial<typeof SENSE> | undefined)?.grid === undefined) SENSE.grid = false;   // (…and one from before their smoke could stand on the coarse grid: it was the blobs)
  if ((k.tables.SENSE as Partial<typeof SENSE> | undefined)?.bounce === undefined) SENSE.bounce = false;   // (…and one from before the light model: the estimate was the direct light alone)
  if ((k.tables.SENSE as Partial<typeof SENSE> | undefined)?.moon === undefined) SENSE.moon = false;   // (…and one from before the estimate had the moon: a night's sky lit nobody)
  if ((k.tables.SENSE as Partial<typeof SENSE> | undefined)?.drift === undefined) SENSE.drift = false;   // (…and one from before the grid's smoke drifted: still air out of doors)
  if (!k.tables.POSEMATH) POSEMATH.singles = false;   // (a file from before a pose's arithmetic could be in singles: it was recorded in doubles, and replays in them — anim/skeleton.ts)
  if (!k.tables.SHAPES) SHAPES.on = false;   // (a file from before a box could say its shape: its vehicles were plain boxes — said again here; the level was stood up by shapesTape's word)
  if ((k.tables.SNEAK as { sideStop?: number } | undefined)?.sideStop === undefined) SNEAK.sideStop = 0;   // (…and from before the partner's step aside was asked along its last quarter metre: the spot alone, as it was)
  { const dt = k.tables.DOORTOOLS as { flowMax?: number; threatStuck?: number } | undefined;   // (…from before yards and very large rooms were left unswept, and before the first man in let a place he could not reach go)
    if (dt?.flowMax === undefined) DOORTOOLS.flowMax = 0;
    if (dt?.threatStuck === undefined) DOORTOOLS.threatStuck = 0; }   // (a recording that carries the key replays its own value)
  if (!k.tables.AIR_SIM) AIR_SIM.on = false;   // (a file from before the air was a simulated field: no field is fed — a picture only, the run is the same either way)
  if ((k.tables.DOORSHOT as { toll?: number } | undefined)?.toll === undefined) DOORSHOT.toll = 1;   // (… or from before a leaf stood twice plasterboard's toll: its doors go on drywall's own count, a later key of the same table)
  if (!k.tables.WALLS) assignDeep(WALLS as unknown as Record<string, unknown>, WALLS_DEFAULTS as unknown as Record<string, unknown>);
  if (typeof (k.tables.HANDS as { carryRide?: number } | undefined)?.carryRide !== 'number') { const Hn = knobTables().HANDS as { carryRide?: number } | undefined; if (Hn) Hn.carryRide = 0; }   // (a recording from before a carried body's limbs rode its carrier: they trail him as they did then)
}

// ================================================================================================================================ RECORD
export interface RecordOpts { seed?: number; note?: string; commit?: string; userAgent?: string; /** the page's HUD runs over this game (see EnvSnap.hud) */ hud?: boolean; }
/** The recorder the Game owns (`g.journal`). request() → 'pending'; the top of the next Game.update that may (canSave, no flash light in the set) takes the
 *  checkpoint, re-enters it, seeds the dice, puts the sensors into record and files frame 0; every update after is a frame until stop() hands back the file. */
export class SessionRecorder {
  state: JournalState = 'idle';
  /** pending: why it has not started yet; idle after a give-up / a stop: why */
  why = '';
  frame = 0;
  private opts: RecordOpts = {}; private pendingLeft = 0;
  private save: SaveState | null = null; private env: EnvSnap | null = null; private knobs: KnobsSnap | null = null; private seed = 0; private t0 = 0; private startedAt = '';
  private die: SimDie | null = null;
  private dts: [number, number][] = []; private views: number[][] = []; private cams: number[][] = []; private inputs: [number, InputRec][] = []; private digests: [number, GameDigest][] = [];
  private lastView = [NaN, NaN, NaN, NaN]; private lastCam = new Array<number>(8).fill(NaN); private lastKeys = '\n'; private lastButtons = NaN; private lastMouse = [NaN, NaN];
  private inUpdate = false;
  private slots: NonNullable<SessionFile['slots']> | null = null; private slotOwn = new Set<string>(); private serial = 0;
  /** which recording is under way (0 = none): a save in memory keeps the number it was made under (savestate.ts writeMem / loadMem) */
  get take(): number { return this.state === 'recording' ? this.serial : 0; }
  /** a running estimate of the streams' JSON size (the HUD's byte count; the sensors keep their own) */
  private bytes = 0;
  /** a stop asked for between frames: taken at the end of the next update, where the closing digest is the state a replay will compare (see stop) */
  private stopAfterNext: string | null = null;
  /** the last file made (stop) — kept for the console / a second delivery */
  last: SessionFile | null = null;
  /** called when a recording stops for any reason with the file (the page delivers it: ui/sessionrec.ts), or with null and the reason when a pending request gave up */
  onStop: ((f: SessionFile | null, why: string) => void) | null = null;
  constructor(private g: Game) {}

  get recording() { return this.state === 'recording'; }
  get stopping() { return this.stopAfterNext !== null; }
  /** ask for a recording: it starts at the top of the next update that may (see canStart); false if one is already asked for / under way */
  request(opts: RecordOpts = {}): boolean { if (this.state !== 'idle') { return false; } this.opts = opts; this.state = 'pending'; this.pendingLeft = JOURNAL_PENDING_FRAMES; this.why = canStart(this.g) ?? ''; return true; }
  /** Give up a pending request / stop a recording. Between frames (F10 arrives before the frame's update) the stop is DEFERRED to the end of the next update and
   *  the file arrives through onStop then — the closing digest must be the state straight after an update, which is what a replay compares (the HUD nulls an
   *  expired speech bubble after the update, for one); `immediate` finalises now, for a caller that knows nothing has touched the game since the last update
   *  (the headless recorder), or a second F10 that will not wait. Returns the file when it was made here and now. */
  stop(why = 'stopped', immediate = false): SessionFile | null {
    if (this.state === 'pending') { this.state = 'idle'; this.why = why; return null; }
    if (this.state !== 'recording') return null;
    if (!immediate && !this.inUpdate) { this.stopAfterNext ??= why; return null; }
    return this.finish(why, !this.inUpdate);
  }
  /** The game is about to be changed BETWEEN frames by something no recording files — a checkpoint loaded off a card or the panel, a restart off a card, a
   *  show or a mission's quit staging the encounter again (savestate.ts Checkpoints.load / loadMem, Game.restartEncounter): the recording ends HERE, whole to
   *  its last frame, with `why` as how it stopped — a tape that ran on across the change replayed as a break of the game's own at that frame — and a new one
   *  is asked for with the same options, so one F10 still covers the session, as one file per stretch (it begins at the next update that may: canStart).
   *  Inside an update nothing is done: what changes the game there came from that frame's own input, which is on the tape. A request still pending stays.
   *  `lost`: the one change inside an update that no replay can make — a save in memory from before the recording put back (savestate.ts loadMem) — ends it
   *  there too, the frame filed whole and the file marked (`meta.lost`): that last frame is the Enter a replay takes another way, so no closing digest is
   *  taken of it and tools/qa/replay.ts plays to the frame before.
   *  Its checks: tools/qa/journalcut-check.ts, tools/qa/journalload-check.ts. */
  cut(why: string, lost = false): void {
    if (this.state !== 'recording' || (this.inUpdate && !lost)) return;
    const opts = this.opts; this.finish(why, !this.inUpdate, lost && this.inUpdate); this.request(opts);
  }
  /** THE STORE IS PART OF WHAT A RECORDING REPLAYS ON: the bare Enter once down (the frame's own input) goes back to a stored checkpoint. The file carries the
   *  store's rows as they stood at frame 0 (`slots`: savestate.ts tapeBegin; a save or an import made BETWEEN frames ends the recording first — Checkpoints.write
   *  / importText call cut — so the rows of frame 0 and the saves the updates make are all there is). A replay writes the recording's own saves again as it
   *  runs; the TEXT of any other slot an update reads is filed here with its frame, once, and tools/qa/replay.ts puts it in the store before that frame.
   *  slotWrote: an update of this recording wrote `key` (the replay will too). slotRead: an update read it (Checkpoints.load). */
  slotWrote(key: string): void { if (this.state === 'recording' && this.inUpdate) this.slotOwn.add(key); }
  slotRead(key: string, text: () => string | null): void {
    if (this.state !== 'recording' || !this.inUpdate || !this.slots || this.slotOwn.has(key)) return;
    const t = text(); if (t) { this.slots.loads.push([this.frame, key, t]); this.slotOwn.add(key); this.bytes += t.length + 24; }
  }
  /** `between`: finished between two frames (cut(), an immediate stop with no update running). The closing digest taken there is, on a page, the state AFTER
   *  the HUD's one write of the last frame (hudExpireBubbles below), not the one straight after its update: the file says so (`meta.between`) and a replay
   *  makes that write before it compares that digest (tools/qa/replay.ts). */
  private finish(why: string, between = false, lost = false): SessionFile {
    const g = this.g; this.stopAfterNext = null;
    if (this.inUpdate) { g.sensors.endFrame(); this.frame++; this.inUpdate = false; }   // (stopped from inside a frame — a hook, the cap: that frame is in the file whole)
    if (lost) { /* no digest of a frame no replay reaches */ }
    else if (!this.digests.length || this.digests[this.digests.length - 1][0] !== this.frame - 1) this.digests.push([this.frame - 1, digestGame(g, true)]);
    else between = false;   // (the last frame was a beat: its digest is the one straight after the update)
    const sensors = g.sensors.journal(); g.sensors.live();
    if (this.die) { if (Math.random === this.die && this.die.sys) Math.random = this.die.sys; this.die = null; }   // (the die it displaced goes back — unless somebody has since put their own in, which stays)
    const seconds = this.dts.reduce((s, [n, dt]) => s + n * dt, 0);
    const file: SessionFile = { v: lost || this.slots?.loads.length ? JOURNAL_VERSION_SLOTS : JOURNAL_VERSION, game: 'twopointsix', recordedAt: this.startedAt, commit: this.opts.commit, note: this.opts.note, seed: this.seed, level: levelFingerprint(g), save: this.save!, env: this.env!, knobs: this.knobs!,
      frames: this.frame, dt: this.dts, view: this.views, cam: this.cams, input: this.inputs, sensors, ...(this.slots ? { slots: this.slots } : {}), digests: this.digests, meta: { seconds: +seconds.toFixed(2), stopped: why, userAgent: this.opts.userAgent, ...(between ? { between: true } : {}), ...(lost ? { lost: true } : {}) } };
    this.state = 'idle'; this.why = why; this.last = file;
    try { this.onStop?.(file, why); } catch (e) { reportError('journal.onStop', e); }
    return file;
  }

  /** Game.update's first statement: start if pending and allowed, then file this frame's dt / view / camera / input and open the sensors' frame */
  frameStart(dt: number, input: Input, cam: FollowCamera, view: View): void {
    if (this.state === 'idle') return;
    const g = this.g;
    if (this.state === 'pending') {
      const why = canStart(g);
      if (why) { this.why = why; if (--this.pendingLeft <= 0) { this.state = 'idle'; this.why = `gave up: ${why}`; try { this.onStop?.(null, this.why); } catch (e) { reportError('journal.onStop', e); } } return; }
      this.begin();
    }
    if (this.frame >= JOURNAL_MAX_FRAMES) { this.finish('cap'); return; }
    const f = this.frame; this.inUpdate = true;
    // dt (run-length)
    const L = this.dts[this.dts.length - 1]; if (L && L[1] === dt) L[0]++; else { this.dts.push([1, dt]); this.bytes += 26; }
    // canvas
    const vw = [view.clientWidth, view.clientHeight, view.width, view.height]; if (vw.some((x, i) => x !== this.lastView[i])) { this.views.push([f, ...vw]); this.lastView = vw; this.bytes += 28; }
    // camera as this update will read it
    const cv = [cam.target[0], cam.target[1], cam.target[2], cam.yaw, cam.pitch, cam.distance, cam.aspect, cam.fov]; let mask = 0; const vals: number[] = [];
    cv.forEach((x, i) => { if (!Object.is(x, this.lastCam[i])) { mask |= 1 << i; vals.push(x); this.lastCam[i] = x; } });
    if (mask) { this.cams.push([f, mask, ...vals]); this.bytes += 10 + vals.length * 19; }
    // input
    const rec: InputRec = {}; let any = false;
    const keys = [...input.keys].sort(); const kj = keys.join('|'); if (kj !== this.lastKeys) { rec.k = keys; this.lastKeys = kj; any = true; }
    if (input.pressed.size) { rec.p = [...input.pressed].sort(); any = true; }
    if (input.clicked) { rec.c = input.clicked; any = true; }
    if (input.buttons !== this.lastButtons) { rec.b = input.buttons; this.lastButtons = input.buttons; any = true; }
    if (input.mouseX !== this.lastMouse[0] || input.mouseY !== this.lastMouse[1]) { rec.m = [input.mouseX, input.mouseY]; this.lastMouse = [input.mouseX, input.mouseY]; any = true; }
    if (input.wheel) { rec.w = input.wheel; any = true; }
    if (any) { this.inputs.push([f, rec]); this.bytes += 12 + (rec.k ? 10 + rec.k.length * 8 : 0) + (rec.p ? 8 + rec.p.length * 8 : 0) + (rec.m ? 40 : 0) + (rec.c !== undefined ? 6 : 0) + (rec.b !== undefined ? 6 : 0) + (rec.w !== undefined ? 10 : 0); }
    g.sensors.beginFrame();
  }
  /** Game.update's last statement: close the sensors' frame, digest on the beat, count the frame */
  frameEnd(): void {
    if (this.state !== 'recording' || !this.inUpdate) return;
    this.inUpdate = false; this.g.sensors.endFrame();
    if ((this.frame + 1) % JOURNAL_DIGEST_EVERY === 0) { this.digests.push([this.frame, digestGame(this.g, true)]); this.bytes += 3000; }
    this.frame++;
    if (this.stopAfterNext !== null) this.finish(this.stopAfterNext);
  }
  private begin(): void {
    const g = this.g;
    this.save = saveState(g, { label: 'session journal', kind: 'harness' });
    loadState(g, JSON.parse(JSON.stringify(this.save)) as SaveState);   // (a copy: loadState keeps nothing of it, but the file must be the bytes as SAVED, untouched by anything a load might normalise in place)
    this.env = envSnapshot(g); this.env.hud = !!this.opts.hud; this.knobs = knobsSnapshot(g);
    this.seed = (this.opts.seed ?? ((Date.now() ^ Math.floor(Math.random() * 0x7fffffff)) >>> 0)) >>> 0;
    this.die = simDie(this.seed, Math.random); Math.random = this.die;   // the simulation's dice from frame 0: seeded, the displaced die on .sys for the mixer / the solver (dice.ts sysRandom) and for the stop
    g.sensors.record();
    { const t = g.saves?.tapeBegin(); this.slots = t && (t.index !== null || t.lastAuto) ? { ...t, loads: [] } : null; this.slotOwn.clear(); this.serial++; }
    this.frame = 0; this.dts = []; this.views = []; this.cams = []; this.inputs = []; this.digests = []; this.bytes = 0;
    this.lastView = [NaN, NaN, NaN, NaN]; this.lastCam.fill(NaN); this.lastKeys = '\n'; this.lastButtons = NaN; this.lastMouse = [NaN, NaN];
    this.startedAt = new Date().toISOString(); this.t0 = g.time; this.state = 'recording'; this.why = '';
  }
  /** the HUD's tag while pending / recording ('' when idle): 'REC f=1234 · 38 KB' */
  hudText(): string {
    if (this.state === 'pending') return `REC waits: ${this.why || '…'}`;
    if (this.state !== 'recording') return '';
    return `REC f=${this.frame} · ${(this.approxBytes() / 1024).toFixed(0)} KB${this.stopAfterNext !== null ? ' · stopping' : ''}`;
  }
  /** the file's size so far, roughly (the sensors' own estimate + the streams here) */
  approxBytes(): number { return 38000 + this.bytes + this.g.sensors.approxBytes(); }   // (the checkpoint + env + knobs ≈ 38 KB, then the streams)
  summary(): string { return this.state === 'idle' ? `journal idle${this.why ? ` (${this.why})` : ''}${this.last ? ` · last: ${this.last.frames} frames / ${this.last.meta.seconds} s` : ''}` : this.state === 'pending' ? `journal pending — ${this.why}` : `journal RECORDING · frame ${this.frame} · ${(this.g.time - this.t0).toFixed(1)} s · seed ${this.seed} · ≈ ${(this.approxBytes() / 1024).toFixed(0)} KB · ${this.g.sensors.summary()}`; }
}
/** why a recording cannot start on this frame (null = it can): the checkpoint's own refusals (a show, daylight, a unit, Sam down / mid-action, a thing in the air, a
 *  body falling — savestate.ts canSave, unforced) plus a flash light alive in the set (a muzzle flash, a stroke: gone in a frame or two, and not in a checkpoint) */
export function canStart(g: Game): string | null {
  const why = canSave(g, false); if (why) return why;
  if (g.engine.lights.lights.some(l => l.ttl >= 0 || l.transient)) return 'a flash is still lit';
  return null;
}

/** The HUD's one write to the game (ui/bubbles.ts, after each frame's update on the game page): a speech bubble past its duration is nulled on its speaker — every
 *  reader in the game guards on the clock anyway, but the field is state (a checkpoint files it, the digest prints it, needs.ts reads its tail). Here so the page's
 *  bubbles and a REPLAY of a page recording (tools/qa/replay.ts, env.hud) make the very same write at the very same point; the headless harness never calls it. */
export function hudExpireBubbles(game: Game): void {
  const speakers = game.civilians.length ? [...game.guards, ...game.civilians] : game.guards;
  for (const s of speakers) { const b = s.bubble; if (!b || (s.state === 'dead' && !s.wound) || (s.civ && s.civ.state === 'gone')) continue; if (game.time - b.t > b.dur) s.bubble = null; }
  const pl = game.player, sb = pl.bubble; if (sb && !pl.down && game.time - sb.t > sb.dur) pl.bubble = null;
}

// ================================================================================================================================ REPLAY (the reading side of the streams)
/** Walks a file's per-frame streams: applyFrame(f, input, cam, view) puts frame f's dt / canvas / camera / input in place and returns dt. Frames in order only. */
export class SessionPlayer {
  private dtI = 0; private dtLeft = 0; private viewI = 0; private camI = 0; private inputI = 0;
  private buttons = 0; private mouse: [number, number] = [0, 0]; private keys: string[] = [];
  constructor(readonly file: SessionFile) { this.dtLeft = file.dt.length ? file.dt[0][0] : 0; }
  get frames() { return this.file.frames; }
  applyFrame(f: number, input: Input, cam: FollowCamera, view: View): number {
    const F = this.file;
    // dt
    while (this.dtLeft <= 0 && this.dtI + 1 < F.dt.length) { this.dtI++; this.dtLeft = F.dt[this.dtI][0]; }
    const dt = F.dt[this.dtI]?.[1] ?? 1 / 60; this.dtLeft--;
    // canvas
    while (this.viewI < F.view.length && F.view[this.viewI][0] <= f) { const r = F.view[this.viewI++]; view.clientWidth = r[1]; view.clientHeight = r[2]; view.width = r[3]; view.height = r[4]; }
    // camera
    let camTouched = false;
    while (this.camI < F.cam.length && F.cam[this.camI][0] <= f) {
      const r = F.cam[this.camI++]; const mask = r[1]; let j = 2; camTouched = true;
      for (let i = 0; i < 8; i++) if (mask & (1 << i)) { const x = r[j++]; if (i < 3) cam.target[i] = x; else if (i === 3) cam.yaw = x; else if (i === 4) cam.pitch = x; else if (i === 5) cam.distance = x; else if (i === 6) cam.aspect = x; else cam.fov = x; }
    }
    if (camTouched) cam.rebuild();
    // input: this frame's record if there is one; the edge sets are this frame's alone either way
    let rec: InputRec | null = null;
    while (this.inputI < F.input.length && F.input[this.inputI][0] <= f) { const [rf, r] = F.input[this.inputI++]; if (rf === f) rec = r; }
    if (rec?.k) this.keys = rec.k; if (rec?.b !== undefined) this.buttons = rec.b; if (rec?.m) this.mouse = [rec.m[0], rec.m[1]];
    input.keys.clear(); for (const k of this.keys) input.keys.add(k);
    input.pressed.clear(); for (const k of rec?.p ?? []) input.pressed.add(k);
    input.clicked = rec?.c ?? 0; input.buttons = this.buttons; input.mouseX = this.mouse[0]; input.mouseY = this.mouse[1]; input.wheel = rec?.w ?? 0;
    return dt;
  }
  /** the digests by frame, for the comparison after each update */
  digestAt(f: number): GameDigest | null { const d = this.digestMap ??= new Map(this.file.digests.map(([k, v]) => [k, v])); return d.get(f) ?? null; }
  private digestMap: Map<number, GameDigest> | null = null;
}

/** a file's sanity, in words ('' = fine): version, the blocks present, the streams' shapes */
export function checkSessionFile(F: Partial<SessionFile> | null | undefined): string {
  if (!F || typeof F !== 'object') return 'not an object';
  if (F.game !== 'twopointsix') return `not a twopointsix session (game ${JSON.stringify(F.game)})`;
  if (F.v !== JOURNAL_VERSION && F.v !== JOURNAL_VERSION_SLOTS) return `version ${F.v} (this build reads ${JOURNAL_VERSION} and ${JOURNAL_VERSION_SLOTS})`;
  if (!F.save || typeof F.save !== 'object') return 'no checkpoint (save)';
  if (typeof F.seed !== 'number' || typeof F.frames !== 'number' || F.frames < 0) return 'seed / frames missing';
  for (const k of ['dt', 'view', 'cam', 'input', 'digests'] as const) if (!Array.isArray(F[k])) return `stream '${k}' missing`;
  if (!F.sensors || !Array.isArray(F.sensors.keys) || !Array.isArray(F.sensors.recs)) return 'sensors block missing';
  if (!F.env || !F.knobs) return 'env / knobs missing';
  if (F.slots !== undefined) { const S = F.slots as Partial<NonNullable<SessionFile['slots']>> | null;
    if (!S || typeof S !== 'object' || (S.index !== null && typeof S.index !== 'string') || !Array.isArray(S.loads) || S.loads.some(l => !Array.isArray(l) || typeof l[0] !== 'number' || typeof l[1] !== 'string' || typeof l[2] !== 'string')) return 'slots block malformed'; }
  const n = (F.dt as [number, number][]).reduce((s, r) => s + (r[0] | 0), 0); if (n !== F.frames) return `dt runs cover ${n} frames, the file says ${F.frames}`;
  return '';
}
