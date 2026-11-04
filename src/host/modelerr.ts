// HOW FAR THE GAME'S OWN SENSES ARE FROM THE PICTURE (the page's `?modelerr`; headless, tools/qa/modelerr.ts). A development tool: nothing of it is made
// without the word, and the game never reads it.
//
// The senses can be worked out by the game itself (game/sense.ts SENSE.cpu: the light estimate, the smoke as blobs or on the coarse grid) where the page
// reads them off the renderer (the probe: render/probe.ts). The two are MODELS OF ONE WORLD and are never slaved to each other, so something has to say how
// far apart they stand. This does: it walks one scripted plan over the level that is open, and at every sample asks the SAME question of both.
//   light   a point at chest or head height: the probe's meter there (W/m², its best axis) beside the estimate (guards.ts lightEstimateAt)
//   smoke   a sight line near a canister the plan sets off, at fixed ages of the smoke: the probe's transmittance along it beside each smoke model's
// and reports the spread of the difference a model, in the units the game decides by: the light meter's own 0‥1 scale (Game.visibilityOf) and the share of
// light a line lets through. Beside each spread, how often model and picture fall on OPPOSITE SIDES OF A DECISION (lit or dark for a man standing there,
// hidden by the plume or not): that is what changes a fight.
//
// THE PLAN IS THE LEVEL'S OWN: stations are picked from its free floor by one rule (the farthest from those already picked), the points round a station by
// another, and the models are stepped on a fixed sixtieth whatever the page's frame rate. So a headless run of the same level asks the models the very same
// questions at the very same ages, and a report made on a GPU (the picture's answers, kept in the file) judges the models as they are LATER, with no GPU: the
// file's `world` word says whether the stand is the one that was recorded.
//
// A MODEL IS A ROW of `MODELS`: a new one (a light model with bounce) is a row here and a row of `BUDGET`, and everything below carries it.
// The plan stands the player at each station (the picture is at its best round the camera), unseen and unhurt, with the AI held still, so that nothing
// moves a door or a lamp between the page and the headless run; the three flags are put back as they were when the plan ends. It is not made on a page
// that plays a tape or a networked game: it moves the player and lets smoke out, and neither of those is that page's to do.
import type { Game } from '../game/game';
import type { Vec3 } from '../math/vec';
import type { SmokeEmitter } from '../smoke/types';
import { CpuSmoke, SmokeGrid, SENSE, GRID, type LightField } from '../game/sense';
import { lightEstimateAt, steadyLamps, LIGHTS } from '../game/guards';
import { smokeParams } from '../game/combat';
import { saveFile, type SavedKind } from './savefile';

export const MODELERR = { on: false };
export const modelErrAsked = (q: URLSearchParams): boolean => q.has('modelerr');
/** what a page was opened on, as the report says it: the address's own words, without this tool's and without any that name a server or a game to join */
export const sceneOf = (q: URLSearchParams): string => [...q.entries()].filter(([k, v]) => !/^(modelerr|net|join|cloud\w*)$/.test(k) && /^[\w.-]*$/.test(v)).map(([k, v]) => v ? `${k}=${v}` : k).join('&');

/** how much of a plan: stations over the level, light points and sight lines a station, the ages (s) a canister's smoke is read at, and the seconds a
 *  station is stood at before anything is asked (the picture's bounce and its history settle) */
export interface Plan { stations: number; light: number; lines: number; ages: number[]; settle: number }
export const PLANS: Record<string, Plan> = {
  full: { stations: 6, light: 40, lines: 32, ages: [3, 8, 15, 25], settle: 2 },
  quick: { stations: 2, light: 24, lines: 16, ages: [3, 6], settle: 0.5 },
  // (what a software adapter can draw in an hour: tools/qa/modelerr-page.cjs)
  tiny: { stations: 1, light: 24, lines: 16, ages: [3, 6], settle: 0.5 },
};
/** under this share of light a plume hides a standing man (guards.ts canSee's own line) */
export const HIDES = 0.3;

