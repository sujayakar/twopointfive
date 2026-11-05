// THE LOOK PROTOTYPE'S LOOSE SNOW ALONE (src/render/powderField.ts + shaders/powder_field.wgsl, carried by render/airField.ts's air) — a standalone check and its
// PICTURES, from headless Chromium on the software WebGPU adapter. It does NOT boot the game: tools/qa/powder-field-entry.ts is bundled with `bun build` and
// loaded in a blank page, which runs only the air solver and the powder. The scene: the game's 40 × 28 m floor, a wall across the wind with a 3 m gap, the snow
// weather's ground wind (0.4 m/s) gusting ±20 %, 40 s; then a walker's and a sprinter's kicks.
// It asserts (one PASS / FAIL line each, exit 1 on any FAIL): nothing non-finite or negative, solids hold none, the open ground upwind carries powder, the
// wall's lee carries far less than the open, the gap's jet carries more than the lee, a sprinter's kick puts in more than a walker's, the powder's own carry
// matches the air's speed there, and in still air it all settles. And it writes the pictures the look is judged by.
// Nothing here has been on a GPU: the ms per step it prints is the software adapter's and says nothing about a Mac.
//   node tools/qa/powder-field-page.cjs [--out DIR] [--powder '{"life":12}'] [--mask FILE --wind 0.33,0.12] [--secs 40]
// (--mask: a look at a level's own map, 200 × 140 bytes as game/airSim.ts airMask draws it: pictures only, no assertions. POWDERFIELD_BREAK=1 = the powder
//  stepped in air that never moves: must fail.)
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const { execFileSync } = require('child_process'); const http = require('http'), path = require('path'), fs = require('fs'), os = require('os'), zlib = require('zlib');
const ROOT = path.resolve(__dirname, '../..'); const argv = process.argv.slice(2), flag = (n, d) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : d; };
const OUT_DIR = path.resolve(flag('--out', fs.mkdtempSync(path.join(os.tmpdir(), 'powder-field-')))), POW_SET = JSON.parse(flag('--powder', '{}')), SECS = +flag('--secs', 40), MASK = flag('--mask', ''), BREAK = process.env.POWDERFIELD_BREAK === '1';
const FLAGS = ['--js-flags=--no-use-libm-trig-functions --no-use-std-math-pow', '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const DT = 1 / 60, AW = 200, AH = 140, ARES = 5;
let browser = null, srv = null; const stop = () => { try { browser && browser.close(); } catch { } try { srv && srv.close(); } catch { } browser = srv = null; };
process.on('exit', stop); for (const s of ['SIGINT', 'SIGTERM', 'SIGHUP']) process.on(s, () => { stop(); process.exit(130); });

// ---- a PNG, by hand (8-bit RGB, no filter)
const CRC = (() => { const t = new Uint32Array(256); for (let n = 0; n < 256; n++) { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; t[n] = c >>> 0; } return t; })();
const crc = b => { let c = 0xffffffff; for (let i = 0; i < b.length; i++) c = CRC[(c ^ b[i]) & 255] ^ (c >>> 8); return (c ^ 0xffffffff) >>> 0; };
const chunk = (type, data) => { const len = Buffer.alloc(4); len.writeUInt32BE(data.length); const td = Buffer.concat([Buffer.from(type, 'ascii'), data]); const c = Buffer.alloc(4); c.writeUInt32BE(crc(td)); return Buffer.concat([len, td, c]); };
function writePng(file, w, h, rgb) {
  const raw = Buffer.alloc((w * 3 + 1) * h); for (let y = 0; y < h; y++) { raw[y * (w * 3 + 1)] = 0; Buffer.from(rgb.buffer, rgb.byteOffset + y * w * 3, w * 3).copy(raw, y * (w * 3 + 1) + 1); }
  const ihdr = Buffer.alloc(13); ihdr.writeUInt32BE(w, 0); ihdr.writeUInt32BE(h, 4); ihdr[8] = 8; ihdr[9] = 2;
  fs.writeFileSync(file, Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk('IHDR', ihdr), chunk('IDAT', zlib.deflateSync(raw, { level: 6 })), chunk('IEND', Buffer.alloc(0))]));
}
const f32 = s => { const b = Buffer.from(s, 'base64'); return new Float32Array(b.buffer, b.byteOffset, b.length / 4); };

