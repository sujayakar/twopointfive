// Quality presets + adaptive resolution + an 'auto' mode that picks the preset for this machine: keeps the demo above ~30 fps on
// smaller GPUs without hand tuning, and climbs back up on big ones.
import { Engine } from '../engine';
import { defaultRcConfig } from '../render/rc';
import type { SmokeSystem } from '../smoke/smoke';

export type QualityName = 'ultra' | 'high' | 'medium' | 'low';
export const QUALITY_NAMES: QualityName[] = ['ultra', 'high', 'medium', 'low'];
const AUTO_LADDER: QualityName[] = ['low', 'medium', 'high'];   // auto never picks ultra (it is 'high' with more headroom spent; opt in by hand)
const LS_MODE = 'twopointsix.quality', LS_AUTO = 'twopointsix.autoPreset', LS_SCALE = 'twopointsix.autoScale';
const RENDER_PASSES = ['composite', 'post', 'fxaa', 'gbuffer'];   // render-pass timestamps are not load (they include the drawable wait) — excluded from every total here
const SCALE_FLOOR = 0.35;                                     // adaptive / calibration never go below this internal-resolution scale
/** relative cost of the per-pixel work by preset at equal pixel count: the direct + penumbra passes (~half of it) scale with the shadow-ray budget */
const RAY_COST: Record<QualityName, number> = { ultra: 1, high: 0.85, medium: 0.69, low: 0.59 };   // (× the trade-offs each preset carries: high's three ≈ ×0.85, all five ≈ ×0.79 — measured 08-07)
const probesOf = (p: Preset) => p.rc.nx * p.rc.ny * p.rc.nz;

type Lossy = { checkerDirect: boolean; gatherThird: boolean; rcHalfRate: boolean; volQuarter: boolean; dimRays: boolean };
interface Preset { scale: number; maxH: number; volSteps: number; rc: { nx: number; ny: number; nz: number }; cascades: number; rbIters: number; ds: [number, number, number]; lossy: Lossy; }   // ds = direct-light shadow samples [strongest light, other leaders, adaptive stop] — measured: 8/4/3 → 1/1/2 halves the direct pass (7.2 → 3.8 ms at 1600×1000), so the cheaper presets spend fewer rays as well as fewer pixels
const PRESETS: Record<QualityName, Preset> = {
  // the cascade probe volume is only a radiance CACHE now (read at the final gather's hit points, one bounce from the
  // eye), so it can be coarse: 1 m probes look the same as 0.5 m ones through the gather and cost a third.
  // rbIters = red/black pairs of the smoke pressure solve (4 pairs ≈ the old 20 Jacobi sweeps, better converged)
  // trade-offs folded in after a side-by-side eyeball with all five on (08-07): ultra stays the untouched reference; high takes the three measured invisible at
  // rest (third-res gather −3.4 ms, checkerboard rays −0.9, dim-pixel rays) and keeps half-res beams + full-rate cascades; medium and low take all five
  ultra:  { scale: 1.0,  maxH: 1200, volSteps: 24, rc: { nx: 40, ny: 4, nz: 28 }, cascades: 4, rbIters: 5, ds: [8, 4, 3], lossy: { checkerDirect: false, gatherThird: false, rcHalfRate: false, volQuarter: false, dimRays: false } },
  high:   { scale: 1.0,  maxH: 1000, volSteps: 20, rc: { nx: 40, ny: 4, nz: 28 }, cascades: 4, rbIters: 4, ds: [8, 4, 3], lossy: { checkerDirect: true,  gatherThird: true,  rcHalfRate: false, volQuarter: false, dimRays: true } },
  medium: { scale: 0.8,  maxH: 900,  volSteps: 14, rc: { nx: 40, ny: 4, nz: 28 }, cascades: 4, rbIters: 3, ds: [6, 3, 3], lossy: { checkerDirect: true,  gatherThird: true,  rcHalfRate: true,  volQuarter: true,  dimRays: true } },
  low:    { scale: 0.65, maxH: 720,  volSteps: 10, rc: { nx: 32, ny: 3, nz: 22 }, cascades: 4, rbIters: 3, ds: [4, 2, 2], lossy: { checkerDirect: true,  gatherThird: true,  rcHalfRate: true,  volQuarter: true,  dimRays: true } },
};

const clamp01 = (v: number, lo: number, hi: number) => Math.max(lo, Math.min(hi, v));
/** document.hidden, unless an automation run says the pane is being pumped anyway (`window.__testVisible`): the headless Browser pane reports hidden
 *  even while it renders, which would keep calibration from ever running under test */
const pageHidden = () => document.hidden && !window.__testVisible;

