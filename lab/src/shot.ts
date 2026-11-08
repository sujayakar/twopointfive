// Headless stills: window.shot({...}) draws a view with a method for so many samples and hands back PNG bytes. For tools and checks.
import { Lab, type Method } from './lab';
import { pathTracer } from './stages/path';
import { Present } from './present';
import { realtime, noDenoise } from './stages/realtime';
import type { V3 } from './math';
import { bench } from './bench';
export interface View { w: number; h: number; hour?: number; cut?: number; look?: V3; yaw?: number; pitch?: number; dist?: number; fov?: number; samples: number; method?: string; tick?: number; doors?: Record<string, number> | number; lamps?: 'auto' | 'on' | 'off'; bias?: number; scale?: number; warm?: number; jpeg?: boolean; run?: boolean; signal?: string; gain?: number; person?: number; go?: boolean; want?: Record<string, number> | number; groups?: Record<string, number> }
const methods: Record<string, () => Method> = { reference: () => pathTracer(), realtime: () => realtime(), raw: () => realtime({ name: 'raw', denoise: noDenoise }) };
export function register(name: string, m: () => Method) { methods[name] = m; }
export async function shots() {
  const lab = new Lab(); await lab.open(null); const present = new Present(lab.gpu); await present.init();
  const made = new Map<string, Method>();
  const shot = async (v: View) => {
    if (v.groups) lab.lampGroup = v.groups; else if (!v.go) lab.lampGroup = {};
    if (!v.go) lab.seek(v.tick ?? 0); lab.hour = v.hour ?? 15.2; lab.cut = lab.wantCut = v.cut ?? 1e9; lab.lampsMode = v.lamps ?? 'auto'; present.bias = v.bias ?? 0; const c = lab.camera; if (v.person !== undefined) { const q = lab.building.people[v.person]!.pos; v.look = [q[0], q[1] + 1, q[2]]; } Object.assign(c.want, { look: v.look ?? [0, 3, 0], yaw: v.yaw ?? 0.62, pitch: v.pitch ?? 0.94, dist: v.dist ?? 44 }); c.fov = v.fov ?? 0.40; c.snap();
    if (!v.go) for (const d of lab.building.doors) d.open = d.want = typeof v.doors === 'number' ? v.doors : v.doors?.[d.name] ?? 0; if (v.want !== undefined) for (const d of lab.building.doors) d.want = typeof v.want === 'number' ? v.want : v.want[d.name] ?? d.want; lab.pose();
    const s = v.scale ?? 1; lab.resize([Math.round(v.w * s), Math.round(v.h * s)], [v.w, v.h]); const name = v.method ?? 'reference'; let m = made.get(name); if (!m) made.set(name, m = methods[name]!()); if (!v.go) { await lab.use(m); lab.frameNo = 0; }
    const t0 = performance.now(); lab.graph.timer.ms.clear(); for (let i = 0; i < v.samples; i++) { if (v.run) lab.step(); present.set(lab, true); lab.draw(); if (i % 2 === 1) await lab.gpu.device.queue.onSubmittedWorkDone(); } await lab.gpu.device.queue.onSubmittedWorkDone(); const ms = (performance.now() - t0) / v.samples;
    const out = lab.gpu.device.createTexture({ size: [v.w, v.h], format: 'rgba8unorm', usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC }); const e = lab.gpu.device.createCommandEncoder(); const pre = name === 'reference' ? 'ref' : 'rt'; lab.graph.flip(); present.run(e, lab, lab.graph.texture(v.signal ?? m.result), out.createView(), 'rgba8unorm', pre, { grain: 0, sharpen: name === 'reference' ? 0 : 0.12, ...(v.signal ? { plain: v.gain ?? 1 } : {}) }); lab.graph.flip(); lab.gpu.device.queue.submit([e.finish()]);
    const { readTexture } = await import('./gpu'); const r = await readTexture(lab.gpu, out); const cv = document.createElement('canvas'); cv.width = v.w; cv.height = v.h; cv.getContext('2d')!.putImageData(new ImageData(new Uint8ClampedArray(r.data), v.w, v.h), 0, 0); out.destroy(); const gpu = Object.fromEntries([...lab.graph.timer.ms].map(([k, v]) => [k, +v.toFixed(3)])); return { png: v.jpeg ? cv.toDataURL('image/jpeg', 0.93) : cv.toDataURL('image/png'), ms, gpu, sum: +Object.values(gpu).reduce((a, b) => a + b, 0).toFixed(2), render: lab.graph.render };
  };
  const stage = (v: View) => { if (v.tick !== undefined) lab.seek(v.tick); lab.hour = v.hour ?? 15.2; lab.cut = lab.wantCut = v.cut ?? 1e9; lab.lampsMode = v.lamps ?? 'auto'; const c = lab.camera; Object.assign(c.want, { look: v.look ?? [0, 3, 0], yaw: v.yaw ?? 0.62, pitch: v.pitch ?? 0.94, dist: v.dist ?? 44 }); c.snap(); for (const d of lab.building.doors) d.open = d.want = typeof v.doors === 'number' ? v.doors : 0; lab.pose(); present.set(lab, true); };
  return { lab, shot, counts: lab.scene.counts, bench: (views: (View & { name: string })[], names: string[], o: object) => bench(lab, views, Object.fromEntries(['reference', ...names].map(n => [n, methods[n]!])), { ...o, stage }) };
}
(globalThis as never as { shots: unknown }).shots = shots;
/** does a piece of shader compile here? (for finding what a driver chokes on) */
(globalThis as never as { tryCode: unknown }).tryCode = async (bodies: [string, string][]) => {
  const lab = new Lab(); await lab.open(null); lab.graph.signal('tryOut', 'rgba16float', [64, 64]); const out: string[] = [];
  for (const [name, body] of bodies) { const t = performance.now(); try { await lab.graph.build([{ name, over: [64, 64], writes: { outT: 'tryOut' }, code: body }]); out.push(`${name} ok ${(performance.now() - t).toFixed(0)} ms`); } catch (e) { out.push(`${name} ${String(e).slice(0, 400)}`); break; } }
  return out;
};
