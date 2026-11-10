// THE READ-OUT, for a page that draws a game it does not hold (#hud; Options ▸ Show frame rate, `?hud`): the machine, the frame, the passes' milliseconds, and how much is drawn. Kept per machine.
import type { Engine } from '../engine';
import type { SmokeSystem } from '../smoke/smoke';
import { tierWords, type Quality } from './quality';
import { RENDER_PASSES } from '../gpu/timer';

const LS = 'twopointsix.readout', EVERY = 10, RENDERED = RENDER_PASSES;
export interface ReadoutSource { engine: Engine; smoke: SmokeSystem; quality: Quality; adapter: string; men(): number; told: { fps: number; tick: number; draw: number; update: number; render: number; hud: number } }
export class Readout {
  on = false;
  private n = 0; private gpu = 0; private read = -1; private el = document.getElementById('hud')!;
  constructor(private s: ReadoutSource, search: URLSearchParams) {
    try { this.on = localStorage.getItem(LS) === '1'; } catch { /* no store */ }
    if (search.has('hud')) this.on = true;
    (window as unknown as { __readout: unknown }).__readout = this;
  }
  set(on: boolean): void { this.on = on; try { localStorage.setItem(LS, on ? '1' : '0'); } catch { /* no store */ } if (!on) this.el.textContent = ''; }
  private words = ''; private until = 0;
  /** a line of a tool's, for a few seconds, whether the read-out is on or not */
  say(words: string): void { this.words = words; this.until = performance.now() + 6000; this.n = 0; if (!this.on) this.el.textContent = words; }
  /** a frame was drawn */
  frame(): void {
    const { engine: E, smoke, quality, told } = this.s, t = E.timer;
    const said = performance.now() < this.until ? this.words : ''; if (!said && this.words) { this.words = ''; if (!this.on) this.el.textContent = ''; }
    if (!this.on) return;
    if (t.resolved !== this.read) { const ms = t.unionMs(RENDERED); this.gpu += (ms - this.gpu) * (this.read < 0 ? 1 : 0.1); this.read = t.resolved; }
    if (this.n++ % EVERY) return;
    const row = (k: string, v: string): string => k.padEnd(11) + v, ms = (v: number): string => v.toFixed(2);
    const lines = [this.s.adapter, ...(said ? [said] : []),
      row('res', `${E.width}×${E.height} ×${E.settings.renderScale.toFixed(2)}${E.settings.taau ? ` → ${E.outWidth}×${E.outHeight} taau${E.sharpOn ? ' sharp' : ''}` : ''}`),
      row('quality', `${tierWords(quality.label)}${quality.adaptive ? ' +adapt' : ''}`), row('fps', told.fps.toFixed(0)), row('gpu ms', ms(this.gpu)),
      row('page ms', `${ms(told.update + told.render + told.hud)} · tick ${ms(told.tick)} · rows ${ms(told.draw)} · draw ${ms(told.render)} · hud ${ms(told.hud)}`)];
    for (const [k, v] of t.results) if (!RENDER_PASSES.includes(k) && t.ranThisFrame(k)) lines.push(row(`  ${k}`, ms(v)));
    lines.push(row('boxes', String(E.world.statics.length + E.world.dynamics.length)), row('men', String(this.s.men())), row('lights', String(E.lights.activeCount)), row('smoke', `${smoke.stats.pool}/${smoke.grid.cap} bricks · ${smoke.stats.stepped} stepped`), row('rc rays', `${(E.rc.stats.intervals / 1e6).toFixed(2)}M`), row('view', `exp ${E.settings.exposure.toFixed(2)}`));
    this.el.textContent = lines.join('\n');
  }
}
