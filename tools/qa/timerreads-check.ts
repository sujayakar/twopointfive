// THE GPU'S TIME IS READ BACK A FRAME IN FIVE, AND EVERY FRAME WHILE SOMEBODY ASKS (src/gpu/timer.ts): frames of a made-up GPU, whose passes run every frame, every second, every
// fourth and every eighth, read through the real timer; each pass's mean held to its own rate and time, however few frames are read.
//   bun run tools/qa/timerreads-check.ts   (exit 0 = all pass; TIMERREADS_BREAK=1 = nobody who asks is heard: must fail; =2 = a frame in four is read: must fail)
import { GpuTimer } from '../../src/gpu/timer';

const BREAK = process.env.TIMERREADS_BREAK ?? '';
let red = 0; const check = (ok: boolean, w: string): void => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const G = globalThis as Record<string, unknown>; G.GPUBufferUsage = { QUERY_RESOLVE: 1, COPY_SRC: 2, COPY_DST: 4, MAP_READ: 8 }; G.GPUMapMode = { READ: 1 };
/** a queue that writes a pass's two stamps where the pass was told to, `ms` long, one after another */
function gpu() {
  const stamps = new BigInt64Array(128); let clock = 1000n, maps = 0, resolves = 0;
  type Buf = { d: BigInt64Array; mapAsync(): Promise<void>; getMappedRange(): ArrayBuffer; unmap(): void };
  const buf = (): Buf => ({ d: new BigInt64Array(128), mapAsync: async () => { maps++; }, getMappedRange() { return this.d.buffer.slice(0) as ArrayBuffer; }, unmap() { /* as it was */ } });
  const device = { createQuerySet: () => ({}), createBuffer: buf } as unknown as GPUDevice;
  const enc = { resolveQuerySet: () => { resolves++; }, copyBufferToBuffer: (_a: unknown, _b: number, to: Buf, _c: number, bytes: number) => { to.d.set(stamps.subarray(0, bytes / 8)); } } as unknown as GPUCommandEncoder;
  return { device, enc, run(at: GPUComputePassTimestampWrites | undefined, ms: number) { const b = clock; clock += BigInt(Math.round(ms * 1e6)); if (at) { stamps[at.beginningOfPassWriteIndex!] = b; stamps[at.endOfPassWriteIndex!] = clock; } }, gap() { clock += 2_000_000n; }, get maps() { return maps; }, get resolves() { return resolves; } };
}
const PASSES = [{ n: 'each', every: 1, ms: 4 }, { n: 'half', every: 2, ms: 2 }, { n: 'quarter', every: 4, ms: 3 }, { n: 'eighth', every: 8, ms: 5 }];
/** `frames` frames, `asked` of each whether somebody asks; what each reading said */
async function play(frames: number, asked: (i: number) => boolean, fps = 60) {
  let ms = 0; const g = gpu(), t = new GpuTimer(g.device, true, () => ms), seen: { at: number; passes: Map<string, number> }[] = []; let was = 0, stamped = 0;
  for (let i = 0; i < frames; i++) {
    ms = i * 1000 / fps; if (asked(i) && BREAK !== '1') t.ask();
    t.beginFrame(); if (BREAK === '2') (t as unknown as { reads: boolean }).reads = i % 4 === 0;
    for (const p of PASSES) { if (i % p.every) { t.idle(p.n); continue; } const at = t.pass(p.n); if (at) stamped++; g.run(at, p.ms); }
    t.resolve(g.enc); t.afterSubmit(); g.gap(); await Promise.resolve(); await Promise.resolve(); await Promise.resolve();
    if (t.resolved !== was) { was = t.resolved; seen.push({ at: i, passes: new Map(t.lastFrame) }); }
  }
  return { g, t, seen, stamped };
}
{ // nobody asks
  const r = await play(1000, () => false), late = r.seen.filter(s => s.at >= 100), means = PASSES.map(p => late.reduce((a, s) => a + (s.passes.get(p.n) ?? 0), 0) / late.length), want = PASSES.map(p => p.ms / p.every);
  check(late.length === 180 && r.g.maps === r.t.resolved && r.g.resolves === r.t.resolved, `nobody asks: ${late.length} of 900 frames read back, and no other is resolved, mapped or stamped (${r.stamped} stamps)`);
  check(means.every((m, k) => Math.abs(m - want[k]) < 0.06 * want[k]), `a pass that runs every second, fourth or eighth frame reads as what it costs a frame: ${means.map((m, k) => `${PASSES[k].n} ${m.toFixed(2)} of ${want[k].toFixed(2)}`).join(', ')}`);
  check(late.every(s => PASSES.every(p => s.passes.has(p.n))) && r.t.ranThisFrame('each') && PASSES.every(p => r.t.results.has(p.n)), 'a pass is known to run on the frames that are not read, and a resting one reads 0');
}
{ // somebody asks, from frame 300 to 500
  const r = await play(700, i => i >= 300 && i < 500), during = r.seen.filter(s => s.at >= 302 && s.at < 500).length, after = r.seen.filter(s => s.at >= 560).length;
  check(during === 198 && after === 28, `while somebody asks every frame is read (${during} of 198), and a half second after the last asking it is a frame in five again (${after} of 140)`);
}
{ // a slow asker, on a fast display: every 400 ms
  const r = await play(1200, i => i % 48 === 0, 120), n = r.seen.filter(s => s.at >= 100).length; check(n === 1100, `one who asks every 400 ms on a 120 Hz display has every frame (${n} of 1100)`);
}
for (const fps of [2, 4, 8, 12, 20, 25, 30, 45, 144]) { // a slow page and a fast one
  const r = await play(400, () => false, fps), from = 100, late = r.seen.filter(s => s.at >= from), want = PASSES.map(p => p.ms / p.every), means = PASSES.map(p => late.reduce((a, s) => a + (s.passes.get(p.n) ?? 0), 0) / late.length), gaps = late.slice(1).map((s, k) => s.at - late[k].at), worst = Math.max(...gaps);
  check(worst <= 5 && (worst === 1 || worst * 1000 / fps <= 260) && means.every((m, k) => Math.abs(m - want[k]) < 0.16 * want[k]), `at ${fps} a second a reading comes at least every ${worst} frames (${(worst * 1000 / fps).toFixed(0)} ms), and the passes read as they are: ${means.map((m, k) => `${m.toFixed(2)} of ${want[k].toFixed(2)}`).join(', ')}`);
}
{ // a slot taken by a pass that then did nothing: what it holds is an older reading's
  const g = gpu(); let ms = 0; const t = new GpuTimer(g.device, true, () => ms); let ghost = 0, span = 0;
  for (let i = 0; i < 200; i++) { ms = i * 1000 / 60; t.beginFrame(); const a = t.pass('whole'), b = t.pass('empty'); g.run(a, 4); if (i < 20) g.run(b, 3); t.resolve(g.enc); t.afterSubmit(); g.gap(); await Promise.resolve(); await Promise.resolve(); await Promise.resolve();
    if (i > 40 && t.lastFrame.has('empty')) { ghost = Math.max(ghost, t.lastFrame.get('empty')!); span = Math.max(span, t.unionMs()); } }
  check(ghost === 0 && Math.abs(span - 4) < 1e-6, `a slot that holds an older reading's stamps reads as nothing (${ghost.toFixed(2)} ms, the frame ${span.toFixed(2)})`);
}
console.log(red ? `${red} FAILED` : 'ALL PASS'); process.exit(red ? 1 : 0);