/** one question, and what each side said: `pic` the picture (null: never asked, or no answer came), `m` a value a model (null: that model does not stand
 *  here). A light sample has its point `p`; a smoke one its line `a` → `b` and the age `t` of the smoke */
export interface Sample { k: 'light' | 'smoke'; s: number; p?: Vec3; a?: Vec3; b?: Vec3; t?: number; pic: number | null; m: Record<string, number | null> }
export interface Report {
  v: 1; plan: string; scene: string;
  /** a word for the questions (how many boxes the level has, the stations, every point and line asked, every age): two runs with one word asked the
   *  same things of the same floor. Not a word for the answers: what a lamp gives moves from tick to tick, and a model is there to be changed */
  world: string;
  stations: Vec3[];
  /** was the picture asked at all (false: a headless run, the models alone) */
  picture: boolean;
  /** the light meter's curve and its line for a lit man, as the game had them (Game.tune, guards.ts LIGHTS.litAt) */
  meter: { slope: number; floor: number; litAt: number };
  /** asked of the picture and never answered */
  unanswered: number;
  where?: Record<string, string>;
  samples: Sample[];
}

// ---- the models: what each is asked, by kind
type LightOf = (r: ModelErr, p: Vec3, skip: (owner: number) => boolean) => number | null;
type SmokeOf = (r: ModelErr, a: Vec3, b: Vec3) => number | null;
export const MODELS: { name: string; kind: 'light' | 'smoke'; light?: LightOf; smoke?: SmokeOf }[] = [
  { name: 'estimate', kind: 'light', light: (r, p, skip) => lightEstimateAt(r.g, p, skip, false, null) },
  { name: 'lit', kind: 'light', light: (r, p, skip) => r.lit && r.lit.grid.stands ? lightEstimateAt(r.g, p, skip, false, r.lit) : null },
  { name: 'blobs', kind: 'smoke', smoke: (r, a, b) => r.blobs.trans(a, b, (p, q) => r.g.col.segmentBlocked(p, q)) },
  { name: 'grid', kind: 'smoke', smoke: (r, a, b) => { const G = r.grid.grid; if (!G) return null; const t = G.along(a, b); return t === -1 ? null : t; } },
];
/** THE BUDGET a model: the most its error may be at the middle and at the 95th percentile (light: on the meter's 0‥1 scale; smoke: as a share of light)
 *  and the largest share of samples on which it may decide the other way from the picture. A model with no row is reported and not judged. The numbers
 *  are a measured run's with room over them, and docs/agents/rust.md says which run (the model against the picture): they are a bound on getting WORSE,
 *  not a claim that this much is good. TODAY'S ARE FROM THE SOFTWARE ADAPTER (one station of the office, Very low): a report from a GPU replaces them. */
export interface Budget { p50: number; p95: number; flips: number }
export const BUDGET: Record<string, Budget> = {
  estimate: { p50: 0.08, p95: 0.15, flips: 0.1 },
  lit: { p50: 0.05, p95: 0.15, flips: 0.1 },
  blobs: { p50: 0.25, p95: 1, flips: 0.45 },
  grid: { p50: 0.08, p95: 1, flips: 0.32 },
};

