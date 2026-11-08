// MEASURE: methods against the reference, over fixed views. For each: milliseconds, and three errors kept apart.
//   bias   the mean of many frames, standing still, against the reference: what would never go away
//   noise  one frame against the next, standing still: what shimmers
//   error  one frame against the reference: what is seen
// All in linear light, relative to the reference's own brightness (so a dark room counts as a bright one does); medians over the picture, so that a few
// fireflies in the reference do not speak for it. The run's picture has a fingerprint: the same build, card and seed give the same one.
import { Watch } from './watch';
import { LEGS } from './tour';
import type { Lab, Method } from './lab';
import type { View } from './shot';
export interface Score { view: string; method: string; ms: number; bias: number; noise: number; error: number; hash: string; passes: Record<string, number> }
const half = (h: number) => { const s = h & 0x8000 ? -1 : 1, e = (h >> 10) & 31, m = h & 1023; return e === 0 ? s * m * 2 ** -24 : e === 31 ? 65504 * s : s * (1 + m / 1024) * 2 ** (e - 15); };
const TABLE = new Float32Array(65536); for (let i = 0; i < 65536; i++) TABLE[i] = half(i);
async function picture(lab: Lab, m: Method): Promise<Float32Array> { lab.graph.flip(); const r = await lab.read(m.result); lab.graph.flip(); const u = new Uint16Array(r.data), n = r.w * r.h, f = new Float32Array(n * 3); for (let i = 0; i < n; i++) { f[i * 3] = TABLE[u[i * 4]!]!; f[i * 3 + 1] = TABLE[u[i * 4 + 1]!]!; f[i * 3 + 2] = TABLE[u[i * 4 + 2]!]!; } return f; }
/** median over pixels of |a − b| ÷ (b + a floor), channels summed */
function apart(a: Float32Array, b: Float32Array): number { const n = a.length / 3; let mean = 0; for (let i = 0; i < b.length; i++) mean += b[i]!; const fl = mean / b.length * 0.02 + 1e-5, v = new Float32Array(n); for (let i = 0; i < n; i++) { const o = i * 3; v[i] = (Math.abs(a[o]! - b[o]!) + Math.abs(a[o + 1]! - b[o + 1]!) + Math.abs(a[o + 2]! - b[o + 2]!)) / (b[o]! + b[o + 1]! + b[o + 2]! + 3 * fl); } v.sort(); return v[n >> 1]!; }
/** a softening of both before they are compared (3 × 3), so that the reference's own grain and half a pixel of jitter are not the answer */
function soft(a: Float32Array, w: number, h: number): Float32Array { const o = new Float32Array(a.length); for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) for (let c = 0; c < 3; c++) { let s = 0, n = 0; for (let j = -1; j <= 1; j++) for (let i = -1; i <= 1; i++) { const X = x + i, Y = y + j; if (X < 0 || Y < 0 || X >= w || Y >= h) continue; s += a[(Y * w + X) * 3 + c]!; n++; } o[(y * w + x) * 3 + c] = s / n; } return o; }
const fnv = (f: Float32Array) => { const u = new Uint32Array(f.buffer); let h = 2166136261; for (let i = 0; i < u.length; i++) { const x = u[i]!; h = Math.imul(h ^ (x >>> 13), 16777619); h = Math.imul(h ^ (x >>> 22), 2246822519) ^ (h >>> 15); } return (h >>> 0).toString(16).padStart(8, '0'); };

