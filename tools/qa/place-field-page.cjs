// THE LOOK PROTOTYPE'S PLACE FIELD ALONE (src/render/placeField.ts + shaders/place_field.wgsl) — a standalone check and its PICTURES, from headless Chromium on
// the software WebGPU adapter. It does NOT boot the game: tools/qa/place-field-entry.ts is bundled with `bun build` and loaded in a blank page, which runs
// only the field's solve over a stand-in for the radiance cache. The scene: the game's 40 × 28 m floor at a light of 0.1, a 6 m room A whose light falls 2.6
// stops from one wall to the other with a 1 m door to the yard, and a room B behind A's far wall, a hundred times darker than the yard and shut off.
// It asserts (one PASS / FAIL line each, exit 1 on any FAIL): the shader compiles and nothing is non-finite, wall cells and only they hold no level, the yard
// has the yard's level, room A has ONE level (not its light's 2.6 stops) near the middle of its light, room B keeps its own through the shared wall, an open
// door does not move room A's level by itself, an opening wider than a doorway joins two places, the field at rest does not creep, a change of light is
// followed within seconds, the eye closes faster than it opens, a window (a sill, a pane and a lintel) is a wall, and the field's word for the open air is the
// level of the cells with no roof over them.
// Nothing here has been on a GPU: the ms it prints is the software adapter's and says nothing about a Mac.
//   node tools/qa/place-field-page.cjs [--out DIR] [--place '{"reach":4}']     (PLACEFIELD_BREAK=1 = no box is a wall: must fail)
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const { execFileSync } = require('child_process'); const http = require('http'), path = require('path'), fs = require('fs'), os = require('os'), zlib = require('zlib');
const ROOT = path.resolve(__dirname, '../..'); const argv = process.argv.slice(2), flag = (n, d) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : d; };
const OUT_DIR = path.resolve(flag('--out', fs.mkdtempSync(path.join(os.tmpdir(), 'place-field-')))), SET = JSON.parse(flag('--place', '{}')), BREAK = process.env.PLACEFIELD_BREAK === '1';
const FLAGS = ['--js-flags=--no-use-libm-trig-functions --no-use-std-math-pow', '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
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
const f32 = s => { const b = Buffer.from(s, 'base64'); return new Float32Array(b.buffer.slice(b.byteOffset, b.byteOffset + b.length)); };
const u8 = s => new Uint8Array(Buffer.from(s, 'base64'));
/** the field from above, ×K: a level as a grey (−11 stops black, −3 white), walls brown, a cell with no answer red */
function fieldPng(file, f, mask, w, h, K = 6) {
  const px = new Uint8Array(w * K * h * K * 3);
  for (let y = 0; y < h * K; y++) for (let x = 0; x < w * K; x++) { const i = Math.floor(y / K) * w + Math.floor(x / K), q = (y * w * K + x) * 3, v = f[i];
    const g = mask[i] === 255 ? [120, 92, 60] : !(v > -500) ? [200, 30, 30] : Array(3).fill(Math.round(255 * Math.min(1, Math.max(0, (v + 11) / 8))));
    px[q] = g[0]; px[q + 1] = g[1]; px[q + 2] = g[2]; }
  writePng(file, w * K, h * K, px);
}

(async () => {
  fs.mkdirSync(OUT_DIR, { recursive: true });
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'place-field-bundle-'));
  execFileSync('bun', ['build', path.join(ROOT, 'tools/qa/place-field-entry.ts'), '--outfile', path.join(tmp, 'place.js'), '--target', 'browser', '--loader', '.wgsl:text'], { cwd: ROOT, stdio: ['ignore', 'ignore', 'inherit'] });
  srv = http.createServer((q, r) => { if (q.url === '/place.js') { r.writeHead(200, { 'Content-Type': 'text/javascript' }); r.end(fs.readFileSync(path.join(tmp, 'place.js'))); } else { r.writeHead(200, { 'Content-Type': 'text/html' }); r.end('<!doctype html><meta charset=utf8><script src="/place.js"></script>'); } });
  await new Promise(res => srv.listen(0, '127.0.0.1', res)); const port = srv.address().port;
  browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = await browser.newPage(); const errors = [];
  page.on('pageerror', e => { errors.push('pageerror: ' + String(e.stack || e).slice(0, 500)); log('PAGEERROR', String(e).slice(0, 300)); });
  page.on('console', m => { const x = m.text(); if (m.type() === 'error' || /WGSL/.test(x)) { errors.push(x.slice(0, 500)); log('console', x.slice(0, 600)); } });
  await page.goto(`http://127.0.0.1:${port}/`); await page.waitForFunction(() => !!window.__place);
  const boot = await page.evaluate(() => window.__place.boot()), L = await page.evaluate(() => window.__place.light); log(`adapter: ${boot.adapter} · out: ${OUT_DIR}`);
  const run = async o => { const r = await page.evaluate(o => window.__place.run(o), { place: SET, noWalls: BREAK, frames: 240, ...o }); r.f = f32(r.rest); r.g = f32(r.later); r.m = u8(r.mask); for (const e of r.gpuErrors) if (!errors.includes(e)) errors.push(e); return r; };
  let fails = 0, checks = 0; const check = (ok, what) => { checks++; if (!ok) fails++; console.log(`${ok ? 'PASS' : 'FAIL'}  ${what}`); return ok; };
  const n = x => x.toFixed(2), lg = Math.log2;
  /** over the free cells whose centres lie in a box of metres (a doorway's cells apart: they hold no level): the least, the most and the mean level */
  const over = (r, f, x0, z0, x1, z1) => { let lo = 1e9, hi = -1e9, s = 0, c = 0; for (let j = 0; j < r.h; j++) for (let i = 0; i < r.w; i++) { const x = r.min[0] + (i + 0.5) / r.res, z = r.min[1] + (j + 0.5) / r.res, v = f[j * r.w + i]; if (x < x0 || x > x1 || z < z0 || z > z1 || r.m[j * r.w + i] === 255 || (r.m[j * r.w + i] & 3) === 1) continue; lo = Math.min(lo, v); hi = Math.max(hi, v); s += v; c++; } return { lo, hi, mean: s / Math.max(c, 1), span: hi - lo, n: c }; };
  const A = [10.2, 8.2, 15.8, 13.8], B = [16.2, 8.2, 21.8, 13.8], FAR = [26, 16, 38, 26], YARD = lg(L.YARD), MID = (lg(L.A_NEAR) + lg(L.A_FAR)) / 2, SPAN = lg(L.A_NEAR / L.A_FAR);

  const open = await run({ door: 'open' }), shut = await run({ door: 'shut' }), wide = await run({ door: 'wide' }), win = await run({ door: 'shut', window: true });
  for (const [k, r] of Object.entries({ open, shut, wide })) fieldPng(path.join(OUT_DIR, `place_${k}.png`), r.f, r.m, r.w, r.h);
  log(`solved to rest in ${open.msSettle.toFixed(0)} ms, a drawn frame's sweeps ${open.msFrame.toFixed(1)} ms on this adapter · maps drawn ${open.drawn} · PLACE ${JSON.stringify(open.place)}`);
  { let bad = 0, walls = 0, doorways = 0, mism = 0; for (const r of [open, shut, wide]) for (let i = 0; i < r.w * r.h; i++) { const v = r.f[i]; if (!Number.isFinite(v)) bad++; if (r.m[i] === 255) walls++; else if ((r.m[i] & 3) === 1) doorways++; if ((r.m[i] === 255 || (r.m[i] & 3) === 1) !== !(v > -500)) mism++; }
    check(errors.length === 0 && bad === 0 && open.f[open.w * open.h + 1] === 1, `the shader compiles, the field says it is live and nothing in it is non-finite (${errors.length} errors, ${bad} non-finite cells)`);
    check(walls > 300 && doorways > 0 && doorways < 20 && mism === 0, `a wall's cell and a doorway's hold no level and every other free cell holds one (${walls} wall cells and ${doorways} of a doorway over three scenes, ${mism} cells the wrong way)`); }
  const ya = over(open, open.f, ...FAR), a = over(open, open.f, ...A), b = over(open, open.f, ...B), as = over(shut, shut.f, ...A), aw = over(wide, wide.f, ...A);
  check(Math.abs(ya.lo - YARD) < 0.05 && Math.abs(ya.hi - YARD) < 0.05, `the open yard holds the yard's light: ${n(ya.lo)} to ${n(ya.hi)} stops against ${n(YARD)}`);
  const deep = over(open, open.f, 10.2, 8.2, 15.8, 12);
  check(as.span < 0.4 && deep.span < 0.4, `room A has one level: ${n(as.span)} stops from its brightest cell to its darkest with the door shut and ${n(deep.span)} with it open (two metres in from the door), both under 0.4, where its light spans ${n(SPAN)}`);
  check(Math.abs(as.mean - MID) < 0.35, `shut off, room A's level is the middle of its own light: ${n(as.mean)} against ${n(MID)} (within 0.35), with one of its probes holding nothing`);
  check(Math.abs(b.mean - lg(L.B_ROOM)) < 0.15 && b.span < 0.15, `room B, behind A's wall and shut off, keeps its own: ${n(b.mean)} against ${n(lg(L.B_ROOM))}, ${n(b.span)} across`);
  check(Math.abs(a.mean - as.mean) < 0.02, `an open door does not move room A's level by itself: ${n(a.mean - as.mean)} of the ${n(YARD - MID)} stops between it and the yard (no level is pulled through a doorway; what a room takes from the yard is the light that comes in, which is the cache's to say and is the same in both scenes here)`);
  check(aw.mean - as.mean > a.mean - as.mean + 0.3, `an opening wider than a doorway joins the two more: room A up by ${n(aw.mean - as.mean)} stops`);
  { const step = over(open, open.f, 12.6, 14.6, 13.4, 15.4), yardBy = over(open, open.f, 12, 17.5, 14, 19);
    check(Math.abs(yardBy.mean - YARD) < 0.25, `two metres out of the door the yard is the yard: ${n(yardBy.mean)} against ${n(YARD)} (the step sits at the threshold: ${n(step.mean)} just outside it)`); }
  { let creep = 0; for (let i = 0; i < open.w * open.h; i++) if (open.m[i] !== 255) creep = Math.max(creep, Math.abs(open.g[i] - open.f[i]));
    check(creep < 0.02, `at rest the field stays where it is: after 240 more frames no cell has moved ${creep.toFixed(4)} stops or more (under 0.02)`); }
  { const want = as.mean + lg(0.1), was = as.mean, got = shut.follow.map(v => (v - was) / (want - was));
    check(got[0] > 0.3 && got[1] > 0.9 && got[1] < 1.1, `room A's light drops to a tenth: the level has gone ${(100 * got[0]).toFixed(0)} % of the way after half a second and ${(100 * got[1]).toFixed(0)} % after three`); }
  { const c = open.closing, o = open.opening, fc = (c[1] - c[0]) / (c[2] - c[0]), fo = (o[1] - o[0]) / (o[2] - o[0]);
    check(Math.abs(c[0] - a.mean) < 0.3 && Math.abs(c[2] - YARD) < 0.1 && Math.abs(o[2] - c[0]) < 0.05, `the eye takes Sam's place outright when told to (${n(c[0])} in room A) and ends at the place he walks to (${n(c[2])} in the yard, ${n(o[2])} back in the room)`);
    check(fc > fo + 0.1 && fc > 0.5 && fc < 0.75 && fo > 0.2 && fo < 0.45, `it closes faster than it opens: 0.35 s after he steps out it has gone ${(100 * fc).toFixed(0)} % of the way, 0.35 s after he steps in ${(100 * fo).toFixed(0)} %`); }
  { const awn = over(win, win.f, ...A); let hole = 0; for (let j = 0; j < win.h; j++) { const x = 10, z = win.min[1] + (j + 0.5) / win.res; if (z > 8 && z < 14 && win.m[j * win.w + Math.floor((x - win.min[0]) * win.res)] !== 255) hole++; }
    check(hole === 0 && Math.abs(awn.mean - as.mean) < 0.05 && awn.span < 0.4, `a window in room A's wall (a sill, a pane and a lintel, none a wall's height) leaves no hole in the map, and the room keeps its level: ${n(awn.mean)} against ${n(as.mean)} without it`);
    let sky = 0, roofed = 0; for (let i = 0; i < open.w * open.h; i++) { if (open.m[i] === 255) continue; if (open.m[i] & 4) sky++; else roofed++; }
    check(sky > 4000 && roofed > 300 && roofed < 500 && Math.abs(open.sky - YARD) < 0.05 && Math.abs(open.mid - YARD) < 0.05 && open.mid >= open.sky - 1e-6 && Math.abs(shut.sky - YARD) < 0.05 && Math.abs(wide.sky - YARD) < 0.3, `the two rooms are under a roof (${roofed} cells) and the field's word for the open air is what the cells under the sky hold: ${n(open.sky)} against the yard's ${n(YARD)} (${n(wide.sky)} with the wide opening letting the room's dark out into the yard)`); }
  { // THE OPEN AIR IS COUNTED BY EVERY CELL AT ONCE (shaders/place_field.wgsl placeSky: a count and a whole-number sum a bin, and the brightest place, which the
    // eye's one invocation reads): its two words against a plain walk of the same field, cell by cell, as the eye itself once walked it. On the yard at one
    // level, on a yard whose light falls six stops across the map (many bins, and a low word well under the middle), and under one roof (no cell under the
    // sky: the brightest place). The walk bins a level as the shader does, in floats. Then the whole light drops two stops with nothing told to the field:
    // 240 frames on the eased words are the walk's of the field as it then stands, and the table itself, read back between two frames, is empty.
    const fr = Math.fround, walk = (r, f = r.f) => { const hist = Array(64).fill(0), sum = Array(64).fill(0); let nSky = 0, top = -1000;
      for (let i = 0; i < r.w * r.h; i++) { const m = r.m[i], l = f[i]; if (m === 255 || !(l > -500)) continue; if ((m & 3) === 2) top = Math.max(top, l);
        if (m & 4) { const b = Math.min(63, Math.max(0, Math.floor(fr(fr(l + 12) * 4)))); hist[b]++; sum[b] += l; nSky++; } }
      let lo = top, mid = top;
      if (nSky) { const want = Math.max(Math.ceil(fr(fr(nSky) * fr(r.place.skyShare))), 1), half = Math.max(Math.ceil(nSky * 0.5), 1); let acc = 0, b0 = -1, b1 = -1;
        for (let b = 0; b < 64; b++) { acc += hist[b]; if (b0 < 0 && acc >= want) b0 = b; if (b1 < 0 && acc >= half) b1 = b; }
        if (b0 >= 0) lo = sum[b0] / hist[b0]; if (b1 >= 0) mid = b0 >= 0 ? Math.max(lo, sum[b1] / hist[b1]) : sum[b1] / hist[b1]; }
      return { lo, mid, nSky, bins: hist.filter(c => c > 0).length }; };
    const fall = await run({ door: 'open', yardFall: 6, brief: true }), roof = await run({ door: 'open', roofAll: true, brief: true }), all = [open, shut, wide, win, fall, roof], K = open.w * open.h;
    let worst = 0, later = 0; for (const r of all) { const w = walk(r); worst = Math.max(worst, Math.abs(r.sky - w.lo), Math.abs(r.mid - w.mid)); }
    for (const r of [fall, roof]) { const w = walk(r, r.g); later = Math.max(later, Math.abs(r.g[K + 2] - w.lo), Math.abs(r.g[K + 3] - w.mid)); }
    const wf = walk(fall), wr = walk(roof), dropF = walk(fall, fall.g).lo - wf.lo, dropR = walk(roof, roof.g).lo - wr.lo, empty = all.every(r => r.table.length === 129 && r.table.every(v => v === 0));
    check(worst < 1e-4 && wf.bins > 12 && fall.mid - fall.sky > 1 && wr.nSky === 0 && roof.sky === wr.lo && roof.mid === wr.lo && later < 0.02 && dropF < -1.8 && dropF > -2.2 && dropR < -1.8 && dropR > -2.2 && empty,
      `the open air's two words are what a plain walk of the field gives, to ${worst.toExponential(1)} of a stop over six scenes (under 1e-4): a yard whose light falls six stops across the map fills ${wf.bins} bins and holds ${n(fall.sky)} at its low word and ${n(fall.mid)} at its middle; under one roof no cell is under the sky and both words are the brightest place's, ${n(roof.sky)}; with the whole light two stops down (${n(dropF)} and ${n(dropR)} by the walk) the words 240 frames on are the walk's of that field to ${later.toFixed(4)} (under 0.02), and the table is empty between frames in all six`); }
  { // THE SECOND BLOCK, the brightest place a stride away (PLACE.near, PLACE.nearDoor; shaders/place_field.wgsl hand; what the Light in order row bounds a
    // pixel's own light by): after the cells and their four words, a number a cell again
    const N2 = open.w * open.h + 4, cellOf = (r, x, z) => Math.floor((z - r.min[1]) * r.res) * r.w + Math.floor((x - r.min[0]) * r.res), lv = (r, x, z) => r.f[cellOf(r, x, z)], nr = (r, x, z) => r.f[N2 + cellOf(r, x, z)];
    let under = 0, own = 0, cells = 0; for (const r of [open, shut, wide]) for (let i = 0; i < r.w * r.h; i++) { if (r.m[i] === 255) continue; if (!(r.f[N2 + i] >= r.f[i] - 1e-3)) under++; }
    for (let j = 0; j < shut.h; j++) for (let i = 0; i < shut.w; i++) { const x = shut.min[0] + (i + 0.5) / shut.res, z = shut.min[1] + (j + 0.5) / shut.res, k = j * shut.w + i; if (shut.m[k] === 255 || x < A[0] || x > B[2] || z < A[1] || z > A[3]) continue; cells++; if (Math.abs(shut.f[N2 + k] - shut.f[k]) < 1e-3) own++; }
    const d0 = nr(open, 13, 13.6), d1 = nr(open, 13, 12.4), d2 = nr(open, 13, 10.6), bOpen = Math.abs(nr(open, 19, 11) - lv(open, 19, 11)), far = nr(open, 30, 20) - lv(open, 30, 20);
    check(open.f.length >= 2 * open.w * open.h + 4 && under === 0 && own === cells && cells > 300 && d0 > YARD - 1 && d0 > d1 && d1 > d2 && d2 >= lv(open, 13, 10.6) && d0 - d1 > 0.5 && bOpen < 1e-3 && Math.abs(far) < 0.1,
      `the brightest place a stride away: never under a cell's own level (${under} cells); with the door shut every cell of both rooms holds its own (${own} of ${cells}); with it open the cell inside the door holds ${n(d0)} against the yard's ${n(YARD)} and it falls into the room (${n(d1)} at 1.6 m, ${n(d2)} at 3.4 m, over the room's ${n(lv(open, 13, 10.6))}); room B behind the wall holds its own (off by ${bOpen.toFixed(4)}); the far yard is the yard's (${n(far)} over its level)`); }
  { // A DOOR OPENS BY DEGREES (scene/placePlan.ts placeMask `steps`; shaders/place_field.wgsl hand and doorShut): the same room with the map told its
    // door's line, the leaf at four angles. The door's line is two cells deep here (the wall's middle lies on a cell boundary)
    const N2 = open.w * open.h + 4, K = open.w * open.h, cellOf = (r, x, z) => Math.floor((z - r.min[1]) * r.res) * r.w + Math.floor((x - r.min[0]) * r.res), STEPS = open.door.steps, PER_STOP = open.door.perStop;
    const shutD = shut, a29 = await run({ door: 'ajar', swing: 0.5 }), a46 = await run({ door: 'ajar', swing: 0.8 }), openD = open;
    fieldPng(path.join(OUT_DIR, 'place_ajar.png'), a29.f, a29.m, a29.w, a29.h);
    const stepOf = m => (m !== 255 && (m & 3) === 1 ? (m >> 3) & 31 : 0), shutOf = m => (stepOf(m) ? (STEPS - stepOf(m)) / PER_STOP : 0);
    const door = r => { let lo = 99, hi = 0, n = 0; for (const z of [13.8, 14.2]) for (let x = 12.7; x < 13.5; x += 0.4) { const m = r.m[cellOf(r, x, z)]; if (m === 255) continue; n++; lo = Math.min(lo, stepOf(m)); hi = Math.max(hi, stepOf(m)); } return { lo, hi, n }; };
    const sS = door(shutD), s29 = door(a29), s46 = door(a46), sO = door(openD); let wallsMoved = 0; for (let i = 0; i < K; i++) if ((a29.m[i] === 255) !== (a46.m[i] === 255) || (a29.m[i] === 255) !== (openD.m[i] === 255)) wallsMoved++;
    check(sS.n === 0 && s29.n >= 4 && s29.lo === s29.hi && s46.lo === s46.hi && sO.lo === sO.hi && s29.lo >= 1 && s29.lo < s46.lo && s46.lo < sO.lo && sO.lo === STEPS && wallsMoved === 0,
      `by degrees a shut door's line is wall, and the doorway's cells say how open the door is: step ${s29.lo} of ${STEPS} with the leaf at 29 degrees, ${s46.lo} at 46, ${sO.lo} swung clear (a doorway whole); between those three no wall cell of the map moved (${wallsMoved})`);
    // the second block as the rule says it, from the levels and the map each run handed back: a cell keeps the most of its own level and what each free
    // neighbour hands it, PLACE.near stops dimmer, or across a doorway's cell PLACE.nearDoor, and ONCE, on the hand out of a door's line, the stops its
    // door stands under wide open
    const model = r => { const lvl = r.g, m = r.m, W = r.w, H = r.h, near = new Float32Array(K); for (let i = 0; i < K; i++) near[i] = m[i] === 255 ? -1000 : lvl[i];
      for (let pass = 0, moved = true; moved && pass < 400; pass++) { moved = false;
        for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) { const k = j * W + i; if (m[k] === 255) continue; let v = near[k];
          for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) { const x = i + di, z = j + dj; if (x < 0 || z < 0 || x >= W || z >= H) continue; const q = z * W + x; if (m[q] === 255) continue;
            const isDoor = (m[k] & 3) === 1 || (m[q] & 3) === 1;
            v = Math.max(v, near[q] - (isDoor ? open.place.nearDoor + Math.max(0, shutOf(m[q]) - shutOf(m[k])) : open.place.near)); }
          if (v > near[k] + 1e-6) { near[k] = v; moved = true; } } }
      let worst = 0; for (let i = 0; i < K; i++) if (m[i] !== 255) worst = Math.max(worst, Math.abs(near[i] - r.g[N2 + i])); return worst; };
    const off = [open, shut, wide, a29, a46].map(model);
    check(off.every(d => d < 0.02), `the brightest place a stride away is the rule's own answer in every cell of all five scenes, the two with a door ajar among them (the furthest any cell is from it: ${off.map(d => d.toFixed(4)).join(', ')} stops)`);
    // THE RULE AS IT IS SAID, not as it is coded: the room's first cell inside the door is held to what a wide door gives it, less a stop for each halving
    // of the door's open share, once, though the hand crosses three cells' edges here
    const nr = (r, x, z) => r.g[N2 + cellOf(r, x, z)], mean = r => over(r, r.f, ...A).mean, inD = [a29, a46, openD].map(r => nr(r, 13, 13.4)), want = [s29.lo, s46.lo].map(st => inD[2] - (STEPS - st) / PER_STOP);
    check(Math.abs(inD[0] - want[0]) < 0.05 && Math.abs(inD[1] - want[1]) < 0.05 && inD[0] > a29.g[cellOf(a29, 13, 13.4)] + 0.3
      && Math.abs(mean(a29) - mean(shutD)) < 0.02 && Math.abs(mean(a46) - mean(shutD)) < 0.02 && Math.abs(mean(openD) - mean(shutD)) < 0.02 && over(a29, a29.f, 10.2, 8.2, 15.8, 12).span < 0.4,
      `what the room takes from the yard comes in with the swing: the first cell of the room inside the door is held to ${n(inD[2])} with the door swung clear, ${n(inD[1])} at 46 degrees (the rule: ${n(want[1])}) and ${n(inD[0])} at 29 (the rule: ${n(want[0])}; the room's own level there is ${n(a29.g[cellOf(a29, 13, 13.4)])}); room A's own level goes ${n(mean(shutD))}, ${n(mean(a29))}, ${n(mean(a46))}, ${n(mean(openD))} from shut to clear (its own, whatever the door does: the swing moves what the room is held to, not the room's level), one level across at every step`); }
  stop(); console.log(`pictures in ${OUT_DIR}`); if (errors.length) console.log(errors.join('\n'));
  console.log(fails || errors.length ? `=== place-field-page: ${fails} of ${checks} FAILED, ${errors.length} errors ===` : `=== place-field-page: ALL ${checks} PASS ===`); process.exit(fails || errors.length ? 1 : 0);
})().catch(e => { console.error(e); stop(); process.exit(1); });