// ---- the spread
export interface Spread { model: string; kind: 'light' | 'smoke'; n: number; bias: number; p50: number; p95: number; max: number; flips: number; stops95: number }
const clamp01 = (x: number) => x < 0 ? 0 : x > 1 ? 1 : x;
const meterOf = (R: Report, E: number) => clamp01(R.meter.slope * Math.log(Math.max(E, 1e-5) / R.meter.floor));
const at = (sorted: number[], q: number) => sorted.length ? sorted[Math.min(sorted.length - 1, Math.floor(q * sorted.length))] : 0;
/** the spread of every model of `R` against `ref`: the picture ('pic'), or another model's name (two models against each other, where no picture was asked) */
export function spreadOf(R: Report, ref = 'pic'): Spread[] {
  const out: Spread[] = [];
  for (const M of MODELS) {
    if (M.name === ref) continue;
    const err: number[] = [], stops: number[] = []; let sum = 0, flips = 0;
    for (const s of R.samples) {
      if (s.k !== M.kind) continue;
      const want = ref === 'pic' ? s.pic : s.m[ref] ?? null, got = s.m[M.name] ?? null;
      if (want === null || got === null || !Number.isFinite(want) || !Number.isFinite(got)) continue;
      let e: number;
      if (M.kind === 'light') {
        const a = meterOf(R, got), b = meterOf(R, want); e = a - b; if ((a >= R.meter.litAt) !== (b >= R.meter.litAt)) flips++;
        if (got > R.meter.floor || want > R.meter.floor) stops.push(Math.abs(Math.log2(Math.max(got, 1e-5) / Math.max(want, 1e-5))));   // (two readings under the meter's floor are both 'dark': no stops between them worth the name)
      } else { e = got - want; if ((got < HIDES) !== (want < HIDES)) flips++; }
      err.push(Math.abs(e)); sum += e;
    }
    err.sort((a, b) => a - b); stops.sort((a, b) => a - b);
    const n = err.length;
    out.push({ model: M.name, kind: M.kind, n, bias: n ? sum / n : 0, p50: at(err, 0.5), p95: at(err, 0.95), max: n ? err[n - 1] : 0, flips: n ? flips / n : 0, stops95: at(stops, 0.95) });
  }
  return out;
}
/** fewer samples than this judge nothing */
export const LEAST = 20;
export interface Verdict { ok: boolean; judged: number; lines: string[] }
/** a report against the budget: every model with a row and enough samples is inside it, or the verdict is not ok */
export function judge(R: Report, budget: Record<string, Budget> = BUDGET): Verdict {
  const lines: string[] = []; let ok = true, judged = 0;
  for (const s of spreadOf(R)) {
    const B = budget[s.model], unit = s.kind === 'light' ? 'of the meter' : 'of the light';
    const said = `${s.model} (${s.kind}): ${s.n} samples · off by ${s.p50.toFixed(3)} at the middle, ${s.p95.toFixed(3)} at the 95th, ${s.max.toFixed(3)} at worst ${unit} · leans ${s.bias >= 0 ? '+' : ''}${s.bias.toFixed(3)} · decides the other way on ${(s.flips * 100).toFixed(1)}%${s.kind === 'light' ? ` · ${s.stops95.toFixed(2)} stops at the 95th` : ''}`;
    if (!R.picture) { lines.push(`${said} · NO PICTURE WAS ASKED`); continue; }
    if (!B) { lines.push(`${said} · no budget row: not judged`); continue; }
    if (s.n < LEAST) { lines.push(`${said} · too few to judge`); continue; }
    const good = s.p50 <= B.p50 && s.p95 <= B.p95 && s.flips <= B.flips; judged++; if (!good) ok = false;
    lines.push(`${said} · budget ${B.p50}, ${B.p95} and ${(B.flips * 100).toFixed(1)}%: ${good ? 'inside' : 'PAST IT'}`);
  }
  return { ok, judged, lines };
}

// ---- the plan
const TICK = 1 / 60, GAP = 30;
const fnv = (h: number, x: number) => Math.imul(h ^ (Math.round(x * 100) | 0), 0x01000193);
function worldOf(R: Report, boxes: number): string {
  let h = fnv(0x811c9dc5, boxes);
  for (const s of R.stations) { h = fnv(h, s[0]); h = fnv(h, s[2]); }
  for (const s of R.samples) for (const v of s.p ?? [...s.a!, ...s.b!, s.t!]) h = fnv(h, v);
  return (h >>> 0).toString(36);
}
/** stations: free floor the player's start can walk to, with a stride of free floor round it; the first is the one nearest the start, each next the one
 *  farthest from all picked so far */
