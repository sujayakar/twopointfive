// The page: input, a clock of fixed ticks, the panel. Everything drawn comes from Lab.
import { Lab, TICK, type Method } from './lab';
import { realtime } from './stages/realtime';
import { pathTracer } from './stages/path';
import { Present } from './present';
import { showError } from './gpu';
import { run } from './bench';
import { Watch } from './watch';
import { Teach } from './teach';
import { tourStep } from './tour';
import { type V3, add, mul, sub, dot, clamp, norm } from './math';
import { boundsOf } from './scene/world';

const $ = <T extends HTMLElement>(id: string) => document.getElementById(id) as T, canvas = $<HTMLCanvasElement>('c'), Q = new URLSearchParams(location.search);
const say = (t: string, k: number) => { $('what').textContent = t; $('bar').style.width = Math.round(k * 100) + '%'; };
async function main() {
  const lab = new Lab(); say('Building the scene', 0.12); await lab.open(canvas); if (Q.has('watch')) lab.graph.guard = 'off'; (window as never as { lab: Lab }).lab = lab;
  const present = new Present(lab.gpu); await present.init(); const rt = realtime(), ref = pathTracer(); let method: Method = rt;
  let scale = Number(Q.get('scale') ?? 0.67), auto = !Q.has('scale'); const cap = Number(Q.get('px') ?? 2.4e6);
  const fit = () => { const dpr = window.devicePixelRatio || 1, w = Math.max(64, innerWidth * dpr), h = Math.max(64, innerHeight * dpr), k = Math.min(1, Math.sqrt(cap / (w * h))), ow = Math.round(w * k), oh = Math.round(h * k); if (canvas.width !== ow || canvas.height !== oh) { canvas.width = ow; canvas.height = oh; } lab.resize([Math.max(32, Math.round(ow * scale)), Math.max(32, Math.round(oh * scale))], [ow, oh]); };
  fit(); say('Lighting every wall', 0.3); await lab.use(rt); say('Reference', 0.8); await lab.use(ref); await lab.use(rt); say('Ready', 1);
  lab.hour = Number(Q.get('hour') ?? 16.4); lab.camera.want = { look: [1.5, 3, 0.5], yaw: 0.62, pitch: 0.86, dist: 50 }; lab.camera.snap();
  // ---- what the hand does, kept until the next tick, so that a run is its ticks and its inputs
  const held = new Set<string>(); let drag: { x: number; y: number; moved: number } | null = null, point = { x: 0.5, y: 0.5 }, tour = Q.has('tour'), torchOn = false, plant: [number, number, number] | true | null = null;
  const teach = new Teach(lab, present, rt, ref, { close() { lab.paused = false; paint(); }, point: () => point }); const study = () => { if (teach.on) { teach.close(); return; } if (method !== rt) act.method(rt); tour = false; lab.paused = true; teach.open(); paint(); };
  const act = { method(m: Method) { method = m; void lab.use(m); paint(); }, level(i: number) { lab.level = i; const l = lab.building.levels[i]!; lab.wantCut = l.cut; lab.camera.want.look = [lab.camera.want.look[0], l.look[1], lab.camera.want.look[2]]; paint(); }, doors(v: number | 'auto') { lab.doorsAuto = v === 'auto'; if (v !== 'auto') for (const d of lab.building.doors) d.want = v; paint(); } };
  (window as never as { teach: Teach }).teach = teach;
  const row = (id: string, items: [string, () => boolean, () => void][]) => { const el = $(id); el.textContent = ''; for (const [t, on, go] of items) { const b = document.createElement('button'); b.textContent = t; b.className = on() ? 'on' : ''; b.onclick = () => { go(); paint(); }; el.append(b); } };
  function paint() {
    row('hide', [['Hide', () => false, () => { document.body.classList.add('nopanel'); }]]);
    row('method', [['Real time', () => method === rt, () => act.method(rt)], ['Reference', () => method === ref, () => act.method(ref)]]);
    row('level', lab.building.levels.map((l, i) => [l.name, () => lab.level === i, () => act.level(i)] as [string, () => boolean, () => void]));
    row('doors', [['Doors open', () => false, () => act.doors(1)], ['shut', () => false, () => act.doors(0)], ['swing', () => lab.doorsAuto, () => act.doors(lab.doorsAuto ? 0 : 'auto')]]);
    row('lamps', (['auto', 'on', 'off'] as const).map(k => ['Lamps ' + k, () => lab.lampsMode === k, () => { lab.lampsMode = k; }] as [string, () => boolean, () => void]));
    row('view', [['Step through the frame', () => teach.on, study]]);
    row('groups', ['atrium', 'orange', 'green', 'blue', 'gallery', 'studio', 'lounge'].map(k => [k, () => (lab.lampGroup[k] ?? 1) > 0, () => { lab.lampGroup[k] = (lab.lampGroup[k] ?? 1) > 0 ? 0 : 1; }] as [string, () => boolean, () => void]));
    row('weather', [['Clear', () => lab.rain === 0, () => { lab.rain = 0; }], ['Rain', () => lab.rain > 0, () => { lab.rain = 1; }], ['Steam', () => lab.steam > 0, () => { lab.steam = lab.steam > 0 ? 0 : 1; }]]);
    $('mode').textContent = method === rt ? 'Real time' : 'Reference, path traced';
    $<HTMLInputElement>('hour').value = String(lab.hour);
  }
  paint(); $<HTMLInputElement>('hour').oninput = e => { lab.hour = Number((e.target as HTMLInputElement).value); }; const w0 = lab.wind; $<HTMLInputElement>('wind').oninput = e => { lab.wind = mul(w0, Number((e.target as HTMLInputElement).value)); };
  // ---- lamps: a pin on each one of this floor; a click on a pin, or on a lamp that can be seen, switches it
  const pins = document.createElement('div'); pins.id = 'pins'; document.body.append(pins); const pinOf: HTMLButtonElement[] = []; const flip = (i: number) => { if (lab.lampOff.has(i)) lab.lampOff.delete(i); else lab.lampOff.add(i); lab.pose(); paint(); };
  const floorOf = (y: number) => y < 3.3 ? 2 : y < 6.7 ? 1 : 0, placePins = () => {
    const c = lab.camera.frame(lab.graph.output[0] / lab.graph.output[1]); lab.building.world.lamps.forEach((l, i) => {
      if (l.kind === 'spot' || l.name === 'pool') return; let b = pinOf[i]; if (!b) { b = pinOf[i] = document.createElement('button'); b.title = l.name; b.setAttribute('aria-label', 'Switch lamp ' + l.name); b.onclick = () => { flip(i); b!.blur(); }; pins.append(b); }
      const v = sub(l.at, c.pos), z = dot(v, c.fwd), x = dot(v, c.right) / dot(c.right, c.right) / z, y = dot(v, c.up) / dot(c.up, c.up) / z, f = floorOf(l.at[1]), here = lab.level === 0 ? f < 2 && l.name === 'terrace' : f === lab.level || (l.name === 'terrace' && lab.level === 1); b.hidden = !here || z < 0.5 || Math.abs(x) > 1.05 || Math.abs(y) > 1.05; b.style.transform = `translate(${(x * 0.5 + 0.5) * innerWidth}px,${(0.5 - y * 0.5) * innerHeight}px)`; b.className = lab.lampOff.has(i) ? 'off' : ''; }); };
  // ---- X: what the exposure looks at: the person nearest the pointer, else the spot under it; again to let go
  const eyePin = document.createElement('div'); eyePin.id = 'eye'; eyePin.hidden = true; document.body.append(eyePin);
  const pinEye = async () => { if (lab.eye !== null) { lab.eye = null; return; } const [w, h] = lab.graph.output; let best = 40 / innerHeight, who = -1; lab.building.people.forEach((p, i) => { const q = lab.project([p.pos[0], p.pos[1] + 1, p.pos[2]]); if (!q || p.pos[1] > lab.cut) return; const d = Math.hypot((q[0] - point.x) * innerWidth / innerHeight, q[1] - point.y); if (d < best) { best = d; who = i; } }); if (who >= 0) { lab.eye = who; return; }
    const r = lab.camera.ray(w / h, point.x, point.y); lab.graph.flip(); const s = lab.graph.texture('surf'); lab.graph.flip(); const buf = lab.gpu.device.createBuffer({ size: 256, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ }), e = lab.gpu.device.createCommandEncoder(); e.copyTextureToBuffer({ texture: s, origin: [Math.min(s.width - 1, Math.floor(point.x * s.width)), Math.min(s.height - 1, Math.floor(point.y * s.height))] }, { buffer: buf, bytesPerRow: 256 }, [1, 1]); lab.gpu.device.queue.submit([e.finish()]); await buf.mapAsync(GPUMapMode.READ); const t = new Float32Array(buf.getMappedRange())[0]!; buf.destroy(); if (t < 1500) lab.eye = add(r.from, mul(r.dir, t)); };
  const placeEye = () => { const at = lab.eyeAt(), q = at && lab.project(at); eyePin.hidden = !q; if (q) { eyePin.style.transform = `translate(${q[0] * innerWidth}px,${q[1] * innerHeight}px)`; } };
  const lampAt = (u: number, v: number): number => { const r = lab.camera.ray(lab.graph.output[0] / lab.graph.output[1], u, v); let best = 1e9, hit = -1; for (const p of lab.building.world.pieces) { if (p.lamp === undefined || p.lamp < 0) continue; const b = boundsOf(p); let a = 0, z = 1e9; for (let k = 0; k < 3; k++) { const s = (b.mn[k]! - 0.06 - r.from[k]!) / r.dir[k]!, t = (b.mx[k]! + 0.06 - r.from[k]!) / r.dir[k]!; a = Math.max(a, Math.min(s, t)); z = Math.min(z, Math.max(s, t)); } if (a < z && a < best && r.from[1] + r.dir[1] * a < lab.cut) { best = a; hit = p.lamp; } } return hit; };
  const pickDoor = (u: number, v: number) => { const r = lab.camera.ray(lab.graph.output[0] / lab.graph.output[1], u, v); let best = 1e9, hit = null as (typeof lab.building.doors)[0] | null; for (const d of lab.building.doors) { const b = boundsOf(d.leaf[0]!); let a = 0, z = 1e9; for (let k = 0; k < 3; k++) { const p = (b.mn[k]! - 0.15 - r.from[k]!) / r.dir[k]!, q = (b.mx[k]! + 0.15 - r.from[k]!) / r.dir[k]!; a = Math.max(a, Math.min(p, q)); z = Math.min(z, Math.max(p, q)); } if (a < z && a < best && r.from[1] + r.dir[1] * a < lab.cut) { best = a; hit = d; } } return hit; };
  canvas.onpointerdown = e => { drag = { x: e.clientX, y: e.clientY, moved: 0 }; canvas.setPointerCapture(e.pointerId); };
  canvas.onpointermove = e => { point = { x: e.clientX / innerWidth, y: e.clientY / innerHeight }; if (!drag) return; const dx = e.clientX - drag.x, dy = e.clientY - drag.y; drag.x = e.clientX; drag.y = e.clientY; drag.moved += Math.abs(dx) + Math.abs(dy); if (drag.moved < 4) return; tour = false; const c = lab.camera; if (e.shiftKey || e.buttons === 2) pan(-dx * c.dist * 0.0007, -dy * c.dist * 0.0009); else { c.want.yaw -= dx * 0.005; c.want.pitch += dy * 0.004; } };
  canvas.onpointerup = e => { if (drag && drag.moved < 5 && teach.on) teach.click(e.clientX / innerWidth, e.clientY / innerHeight); else if (drag && drag.moved < 5 && lampAt(e.clientX / innerWidth, e.clientY / innerHeight) >= 0) flip(lampAt(e.clientX / innerWidth, e.clientY / innerHeight)); else if (drag && drag.moved < 5) { const d = pickDoor(e.clientX / innerWidth, e.clientY / innerHeight); if (d) { lab.doorsAuto = false; d.want = d.want > 0.5 ? 0 : 1; paint(); } } drag = null; };
  canvas.oncontextmenu = e => e.preventDefault(); canvas.onwheel = e => { e.preventDefault(); tour = false; lab.camera.want.dist *= Math.exp(e.deltaY * 0.0011); };
  const pan = (x: number, z: number) => { const c = lab.camera, s = Math.sin(c.yaw), k = Math.cos(c.yaw); c.want.look = [clamp(c.want.look[0] + x * k + z * s, -18, 18), c.want.look[1], clamp(c.want.look[2] - x * s + z * k, -14, 14)]; };
  onkeydown = e => { if (e.metaKey || e.ctrlKey) return; const k = e.key.toLowerCase(); held.add(k); if (e.repeat) return; if (k === 'p') act.method(method === rt ? ref : rt); if (k === '1' || k === '2' || k === '3') act.level(Number(k) - 1); if (k === 'f') { torchOn = !torchOn; plant = null; } if (k === 'g') plant = plant ? null : true; if (k === 't' && !teach.on) { study(); return; } if (teach.on && teach.key(k)) { e.preventDefault(); return; } if (k === 'c') tour = !tour; if (k === 'x') void pinEye(); if (k === 'h') document.body.classList.toggle('bare'); if (k === 'o') act.doors(lab.building.doors.some(d => d.want > 0.5) ? 0 : 1); if (k === ' ') lab.paused = !lab.paused; };
  onkeyup = e => { held.delete(e.key.toLowerCase()); if (e.key === ' ') teach.hold(false); }; onblur = () => held.clear(); onresize = fit;
  // ---- the torch at the pointer: a lamp like any other
  const hand = lab.building.world.lamps.length; lab.building.world.lamps.push({ kind: 'spot', at: [0, -50, 0], u: [0.03, 0, 0], v: [0, -1, 0], radiance: [52000, 49000, 43000], cos: 0.93, name: 'hand', on: 0 });
  if (Q.has('bench')) {
    $('load').remove(); auto = false; const [bw, bh] = (Q.get('size') ?? '1920x1080').split('x').map(Number) as [number, number]; canvas.width = bw; canvas.height = bh; lab.resize([Math.round(bw * 0.75), Math.round(bh * 0.75)], [bw, bh]); present.set(lab, true); document.body.classList.add('bare'); const out = $('bench'); out.hidden = false;
    const text = await run(lab, rt, Number(Q.get('bench')) || 600, l => tourStep(l, act), (s, k) => { out.textContent = `Benchmark, ${s}\n${'█'.repeat(Math.round(k * 30))}${'·'.repeat(30 - Math.round(k * 30))}`; }, (e, result) => present.run(e, lab, result, lab.gpu.context!.getCurrentTexture().createView(), lab.gpu.format, 'rt'));
    out.textContent = text; const b = document.createElement('button'); b.className = 'on'; b.textContent = 'Copy'; b.onclick = () => { void navigator.clipboard.writeText(text).then(() => { b.textContent = 'Copied'; }, () => { getSelection()?.selectAllChildren(out); }); }; out.prepend(b, '\n\n'); (window as never as { benchText: string }).benchText = text; return;
  }
  // ?watch: every write looked at, and the first stage to make a bad number named. Nothing is cleaned meanwhile
  const watch = Q.has('watch') ? await new Watch(lab.graph).init() : null; let looking = false;
  let acc = 0, last = performance.now(), slow = 0, seen = 0, ms = 16, frames = 0, bad = false; setTimeout(() => { $('load').style.opacity = '0'; setTimeout(() => $('load').remove(), 900); }, 300);
  const loop = () => {
    requestAnimationFrame(loop); if (bad) return; const now = performance.now(), dt = Math.min(0.1, (now - last) / 1000); last = now; ms = ms * 0.95 + dt * 1000 * 0.05;
    try {
      if (method === rt && !lab.paused) {
        acc += dt; let n = 0; while (acc >= TICK && n < 4) { acc -= TICK; n++;
          const sp = lab.camera.dist * 0.012; if (held.has('w')) pan(0, -sp); if (held.has('s')) pan(0, sp); if (held.has('a')) pan(-sp, 0); if (held.has('d')) pan(sp, 0); if (held.has('q')) lab.camera.want.yaw += 0.02; if (held.has('e')) lab.camera.want.yaw -= 0.02; if (held.size) tour = held.has('c') ? tour : false;
          if (tour) tourStep(lab, act); lab.step(); } if (n === 4) acc = 0;
      }
      if (method === rt) {                                       // the torch at the pointer: from over your shoulder, or from where G planted it
        const r = lab.camera.ray(lab.graph.output[0] / lab.graph.output[1], point.x, point.y), l = lab.building.world.lamps[hand]!, plane = lab.level === 2 ? 0.8 : lab.level === 1 ? 4.2 : 4, at = add(r.from, mul(r.dir, (plane - r.from[1]) / Math.min(r.dir[1], -0.05))); if (plant === true) { const f = lab.level === 2 ? 0.02 : 3.45; plant = add(r.from, mul(r.dir, (f - r.from[1]) / Math.min(r.dir[1], -0.05))); plant[1] = f + 1.25; torchOn = true; }
        const src: V3 = plant ? [...plant] : add(at, [1.2, lab.level === 0 ? 4.5 : 1.35, 2.6]); if (src[1] > lab.cut - 0.1) src[1] = lab.cut - 0.1; l.at = src; l.v = norm(sub(at, src)); lab.scene.lampOn[hand] = torchOn ? 1 : 0; l.on = torchOn ? 1 : 0; void dot;
      }
      if (teach.on) { const sp = lab.camera.dist * 0.012 * dt * 60; if (held.has('w')) pan(0, -sp); if (held.has('s')) pan(0, sp); if (held.has('a')) pan(-sp, 0); if (held.has('d')) pan(sp, 0); lab.camera.step(dt); if (!teach.due(now)) return; }
      // hold the frame time: fewer pixels when it is dear, more when there is room
      let gpu = 0; for (const v of lab.graph.timer.ms.values()) gpu += v; gpu = lab.graph.timer.span > 0 ? Math.min(gpu, lab.graph.timer.span) : gpu;
      if (auto && !teach.on && method === rt && ++seen > 45) { const cost = lab.gpu.timestamps && gpu > 0.2 ? gpu : ms; if (cost > 15.0) slow++; else if (cost < 11.0) slow--; else slow = 0; if (slow > 14 && scale > 0.4) { scale = Math.max(0.4, scale - 0.07); slow = 0; seen = 0; fit(); } else if (slow < -90 && scale < 1) { scale = Math.min(1, scale + 0.06); slow = 0; seen = 0; fit(); } }
      const pre = method === ref ? 'ref' : 'rt';
      if (watch) lab.graph.watch = looking ? null : watch; lab.graph.timer.on = frames % 8 === 0; present.set(lab); lab.draw((e, result) => { const to = lab.gpu.context!.getCurrentTexture().createView(); if (teach.on && method === rt) teach.draw(e, result, to); else present.run(e, lab, result, to, lab.gpu.format, pre); });
      if (watch && !looking) { looking = true; void watch.read().then(b => { looking = false; if (b.length) { bad = true; showError(watch.say()); } }); }
      placePins(); placeEye(); if (teach.on) { if (frames % 8 === 0) void teach.probe(); if (frames % 30 === 0) teach.costs(); }
      if (++frames % 12 === 0) { const t = [...lab.graph.timer.ms].sort((a, b) => b[1] - a[1]); $('stats').textContent = `${(1000 / ms).toFixed(0)} fps  ${lab.graph.render.join('×')} → ${lab.graph.output.join('×')}\n` + (method === ref ? `${ref.samples} paths a pixel\n` : '') + (gpu ? `GPU ${gpu.toFixed(1)} ms\n` + t.slice(0, 14).map(([k, v]) => k.padEnd(16) + v.toFixed(2)).join('\n') : lab.gpu.info); }
    } catch (e) { bad = true; showError(String((e as Error).stack ?? e)); }
  };
  requestAnimationFrame(loop);
}
main().catch(e => { console.error(e); showError(String(e.stack ?? e)); $('what').textContent = String(e.message ?? e); });
