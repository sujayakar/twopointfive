// THE MEASURING TOOLS, on a page that draws a game it does not hold: F9 records frames (ui/perfrec.ts), F8 prices the layers one at a time over a held world (Shift+F8: left running;
// ui/perfsweep.ts), F6 captures a frame (host/capture.ts). What they read of the game is its tally (DrawGame.tally).
import type { Engine } from '../engine';
import type { FollowCamera } from './camera';
import type { Gpu } from '../gpu/device';
import type { Input } from './input';
import type { SmokeSystem } from '../smoke/smoke';
import type { Quality } from '../ui/quality';
import type { DrawGame } from './drawgame';
import { PerfRecorder, type MeasuredGame } from '../ui/perfrec';
import { PerfSweep } from '../ui/perfsweep';
import type { Bench } from '../ui/bench';
import { FRAMECAP, type FrameCap } from '../render/framecap';
import { pageSay } from '../view/pagesaid';
import { Fog, Section } from '../draw/sections';
import { FOG_HELD } from '../draw/fog';
import { typedInto } from './keymap';
import { captureKey, frameCapture } from './capture';

export interface MeasureDeps { gpu: Gpu; engine: Engine; smoke: SmokeSystem; quality: Quality; cam: FollowCamera; input: Input; canvas: HTMLCanvasElement; game: DrawGame; marks(): number; /** why not now */ blocked(benchSweep: boolean): string | null; say(words: string): void }

/** the game as a take reads it */
function measured(game: DrawGame, engine: Engine, smoke: SmokeSystem, marks: () => number): MeasuredGame {
  const D = game.drawn;
  return {
    get guards() { return game.tally().men.map(([state, lit]) => ({ state, flashlight: { enabled: lit !== 0 } })); },
    get weather() { const W = engine.weather; return { rainRate: W.rain, strokeT: 0 }; }, smoke, decals: { get count() { return marks(); } },
    get qrf() { return { state: game.tally().qrf }; }, get escalation() { return game.tally().escalation; },
    get saves() { const t = game.tally(); return { stats: { autosaves: t.autosaves, manual: t.manual, loads: t.loads } }; },
    get time() { return game.time; },
    get turned() { return game.turnedNow; },
    fogOfWar: { get on() { return D.n[Section.fog] > 0 && D.u[D.at[Section.fog] + Fog.on] !== 0; }, knobs: { get edge() { return FOG_HELD.edge ?? D.f[D.at[Section.fog] + Fog.edge]; }, set edge(v: number) { FOG_HELD.edge = v === D.f[D.at[Section.fog] + Fog.edge] ? null : v; } } },
  };
}

export class Measure {
  /** the benchmark, where the page has one, and what follows each frame of it */
  bench: Bench | null = null; after: (() => void) | null = null;
  readonly rec: PerfRecorder; readonly sweep: PerfSweep; readonly cap: FrameCap;
  constructor(d: MeasureDeps) {
    const g = measured(d.game, d.engine, d.smoke, d.marks), note = (s: string): void => { d.say(s); pageSay(s); };
    this.rec = new PerfRecorder(d.engine, g, d.cam, d.quality, note);
    this.sweep = new PerfSweep(d.engine, g, d.quality, d.smoke, this.rec, note, () => (this.bench?.active && !this.bench.sweeping ? 'the benchmark is running' : d.blocked(!!this.bench?.sweeping)));
    this.cap = frameCapture(d.gpu, d.engine, d.quality, note);
    captureKey(shift => { if (!this.bench?.active) this.cap.start({ whole: !shift, at: [d.input.mouseX / Math.max(1, d.canvas.clientWidth), d.input.mouseY / Math.max(1, d.canvas.clientHeight)] }); });
    window.addEventListener('keydown', e => { if (e.repeat || e.metaKey || e.ctrlKey || e.altKey || typedInto(e.target) || this.bench?.active) return; if (e.code === 'F8') { e.preventDefault(); this.sweep.toggle(e.shiftKey); } else if (e.code === 'F9') { e.preventDefault(); this.rec.toggle(); } });
    Object.assign(window, { __perfrec: this.rec, __perfsweep: this.sweep });
  }
  /** the world holds still for a take */
  get freezing(): boolean { return this.sweep.freezing || !!this.bench?.freezing; }
  /** a take is on: the picture is held, no ladder is fed */
  get active(): boolean { return this.sweep.active || !!this.bench?.active; }
  /** the GPU's time is read frame by frame */
  get reads(): boolean { return this.active || this.rec.recording; }
  /** auto quality may measure the machine */
  calibrates(engine: Engine): boolean { const b = this.bench; return !b || (!b.timing && !(b.active && engine.standsIn)); }
  /** a display frame begins */
  begins(): void { this.rec.cpuFrameStart = performance.now(); }
  /** … its halves are timed */
  timed(update: number, render: number): void { this.rec.cpuUpdate = update; this.rec.cpuRender = render; }
  /** … and it has been drawn: what the tools themselves do of it */
  drawn(dt: number, fps: number): void {
    if (FRAMECAP.on) this.cap.frame(performance.now() - this.rec.cpuRender, this.rec.cpuRender);
    this.rec.frame(dt, fps); this.sweep.tick(); this.bench?.tick(dt); this.after?.();
  }
}
