// The page: input, a clock of fixed ticks, the panel. Everything drawn comes from Lab.
import { Lab, TICK, type Method } from './lab';
import { realtime } from './stages/realtime';
import { pathTracer } from './stages/path';
import { Present } from './present';
import { showError } from './gpu';
import { type V3, add, mul, sub, dot, clamp, norm } from './math';
import { boundsOf } from './scene/world';

const $ = <T extends HTMLElement>(id: string) => document.getElementById(id) as T, canvas = $<HTMLCanvasElement>('c'), Q = new URLSearchParams(location.search);
const say = (t: string, k: number) => { $('what').textContent = t; $('bar').style.width = Math.round(k * 100) + '%'; };
async function main() {
  const lab = new Lab(); say('Building the scene', 0.12); await lab.open(canvas); (window as never as { lab: Lab }).lab = lab;
  const present = new Present(lab.gpu); await present.init(); const rt = realtime(), ref = pathTracer(); let method: Method = rt;
  let scale = Number(Q.get('scale') ?? 0.67), auto = !Q.has('scale'); const cap = Number(Q.get('px') ?? 2.4e6);
  const fit = () => { const dpr = window.devicePixelRatio || 1, w = Math.max(64, innerWidth * dpr), h = Math.max(64, innerHeight * dpr), k = Math.min(1, Math.sqrt(cap / (w * h))), ow = Math.round(w * k), oh = Math.round(h * k); if (canvas.width !== ow || canvas.height !== oh) { canvas.width = ow; canvas.height = oh; } lab.resize([Math.max(32, Math.round(ow * scale)), Math.max(32, Math.round(oh * scale))], [ow, oh]); };
  fit(); say('Lighting every wall', 0.3); await lab.use(rt); say('Reference', 0.8); await lab.use(ref); await lab.use(rt); say('Ready', 1);
  lab.hour = Number(Q.get('hour') ?? 16.4); lab.camera.want = { look: [1.5, 3, 0.5], yaw: 0.62, pitch: 0.86, dist: 50 }; lab.camera.snap();
  // ---- what the hand does, kept until the next tick, so that a run is its ticks and its inputs
  const held = new Set<string>(); let drag: { x: number; y: number; moved: number } | null = null, point = { x: 0.5, y: 0.5 }, tour = Q.has('tour'), torchOn = false, viewIx = 0;
  const views = ['picture', 'vis', 'bounce', 'mirror', 'air', 'emit'] as const; const signalOf: Record<string, string> = { vis: 'visA', bounce: 'bounceB', mirror: 'mirrorB', air: 'airH', emit: 'emit' };
  const act = { method(m: Method) { method = m; void lab.use(m); paint(); }, level(i: number) { lab.level = i; const l = lab.building.levels[i]!; lab.wantCut = l.cut; lab.camera.want.look = [lab.camera.want.look[0], l.look[1], lab.camera.want.look[2]]; paint(); }, doors(v: number | 'auto') { lab.doorsAuto = v === 'auto'; if (v !== 'auto') for (const d of lab.building.doors) d.want = v; paint(); } };
  const row = (id: string, items: [string, () => boolean, () => void][]) => { const el = $(id); el.textContent = ''; for (const [t, on, go] of items) { const b = document.createElement('button'); b.textContent = t; b.className = on() ? 'on' : ''; b.onclick = () => { go(); paint(); }; el.append(b); } };
  function paint() {
    row('method', [['Real time', () => method === rt, () => act.method(rt)], ['Reference', () => method === ref, () => act.method(ref)]]);
    row('level', lab.building.levels.map((l, i) => [l.name, () => lab.level === i, () => act.level(i)] as [string, () => boolean, () => void]));
    row('doors', [['Doors open', () => false, () => act.doors(1)], ['shut', () => false, () => act.doors(0)], ['swing', () => lab.doorsAuto, () => act.doors(lab.doorsAuto ? 0 : 'auto')]]);
    row('lamps', (['auto', 'on', 'off'] as const).map(k => ['Lamps ' + k, () => lab.lampsMode === k, () => { lab.lampsMode = k; }] as [string, () => boolean, () => void]));
    row('view', views.map((v, i) => [v, () => viewIx === i, () => { viewIx = i; }] as [string, () => boolean, () => void]));
    $('mode').textContent = method === rt ? 'Real time' : 'Reference, path traced';
    $<HTMLInputElement>('hour').value = String(lab.hour);
  }
  paint(); $<HTMLInputElement>('hour').oninput = e => { lab.hour = Number((e.target as HTMLInputElement).value); }; const w0 = lab.wind; $<HTMLInputElement>('wind').oninput = e => { lab.wind = mul(w0, Number((e.target as HTMLInputElement).value)); };
  const pickDoor = (u: number, v: number) => { const r = lab.camera.ray(lab.graph.output[0] / lab.graph.output[1], u, v); let best = 1e9, hit = null as (typeof lab.building.doors)[0] | null; for (const d of lab.building.doors) { const b = boundsOf(d.leaf[0]!); let a = 0, z = 1e9; for (let k = 0; k < 3; k++) { const p = (b.mn[k]! - 0.15 - r.from[k]!) / r.dir[k]!, q = (b.mx[k]! + 0.15 - r.from[k]!) / r.dir[k]!; a = Math.max(a, Math.min(p, q)); z = Math.min(z, Math.max(p, q)); } if (a < z && a < best && r.from[1] + r.dir[1] * a < lab.cut) { best = a; hit = d; } } return hit; };
  canvas.onpointerdown = e => { drag = { x: e.clientX, y: e.clientY, moved: 0 }; canvas.setPointerCapture(e.pointerId); };
  canvas.onpointermove = e => { point = { x: e.clientX / innerWidth, y: e.clientY / innerHeight }; if (!drag) return; const dx = e.clientX - drag.x, dy = e.clientY - drag.y; drag.x = e.clientX; drag.y = e.clientY; drag.moved += Math.abs(dx) + Math.abs(dy); if (drag.moved < 4) return; tour = false; const c = lab.camera; if (e.shiftKey || e.buttons === 2) pan(-dx * c.dist * 0.0007, -dy * c.dist * 0.0009); else { c.want.yaw -= dx * 0.005; c.want.pitch += dy * 0.004; } };
  canvas.onpointerup = e => { if (drag && drag.moved < 5) { const d = pickDoor(e.clientX / innerWidth, e.clientY / innerHeight); if (d) { lab.doorsAuto = false; d.want = d.want > 0.5 ? 0 : 1; paint(); } } drag = null; };
  canvas.oncontextmenu = e => e.preventDefault(); canvas.onwheel = e => { e.preventDefault(); tour = false; lab.camera.want.dist *= Math.exp(e.deltaY * 0.0011); };
  const pan = (x: number, z: number) => { const c = lab.camera, s = Math.sin(c.yaw), k = Math.cos(c.yaw); c.want.look = [clamp(c.want.look[0] + x * k + z * s, -18, 18), c.want.look[1], clamp(c.want.look[2] - x * s + z * k, -14, 14)]; };
  onkeydown = e => { if (e.metaKey || e.ctrlKey) return; const k = e.key.toLowerCase(); held.add(k); if (e.repeat) return; if (k === 'p') act.method(method === rt ? ref : rt); if (k === '1' || k === '2' || k === '3') act.level(Number(k) - 1); if (k === 'f') torchOn = !torchOn; if (k === 'v') { viewIx = (viewIx + 1) % views.length; paint(); } if (k === 'c') tour = !tour; if (k === 'h') document.body.classList.toggle('bare'); if (k === 'o') act.doors(lab.building.doors.some(d => d.want > 0.5) ? 0 : 1); if (k === ' ') lab.paused = !lab.paused; };
  onkeyup = e => held.delete(e.key.toLowerCase()); onblur = () => held.clear(); onresize = fit;
  // ---- the torch at the pointer: a lamp like any other
  const hand = lab.building.world.lamps.length; lab.building.world.lamps.push({ kind: 'spot', at: [0, -50, 0], u: [0.03, 0, 0], v: [0, -1, 0], radiance: [52000, 49000, 43000], cos: 0.93, name: 'hand', on: 0 });
  let acc = 0, last = performance.now(), slow = 0, seen = 0, ms = 16, frames = 0, bad = false; setTimeout(() => { $('load').style.opacity = '0'; setTimeout(() => $('load').remove(), 900); }, 300);
  const loop = () => {
    requestAnimationFrame(loop); if (bad) return; const now = performance.now(), dt = Math.min(0.1, (now - last) / 1000); last = now; ms = ms * 0.95 + dt * 1000 * 0.05;
    try {
      if (method === rt && !lab.paused) {
        acc += dt; let n = 0; while (acc >= TICK && n < 4) { acc -= TICK; n++;
          const sp = lab.camera.dist * 0.012; if (held.has('w')) pan(0, -sp); if (held.has('s')) pan(0, sp); if (held.has('a')) pan(-sp, 0); if (held.has('d')) pan(sp, 0); if (held.has('q')) lab.camera.want.yaw += 0.02; if (held.has('e')) lab.camera.want.yaw -= 0.02; if (held.size) tour = held.has('c') ? tour : false;
          if (tour) tourStep(lab, act); lab.step(); } if (n === 4) acc = 0;
        const r = lab.camera.ray(lab.graph.output[0] / lab.graph.output[1], point.x, point.y), l = lab.building.world.lamps[hand]!; const plane = lab.level === 2 ? 0.8 : lab.level === 1 ? 4.2 : 4; const t = (plane - r.from[1]) / Math.min(r.dir[1], -0.05), at = add(r.from, mul(r.dir, t)); const src: V3 = add(at, [1.2, lab.level === 0 ? 4.5 : 1.35, 2.6]); if (src[1] > lab.cut - 0.1) src[1] = lab.cut - 0.1; l.at = src; l.v = norm(sub(at, src)); lab.scene.lampOn[hand] = torchOn ? 1 : 0; l.on = torchOn ? 1 : 0; void dot;
      }
      // hold the frame time: fewer pixels when it is dear, more when there is room
      let gpu = 0; for (const v of lab.graph.timer.ms.values()) gpu += v;
      if (auto && method === rt && ++seen > 45) { const cost = lab.gpu.timestamps && gpu > 0.2 ? gpu : ms; if (cost > 15.0) slow++; else if (cost < 11.0) slow--; else slow = 0; if (slow > 14 && scale > 0.4) { scale = Math.max(0.4, scale - 0.07); slow = 0; seen = 0; fit(); } else if (slow < -90 && scale < 1) { scale = Math.min(1, scale + 0.06); slow = 0; seen = 0; fit(); } }
      const pre = method === ref ? 'ref' : 'rt';
      present.set(lab); lab.draw((e, result) => { const v = views[viewIx]!, plain = v !== 'picture' && method === rt, src = plain ? lab.graph.texture(signalOf[v]!) : result; present.run(e, lab, src, lab.gpu.context!.getCurrentTexture().createView(), lab.gpu.format, pre, plain ? { plain: v === 'vis' || v === 'air' ? 1 : 0.25 } : {}); });
      if (++frames % 12 === 0) { const t = [...lab.graph.timer.ms].sort((a, b) => b[1] - a[1]); $('stats').textContent = `${(1000 / ms).toFixed(0)} fps  ${lab.graph.render.join('×')} → ${lab.graph.output.join('×')}\n` + (method === ref ? `${ref.samples} paths a pixel\n` : '') + (gpu ? `GPU ${gpu.toFixed(1)} ms\n` + t.slice(0, 14).map(([k, v]) => k.padEnd(16) + v.toFixed(2)).join('\n') : lab.gpu.info); }
    } catch (e) { bad = true; showError(String((e as Error).stack ?? e)); }
  };
  requestAnimationFrame(loop);
}
/** the tour: a pure function of the tick */
function tourStep(lab: Lab, act: { level(i: number): void; doors(v: number | 'auto'): void }) {
  const t = lab.tick * TICK, c = lab.camera.want, leg = Math.floor(t / 14) % 5; c.yaw = 0.62 + t * 0.045; c.pitch = 0.80 + 0.16 * Math.sin(t * 0.11); c.dist = [52, 34, 38, 30, 46][leg]!; const look: V3[] = [[1, 3, 0], [7.5, 3.6, 0.5], [-4, 3.4, -1], [2, 0.6, 0], [0, 0.6, 0]]; c.look = look[leg]!; const lv = [0, 0, 1, 2, 2][leg]!; if (lv !== lab.level) act.level(lv); if (!lab.doorsAuto) act.doors('auto'); lab.hour = 7.5 + ((t * 0.12) % 14.5);
}
main().catch(e => { console.error(e); showError(String(e.stack ?? e)); $('what').textContent = String(e.message ?? e); });
