// THE WEATHER'S DUST IN THE ROOMS — where a picked weather's dust / fog stands under the level's roofs, and how it gets there.
//
// Out of doors the air carries the weather's whole density. Under a roof it carried a fixed share of it (the preset's `leak`), the same in the cupboard at the
// back as on the threshold of an open door — so the picture had a line at every wall. Here the share is a FIELD over the plan (scene/airGrid.ts: one number a
// half-metre cell, 1 = the open air's density): the open air is held at 1, a room's air is carried toward its neighbours' through every OPENING between two cells
// (a doorway with its leaf swung clear, a window without its glass, a hole blown in a wall) and settles back toward the leak wherever nothing feeds it:
//
//     ds∕dt = D · Σ open neighbours (s_n − s) ∕ h²  −  k · (s − leak)                    D = AIRDUST.diffuse, k = AIRDUST.settle
//
// A shut room stays at the leak EXACTLY (no neighbour differs, nothing to settle). Open its door and the dust comes in over a few seconds, thick on the
// threshold and thinning over √(D∕k) ≈ 3.5 m; shut it and the room clears again over 1∕k ≈ 8 s. Blow the wall out and it comes in across the whole breach.
//
// ONE FIELD, two readers: the picture (BoxWorld.setAirGrid → common.wgsl wxIndoor → the volumetric pass's march and the lamps' haze) and the men's eyes
// (weather.ts airTrans: the fog's optical depth along a sight line × the mean share along it). The second is a rule of the game, so it sits behind a switch of
// this module — AIRDUST.on: ON in the page (host/profile.ts pageSwitches; ?flatfog = off, with the picture's flat veil), held OFF on the frozen QA stand
// (host/standup.ts; TPS_AIRDUST=1), carried by a recording (knobs.ts). Off = airTrans as it was, and the grid holds the leak under every roof.
//
// WHAT AN OPENING IS. Between two neighbouring cells the way is open when a straight line between their centres is clear of every SOLID static — walls, intact
// glass, tall furniture — at ankle, chest or head-of-door height, asked both ways (StaticCollision.solidBlocked with the leaves taken out: a cell whose centre is
// inside a wall is shut off on every side) — and no door LEAF stands across it (the leaves' own rectangles on the plan, re-read whenever one has moved).
// The statics' half is asked of every edge once, when the field is seeded; after that only the SHUT edges are asked again, a few each step, the whole list
// every AIRDUST.recheckSecs — which is how a wall that breaks or a pane that falls out is noticed, without this file knowing who breaks walls.
// KNOWN: an opening is whole 0.5 m edges, so a 0.9 m doorway lets in one cell's width or two by where it sits on the lattice (no part-open edge).
//
// THE RULE IS THE MODULE'S (rust/sim/src/air.rs): the field, the openings' tables and the clocks are a part of the weather's table in the game's block, and a rollback carries
// them with it. What stays here is the caller and the picture's copy: `send` hands the renderer the field when it has moved enough.
//
// DETERMINISM. No dice, no clock but the game's dt: fixed steps of AIRDUST.step off an accumulator. A checkpoint carries the field, the statics' half of the
// openings, the re-check's cursor and the accumulator (snapshot / restore below); the leaves' half is re-read from the doors as the file put them back.
// restartEncounter → resetLevelState seeds it afresh (reset()).
import type { Game } from './game';
import { WORLD } from '../scene/world';
import { AIR_CELL, AIR_W, AIR_H, AIR_ROWS } from '../scene/airGrid';
import { sim, u8, u32 } from '../sim/sim';
import { WX, WX_KNOBS, WX_WATCH, wxAt, wxCap, wxGet, wxOut, wxSet } from '../sim/weatherhome';

export const AIRDUST = {
  /** the men's eyes read the field (and the dust moves through openings). The module's own default is the frozen stand's; the page raises it (host/profile.ts) */
  on: false,
  /** how fast the dust spreads through open air indoors (m²/s) */
  diffuse: 1.5,
  /** how fast a room's air settles back to the weather's leak (1/s) */
  settle: 0.12,
  /** the fixed step (s): D · step ∕ h² must stay under ¼ (0.2 at these numbers) */
  step: 1 / 30,
  /** every shut edge is asked again within this long (s) */
  recheckSecs: 3,
};

const N = AIR_W * AIR_H;
/** the renderer's copy is sent again once the cells have moved this far (summed over the steps since it last was): the picture never stands further than this from the men's eyes */
const SEND = 2e-4;

export interface AirDustSnap { acc: number; cur: number; /** [cell, share, cell, share …] for the roofed cells off the leak */ s: number[]; /** the statics' half of the openings, a bit an edge of `edges`, as hex */ stat: string }

/** where the module says a step's tale stands: how far the field moved, the steps taken, the lines asked, the open ways the last step found */
const said = (): number[] => wxOut(4);
/** one word of a run of numbers, for a check */
function wordOf(d: ArrayLike<number>, n = d.length): number { let h = 2166136261; const f = new Float64Array(1), u = new Uint32Array(f.buffer); for (let i = 0; i < n; i++) { f[0] = d[i]; h = Math.imul(h ^ u[0], 16777619); h = Math.imul(h ^ u[1], 16777619); } return h >>> 0; }