export class Quality {
  current: QualityName = 'high';
  adaptive = true;
  /** 'auto': the preset itself moves with the measured frame rate (and the pick is remembered for the next visit) */
  auto = true;
  /** ceiling for the adaptive scaler = the preset's scale */
  private maxScale = 1.0;
  private acc = 0; private frames = 0; private slow = 0; private fast = 0; private cooldown = 2.5;
  private starved = 0; private cruising = 0;   // auto mode: consecutive 1 s windows where resolution scaling was not enough / where there was obvious headroom
  lastChange = '';
  /** boot calibration (auto only): a couple of seconds of GPU pass timings behind the start card → preset AND scale in one step, instead of crawling
   *  down 0.1 at a time while the player already suffers. `sample()` stays quiet until it has decided. */
  private calib: { frames: number; t: number; gpu: number[]; fixed: number[]; dts: number[]; done: boolean } = { frames: 0, t: 0, gpu: [], fixed: [], dts: [], done: false };
  get calibrating() { return this.auto && !this.calib.done; }

  constructor(private engine: Engine, private smoke: SmokeSystem) {
    // remembered choice: 'auto' (default) resumes from the preset auto settled on last time; a named preset is a manual pin
    let mode: string | null = null, remembered: string | null = null;
    try { mode = localStorage.getItem(LS_MODE); remembered = localStorage.getItem(LS_AUTO); } catch { /* private mode etc. */ }
    if (mode && (QUALITY_NAMES as string[]).includes(mode)) { this.auto = false; this.apply(mode as QualityName); }
    else {   // (a remembered 'ultra' is a hand pin, never an auto pick)
      this.auto = true; const start = remembered && (AUTO_LADDER as string[]).includes(remembered) ? remembered as QualityName : 'high'; this.apply(start);
      let sc: number | null = null; try { const v = parseFloat(localStorage.getItem(LS_SCALE) ?? ''); if (isFinite(v)) sc = v; } catch { /* */ }
      if (sc !== null && sc < this.maxScale - 1e-3) this.engine.settings.renderScale = clamp01(sc, SCALE_FLOOR, this.maxScale);   // resume at the scale this machine settled on last time; calibration refines it
      this.lastChange = `auto: starting at ${this.current}${sc !== null && sc < this.maxScale - 1e-3 ? ` ×${this.engine.settings.renderScale.toFixed(2)}` : ''} (calibrating)`;
    }
  }

  /** The label for menus: 'auto (high)' or the pinned preset. */
  get label() { return this.auto ? `auto (${this.current})` : this.current; }

  /** Pin a preset by hand (turns auto off and remembers the pin). */
  pin(name: QualityName) { this.auto = false; this.calib.done = true; this.apply(name); try { localStorage.setItem(LS_MODE, name); } catch { /* */ } }
  /** Back to auto: keep the current preset as the starting point and let sampling move it. */
  setAuto() { this.auto = true; this.starved = this.cruising = 0; if (!(AUTO_LADDER as string[]).includes(this.current)) this.apply('high'); this.lastChange = `auto (from ${this.current})`; try { localStorage.setItem(LS_MODE, 'auto'); localStorage.setItem(LS_AUTO, this.current); } catch { /* */ } }   // auto lives on the low…high ladder: from a pinned ultra it starts at high

  apply(name: QualityName) {
    const p = PRESETS[name]; const S = this.engine.settings; this.current = name;
    S.renderScale = p.scale; S.maxInternalHeight = p.maxH; S.volSteps = p.volSteps; this.maxScale = p.scale;
    S.directSamplesTop = p.ds[0]; S.directSamplesOther = p.ds[1]; S.directAdaptiveMin = p.ds[2];
    S.checkerDirect = p.lossy.checkerDirect; S.gatherThird = p.lossy.gatherThird; S.rcHalfRate = p.lossy.rcHalfRate; S.volQuarter = p.lossy.volQuarter; S.dimRays = p.lossy.dimRays;   // (the pause-menu rows still override these by hand, until the next preset is applied)
    const cur = this.engine.rc.cfg;
    if (cur.nx !== p.rc.nx || cur.ny !== p.rc.ny || cur.nz !== p.rc.nz || cur.cascades !== p.cascades) this.engine.rebuildRc({ ...defaultRcConfig, ...p.rc, cascades: p.cascades, interval0: cur.interval0 });
    this.smoke.params.rbIters = p.rbIters;
    this.lastChange = `preset ${name}`; this.cooldown = 3; this.slow = this.fast = 0;
    if (this.auto) { try { localStorage.setItem(LS_AUTO, name); } catch { /* */ } }
  }

