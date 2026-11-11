// Renderer orchestration: owns GPU resources and encodes the frame.
import { Gpu, makeBuffer, PIPE_QUEUE, laterProgress } from './gpu/device';
import { GpuTimer } from './gpu/timer';
import { BoxWorld, planBoxes, type Box } from './scene/boxes';
import { FRAME_BYTES, FRAME_HEAD_BYTES, MAX_CARDS, FrameFlags, FrameParams, wallSidesConfig, createSceneBindGroup, createSceneLayout, writeFrame, writeFrameOut, meshBinding, WeatherFrame, noWeather, lookConfig } from './render/frame';
import { GBuffer, SkinnedDrawable, lookAmounts } from './render/gbuffer';
import { RadianceCascades, RcConfig, defaultRcConfig } from './render/rc';
import { DiceRing, ringPoints, ringSlot } from './render/diceRing';
import { LightingPasses, type FireRecord } from './render/passes';
import { LightSet, cardSlotsFor, type RtLight } from './render/lights';
import { emissiveDayK } from './render/photometry';
import { lookVeilFrame } from './render/lookveil';
import { PowderField, powderDrawWord, powderWarm } from './render/powderField';
import { PlaceField } from './render/placeField';
import { PlaceMap, type PlaceGap } from './scene/placePlan';
import { WallSides } from './render/wallSides';
import { lookGrade, lookGradeFrame } from './render/lookgrade';
import { lookExpoFrame, lookExpoBound, lookNightSkyE } from './render/lookexpo';
import { planOpeningsSteps, openAreas, openingLooks, openingReach, openingsKey, leavesKey, LOOK_OPENINGS, type Opening } from './scene/openings';
import { buildWallRaster, wallRasterHandOnSteps, rasterRect, reachMaskSome, REACH_WORK, REACH_PACE, type ReachJob, type ReachOnly } from './render/reach';
import { Steps, WHOLE, clock, spend, type Pace } from './render/steps';
import { PLAN_PACE } from './render/planPace';
import { MeshShadows } from './render/meshShadows';
import { RigFit, RIG_KPAD } from './render/sdfrig';
import { Decals } from './render/decals';
import { Particles } from './render/particles';
import { TexturePool } from './render/texpool';
import { reachLevel } from './render/reach';
import { ReachKeep, type KeepGuest } from './render/reachKeep';
import { WORLD } from './scene/world';
import { FollowCamera } from './host/camera';
import { Vec3, m4, v3, smoothstep } from './math/vec';

import { RenderSettings, defaultSettings } from './settings';
import { FogSight, FOG_SIGHT_BYTES, type FogFrame } from './render/fogofwar';
import { outlineBoxes, type OutlineFrame } from './render/outline';
export type { RenderSettings } from './settings';
export { defaultSettings } from './settings';

/** the identity of a cascade GRID (what needs its own buffers / dice volume): probe counts, directions, cascade count — the interval length is a live uniform (RadianceCascades.setInterval0) */
const rcGridKey = (c: RcConfig) => `${c.nx}x${c.ny}x${c.nz}/d${c.d0}/c${c.cascades}`;

/** What the renderer needs from the smoke sim (smoke/smoke.ts; the readers are shaders/smoke_read.wgsl): the table of loose bounds round the smoke (uniform, eight records), the brick atlas,
 *  the page texture that says which brick is where (scene binding 14), how many bound records to walk (0 = no smoke), and the sim's encoder. */
export interface SmokeBinding { uniform: GPUBuffer; atlasView: GPUTextureView; pageView: GPUTextureView; count: number; /** look only (smoke/dustTint.ts): some brick holds destruction dust → the frame's bit that the volumetric pass's dust look asks (smokeDrawWords) */ /** look only: the system lets a can's smoke out in bands (the smoke lab's row, SmokeSystem.streak) → a tint is drawn as a tint alone */ streaks?: boolean; dusty?: boolean; encode?: (enc: GPUCommandEncoder, ts: (n: string) => GPUComputePassTimestampWrites | undefined) => void; }

const NO_BEAM = (): boolean => false;
/** no doors' lines: one list, for what is kept of them stands while the list does */
const NO_GAPS: readonly PlaceGap[] = [];
/** glassLightMasks' sample lattice: three stations along a pane × a cell just off either face (module consts — review R1.4: no per-light-per-pane literals) */
const GLASS_STATIONS: readonly number[] = [-0.7, 0, 0.7], GLASS_SIDES: readonly number[] = [-0.55, 0.55];

/** the walls as the daylight's reach masks read them, with their jump field: nobody's state (render/reach.ts ReachJob.own) */
type OpenWalls = { solid: Uint8Array; jumps?: Uint8Array };
type Rect4 = [number, number, number, number];
/** ONE OPENING THE LOOK LIGHTS (engine.ts dayOpenings): where it is now, its light, its reach mask with the place that was decided at, the mask in
 *  the making (of the walls and the place it was begun with, decided again from `only` or whole), and what the next one is OWED: the rects the
 *  walls have changed in since the last was begun, or all of it. */
interface OpenRec { o: Opening; l: RtLight; mask: Uint32Array | null; at: Opening | null;
  job: (ReachJob & { own: OpenWalls; at: Opening; only: ReachOnly | undefined }) | null; owed: Rect4[] | 'all' }
/** rects of changed walls an opening's next mask is owed one by one; past these it is owed all of it */
const OWED_MOST = 8;
/** was a mask decided at `a` decided for the light `b` gives: the same points along the same wall, under the same roof (all that render/reach.ts
 *  reads of a light's place: a sill or a head that moved is the same mask) */