export class AirDust {
  /** the field: the share of the open air's density at each cell (scene/airGrid.ts order). The module's numbers (rust/sim/src/air.rs, the weather's table in the block; a view made for the asker, never kept); `gpu` is the copy the renderer takes */
  get s(): Float64Array { const o = wxAt(WX.airS) >>> 3; return this.g.sim.d.subarray(o, o + N); }
  /** the men's eyes read the field this run (AIRDUST.on, a weather with fog in) */
  get live(): boolean { this.ensure(); return this.on; }
  /** the field stands: built now if it is owed (a reader in the module's own hands asks first: weather.ts airTrans) */
  ready(): void { this.ensure(); }
  private get on(): boolean { return wxGet(this.g.sim, WX.airOn) !== 0; }
  private get leak(): number { return wxGet(this.g.sim, WX.airLeak); }
  /** the roofed cells, and the edges with a roofed cell at either end (id = cell × 2 + dir, 0: to +x, 1: to +z): the module's lists, views made for the asker and never kept */
  private get cells(): Uint32Array { const o = this.table(0) >>> 2; return u32().subarray(o, o + wxGet(this.g.sim, WX.airCells)); }
  private get edges(): Uint32Array { const o = this.table(1) >>> 2; return u32().subarray(o, o + wxGet(this.g.sim, WX.airEdges)); }
  /** where a table of the slot's stands (rust/sim/src/air.rs Plan: made from the block's roofs and the leaves, never in the block) */
  private table(which: number): number { return sim().air_table(this.g.sim.bind(), this.g.sim.stamp, which); }
  private tableBytes(which: number, n: number): Uint8Array { const a = this.table(which); return u8().subarray(a, a + n); }
  private bytes(c: number, n: number): Uint8Array { const w = this.g.sim.w; return new Uint8Array(w.buffer, w.byteOffset + wxAt(c), n); }
  private dirty = true; private drift = 0;
  private gpu = new Float32Array(AIR_ROWS * 4);
  private seeded = false;
  stats = { steps: 0, probes: 0, sends: 0, roofed: 0, edges: 0, open: 0 };

  constructor(private g: Game) {
    if (wxCap(2) !== AIR_W || wxCap(3) !== AIR_H || AIR_CELL !== 0.5 || WORLD.min[0] !== 0 || WORLD.min[2] !== 0) throw new Error(`the dust's plan is ${AIR_W} × ${AIR_H}, and rust/sim/src/air.rs holds ${wxCap(2)} × ${wxCap(3)}`);
    this.s.fill(1);   // (a block's zeros are no air at all: every cell starts as the open air's. A game with no weather makes no call into the module: it is bound as its first tick or question opens)
    if (g.weather?.kind) this.ensure();
  }

  /** the share at a spot of the plan (the cell it stands in; off the grid = the open air): reads the module's field */
  shareAt(x: number, z: number): number {
    if (this.dirty) this.ensure();
    const ix = Math.floor((x - WORLD.min[0]) / AIR_CELL), iz = Math.floor((z - WORLD.min[2]) / AIR_CELL);
    return ix < 0 || iz < 0 || ix >= AIR_W || iz >= AIR_H ? 1 : this.g.sim.d[(wxAt(WX.airS) >>> 3) + ix + iz * AIR_W];
  }

  /** the roofs or the weather changed (weather.ts setKind), or the level was put back as authored (game.ts resetLevelState): the field from nothing — every roofed
   *  cell at the leak, every opening asked, then (the switch on) run on until it stands still: the level opens with the dust already in its open doorways. LAZY: the
   *  call only marks the field stale, the first reader builds it (a checkpoint's load asks three times — the level put back, the weather set, the file's slice — and pays once) */
  reseed() { this.dirty = true; }
  private ensure() { if (this.dirty) { this.dirty = false; this.build(); } }
  private tally(t: readonly number[]) { this.stats.steps += t[1]; this.stats.probes += t[2]; if (t[1] > 0) this.stats.open = t[3]; }   // (the open ways the last step found: a tick with no step leaves them)
  private build() {
    const g = this.g, W = g.weather, f = W?.fog ?? null, has = !!W?.kind && !!f, x = sim(), st = g.sim.bind(), on = has && AIRDUST.on;
    x.air_begin(st, g.sim.stamp, on ? 1 : 0); this.drift = 0;
    if (!has) { if (this.seeded) { this.seeded = false; x.air_clear(st, g.sim.stamp); } return; }
    this.seeded = true; W.roofsNow(); x.air_seed(st, g.sim.stamp, f!.leak);
    this.stats.roofed = wxGet(g.sim, WX.airCells); this.stats.edges = wxGet(g.sim, WX.airEdges);
    if (on) {
      WX_KNOBS.here();
      g.col.rule(() => {
        const M = g.col.ruleOpen();
        x.air_settle(g.sim.bind(), g.sim.stamp, M.slot, M.ruleSet, M.rulePanes);
        const t = said(); this.tally(t);
        if (WX_WATCH.on) WX_WATCH.saidMany('air.settle', this.tale(t), []);
      });
    }
    this.send();
  }
  /** what a check holds a step of the field to: its tale's lines and open ways, and the field and its clocks as they stand */
  private tale(t: readonly number[]): number[] { const st = this.g.sim; return [t[0], t[2], t[3], wxGet(st, WX.airAcc), wxGet(st, WX.airCur), wordOf(this.s), wordOf(this.bytes(WX.airStat, N * 2)), wordOf(this.tableBytes(3, N * 2))]; }
  /** the level as authored again (game.ts resetLevelState, after the doors and the walls are back) */
  reset() { this.reseed(); }

