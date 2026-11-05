// THE LOOSE SNOW'S TEST PAGE, the half that runs in the browser (tools/qa/powder-field-page.cjs bundles this with `bun build` and loads it in a blank page on
// the software WebGPU adapter). ONLY the air solver and the powder it carries (src/render/airField.ts, powderField.ts): no game, no renderer. It takes a cell map
// from the node half, steps both under a gusting wind with scripted kicks, and hands back the read-back fields.
import { AirField, AIR_RES } from '../../src/render/airField';
import { PowderField, POWDER, POW_RES } from '../../src/render/powderField';

const WORLD_W = 40, WORLD_H = 28;
const b64 = (a: ArrayBufferView): string => { const u = new Uint8Array(a.buffer, a.byteOffset, a.byteLength); let s = ''; for (let i = 0; i < u.length; i += 0x8000) s += String.fromCharCode(...u.subarray(i, i + 0x8000)); return btoa(s); };
const unb64 = (s: string): Uint8Array => { const b = atob(s), u = new Uint8Array(b.length); for (let i = 0; i < b.length; i++) u[i] = b.charCodeAt(i); return u; };

interface RunOpts {
  /** the air's cell map, base64 of w × h bytes (255 = solid, else the wind's share × 254) */
  mask: string;
  /** the ambient wind at the ground (m/s, x and z), its gust as a share and the gust's period (s) */
  wind: [number, number]; gust?: number; gustPeriod?: number;
  steps: number; dt: number;
  /** steps (1-based, after that many) at which the powder (and the air) is kept */
  snaps?: number[];
  /** kicks: after step `at`, PowderField.kick(x, z, force, dx, dz) */
  kicks?: { at: number; x: number; z: number; force: number; dx?: number; dz?: number }[];
  /** then this many steps with the wind at 0, and the powder again */
  stillSteps?: number;
  powder?: Partial<typeof POWDER>;
  /** the BREAK path: the powder is stepped against air that stands still (the air's own step is left out) */
  noAir?: boolean;
}

let device: GPUDevice | null = null; const gpuErrors: string[] = [];
async function boot(): Promise<{ adapter: string }> {
  const ad = await navigator.gpu.requestAdapter(); if (!ad) throw new Error('no WebGPU adapter');
  device = await ad.requestDevice(); device.addEventListener('uncapturederror', e => { gpuErrors.push(String((e as GPUUncapturedErrorEvent).error?.message).slice(0, 400)); });
  const info = (ad as unknown as { info?: { vendor?: string; architecture?: string; description?: string } }).info;
  return { adapter: info ? `${info.vendor ?? ''} ${info.architecture ?? ''} ${info.description ?? ''}`.trim() : '?' };
}

async function run(o: RunOpts) {
  const dev = device!; const saved = { ...POWDER }; Object.assign(POWDER, o.powder ?? {});
  const air = new AirField(dev, WORLD_W, WORLD_H); await air.whenReady; air.setCells(unb64(o.mask));
  const pow = new PowderField(dev, air); await pow.whenReady; const view0 = pow.view;
  const gust = o.gust ?? 0, period = o.gustPeriod ?? 8;
  const windAt = (t: number): [number, number] => { const k = 1 + gust * Math.sin(2 * Math.PI * t / period); return [o.wind[0] * k, o.wind[1] * k]; };
  const snaps: { step: number; t: number; wind: number; powder: string; air: string }[] = [];
  const one = (dt: number, time: number, wind: [number, number]) => { const enc = dev.createCommandEncoder(); if (!o.noAir) air.step(enc, dt, time, wind); pow.step(enc, dt, time, Math.hypot(wind[0], wind[1])); dev.queue.submit([enc.finish()]); };
  let t = 0, ms = 0, timed = 0; const kicks = (o.kicks ?? []).slice().sort((a, b) => a.at - b.at); let ki = 0;
  for (let s = 1; s <= o.steps; s++) {
    const clock = s % 50 === 0; if (clock) await dev.queue.onSubmittedWorkDone();
    const w = windAt(t), c0 = performance.now(); one(o.dt, t, w); t += o.dt;
    if (clock) { await dev.queue.onSubmittedWorkDone(); ms += performance.now() - c0; timed++; }
    while (ki < kicks.length && kicks[ki].at <= s) { const k = kicks[ki++]; pow.kick(k.x, k.z, k.force, k.dx ?? 0, k.dz ?? 0); }
    if (o.snaps?.includes(s)) snaps.push({ step: s, t, wind: Math.hypot(w[0], w[1]), powder: b64(await pow.readBack()), air: b64(await air.readBack()) });
  }
  let still: string | null = null;
  if (o.stillSteps) { for (let s = 0; s < o.stillSteps; s++) { one(o.dt, t, [0, 0]); t += o.dt; if (s % 100 === 99) await dev.queue.onSubmittedWorkDone(); } still = b64(await pow.readBack()); }
  const out = { w: pow.w, h: pow.h, res: POW_RES, airW: air.w, airH: air.h, airRes: AIR_RES, snaps, still, msPerStep: timed ? ms / timed : 0, sameView: view0 === pow.view, kicked: pow.kicked, dropped: pow.dropped, powder: { ...POWDER }, gpuErrors: gpuErrors.slice() };
  pow.destroy(); air.destroy(); Object.assign(POWDER, saved); return out;
}
(window as unknown as { __pow: unknown }).__pow = { boot, run };