// THE POWDER FROM ABOVE, ×K: the ground dark so the powder shows (on the page it is white on white and reads by its light), what it hides = 1 − e^(−column),
// read between its cells' centres as the picture reads it. Solids grey, roofed ground a darker floor. `box` = [x0, z0, x1, z1] m: that part only.
function powderPng(file, f, w, h, res, mask, K = 3, box = null) {
  const b = box ?? [0, 0, w / res, h / res], X0 = Math.round(b[0] * res * K), Y0 = Math.round(b[1] * res * K), W = Math.round((b[2] - b[0]) * res * K), H = Math.round((b[3] - b[1]) * res * K), px = new Uint8Array(W * H * 3);
  const at = (gx, gz) => { gx = Math.max(0, Math.min(w - 1.001, gx)); gz = Math.max(0, Math.min(h - 1.001, gz)); const i = Math.floor(gx), j = Math.floor(gz), fx = gx - i, fz = gz - j, o = (j * w + i) * 4; return (f[o] * (1 - fx) + f[o + 4] * fx) * (1 - fz) + (f[o + w * 4] * (1 - fx) + f[o + w * 4 + 4] * fx) * fz; };
  for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
    const wx = (x + X0 + 0.5) / (res * K), wz = (y + Y0 + 0.5) / (res * K), m = mask[Math.min(AH - 1, Math.floor(wz * ARES)) * AW + Math.min(AW - 1, Math.floor(wx * ARES))];
    const hide = 1 - Math.exp(-Math.max(0, at(wx * res - 0.5, wz * res - 0.5))), g = m === 255 ? [120, 108, 96] : m < 127 ? [26, 26, 30] : [44, 52, 66], q = (y * W + x) * 3;
    px[q] = g[0] + (250 - g[0]) * hide; px[q + 1] = g[1] + (251 - g[1]) * hide; px[q + 2] = g[2] + (255 - g[2]) * hide;
  }
  writePng(file, W, H, px);
}
// the mean column over a box of metres (open cells only), and the worst value anywhere
function mean(f, w, res, mask, x0, z0, x1, z1, ch = 0) { let s = 0, n = 0; for (let j = Math.round(z0 * res); j < Math.round(z1 * res); j++) for (let i = Math.round(x0 * res); i < Math.round(x1 * res); i++) { if (mask[Math.floor(j / res * ARES) * AW + Math.floor(i / res * ARES)] === 255) continue; s += f[(j * w + i) * 4 + ch]; n++; } return n ? s / n : 0; }
function sane(f, w, h, res, mask) { let bad = 0, neg = 0, solid = 0, max = 0; for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) { const o = (j * w + i) * 4, a = f[o], b = f[o + 1]; if (!Number.isFinite(a + b + f[o + 2] + f[o + 3])) bad++; if (a < 0 || b < 0) neg++; if (mask[Math.floor(j / res * ARES) * AW + Math.floor(i / res * ARES)] === 255 && (a !== 0 || b !== 0)) solid++; if (a > max) max = a; } return { bad, neg, solid, max }; }

// the check's own scene: a wall across the wind at x = 14 m with a gap from z = 12.5 to 15.5
function gateMask() { const m = new Uint8Array(AW * AH).fill(254); for (let j = 0; j < AH; j++) { const z = (j + 0.5) / ARES; if (z < 12.5 || z > 15.5) m[j * AW + 14 * ARES] = 255; } return m; }