export function stationsOf(g: Game, n: number): Vec3[] {
  const nav = g.col.nav, W = nav.W, H = nav.H, c = nav.cell, B = nav.blocked;
  const me = g.player.char.pos, sx = Math.min(W - 1, Math.max(0, Math.floor(me[0] / c))), sz = Math.min(H - 1, Math.max(0, Math.floor(me[2] / c)));
  const seen = new Uint8Array(W * H), queue = new Int32Array(W * H); let head = 0, tail = 0;
  // (the start may be standing in a cell the bake calls blocked: the walk begins from the nearest free one in its ring of eight)
  let s0 = sz * W + sx; if (B[s0]) for (let dz = -1; dz <= 1; dz++) for (let dx = -1; dx <= 1; dx++) { const x = sx + dx, z = sz + dz; if (x >= 0 && z >= 0 && x < W && z < H && !B[z * W + x] && B[s0]) s0 = z * W + x; }
  if (B[s0]) return [];
  seen[s0] = 1; queue[tail++] = s0;
  while (head < tail) { const i = queue[head++], x = i % W, z = (i - x) / W;
    if (x > 0 && !seen[i - 1] && !B[i - 1]) { seen[i - 1] = 1; queue[tail++] = i - 1; }
    if (x < W - 1 && !seen[i + 1] && !B[i + 1]) { seen[i + 1] = 1; queue[tail++] = i + 1; }
    if (z > 0 && !seen[i - W] && !B[i - W]) { seen[i - W] = 1; queue[tail++] = i - W; }
    if (z < H - 1 && !seen[i + W] && !B[i + W]) { seen[i + W] = 1; queue[tail++] = i + W; } }
  const cand: Vec3[] = [];
  for (let z = 2; z < H - 2; z += 4) for (let x = 2; x < W - 2; x += 4) {
    let free = true; for (let dz = -2; dz <= 2 && free; dz++) for (let dx = -2; dx <= 2; dx++) if (!seen[(z + dz) * W + x + dx]) { free = false; break; }
    if (free) cand.push([(x + 0.5) * c, 0, (z + 0.5) * c]);
  }
  const out: Vec3[] = []; if (!cand.length) return out;
  const d2 = (p: Vec3, q: Vec3) => (p[0] - q[0]) * (p[0] - q[0]) + (p[2] - q[2]) * (p[2] - q[2]);
  let first = 0; for (let i = 1; i < cand.length; i++) if (d2(cand[i], me) < d2(cand[first], me)) first = i;
  out.push(cand[first]);
  while (out.length < n && out.length < cand.length) {
    let best = -1, far = -1; for (let i = 0; i < cand.length; i++) { let near = 1e30; for (const o of out) near = Math.min(near, d2(cand[i], o)); if (near > far) { far = near; best = i; } }
    if (far <= 0) break; out.push(cand[best]);
  }
  return out;
}
/** the points round a station: rings of ten, each turned a little from the last, on free floor, at chest and head height by turns */
function pointsOf(g: Game, s: Vec3, n: number): Vec3[] {
  const out: Vec3[] = [], nav = g.col.nav;
  for (const [ri, r] of [1, 2, 3.5, 5, 7, 9].entries()) for (let k = 0; k < 10; k++) {
    const a = (k + ri * 0.37) * Math.PI / 5, x = s[0] + Math.sin(a) * r, z = s[2] + Math.cos(a) * r;
    if (x < 0.5 || z < 0.5 || x > nav.W * nav.cell - 0.5 || z > nav.H * nav.cell - 0.5 || nav.isBlocked(x, z)) continue;
    out.push([x, out.length % 2 ? 1.6 : 1.2, z]); if (out.length >= n) return out;
  }
  return out;
}
/** the sight lines at a station, at eye height with no wall across them: first the ones straight through where the canister stands, then pairs of the
 *  station's points dealt by a small die of the plan's own */
