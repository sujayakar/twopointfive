// THE TERRACE, TWICE: the garden corner of the rooftop as boxes and in fine geometry, to look at and to time on whatever machine opens the page.
import { Lab, TICK, type Method } from './lab';
import { Present } from './present';
import { realtime } from './stages/realtime';
import { pathTracer } from './stages/path';
import { withMeshes, RASTER, drawn } from './stages/meshes';
import { buildCorner, giveCorner, type Corner } from './scene/corner';
import { measureLost } from './mesh/lost';
import { showError } from './gpu';
import type { V3 } from './math';

const $ = (id: string) => document.getElementById(id)!, wait = (ms = 0) => new Promise(r => setTimeout(r, ms)), FL = 3.4, LENS = 28 * Math.PI / 180;
const VIEWS: Record<string, { look: V3; dist: number; pitch: number; yaw: number }> = { 'The game, 21 m': { look: [7.8, FL, 1.2], dist: 21, pitch: 0.94, yaw: 2.55 }, 'All the way out, 40 m': { look: [8, FL, 1.6], dist: 40, pitch: 0.94, yaw: 2.55 }, 'All the way in, 6 m': { look: [6.9, FL + 0.2, 0.5], dist: 6, pitch: 0.8, yaw: 2.55 }, 'Walking': { look: [6.6, FL + 0.8, 0.6], dist: 5.5, pitch: 0.3, yaw: 2.25 } };
const HOURS: Record<string, number> = { Day: 15.4, Dusk: 17.8, Night: 21.5 };
interface Side { name: string; lab: Lab; present: Present; rt: Method; ref: Method; info: string }