export async function bench(lab: Lab, views: (View & { name: string })[], methods: Record<string, () => Method>, o: { reference?: number; settle?: number; mean?: number; stage: (v: View) => void }): Promise<Score[]> {
  const out: Score[] = [], made = new Map<string, Method>(), get = (n: string) => { let m = made.get(n); if (!m) made.set(n, m = methods[n]!()); return m; }, idle = () => lab.gpu.device.queue.onSubmittedWorkDone();
  for (const v of views) {
    o.stage(v); lab.resize([Math.round(v.w * (v.scale ?? 1)), Math.round(v.h * (v.scale ?? 1))], [v.w, v.h]);
    const ref = get('reference'); await lab.use(ref); lab.frameNo = 0; for (let i = 0; i < (o.reference ?? 400); i++) { lab.draw(); if (i % 4 === 3) await idle(); } await idle(); const truth = soft(await picture(lab, ref), v.w, v.h);
    for (const name of Object.keys(methods)) {
      if (name === 'reference') continue; const m = get(name); await lab.use(m); lab.frameNo = 0; lab.graph.timer.ms.clear(); for (let i = 0; i < (o.settle ?? 240); i++) { lab.draw(); if (i % 4 === 3) await idle(); } await idle();
      const one = await picture(lab, m), sum = new Float32Array(one.length), N = o.mean ?? 24; let last = one, noise = 0; sum.set(one);
      for (let i = 1; i < N; i++) { lab.draw(); await idle(); const p = await picture(lab, m); if (i <= 4) noise += apart(p, last) / 4; last = p; for (let k = 0; k < p.length; k++) sum[k]! += p[k]!; } for (let k = 0; k < sum.length; k++) sum[k]! /= N;
      const passes = Object.fromEntries([...lab.graph.timer.ms].map(([k, x]) => [k, +x.toFixed(3)]));
      out.push({ view: v.name, method: name, ms: +Object.values(passes).reduce((a, b) => a + b, 0).toFixed(2), bias: +apart(soft(sum, v.w, v.h), truth).toFixed(4), noise: +noise.toFixed(4), error: +apart(soft(one, v.w, v.h), truth).toFixed(4), hash: fnv(one), passes });
    }
  }
  return out;
}
/** everything a denoiser or an upscaler could learn from at this view: the noisy signals, what is known exactly, the reference; as named float planes */
export async function capture(lab: Lab, method: Method, reference: Method, paths: number): Promise<{ name: string; w: number; h: number; format: string; data: ArrayBuffer }[]> {
  const out: { name: string; w: number; h: number; format: string; data: ArrayBuffer }[] = []; await lab.use(method); lab.draw(); lab.graph.flip(); for (const s of ['surf', 'aux', 'emit', 'front', 'vis', 'lampU', 'bounce', 'mirror', 'color', method.result]) out.push({ name: s, ...await lab.read(s) }); lab.graph.flip();
  await lab.use(reference); for (let i = 0; i < paths; i++) { lab.draw(); if (i % 4 === 3) await lab.gpu.device.queue.onSubmittedWorkDone(); } out.push({ name: 'reference', ...await lab.read(reference.result) }); return out;
}

/** poison every history with NaN and Inf, then see how many frames until the picture holds none, and that it comes back to what it was */
export async function poison(lab: Lab, m: Method): Promise<{ frames: number; apart: number }> {
  await lab.use(m); for (let i = 0; i < (lab.graph.output[0] > 200 ? 400 : 60); i++) { lab.draw(); if (i % 4 === 3) await lab.gpu.device.queue.onSubmittedWorkDone(); } const before0 = await picture(lab, m), d = lab.gpu.device; for (let i = 0; i < (lab.graph.output[0] > 200 ? 1200 : 30); i++) { lab.draw(); if (i % 4 === 3) await d.queue.onSubmittedWorkDone(); } const before = await picture(lab, m), drift = `${(apart(before0, before) * 100).toFixed(2)} %`;
  for (const [name, s] of lab.graph.signals) { if (!s.history || s.format !== 'rgba16float') continue; for (const ref of [name, name + '@prev']) { const t = lab.graph.texture(ref), bad = new Uint16Array(t.width * t.height * 4); for (let i = 0; i < bad.length; i++) bad[i] = i % 3 ? 0x7e00 : 0x7c00; d.queue.writeTexture({ texture: t }, bad, { bytesPerRow: t.width * 8 }, [t.width, t.height]); } }
  { const c = lab.graph.buffers.get('cells'); if (c) { const now = new Float32Array(await lab.readBuffer('cells')); for (let i = 0; i < now.length; i += 8) if (now[i + 3]! > 0) { now[i] = NaN; now[i + 1] = Infinity; now[i + 3] = NaN; now[i + 7] = NaN; } d.queue.writeBuffer(c, 0, now); d.queue.writeBuffer(lab.graph.buffers.get('fresh')!, 0, now); } const e = lab.graph.buffers.get('exposure'); if (e) d.queue.writeBuffer(e, 0, new Float32Array([NaN, NaN, NaN, NaN])); }
  // two copies of the question: how far the picture is from what it was at these frames, and how far an unharmed one drifts in as many
  const big = lab.graph.output[0] > 200, at = big ? [100, 400, 1200] : [10, 30], far: string[] = []; let frames = -1, last = 1;
  for (let i = 1; i <= at[at.length - 1]!; i++) { lab.draw(); await d.queue.onSubmittedWorkDone(); if (frames < 0 && i <= 12) { const p = await picture(lab, m); if (!p.some(x => Math.abs(x) > 65000)) frames = i; } if (at.includes(i)) { last = apart(await picture(lab, m), before); far.push(`${i}: ${(last * 100).toFixed(2)} %`); } }
  return { frames, apart: last, far, drift } as never;
}