  /** once a frame, after the doors and whatever breaks have moved (game.ts): through the openings as they stand (rust/sim/src/air.rs update) */
  update(dt: number) {
    this.ensure(); if (!this.on) return;
    const g = this.g; WX_KNOBS.here();
    g.col.rule(() => {
      const M = g.col.ruleOpen();
      sim().air_update(g.sim.bind(), g.sim.stamp, M.slot, M.ruleSet, M.rulePanes, dt);
      const t = said(); this.drift += t[0]; this.tally(t);
      if (WX_WATCH.on) WX_WATCH.saidMany('air.update', this.tale(t), [dt]);
    });
    if (this.drift > SEND) this.send();
  }

  /** the field as it stands, written again (host/putback.ts: a save put back in memory took the field back, not the GPU's copy of it); no book of this module moves */
  resend() { if (!this.on) return; const s = this.s, o = this.gpu; for (let i = 0; i < N; i++) o[i] = s[i]; this.g.engine.world.setAirGrid(o); }
  private send() {
    const s = this.s, o = this.gpu; for (let i = 0; i < N; i++) o[i] = s[i];
    this.g.engine.world.setAirGrid(o); this.stats.sends++; this.drift = 0;
  }

  // ---- checkpoints (savestate.ts)
  snapshot(): AirDustSnap | null {
    if (!this.live) return null;
    const out: number[] = [], C = this.cells, E = this.edges, s = this.s, leak = this.leak, stat = this.bytes(WX.airStat, N * 2);
    for (let q = 0; q < C.length; q++) { const i = C[q]; if (s[i] !== leak) out.push(i, s[i]); }
    let hex = ''; for (let k = 0; k < E.length; k += 4) { let nib = 0; for (let b = 0; b < 4 && k + b < E.length; b++) if (stat[E[k + b]]) nib |= 1 << b; hex += nib.toString(16); }
    return { acc: wxGet(this.g.sim, WX.airAcc), cur: wxGet(this.g.sim, WX.airCur), s: out, stat: hex };
  }
  /** the file's field over the one just seeded (savestate.ts loadState: after the weather, the walls, the doors and the glass are the file's). A file without the
   *  slice, malformed, or of another plan: the seeded field stands. An opening is the file's only where the level as loaded still has it (a pane that was shot out
   *  is whole again after a load: its edge is shut, whatever the file says). */
  restore(sn: AirDustSnap | null | undefined) {
    this.dirty = true; this.ensure();
    if (!sn || !this.on) return;
    const g = this.g, E = this.edges; if (typeof sn.stat !== 'string' || sn.stat.length !== Math.ceil(E.length / 4) || !Array.isArray(sn.s)) return;
    const stat = this.bytes(WX.airStat, N * 2), C = this.cells, s = this.s, leak = this.leak, indoor = this.tableBytes(2, N);
    for (let k = 0; k < E.length; k++) stat[E[k]] &= (parseInt(sn.stat[k >> 2], 16) >> (k & 3)) & 1;
    for (let q = 0; q < C.length; q++) s[C[q]] = leak;
    for (let k = 0; k + 1 < sn.s.length; k += 2) { const i = sn.s[k] | 0; if (i >= 0 && i < N && indoor[i]) s[i] = +sn.s[k + 1]; }
    wxSet(g.sim, WX.airAcc, +sn.acc || 0); wxSet(g.sim, WX.airCur, E.length ? ((sn.cur | 0) % E.length + E.length) % E.length : 0);
    g.col.rule(() => { const M = g.col.ruleOpen(); sim().air_reread(g.sim.bind(), g.sim.stamp, M.slot, M.ruleSet); });
    this.send();
  }

  summary(): string {
    this.ensure(); if (!this.seeded) return 'no weather in the air';
    const s = this.s, C = this.cells; let mx = 0, sum = 0; for (const i of C) { sum += s[i]; if (s[i] > mx) mx = s[i]; }
    return `${this.on ? 'moving' : 'still (switch off)'} · ${C.length} roofed cells · mean share ${(sum / Math.max(1, C.length)).toFixed(3)} (leak ${this.leak.toFixed(3)}, max ${mx.toFixed(3)}) · ${this.stats.open} open ways`;
  }
}