async function made(say: (t: string) => void): Promise<Corner> {
  const text = document.querySelector('script[type=module]')?.textContent ?? '';
  try {
    if (text.length < 1000) throw new Error('no text of its own'); const url = URL.createObjectURL(new Blob([text], { type: 'text/javascript' })), w = new Worker(url, { type: 'module' });
    return await new Promise<Corner>((ok, no) => { w.onmessage = e => { if (e.data.say) say('Making ' + e.data.say); else { ok(e.data.corner); w.terminate(); } }; w.onerror = no; w.postMessage('make'); });
  } catch { say('Making the stones (the page stands still meanwhile)'); await wait(); await wait(); return buildCorner(); }
}
async function page() {
  const canvas = $('c') as HTMLCanvasElement, say = (t: string) => { $('what').textContent = t; }, t0 = performance.now(), corner = await made(say), tMade = (performance.now() - t0) / 1000; giveCorner(corner);
  const open = async (name: string, scene: string): Promise<Side> => { say('Opening ' + name.toLowerCase()); await wait(); const lab = new Lab(); await lab.open(canvas, scene); const present = new Present(lab.gpu); await present.init(); const rt = withMeshes(realtime()), ref = pathTracer(); lab.resize([960, 540], [1280, 720]); say('Compiling ' + name.toLowerCase()); await wait(); lab.seek(600); await lab.use(rt); lab.haze = 0; lab.camera.fov = LENS; lab.knobs[1] = 1; let info = ''; if (lab.building.world.meshes) { say('Measuring what levels lose of their light'); await wait(); info = `light measured in ${((await measureLost(lab, RASTER.tau, 1)).ms / 1000).toFixed(1)} s`; await lab.use(rt); } return { name, lab, present, rt, ref, info }; };
  const sides = [await open('Boxes', 'rooftop'), await open('Fine geometry', 'corner')]; (window as never as { sides: unknown }).sides = sides;
  let on = 1, ref = false, busy = false, timing = false, hour = 15.4; const want = { ...VIEWS['The game, 21 m']!, look: [...VIEWS['The game, 21 m']!.look] as V3 }; let scale = 0.75;
  const front = () => { const s = sides[on]!; s.lab.gpu.context!.configure({ device: s.lab.gpu.device, format: s.lab.gpu.format, alphaMode: 'opaque' }); }; front();
  const fit = () => { if (timing) return; const dpr = window.devicePixelRatio || 1, w = Math.max(64, innerWidth * dpr), h = Math.max(64, innerHeight * dpr), k = Math.min(1, Math.sqrt(2.4e6 / (w * h))), ow = Math.round(w * k), oh = Math.round(h * k); if (canvas.width !== ow || canvas.height !== oh) { canvas.width = ow; canvas.height = oh; } sides[on]!.lab.resize([Math.round(ow * scale), Math.round(oh * scale)], [ow, oh]); };
  // ---- hands
  let drag: { x: number; y: number; pan: boolean } | null = null; canvas.onpointerdown = e => { drag = { x: e.clientX, y: e.clientY, pan: e.button === 2 || e.shiftKey }; canvas.setPointerCapture(e.pointerId); }; canvas.onpointerup = canvas.onpointercancel = () => { drag = null; }; canvas.oncontextmenu = e => e.preventDefault();
  canvas.onpointermove = e => { if (!drag) return; const dx = e.clientX - drag.x, dy = e.clientY - drag.y; drag.x = e.clientX; drag.y = e.clientY; if (drag.pan) { const k = want.dist * 0.0009, s = Math.sin(want.yaw), c = Math.cos(want.yaw); want.look = [want.look[0] + (-dx * c - dy * s) * k, want.look[1], want.look[2] + (dx * s - dy * c) * k]; } else { want.yaw -= dx * 0.005; want.pitch = Math.max(0.08, Math.min(1.5, want.pitch + dy * 0.004)); } };
  canvas.onwheel = e => { e.preventDefault(); want.dist = Math.max(4, Math.min(60, want.dist * Math.exp(e.deltaY * 0.0012))); };
  const held = new Set<string>(); onkeydown = e => { if (e.metaKey || e.ctrlKey) return; held.add(e.key.toLowerCase()); if (e.key === ' ') { e.preventDefault(); void turn(1 - on); } if (e.key === 'p') void trace(!ref); }; onkeyup = e => held.delete(e.key.toLowerCase()); onblur = () => held.clear(); onresize = fit;
  // ---- the panel
  const paints: (() => void)[] = [], paint = () => paints.forEach(f => f()), buttons = (id: string, names: string[], get: () => number, set: (i: number) => void) => { const el = $(id); el.innerHTML = names.map((n, i) => `<button data-i="${i}">${n}</button>`).join(''); el.onclick = e => { const b = (e.target as HTMLElement).closest('button'); if (b && !busy && !timing) { set(Number(b.dataset.i)); paint(); } }; paints.push(() => el.querySelectorAll('button').forEach((b, i) => b.classList.toggle('on', i === get()))); };
  const turn = async (i: number) => { if (busy || timing || i === on) return; busy = true; on = i; front(); fit(); await sides[on]!.lab.use(ref ? sides[on]!.ref : sides[on]!.rt); busy = false; paint(); }, trace = async (r: boolean) => { if (busy || timing || r === ref) return; busy = true; ref = r; try { await sides[on]!.lab.use(ref ? sides[on]!.ref : sides[on]!.rt); } catch (e) { showError(String(e)); ref = false; } busy = false; paint(); };
  buttons('side', sides.map(s => s.name), () => on, i => void turn(i)); buttons('ref', ['Real time', 'Path traced'], () => ref ? 1 : 0, i => void trace(i === 1)); buttons('view', Object.keys(VIEWS), () => -1, i => { const v = Object.values(VIEWS)[i]!; Object.assign(want, v, { look: [...v.look] }); }); buttons('hour', Object.keys(HOURS), () => Object.values(HOURS).indexOf(hour), i => { hour = Object.values(HOURS)[i]!; }); $('more').onclick = () => { $('more').textContent = $('panel').classList.toggle('shut') ? 'More' : 'Less'; };
  // ---- timing: the same frames of both, turn about
  $('time').onclick = () => { if (!timing) void time(); };
  async function time() {
    timing = true; while (busy) await wait(20); busy = true; const out = $('out'), was = { on, ref, hour }, lines: string[] = [], put = (l: string) => { lines.push(l); out.textContent = lines.join('\n'); }; out.hidden = false; const gpu = sides[0]!.lab.gpu;
    put(`${gpu.info || navigator.userAgent}   1920 x 1080 drawn at ${scale}`); put(`fine: ${(corner.data.byteLength / 1e6).toFixed(0)} MB of meshes, made in ${tMade.toFixed(1)} s, ${sides[1]!.info}`); put(''.padEnd(30) + 'boxes          fine           meshes'); put(''.padEnd(30) + 'GPU    clock   GPU    clock   of it');
    for (const [hn, hr] of Object.entries(HOURS)) for (const [vn, v] of Object.entries(VIEWS)) {
      if (hn === 'Dusk' && vn.startsWith('All')) continue; const got: { gpu: number; clock: number; mesh: number }[] = [];
      for (const i of [0, 1]) { const s = sides[i]!, lab = s.lab; on = i; front(); lab.smooth = false; lab.resize([Math.round(1920 * scale), Math.round(1080 * scale)], [1920, 1080]); if (canvas.width !== 1920) { canvas.width = 1920; canvas.height = 1080; } await lab.use(s.rt); Object.assign(lab.camera.want, v, { look: [...v.look] }); lab.camera.snap(); lab.hour = hr; lab.pose(); lab.graph.timer.on = true; const spans: number[] = [], mesh: number[] = []; let t = 0; for (let f = 0; f < 150; f++) { lab.step(); s.present.set(lab, f === 0); lab.draw((e, r) => s.present.run(e, lab, r, lab.gpu.context!.getCurrentTexture().createView(), lab.gpu.format, 'rt')); if (f % 4 === 3) await lab.gpu.device.queue.onSubmittedWorkDone(); if (f === 69) t = performance.now(); if (f >= 70) { spans.push(lab.graph.timer.span); const k = lab.graph.timer.last; let m = 0; for (const [n, x] of k) if (n.startsWith('mesh ')) m += x; mesh.push(m); } } await lab.gpu.device.queue.onSubmittedWorkDone(); const med = (l: number[]) => l.sort((a, b) => a - b)[l.length >> 1]!; got.push({ gpu: med(spans), clock: (performance.now() - t) / 80, mesh: med(mesh) }); await wait(); }
      const n = (x: number) => x.toFixed(2).padStart(5); put(`${hn.padEnd(6)} ${vn.padEnd(22)} ${n(got[0]!.gpu)}  ${n(got[0]!.clock)}   ${n(got[1]!.gpu)}  ${n(got[1]!.clock)}   ${n(got[1]!.mesh)}`);
    }
    put('GPU: from the frame\'s first pass to its last, ms. clock: what a frame took by the clock, up to four at once.'); (window as never as { timeText: string }).timeText = lines.join('\n'); on = was.on; hour = was.hour; timing = false; busy = false; front(); fit(); for (const s of sides) { s.lab.smooth = true; await s.lab.use(s === sides[on] && ref ? s.ref : s.rt); } paint();
  }
  // ---- the loop
  for (const s of sides) s.lab.smooth = true; paint(); $('load').remove(); let last = performance.now(), acc = 0, frames = 0, flight = 0, fps = 60, wasHour = [-1, -1], wasAt = ''; let seen = ''; const at = { ...want, look: [...want.look] as V3 };
  const loop = (now: number) => {
    requestAnimationFrame(loop); if (timing || busy || flight > 2) return; const s = sides[on]!, lab = s.lab, c = lab.camera, dt = Math.min(0.1, (now - last) / 1000); last = now; fps += (1 / Math.max(dt, 1e-3) - fps) * 0.05;
    const k = dt * want.dist * 0.5, sn = Math.sin(want.yaw), cs = Math.cos(want.yaw), x = (held.has('d') ? 1 : 0) - (held.has('a') ? 1 : 0), z = (held.has('s') ? 1 : 0) - (held.has('w') ? 1 : 0); want.look = [want.look[0] + (x * cs + z * sn) * k, want.look[1] + ((held.has('e') ? 1 : 0) - (held.has('q') ? 1 : 0)) * k, want.look[2] + (-x * sn + z * cs) * k];
    const ease = 1 - Math.exp(-dt * 8), go = (a: number, b: number) => Math.abs(b - a) < 1e-5 ? b : a + (b - a) * ease; at.look = at.look.map((v, i) => go(v, want.look[i]!)) as V3; at.yaw = go(at.yaw, want.yaw); at.pitch = go(at.pitch, want.pitch); at.dist = go(at.dist, want.dist); Object.assign(c.want, at, { look: [...at.look] }); c.snap();
    if (wasHour[on] !== hour) { wasHour[on] = hour; lab.hour = hour; lab.pose(); } acc = ref ? 0 : acc + dt; let n = 0; while (acc >= TICK && n++ < 4) { acc -= TICK; lab.step(); } if (n >= 4) acc = 0; c.snap(); const where = JSON.stringify(at); if (ref && where !== wasAt) s.ref.reset?.(lab); wasAt = where; fit(); frames++; lab.graph.timer.on = frames % 12 === 0; s.present.set(lab); flight++;
    lab.draw((e, r) => s.present.run(e, lab, r, lab.gpu.context!.getCurrentTexture().createView(), lab.gpu.format, ref ? 'ref' : 'rt')); void lab.gpu.device.queue.onSubmittedWorkDone().then(() => { flight--; });
    if (frames % 30 === 0 && on === 1 && !ref) void drawn(lab).then(d => { seen = `${(d.triangles / 1e6).toFixed(2)} M triangles in ${d.slots} clusters`; });
    if (frames % 15 === 0) { const t = lab.graph.timer.ms; let mesh = 0; for (const [nm, v] of t) if (nm.startsWith('mesh ')) mesh += v; $('stats').textContent = `${fps.toFixed(0)} fps   GPU ${lab.graph.timer.span.toFixed(1)} ms   ${lab.graph.render.join(' x ')}\na pixel is ${(2 * Math.tan(LENS / 2) * at.dist / lab.graph.render[1] * 1000).toFixed(1)} mm` + (ref ? `\n${(s.ref as unknown as { samples: number }).samples} paths a pixel` : on === 1 ? `\n${seen}\nmeshes ${mesh.toFixed(2)} ms` : ''); }
  };
  fit(); await sides[on]!.lab.use(sides[on]!.rt); requestAnimationFrame(loop);
}
if (typeof document === 'undefined') (globalThis as never as { onmessage: unknown }).onmessage = () => { const post = postMessage as (m: unknown, t?: Transferable[]) => void, corner = buildCorner(say => post({ say })); post({ corner }, [corner.data.buffer]); };
else page().catch(e => { const text = e instanceof Error ? e.message : String(e), what = document.getElementById('what'); if (what) what.textContent = text; showError(String(e?.stack ?? e)); console.error(e); });