(async () => {
  fs.mkdirSync(OUT_DIR, { recursive: true });
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'powder-field-bundle-'));
  execFileSync('bun', ['build', path.join(ROOT, 'tools/qa/powder-field-entry.ts'), '--outfile', path.join(tmp, 'pow.js'), '--target', 'browser', '--loader', '.wgsl:text'], { cwd: ROOT, stdio: ['ignore', 'ignore', 'inherit'] });
  srv = http.createServer((q, r) => { if (q.url === '/pow.js') { r.writeHead(200, { 'Content-Type': 'text/javascript' }); r.end(fs.readFileSync(path.join(tmp, 'pow.js'))); } else { r.writeHead(200, { 'Content-Type': 'text/html' }); r.end('<!doctype html><meta charset="utf-8"><title>powder field</title><script type="module" src="/pow.js"></script>'); } });
  await new Promise(res => srv.listen(0, '127.0.0.1', res)); const port = srv.address().port;
  browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = await browser.newPage(); const errors = [];
  page.on('pageerror', e => { errors.push('pageerror: ' + String(e.stack || e).slice(0, 500)); log('PAGEERROR', String(e).slice(0, 300)); });
  page.on('console', m => { const x = m.text(); if (m.type() === 'error' || /WGSL/.test(x)) { errors.push(x.slice(0, 500)); log('console', x.slice(0, 600)); } });
  await page.goto(`http://127.0.0.1:${port}/`); await page.waitForFunction(() => !!window.__pow);
  const boot = await page.evaluate(() => window.__pow.boot()); log(`adapter: ${boot.adapter} · out: ${OUT_DIR}`);
  const run = async o => { const r = await page.evaluate(o => window.__pow.run(o), { powder: POW_SET, noAir: BREAK, ...o }); for (const s of r.snaps) { s.f = f32(s.powder); s.a = f32(s.air); } if (r.still) r.stillF = f32(r.still); for (const e of r.gpuErrors) if (!errors.includes(e)) errors.push(e); return r; };
  let fails = 0, checks = 0; const check = (ok, what) => { checks++; if (!ok) fails++; console.log(`${ok ? 'PASS' : 'FAIL'}  ${what}`); return ok; };
  const n = x => (Math.abs(x) >= 100 ? x.toFixed(0) : Math.abs(x) >= 10 ? x.toFixed(1) : x.toFixed(3));
  const STEPS = Math.round(SECS / DT);
  // a walker and a sprinter along +x from x = 20 m, each a step's kick: the walker's from 8 s before the end, the sprinter's from 3 s before
  const walkers = (zW, zS, x0) => { const k = []; for (let t = SECS - 8, i = 0; t < SECS - 0.2; t += 0.5, i++) k.push({ at: Math.round(t / DT), x: x0 + 1.3 * (t - (SECS - 8)), z: zW + (i & 1 ? 0.12 : -0.12), force: 0.45, dx: 1, dz: 0 });
    for (let t = SECS - 3, i = 0; t < SECS - 0.1; t += 0.28, i++) k.push({ at: Math.round(t / DT), x: x0 + 4.5 * (t - (SECS - 3)), z: zS + (i & 1 ? 0.14 : -0.14), force: 1.2, dx: 1, dz: 0 }); return k; };

  if (MASK) {   // ---- a look at a level's own map: pictures only
    const mask = new Uint8Array(fs.readFileSync(MASK)); const wind = flag('--wind', '0.33,0.12').split(',').map(Number);
    const kz = +flag('--kickz', 24.5), kx = +flag('--kickx', 6);
    const r = await run({ mask: Buffer.from(mask).toString('base64'), wind, gust: 0.2, gustPeriod: 9, steps: STEPS, dt: DT, snaps: [STEPS - 120, STEPS], kicks: walkers(kz, kz + 1.2, kx) });
    log(`${n(r.msPerStep)} ms a step (air + powder) on this adapter · kicks ${r.kicked} (${r.dropped} dropped) · POWDER ${JSON.stringify(r.powder)}`);
    r.snaps.forEach((s, k) => { powderPng(path.join(OUT_DIR, `powder_level_t${k}.png`), s.f, r.w, r.h, r.res, mask, 3); const S = sane(s.f, r.w, r.h, r.res, mask); log(`t${k}: max column ${n(S.max)}, mean over the map ${n(mean(s.f, r.w, r.res, mask, 0, 0, 40, 28))}, ${S.bad} non-finite`); });
    const last = r.snaps[r.snaps.length - 1]; for (const [name, b] of Object.entries({ road: [2, 20, 30, 28], yard: [10, 10, 34, 23] })) powderPng(path.join(OUT_DIR, `powder_level_${name}.png`), last.f, r.w, r.h, r.res, mask, 6, b);
    stop(); console.log(`pictures in ${OUT_DIR}`); if (errors.length) console.log(errors.join('\n')); console.log('=== powder-field-page: a look only, nothing asserted ==='); process.exit(errors.length ? 1 : 0);
  }

  // ---- the check: the gate scene
  const mask = gateMask(), W = 0.4;
  const r = await run({ mask: Buffer.from(mask).toString('base64'), wind: [W, 0], gust: 0.2, gustPeriod: 9, steps: STEPS, dt: DT, snaps: [STEPS - 480 - 1, STEPS], kicks: walkers(4, 8, 20), stillSteps: 900 });
  log(`${n(r.msPerStep)} ms a step (air + powder) on this adapter · kicks ${r.kicked} (${r.dropped} dropped) · POWDER ${JSON.stringify(r.powder)}`);
  r.snaps.forEach((s, k) => powderPng(path.join(OUT_DIR, `powder_gate_t${k}.png`), s.f, r.w, r.h, r.res, mask, 3));
  const before = r.snaps[0], end = r.snaps[1], f = end.f, S = sane(f, r.w, r.h, r.res, mask);
  check(S.bad === 0 && S.neg === 0, `no NaN, Inf or negative powder (${S.bad} non-finite texels, ${S.neg} negative); the largest column ${n(S.max)}`);
  check(S.solid === 0, `solid cells hold none (${S.solid} that do)`);
  check(r.sameView, '`view` is the same object after every step');
  const open = mean(f, r.w, r.res, mask, 3, 2, 12, 26), lee = mean(f, r.w, r.res, mask, 14.4, 18, 16.5, 26), jet = mean(f, r.w, r.res, mask, 14.4, 13, 18, 15);
  check(open > 0.02, `the open ground upwind of the wall carries powder: a mean column of ${n(open)} (> 0.02)`);
  check(lee < 0.35 * open, `the wall's lee (its first two metres) carries ${n(lee)}: under 35 % of the open's ${n(open)} (${(100 * lee / Math.max(open, 1e-9)).toFixed(0)} %)`);
  check(jet > 2.5 * lee, `the gap's jet carries ${n(jet)}: over 2.5 × the lee's ${n(lee)}`);
  { // the kicks: what the walker's lane and the sprinter's hold over what was there 8 s before, per kick still alive
    const lane = (z, x0, x1) => mean(f, r.w, r.res, mask, x0, z - 0.6, x1, z + 0.6) - mean(before.f, r.w, r.res, mask, x0, z - 0.6, x1, z + 0.6);
    const walk = lane(4, 26, 30.5), sprint = lane(8, 26, 34);
    check(sprint > 1.8 * walk && walk > 0, `a sprinter's kicks put in more than a walker's: ${n(sprint)} against ${n(walk)} of mean column over their lanes' last metres (over 1.8 ×)`); }
  { // the carry: the powder texel's own velocity (b, a) is the air's there × POWDER.velK
    const ds = []; let pa = 0, aa = 0; for (let j = 20; j < r.h - 20; j += 7) for (let i = 20; i < 130; i += 7) { const o = (j * r.w + i) * 4, ai = Math.floor(i / r.res * ARES), aj = Math.floor(j / r.res * ARES), ao = (aj * AW + ai) * 4; if (mask[aj * AW + ai] === 255) continue; ds.push(Math.hypot(f[o + 2] - end.a[ao] * r.powder.velK, f[o + 3] - end.a[ao + 1] * r.powder.velK)); pa += f[o + 2] * end.a[ao] + f[o + 3] * end.a[ao + 1]; aa += end.a[ao] * end.a[ao] + end.a[ao + 1] * end.a[ao + 1]; }
    ds.sort((a, b) => a - b); const cnt = ds.length, mid = ds[cnt >> 1] ?? 9, ratio = aa > 0 ? pa / aa : 0;   // (the powder reads the air BETWEEN its cells and this reads the cell: in an eddy the two differ cell by cell, so the fit over all of them and the middle difference, not the worst)
    check(cnt > 100 && Math.abs(ratio - r.powder.velK) < 0.04 && mid < 0.06, `the powder is carried at the air's own velocity where it is: over ${cnt} cells it is × ${ratio.toFixed(3)} of the air's (${r.powder.velK} ± 0.04) and half the cells differ by under ${n(mid)} m/s (< 0.06: the two grids' cells differ)`); }
  { const s = sane(r.stillF, r.w, r.h, r.res, mask), m = mean(r.stillF, r.w, r.res, mask, 0, 0, 40, 28), was = mean(f, r.w, r.res, mask, 0, 0, 40, 28);
    check(s.bad === 0 && m < 0.01 * was, `15 s of no wind: the mean column ${n(was)} → ${m.toExponential(2)} (under 1 %): it settles, and still air lifts none`); }
  stop(); console.log(`pictures in ${OUT_DIR}`); if (errors.length) console.log(errors.join('\n'));
  console.log(fails || errors.length ? `=== powder-field-page: ${fails} of ${checks} FAILED, ${errors.length} errors ===` : `=== powder-field-page: ALL ${checks} PASS ===`); process.exit(fails || errors.length ? 1 : 0);
})().catch(e => { console.error(e); stop(); process.exit(1); });