function linesOf(g: Game, s: Vec3, pts: Vec3[], n: number): [Vec3, Vec3][] {
  const out: [Vec3, Vec3][] = [], eye = 1.5, clear = (a: Vec3, b: Vec3) => !g.col.segmentBlocked(a, b);
  for (let k = 0; k < 4 && out.length < n; k++) for (const r of [2, 4]) {
    const t = k * Math.PI / 4, a: Vec3 = [s[0] + Math.sin(t) * r, eye, s[2] + Math.cos(t) * r], b: Vec3 = [s[0] - Math.sin(t) * r, eye, s[2] - Math.cos(t) * r];
    if (clear(a, b)) out.push([a, b]);
  }
  let die = 0x2545f491 ^ Math.round(s[0] * 64) ^ (Math.round(s[2] * 64) << 12); const roll = (m: number) => { die = Math.imul(die, 1664525) + 1013904223 | 0; return (die >>> 8) % m; };
  for (let tries = 0; tries < 40 * n && out.length < n && pts.length > 1; tries++) {
    const i = roll(pts.length), j = roll(pts.length); if (i === j) continue;
    const a: Vec3 = [pts[i][0], eye, pts[i][2]], b: Vec3 = [pts[j][0], eye, pts[j][2]], L = Math.hypot(a[0] - b[0], a[2] - b[2]);
    if (L >= 1.5 && L <= 12 && clear(a, b)) out.push([a, b]);
  }
  return out;
}