/** the same run every time: the tour's ticks from the first, at a fixed size. Mean GPU time of each pass, and the picture's fingerprint at four ticks */
export async function run(lab: Lab, m: Method, ticks: number, script: (lab: Lab) => void, say: (t: string, done: number) => void, show?: (e: GPUCommandEncoder, result: GPUTexture) => void): Promise<string> {
  await lab.use(m); lab.frameNo = 0; const sum = new Map<string, number>(), marks: string[] = []; let n = 0, worst = 0;
  for (let i = 1; i <= ticks; i++) {
    script(lab); lab.step(); lab.draw(show); await lab.gpu.device.queue.onSubmittedWorkDone(); if (show && i % 10 === 0) say(`tick ${i} of ${ticks}`, i / ticks); if (i > 60) { n++; let t = 0; for (const [k, v] of lab.graph.timer.last) { sum.set(k, (sum.get(k) ?? 0) + v); t += v; } worst = Math.max(worst, t); }
    if (i % Math.floor(ticks / 4) === 0) { marks.push(fnv(await picture(lab, m))); }
  }
  // the same frames with the GPU never left idle (three in flight, nothing shown): what a frame truly costs, where clocks drop between frames or passes overlap
  const legs: string[] = []; for (const [k, l] of LEGS.entries()) {
    const c = lab.camera; lab.level = l.level; lab.cut = lab.wantCut = lab.building.levels[l.level]!.cut; Object.assign(c.want, { look: [...l.look], dist: l.dist }); c.snap(); say(`back to back ${k + 1} of ${LEGS.length}`, 1);
    let t0 = 0; const N = Math.min(150, ticks), warm = N >> 2, flying: Promise<undefined>[] = []; for (let i = 0; i < N; i++) { if (i === warm) { await Promise.all(flying.splice(0)); t0 = performance.now(); } lab.step(); lab.draw(); flying.push(lab.gpu.device.queue.onSubmittedWorkDone()); if (flying.length > 2) await flying.shift(); } await Promise.all(flying); legs.push(`${l.name} ${((performance.now() - t0) / (N - warm)).toFixed(2)}`);
  }
  const total = [...sum.values()].reduce((a, b) => a + b, 0) / Math.max(1, n); return `${lab.gpu.info}\n${lab.graph.render.join('x')} to ${lab.graph.output.join('x')}, ${ticks} ticks\nGPU mean ${total.toFixed(2)} ms, worst ${worst.toFixed(2)} (one frame at a time)\nback to back, ms a frame: ${legs.join(', ')}\n${[...sum].sort((a, b) => b[1] - a[1]).map(([k, v]) => k.padEnd(16) + (v / n).toFixed(3)).join('\n')}\npicture ${marks.join(' ')}`;
}