  /** Boot calibration tick — call every presented frame (the start card renders the live scene behind it, which is the load that matters) until
   *  `calibrating` goes false. Uses GPU compute-pass timings when timestamp queries exist (vsync cannot hide headroom from those), else frame times. */
  calibrateTick(rawDt: number) {
    if (!this.calibrating || pageHidden() || rawDt >= 0.5) return;
    const c = this.calib; c.frames++;
    if (c.frames <= 24) return;                                   // pipelines warm up, first smoke/rc allocations, shader compiles
    c.t += rawDt; c.dts.push(rawDt);
    const tm = this.engine.timer;
    if (tm.enabled && tm.results.size) {
      let fixed = 0; for (const [k, v] of tm.results) if (/^rc |^probe$|^smoke$/.test(k)) fixed += v;   // per-probe / per-voxel work: does not shrink with resolution
      c.gpu.push(tm.total(RENDER_PASSES)); c.fixed.push(fixed);
    }
    if (c.dts.length >= 54 || c.t > 3) this.decide();
  }
  /** Pure part of the decision so it can be exercised with synthetic numbers: given the measured compute ms (total, fixed part) at preset `cur` and
   *  effective scale `effNow` on a canvas `cssH` CSS px tall at `dpr`, return the highest ladder preset (and scale) predicted to fit `budgetMs`. */
  plan(gpuMs: number, fixedMs: number, cur: QualityName, effNow: number, cssH: number, dpr: number, budgetMs = 24): { preset: QualityName; scale: number; predicted: number; note: string } {
    const pixelMs = Math.max(0.5, gpuMs - fixedMs); const P0 = PRESETS[cur];
    const effOf = (p: Preset, scale: number) => Math.min(scale, p.maxH / Math.max(1, cssH * dpr));
    const predict = (name: QualityName, scale: number) => { const p = PRESETS[name]; const e = effOf(p, scale); return fixedMs * (probesOf(p) / probesOf(P0)) * (p.rbIters / P0.rbIters > 1 ? 1 : 1) + pixelMs * (e * e) / Math.max(1e-4, effNow * effNow) * (RAY_COST[name] / RAY_COST[cur]); };
    for (let i = AUTO_LADDER.length - 1; i >= 0; i--) {          // high → medium → low at their own full scale
      const name = AUTO_LADDER[i]; const pr = predict(name, PRESETS[name].scale);
      if (pr <= budgetMs) return { preset: name, scale: PRESETS[name].scale, predicted: pr, note: 'fits' };
    }
    // nothing fits at full scale: 'low' with the resolution solved for the budget (down to the floor)
    const low = PRESETS.low; const fixedLow = fixedMs * (probesOf(low) / probesOf(P0)); const room = budgetMs - fixedLow;
    const perEff2 = pixelMs * (RAY_COST.low / RAY_COST[cur]) / Math.max(1e-4, effNow * effNow);   // ms per unit of eff²
    let eff = room > 0 ? Math.sqrt(room / perEff2) : 0; eff = Math.max(SCALE_FLOOR, Math.min(low.scale, eff));
    const cap = low.maxH / Math.max(1, cssH * dpr); const scale = Math.min(low.scale, Math.max(SCALE_FLOOR, cap < eff ? low.scale : eff));   // if the height cap already gives fewer pixels than needed, the nominal scale can stay up
    const pr = fixedLow + perEff2 * Math.min(eff, cap) ** 2;
    return { preset: 'low', scale: +scale.toFixed(2), predicted: pr, note: pr <= budgetMs ? 'low, reduced scale' : 'below target even at the floor' };
  }
  private decide() {
    const c = this.calib; c.done = true;
    const med = (a: number[]) => { const b = [...a].sort((x, y) => x - y); return b.length ? b[Math.floor(b.length / 2)] : NaN; };
    const S = this.engine.settings; const canvas = this.engine.gpu.canvas as HTMLCanvasElement; const dpr = window.devicePixelRatio || 1;
    let gpuMs = med(c.gpu), fixedMs = med(c.fixed); const dtMed = med(c.dts); const fps = 1 / dtMed;
    let source = 'gpu timers';
    if (!(gpuMs > 0)) {                                             // no timestamp queries: frame time is all we have — above ~50 fps it hides the headroom, so only act when slow
      if (fps > 50) { this.lastChange = `calibrated: ${fps.toFixed(0)} fps at ${this.current}, no GPU timers — leaving it to the runtime ladder`; return; }
      gpuMs = Math.max(4, 1000 * dtMed - 4); fixedMs = gpuMs * 0.3; source = 'frame time';
    }
    const r = this.plan(gpuMs, fixedMs, this.current, this.engine.effectiveScale || S.renderScale, canvas.clientHeight || 800, dpr);
    const from = `${this.current} ×${(this.engine.effectiveScale || S.renderScale).toFixed(2)}`;
    if (r.preset !== this.current) this.apply(r.preset);
    S.renderScale = Math.min(this.maxScale, r.scale); this.cooldown = 2.5; this.slow = this.fast = this.starved = this.cruising = 0;
    this.lastChange = `calibrated (${source}): ${gpuMs.toFixed(1)} ms compute at ${from} → ${r.preset} ×${S.renderScale.toFixed(2)} (predicted ${r.predicted.toFixed(1)} ms; ${r.note})`;
    try { localStorage.setItem(LS_AUTO, r.preset); localStorage.setItem(LS_SCALE, String(S.renderScale)); } catch { /* */ }
  }

