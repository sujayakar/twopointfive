// WHAT A RUN'S RECIPE CARRIES OF THE GAME'S SETTINGS: the knob tables by name, the engine-side facts the simulation reads, and the two laid back over a game (the kept
// replay's stand, replay/recipestand.ts; tools/qa/rerun.ts; the harness). `applyKnobs` also says what a tape from before a key means: the key's rule off.
// The file keeps its old name from the F10 recorder it once also held. That recorder is gone.
import type { Game } from './game';
import { DESTRUCTION } from './destructionSwitch';
import { LIVED } from '../scene/lived';
import { SHAPES } from '../scene/shapes';
import { cutConfig } from '../scene/cut';
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

/** the module knob tables a recording carries and a replay applies (kept per machine or flipped on the panel: the maintainer's values are not the harness's) */
/** the knob tables a recording captures — LAZY: journal.ts sits on the game.ts ↔ guards.ts import cycle, and the browser bundle hoists module bindings, so an eager object
 *  literal here captured `undefined` for tables whose module had not initialised yet (the recorder threw on its first page frame; headless import order hid it) */
export function knobTables(): Record<string, object> { return knobsNamed(JOURNAL_KNOBS); }
// (the tables themselves, and why each is in a recording, are knobs.ts's now — GLASSBREAK: rounds, sound and men pass through its holes, gameplay, while the dust table is
//  presentation and stays out; DOORSTAY: whether any leaf has a closer is gameplay for everyone who walks, looks or listens through a doorway; HANDLE: where a man stands to
//  open a door; DOORMANNERS: who waits for whom at a frame, both sides of a squad map)

/** the engine-side facts the simulation reads (see the head) + the fixtures' restrike stagger by target name */
export interface EnvSnap { debrisCap: number; emissiveScale: number; flashlightRadius: number; markings: boolean; particlesOn: boolean;
  /** recorded under the game page's HUD (main.ts). The HUD used to null an expired speech bubble on its speaker after each update, and a replay of such a
   *  file did the same; it writes nothing of the game now (a line is over when its time is), and the field only says where the file was made */
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