interface Ask { key: string; sample: Sample; line: boolean; frames: number; asked: boolean }
/** ONE WALK OF A PLAN. `frame()` once after each update of the game; `done` when the report is whole. */
export class ModelErr {
  /** the smoke as each model has it, fed the plan's own canisters (the game's own stays as it is, whatever SENSE says) */
  blobs = new CpuSmoke(); grid = new CpuSmoke();
  /** the light model's own field (sense.ts SENSE.bounce), stepped from the first tick of the walk and never fed smoke: the light is read before the canister */
  lit: LightField | null = null;
  report: Report; done = false;
  private stations: Vec3[]; private at = -1; private phase: 'settle' | 'gap' | 'smoke' | 'end' = 'settle'; private left = 0; private age = 0; private ageAt = 0;
  private was: { unseen: boolean; god: boolean; ai: boolean };
  private pts: Vec3[] = []; private lines: [Vec3, Vec3][] = []; private asks: Ask[] = []; private seen: number; private owed = 0; private keyN = 0;
  constructor(readonly g: Game, readonly plan: Plan, planName: string, scene: string, readonly picture: boolean) {
    this.grid.grid = SmokeGrid.make(); this.was = { unseen: g.playerInvisible, god: g.godMode, ai: g.aiEnabled };
    // (the light model's field as it stands once it has swept the plan and come to rest: two sweeps' worth of steps at once, so that the walk reads its
    // light when it always did; a game's own field has been stepping since the level loaded)
    { const own = new CpuSmoke(); own.grid = SmokeGrid.make(); if (own.grid) this.lit = own as LightField; }
    if (this.lit) for (let n = 0; n < 2 * SENSE.glow.sweep; n += 4) this.lit.grid.step(g.col, 4 * GRID.step, col => steadyLamps(g, col));
    this.stations = stationsOf(g, plan.stations); this.seen = g.time;
    this.report = { v: 1, plan: planName, scene, world: '', stations: this.stations, picture, meter: { slope: g.tune.visSlope, floor: g.tune.visFloor, litAt: LIGHTS.litAt }, unanswered: 0, samples: [] };
    if (!this.stations.length) { this.done = true; this.release(); this.report.world = worldOf(this.report, g.col.boxes.length); } else this.next();
  }
  get station(): number { return this.at; }
  /** the walk is over: its two grids give back the blocks they stood in (sense.ts SmokeGrid.free: each held one of the module's slots) */
  private release(): void { this.grid.grid?.free(); this.grid.grid = null; if (this.lit) { this.lit.grid.free(); this.lit = null; } }
  private hold(): void { const g = this.g; g.playerInvisible = true; g.godMode = true; g.aiEnabled = false; }
  private next(): void {
    this.at++; if (this.at >= this.stations.length) { this.phase = 'end'; return; }
    const g = this.g, s = this.stations[this.at];
    g.smoke.clearAll?.(); this.blobs.clear(); this.grid.clear();
    g.teleportPlayer([s[0], 0, s[2]]);
    this.pts = pointsOf(g, s, this.plan.light); this.lines = linesOf(g, s, this.pts, this.plan.lines);
    this.phase = 'settle'; this.left = Math.round(this.plan.settle / TICK);
  }
  private ask(sample: Sample, line: boolean): void { this.report.samples.push(sample); if (this.picture) this.asks.push({ key: `modelerr:${this.keyN++}`, sample, line, frames: 0, asked: false }); }
  /** a sixtieth of the plan */
  private tick(): void {
    const g = this.g, me = g.player.char.id, skip = (o: number) => o === me;
    this.lit?.grid.step(g.col, TICK, col => steadyLamps(g, col));
    if (this.phase === 'settle') {
      if (this.left-- > 0) return;
      for (const p of this.pts) { const m: Record<string, number | null> = {}; for (const M of MODELS) if (M.light) m[M.name] = M.light(this, p, skip); this.ask({ k: 'light', s: this.at, p, pic: null, m }, false); }
      this.phase = 'gap'; this.left = GAP; return;   // (the picture reads its light before there is any smoke in it: the canister waits half a second)
    }
    if (this.phase === 'gap') {
      if (this.left-- > 0) return;
      const s = this.stations[this.at], P = smokeParams(g) ?? { canisterDuration: 32, canisterDensity: 95, canisterRadius: 0.2, canisterTemp: 1.6, canisterSpeed: 1.9 };
      const e: SmokeEmitter = { pos: [s[0], 0.1, s[2]], dir: [0, 1, 0], speed: P.canisterSpeed, radius: P.canisterRadius, density: P.canisterDensity, temperature: P.canisterTemp, ttl: P.canisterDuration, age: 0, kind: 'canister' };
      g.smoke.emit(e); this.blobs.emit(e); this.grid.emit(e);
      this.phase = 'smoke'; this.age = 0; this.ageAt = 0; return;
    }
    if (this.phase !== 'smoke') return;
    this.blobs.step(TICK); this.grid.step(TICK); this.grid.grid?.step(g.col, TICK); this.age++;
    const t = this.plan.ages[this.ageAt];
    if (this.age < Math.round(t / TICK)) return;
    for (const [a, b] of this.lines) { const m: Record<string, number | null> = {}; for (const M of MODELS) if (M.smoke) m[M.name] = M.smoke(this, a, b); this.ask({ k: 'smoke', s: this.at, a, b, t, pic: null, m }, true); }
    if (++this.ageAt >= this.plan.ages.length) this.next();
  }
  /** once a frame of the game, after its update: the plan moves by the game's own seconds, and the picture is asked what is owed and read what has come */
  frame(): void {
    if (this.done) return;
    const g = this.g; this.hold();
    this.owed += g.time - this.seen; this.seen = g.time;
    { let n = 0; while (this.owed >= TICK - 1e-9 && n++ < 6 && this.phase !== 'end') { this.owed -= TICK; this.tick(); } if (this.owed > 6 * TICK) this.owed = 6 * TICK; }
    // the picture: what has been answered is read, and as many of the rest are asked as the probe has places for after the game's own (eight are left to it)
    const P = g.probe as Game['probe'] & { room?: number }; let room = Math.max(0, (P.room ?? 64) - 8);
    for (let i = this.asks.length - 1; i >= 0; i--) { const q = this.asks[i]; if (!q.asked) continue;
      const r = P.get(q.key); q.frames++;
      if (r !== undefined) { q.sample.pic = r[0]; P.results.delete(q.key); this.asks.splice(i, 1); }
      else if (q.frames > 120) { this.report.unanswered++; this.asks.splice(i, 1); } }
    for (const q of this.asks) { if (room <= 0) break; if (q.asked && q.frames % 4) continue;   // (an ask the probe let go of for a higher rank is asked again every fourth frame until it is answered)
      const s = q.sample; if (q.line) P.querySegment(q.key, s.a!, s.b!, 3); else P.query(q.key, s.p!, g.player.char.id, 3); q.asked = true; room--; }
    if (this.at >= this.stations.length && !this.asks.length) { this.done = true; this.release(); this.report.world = worldOf(this.report, g.col.boxes.length); g.playerInvisible = this.was.unseen; g.godMode = this.was.god; g.aiEnabled = this.was.ai; }   // (the game is handed back as it was found)
  }
}

