// THE BENCHMARK (ui/bench.ts; title ▸ Extras ▸ Benchmark, or ?bench on the address), on a page that draws a game it does not hold: a page a scene, each a real mission taken by the module's
// hands-off player (rust/game_run hands_off.rs).
import type { Engine } from '../engine';
import type { Gpu } from '../gpu/device';
import type { Input } from '../game/input';
import type { SmokeSystem } from '../smoke/smoke';
import { Bench, benchView, machineFacts, type BenchScene } from '../ui/bench';
import { benchBegin, benchLegOfPage } from '../ui/benchrun';
import * as boot from '../ui/boot';
import { shaderRowsOn } from '../ui/shaderrows';
import { missionsBusy } from '../ui/missions';
import { optionsOpen, type OptionsGame } from '../ui/options';
import type { Quality } from '../ui/quality';
import { BENCH_SCENES } from './benchScenes';
import type { DrawGame } from './drawgame';
import { leversKept } from './machine';
import type { Measure } from './measure';

export interface DrawBenchDeps { gpu: Gpu; engine: Engine; smoke: SmokeSystem; quality: Quality; input: Input; game: DrawGame; settings: OptionsGame; measure: Measure; say(words: string): void }
export function drawBench(d: DrawBenchDeps): { bench: Bench; begin: () => void; go: () => void } {
  const { game, input, quality, measure } = d, page = benchLegOfPage(), covered = (): boolean => missionsBusy() || optionsOpen(), hz = (): number => quality.displayHzPin || quality.displayHz;
  let held = false, god = false, away = false;
  document.addEventListener('visibilitychange', () => { if (document.hidden) away = true; });
  const off = (): void => { if (!held) return; held = false; game.handsOff(null); d.settings.god = god; input.lockExcept = null; };
  const scene: BenchScene | null = page ? {
    start: () => { if (!game.tally().clear) return 'the mission did not start';
      const S = page.scene; god = d.settings.god; held = true; d.settings.god = S.god;
      input.lockExcept = new Set(); input.keys.clear(); input.buttons = 0;   // (the real keyboard is out of the scene: a visitor's key would make it another fight)
      game.handsOff({ ...S.plan, seed: S.seed, cap: page.cap }); return null; },
    active: () => { const h = game.hands(); return !!h && !h.end; }, stop: off,
    heavy: () => { const h = game.hands(); return !!h && h.t >= 12 && h.t - h.shotAt < 0.4; },   // (rounds in the air, a dozen seconds in: the stack is through its first door by then)
    said: () => { const h = game.hands(); return h ? `${h.end === 'taken' ? 'taken in' : h.end === 'sam' ? 'Sam down at' : 'fighting at'} ${Math.round(h.t)} s, ${h.up} of ${h.of} up` : ''; },
  } : null;
  const blocked = (): string | null => measure.sweep.active ? 'the sweep is running' : covered() ? 'a screen is open' : null;
  const bench = new Bench(d.engine, quality, measure.rec, { boot: () => boot.summary(), rows: shaderRowsOn, machine: () => machineFacts(d.gpu.adapterInfo, d.gpu.adapterFallback, hz()), hz, view: benchView(covered), smoke: d.smoke,
    blocked, covered: () => covered() ? 'a screen came up over it' : null, scene, away: () => { const a = away; away = false; return a; },
    sweep: { start: o => measure.sweep.start(o), active: () => measure.sweep.active, flipped: () => measure.sweep.flipped, progress: () => measure.sweep.progress, stop: why => measure.sweep.stop(why), settled: () => measure.sweep.settled } });
  measure.bench = bench;
  (window as unknown as { __bench: unknown }).__bench = Object.defineProperties(bench, { hands: { get: () => game.hands() }, sceneDefs: { value: BENCH_SCENES }, begin: { value: (o?: Parameters<typeof benchBegin>[2]) => benchBegin(true, location, o) } });
  const secsOf = (b: string | null) => { const s = parseFloat(b ?? ''); return isFinite(s) && s > 0 ? { measureMs: Math.min(60, s) * 1000, settleMs: Math.min(2000, Math.max(300, s * 500)) } : {}; };
  return {
    bench,
    /** asked for on the title: a page load of the first scene */
    begin: () => { const no = bench.active ? 'it is running' : measure.rec.recording ? 'a recording is running' : blocked() ?? benchBegin(true); if (no) d.say(`benchmark not now: ${no}`); },
    /** a scene's page of a run under way: its part starts by itself. A display frame of it: a scene's hands are let go when its run is over */
    go: () => { if (page) { if (page.leg.index === 0 && leversKept()) page.leg.notes.push('this machine keeps difficulty levers of its own: its fights are not the set\'s as dealt');
      const said = bench.start({ ...secsOf(page.secs), sceneCapMs: page.cap * 1250, heavyCapMs: Math.min(45, page.cap * 0.6) * 1000, ...page.opts }, page.leg); if (!bench.active) { page.leg.over(); d.say(`benchmark ${said}`); } } measure.after = () => { if (held && !bench.active) off(); }; },
  };
}