const sameSpot = (a: Opening | null, b: Opening): boolean => a !== null && (a === b || (a.mid[0] === b.mid[0] && a.mid[2] === b.mid[2] && a.dir[0] === b.dir[0] && a.dir[2] === b.dir[2] && a.half[0] === b.half[0] && (a.mid[1] > WORLD.ceilingY) === (b.mid[1] > WORLD.ceilingY)));
/** an opening's mask is an OLD one (it serves, and is decided again): the walls changed since it was begun, or it was decided somewhere else */
const maskOld = (r: OpenRec): boolean => r.mask !== null && (r.owed === 'all' || r.owed.length > 0 || !sameSpot(r.at, r.o));
/** an opening has a mask to be decided: none yet, one in the making, or an old one (a check's too) */
export const openMaskOwed = (r: OpenRec): boolean => r.mask === null || r.job !== null || maskOld(r);
export class Engine {
  device: GPUDevice;
  world: BoxWorld;
  lights: LightSet;
  /** the lights' reach masks and the work of keeping them after a wall changes (render/reachKeep.ts): the picture's, a step of it every drawn frame */
  reach: ReachKeep;
  rc: RadianceCascades;
  gbuffer: GBuffer;
  passes: LightingPasses;
  timer: GpuTimer;
  /** the canvas's size in the window, by whoever reads it once a frame (host/viewsize.ts); null = the canvas is asked */
  view: { readonly clientWidth: number; readonly clientHeight: number } | null = null;
  /** the render-target pool the g-buffer and the lighting passes share (render/texpool.ts): a size the adaptive ladder returns to costs no allocation */
  pool: TexturePool;
  sceneLayout: GPUBindGroupLayout;
  sceneBG: GPUBindGroup | null = null;
  frameBuf: GPUBuffer; frameData = new ArrayBuffer(FRAME_BYTES);
  /** SHARP DIRECT this frame (settings.sharpDirect ∧ TAAU ∧ no debug view ∧ internal ≠ output — resize decides), the OUTPUT frame block (the same bytes with screen = the output size: frame.ts writeFrameOut)
   *  and its scene group, group 0 of every pass that runs at the output size then */
  sharpOn = false; frameBufOut: GPUBuffer; frameDataOut = new ArrayBuffer(FRAME_BYTES); sceneBGOut: GPUBindGroup | null = null;
  /** FOG OF WAR (rust/game_rules/src/stealth/fogofwar fills it every frame; host/drawpage.ts hands the frame's object over once): null / `on` false = no pass, the frame as it was */
  fogFrame: FogFrame | null = null;
  /** THE OPEN FIRES to draw this frame (rust/game_rules/src/air/fire.rs writes the list every update; empty = none): the composite adds each one's flame, coals and the smoke off its tips (passes.setFires; its embers are particles) */
  fires: FireRecord[] = [];
  /** the look's numbers every fire is drawn with (rust/game_rules/src/air/fire.rs fireDraws; fire.wgsl FireLook's twelve, in order); null = none said yet */
  fireLook: number[] | null = null;
  /** which of the frame's boxes are thin lines (a laser's beam, a tracer's streak) and which of those the air shows, with its bead; null = none said (the host's word) */
  beams: { is(b: Box): boolean; airOf(b: Box): Box | null | undefined; /** the boxes to look among (absent = the frame's own) */ list?: Box[] } | null = null;
  /** the outline in the dark (render/outline.ts): the host's frame for it, null = never asked. Written by the host once a frame;
   *  render() adds the box on the picture its own camera gives and the exposure */
  outline: OutlineFrame | null = null;
  /** … its SIGHT TABLE (render/fogofwar.ts FogSight — the scene group's binding 11; dispatched at the head of a frame only while the game asks for the full
   *  picture under the fog), the 1 × 1 stand-in the table's own pass is bound with, and that pass's scene group (every resource sceneBG's but binding 11) */
  fogSight: FogSight; private fogSightStand: GPUBuffer; private sceneBGPlain: GPUBindGroup | null = null;
  dummyStorage: GPUBuffer;
  smoke: SmokeBinding;
  settings: RenderSettings = defaultSettings();
  /** what the weather wants drawn in the outdoor air this frame (rust/game_rules/src/air/weather.rs writes it before render; all zero = nothing, the default) */
  weather: WeatherFrame = noWeather();
  /** the view's look-at point as of the last render() (the smoke solver spends its pool by distance from it — smoke.ts sparse.viewBias / farCells) */
  viewFocus: Vec3 = [0, 0, 0];
  frameIdx = 0;
  /** effectiveScale = internal height / (CSS height × dpr), the true pixel ratio (the cost model's unit; = capScale × renderScale); capScale = min(1, maxInternalHeight / (CSS height × dpr)),
   *  what renderScale 1.0 gets on this window */
  effectiveScale = 1; capScale = 1; lastDt = 1 / 60;
  /** internal (render) size — what the g-buffer and every lighting pass run at */
  width = 0; height = 0;
  /** OUTPUT size = the canvas backing store: with TAAU the cap's rows (min(maxInternalHeight, CSS height × dpr)) whatever the render scale — the resolve upsamples internal → this and the
   *  bloom / tonemap / sharpen run here; without it = the internal size, as it always was */
  outWidth = 0; outHeight = 0;
  // ---- TAAU bookkeeping (see RenderSettings.taau): this frame's jitter, the jittered matrices the frame uniform carries, and what the NEXT frame reprojects with
  /** this frame's jitter in internal pixels (0, 0 with TAAU off, under a debug view, or with taauPhases 0) */
  jitter: [number, number] = [0, 0];
  /** the previous render's jitter (frame.jitter.zw: a history keyed on the unjittered pixel — the gather's reservoirs — takes both jitters out of its reprojection) */
  private prevJitter: [number, number] = [0, 0];
  private vpJ = m4.create(); private invVpJ = m4.create();
  /** the previous RENDER's jittered / unjittered view-projection (what its depth and histories were made with) and camera pose (cut detection); valid once havePrev */
  private prevVpJ = m4.create(); private prevVpU = m4.create(); private prevCamPos: Vec3 = [0, 0, 0]; private prevCamFwd: Vec3 = [0, 0, -1]; private havePrev = false;
  /** the unjittered camera moved since the last render (direct-motion lane: passes.restirMotionLive keys on it) */
  camMovedAny = false;
  private taauWas = false; private debugWas = 0;
  /** why the TAAU history was last dropped, and how many times (the Tab panel words it; QA counts it) */
  taaResets = 0; lastTaaReset = '';
  /** THE LOOK PROTOTYPE's loose snow in the air (render/powderField.ts: the powder the wind picks up and a boot kicks, carried by `air` below, drawn by the
   *  composite). Built the first frame the look page has lying snow, an air field and either row up; dropped when any of those goes (the rows at 0, the air
   *  field replaced). Picture only. src/picture/snow/fx.ts files the kicks into it. */
  powder: PowderField | null = null; private powderAir: unknown = null;
  /** THE LOOK PROTOTYPE's place field (render/placeField.ts: one level of light a room, which the One exposure row shows every place by): built with the
   *  engine and kept; null on a device that cannot make it */
  diceRing: DiceRing | null = null;   // the look page's steady bounce (render/diceRing.ts): made at the first day frame with the row up
  /** THE LOOK PROTOTYPE's wall sides (render/wallSides.ts: which place each cell of the floor plan is in, carried by the frame so that every cache read keeps to its own side of a wall): the look page's row `walls` */
  readonly wallSides = new WallSides();
  /** HOW SMOKE IS DRAWN, AS ROWS (render/smokeLook.ts writes them; and all at 0 the frame's two words are 0 = the picture as
   *  it was). creases: the soft light on a sample dimmed by the smoke round it (0‥2.5) · selfK: the self-shadow's extinction (0 = the shader's own 6) · selfReach: how far
   *  toward the light it looks, m (0 = 0.72) (vol_common.wgsl). The second word's bit 1 is up while some
   *  brick of the bound system's smoke holds dust (SmokeBinding.dusty), and with it down the march reads smoke as plain smoke (dust_look.wgsl dustAlive). */
  readonly smokeDraw = { creases: 0, selfK: 0, selfReach: 0, ahead: 0 };   // (ahead: how much a lamp behind smoke glows through it, 0‥1, vol_common.wgsl smokeAheadLobe; 0 = smoke only dims a lamp behind it, as it was)
  private smokeDrawWords(): [number, number] {
    const d = this.smokeDraw, b = (x: number, k: number) => Math.max(0, Math.min(255, Math.round(x * k)));
    return [(b(d.creases, 100) | b(d.selfK, 4) << 8 | b(d.selfReach, 50) << 16 | Math.min(100, b(d.ahead, 100)) << 24) >>> 0, this.smoke?.dusty ? 2 : 0];
  }
  /** … and the one map of the level both read (placeField.ts PlaceMap); which tables each frame block was last sent (WallSides.made; −1 = never) */
  readonly placeMap = new PlaceMap(); private sidesSent = -1; private sidesSentOut = -1;
  place: PlaceField | null = null; private placeWas: PlaceField | null = null; private placeDead = false;
  /** THE LOOK PROTOTYPE's eye (render/placeField.ts: the place field's own, eased on the GPU): `snap` = it takes its next reading outright and the field is solved to rest first (the first frame, a restart: src/picture/air.ts says; a still sets it by hand). Picture only. */
  lookEye = { snap: true };
  /** this frame's shadeCfg and eyeCfg as the frame was handed them (a tool's and the debugger's) */
  lookShadeCfg: [number, number, number, number] | undefined = undefined; lookEyeCfg: [number, number, number, number] | undefined = undefined;
  private lookGradeCfg: [number, number, number, number] | null = null;   // (this frame's gradeCfg, render/lookgrade.ts: the day's contrast, which the one exposure model tones a place by)
  /** THE LOOK PROTOTYPE's daylight at openings (scene/openings.ts; the panel's "Daylight at openings" row): one area light just inside each place the level
   *  is open to the sky by (a window, an open door, a blown hole, a passage's mouth). THE PICTURE'S ALONE: the lights are this engine's (`dayLights`, written
   *  here every frame from this method's own records and handed to the upload), never in the light set the game reads and a checkpoint holds, and their
   *  masks leave no note in reach.ts's side tables: nothing in the update, a checkpoint or a recording can tell the row is on. The openings are found again when the level's boxes have changed and then stood still for a few frames, what
   *  each door's leaf covers when a leaf swings, and the strength and colour are set every frame from the sky and the sun that are up. Each light has the
   *  cells it can reach (render/reach.ts, as a lamp has: without them it is a candidate in every room of the level), decided here against this
   *  method's own raster of the walls, a frame's share of the work at a time, the open and the near first; the old one serves until the new one is whole.
   *  None with the row at 0. At every hour: one is out only while the picture would show nothing of what it looks out on. */
  private openRecs: OpenRec[] = [];
  private openLevel: unknown = null; private openSerial = -1; private openTick = 0; private openSeen = NaN; private openPlanned = NaN; private openWait = 0; private openLeaves = NaN;
  private openWalls: OpenWalls | null = null;
  /** the masks have had their first frame on this level with the rooms row up: from then on none is decided whole at once (lookOpeningLights, the reach masks) */
  private openBegun = false;
  /** the openings being found again and the walls being read again, a little a frame (render/steps.ts, render/planPace.ts): what was last planned serves
   *  until it is whole. `openJobKey` = the boxes' key (openingsKey) as the job took them. */
  private openJob = new Steps<{ plan: Opening[]; walls: OpenWalls; changed: boolean; rect: Rect4 | null }>(); private openJobKey = NaN;
  /** THE PLANNERS' PACE (render/planPace.ts): the drawn frames left in which every plan is made at once whatever the pace: a page's first frames, and after the level has been put back to another state under the picture (startPlansOver:
   *  a restart, a save put back). The level then is not the one the plans in use were made for. */
  private planFresh = LOOK_OPENINGS.settle + 2;
  /** this drawn frame's allowance of planning: render's first words on the pace, and a check's */
  private framePace(): Pace {
    const on = PLAN_PACE.on && this.planFresh <= 0; if (this.planFresh > 0) this.planFresh--;
    return on ? { left: Infinity, ms: PLAN_PACE.ms } : WHOLE;
  }
  /** THE LEVEL IS ANOTHER STATE OF ITSELF (a restart, a save put back): what the planners have in hand was of the state before, so it is
   *  given up, and for a few drawn frames every plan is made at once, as on a page's first frames. Not for a change of light or weather: the level
   *  stands, and a job in hand is still of it. */
  startPlansOver(): void { this.planFresh = LOOK_OPENINGS.settle + 2; this.wallSides.drop(); this.openJob.drop(); }
  /** EVERYTHING THAT CAN CHANGE UNDER IT IS TAKEN BEFORE ITS FIRST STOP: a copy of what the planners ask of each static (scene/boxes.ts planBoxes) and the
   *  doors' lines. So it is finished whatever moves meanwhile, and what it returns is of ONE level, as it stood in the call that began it. In turn: the
   *  openings the walls leave (scene/openings.ts), the walls as the reach masks read them (render/reach.ts: full-height walls, a doorway open ground that
   *  is ranked down, as the lamps' own raster has it, render/reachKeep.ts), and whether that raster differs from the one in use, with WHERE (`rect`, in
   *  metres; null with `changed` = no telling: all of it) (where the walls only opened, a blast, the old raster's jump field is handed on: reach.ts
   *  wallRasterHandOnSteps). */
  private *openReplan(): Generator<void, { plan: Opening[]; walls: OpenWalls; changed: boolean; rect: Rect4 | null }, void> {
    const boxes = planBoxes(this.world.statics), doors = (lookConfig.doorGaps?.() ?? []).map(g => ({ hinge: [g[0], g[1]] as [number, number], closedDir: Math.atan2(g[3] - g[1], g[2] - g[0]), width: Math.hypot(g[2] - g[0], g[3] - g[1]) }));
    yield;
    const plan = yield* planOpeningsSteps(boxes); yield;
    const walls: OpenWalls = { solid: buildWallRaster(boxes, doors) }; yield;
    const was = this.openWalls, d = was ? yield* wallRasterHandOnSteps(was, walls) : true;
    return { plan, walls, changed: d !== null, rect: d === null || d === true ? null : rasterRect(d) };
  }
  /** this frame's opening lights, for the upload (lights.ts upload `own`) and the glass masks */
  private dayLights: RtLight[] = [];
  /** A SHARE OF ONE OPENING'S MASK: at most `steps` sight-line steps of it; true when it is whole. BEGUN, it is of the walls as they stand and the
   *  opening where it is, and it is finished as begun whatever changes after: walls that change every few frames, or an opening found again a little
   *  way off each time, keep no light waiting. Whole, it is the record's, and an OLD one (maskOld: it serves, and is decided again) if the walls
   *  changed or the opening moved while it was made. DECIDED AGAIN ONLY BEHIND WHAT CHANGED, the lamps' own rule (render/reach.ts ReachOnly: a cell
   *  whose line to the opening passes no rect the walls changed in keeps its level; the same mask as one decided whole), where the old mask was
   *  decided at this very place. Until 11-05 every change of the walls anywhere had every opening's mask decided whole again: on a real box 63 to
   *  70 ms of sight lines after each charge, the larger part of all mask work. */
  private openMaskSome(r: OpenRec, walls: OpenWalls, steps: number): boolean {
    let job = r.job;
    if (!job) { job = r.job = { budget: steps, own: walls, at: r.o, only: r.mask && r.owed !== 'all' && sameSpot(r.at, r.o) ? { prev: r.mask, rects: r.owed } : undefined }; r.owed = []; }
    job.budget = steps;
    const got = reachMaskSome(job.own.solid, job.at.mid, job.at.dir, job.at.half[0], null, job.only, job);   // (the spread of the light's own points along the wall)
    if (!got) return false;
    r.mask = got; r.at = job.at; r.job = null; return true;
  }
  /** A DRAWN FRAME'S MASK WORK AND THE OPENING LIGHTS. On the count (no clock in the frame's pace: a stand with none to keep, a check's;
   *  `?reachpace=old`) the keeper's step and then the openings, each with its own share of sight-line steps, as they always had. On the clock the
   *  openings are found, the keeper's ONE step works the lamps' masks and the openings' owed ones as its guests in one allowance of time
   *  (render/reachKeep.ts), and then the lights are written: a mask is used on the frame it is whole. */
  private lightMasks(plan: Pace): void {
    if (plan.ms === undefined || REACH_PACE.whole) { this.reach.tick(); this.lookOpeningLights(plan); return; }
    // (a finding that throws ends the frame before the keeper's step: the next frame has it, since a finding that threw is not begun again)
    const day = this.dayOpenings(plan, true);
    this.reach.tickTimed(8, day?.guests);
    day?.light();
  }
  /** the opening lights of a frame on the count: found, their masks given a share of steps each, written */
  private lookOpeningLights(pace: Pace = WHOLE): void { this.dayOpenings(pace, false)?.light(); }
  /** THE OPENINGS OF A FRAME: found again where the walls have changed, and what each is owed noted. On the count their masks are worked here; on
   *  the clock (`clocked`) the masks owed are returned as the keeper's `guests`. `light()` then writes the lights' records. Null where there is
   *  nothing to light (off the look's page, the row down, a sky that gives nothing). */
  private dayOpenings(pace: Pace, clocked: boolean): { guests: KeepGuest[] | undefined; light: () => void } | null {
    const L = this.lights, W = this.world, K = LOOK_OPENINGS, recs = this.openRecs, day = this.dayLights;
    const none = () => { for (const r of recs) r.l.slot = -1; recs.length = 0; day.length = 0; this.openLevel = null; this.openWalls = null; this.openBegun = false; this.openJob.drop(); };
    if (!(lookConfig.windows > 0)) { if (recs.length || day.length || this.openJob.busy) none(); return null; }
    const level = W.statics[0] ?? null;
    if (level !== this.openLevel) { none(); this.openLevel = level; this.openSerial = -1; this.openSeen = this.openPlanned = this.openLeaves = NaN; this.openWait = 0; }   // (a new level: its statics are new boxes)
    const sun = L.lights.find(l => l.kind === 2 && l.enabled && !l.transient && l.intensity > 0);
    // DAYLIGHT COUNTED ONCE (render/lights.ts LIGHT_PORTAL): each opening is a PORTAL, what is seen through it at every hour (the sky itself, and the
    // ground outside at the engine's guess); the bounce's rays that leave by it take no sky WITHIN ITS REACH and the rest of it as the light fades out toward its range (common.wgsl skyKeep; openingReach below: since 11-05 a reach is 3 to 10 m of most rooms, so the far part of a deep room has its sky by the bounce). The only way since 11-04: until
    // then, behind a row, each was a disk as bright as one guess at all it looks out on, by day only, beside the bounce's own share of the same sky.
    const isOpen = (o: Opening) => o.ajar > 0;
    const rad = openingLooks(this.settings.skyZenith, this.settings.skyHorizon, lookConfig.sky, sun), m = Math.max(rad[0], rad[1], rad[2]);
    if (!(m > 1e-6)) { for (const r of recs) r.l.enabled = false; this.openJob.drop(); return null; }   // (a sky that gives nothing; a finding in hand is given up, and the openings are found again when the sky is back)
    // the walls: looked at when the world says a box has changed, and every eighth frame besides (not every change moves the world's serial); found again
    // once they have changed and then stood still (at once on a level's first frame), A LITTLE A FRAME (render/planPace.ts: found whole on the frame
    // the walls had stood still, the plan, the raster and its scan were 9 to 15 ms of that frame in Chromium on the office, cold each time). The
    // finding takes its own copy of the boxes in the call that begins it and is finished whatever moves after, a step at least every frame: what it
    // lands is the plan of the level as it stood then (a level whose boxes change every few frames still gets its openings found). `openPlanned` is
    // then the key the boxes had when it began, as it always was the key last seen: if they have moved since, they are found again once they stand
    // still. What was planned last serves until each lands. With no pace given, this is the schedule it always had, to the frame.
    const serial = W.geomSerial;
    if (serial !== this.openSerial || (this.openTick++ & 7) === 0) { this.openSerial = serial;
      const key = openingsKey(W.statics); if (key !== this.openSeen) { this.openWait = Number.isNaN(this.openPlanned) ? 0 : K.settle; this.openSeen = key; } }
    const allow = Number.isNaN(this.openPlanned) ? WHOLE : pace;   // (a level's first plan: whole, on its first frame)
    if (this.openPlanned !== this.openSeen && this.openWait-- <= 0 && !this.openJob.busy) { this.openJobKey = this.openSeen; this.openJob.begin(this.openReplan()); }
    let found: ReturnType<typeof this.openJob.go>;
    try { found = this.openJob.go(allow); } catch (e) { this.openPlanned = this.openJobKey; throw e; }   // (a finding that throws is not begun again for the same boxes: the plan in use serves, as it did when the key was kept before the work)
    if (found) {
      this.openPlanned = this.openJobKey; this.openLeaves = NaN;
      const plan = found.plan, was = recs.slice(), next: typeof recs = [], used = new Set<unknown>();
      const at = (a: Opening, b: Opening) => a.dir[0] * b.dir[0] + a.dir[2] * b.dir[2] > 0.9 ? Math.hypot(a.pos[0] - b.pos[0], a.pos[1] - b.pos[1], a.pos[2] - b.pos[2]) : Infinity;
      // one that is where it was, or a little way off, keeps its light, its mask and the job in hand on it (a replan that moved no wall costs it nothing:
      // until 11-05 every replan dropped every job, and a crate shot to bits every few frames could keep a mask from ever being whole); a mask decided
      // anywhere but exactly here is an OLD mask (maskOld). ONE THAT IS NEW WHERE ANOTHER STOOD (a second hole blown beside the first makes one opening of the
      // two, its middle somewhere else; a hole takes in a window) takes the mask of the nearest it lies over, as its old mask, and that one's light
      // and its job in hand if they are free: what was lit stays lit until its own mask is whole, and a mask half made for a hole that has since
      // grown is finished and serves; its own is owed whole. Only an opening where none was has no mask, and no light until it has one.
      for (const pass of [0.05, K.same]) for (const o of plan) { if (used.has(o)) continue; const r = was.find(w => !used.has(w) && at(w.o, o) < pass); if (r) { used.add(o); used.add(r); next.push({ ...r, o }); } }
      for (const o of plan) { if (used.has(o)) continue;
        let over: OpenRec | null = null, od = Infinity;   // (the nearest with a mask; with none, the nearest with one in the making)
        for (const w of was) { if (!w.mask && !w.job) continue; const d = at(w.o, o), s = d + (w.mask ? 0 : 1e6); if (d < w.o.half[0] + o.half[0] && s < od) { od = s; over = w; } }
        const mine = over !== null && !used.has(over); if (mine) used.add(over!);
        next.push({ o, l: mine ? over!.l : { name: 'opening', group: 'sky', kind: 3, pos: [0, 0, 0], dir: [1, 0, 0], color: [1, 1, 1], intensity: 0, range: 0, innerDeg: 0, outerDeg: 90, radius: 0.1, volumetric: 0, owner: 0, enabled: false, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: 0, broad: true, portal: { halfWide: 0, share: 1 } }, mask: over ? over.mask : null, at: over ? over.at : null, job: mine ? over!.job : null, owed: over ? 'all' : [] }); }   // (another opening's mask: owed all of it)
      for (const r of was) if (!used.has(r)) r.l.slot = -1;
      recs.length = 0; for (const r of next) recs.push(r);
      // the walls as the reach masks read them: only where THEY have changed is any mask decided again (a pane shot out, a crate gone: none is), the old
      // one serving until the new is whole. Each mask, and each in the making, is owed the rect that changed (a job in hand is finished against the
      // walls it began with, and its mask is then an old one), or all of it where there is no telling or the rects have piled up
      if (found.changed) { this.openWalls = found.walls;
        const c = found.rect;
        for (const r of recs) { if ((!r.mask && !r.job) || r.owed === 'all') continue;
          if (!c) r.owed = 'all'; else if (r.owed.some(q => q[0] <= c[0] && q[1] <= c[1] && q[2] >= c[2] && q[3] >= c[3])) continue;   // (a rect inside one already owed adds nothing: a wall chewed in one place)
          else if (r.owed.length >= OWED_MOST) r.owed = 'all'; else r.owed.push(c); } }
    }
    const leaves = leavesKey(W.dynamics); if (leaves !== this.openLeaves) { this.openLeaves = leaves; openAreas(recs.map(r => r.o), W.dynamics); }   // (a leaf swung)
    // more open than can be lit at once: the nearest to Sam are (as the pane list is the nearest glass)
    const e = lookConfig.eyeAt?.() ?? null, fx = e ? e[0] : this.viewFocus[0], fz = e ? e[1] : this.viewFocus[2], masks = lookConfig.rooms > 0;
    let cut = Infinity;
    if (recs.length > K.lit) { const far: number[] = []; for (const r of recs) if (isOpen(r.o)) far.push(Math.hypot(r.o.pos[0] - fx, r.o.pos[2] - fz)); if (far.length > K.lit) cut = far.sort((a, b) => a - b)[K.lit - 1]; }
    // the reach masks: a frame's share of sight-line steps (render/reach.ts REACH_PACE, the pace the keeper decides a lamp's at). WHOLE AT ONCE only on the masks'
    // first frame (a level's first frame, the rooms row put on), and then EVERY opening's, shut or far as it may be: no room is shown dark, nobody is
    // playing that frame, and a door swung or a step taken on the next finds its mask made. IN PLAY NOTHING IS decided whole: an opening where none was
    // (a hole blown in a wall) is first in line for the frame's share and its light comes on a few frames later, with its mask whole. Until 11-05 it was
    // decided whole on the frame the blast's walls were read: tens of milliseconds, with the jump field made again beside it. `?reachpace=old`
    // (REACH_PACE.whole, the game's own word for the pace as it was) decides such a one whole at once here too.
    if (!masks) this.openBegun = false;
    let guests: KeepGuest[] | undefined;
    if (masks && this.openWalls) { const c0 = REACH_WORK.steps, walls = this.openWalls, first = !this.openBegun; this.openBegun = true;
      // one begun is finished first; then the open before the shut, one with no mask at all before one whose old mask still serves, the near before the far
      const rank = (r: OpenRec, d: number): number => d + (isOpen(r.o) ? 0 : 1000) + (r.mask ? 500 : 0) - (r.job ? 2000 : 0);
      if (first || !clocked) for (;;) {
        let best: OpenRec | null = null, score = Infinity, now = false;
        for (const r of recs) { if (!openMaskOwed(r)) continue; const d = Math.hypot(r.o.pos[0] - fx, r.o.pos[2] - fz), urgent = !r.mask && (first || (REACH_PACE.whole && isOpen(r.o) && d <= cut));
          const s = rank(r, d) + (urgent ? -5000 : 0); if (s < score) { score = s; best = r; now = urgent; } }
        if (!best) break;
        const left = now ? Infinity : REACH_PACE.steps - (REACH_WORK.steps - c0); if (left <= 0) break;
        if (!this.openMaskSome(best, walls, left)) break; }
      else {
        // ON THE CLOCK (the frame's pace carries one, and this is not the masks' first frame): THE MASKS ARE THE KEEPER'S GUESTS (render/reachKeep.ts
        // KeepGuest), worked in its one allowance of time with the lamps', in the same order. A guest is `first`, in the class that leads most
        // often, when its light is OUT for want of the mask and would be lit with it (open, among the lit few, under a sky the picture shows): a
        // hole just blown.
        const shows = m * this.settings.exposure > K.seen;
        for (const r of recs) { if (!openMaskOwed(r)) continue; const d = Math.hypot(r.o.pos[0] - fx, r.o.pos[2] - fz);
          (guests ??= []).push({ rank: rank(r, d), first: !r.mask && isOpen(r.o) && d <= cut && shows, some: steps => this.openMaskSome(r, walls, steps) }); } } }
    return { guests, light: () => {
      day.length = 0;
      for (const r of recs) {
        // A portal's record is as bright as what is open of its rectangle now, which is all that a pass whose ray stops short of it can know; beside it the
        // record says what share of the rectangle that is, and the picture's direct light takes the whole and lets its own rays find what of it is wall
        // or leaf (common.wgsl LIGHT_PORTAL, "its record")
        const o = r.o, l = r.l, whole = 4 * o.half[0] * o.half[1], open = Math.min(Math.max(o.ajar, 1e-6 * whole), whole), I = m * open;   // (radiance × area = its strength straight out)
        const at = o.mid;   // (a portal is its rectangle, about the rectangle's middle)
        l.pos[0] = at[0]; l.pos[1] = at[1]; l.pos[2] = at[2]; l.dir[0] = o.dir[0]; l.dir[1] = o.dir[1]; l.dir[2] = o.dir[2]; l.radius = Math.hypot(o.half[0], o.half[1]);
        l.range = openingReach(m * this.settings.exposure, Math.max(whole, o.of), l.radius);   // (as far as its own light is worth a ray, wide open: a swinging leaf does not move it)
        l.color[0] = rad[0] / m; l.color[1] = rad[1] / m; l.color[2] = rad[2] / m; l.intensity = I; l.peakIntensity = I; this.reach.give(l, masks ? r.mask ?? undefined : undefined);
        l.portal!.halfWide = o.half[0]; l.portal!.share = open / whole; l.portal!.deep = this.settings.portalHole ? o.deep : 0; l.portal!.rims = o.rims;   // (&holedepth=0: no depth, the whole rectangle and every ray for itself, as it was)
        // (lit from the first hand's width a door opens, where its own rays let next to nothing through, and while the picture would show anything of what it looks out on)
        l.enabled = o.ajar > 0 && m * this.settings.exposure > K.seen && Math.hypot(o.pos[0] - fx, o.pos[2] - fz) <= cut && (!masks || !!r.mask);
        day.push(l);
      }
    } };
  }
  private powderLive(): PowderField | null {
    const air = this.air;
    if (!((lookConfig.veil > 0 || lookConfig.puffs > 0) && air?.cells && this.settings.airSim && lookConfig.snow?.())) {
      if (this.powder) { this.passes.setPowder(null); this.powder.destroy(); this.powder = null; this.powderAir = null; }
      return null;
    }
    if (this.powderAir !== air) { this.passes.setPowder(null); this.powder?.destroy(); this.powder = new PowderField(this.device, { w: air.w, h: air.h, view: air.view, cells: air.cells }); this.powderAir = air; }
    return this.powder;
  }
  /** Drop the TAAU history: the next resolve takes this frame's picture outright (a camera cut nobody could reproject across). The engine calls it itself on a jump of the eye
   *  (> 2.5 m or > 30° between two renders), when TAAU or a debug view switches, and on the first frame; game code may call it for cuts it knows about (harmless twice). */
  resetTaa(reason = 'by hand') { this.passes.resetTaa(); this.taaResets++; this.lastTaaReset = reason; }
  /** radical-inverse (Halton) of i in base b */
  private static halton(i: number, b: number) { let f = 1, r = 0; while (i > 0) { f /= b; r += f * (i % b); i = Math.floor(i / b); } return r; }
  skin: SkinnedDrawable | null = null;
  /** mesh-shadow trees for the skinned characters (attached by whoever owns the CharacterRenderer: the page); bound at scene binding 13 */
  meshShadows: MeshShadows | null = null;
  /** the decal list (experiment E1, render/decals.ts — owned by the Game, which feeds it; null in the viewer): synced + uploaded each frame before the g-buffer, which
   *  reads it at its group 1 binding 3 under FrameFlags.Decals */
  decals: Decals | null = null;
  /** the GPU particle system (render/particles.ts; attached by the page): stepped after the cascades, drawn into the HDR target after the composite while settings.gpuParticles is on */
  particles: Particles | null = null;
  /** measurement only: timestamp the particles' render pass as 'pdraw' (render-pass timings are unreliable on Metal and feed every GPU total — leave off) */
  particlesTimeDraw = false;
  /** false when this adapter's storage-buffer budget left no slot for the mesh binding (frame.ts meshBinding): settings.meshShadows is then inert */
  readonly meshShadowsAvailable: boolean;
  cpuMs = { boxes: 0, encode: 0 };
  /** extra GPU work encoded after lighting (irradiance probes etc.): (enc, sceneBG, diceView, ts) */
  afterSubmit: (() => void)[] = [];