// ---- the page
let run: ModelErr | null = null, card: HTMLElement | null = null, said = -2;
const KIND: SavedKind = 'modelerr';
/** ONCE, when the page has its game: `plan` is the word's value (`?modelerr=quick`), `scene` what the page was opened on, `where` the build and the adapter */
let boot: { game: Game; plan: string; scene: string; where: () => Record<string, string> } | null = null;
export function modelErrBoot(game: Game, planWord: string | null, scene: string, where: () => Record<string, string>): void {
  MODELERR.on = true; boot = { game, plan: planWord && PLANS[planWord] ? planWord : 'full', scene, where };
}
/** the report as lines a person reads */
export function reportText(R: Report): string[] {
  const v = judge(R), out = [`The game's own senses against the picture · ${R.scene || 'the plain page'} · plan ${R.plan} · ${R.stations.length} stations · ${R.samples.length} samples${R.unanswered ? ` · ${R.unanswered} never answered` : ''}`, ...(R.picture ? v.lines : [])];
  if (!R.picture) { const two = spreadOf(R, 'blobs').find(s => s.model === 'grid'); if (two && two.n) out.push(`grid against blobs: ${two.n} lines · off by ${two.p50.toFixed(3)} at the middle, ${two.p95.toFixed(3)} at the 95th · decides the other way on ${(two.flips * 100).toFixed(1)}%`); }
  out.push(!R.picture ? 'No picture was asked: nothing is judged.' : !v.judged ? 'No model has a budget row and enough samples: nothing is judged.' : v.ok ? 'Every judged model is inside its budget.' : 'A MODEL IS PAST ITS BUDGET.');
  return out;
}
/** ONCE A PAGE FRAME, after the game's step: a return on its first line without the word */
export function modelErrFrame(): void {
  if (!MODELERR.on || !boot) return;
  const g = boot.game;
  if (!run) { if (!(g.time > 0)) return; run = new ModelErr(g, PLANS[boot.plan], boot.plan, boot.scene, true); (window as unknown as { __modelerrRun?: ModelErr }).__modelerrRun = run; }
  if (run.done) { if (!card) finish(run.report); return; }
  run.frame();
  if (run.station !== said && run.station < run.report.stations.length) { said = run.station; g.msg(`Measuring the senses against the picture: spot ${said + 1} of ${run.report.stations.length}. Hands off until it says it's done.`); }
}
function finish(R: Report): void {
  try { R.where = boot!.where(); } catch { /* said without it */ }
  const text = reportText(R), g = boot!.game; (window as unknown as { __modelerr?: Report }).__modelerr = R;
  let went = 'It was not sent anywhere: copy it from window.__modelerr.';
  try { const w = saveFile(KIND, `modelerr-${R.scene.replace(/[^a-z0-9]+/gi, '-')}-${R.plan}.json`, new Blob([JSON.stringify(R)], { type: 'application/json' }), { meta: { scene: R.scene, plan: R.plan, world: R.world } });
    if (w.local) went = 'The report was downloaded.'; else if (w.went) went = 'The report is being sent to the developer.'; } catch { /* the line above stands */ }
  card = document.createElement('pre');
  card.style.cssText = 'position:fixed;left:12px;top:12px;z-index:9999;max-width:calc(100vw - 48px);white-space:pre-wrap;margin:0;padding:12px 14px;background:rgba(10,12,14,0.92);color:#d8dee4;font:12px/1.5 ui-monospace,monospace;border:1px solid #39424c;border-radius:6px';
  card.textContent = [...text, went].join('\n'); document.body.appendChild(card);
  g.msg('The measurement is done.'); console.info(text.join('\n'));
}