/** every signal's fingerprint at each of the first ticks of the fixed run: where two runs first part is what does not repeat */
export async function trace(lab: Lab, m: Method, ticks: number, script: (lab: Lab) => void): Promise<string[]> {
  await lab.use(m); lab.frameNo = 0; const out: string[] = [], h8 = (b: ArrayBuffer) => { const u = new Uint32Array(b, 0, b.byteLength >> 2); let h = 2166136261; for (let i = 0; i < u.length; i++) { const x = u[i]!; h = Math.imul(h ^ (x >>> 13), 16777619); h = Math.imul(h ^ x, 2246822519) ^ (h >>> 15); } return (h >>> 0).toString(16).padStart(8, '0'); };
  const table = async () => { const k = new Uint32Array(await lab.readBuffer('keys')), c = new Uint32Array(await lab.readBuffer('cells')), n = new Uint32Array(await lab.readBuffer('norms')), ix: number[] = []; for (let i = 0; i < k.length; i++) if (k[i]) ix.push(i); ix.sort((a, b) => k[a]! - k[b]!); const o = new Uint32Array(ix.length * 10); ix.forEach((s, i) => { o[i * 10] = k[s]!; o[i * 10 + 1] = n[s]!; for (let q = 0; q < 8; q++) o[i * 10 + 2 + q] = c[s * 8 + q]!; }); return `${ix.length}:${h8(o.buffer)}`; };
  const { readBuffer } = await import('./gpu'), say = async (t: string) => { out.push(`${t} table ${await table()}`); for (const [n, b] of [['pieces', lab.scene.pieces], ['nodes', lab.scene.nodes], ['insts', lab.scene.insts], ['lamps', lab.scene.lamps], ['frame', lab.scene.frame]] as [string, GPUBuffer][]) out.push(`${t} ${n} ${h8(await readBuffer(lab.gpu, b))}`); };
  await say('0');
  for (let i = 1; i <= ticks; i++) { script(lab); lab.step(); lab.draw(); await lab.gpu.device.queue.onSubmittedWorkDone(); await say(String(i)); lab.graph.flip(); for (const s of ['id', 'idClear', 'surf', 'aux', 'emit', 'front', 'vis', 'lampU', 'bounce', 'mirror', 'air', 'visH', 'bounceH', 'mirrorH', 'airH', 'color', m.result]) out.push(`${i} ${s} ${h8((await lab.read(s)).data)}`); lab.graph.flip(); out.push(`${i} exposure ${h8(await lab.readBuffer('exposure'))}`); }
  return out;
}

/** the fixed run with every write looked at (guards off), in clear weather and then in rain; stops at the first bad number */
export async function watched(lab: Lab, m: Method, ticks: number, script: (lab: Lab) => void): Promise<string> {
  const w = await new Watch(lab.graph).init(); lab.graph.watch = w; await lab.use(m); lab.frameNo = 0;
  for (let i = 1; i <= ticks; i++) { if (i === Math.floor(ticks / 2)) lab.rain = 1; script(lab); lab.step(); lab.draw(); if ((await w.read()).length) break; } lab.graph.watch = null; return w.say();
}
/** a stage broken for one frame (graph.spoil), with the guards as in play: bad numbers in the picture at that frame and the ones after, and bad pixels left in any history */
export async function spoilt(lab: Lab, m: Method, w: number, h: number): Promise<{ picture: number[]; after: number }> {
  lab.resize([w, h], [w, h]); await lab.use(m); lab.frameNo = 0; const at = lab.graph.spoil!.frame, seen: number[] = [], count = (d: ArrayBuffer) => { const u = new Uint16Array(d); let k = 0; for (let i = 0; i < u.length; i++) if ((u[i]! & 0x7c00) === 0x7c00) k++; return k; };
  for (let i = 0; i <= at + 3; i++) { lab.step(); lab.draw(); await lab.gpu.device.queue.onSubmittedWorkDone(); if (i >= at) { lab.graph.flip(); seen.push(count((await lab.read(m.result)).data)); lab.graph.flip(); } }
  let after = 0; for (const [n, s] of lab.graph.signals) if (s.format === 'rgba16float' && s.history) after += count((await lab.read(n + '@prev')).data); return { picture: seen, after };
}

