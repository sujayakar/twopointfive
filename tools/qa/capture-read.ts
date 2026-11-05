// Reads a frame capture (src/render/framecap.ts; F6 on the page, or `__framecap.start()` / `.arm()`): bun run tools/qa/capture-read.ts <file.tpscap.gz> [--out <dir>]
// (a capture handed over in pieces, `<name>.part1of6.tpscap.gz` …: name any one of them, with the others beside it: tools/qa/capture-pieces.ts)
// Writes, beside the file (or under --out): report.txt (the trace, each frame, and for every target what moved from frame to frame), strips/<target>.png (the
// target's window over the frames, left to right) and index.html (the same, to look through in a browser).
//
// How a target is shown: a colour target is exposed so its 98th percentile over the run sits near white, then gamma; a one-channel target is stretched between its
// 1st and 99th percentile; a two-channel one is red and green about zero; an integer one is a colour per value. The numbers in the report are the raw ones.
// Per frame: `moved` = the share of texels that differ from the frame before, `by` = how far they moved as a share of the target's level (noise is a few
// hundredths, a history that restarts is most of 1), `zero` = the share that are all zero (a cleared target), `mean` / `max` of the value or the luminance, and a count of NaN or infinite texels.
import { gunzipSync } from 'node:zlib';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { captureName, readCapture } from './capture-pieces';
import { unpackCapture, TEXEL_BYTES, type CapEntry, type CapMeta } from '../../src/render/framecap';
import { png } from './kit-figure-sheet';

const args = process.argv.slice(2), file = args.find(a => !a.startsWith('--'));
if (!file) { console.error('usage: bun run tools/qa/capture-read.ts <file.tpscap.gz> [--out <dir>]'); process.exit(2); }
const outAt = args.indexOf('--out'), OUT = outAt >= 0 ? args[outAt + 1] : join(dirname(file), captureName(file));
const bytes = readCapture(file), raw = bytes[0] === 0x1f && bytes[1] === 0x8b ? new Uint8Array(gunzipSync(bytes)) : bytes;
const { meta, texels } = unpackCapture(raw);
mkdirSync(join(OUT, 'strips'), { recursive: true });

const half = (h: number) => { const e = (h >> 10) & 31, m = h & 1023, s = h & 0x8000 ? -1 : 1; return e === 0 ? s * m * 2 ** -24 : e === 31 ? (m ? NaN : s * Infinity) : s * (1 + m / 1024) * 2 ** (e - 15); };
const uf = (bits: number, mb: number) => { const e = bits >> mb, m = bits & ((1 << mb) - 1); return e === 0 ? m / (1 << mb) * 2 ** -14 : e === 31 ? (m ? NaN : Infinity) : (1 + m / (1 << mb)) * 2 ** (e - 15); };
type Kind = 'colour' | 'scalar' | 'pair' | 'ints' | 'unorm';
interface Dec { kind: Kind; ch: number; v: Float32Array | null; w: number; h: number; u8: Uint8Array }
/** an entry's texels as floats, `ch` a texel (null for an integer target: those are shown and compared by their bytes) */
function decode(e: CapEntry): Dec {
  const [, , w, h] = e.rect, n = w * h, u8 = texels.subarray(e.at, e.at + e.bytes), dv = new DataView(u8.buffer, u8.byteOffset, u8.byteLength), f = e.format;
  const mk = (kind: Kind, ch: number, get: (i: number, c: number) => number): Dec => { const v = new Float32Array(n * ch); for (let i = 0; i < n; i++) for (let c = 0; c < ch; c++) v[i * ch + c] = get(i, c); return { kind, ch, v, w, h, u8 }; };
  if (f === 'rgba16float') return mk('colour', 4, (i, c) => half(dv.getUint16(i * 8 + c * 2, true)));
  if (f === 'rgba32float') return mk('colour', 4, (i, c) => dv.getFloat32(i * 16 + c * 4, true));
  if (f === 'rg11b10ufloat') return mk('colour', 3, (i, c) => { const p = dv.getUint32(i * 4, true); return c === 0 ? uf(p & 2047, 6) : c === 1 ? uf((p >> 11) & 2047, 6) : uf((p >> 22) & 1023, 5); });
  if (f === 'rgb9e5ufloat') return mk('colour', 3, (i, c) => { const p = dv.getUint32(i * 4, true); return ((p >> (9 * c)) & 511) * 2 ** ((p >>> 27) - 24); });
  if (f === 'r16float') return mk('scalar', 1, i => half(dv.getUint16(i * 2, true)));
  if (f === 'r32float' || f === 'depth32float') return mk('scalar', 1, i => dv.getFloat32(i * 4, true));
  if (f === 'rg16float') return mk('pair', 2, (i, c) => half(dv.getUint16(i * 4 + c * 2, true)));
  if (f === 'rg32float') return mk('pair', 2, (i, c) => dv.getFloat32(i * 8 + c * 4, true));
  if (f === 'r8unorm') return mk('unorm', 1, i => u8[i] / 255);
  if (f === 'rg8unorm') return mk('unorm', 2, (i, c) => u8[i * 2 + c] / 255);
  if (f === 'rgba8unorm' || f === 'rgba8unorm-srgb') return mk('unorm', 4, (i, c) => u8[i * 4 + c] / 255);
  if (f === 'bgra8unorm' || f === 'bgra8unorm-srgb') return mk('unorm', 4, (i, c) => u8[i * 4 + (c === 0 ? 2 : c === 2 ? 0 : c)] / 255);
  if (f === 'rgb10a2unorm') return mk('unorm', 4, (i, c) => { const p = dv.getUint32(i * 4, true); return c < 3 ? ((p >> (10 * c)) & 1023) / 1023 : (p >>> 30) / 3; });
  return { kind: 'ints', ch: 0, v: null, w, h, u8 };
}
const lum = (d: Dec, i: number) => { const v = d.v!, c = d.ch; return d.kind === 'colour' || (d.kind === 'unorm' && c >= 3) ? 0.2126 * v[i * c] + 0.7152 * v[i * c + 1] + 0.0722 * v[i * c + 2] : d.kind === 'pair' || c === 2 ? Math.hypot(v[i * c], v[i * c + 1]) : v[i * c]; };
const pct = (xs: number[], p: number) => { if (!xs.length) return 0; const s = xs.slice().sort((a, b) => a - b); return s[Math.min(s.length - 1, Math.max(0, Math.floor(p * (s.length - 1))))]; };