  constructor(public gpu: Gpu, rcCfg: RcConfig = defaultRcConfig, world?: (device: GPUDevice) => BoxWorld) {
    const device = this.device = gpu.device;
    const maxStorage = device.limits.maxStorageBuffersPerShaderStage;
    if (maxStorage < 9) throw new Error(`This GPU / browser exposes ${maxStorage} storage buffers per shader stage; the renderer needs 9 (scene group 7 + the cascade trace's / light probe's 2; the smoke's link 9).`);   // (readable on the start card, not a validation error)
    // the mesh-shadow trees want one more read-only storage slot in the scene group: 7 + mesh 1, plus the cascade trace's / probe's 2 in
    // their own group, must fit the per-stage limit — 10, which is exactly what Chrome offers on Metal. Where it does not fit the
    // binding is left out of the layout altogether (a stub meshLoad() in the prelude) and the experiment is unavailable rather than costing the reach rows their
    // measured storage path. Decided once, here, before any scene pipeline is compiled.
    meshBinding.enabled = 7 + 1 + 2 <= maxStorage;
    this.meshShadowsAvailable = meshBinding.enabled;
    this.timer = new GpuTimer(device, gpu.hasTimestamps);
    this.sceneLayout = createSceneLayout(device);
    this.fogSight = new FogSight(device, this.sceneLayout);
    this.fogSightStand = device.createBuffer({ label: 'fogSightStand', size: FOG_SIGHT_BYTES, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST });   // (zeros: 'no table written' — but for the list of thin statics, which the sight table's own pass reads here: render/fogofwar.ts FogSight.stand)
    this.fogSight.stand = this.fogSightStand;
    this.world = world?.(device) ?? new BoxWorld(device); this.fogSight.world = this.world;
    this.lights = new LightSet(device);
    this.reach = new ReachKeep(this.world, this.lights);
    this.frameBuf = makeBuffer(device, FRAME_BYTES, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'frame');
    this.frameBufOut = makeBuffer(device, FRAME_BYTES, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'frameOut');
    this.dummyStorage = makeBuffer(device, 256, GPUBufferUsage.STORAGE, 'dummy');
    // smoke placeholder until the sim is attached
    const dummyTex = device.createTexture({ size: [1, 1, 1], dimension: '3d', format: 'rgba16float', usage: GPUTextureUsage.TEXTURE_BINDING });
    const smokeUbo = makeBuffer(device, 64 * 8, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'smokeBoundsDummy');
    const dummyPage = device.createTexture({ size: [1, 1, 1], dimension: '3d', format: 'r32uint', usage: GPUTextureUsage.TEXTURE_BINDING });
    this.smoke = { uniform: smokeUbo, atlasView: dummyTex.createView(), pageView: dummyPage.createView(), count: 0 };
    this.rc = new RadianceCascades(device, this.sceneLayout, { ...rcCfg }, true);   // (deferred: its kernels are init()'s, like every other boot pipeline)
    this.pool = new TexturePool(device);
    this.gbuffer = new GBuffer(device, this.sceneLayout, this.pool, this.world.maxBoxes);   // (its previous-transform buffer is sized to the world's box capacity)
    this.passes = new LightingPasses(device, this.sceneLayout, gpu.format, gpu.linearSampler, this.pool);
    this.buildPlace();   // (THE LOOK PROTOTYPE: its two small programs are asked for with the boot's own, so the page's first frame has a live field. The page's rows are not read yet here, so a page with the shade row at 0 on its address compiles them unused. Until it is live no place takes a gain, composite.wgsl lookPlaceLevels, and a page that got it late would step)
  }
  /** THE LOOK PROTOTYPE's place field, made once (render/placeField.ts); a device that cannot make it is asked once and no more */
  /** NOTHING IS COMPILED AT ITS FIRST USE. The fog of war's sight table and its pass made their programs the first frame a mission drew the fog: the title
   *  never built them, so the first squad mission of a visit compiled three programs under play. The page asks for them
   *  here, once its boot's own are in (host/drawpage.ts, after the boot's freeze): they compile behind the start, counted with the later set while they do (a brief's
   *  Start waits for them), and on the kept page (gpu/keep.ts) a mission finds them made. A pass that gains a program of its own asks for it here too. */
  warmFirstUse(): void { this.fogSight.warm(); this.passes.warmFog(); powderWarm(this.device); }   // (the look's loose snow steps only on a snow level: asked for on every level, so that none compiles it in play)
  private buildPlace(): void { if (this.place || this.placeDead) return; try { this.place = new PlaceField(this.device, this.rc.diceView); } catch (err) { console.warn('place field: not built on this device', err); this.placeDead = true; } }

  /** true once init() has resolved: every boot pipeline exists and render() may run */
  ready = false;
  /** THE BOOT'S SECOND HALF. The constructor is synchronous and cheap (layouts, buffers, shader-module handles); this creates every pipeline the renderer boots with — the cascades',
   *  the g-buffer's, the lighting passes' (+ the v3 denoiser, the target-ops kernels) — through createComputePipelineAsync / createRenderPipelineAsync (gpu/device.ts has
   *  the why: the synchronous forms froze the whole browser on a cold shader cache), all in flight at once, and resolves when the last is ready. `opts.sharp` / `opts.gbaa`: also
   *  prepare the sharp-direct trio + its g-buffer down pass / the GBAA kernel, which are otherwise compiled synchronously on the first frame a preset switches them on. Await it
   *  (or use Engine.create) before the first render(); gpu/device.ts pipelineProgress counts the pipelines in for a progress line. */
  init(opts: { sharp?: boolean; gbaa?: boolean; onGroup?: (name: string) => void } = {}): Promise<void> {
    return this.initP ??= Promise.all([this.rc.init().then(() => opts.onGroup?.('cascades')), this.gbuffer.init({ sharp: opts.sharp }).then(() => opts.onGroup?.('gbuffer')), this.passes.init(opts), this.gbuffer.fillLook().then(() => { opts.onGroup?.('materials'); }), this.place ? this.place.whenReady.catch(() => { }) : null])   // (fillLook: the look's material maps, made a layer at a time while the pipelines above compile)   // (onGroup: the boot timeline's per-group marks — passes reports froxels / denoiser / targetOps / passes itself)
      .then(() => { this.ready = true; });   // (memoised: a second caller awaits the same build)
  }
  private initP: Promise<void> | null = null;
  /** SOME PASS IS DRAWING WITH A STAND-IN WHILE ITS OWN PROGRAM COMPILES (gpu/device.ts laterProgress): a pipeline the boot left for later is not in yet,
   *  so this frame's picture and its GPU times are the stand-ins', not the settings' own. False again once every one asked for has landed or failed (a failed one keeps its
   *  stand-in for good: nothing more will change). Always false with the first-frame start off. */
  get standsIn(): boolean { return laterProgress.pending > 0; }
  /** construct + init in one await (the viewer, the editor, tools) */
  static async create(gpu: Gpu, opts: { sharp?: boolean; gbaa?: boolean; rcCfg?: RcConfig; onGroup?: (name: string) => void } = {}): Promise<Engine> {
    const e = new Engine(gpu, opts.rcCfg ?? defaultRcConfig);
    await e.init(opts);
    return e;
  }