  /** Feed real frame times (only for frames actually presented at display rate). */
  sample(dt: number) {
    if (!this.adaptive && !this.auto) return;
    if (this.calibrating) return;                                   // the boot calibration decides first
    this.acc += dt; this.frames++; this.cooldown -= dt;
    if (this.acc < 1.0) return;
    const avg = this.acc / this.frames; this.acc = 0; this.frames = 0;
    if (this.cooldown > 0) return;
    const S = this.engine.settings;
    // step from the scale actually in effect: on a big / HiDPI window the internal-height cap already undercuts renderScale, and
    // stepping the nominal value down from 1.0 would change nothing for several rounds
    const eff = Math.min(S.renderScale, this.engine.effectiveScale || S.renderScale);
    if (this.adaptive) {
      if (avg > 1 / 29) { this.slow++; this.fast = 0; } else if (avg < 1 / 45) { this.fast++; this.slow = 0; } else { this.slow = 0; this.fast = 0; }   // 'fast' = comfortably above the 30 fps target, so capped 48/50 Hz displays can climb back too
      if (this.slow >= 2 && eff > SCALE_FLOOR + 0.01) {   // proportional step: pixels ∝ scale², so aim straight at the scale that would give ~31 fps (at least 0.05, at most 0.2 down)
        const want = eff * Math.sqrt((1 / 31) / avg); S.renderScale = +Math.max(SCALE_FLOOR, Math.min(eff - 0.05, Math.max(eff - 0.2, want))).toFixed(2);
        this.slow = 0; this.cooldown = 1.2; this.lastChange = `adaptive ↓ scale ${S.renderScale.toFixed(2)} (${(1 / avg).toFixed(0)} fps)`;
      }
      else if (this.fast >= 4 && S.renderScale < this.maxScale - 1e-3 && eff >= S.renderScale - 1e-3) { S.renderScale = Math.min(this.maxScale, +(S.renderScale + 0.05).toFixed(2)); this.fast = 0; this.cooldown = 1.5; this.lastChange = `adaptive ↑ scale ${S.renderScale.toFixed(2)}`; }   // (no point raising the nominal scale while the height cap is what limits us)
      if (this.auto) { try { localStorage.setItem(LS_SCALE, String(S.renderScale)); } catch { /* */ } }
    }
    if (this.auto) {
      // the preset moves only when resolution scaling has run out of road: still slow with the scale already well down → a cheaper
      // preset; comfortably fast at the preset's full scale for a while → the next one up. Long windows + cooldowns so it settles.
      // judged on what the adaptive scaler DID (the nominal renderScale), not on the effective scale: on a HiDPI window the internal-height
      // cap undercuts the nominal scale permanently, which says nothing about load
      const i = AUTO_LADDER.indexOf(this.current as QualityName);
      const capLimited = eff < S.renderScale - 1e-3;                       // the internal-height cap, not the scaler, is what limits resolution right now
      const start = Math.min(this.maxScale, this.engine.capScale);          // what the window gets before adaptive takes anything
      const atFloor = eff <= SCALE_FLOOR + 0.02;                              // adaptive will not step from here (a cap already under it counts too)
      const cutDeep = S.renderScale <= start - 0.14;                          // a real cut has already been taken off — a cheaper preset (fewer rays, coarser cache) buys more than pixels now
      const floorHit = !this.adaptive || atFloor || cutDeep;
      if (avg > 1 / 28 && floorHit) this.starved++; else this.starved = 0;
      if (avg < 1 / 52 && (S.renderScale >= this.maxScale - 1e-3 || capLimited)) this.cruising++; else this.cruising = 0;   // fast with the scaler not holding anything back
      if (this.starved >= 2 && i > 0) { const keep = S.renderScale; this.apply(AUTO_LADDER[i - 1]); S.renderScale = Math.min(keep, S.renderScale); this.lastChange = `auto ↓ ${this.current} (${(1 / avg).toFixed(0)} fps)`; this.cooldown = 4; this.starved = 0; }   // keep the scale adaptive had reached: the cheaper preset must not start by drawing more pixels
      else if (this.cruising >= 6 && i >= 0 && i < AUTO_LADDER.length - 1) { this.apply(AUTO_LADDER[i + 1]); this.lastChange = `auto ↑ ${this.current} (${(1 / avg).toFixed(0)} fps)`; this.cooldown = 4; this.cruising = 0; }
    }
  }
}
