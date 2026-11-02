// The frame capture's parts that need no GPU (src/render/framecap.ts; tools/qa/capture-read.ts): the window a target gives, the file there and back, the trace
// (a change of state is an event, a long frame is one, an armed capture starts on the change it waits for), and the reader on a made-up capture in which one
// history restarts on one frame: it must name that target and that frame, and no other.
//   bun run tools/qa/framecap-check.ts     (exit 0 = all pass; FRAMECAP_BREAK=1 makes the made-up capture's restart frame like its neighbours and must fail)
import { gzipSync } from 'node:zlib';
import { mkdtempSync, readFileSync, writeFileSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { cropRect, packCapture, unpackCapture, FrameCap, TEXEL_BYTES, type CapMeta, type CapEntry, type StateV } from '../../src/render/framecap';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.FRAMECAP_BREAK === '1';

// (1) the window
const win = { u: 0.5, v: 0.5, px: 256, whole: false };
{ const a = cropRect(3456, 2234, 3456, 2234, win), b = cropRect(1728, 1117, 3456, 2234, win), c = cropRect(576, 372, 3456, 2234, win);
  check(a[2] === 256 && a[3] === 256 && b[2] === 128 && b[3] === 128 && c[2] === 43 && Math.abs((a[0] + a[2] / 2) / 3456 - (b[0] + b[2] / 2) / 1728) < 0.001 && Math.abs((a[1] + a[3] / 2) / 2234 - (c[1] + c[3] / 2) / 372) < 0.01, `a window is the same part of the picture in a target of any size: 256 px of the output is ${b[2]} of a half-size target and ${c[2]} of a sixth-size one, about the same point`); }
{ const e = cropRect(1728, 1117, 3456, 2234, { ...win, u: 0, v: 1 }), w = cropRect(1728, 1117, 3456, 2234, { ...win, whole: true }), t = cropRect(512, 64, 3456, 2234, win), one = cropRect(1, 1, 3456, 2234, win);
  check(e[0] === 0 && e[1] + e[3] === 1117 && w.join() === '0,0,1728,1117' && t.join() === '0,0,512,64' && one.join() === '0,0,1,1', 'at the picture\'s corner the window stays inside it; the whole picture, a target of another shape and a one-texel stand-in are taken whole'); }

// (2) the file, there and back
const mkMeta = (frames: CapMeta['frames']): CapMeta => ({ version: 1, name: 'cap-check', why: 'a check', page: '/?look', adapter: 'none', out: [64, 64], window: win, frames, skipped: [{ label: 'froxels 160×90×64 rgba16float', why: 'a volume or a stack of layers' }], trace: { events: [{ n: 3, t: 100, kind: 'targets', text: '1: a → 2: histories restarted' }], first: 1, t: [0, 16, 33], dt: [0, 16, 17], cpu: [2, 2, 2] } });
const half = (x: number) => { const f = new Float32Array([x]), b = new Uint32Array(f.buffer)[0], e = ((b >> 23) & 255) - 127 + 15; return x === 0 ? 0 : ((b >>> 31) << 15) | (Math.max(1, Math.min(30, e)) << 10) | ((b >> 13) & 1023); };
const W = 16, blobs: Uint8Array[] = [], frames: CapMeta['frames'] = []; let at = 0;
for (let f = 0; f < 6; f++) {
  const entries: CapEntry[] = [], add = (label: string, format: string, fill: (i: number, out: DataView) => void) => { const bpp = TEXEL_BYTES[format], u = new Uint8Array(W * W * bpp), dv = new DataView(u.buffer); for (let i = 0; i < W * W; i++) fill(i, new DataView(u.buffer, i * bpp, bpp)); void dv; entries.push({ label, id: entries.length + 1, format, full: [32, 32], rect: [8, 8, W, W], at, bytes: u.length }); at += u.length; blobs.push(u); };
  const restart = f === 3 && !BREAK;
  add('gi', 'rgba16float', (i, o) => { const v = restart ? 0 : 0.5 + 0.001 * ((i * 7 + (i % 5 === 0 ? f : 0)) % 9); for (let c = 0; c < 3; c++) o.setUint16(c * 2, half(v * (1 + c * 0.1)), true); o.setUint16(6, half(1), true); });   // a history: a fifth of it moves a frame, and on one frame it is cleared
  add('gAlbedo', 'rgba8unorm', (i, o) => { o.setUint8(0, 40 + (i % W) * 8); o.setUint8(1, 90); o.setUint8(2, 60); o.setUint8(3, 255); });   // (still)
  add('gId', 'r32uint', (i, o) => o.setUint32(0, 1 + ((i / 64) | 0), true));
  add('gDepth', 'depth32float', (i, o) => o.setFloat32(0, 0.9 - 0.0001 * i, true));
  if (f < 4) add('vol', 'rgba16float', (i, o) => { for (let c = 0; c < 4; c++) o.setUint16(c * 2, half(0.25), true); }); else { const u = new Uint8Array(8 * 8 * 8); entries.push({ label: 'vol', id: 9, format: 'rgba16float', full: [16, 16], rect: [4, 4, 8, 8], at, bytes: u.length }); at += u.length; blobs.push(u); }   // (re-made at half the size on frame 4, and empty from there)
  frames.push({ n: 100 + f, t: 16 * f, dt: 16, cpu: 2, state: { internal: '32×32', targets: restart ? '2: histories restarted' : '1: a' }, entries });
}
const meta = mkMeta(frames), packed = packCapture(meta, blobs), back = unpackCapture(packed);
check(JSON.stringify(back.meta) === JSON.stringify(meta) && back.texels.length === at && back.texels[frames[2].entries[1].at] === blobs[2 * 5 + 1][0], `a capture packs into one file and unpacks to the same record and texels (${packed.length} bytes for 6 frames of 5 targets)`);
let threw = false; try { unpackCapture(gzipSync(packed)); } catch { threw = true; } check(threw, 'a file that is still gzipped is refused by name, not read as a capture');

// (3) the reader
const dir = mkdtempSync(join(tmpdir(), 'framecap-')), file = join(dir, 'cap-check.tpscap.gz'); writeFileSync(file, gzipSync(packed));
const run = Bun.spawnSync(['bun', 'run', join(import.meta.dir, 'capture-read.ts'), file]), outDir = join(dir, 'cap-check'), rep = existsSync(join(outDir, 'report.txt')) ? readFileSync(join(outDir, 'report.txt'), 'utf8') : '';
check(run.exitCode === 0 && ['gi', 'gAlbedo', 'gId', 'gDepth'].every(l => existsSync(join(outDir, 'strips', l + '.png'))) && existsSync(join(outDir, 'index.html')), 'the reader writes a strip of every target, a report and a page to look through');
const out = rep.slice(rep.indexOf('FRAMES THAT STAND OUT')).split('\n\n')[0].split('\n').slice(1).map(l => l.trim());
check(out.length === 2 && /^gi: f103 f104$/.test(out[0] ?? '') && /^vol: f104 f105$/.test(out[1] ?? '') && /f104 RE-MADE as 16×16 rgba16float/.test(rep), `of the made-up capture it names the history that restarts and the frames it does so on (cleared, then everything moving as it comes back), the target re-made at another size and empty from that frame on, and nothing else: ${out.join(' | ') || 'none'}`);
check(/targets: 1: a → 2: histories restarted/.test(rep) && /NOT TAKEN\n {2}froxels/.test(rep) && /frame 103: .*targets 2: histories restarted/.test(rep), 'the report carries the trace, each frame\'s state and what was not taken');

// (4) the trace, and an armed capture
{ let state: Record<string, StateV> = { scale: 1, targets: '1: a' }, saved = 0, copies = 0;
  const dev = { createCommandEncoder: () => ({ finish: () => ({}), copyTextureToBuffer: () => { copies++; } }), queue: { submit: () => {} }, createBuffer: () => ({}) } as unknown as GPUDevice;
  (globalThis as any).GPUTextureUsage ??= { COPY_SRC: 1 }; (globalThis as any).GPUBufferUsage ??= { COPY_DST: 8, MAP_READ: 1 };
  const fc = new FrameCap(dev, { state: () => state, out: () => [64, 64], adapter: 'none', save: async () => { saved++; return 'nowhere'; } });
  let t = 0; const step = (ms = 16) => { t += ms; fc.frame(t, 1); };
  for (let i = 0; i < 40; i++) step(); check(fc.events.length === 0 && fc.trace().dt.length === 40, 'forty even frames with nothing changed leave no event, and their lengths are kept');
  step(120); check(fc.events.length === 1 && fc.events[0].kind === 'long' && fc.events[0].n === 41, `a frame of 120 ms among 16s is an event (${fc.events[0]?.text})`);
  fc.arm({ frames: 2 }); for (let i = 0; i < 5; i++) step(); check(fc.waiting && !fc.active, 'an armed capture waits while nothing changes');
  state = { ...state, scale: 0.85 }; step(); const ev = fc.events[fc.events.length - 1];
  check(ev.kind === 'scale' && ev.text === '1 → 0.85' && fc.active && !fc.waiting, 'a change of any key of the renderer\'s state is an event that names it, and the armed capture starts on that frame');
  const done = async () => { for (let i = 0; i < 400 && fc.active; i++) await new Promise(r => setTimeout(r, 10)); };
  step(); await done(); check(!fc.active && saved === 1 && /2 frames/.test(fc.status), `two frames on it is done and saved once (${fc.status})`);
  { const said = fc.start({ frames: NaN as number, at: [NaN, 2] as [number, number] }); step(); step(); step(); step(); step(); step(); step(); step(); await done(); check(/8 frames/.test(said) && saved === 2 && !fc.active, 'a frame count that is no number is the usual 8, and it ends'); }
  { fc.start({ frames: 4 }); step(); const st = fc.stop(); step(); step(); step(); await done(); check(st === 'capture stopped' && saved === 2 && !fc.active, 'a capture stopped part way saves nothing and is over'); }
  fc.arm({ frames: 1 }, ['long']); state = { ...state, scale: 1 }; step(); const held = fc.waiting; step(200); await done();
  check(held && saved === 3, 'armed for a long frame only, a change of state does not start it and the long frame does'); void copies; }
console.log(red ? `=== framecap: FAILED (${red}) ===` : '=== framecap: ALL PASS ===');
process.exit(red ? 1 : 0);