  setSmoke(s: SmokeBinding) { this.smoke = s; this.sceneBG = null; this.sceneBGOut = null; }
  /** Attach the mesh-shadow trees for the characters `skin` draws (or null to detach); the scene group rebinds on the next frame. */
  setMeshShadows(m: MeshShadows | null) { this.meshShadows = m; this.sceneBG = null; this.sceneBGOut = null; }
  /** the SDF character rig's radius fit for the mannequin `skin` draws (render/sdfrig.ts fitRig — the page attach it beside the mesh shadows). BoxWorld.upload packs it into every frame's proxy
   *  list (fitted r0 / r1 per slot, the cull sphere fitted); null = the list as the game wrote it: hand radii, no taper byte (each record traced as an untapered cone), 1.5 m spheres */
  rigFit: RigFit | null = null;
  setRigFit(f: RigFit | null) { this.rigFit = f; }
  /** Attach the decal list the g-buffer reads (or null); the g-buffer's object group rebinds by itself (keyed on the buffer). */
  setDecals(d: Decals | null) { this.decals = d; }
  /** Is FrameFlags.Decals up this frame? (the setting, a list attached, and at least one mark in it — an empty list costs the fragment stage nothing) */
  get decalsOn(): boolean { return this.settings.decals && !!this.decals && this.decals.count > 0; }
  /** Attach the GPU particle system (or null): its passes join the frame from the next render() while settings.gpuParticles is on. */
  setParticles(p: Particles | null) { this.particles = p; }
  /** THE AIR OVER THE FLOOR (render/airField.ts; src/picture/air.ts feeds it): the wind as a small fluid simulation over the level's floor plan — walls, furniture
   *  and door leaves stand in it, the storm and the vents drive it. Read by the particle step (the blown snow's grains and wisps) and by nothing else: a picture
   *  only. host/drawpage.ts attaches one on every level (`?air=0`: none — null = the engine as it was); the game hands over, once a frame, the ambient wind (`airWind`,
   *  m/s along x and z, the gust on it) and whether to run at all (`airOn`: src/picture/air.ts AIR_SIM.on, live — a perf sweep can flip it mid-take). STEPPED ON
   *  ITS OWN CLOCK, not once a drawn frame (settings.airSim, live; settings.airEvery): when at least 0.8 of `airEvery` sixtieths of a second have gone by since the last step (a 120 Hz page steps every
   *  other frame, a 30 Hz one once a frame by the solver's own 1/30 s bound), in ONE compute pass timed as 'air' (per-cell work: it does not shrink with the
   *  render scale). */
  air: { step(enc: GPUCommandEncoder, dt: number, time: number, wind: readonly [number, number], ts?: GPUComputePassTimestampWrites): void; readonly view: GPUTextureView; readonly ready?: boolean; setCells(mask: Uint8Array, rect?: readonly [number, number, number, number]): void; setSources?(recs: Float32Array, n: number): void; reset(): void; readonly w: number; readonly h: number; readonly cells?: GPUTextureView } | null = null;
  airWind: [number, number] = [0, 0];
  airOn = false;
  private airAcc = 0;
  /** Is the GPU particle path live this frame? (the setting ∧ a Particles attached) — what rust/game_rules/src/breakage/particle_fx.rs routes the effect sites by */
  get particlesOn(): boolean { return this.settings.gpuParticles && !!this.particles; }

  /** A new cascade grid (a preset with another probe density, the Tab panel's 'rebuild cache'). WORLD-SPACE cache: what a (re)started instance does about its empty / stale contents
   *  (amortized upkeep, holding the old dice while the new one fills…) is render/rc.ts's business (renderer v3 lane P0-A); this side only decides WHICH instance is live and rebuilds
   *  the bind groups that name its dice view. The SCREEN histories are deliberately left alone: the indirect's history converges on the refilled cache through its own clip + EMA.
   *  Instances are PARKED by grid, not destroyed (rcPark): the auto ladder's medium ↔ low step crosses probe grids, and building an instance costs 10–40 ms of pipeline / buffer
   *  creation on the reference machine (measured 08-11) — a parked one is swapped back in for nothing, still holding the light field it had (minutes stale at worst: closer to right
   *  than the black a fresh one starts from; the dice EMA refreshes 90 % of it in seven frames). ~1.5 MB per parked grid. A rebuild request for the grid already live (the Tab
   *  button) still starts that grid over from nothing, as it says. */
  rebuildRc(cfg: RcConfig) {
    const key = rcGridKey(cfg), cur = rcGridKey(this.rc.cfg);
    if (key === cur) { this.rc.destroy(); this.rc = new RadianceCascades(this.device, this.sceneLayout, { ...cfg }); this.rc.invalidateWorld(); }   // (re-primed: whole-level bakes first + a history-free first dice bake — P0-A)
    else {
      const old = this.rc; this.rcPark.set(cur, old);
      const parked = this.rcPark.get(key);
      if (parked) { this.rcPark.delete(key); parked.setInterval0(cfg.interval0); this.rc = parked; }
      else { this.rc = new RadianceCascades(this.device, this.sceneLayout, { ...cfg }); this.rc.invalidateWorld(); }
      this.rc.staggerUpper = old.staggerUpper;   // (halfRate / diceBlend are written every frame; the stagger switch is a by-hand Tab row — it follows the renderer, not the instance)
      while (this.rcPark.size > 2) { const [k, v] = this.rcPark.entries().next().value as [string, RadianceCascades]; v.destroy(); this.rcPark.delete(k); }
    }
    this.passes.externalViewsChanged(); this.sceneBG = null;
  }
  /** parked cascade instances by grid (rebuildRc) */
  private rcPark = new Map<string, RadianceCascades>();
  /** Build (and park) the instances for these grids now — at boot, behind the start card, where pipeline creation is free — so the ladder's first step onto any of them later
   *  swaps instead of builds (ui/quality.ts hands over the ladder's grids). No-op for the live grid or one already parked. */
  prewarmRc(cfgs: RcConfig[]) {
    // A prewarmed instance has never baked: its dice is zeros, which is no history — re-primed like rebuildRc's fresh build (whole-level bakes + a history-free first
    // dice bake once it goes live), else its first ~10 live frames blend the real field in at 0.3 a frame from black and the bounce, the smoke's ambient AND the light
    // meter dip on the ladder's first hop onto it (review R10.1). The flags just sit on the parked instance until it is swapped in.
    for (const cfg of cfgs) {
      const key = rcGridKey(cfg); if (key === rcGridKey(this.rc.cfg) || this.rcPark.has(key)) continue;
      const inst = new RadianceCascades(this.device, this.sceneLayout, { ...cfg }); inst.invalidateWorld(); this.rcPark.set(key, inst);
    }
  }
  /** Something changed the WORLD wholesale (level geometry reloaded, an hour laid over it): re-prime the radiance cache in place (rc.invalidateWorld —
   *  P0-A) AND every parked grid, which would otherwise come back on a later hop holding the field from before the change (a night grid parked through a day, a
   *  day grid swapped in at night: seven frames of the wrong bounce and the wrong light-meter reading — review R10.1). Not for screen events — resize / scale / checkerboard /
   *  divisor flips leave the world cache valid and must not call this. */
  invalidateWorldCache() { this.rc.invalidateWorld(); for (const parked of this.rcPark.values()) parked.invalidateWorld(); }
  /** The LEVEL was put back to its start under the picture (rust/game_rules/src/tick/savestate: a restart — lamps that were out burn, holed walls stand, the eye is somewhere else): the
   *  screen histories, the reservoirs, the haze volume's history and the radiance cache all still hold the level as it stood, 'wrong wholesale'. Each starts from
   *  nothing, as on a boot's first frame, so the fresh level is not blended in over the frames after. */
  restartPicture(why = 'level restarted') { this.passes.restartHistories(why); this.invalidateWorldCache(); this.startPlansOver(); }   // (restartHistories clears the resolve's history and the reservoirs with the rest)

  /** the scene group's resources over one frame block (binding 0): the one list ensureSceneBG and its output-size twin both build from */
  private sceneGroup(frame: GPUBuffer, fogSight: GPUBuffer = this.fogSight.buf): GPUBindGroup {
    return createSceneBindGroup(this.device, this.sceneLayout, {
      frame, boxGeo: this.world.geoBuf, boxMat: this.world.matBuf, gridCells: this.world.cellBuf, gridItems: this.world.itemBuf,
      sceneInfo: this.world.infoBuf, lights: this.lights.buf, smoke: this.smoke.uniform, smokeAtlasView: this.smoke.atlasView, smokePage: this.smoke.pageView, lightReach: this.lights.reachBuf, linSamp: this.gpu.linearSampler, capsules: this.world.capsuleBuf,
      mesh: this.meshShadows?.buf ?? this.dummyStorage, fogSight,
    });
  }
  private ensureSceneBG() {
    const meshBuf = this.meshShadows?.buf ?? this.dummyStorage;   // (256 zero bytes = a zero header = an empty tree, and the flag is never raised without a MeshShadows anyway)
    if (this.sceneBG && this.sceneBGItems === this.world.itemBuf && this.sceneBGMesh === meshBuf) return;
    this.sceneBG = this.sceneGroup(this.frameBuf);
    this.sceneBGPlain = this.sceneGroup(this.frameBuf, this.fogSightStand);
    this.sceneBGItems = this.world.itemBuf; this.sceneBGMesh = meshBuf;
  }
  private sceneBGItems: GPUBuffer | null = null; private sceneBGMesh: GPUBuffer | null = null;
  /** the OUTPUT frame block's twin of the scene group (sharp direct): every resource the same but binding 0 = frameBufOut; rebuilt with the same keys */
  private ensureSceneBGOut() {
    const meshBuf = this.meshShadows?.buf ?? this.dummyStorage;
    if (this.sceneBGOut && this.sceneBGOutItems === this.world.itemBuf && this.sceneBGOutMesh === meshBuf && this.sceneBGOutSmoke === this.smoke.uniform) return;
    this.sceneBGOut = this.sceneGroup(this.frameBufOut);
    this.sceneBGOutItems = this.world.itemBuf; this.sceneBGOutMesh = meshBuf; this.sceneBGOutSmoke = this.smoke.uniform;
  }
  private sceneBGOutItems: GPUBuffer | null = null; private sceneBGOutMesh: GPUBuffer | null = null; private sceneBGOutSmoke: GPUBuffer | null = null;
  /** Is the mesh-shadow path live this frame? (the setting, the adapter budget and an attached MeshShadows all three) */
  get meshShadowsOn(): boolean { return this.settings.meshShadows && this.meshShadowsAvailable && !!this.meshShadows; }

  resize() {
    const canvas = this.gpu.canvas, view = this.view ?? canvas;
    const dpr = window.devicePixelRatio || 1;
    const cw = Math.max(1, Math.floor(view.clientWidth * dpr)), ch = Math.max(1, Math.floor(view.clientHeight * dpr));
    // canvas backing store at internal res; browser upscales. Internal height = min(maxInternalHeight, CSS height × dpr) × renderScale: the cap is what ×1.0 means on
    // this window and every step of the scale below it renders fewer pixels (until 08-11 the scale multiplied the window and was THEN capped, so on a HiDPI window —
    // 1440 device px against a 1000-px cap — everything from ×0.70 to ×1.00 rendered the same 1000 rows: a dead zone on the slider and rungs the adaptive ladder
    // announced but never took)
    const capH = Math.min(this.settings.maxInternalHeight, ch);                            // the rows this window gets at the current cap (render scale 1.0)
    const h = Math.max(16, Math.floor(capH * Math.max(0.05, Math.min(1, this.settings.renderScale || 1)) + 1e-6));
    const scale = h / ch, w = Math.max(16, Math.floor(cw * scale + 1e-6));
    this.capScale = capH / ch; this.effectiveScale = scale;   // effectiveScale keeps its meaning — internal / (CSS × dpr), what was actually rendered — the cost model's pixel term and rust/game_rules/src/tick/game's metres-per-pixel are in those units
    // OUTPUT size: with TAAU the cap's rows whatever the scale (exactly what scale 1.0 renders — so the resolve's cost is bounded by the preset, and 'TAAU at 0.77' presents the same canvas as 'no TAAU at
    // 1.0'); without it the internal size (the canvas the browser upscales, as before)
    const oh = this.settings.taau ? Math.max(16, Math.floor(capH + 1e-6)) : h, ow = this.settings.taau ? Math.max(16, Math.floor(cw * (oh / ch) + 1e-6)) : w;
    if (canvas.width !== ow || canvas.height !== oh) { canvas.width = ow; canvas.height = oh; }
    // SHARP DIRECT: live only where there is something to decouple — TAAU on (the output-size history), the internal size below the output, and no debug view (those bypass the resolve and read
    // internal-size diagnostics); mirrored into the passes BEFORE their resize, like the TAAU switch (the composite-size set follows it)
    const sharpAsked = !!this.settings.sharpDirect && this.settings.taau && this.settings.debugView === 0 && (ow !== w || oh !== h);
    // START ON FIRST FRAME SHADERS (gpu/device.ts PIPE_QUEUE.later): … or while no fragment composite is in yet — the pair is a LATER twin when the boot built the sharp
    // path, passes.ts traceJobs: sharp direct stands in for it then, even where there is nothing to decouple. With the row off `sharp` is `sharpAsked`, the line as it always was.
    const sharp = sharpAsked || (PIPE_QUEUE.later && this.settings.taau && this.settings.debugView === 0 && !this.passes.fragCompReady);
    this.passes.setSharp(sharp);
    this.passes.setGbaa(!!this.settings.gbaa && this.settings.taau && this.settings.debugView === 0 && (sharp || this.settings.taauPhases <= 0));   // GBAA: on the TAAU path only (its blend sits between the composite and the resolve; the debug views and the FXAA chain bypass it), and only on a 1 : 1 raster it can own — sharp direct live, or the jitter off — never under the jittered upscale (that is C4: measured softer than main)
    this.passes.setOutlineCopy(!!this.outline?.on && this.settings.debugView === 0);   // THE OUTLINE BY THE PICTURE (render/outline.ts): a copy of the composite for it to read, kept only while its rows ask
    this.passes.fogFrame = this.fogFrame; this.passes.setFog(!!this.fogFrame?.on && this.settings.debugView === 0);   // FOG OF WAR (rust/game_rules/src/stealth/fogofwar): its pass exists only while the game's switch is on (hdr takes the storage + copy usages then), and never under a debug view
    this.passes.setSceneCopy(!!this.settings.particleRain && !!this.particles);   // PARTICLE RAIN v2: keep a copy of the composite for the rain streaks to refract (GBAA's own when GBAA is on; else one the particles fill before their splat) — off = nothing kept
    if (w !== this.width || h !== this.height || ow !== this.outWidth || oh !== this.outHeight || this.passes.stale || sharp !== this.sharpOn) {   // (stale: TAAU / GBAA switched — the output- or composite-size set is rebuilt even at an unchanged size)
      this.width = w; this.height = h; this.outWidth = ow; this.outHeight = oh; this.sharpOn = sharp;
      this.gbuffer.keepTargets = this.passes.keepHistories = this.settings.keepHistories;
      this.gbuffer.resize(w, h, ow, oh, sharp); this.passes.resize(w, h, ow, oh); this.passes.externalViewsChanged();   // (the passes' bind groups name the g-buffer's views: rebuilt against the new ones)
    }
  }