// ---- what shimmers, and what is wrong while the camera moves; each by what the pixel shows
export const KINDS = ['matte', 'shiny', 'clear', 'leaves'] as const;
/** what each output pixel shows: 0 matte, 1 shiny, 2 through a clear layer, 3 leaves, 255 sky */
async function kindsOf(lab: Lab, w: number, h: number): Promise<Uint8Array> {
  lab.graph.flip(); const s = await lab.read('surf'), f = await lab.read('front'); lab.graph.flip(); const S = new Uint32Array(s.data), T = new Float32Array(s.data), F = new Uint32Array(f.data), o = new Uint8Array(w * h);
  for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) { const i = 4 * (Math.min(s.h - 1, Math.floor((y + 0.5) * s.h / h)) * s.w + Math.min(s.w - 1, Math.floor((x + 0.5) * s.w / w))); const rough = (S[i + 2]! >>> 24) / 255, metal = ((S[i + 3]! >>> 16) & 15) > 7; o[y * w + x] = F[i] ? 2 : T[i]! > 1.5e3 ? 255 : (S[i + 3]! >>> 26) & 1 ? 3 : rough < 0.3 || metal ? 1 : 0; }
  return o;
}
export interface ByKind { [kind: string]: { share: number; mean: number; p99: number } }
function byKind(v: Float32Array, k: Uint8Array): ByKind { const out: ByKind = {}; for (let c = -1; c < KINDS.length; c++) { const l: number[] = []; for (let i = 0; i < v.length; i++) if (c < 0 ? k[i] !== 255 : k[i] === c) l.push(v[i]!); if (!l.length) continue; const s = Float32Array.from(l).sort(); let m = 0; for (const x of l) m += x; out[c < 0 ? 'all' : KINDS[c]!] = { share: +(l.length / v.length).toFixed(3), mean: +(m / l.length).toFixed(3), p99: +s[Math.floor(s.length * 0.99)]!.toFixed(2) }; } return out; }
/** nothing moves but the dice: how much each pixel of the shown picture (0‥255) changes about its mean, over 32 frames */
export async function flicker(lab: Lab, m: Method, shown: () => Promise<Uint8ClampedArray>, w: number, h: number, settle = 240, N = 32): Promise<{ by: ByKind; map: Float32Array }> {
  await lab.use(m); lab.frameNo = 0; for (let i = 0; i < settle; i++) { lab.draw(); if (i % 4 === 3) await lab.gpu.device.queue.onSubmittedWorkDone(); }
  const a = new Float32Array(w * h), b = new Float32Array(w * h);  for (let f = 0; f < N; f++) { lab.draw(); const p = await shown(); for (let i = 0; i < a.length; i++) { const l = 0.2126 * p[i * 4]! + 0.7152 * p[i * 4 + 1]! + 0.0722 * p[i * 4 + 2]!; a[i]! += l; b[i]! += l * l; } }
  for (let i = 0; i < a.length; i++) { const u = a[i]! / N; b[i] = Math.sqrt(Math.max(0, b[i]! / N - u * u)); } return { by: byKind(b, await kindsOf(lab, w, h)), map: b };
}
/** the camera has been turning for 90 frames: that frame against the reference from where it stands; and the same once it has stood still for 120 */
export async function moving(lab: Lab, m: Method, ref: Method, w: number, h: number, paths = 400, quick = false): Promise<{ moving: ByKind; still: ByKind; maps: Float32Array[] }> {
  const idle = () => lab.gpu.device.queue.onSubmittedWorkDone(), c = lab.camera; await lab.use(m); lab.frameNo = 0; const yaw = c.want.yaw; c.want.yaw = yaw - 0.36; c.snap(); for (let i = 0; i < (quick ? 4 : 120); i++) { lab.draw(); if (i % 4 === 3) await idle(); }
  for (let i = 0; i < (quick ? 4 : 90); i++) { c.want.yaw += 0.004; c.snap(); lab.draw(); if (i % 4 === 3) await idle(); } const a = await picture(lab, m), kinds = await kindsOf(lab, w, h); for (let i = 0; i < (quick ? 4 : 120); i++) { lab.draw(); if (i % 4 === 3) await idle(); } const s = await picture(lab, m);
  await lab.use(ref); lab.frameNo = 0; for (let i = 0; i < paths; i++) { lab.draw(); if (i % 4 === 3) await idle(); } const t = soft(await picture(lab, ref), w, h); c.want.yaw = yaw; c.snap();
  let mean = 0; for (const x of t) mean += x; const fl = mean / t.length * 0.02 + 1e-5, off = (q: Float32Array) => { const p = soft(q, w, h), v = new Float32Array(w * h); for (let i = 0; i < v.length; i++) { const o = i * 3; v[i] = Math.min(100, 100 * (Math.abs(p[o]! - t[o]!) + Math.abs(p[o + 1]! - t[o + 1]!) + Math.abs(p[o + 2]! - t[o + 2]!)) / (t[o]! + t[o + 1]! + t[o + 2]! + 3 * fl)); } return v; };
  const A = off(a), S = off(s); return { moving: byKind(A, kinds), still: byKind(S, kinds), maps: [A, S] };
}
