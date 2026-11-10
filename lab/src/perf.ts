// WHERE THE TIME GOES, frame by frame: when each was asked for, what the CPU did in it, when the GPU was done, and what else happened. A graph, the worst frames
// with what was going on in them, and a report to copy.
export interface Frame { n: number; at: number; gap: number; cpu: number; from: number; sim: number; send: number; encode: number; ticks: number; gpu: number; done: number; w: number; h: number; what: string[] }
const NAMES: [keyof Frame, string][] = [['gap', 'between frames'], ['cpu', 'CPU in the frame'], ['sim', '  the world\'s ticks'], ['send', '  sending it'], ['encode', '  encoding'], ['done', 'asked to GPU done'], ['gpu', 'GPU, first pass to last']];
export class Perf {
  /** where the camera is and what the weather does, so that the same view can be stood elsewhere */ where = () => '';
  ring: Frame[] = []; private el = document.createElement('div'); private cv = document.createElement('canvas'); shown = false; private now: Frame | null = null; private events: string[] = []; private last = 0; private n = 0; private skipped = 0; hold = false; vsync = 16.7; private gaps: number[] = [];
  constructor(private gpuName: string, private pace: { mode: string; label(): string; next(): void }) { this.el.id = 'perf'; this.el.hidden = true; document.body.append(this.el); }
  /** something that may cost a frame */
  note(what: string) { this.events.push(what); }
  /** every refresh of the display, drawn or not */
  refresh(t: number) { const d = t - this.last; this.last = t; if (d > 2 && d < 60) { this.gaps.push(d); if (this.gaps.length > 120) this.gaps.shift(); if (this.gaps.length > 30) { const s = [...this.gaps].sort((a, b) => a - b); this.vsync = s[Math.floor(s.length * 0.2)]!; } } }
  skip() { this.skipped++; }
  begin(t: number, stamp: number) { const p = this.ring[this.ring.length - 1]; this.now = { n: ++this.n, at: stamp, from: t, gap: p ? stamp - p.at : 0, cpu: 0, sim: 0, send: 0, encode: 0, ticks: 0, gpu: 0, done: 0, w: 0, h: 0, what: [] }; return this.now; }
  end(q: GPUQueue) { const f = this.now; if (!f) return; f.cpu = performance.now() - f.from; f.what = this.events; this.events = []; const t0 = f.from; void q.onSubmittedWorkDone().then(() => { f.done = performance.now() - t0; }); if (!this.hold) { this.ring.push(f); if (this.ring.length > 1200) this.ring.shift(); } this.now = null; if (this.shown && f.n % 90 === 0 && !this.hold) { this.paint(); this.note('this panel drawn'); } }
  toggle() { this.shown = !this.shown; this.el.hidden = !this.shown; if (this.shown) this.build(); }
  private build() {
    this.el.innerHTML = '<header><b>Performance</b><span id="pf-say"></span><button data-a="pace"></button><button data-a="hold">Hold</button><button data-a="copy">Copy report</button><button data-a="close">Close</button></header>'; this.cv = document.createElement('canvas'); this.cv.width = 1200; this.cv.height = 220; this.el.append(this.cv); const d = document.createElement('div'); d.id = 'pf-rows'; this.el.append(d);
    this.el.onclick = e => { const b = (e.target as HTMLElement).closest('button'); if (!b) return; const a = b.dataset.a; setTimeout(() => { if (this.shown) this.paint(); }, 0); if (a === 'close') this.toggle(); else if (a === 'hold') { this.hold = !this.hold; b.textContent = this.hold ? 'Go on' : 'Hold'; } else if (a === 'pace') { this.pace.next(); this.paint(); } else if (a === 'copy') void (async () => { b.textContent = 'Reading'; const t = this.report() + '\n\n' + await this.more().catch(e => String(e)); void navigator.clipboard.writeText(t).then(() => { b.textContent = 'Copied'; setTimeout(() => { b.textContent = 'Copy report'; }, 1500); }, () => { const p = document.createElement('textarea'); p.value = t; this.el.append(p); p.select(); }); })(); b.blur(); };
  }
  private q(k: keyof Frame, p: number, r = this.recent()) { const v = r.map(f => f[k] as number).filter(x => x > 0).sort((a, b) => a - b); return v.length ? v[Math.min(v.length - 1, Math.floor(v.length * p))]! : 0; }
  recent(s = 10) { const t = this.ring[this.ring.length - 1]?.at ?? 0; return this.ring.filter(f => f.at > t - s * 1000 && !f.what.includes('this panel drawn')); }
  /** how even: the share of frames shown within a quarter refresh of the usual gap */
  even() { const r = this.recent(), m = this.q('gap', 0.5); return r.length ? r.filter(f => Math.abs(f.gap - m) < this.vsync * 0.35).length / r.length : 1; }
  paint() {
    const r = this.recent(), c = this.cv.getContext('2d')!, W = this.cv.width, H = this.cv.height, top = 50, y = (ms: number) => H - Math.min(ms, top) / top * H; c.fillStyle = '#0c0f13'; c.fillRect(0, 0, W, H);
    c.strokeStyle = 'rgba(255,255,255,.16)'; c.fillStyle = 'rgba(255,255,255,.55)'; c.font = '11px ui-monospace,Menlo,monospace'; for (const ms of [this.vsync, this.vsync * 2, this.vsync * 3, 33.3]) { c.beginPath(); c.moveTo(0, y(ms)); c.lineTo(W, y(ms)); c.stroke(); c.fillText(ms.toFixed(1), 4, y(ms) - 3); }
    const show = r.slice(-400), dx = W / 400; show.forEach((f, i) => { const x = i * dx; c.fillStyle = f.gap > this.vsync * 1.6 + this.q('gap', 0.5, show) - this.vsync ? '#e8121c' : '#5a6772'; c.fillRect(x, y(f.gap), Math.max(1, dx - 0.6), H - y(f.gap)); if (f.what.length) { c.fillStyle = '#ffd7a8'; c.fillRect(x, 0, dx, 6); } });
    for (const [k, col] of [['done', '#5fc9b0'], ['cpu', '#79b4f2'], ['gpu', '#b39df0']] as const) { c.strokeStyle = col; c.beginPath(); let on = false; show.forEach((f, i) => { const v = f[k]; if (v <= 0) return; if (on) c.lineTo(i * dx, y(v)); else c.moveTo(i * dx, y(v)); on = true; }); c.stroke(); }
    const n = (x: number) => x.toFixed(1).padStart(6), rows = NAMES.map(([k, name]) => `${name.padEnd(26)}${n(this.q(k, 0.5))}${n(this.q(k, 0.95))}${n(this.q(k, 0.99))}${n(this.q(k, 1))}`), bad = [...r].sort((a, b) => b.gap - a.gap).slice(0, 6).filter(f => f.gap > this.q('gap', 0.5) * 1.35), g = r.length > 1 ? (r.length - 1) / ((r[r.length - 1]!.at - r[0]!.at) / 1000) : 0;
    document.getElementById('pf-say')!.textContent = `${g.toFixed(1)} frames a second, ${(this.even() * 100).toFixed(0)} % even, display ${(1000 / this.vsync).toFixed(0)} Hz`; this.el.querySelector('[data-a=pace]')!.textContent = this.pace.label();
    document.getElementById('pf-rows')!.textContent = `ms, last 10 s              usual  19/20 99/100  worst\n${rows.join('\n')}\n\nWorst frames\n${bad.map(f => `${f.gap.toFixed(1).padStart(6)} ms  cpu ${f.cpu.toFixed(1)} (ticks ${f.ticks}: ${f.sim.toFixed(1)}, send ${f.send.toFixed(1)}, encode ${f.encode.toFixed(1)})  GPU done ${f.done.toFixed(1)}  ${f.what.join(', ')}`).join('\n') || '  none: all near the usual'}\n\n(drawn once in 90 frames, since drawing it is itself a hitch; those frames are left out)   bars: gap between frames   green: asked to GPU done   blue: CPU   violet: GPU   orange ticks: something happened`;
  }
  /** what else a report says */ more: () => Promise<string> = async () => '';
  report() {
    const r = this.recent(20), n = (x: number) => x.toFixed(2), count = new Map<string, number>(); for (const f of r) for (const w of f.what) count.set(w, (count.get(w) ?? 0) + 1);
    return [`Rooftop Light Lab frame report`, this.where(), this.gpuName, navigator.userAgent, `display ${(1000 / this.vsync).toFixed(1)} Hz, devicePixelRatio ${devicePixelRatio}, window ${innerWidth}x${innerHeight}, pace ${this.pace.label()}, refreshes passed over ${this.skipped}`, `${r.length} frames, ${(this.even() * 100).toFixed(0)} % even, drawn ${r[r.length - 1]?.w}x${r[r.length - 1]?.h}`,
      `ms: usual, 95th, 99th, worst`, ...NAMES.map(([k, name]) => `${name.trim()}: ${[0.5, 0.95, 0.99, 1].map(p => n(this.q(k, p, r))).join(', ')}`), `happened: ${[...count].map(([k, v]) => `${k} x${v}`).join(', ') || 'nothing'}`, `gaps: ${r.slice(-240).map(f => Math.round(f.gap * 10) / 10).join(' ')}`,
      `worst: ${[...r].sort((a, b) => b.gap - a.gap).slice(0, 12).map(f => `[${n(f.gap)} cpu ${n(f.cpu)} sim ${n(f.sim)} send ${n(f.send)} enc ${n(f.encode)} done ${n(f.done)} gpu ${n(f.gpu)} ticks ${f.ticks} ${f.what.join('+')}]`).join(' ')}`].join('\n');
  }
}