const EVERY = 'EVERY TARGET (moved = share of texels that differ from the frame before · by = how far, as a share of the level · zero = share all zero · ⚑ = a frame that stands out from the target\'s others)';
interface Row { n: number; remade: string; moved: number; by: number; zero: number; mean: number; max: number; bad: number }
const labels = [...new Set(meta.frames.flatMap(f => f.entries.map(e => e.label)))].sort((a, b) => a.localeCompare(b));
const report: string[] = [], html: string[] = [], esc = (s: string) => s.replace(/[&<>]/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;' }[c]!));
const m: CapMeta = meta;
report.push(`capture ${m.name} · ${m.why}`, `page ${m.page}`, `adapter ${m.adapter}`, `picture ${m.out[0]}×${m.out[1]} · window ${m.window.whole ? 'the whole picture' : `${m.window.px} px at (${m.window.u.toFixed(3)}, ${m.window.v.toFixed(3)})`} · ${m.frames.length} frames · ${labels.length} targets`, '');
report.push('WHAT THE RENDERER CHANGED (frame, ms on the page\'s clock, what)');
for (const e of m.trace.events) report.push(`  ${String(e.n).padStart(7)}  ${String(e.t).padStart(9)}  ${e.kind}: ${e.text}`);
if (!m.trace.events.length) report.push('  nothing since the page opened');
{ const dt = m.trace.dt.filter(x => x > 0); report.push('', `FRAME LENGTHS, the last ${dt.length} frames: median ${pct(dt, 0.5).toFixed(1)} ms · 95th ${pct(dt, 0.95).toFixed(1)} · longest ${Math.max(0, ...dt).toFixed(1)} · the renderer's CPU side, median ${pct(m.trace.cpu, 0.5).toFixed(1)} ms`); }
report.push('', 'THE CAPTURED FRAMES');
m.frames.forEach((f, i) => { const was = i ? m.frames[i - 1].state : null, ch = Object.entries(f.state).filter(([k, v]) => !was || was[k] !== v);   // (the first frame's state whole, then what changed)
  report.push(`  frame ${f.n}: ${f.dt} ms, CPU ${f.cpu} ms${ch.length ? ' · ' + ch.map(([k, v]) => `${k} ${v}`).join(' · ') : ''}`); });
{ const g = m.frames[0]?.gpu; if (g && Object.keys(g).length) report.push('', `GPU PASSES at the first frame (ms, smoothed): ${Object.entries(g).sort((a, b) => b[1] - a[1]).map(([k, v]) => `${k} ${v}`).join(' · ')}`); }
report.push('', EVERY);

html.push(`<!doctype html><meta charset=utf8><title>${esc(m.name)}</title><style>body{font:13px ui-monospace,Menlo,monospace;background:#16181c;color:#d8dbe0;margin:16px}img{image-rendering:pixelated;display:block;margin:4px 0;max-width:100%}h3{margin:18px 0 2px}small{color:#8b93a0}pre{white-space:pre-wrap}.f{color:#ffb454}</style>`);
const flagged: string[] = [];
for (const label of labels) {
  const es = m.frames.map(f => f.entries.find(e => e.label === label) ?? null), first = es.find(Boolean)!;
  const decs = es.map(e => e ? decode(e) : null), d0 = decs.find(Boolean)!;
  // how it is shown: one mapping for the run, so a change of brightness between frames is seen
  const sample: number[] = []; for (const d of decs) if (d?.v) { const n = d.w * d.h, step = Math.max(1, Math.floor(n / 4000)); for (let i = 0; i < n; i += step) { const x = lum(d, i); if (Number.isFinite(x)) sample.push(x); } }
  const lo = d0.kind === 'scalar' ? pct(sample, 0.01) : 0, hi = Math.max(pct(sample, d0.kind === 'scalar' ? 0.99 : 0.98), lo + 1e-9);
  const cw = Math.max(...es.map(e => e ? e.rect[2] : 0)), chh = Math.max(...es.map(e => e ? e.rect[3] : 0));   // (the cell is the run's largest window: a target re-made at another size mid-run is drawn into it, and said)
  const k = Math.max(1, Math.min(8, Math.ceil(128 / chh))), W = cw * k, H = chh * k, gap = 3, SW = W * es.length + gap * (es.length - 1), strip = new Uint8Array(SW * H * 4);
  const rows: Row[] = []; let prev: Dec | null = null;
  es.forEach((e, fi) => {
    const d = decs[fi]; if (!e || !d) { prev = null; return; }
    const remade = !!prev && (prev.w !== d.w || prev.h !== d.h || prev.u8.length !== d.u8.length); if (remade) prev = null;   // (another size or format than the frame before: nothing to compare with, and a thing to say)
    const px: number[] = new Array(d.w * d.h * 3);
    const n = d.w * d.h, bpp = TEXEL_BYTES[e.format]; let moved = 0, zero = 0, sum = 0, max = 0, bad = 0, cnt = 0, dif = 0, base = 0;
    for (let i = 0; i < n; i++) {
      let z = true, mv = false; for (let b = 0; b < bpp; b++) { const x = d.u8[i * bpp + b]; if (x) z = false; if (prev && prev.u8.length === d.u8.length && prev.u8[i * bpp + b] !== x) mv = true; }
      if (z) zero++; if (mv) moved++;
      let r = 0, g = 0, bl = 0;
      if (d.v) {
        const x = lum(d, i); if (Number.isFinite(x)) { sum += x; cnt++; if (x > max) max = x; } else bad++;
        if (prev?.v && prev.v.length === d.v.length) { const p = lum(prev, i); if (Number.isFinite(x) && Number.isFinite(p)) { dif += Math.abs(x - p); base += Math.max(Math.abs(x), Math.abs(p)); } }
        const c = d.ch, v = d.v;
        if (d.kind === 'colour') { const s = 0.8 / hi, sh = (x: number) => Number.isNaN(x) ? NaN : Math.max(0, x * s) ** (1 / 2.2); r = sh(v[i * c]); g = sh(v[i * c + 1]); bl = sh(v[i * c + 2]); }   // (a negative channel is black, not a NaN's magenta)
        else if (d.kind === 'scalar') { r = g = bl = (v[i] - lo) / (hi - lo); }
        else if (d.kind === 'pair') { r = 0.5 + 0.5 * v[i * 2] / hi; g = 0.5 + 0.5 * v[i * 2 + 1] / hi; bl = 0.5; }
        else { r = v[i * c]; g = c > 1 ? v[i * c + 1] : r; bl = c > 2 ? v[i * c + 2] : c === 1 ? r : 0; }
      } else { let hsh = 2166136261; for (let b = 0; b < bpp; b++) hsh = Math.imul(hsh ^ d.u8[i * bpp + b], 16777619); if (!z) { r = ((hsh >>> 8) & 255) / 255 * 0.8 + 0.2; g = ((hsh >>> 16) & 255) / 255 * 0.8 + 0.2; bl = (hsh >>> 24) / 255 * 0.8 + 0.2; } }
      const R = Math.max(0, Math.min(255, Math.round((Number.isFinite(r) ? r : 1) * 255))), G = Math.max(0, Math.min(255, Math.round((Number.isFinite(g) ? g : 0) * 255))), B = Math.max(0, Math.min(255, Math.round((Number.isFinite(bl) ? bl : 1) * 255)));
      px[i * 3] = R; px[i * 3 + 1] = G; px[i * 3 + 2] = B;
    }
    for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) { const sx = Math.min(d.w - 1, Math.floor(x * d.w / W)), sy = Math.min(d.h - 1, Math.floor(y * d.h / H)), o = (y * SW + fi * (W + gap) + x) * 4, q = (sy * d.w + sx) * 3; strip[o] = px[q]; strip[o + 1] = px[q + 1]; strip[o + 2] = px[q + 2]; strip[o + 3] = 255; }
    rows.push({ n: m.frames[fi].n, remade: remade ? `${e.full[0]}×${e.full[1]} ${e.format}` : '', moved: prev ? moved / n : NaN, by: prev && d.v ? (base > 0 ? dif / base : 0) : NaN, zero: zero / n, mean: cnt ? sum / cnt : NaN, max, bad }); prev = d;
  });
  const safe = label.replace(/[^\w.#-]+/g, '_'); writeFileSync(join(OUT, 'strips', safe + '.png'), png(strip, SW, H));
  // a frame that stands out: of the frames the target was WRITTEN on (one of a pair of histories is written every other frame), it changed by far more than
  // the target's other frames do, or it went to zero, or it was re-made at another size, or (with four frames or more) its level left the others', or it holds a NaN
  const wrote = rows.filter(r => r.moved > 0 && Number.isFinite(r.by)), medBy = pct(wrote.map(r => r.by), 0.5), means = rows.map(r => r.mean).filter(Number.isFinite), medMean = pct(means, 0.5);
  const flag = (r: Row, i: number) => !!r.remade || (wrote.length >= 3 && r.moved > 0 && r.by > 3 * medBy + 0.1) || (i > 0 && r.zero > 0.9 && rows[i - 1].zero < 0.5) || (rows.length >= 4 && Number.isFinite(r.mean) && medMean > 1e-6 && (r.mean > 2 * medMean || r.mean < 0.5 * medMean)) || r.bad > 0;
  const num = (x: number, p = 2) => Number.isFinite(x) ? (Math.abs(x) >= 100 || (x !== 0 && Math.abs(x) < 0.01) ? x.toExponential(1) : x.toFixed(p)) : '–';
  const line = (r: Row, i: number) => `${flag(r, i) ? '⚑' : ' '} f${r.n}${r.remade ? ` RE-MADE as ${r.remade}` : ''} moved ${num(r.moved)}${d0.v ? ` by ${num(r.by)}` : ''} zero ${num(r.zero)}${d0.v ? ` mean ${num(r.mean, 3)} max ${num(r.max, 3)}` : ''}${r.bad ? ` NaN/inf ${r.bad}` : ''}`;
  const head = `${label} · ${first.format} · ${first.full[0]}×${first.full[1]}${first.rect[2] !== first.full[0] || first.rect[3] !== first.full[1] ? `, window ${first.rect[2]}×${first.rect[3]} at ${first.rect[0]},${first.rect[1]}` : ''}${d0.v && d0.kind !== 'unorm' ? ` · shown ${d0.kind === 'scalar' ? `${num(lo, 3)} to ${num(hi, 3)}` : `with ${num(hi, 3)} near white`}` : ''}`;
  report.push(`  ${head}`); rows.forEach((r, i) => report.push(`     ${line(r, i)}`));
  if (rows.some(flag)) flagged.push(`${label}: ${rows.filter(flag).map(r => `f${r.n}`).join(' ')}`);
  html.push(`<h3>${esc(label)}</h3><small>${esc(head)}</small><img src="strips/${encodeURIComponent(safe)}.png"><pre>${rows.map((r, i) => flag(r, i) ? `<span class=f>${esc(line(r, i))}</span>` : esc(line(r, i))).join('\n')}</pre>`);
}
report.push('', 'FRAMES THAT STAND OUT', ...(flagged.length ? flagged.map(s => '  ' + s) : ['  none']));
if (m.skipped.length) report.push('', 'NOT TAKEN', ...m.skipped.map(s => `  ${s.label}: ${s.why}`));
writeFileSync(join(OUT, 'report.txt'), report.join('\n') + '\n');
const cut = report.indexOf(EVERY);
writeFileSync(join(OUT, 'index.html'), html[0] + `<pre>${esc(report.slice(0, cut).join('\n'))}\n${esc(report.slice(report.indexOf('FRAMES THAT STAND OUT')).join('\n'))}</pre>` + html.slice(1).join('\n'));
console.log(`${m.name}: ${m.frames.length} frames, ${labels.length} targets → ${OUT}/report.txt, ${OUT}/index.html, ${OUT}/strips/`);
if (flagged.length) console.log('stand out:\n  ' + flagged.join('\n  '));