  /** irradiance (W/m², unshadowed, on-axis) under which a light's tint through a pane is invisible: a 20 % tint of it is under half a per cent of display white */
  static readonly GLASS_MIN_E = 0.02;
  private glassEx = new Float64Array(24); private glassEz = new Float64Array(24); private glassCells = new Int32Array(24 * GLASS_STATIONS.length * GLASS_SIDES.length);
  /** GLASS: per light, the bits of the listed panes (world.glassList order = SceneInfo.glass order; the first 24) that light can possibly shine THROUGH onto
   *  anything: the pane's bounds inside the light's range box (a directional light: every pane), and — for a light with a reach mask (reach.ts: the level's
   *  static fixtures) — some 1 m cell just off either face of the pane inside that mask (a light that reaches neither side of a pane lights nothing through
   *  it). Errs toward 'set' like the reach masks themselves. → RtLight.glassMask, which upload() packs above the owner byte; direct_common.wgsl glassPairMask narrows
   *  it per 8×8 tile. lights × panes cheap tests a frame; nothing with the experiment off or no pane listed. */
  private glassLightMasks() {   // (GLASS_STATIONS / GLASS_SIDES below the class: the sample lattice as module consts, not two array literals per light per pane per frame — review R1.4)
    const w = this.world, list = this.settings.glass ? w.glassList : [];
    const LL = this.lights.lights, DL = this.dayLights;   // (the picture's own lights too: daylight from a window is tinted by the next room's glass like a lamp's)
    if (!list.length) { for (let i = 0; i < LL.length; i++) LL[i].glassMask = 0; for (let i = 0; i < DL.length; i++) DL[i].glassMask = 0; return; }
    const W = WORLD.gridW, H = WORLD.gridH, cs = WORLD.gridCell;
    const cellOf = (x: number, z: number) => { const cx = Math.floor((x - WORLD.min[0]) / cs), cz = Math.floor((z - WORLD.min[2]) / cs); return cx < 0 || cz < 0 || cx >= W || cz >= H ? -1 : cz * W + cx; };
    // per listed pane, once a frame (all of it was worked out again for every light): its world half extents in x / z, and the grid cells just off either face at the three
    // stations along it (−1 = off the grid) — the cells a light's reach mask is asked about
    const EX = this.glassEx, EZ = this.glassEz, CELLS = this.glassCells, NC = GLASS_STATIONS.length * GLASS_SIDES.length;
    for (let k = 0; k < list.length && k < 24; k++) {
      const b = w.statics[list[k]]; if (!b) continue;
      { const c = Math.abs(Math.cos(b.yaw)), sn = Math.abs(Math.sin(b.yaw)); EX[k] = c * b.h[0] + sn * b.h[2]; EZ[k] = sn * b.h[0] + c * b.h[2]; }
      const thinX = b.h[0] < b.h[2];   // thin along local x (a pane in a wall along Z) or along local z
      const c = Math.cos(b.yaw), sn = Math.sin(b.yaw);
      const nx = thinX ? c : sn, nz = thinX ? -sn : c;                 // world direction of the thin axis (local +x → (cos, 0, −sin), +z → (sin, 0, cos))
      const tx = thinX ? sn : c, tz = thinX ? c : -sn;                 // …and of the long one
      const half = thinX ? b.h[2] : b.h[0];
      let j = k * NC;
      for (let ui = 0; ui < GLASS_STATIONS.length; ui++) for (let si = 0; si < GLASS_SIDES.length; si++) { const u = GLASS_STATIONS[ui], side = GLASS_SIDES[si], ci = cellOf(b.c[0] + tx * u * half + nx * side, b.c[2] + tz * u * half + nz * side); CELLS[j++] = ci >= 0 ? ci : -1; }   // (a NaN cell — a corrupt pane — must stay 'off the grid', not become cell 0 in the Int32Array)
    }
    for (let n = 0; n < LL.length + DL.length; n++) {
      const l = n < LL.length ? LL[n] : DL[n - LL.length]; const ghost = (l.fogGhost ?? 0) > 0 && !(l.enabled && l.intensity > 0);   // (a lamp out but lit in the fog's picture — lights.ts RtLight.fogGhost — shines through the panes it did)
      if (!l.enabled && !ghost) { l.glassMask = 0; continue; }
      let m = 0; const lm = this.reach.of(l);
      const peak = Math.max(l.color[0], l.color[1], l.color[2]) * (ghost ? l.fogGhost! : l.intensity);   // on-axis intensity (W/sr-ish): what evalLight divides by d²
      const cosO = Math.cos(Math.min(89, l.outerDeg + 8) * Math.PI / 180);      // a spot's cone, opened 8° (the pane is judged by a handful of points, not its whole face)
      for (let k = 0; k < list.length && k < 24; k++) {
        const b = w.statics[list[k]]; if (!b) continue;
        if (l.kind !== 2) {   // range box vs the pane's world bounds; then: could this light put GLASS_MIN_E on any of the pane at all (distance to its nearest point, the spot's cone / the panel's facing over its corners and centre)? A light too dim or pointed away tints nothing anybody can see
          const ex = EX[k], ez = EZ[k];
          const r = l.range + 0.05;
          if (Math.abs(l.pos[0] - b.c[0]) > r + ex || Math.abs(l.pos[1] - b.c[1]) > r + b.h[1] || Math.abs(l.pos[2] - b.c[2]) > r + ez) continue;
          const qx = Math.max(Math.abs(l.pos[0] - b.c[0]) - ex, 0), qy = Math.max(Math.abs(l.pos[1] - b.c[1]) - b.h[1], 0), qz = Math.max(Math.abs(l.pos[2] - b.c[2]) - ez, 0);
          const d2 = Math.max(qx * qx + qy * qy + qz * qz, l.radius * l.radius, 0.01);
          if (peak / d2 < Engine.GLASS_MIN_E) continue;
          if ((l.kind === 1 || l.kind === 3) && qx * qx + qy * qy + qz * qz > (ex + b.h[1] + ez) ** 2 * 0.5) {   // facing (only judged from further off than ~the pane's own size: a torch a metre from a bay aimed between the sample points must not drop the bit and pop the tint): some corner / the centre of the pane inside the (opened) cone, or in front of the one-sided panel
            let facing = false;
            for (let ci = 0; ci < 9 && !facing; ci++) {
              const px = ci === 8 ? b.c[0] : b.c[0] + (ci & 1 ? ex : -ex), py = ci === 8 ? b.c[1] : b.c[1] + (ci & 2 ? b.h[1] : -b.h[1]), pz = ci === 8 ? b.c[2] : b.c[2] + (ci & 4 ? ez : -ez);
              const dx = px - l.pos[0], dy = py - l.pos[1], dz = pz - l.pos[2]; const dl = Math.hypot(dx, dy, dz) || 1;
              const cd = (dx * l.dir[0] + dy * l.dir[1] + dz * l.dir[2]) / dl;
              facing = l.kind === 3 ? cd > 0.02 : cd > cosO;
            }
            if (!facing) continue;
          }
        }
        if (lm) {   // some cell just off either face, at three stations along the pane, inside the light's reach
          let any = false;
          for (let j = k * NC, e = j + NC; j < e; j++) { const ci = CELLS[j]; if (ci >= 0 && reachLevel(lm, ci) > 0) { any = true; break; } }
          if (!any) continue;
        }
        m |= 1 << k;
      }
      l.glassMask = m >>> 0;
    }
  }
  private offscreenTarget: GPUTexture | null = null;
  /** Read back the last offscreen frame (render with present=false first) as RGBA8 pixels — for automated captures. */
  async readbackFrame(): Promise<{ w: number; h: number; data: Uint8Array; bgra: boolean } | null> {
    const tex = this.offscreenTarget; if (!tex) return null;
    const w = tex.width, h = tex.height; const bpr = Math.ceil(w * 4 / 256) * 256;
    const buf = this.device.createBuffer({ size: bpr * h, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    const enc = this.device.createCommandEncoder(); enc.copyTextureToBuffer({ texture: tex }, { buffer: buf, bytesPerRow: bpr }, [w, h]); this.device.queue.submit([enc.finish()]);
    await buf.mapAsync(GPUMapMode.READ);
    const src = new Uint8Array(buf.getMappedRange()); const data = new Uint8Array(w * h * 4);
    for (let y = 0; y < h; y++) data.set(src.subarray(y * bpr, y * bpr + w * 4), y * w * 4);
    buf.unmap(); buf.destroy();
    return { w, h, data, bgra: this.gpu.format.startsWith('bgra') };
  }
  /** present=false renders into an internal target instead of the swap chain (scripted runs while the tab is hidden, where getCurrentTexture() stalls). */
  render(cam: FollowCamera, time: number, dt: number, present = true) {
    if (!this.ready) throw new Error('Engine.render() before init() resolved — await engine.init() / Engine.create() first');   // (a readable line instead of setPipeline(undefined) deep in a pass)
    this.lastDt = dt; this.viewFocus = cam.focus;
    if (this.outline) { this.outline.exposure = this.settings.exposure; outlineBoxes(this.outline, p => cam.project(p, 1, 1)); }   // THE OUTLINE IN THE DARK (host/findme.ts → render/outline.ts): the box round each man on THIS camera's picture
    this.passes.outline = this.settings.debugView === 0 ? this.outline : null;   // (never under a debug view, as the fog)
    this.passes.smokeDiffuseBilinear = this.settings.smokeDiffuseWeight === 'bilinear'; this.passes.smokeDiffuseTau0 = this.settings.smokeDiffuseStart ?? 0.35;   // (smoke battery experiment, post.wgsl diffuseMode)
    this.gbuffer.keepTargets = this.passes.keepHistories = this.settings.keepHistories;   // (the switch for everything a change of size does — texpool.ts; mirrored before either can happen below)
    if (!this.settings.keepHistories && this.pool.parkedBytes) this.pool.evict(true);    // switched off: nothing stays parked
    else this.pool.tick();                                                                 // parked targets not wanted back within a couple of minutes are let go
    this.passes.setDivisors(this.settings.gatherThird ? 3 : 2, this.settings.volQuarter ? 4 : 2);   // (lossy options; re-creates the targets on the grid that changed)
    this.passes.setTaau(this.settings.taau);   // (mirrored before resize: the output-size target set — the resolve's history pair — exists only while it is on)
    { const S = this.settings, refl = S.beamReflect && S.rtReflect, air = S.beamAir && S.debugView === 0;   // (a debug view shows the boxes: the air look is added in the composite only)
      // how much of a beam the AIR shows (0.4 ≈ the old box's brightness under the tube): a little in clear indoor air, more by the haze, most in rain — smoke is read per pixel in the shader
      const A = air ? S.beamAirK * (0.03 + 0.09 * Math.min(2, S.hazeDensity * lookConfig.air / 0.035)) : 0;   // (0.12 at the default haze: the software adapter's stills put the line at the old box's level, clipped, at 0.4)
      this.passes.setBeams((refl || air) && this.beams ? this.beams.list ?? this.world.dynamics : null, this.beams?.is ?? NO_BEAM, this.viewFocus, { reflect: refl, air: air ? this.beams?.airOf ?? null : null, A, rain: this.weather.rain, wet: S.wetMirror && S.rtReflect ? 2 : 0, coats: Math.max(0, Math.min(1, lookConfig.pressed)) });
    }   // (lasers / tracers in the reflections: ≤ 8 segments in a small uniform; none = a zero count, the pass as it was)
    this.passes.setFireLook(this.fireLook);
    this.passes.setFires(this.settings.debugView === 0 ? this.fires : null);   // the open fires' flames (rust/game_rules/src/air/fire.rs), behind the beams in the same uniform
    this.passes.setRt(this.settings.rtReflect, this.settings.rtReflectDiv);   // (likewise before resize: the reflection targets' divisor — pooled + re-made on change like the gather's — and the switch that first allocates them at all)
    this.resize();
    cam.aspect = this.width / this.height;
    cam.rebuild();
    // ---- TAAU: this frame's jitter and the matrices everything downstream sees. The jitter is a translation of the projection in NDC (content at unjittered pixel s lands at s + jitter); the
    // frame uniform carries the JITTERED view-projection, its inverse and the previous render's jittered one, so the g-buffer, every pass that rebuilds a ray from a pixel (worldFromDepth) and
    // every reprojection into an internal-resolution history agree to the bit; only the resolve (and the HUD, which projects on the CPU with the camera's own matrices) ever sees unjittered ones.
    const taauOn = this.settings.taau, s0 = this.settings;
    if (taauOn !== this.taauWas) { this.taauWas = taauOn; if (taauOn) this.resetTaa('taau on'); }
    if (taauOn && (s0.debugView !== 0) !== (this.debugWas !== 0)) this.resetTaa('debug view');
    this.debugWas = s0.debugView;
    let jx = 0, jy = 0;
    if (taauOn && s0.debugView === 0 && s0.taauPhases > 0) { const n = Math.max(1, Math.floor(s0.taauPhases)); const i = (this.frameIdx % n) + 1; jx = Engine.halton(i, 2) - 0.5; jy = Engine.halton(i, 3) - 0.5; }
    const decouple = this.sharpOn && !!s0.sharpDecouple && (jx !== 0 || jy !== 0);   // DECOUPLED JITTER: the internal raster keeps the full ±½ INTERNAL-px jitter, the output raster none (writeFrameOut / jitterRes below)
    if (this.sharpOn && !decouple) { jx *= this.width / this.outWidth; jy *= this.height / this.outHeight; }   // sharp direct: the raster to antialias is the OUTPUT one — ± ½ output px, expressed in internal px as everything downstream expects (the same NDC translation either way)
    this.jitter[0] = jx; this.jitter[1] = jy;
    m4.copy(this.vpJ, cam.viewProj);
    if (jx !== 0 || jy !== 0) { const ax = 2 * jx / this.width, ay = -2 * jy / this.height; for (let c = 0; c < 4; c++) { const w = this.vpJ[c * 4 + 3]; this.vpJ[c * 4] += ax * w; this.vpJ[c * 4 + 1] += ay * w; } m4.invert(this.invVpJ, this.vpJ); }
    else m4.copy(this.invVpJ, cam.invViewProj);
    let cutNow = false;   // (a cut this frame: nothing reprojects across it)
    if (!this.havePrev) { m4.copy(this.prevVpJ, this.vpJ); m4.copy(this.prevVpU, cam.viewProj); this.prevCamPos = v3.copy(cam.pos); this.prevCamFwd = v3.copy(cam.forward); this.havePrev = true; cutNow = true; if (taauOn) this.resetTaa('first frame'); }
    else {   // a cut nobody can reproject across: the eye jumped rooms (> 2.5 m between two renders — 150 m/s; the follow camera is damped, the tour / breach / attract cut) or swung > 30° at once
      const jump = v3.dist(cam.pos, this.prevCamPos), turn = v3.dot(cam.forward, this.prevCamFwd);
      if (jump > 2.5 || turn < 0.866) { cutNow = true; if (taauOn) this.resetTaa(jump > 2.5 ? `cut (${jump.toFixed(1)} m)` : 'cut (turn)'); else this.passes.dropRestirBeliefs(); }   // (resetTaa drops the ReSTIR tiles' light beliefs too; without TAAU that is all a cut needs from here)
    }
    { // INFRARED LIGHTS (lights.ts LightSet.ir, rust/game_rules/src/teammates/irlight.rs): in the list only while this frame's picture is the tube's — post.wgsl's own swap. The frame the
      // tube comes off, every screen history still holds rooms lit by them (the resolve's, the direct light's, the bounce's, the reflections', the reservoirs' —
      // in a tube frame they are steady lights like any torch): 'wrong wholesale', the case passes.restartHistories names — so no bare-eyed frame shows what only a tube saw.
      const tube = this.settings.nvAmount > 0.5;
      if (!tube && this.lights.tube && this.lights.irSpell) this.passes.restartHistories('the goggles came off infrared light');
      if (!tube) this.lights.irSpell = false;
      this.lights.tube = tube;
    }
    const matSame = this.prevVpU.every((v, i) => v === cam.viewProj[i]);
    const camStill = taauOn && matSame;
    // the CAMERA's own motion this frame in output px (the look-at point and four points 3 m round it through the previous / current unjittered view-projection — an orbit leaves the
    // look-at itself still): the motion-sharpen term of the final blit (taauSharpenMotion) and the direct estimator's motion bits below
    let px = 0;
    if (this.havePrev && !matSame) {
      const prj = (m: ArrayLike<number>, q: Vec3): [number, number] | null => { const x = m[0] * q[0] + m[4] * q[1] + m[8] * q[2] + m[12], y = m[1] * q[0] + m[5] * q[1] + m[9] * q[2] + m[13], w = m[3] * q[0] + m[7] * q[1] + m[11] * q[2] + m[15]; return w > 1e-4 ? [x / w, y / w] : null; };
      for (const off of [[0, 0], [3, 0], [-3, 0], [0, 3], [0, -3]]) { const q: Vec3 = [cam.target[0] + off[0], cam.target[1], cam.target[2] + off[1]]; const a = prj(this.prevVpU, q), b = prj(cam.viewProj, q); if (a && b) px = Math.max(px, Math.hypot((b[0] - a[0]) * 0.5 * (this.outWidth || this.width), (b[1] - a[1]) * 0.5 * (this.outHeight || this.height))); }
    }
    // the unjittered camera MOVED since the last render (TAAU or not; a cut is not motion): the direct estimator's motion bits go up on these frames only. By displacement,
    // not matrix equality — the follow camera's damping keeps nudging the f32 matrices for seconds after the picture has visibly settled, and the motion work (+0.2 ms at high, its levers inert
    // under a pixel's own 1 px/frame) would run all that while (review of 5a08434); 0.02 px a frame is far under anything either program does differently
    this.camMovedAny = this.havePrev && !cutNow && px > 0.02;
    let camMot = 0;
    if (taauOn && !camStill) camMot = Math.max(0, Math.min(1, (px - 0.125) / 0.375));   // 0 at ≤ ⅛ px / frame … 1 at ≥ ½ (taau.wgsl MOTION_PX's ramp)
    const t0 = performance.now();
    this.world.rig = this.rigFit ? { r0: this.rigFit.r0, r1: this.rigFit.r1, kPad: RIG_KPAD, byOwner: this.rigFit.byOwner } : null;   // the SDF rig: the fit packed into this frame's proxy list at upload (r0 over the hand radius, r1 in misc's top byte, the cull sphere fitted + the smooth union's bulge at k ≥ 1 — never tighter than the k = 1 sphere: it is also the mesh shadows' broad phase at ultra, and a finger can reach a few cm past the forearm cone) — null = no fit attached: the list as the game wrote it
    // THE PLANNERS' PACE (render/planPace.ts, render/steps.ts): this frame's allowance of planning, between the order the statics are drawn in (made
    // whole, on a drawn frame once they have stood still: its time is taken off the allowance), the openings and the wall sides (each a job in steps,
    // a step at least every frame), and the place map (drawn at once, its time taken off too). At once for all of them with the pace off, and for the
    // few frames after the picture has started over.
    const plan = this.framePace();
    this.world.upload();
    this.lightMasks(plan);   // (the lights' reach masks: where a wall changed, a step of deciding them again, before anything of this frame reads one; and THE LOOK PROTOTYPE's opening lights, each opening to the sky a light of its own, scene/openings.ts, whose masks are decided in the same step)
    const hourExp = lookExposure(lookConfig.exposure?.() ?? null, this.settings.skyZenith, this.settings.skyHorizon);   // (THE LOOK PROTOTYPE: the exposure of the hour that is up against the night's, render/photometry.ts)
    const shadeCfg = lookConfig.shade > 0 ? lookShade(this.settings.skyZenith, this.settings.skyHorizon, lookConfig.sky, lookConfig.shade) : undefined;   // (THE LOOK PROTOTYPE's shade row, below: the share of a darker place's dark the one exposure model brings up, render/lookexpo.ts; at 0 no place takes a gain)
    const placeOn = !!shadeCfg && shadeCfg[0] > 0;   // (THE LOOK PROTOTYPE's place field, above: wherever the model runs, which is wherever the shade row is up, at every hour)
    if (placeOn) this.buildPlace();   // (a device that cannot build the field shows every place as the open air, with no gain taken: what every page shows for the frames before its field is live, composite.wgsl lookPlaceLevels)
    let eyesWidest: [number, number] | null = null;   // (frame.placeCfg's z and w, render/lookexpo.ts lookExpoBound)
    if (placeOn) { const own = hourExp < 1 ? lookConfig.nightSky?.() ?? null : null;   // (with an hour up, the level's own sky by the page's word: a night's unless the level is its own day, which lookExposure says of it as it does of any sky)
      eyesWidest = lookExpoBound(lookConfig.dark, lookNightSkyE(hourExp, shadeCfg?.[3], own && lookExposure(null, own.zenith, own.horizon) >= 1 ? Math.PI * lookConfig.sky * lookSkyLum(own.zenith, own.horizon, 0.947) : null), this.settings.nvAmount); }
    { const P = this.passes; P.gradeH = lookConfig.gradeH; P.gradeS = lookConfig.gradeS; P.gradeSat = lookConfig.gradeSat; P.gradeC = 1; this.lookGradeCfg = null;   // (THE LOOK PROTOTYPE's grading rows, post.wgsl agx)
      if (lookConfig.gradeC !== 1) { const zen = this.settings.skyZenith, hor = this.settings.skyHorizon, g = lookGrade(lookConfig.sky * lookSkyLum(zen, hor, 0.8), Math.PI * lookConfig.sky * lookSkyLum(zen, hor, 0.947), this.lights.lights, this.settings.exposure * lookConfig.gradeExp, lookConfig.gradeC); P.gradeC = g[0]; P.gradeP = g[1]; this.lookGradeCfg = lookGradeFrame(g, this.settings.exposure * lookConfig.gradeExp); } }
    this.glassLightMasks();   // (glass: which listed panes each light can reach at all → RtLight.glassMask, packed by the upload below; all 0 with the experiment off)
    this.lights.dayExp = lookConfig.lamps > 0 ? hourExp : 1; this.lights.dayAmount = lookConfig.lamps;   // (THE LOOK PROTOTYPE's real light values: render/photometry.ts)
    const numLights = this.lights.upload(time, this.dayLights, this.reach);
    let portal0 = 0; for (const l of this.dayLights) if ((l.slot ?? -1) >= 0 && (!portal0 || l.slot! + 1 < portal0)) portal0 = l.slot! + 1;   // (the list's first portal, + 1: frame.placeCfg.y, common.wgsl skyKeep)
    const place = placeOn && this.place?.ready ? this.place : null; { const pb = place ? place.buffer : null; this.passes.setPlace(pb); this.particles?.setPlace(pb); }   // (the composite's, the smoke march's and the particles' splat's: one field, or the zeros that say none)
    const placeSnap = this.lookEye.snap || this.placeWas !== place;   // (a first frame, a restart, a still, a field just come live: the field and its eye are settled outright; cleared below once a field has been stepped)
    const powder = this.powderLive(); this.passes.setPowder(powder?.ready ? powder.view : null);   // (THE LOOK PROTOTYPE's loose snow in the air, above)
    const veil = powder?.ready ? lookVeilFrame({ x: WORLD.min[0], z: WORLD.min[2], w: WORLD.max[0] - WORLD.min[0], d: WORLD.max[2] - WORLD.min[2] }, 1, this.lights.lights) : null;   // (null anywhere else = the frame's two fields stay zeros)
    if (this.decals) { this.decals.enabled = this.settings.decals; this.decals.frame(this.world); }   // decals (E1): the setting is the picture's switch (off = nothing uploaded; the list stands, it is the game's); attached marks follow their boxes as of THIS upload, the buffer is rewritten only when the list changed
    this.passes.setDecalBuffer(this.decals?.buf ?? null);   // (the composite reads it too: the crack stars rounds leave on glass panes — a no-op unless the buffer changed)
    this.cpuMs.boxes = performance.now() - t0;
    this.ensureSceneBG();

    const s = this.settings;
    // torch bounce cards: nominate the carried spots (RtLight.card) uploaded this frame — the MAX_CARDS nearest the view's look-at point if there are more —
    // by light slot; cards.wgsl rebuilds each one's footprint card ahead of the gather. None with the setting off (no dispatch, flag down).
    const cardSlots = s.torchBounceCards ? cardSlotsFor(this.lights.lights, cam.focus, MAX_CARDS) : [];
    this.passes.numCards = cardSlots.length;
    const meshOn = this.meshShadowsOn;   // FrameFlags.MeshShadows this frame (setting ∧ adapter budget ∧ trees attached); the trees are re-skinned + refit below before anything traces
    const decalsOn = this.decalsOn;      // FrameFlags.Decals this frame (setting ∧ a list attached ∧ something in it)
    const partOn = this.particlesOn;     // FrameFlags.Particles this frame (setting ∧ a Particles attached): its emits are dealt slots and uploaded now, stepped after the cascades, drawn after the composite
    const rtOn = s.rtReflect;            // RT reflections this frame (frame.rtCfg.w = the divisor, 0 while off) — and Gloss is then held DOWN whatever settings.gloss says (the reflection pass covers the polished floors: rtReflect ⊃ gloss)
    const rigBits = (s.charSelfShadow ? 64 : 0) | (s.meshShadowsNear ? 128 : 0) | (s.meshShadowsDirect ? 256 : 0);   // the SDF character rig's EXTRAS this frame (frame.rigCfg.x: 64 character self-shadow · 128 mesh shadows near-only · 256 in the direct light's rays alone; the bits under 64 are free — the round cones themselves are unconditional)
    this.particles?.prepare(dt, partOn, s.particleBirths);
    const numTransientNow = this.lights.lights.reduce((n, l) => n + ((l.ttl >= 0 || l.transient) && l.enabled ? 1 : 0), 0);   const firstTransientNow = numTransientNow ? this.lights.firstTransientSlot : 0;   // (+ the lowest SLOT holding one, from the upload above — the gather's subtraction loop starts there instead of at slot 0 (frame.p5Cfg.w; 0 when none is alive, so a still's frame block is byte for byte what it was — review R2.2))   // transient (ttl / flagged) lights alive this frame — the gather skips its subtraction loop at 0 (the goggles' infrared lights are NOT among them: steady lights in the tube's picture — lights.ts LightSet.ir)
    this.rc.halfRate = s.rcHalfRate;
    // THE LOOK PROTOTYPE, light keeps to its room (the look panel's row `rooms`; lookConfig.rooms 0 = the cache as it was; DAY AND NIGHT, and the game's light
    // meter reads the cache, so on the look page the row changes what the meter and the guards read): the cascades' hand-over overlap (render/rc.ts)
    // and the cache rays' bounce read (lighting_common.wgsl hitRadiance, FrameFlags.RcRoom). Both stop the open air's daylight reaching a shut room's probes.
    const lookSides = wallSidesConfig.on, lookGaps = lookSides || placeOn ? lookConfig.doorGaps?.() ?? NO_GAPS : NO_GAPS;
    if (lookSides || placeOn) { const t = clock(); this.placeMap.update(this.world.statics, this.world.dynamics, lookGaps, this.world.staticsTold); spend(plan, clock() - t); }   // (the map of the level as it stands, ONE a frame and one for both readers: the wall sides here and the place field below. Doors open by degrees in it and no leaf is a wall, scene/placePlan.ts)
    if (lookSides || placeOn) this.wallSides.update(this.world.statics, lookGaps, this.rc.cfg.nx, this.rc.cfg.nz, this.placeMap, plan); else this.wallSides.drop();   // (the plan's words: the Walls have two sides row's for the cache reads, and the exposure's for a pixel's place whatever that row says, composite.wgsl lookPlaceLevels. Made again only when the map's shape changes)
    const sidesStamp = lookSides || placeOn ? this.wallSides.made : -1;   // (which tables the frame block carries this frame; −1 = none)
    const lookRoom = lookConfig.rooms > 0; this.rc.setOverlap(lookRoom ? 1 : 0);
    const lampK = lookRoom && this.lights.dayExp < 1 ? Math.pow(this.lights.dayExp, Math.min(1, this.lights.dayAmount)) : 1;   // (× the night's strength a fixture is drawn at this hour, render/photometry.ts lightDayK: 1 at night and with the row down. Every FIXED night level a lamp's light is tested against is held at this share below: the cache's shadow-ray threshold, the direct estimators' floor, the dim-pixel rule)
    this.rc.amortize = s.rcAmortize; this.rc.setPeriods(s.rcPeriods); this.rc.setChunks(s.rcChunks);   // amortized cascade upkeep (rc.ts): the settings are the source of truth, mirrored like the other cache options
    const jf = (this.rc.staggerUpper || this.rc.halfRate) ? (this.frameIdx >> 1) : this.frameIdx;   // same direction jitter for the pair of frames that share cascade work (stagger: c2+ every other frame; half rate: uppers even / c0 + bake odd)
    // STEADY BOUNCE (the look page's row `steady`, render/diceRing.ts; the maintainer on his GPU, 11-01: "lots of temporal instability for midday", "strange
    // shimmering on metallic barrels", "on the guy's back too", "night, dawn, and dusk look pretty good. midday and overcast are the broken ones"). By day
    // the cache's direction shift is one of a few points taken in turn and its dice is the mean of its last bake at each, so a still scene's bounce does
    // not change from frame to frame. At night, with the row down and until the ring's one program is ready: the cache's own mean, as it was.
    const ringN = lookRoom && (s.flags & FrameFlags.Temporal) ? ringPoints(lampK, lookConfig.steady) : 0;
    if (ringN && !this.diceRing) { try { this.diceRing = new DiceRing(this.device); } catch (err) { console.warn('dice ring: not built on this device', err); lookConfig.steady = 0; lookAmounts.steady = 0; } }
    const ring = ringN && this.diceRing?.ready && this.diceRing.fits(this.rc.dice) ? this.diceRing : null; if (!ring) this.diceRing?.drop();
    const ringAt = ring ? ringSlot(jf, ringN) : 0, ringFresh = this.rc.reprimed, jk = ring ? ringAt : jf;
    this.rc.setDiceBlend((s.flags & FrameFlags.Temporal) && !ring ? 0.3 : 1.0);
    const fp: FrameParams = {
      viewProj: taauOn ? this.vpJ : cam.viewProj, invViewProj: taauOn ? this.invVpJ : cam.invViewProj, prevViewProj: taauOn ? this.prevVpJ : cam.prevViewProj,   // (TAAU off: the camera's own three, untouched — the old path bit for bit)
      camPos: cam.pos, camDir: cam.forward, time, dt, width: this.width, height: this.height,
      frameIdx: this.frameIdx, numLights, flags: (s.flags & ~(FrameFlags.ReachMask | FrameFlags.Gloss | FrameFlags.MeshShadows | FrameFlags.Decals | FrameFlags.Particles | FrameFlags.Glass | FrameFlags.BounceCards))
        | (s.nvAmount > 0.001 ? FrameFlags.NightVision : 0) | (s.lightReach ? FrameFlags.ReachMask : 0)
        | (s.gloss && !rtOn ? FrameFlags.Gloss : 0) | (meshOn ? FrameFlags.MeshShadows : 0)
        | (decalsOn ? FrameFlags.Decals : 0) | (partOn ? FrameFlags.Particles : 0) | (s.glass ? FrameFlags.Glass : 0) | (cardSlots.length > 0 ? FrameFlags.BounceCards : 0)
        | (lookRoom ? FrameFlags.RcRoom : 0),
      shadeCfg: this.lookShadeCfg = shadeCfg, eyeCfg: this.lookEyeCfg = lookExpoFrame(shadeCfg, lookConfig.shade, lookConfig.eyes, lookConfig.glare, hourExp) ?? undefined,   // (the one exposure model, render/lookexpo.ts: w under 0 is composite.wgsl lookShadeGain's word; null = no shade row: no pixel takes a gain by its place)
      veilA: veil?.[0], veilB: veil?.[1], gradeCfg: [this.lookGradeCfg?.[0] ?? 0, this.lookGradeCfg?.[1] ?? 0, 0, 0],   // (z and w: spare. z was the read of the cache on this side of a wall, gone with the guess it served, composite.wgsl lookPlaceLevels; w the Room contrast row's)
      exposure: s.exposure * lookConfig.gradeExp,   // (THE LOOK PROTOTYPE's Exposure row: × the hour's own)   // gloss / meshShadows / decals / gpuParticles / glass / torchBounceCards: the boolean settings are the single source of truth for their bits (frame.ts FrameFlags is the one table; the cards bit only while a card light is actually up, the decals bit only while a mark exists)
      skyZenith: lookConfig.sky !== 1 ? [s.skyZenith[0] * lookConfig.sky, s.skyZenith[1] * lookConfig.sky, s.skyZenith[2] * lookConfig.sky] : s.skyZenith, skyHorizon: lookConfig.sky !== 1 ? [s.skyHorizon[0] * lookConfig.sky, s.skyHorizon[1] * lookConfig.sky, s.skyHorizon[2] * lookConfig.sky] : s.skyHorizon, hazeDensity: s.hazeDensity * lookConfig.air,
      rcInterval0: this.rc.cfg.interval0, rcC0Dims: [this.rc.cfg.nx, this.rc.cfg.ny, this.rc.cfg.nz], rcNumCascades: this.rc.cfg.cascades, rcD0: this.rc.cfg.d0, rcFrameParity: this.frameIdx & 1,
      numSmoke: this.smoke.count, debugView: s.debugView, indirectScale: s.indirectScale, emissiveScale: s.emissiveScale * emissiveDayK(this.lights.dayExp, this.lights.dayAmount), volSteps: s.volSteps,
      capColor: s.capColor, post: [s.nvAmount, s.nvGain, s.hitFlash, Math.tan(cam.fov / 2)],
      directCfg: [0, 0, s.directBlurCap, 0],   // (.z the penumbra filter's cap; the rest free)
      rcJitter: (s.flags & FrameFlags.Temporal) ? [0.1 + 0.8 * ((jk * 0.7548776662466927) % 1), 0.1 + 0.8 * ((jk * 0.5698402909980532) % 1)] : [0.5, 0.5],   // (jk: the frame's own point, or under the steady bounce's ring one of its few)
      numTransient: numTransientNow,
      smokeDraw: this.smokeDrawWords(), powderDraw: powderDrawWord(),
      sides: lookSides || placeOn ? this.wallSides.words : null, sideCols: lookSides && this.wallSides.fits ? this.wallSides.cols : null,   // (THE LOOK PROTOTYPE's Walls have two sides row: lighting_common.wgsl diceTaps, diceFacePointBy)
      placeCfg: (placeOn && lookConfig.order > 0) || portal0 > 0 || eyesWidest ? [placeOn && lookConfig.order > 0 ? Math.min(lookConfig.order, 1) : 0, portal0, eyesWidest?.[0] ?? 0, eyesWidest?.[1] ?? 0] : undefined,   // (THE LOOK PROTOTYPE, light keeps its order across places: with the place field · z, w: the eyes' widest, render/lookexpo.ts lookExpoBound)
      lampCfg: lampK < 1 ? [lampK, lampK / emissiveDayK(this.lights.dayExp, this.lights.dayAmount), lookConfig.steady > 0 ? 1 : 0, Math.max(lookConfig.roomBright, 0.001)] : undefined,   // (.z: the Steady bounce row is up, lighting_common.wgsl dayBoundK · .w: the Room brightness row, composite.wgsl lookExpoGain)   // (common.wgsl lampFloorK, boxEmissiveRay)
      secMinE: s.secMinE * lampK,   // (by day a fixture is drawn at the hour's exposure × its night strength, render/photometry.ts, so the night's threshold is held at the same share of it: every lamp the night's cache carries, the day's does too)
      //   // R2 sequence over the octahedral texel: cascade direction aliasing becomes noise for the indirect temporal filter
      lossyCfg: [this.passes.gatherDiv, this.passes.volDiv, 0, Math.max(0, s.giFireflyK)],   // .w: the gather's relative firefly bound (0 = off)
      cardSlots,
      weather: this.weather,   // rain / splashes / sky flash: drawn by the volumetric pass under FLAG_VOLUMETRICS (beams off = no weather on screen; the game side carries on)
      glassCfg: [Math.max(0, s.glassReflect), Math.max(0, s.glassTintDepth), Math.max(0, s.glassDust), s.glassRefract && s.glass ? Math.max(1, Math.min(3, s.glassIor || 1.5)) : 0],   // .w: refraction — the index of refraction while on (≥ 1), 0 = off (composite.wgsl glassRefractPath)
      wxAirSteps: Math.max(0, Math.min(24, Math.round(s.wxAirSteps || 0))),   // the weather air march's step cap (volCfg.w bits 24‥28; 0 = half the tier's beam steps)
      volCfg: [Math.max(1, Math.min(6, Math.round(s.volLightCap || 4))), Math.max(1, Math.min(3, Math.round(s.volShadowEvery || 1))), Math.max(10, Math.min(30, Math.round((s.volStepVoxels || 1.5) * 10))),
        Math.max(3, Math.min(15, Math.round(s.smokeShadowSteps || 8))) | (Math.max(0, Math.min(15, Math.round((s.smokeMultiScatter ?? 1.0) * 10))) << 12) | (Math.max(0, Math.min(15, Math.round((s.smokeTaaReact ?? 0) * 10))) << 16) | ((s.smokeUpsampleRelax ? 1 : 0) << 20)],   // (+ bits 16‥19 the TAAU smoke reactivity × 10, bit 20 the upsample's depth relax — smoke battery experiments)   // (.w bits 12‥15: the multiple-scattering octave's gain × 10, vol_common.wgsl smokeMsGain)   // (.w bits 8‥11 free)   // volumetric caps: K lights per march step / haze ray (1 or more: the march has no path without a cap; a settings table no preset has written, 0, is read as high's 4), shadow-ray reuse period, march step in tenths of a voxel; .w = the SHADOW rays' smoke sample count (common.wgsl smokeShadowSteps: 8 = as ever)
      prevCamPos: this.prevCamPos,   // the eye of the previous RENDER (TAAU's per-render snapshot, seeded with this frame's on the first): read by the specular denoiser's mirror-image validation
      rtCfg: [Math.max(0, Math.min(1, s.rtMaxRough)), Math.max(0, Math.min(4, Math.round(s.rtRays))), Math.max(0, Math.min(1, s.rtAniso)) + 2 * Math.round(10 * Math.max(0, Math.min(1, s.wetBreakup ?? 0))), rtOn ? this.passes.rtDiv : 0],   // .z = the anisotropy 0‥1 + 2 × the wet ground's break-up in tenths (0 = .z is the anisotropy alone, as it always was)
        // .w = 0: the reflection pass is off this frame (nothing reads the rest then)
      matCfg: [Math.max(0, s.rtGlints), Math.max(0, s.rtStrength), 60, Math.max(0, Math.min(1, s.detail))],   // .w = the g-buffer's detail-layer amount (0 = fsBox as it was)
      p5Cfg: [0, 0, 0, firstTransientNow],   // (.x‥.z free: the resampling gather's and the reflection twin's bits until 2026-11-07; .w = the first transient light's slot)
      jitter: [this.jitter[0], this.jitter[1], this.prevJitter[0], this.prevJitter[1]],
      markCfg: [s.markings ? Math.max(0.005, Math.min(0.2, s.markingTexel)) : 0, Math.max(0, Math.min(1, s.markingContrast)), Math.max(0, s.markingEmissive), s.markingReflect ? (s.markingReflPattern === false ? -1 : 1) * (1 + Math.max(0, Math.min(4, s.markingReflCone))) : 0],   // markings: .x 0 = off everywhere; .w 0 = not at reflection hits, else ±(1 + the traced-hit cone's lobe share) — negative = the analytic highlight left uniform (markings.wgsl markConeK / reflect.wgsl)
      sharpCfg: this.sharpOn ? [this.width, this.height, this.outWidth, this.outHeight] : [0, 0, 0, 0],   // sharp direct: the two sizes every twin kernel maps between (z 0 = off: every pass as it was)
      sharpCfgB: [0, 0, 0, Math.max(1e-4, s.sharpPlaneK)],   // its knob (w = the same-plane k; xyz free)
      sharpCfgC: [decouple ? this.jitter[0] : 0, decouple ? this.jitter[1] : 0,   // DECOUPLED JITTER: what every internal ↔ output texel map adds under it (0 = one raster jitter, the sharp path as it was)
        (Math.max(0, Math.min(3, Math.round(s.sharpSliver))) & 3) | ((Math.max(0, Math.min(64, Math.round(s.sharpSliverCap))) & 127) << 8) | (s.dnShadowOccl ? 1 << 15 : 0),   // .z = the SLIVER rule | per-tile ray cap << 8 (sharp_direct.wgsl; 0 = the fallback as it was) — an exact small integer
        s.gbaaClamp ? 1 : 0],   // .w = GBAA's same-surface clamp (gbaa.wgsl; 0 = the blend as it was)
      rigCfg: [rigBits, 0, 0, 0], rigCfgB: [0, 0, 0, 0],   // (.x the rig's two bits; the rest free: the soft shadow and smooth union trials' numbers until 2026-11-07)
    };
    this.passes.smokeSlots = fp.numSmoke;   // (what the shaders are told: 0 = no smoke march anywhere this frame, so the smoke diffusion's blur chain has no reader — passes.ts)
    writeFrame(this.frameData, fp);
    // (the look prototype's two tables are 30 KB of the block and change when a door comes to rest: they are sent when they have changed, the head every frame)
    this.device.queue.writeBuffer(this.frameBuf, 0, this.frameData, 0, sidesStamp >= 0 && sidesStamp !== this.sidesSent ? FRAME_BYTES : FRAME_HEAD_BYTES); if (sidesStamp >= 0) this.sidesSent = sidesStamp;
    this.passes.sliverRays = Math.round(s.sharpSliver) >= 1 && Math.round(s.sharpSliverCap) > 0;   // (the sliver rule picks the sharp kernel's traced variant)
    if (this.sharpOn) { writeFrameOut(this.frameDataOut, this.frameData, this.outWidth, this.outHeight, decouple ? { viewProj: cam.viewProj, invViewProj: cam.invViewProj, prevViewProj: this.prevVpU } : undefined); this.device.queue.writeBuffer(this.frameBufOut, 0, this.frameDataOut, 0, sidesStamp >= 0 && sidesStamp !== this.sidesSentOut ? FRAME_BYTES : FRAME_HEAD_BYTES); if (sidesStamp >= 0) this.sidesSentOut = sidesStamp; this.ensureSceneBGOut(); this.passes.sceneBGOut = this.sceneBGOut; }   // the OUTPUT frame block + its scene group for the output-size passes (decoupled: with the camera's own unjittered matrices — its raster stands still)
    else this.passes.sceneBGOut = null;
    const jitterRes: [number, number] = decouple ? [0, 0] : this.sharpOn ? [this.jitter[0] * this.outWidth / this.width, this.jitter[1] * this.outHeight / this.height] : this.jitter;   // the resolve reads the jitter in the pixels of the picture it resolves (the output's under sharp direct; none when the output raster is decoupled from the internal jitter)
    if (taauOn) this.passes.setTaauFrame({ prevViewProjU: this.prevVpU, jitter: jitterRes, bypass: s.debugView !== 0, alpha: s.taauAlpha, clipGamma: s.taauClipGamma, reactK: s.taauReactive, kSharp: s.taauKernel, velK: s.taauVelK, sharpen: s.taauSharpen + s.taauSharpenMotion * camMot, camStill, motionA: s.taauMotionAlpha, histSharp: s.taauHistSharp,
      particles: partOn && s.debugView === 0 && !!this.particles && this.particles.drawEnabled && this.particles.liveSlots > 0 });   // (the resolve's own uniform: last frame's UNJITTERED view-projection for the output history, this frame's jitter, the knobs; 'particles' = their splat runs this frame, so its reactive mask is fresh)
    // what the NEXT render reprojects with / detects a cut against (snapshotted now: everything above has read the previous values)
    m4.copy(this.prevVpJ, this.vpJ); m4.copy(this.prevVpU, cam.viewProj); this.prevCamPos = v3.copy(cam.pos); this.prevCamFwd = v3.copy(cam.forward);
    this.prevJitter[0] = this.jitter[0]; this.prevJitter[1] = this.jitter[1];

    const t1 = performance.now();
    const timer = this.timer; timer.beginFrame();
    const enc = this.device.createCommandEncoder({ label: 'frame' });
    const ts = (n: string) => timer.pass(n);
    this.fogSight.encode(enc, this.passes.fogOn ? this.fogFrame : null, this.sceneBGPlain!, ts); if (this.fogFrame) this.fogFrame.sight = this.fogSight.live;   // the fog of war's sight table, before anything lights a pixel (a no-op unless the full-picture fog is on)
    if (this.smoke.encode) this.smoke.encode(enc, ts);
    if (meshOn) { if (this.meshShadows!.plan(this.world.charCount)) this.meshShadows!.encode(enc, ts('mesh')); else timer.idle('mesh'); }   // skin + refit the tree of every listed character who has moved from this frame's joint matrices — before the cascades, the first pass that traces
    this.gbuffer.encode(enc, this.sharpOn ? this.sceneBGOut! : this.sceneBG!, this.world.visibleBuf, this.world.visibleCount, this.skin, this.dummyStorage, this.decals?.buf ?? this.dummyStorage, ts('gbuffer') as GPURenderPassTimestampWrites | undefined, this.world.visibleCutCount);   // (the decal buffer is only read under FrameFlags.Decals: without a list the dummy is bound and the flag stays down) — under sharp direct at the OUTPUT size (the output frame block: its pixel-footprint terms mean an output pixel)
    this.gbuffer.encodeDown(enc, this.sceneBG!);   // sharp direct: the internal-size view every internal pass binds, derived from that raster (a no-op with it off)
    this.rc.encode(enc, this.sceneBG!, timer, this.frameIdx);
    if (ring && this.rc.diceBaked) ring.step(enc, this.rc.dice, ringAt, ringN, ringFresh, ts); else if (ring) timer.idle('diceRing');   // (the bake just made, kept as its point, and the mean of the points held written back: what every pass below reads)
    if (place) { place.setMap(this.placeMap); place.step(enc, this.rc.diceView, dt, lookConfig.eyeAt?.() ?? null, placeSnap, ts); this.lookEye.snap = false; } this.placeWas = place;   // (after the cache's own pass: the field is solved from this frame's light)
    const airRuns = !!this.air && this.airOn && s.airSim && this.air.ready !== false;   // THE AIR (above): at most one step a drawn frame, on its own clock; a paused frame (dt 0) steps nothing
    if (airRuns || powder) {
      this.airAcc += dt;
      if (dt > 0 && this.airAcc >= 0.8 / 60 * Math.max(1, s.airEvery || 1)) {
        const h = Math.min(this.airAcc, 1 / 30); this.airAcc = 0;
        if (airRuns) this.air!.step(enc, h, time, this.airWind, ts('air'));
        powder?.step(enc, h, time, airRuns ? Math.hypot(this.airWind[0], this.airWind[1]) : 0, Math.min(1, lookConfig.veil), airRuns, ts);   // (the look prototype's powder rides the air as it now stands: stepped on the air's clock, and in air that has come to rest too, where a boot's puff still rises and falls)
      } else { if (airRuns) timer.idle('air'); if (powder) timer.idle('powder'); }   // (a frame between two steps — a fast display, airEvery — costs nothing: said, or the pass would go on showing its last step's time)
    }
    if (partOn) this.particles!.setAir(airRuns ? this.air!.view : null);   // (what the particle step reads: the field while it is stepped — its stand-in, still air, otherwise)
    if (partOn && this.particles!.hasWork) this.particles!.encodeUpdate(enc, this.sceneBG!, this.rc.diceView, ts('particles'));   // (only ask the timer for a slot when the pass WILL encode: an allocated-but-unwritten timestamp pair resolves to stale garbage — ultra summed a fictional 7–12 ms 'particles' with nothing alive, 08-10 measurements)   // births + the step (collide against this frame's boxes, shade off this frame's lights and dice) — a return with nothing live
    let canvasView: GPUTextureView;
    if (present) canvasView = this.gpu.ctx.getCurrentTexture().createView();
    else {
      if (!this.offscreenTarget || this.offscreenTarget.width !== this.outWidth || this.offscreenTarget.height !== this.outHeight) {   // (pooled like the real targets under keepHistories, so a scripted resize measures what a presented one costs — the canvas's own backing store aside)
        const desc = { label: 'offscreen', size: [this.outWidth, this.outHeight] as [number, number], format: this.gpu.format, usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC };
        if (this.settings.keepHistories) { this.pool.release(this.offscreenTarget); this.offscreenTarget = this.pool.acquire(desc).tex; }
        else { this.offscreenTarget?.destroy(); this.offscreenTarget = this.device.createTexture(desc); }
      }
      canvasView = this.offscreenTarget.createView();
    }
    this.passes.debugView = s.debugView;
    { const P = this.passes; P.restirRays = s.restirRays; P.restirSpatial = s.restirSpatial; P.restirHistory = s.restirHistory; P.restirRadius = s.restirRadius; P.restirGradient = s.restirGradient; P.restirGradK = s.restirGradK; P.restirYoung = s.restirYoung; P.restirSpatialAll = s.restirSpatialAll; P.restirPenRays = s.restirPenRays; P.restirAudit = s.restirAudit; P.heldRays = s.heldRays; P.heldAdaptive = !!s.heldAdaptive; P.restirStableRetrace = !!s.restirStableRetrace; P.restirDimClass = s.restirDimClass || 0; P.restirHueSplit = s.restirHueSplit; P.restirKnownDark = s.restirKnownDark; P.restirJitter = this.jitter;
      P.restirMotion = s.restirMotion; P.restirMotionLive = s.restirMotion > 0 && this.camMovedAny; P.restirMotionV = s.restirMotionV; P.restirMotionMin = s.restirMotionMin; P.restirMotionPx = s.restirMotionPx; P.restirMotionTaps = s.restirMotionTaps;   // (MOTION: 0 = main's pipeline; passes.encode picks the RESTIR_MOTION twin otherwise)
      P.writeRestir(this.lights); }   // ReSTIR DI: the settings are the source of truth; the uniform (cfg + last frame's slot maps and light records, kept by lights.upload above) goes up every frame the pass runs
    { // the denoiser's clients' knobs (the settings are the source of truth, mirrored like every other pass option)
      const K = this.passes.den.knobs;
      K.shIters = s.dnShadowIters; K.shCap = s.directHistory ? s.dnShadowCap : 0; K.shClampK = s.dnShadowClampK; K.shSigmaL = s.dnShadowSigmaL; K.shPrior = s.dnShadowPrior; K.shLitAlike = s.dnShadowLitAlike; K.shOccl = s.dnShadowOccl; K.shMoveCap = s.dnShadowMoveCap; K.shMoveThresh = s.dnShadowMoveThresh; K.shJump = s.dnShadowJump;
      K.giIters = s.dnGiIters; K.giCap = s.dnGiCap; K.giClampK = s.dnGiClampK; K.giSigmaL = s.dnGiSigmaL; K.giFast = s.dnGiFast;
      K.spIters = s.dnSpecIters; K.spCap = s.dnSpecCap; K.spClampK = s.dnSpecClampK; K.spSigmaL = s.dnSpecSigmaL; K.spRough = s.dnSpecRough; K.spMaxRough = s.rtMaxRough; K.rfIters = s.rtAtrous; K.rfCap = s.rtHistory;   // (+ the reflection chain's per-pixel cutoff, its own iteration count and history cap)
      K.depthK = s.dnDepthK; K.normalPow = s.dnNormalPow; K.motionVectors = s.dnMotionVectors;   // (the g-buffer writes gVel on both the TAAU and the FXAA path: prev − now through frame.prevViewProj, jittered or not — consistent with the reprojection either way)
      this.passes.den.writeParams({ gatherDiv: this.passes.gatherDiv, reflDiv: rtOn ? this.passes.rtDiv : 0, temporalFlag: (fp.flags & FrameFlags.Temporal) !== 0, shadowTemporal: !!s.directHistory, historyScale: this.passes.directHistoryScale < 1 ? 0.5 : 1, cueUv: true });
    }
    // the particles' splat goes INTO the HDR target between the composite and the bloom (so hot sparks bloom and nothing has a history to smear); debug views keep the screen theirs.
    // Not timestamped: render-pass timings on Metal read as a frame (see CLAUDE.md) and would poison the HUD total and auto quality's calibration — particles.timeDraw opts in for a look
    const overlay = partOn && s.debugView === 0 && this.particles!.drawEnabled ? (e: GPUCommandEncoder) => this.particles!.encodeDraw(e, this.sharpOn ? this.sceneBGOut! : this.sceneBG!, this.passes.hdrView, this.passes.reactView, this.sharpOn ? this.gbuffer.depthOView : this.gbuffer.depthView, this.passes.volView, this.particlesTimeDraw ? ts('pdraw') as GPURenderPassTimestampWrites | undefined : undefined, s.particleRain ? this.passes.sceneCopy : null) : undefined;   // (+ the composite's copy for the particle rain's refraction while it rains particles)   // (sharp direct: the splat lands in the OUTPUT-size hdr against the output-size depth)
    this.passes.encode(enc, this.sceneBG!, this.gbuffer, this.rc, canvasView, ts, fp.flags, overlay);
    this.gbuffer.snapshotPrev(enc, this.world, this.skin);   // this frame's box transforms + joint palette → the 'previous' buffers the NEXT g-buffer pass reads for its motion vectors (copies at the tail of the command buffer; next frame's uploads are ordered after them)
    timer.resolve(enc);
    this.device.queue.submit([enc.finish()]);
    timer.afterSubmit();
    this.particles?.afterSubmit();
    for (const f of this.afterSubmit) f();
    this.cpuMs.encode = performance.now() - t1;
    this.frameIdx++;
  }
}

export { FrameFlags };

/** THE LOOK PROTOTYPE, eyes adjust to shade (frame.shadeCfg; the one exposure model's, render/lookexpo.ts, place_common.wgsl lookExpoFor): the row's amount,
 *  AT EVERY HOUR (it faded out into a night until 11-04; what holds a night now is the model's own bound, render/lookexpo.ts lookExpoBound), and the two
 *  things a shaded place is measured against until the place field is live, both from the sky ALONE as common.wgsl skyRadiance paints it (horizon to
 *  zenith over the first 0.4 of height): the mean radiance open air holds (half the upper hemisphere's mean, 0.8 zenith + 0.2 horizon) and the irradiance
 *  open ground gets (π × (0.947 zenith + 0.053 horizon)). The third number is spare (it was the most the rows before the model brought a pixel up). */
function lookShade(zen: ArrayLike<number>, hor: ArrayLike<number>, skyK: number, amount: number): [number, number, number, number] {
  const up = skyK * lookSkyLum(zen, hor, 0.8);
  return [Math.min(1, amount), 0.5 * up, 0, Math.PI * skyK * lookSkyLum(zen, hor, 0.947)];
}
/** the sky's luminance, `a` of the zenith and the rest of the horizon (0.8 = the upper hemisphere's mean radiance, 0.947 = its cosine-weighted one, over common.wgsl skyRadiance's ramp) */
function lookSkyLum(zen: ArrayLike<number>, hor: ArrayLike<number>, a: number): number { const b = 1 - a; return 0.2126 * (a * zen[0] + b * hor[0]) + 0.7152 * (a * zen[1] + b * hor[1]) + 0.0722 * (a * zen[2] + b * hor[2]); }

/** THE LOOK PROTOTYPE's real light values (render/photometry.ts; lights.ts LightSet.dayExp): the exposure of the hour that is up, against the night's.
 *  `said` = the page's word (host/drawpage.ts: the game's hour through photometry.ts hourExposure; null = a night, or nobody says). Where nobody says, a level lit as a
 *  day by its OWN sky is taken for a clear midday by how bright that sky is, read WITHOUT the Sky light row: its mean
 *  radiance from 0.08 to 0.35 runs the exposure from the night's 1 to midday's 1/5,000. A night is 1 exactly: nothing of it is touched. Never a guess from a
 *  light's strength, since most levels' own moons are 0.9 and one is 2.6. */
function lookExposure(said: number | null, zen: ArrayLike<number>, hor: ArrayLike<number>): number {
  return said !== null ? said : Math.pow(10, -3.7 * smoothstep(0.08, 0.35, lookSkyLum(zen, hor, 0.8)));
}
