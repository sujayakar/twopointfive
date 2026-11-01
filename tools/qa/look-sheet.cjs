// THE LOOK SHEET: the look page's picture at every hour in every kind of place, as ONE contact sheet, so that a change to the day's light or exposure is looked at
// everywhere before anybody is told it is done. The maintainer, 10-31, after finding a fifth day case himself (a lamp in a windowless room that threw no shadow on a
// grey day): "overall i worry that we're finding these cases one by one and not fixing them systematically". One boot of the page in headless Chromium on the
// SOFTWARE adapter (see page-look.cjs for what that adapter can and cannot do: nothing here is seen on a GPU, and a cell is a few frames old, not converged).
// Rows are hours, columns are places on the pass, each place with Sam standing where the column says (the picture is exposed for where he stands):
//   yard       the open yard, Sam in it                    yard-in    the same view with Sam indoors (the yard seen by eyes that are in a room)
//   window     a room with windows, Sam in it              window-out that room with Sam out in the yard (a room seen by eyes that are outdoors)
//   lamp       a room with no window and one bulb, Sam in it
//   kitchen    the kitchen with its three doors open, Sam in it    lamp-open  that view with Sam in the bulb room next door (doors open: how he plays it)
// --flicker N draws N more frames of each cell and says how much 8 px blocks of the picture change between them (display levels, 0 to 255), with a map
// beside the cell's still (`<cell>_flicker.png`): one settled frame cannot show a picture that will not hold still.
// A row can carry look rows of its own after a plus: `overcast+lamps:0` is the grey day with the Real light values row at 0 (the panel's keys, window.__look),
// and `overcast+expo:0` is it with the One exposure row off (the rows it replaces): a change is drawn before and after on one boot.
//   node tools/qa/look-sheet.cjs [--out DIR] [--rows night,dawn,midday,overcast,dusk] [--views yard,yard-in,window,window-out,lamp] [--settle 3] [--w 640 --h 400]
//        [--first overcast,dusk  (rows to draw first; the sheet keeps --rows' order)] [--scale 0.5] [--quality high] [--cell 480] [--set roomc:0.6,contrast:1.35] [--eval '<js run in the page after the boot, its value is logged>'] [--pageshot FILE]
//   env: PORT = a dev server that is already up (default: this starts tools/serve.ts on a free port and stops it)
//   writes DIR/sheet.png (after every row, so a long run can be read early), DIR/cells/<row>_<view>.png, DIR/sheet.json (per cell: the hour, the exposure, how
//   bright and how wide the shown picture is) and DIR/sheet.md (the same as a table, with the cells its numbers call dark, flat or burnt out; LOOK at them all the same)
//   exit 0 = every cell drawn with no page or GPU error · 1 = an error or a cell missing · 2 = the script died
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const { spawn } = require('child_process'); const net = require('net'), path = require('path'), fs = require('fs');
const argv = process.argv.slice(2), flag = (n, d) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : d; };
const ROOT = path.resolve(__dirname, '../..'), OUT = path.resolve(flag('--out', 'look-sheet')), QUALITY = flag('--quality', 'high'), SCALE = +flag('--scale', 0.5), VW = +flag('--w', 640), VH = +flag('--h', 400);
const SETTLE = +flag('--settle', 3), CELL = +flag('--cell', 480), SET = flag('--set', ''), EVAL = flag('--eval', ''), EVAL_END = flag('--eval-end', ''), PAGESHOT = flag('--pageshot', '');
const FLICK = +flag('--flicker', 0), FLICK_EASE = argv.includes('--flicker-ease');   // --flicker N: after a cell's still, N more frames of the same stand: how much 8 px blocks of the picture change from frame to frame (the maintainer on his GPU, 11-01: "lots of temporal instability for midday") --flicker-ease: those frames run a sixtieth of a second each with the eye easing as in play (not snapped)
const HOURS = ['night', 'dawn', 'midday', 'overcast', 'dusk'];
const VIEWS = {   // levels/pass.json: the tea room has two windows on the yard (x 13 to 21, z under 11), the store (x 28 to 34, z 5 to 11) has none and one bulb at [31, 2.33, 8]
  'yard': { title: 'yard, Sam in it', look: [20, 0, 15.5], yaw: 0, pitch: 60, dist: 13, sam: [20, 0, 17.5] },
  'yard-in': { title: 'yard, Sam indoors', look: [20, 0, 15.5], yaw: 0, pitch: 60, dist: 13, sam: [16, 0, 6.6] },
  'window': { title: 'room with windows, Sam in it', look: [18.4, 0, 8.2], yaw: -90, pitch: 62, dist: 9, sam: [16, 0, 6.6] },
  'window-out': { title: 'room with windows, Sam in the yard', look: [18.4, 0, 8.2], yaw: -90, pitch: 62, dist: 9, sam: [20, 0, 17.5] },
  'kitchen': { title: 'kitchen, its three doors open, Sam in it', look: [27.5, 0, 8], yaw: 0, pitch: 60, dist: 11, sam: [26.4, 0, 8.3], doors: ['store', 'back', 'kitchen'] },   // (the maintainer's two rooms, 11-01: "easy to get into a state where the exposure is off, like moving between these two rooms")
  drums: { title: 'the bulb room, its steel drums and Sam close up, the same doors open', look: [32.3, 0, 7.4], yaw: 0, pitch: 55, dist: 6.5, sam: [31.6, 0, 8.6], doors: ['store', 'back', 'kitchen'] },
  'lamp-open': { title: 'the bulb room with its door and the kitchen doors open, Sam in it', look: [27.5, 0, 8], yaw: 0, pitch: 60, dist: 11, sam: [30.4, 0, 9.6], doors: ['store', 'back', 'kitchen'] },
  'lamp': { title: 'windowless room with a bulb, Sam in it', look: [31, 0, 8], yaw: 0, pitch: 62, dist: 8, sam: [30.4, 0, 9.6] },
};
const ROWS = flag('--rows', HOURS.join(',')).split(',').map(r => { const [hour, set] = r.split('+'); return { id: r, hour, set: set || '' }; });
const FIRST = flag('--first', '').split(',').filter(Boolean), RUN = [...FIRST.map(id => ROWS.find(r => r.id === id)).filter(Boolean), ...ROWS.filter(r => !FIRST.includes(r.id))];
const COLS = flag('--views', Object.keys(VIEWS).join(',')).split(',');
for (const r of ROWS) if (!HOURS.includes(r.hour)) { console.error(`no hour '${r.hour}' (${HOURS.join(' ')})`); process.exit(2); }
for (const c of COLS) if (!VIEWS[c]) { console.error(`no view '${c}' (${Object.keys(VIEWS).join(' ')})`); process.exit(2); }
const FLAGS = ['--js-flags=--no-use-libm-trig-functions --no-use-std-math-pow', '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const freePort = () => new Promise((res, rej) => { const s = net.createServer(); s.unref(); s.on('error', rej); s.listen(0, '127.0.0.1', () => { const p = s.address().port; s.close(() => res(p)); }); });
let srv = null, browser = null; const stop = () => { try { browser && browser.close(); } catch { } try { srv && srv.kill(); } catch { } srv = null; };
process.on('exit', stop); for (const s of ['SIGINT', 'SIGTERM', 'SIGHUP']) process.on(s, () => { stop(); process.exit(130); });
const png = (file, url) => fs.writeFileSync(file, Buffer.from(url.split(',')[1], 'base64'));
const pairs = s => Object.fromEntries(s.split(/[,/]/).filter(kv => kv && !kv.startsWith('x.')).map(kv => { const [k, v] = kv.split(':'); return [k, +v]; }));
// a row's ENGINE words, for finding where a room's light comes from (joined to its look rows with a slash: `overcast+rooms:0/x.bounce:0`):
//   x.direct:0 / x.indirect:0 / x.air:0  the picture without its direct light, its bounced light, its lit air (frame flags Direct, Indirect, Volumetrics)
//   x.lamp:0  the mains are cut for the row (every fixture on them is out): what still lights a shut room is not a lamp's
//   x.sun:0  the hour's directional light is out for the row
//   x.s.<name>:<number>  the engine setting of that name for the row (x.s.rtStrength:0 = no reflections)
//   x.restir:0  the older leader and tail direct estimator for the row
//   x.glow:0  no face glows (the frame's emissive scale at 0)
//   x.view:N  the engine's debug view N in place of the picture (3 direct light, 4 indirect light, 5 the lit air)
//   x.bounce:0  the cache's rays take no bounced light at what they hit (frame flag Bounce). The look row `rooms:0` is the cache as it was before the leaks were mended.
const words = s => Object.fromEntries(s.split(/[,/]/).filter(kv => kv.startsWith('x.')).map(kv => { const [k, v] = kv.slice(2).split(':'); return [k, +v]; }));

(async () => {
  fs.mkdirSync(path.join(OUT, 'cells'), { recursive: true });
  let port = +process.env.PORT || 0;
  if (!port) { port = await freePort(); srv = spawn('bun', ['run', 'tools/serve.ts'], { cwd: ROOT, env: { ...process.env, PORT: String(port) }, stdio: 'ignore' });
    let up = false; for (let i = 0; i < 240 && !up; i++) { try { up = (await fetch(`http://127.0.0.1:${port}/`)).ok; } catch { } if (!up) await new Promise(r => setTimeout(r, 250)); }
    if (!up) throw new Error('the dev server did not come up'); }
  log('server on', port);
  browser = await chromium.launch({ args: FLAGS });
  const errors = [], page = await browser.newPage({ viewport: { width: VW, height: VH }, deviceScaleFactor: 1 });
  page.on('pageerror', e => { errors.push('pageerror: ' + String(e.stack || e).slice(0, 400)); log('PAGEERROR', String(e).slice(0, 300)); });
  page.on('console', m => { const x = m.text(); if (m.type() !== 'error' || /Failed to load resource/.test(x)) return; errors.push(x.slice(0, 400)); log('console.error', x.slice(0, 300)); });
  const url = `http://localhost:${port}/?look&roomclear&squad4&levelFile=pass.json&nofow&seed=1&paused&mute&quality=${QUALITY}`;
  log('goto', url); await page.goto(url, { waitUntil: 'commit' });
  for (let i = 0; ; i++) { try { if (await page.evaluate(() => !!window.__ready)) break; } catch { } if (i > 1800) throw new Error('never __ready: ' + errors.join(' | ')); await new Promise(r => setTimeout(r, 500)); }
  const info = await page.evaluate(async ([SCALE, SET]) => {
    window.__testVisible = true; await window.__ready; const g = window.__game, E = window.__engine; try { window.__menu.close(); } catch { }
    window.__gpuErrs = []; E.device.addEventListener('uncapturederror', e => { window.__gpuErrs.push(String(e.error && e.error.message).slice(0, 300)); });
    window.__done = async () => { await E.device.queue.onSubmittedWorkDone(); await new Promise(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => res(); ch.port2.postMessage(0); }); };
    const real = E.render.bind(E); window.__draw = on => { E.render = on ? (c, t, dt) => real(c, t, dt, false) : () => { }; };
    // one frame read back: a canvas of it, and how bright and how wide the SHOWN picture is (the pixels decoded to light: p5 / p50 / p95 and the share burnt out or black)
    window.__grab = async () => { const fr = await E.readbackFrame(); if (!fr) return null; const d = fr.data; if (fr.bgra) for (let i = 0; i < d.length; i += 4) { const b = d[i]; d[i] = d[i + 2]; d[i + 2] = b; }
      for (let i = 3; i < d.length; i += 4) d[i] = 255;
      const lin = v => { v /= 255; return v <= 0.04045 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4); }, L = new Float32Array(d.length / 4); let burnt = 0, black = 0;
      for (let i = 0, j = 0; i < d.length; i += 4, j++) { L[j] = 0.2126 * lin(d[i]) + 0.7152 * lin(d[i + 1]) + 0.0722 * lin(d[i + 2]); if (d[i] >= 250 && d[i + 1] >= 250 && d[i + 2] >= 250) burnt++; if (d[i] <= 8 && d[i + 1] <= 8 && d[i + 2] <= 8) black++; }
      const mean = L.reduce((a, b) => a + b, 0) / L.length; L.sort(); const q = p => L[Math.min(L.length - 1, Math.floor(p * L.length))];
      const c = document.createElement('canvas'); c.width = fr.w; c.height = fr.h; c.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(d.buffer, d.byteOffset, d.length), fr.w, fr.h), 0, 0);
      const bw = Math.floor(fr.w / 8), bh = Math.floor(fr.h / 8), blocks = new Float32Array(bw * bh);   // display luma (0 to 255) of each 8 px block: what --flicker compares between frames
      for (let y = 0; y < bh * 8; y++) for (let x = 0; x < bw * 8; x++) { const i = (y * fr.w + x) * 4; blocks[(y >> 3) * bw + (x >> 3)] += (0.2126 * d[i] + 0.7152 * d[i + 1] + 0.0722 * d[i + 2]) / 64; }
      return { canvas: c, blocks, bw, bh, stats: { mean: +mean.toFixed(4), p5: +q(0.05).toFixed(4), p50: +q(0.5).toFixed(4), p95: +q(0.95).toFixed(4), burnt: +(burnt / L.length).toFixed(3), black: +(black / L.length).toFixed(3) } }; };
    window.__park = () => { const ops = (g.nightBreach && g.nightBreach.ops) || []; ops.forEach((o, i) => { o.char.pos = [4 + i * 1.2, 0, 24]; }); return ops.length; };   // (the squad out of every view, by the gate)
    window.__cells = {}; window.__draw(false); E.settings.renderScale = SCALE;
    { let s = 4242 >>> 0; Math.random = () => { s = (s + 0x6D2B79F5) >>> 0; let t = s; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
    g.godMode = true; g.aiEnabled = false;
    if (SET) window.__look.set(SET);
    window.__look0 = window.__look.get();
    for (let i = 0; i < 20; i++) window.__runFrames(1, 1 / 60);   // undrawn: poses settle
    return { pipelines: window.__boot && window.__boot.pipelines, quality: window.__quality && window.__quality.current, parked: window.__park(), look: window.__look0 };
  }, [SCALE, SET ? pairs(SET) : null]);
  log('booted', JSON.stringify(info));
  if (EVAL) log('eval', JSON.stringify(await page.evaluate(js => { try { return (0, eval)(js); } catch (e) { return 'threw: ' + String(e); } }, EVAL)));
  if (PAGESHOT) { await page.screenshot({ path: PAGESHOT }); log('page shot', PAGESHOT); }
  const D = Math.PI / 180, table = {};
  const sheet = async () => png(path.join(OUT, 'sheet.png'), await page.evaluate(([rows, cols, CELL, VW, VH, VIEWS]) => {
    const cw = CELL, ch = Math.round(CELL * VH / VW), gx = 150, gy = 44, c = document.createElement('canvas'); c.width = gx + cols.length * cw; c.height = gy + rows.length * ch;
    const x = c.getContext('2d'); x.fillStyle = '#101314'; x.fillRect(0, 0, c.width, c.height); x.fillStyle = '#dfe7e1'; x.textBaseline = 'middle';
    cols.forEach((v, i) => { x.font = '600 14px ui-monospace, Menlo, monospace'; x.fillText(VIEWS[v].title, gx + i * cw + 8, gy / 2); });
    rows.forEach((r, j) => { x.font = '600 15px ui-monospace, Menlo, monospace'; const [h, set] = r.split('+'); x.fillText(h, 10, gy + j * ch + ch / 2 - (set ? 10 : 0)); if (set) { x.font = '12px ui-monospace, Menlo, monospace'; x.fillText(set, 10, gy + j * ch + ch / 2 + 10); }
      cols.forEach((v, i) => { const cell = (window.__cells[r] || {})[v]; if (cell) x.drawImage(cell, gx + i * cw, gy + j * ch, cw, ch); else { x.strokeStyle = '#2c3432'; x.strokeRect(gx + i * cw + 0.5, gy + j * ch + 0.5, cw - 1, ch - 1); } }); });
    return c.toDataURL('image/png');
  }, [ROWS.map(r => r.id), COLS, CELL, VW, VH, VIEWS]));
  for (const row of RUN) {
    // the hour as the page's own button sets it, the row's own look numbers over the page's, then three drawn frames for the light to follow before any cell is taken
    await page.evaluate(async ([hour, set, v, D, x]) => {
      const E = window.__engine; window.__look.set(window.__look0); if (set) window.__look.set(set);
      if (window.__flags0 === undefined) window.__flags0 = E.settings.flags;
      E.settings.flags = window.__flags0 & ~((x.bounce === 0 ? 128 : 0) | (x.direct === 0 ? 1 : 0) | (x.indirect === 0 ? 2 : 0) | (x.air === 0 ? 4 : 0));
      if (window.__emis0 === undefined) window.__emis0 = E.settings.emissiveScale;
      E.settings.emissiveScale = x.glow === 0 ? 0 : window.__emis0;
      for (const l of window.__sunOff || []) l.enabled = true; window.__sunOff = [];
      if (window.__dark) { window.__game.endBlackout(); window.__dark = false; }
      window.__hour.set(hour);
      if (x.sun === 0) for (const l of E.lights.lights) if (l.kind === 2 && l.enabled) { l.enabled = false; window.__sunOff.push(l); }
      if (x.lamp === 0) { window.__game.setBlackout(3600, 'debug'); window.__dark = true; }
      window.__set0 = window.__set0 || {}; for (const k in window.__set0) E.settings[k] = window.__set0[k];
      for (const k in x) if (k.startsWith('s.')) { const n = k.slice(2); if (!(n in window.__set0)) window.__set0[n] = E.settings[n]; E.settings[n] = x[k]; }
      if (window.__restir0 === undefined) window.__restir0 = E.settings.restir;
      E.settings.restir = x.restir === 0 ? false : window.__restir0;
      E.rc.invalidateWorld(); E.settings.debugView = x.view > 0 ? x.view : 0;
      window.__stage({ look: v.look, yaw: v.yaw * D, pitch: v.pitch * D, dist: v.dist }, { player: { pos: v.sam, holstered: true } }); window.__park(); if (E.lookEye) E.lookEye.snap = true;
        if (v.doors) for (const d of window.__game.doors.list) { const o = v.doors.includes(d.def.name); d.latched = !o; d.vel = 0; d.angle = o ? (d.swing > 0 ? d.maxA : d.minA) * 0.85 : 0; }   // a view with doors: those stand open, every other one shut
      window.__draw(true); for (let i = 0; i < 3; i++) { window.__runFrames(1, 1 / 30); await window.__done(); } window.__draw(false);
    }, [row.hour, row.set ? pairs(row.set) : null, VIEWS[COLS[0]], D, words(row.set)]);
    table[row.id] = {};
    for (const col of COLS) {
      const r = await page.evaluate(async ([rowId, col, v, n, D, FLICK, FLICK_EASE]) => {
        const E = window.__engine, g = window.__game;
        window.__stage({ look: v.look, yaw: v.yaw * D, pitch: v.pitch * D, dist: v.dist }, { player: { pos: v.sam, holstered: true } }); window.__park(); if (E.lookEye) E.lookEye.snap = true;
        if (v.doors) for (const d of window.__game.doors.list) { const o = v.doors.includes(d.def.name); d.latched = !o; d.vel = 0; d.angle = o ? (d.swing > 0 ? d.maxA : d.minA) * 0.85 : 0; }   // a view with doors: those stand open, every other one shut
        window.__draw(false); window.__runFrames(2, 1 / 60); if (E.lookEye) E.lookEye.snap = true; window.__draw(true); const ms = [];
        for (let i = 0; i < n; i++) { const t0 = performance.now(); if (E.lookEye) E.lookEye.snap = true; window.__runFrames(1, 1 / 600); await window.__done(); ms.push(Math.round(performance.now() - t0)); }
        const fr = await window.__grab(); let flick = null;
        if (fr && FLICK > 0) { const B = [fr.blocks];   // the frames after the still: each block's spread over them, in display levels (0 to 255)
          const ser = [], r3 = x => typeof x === 'number' && isFinite(x) ? +x.toPrecision(3) : null, word = () => [r3(window.__game.player.irradiance), E.diceRing ? E.diceRing.mask : null];   // a frame's words: the light meter at Sam (the radiance cache there) and the steady bounce's slots held (render/diceRing.ts)
          for (let i = 0; i < FLICK; i++) { if (E.lookEye && !FLICK_EASE) E.lookEye.snap = true; window.__runFrames(1, FLICK_EASE ? 1 / 60 : 1 / 600); await window.__done(); const f2 = await window.__grab(); if (f2) { B.push(f2.blocks); let m = 0; for (const x of f2.blocks) m += x; ser.push([+(m / f2.blocks.length).toFixed(2), ...word()]); } }
          const nb = fr.blocks.length, sd = new Float32Array(nb); for (let b = 0; b < nb; b++) { let m = 0; for (const a of B) m += a[b]; m /= B.length; let q = 0; for (const a of B) q += (a[b] - m) * (a[b] - m); sd[b] = Math.sqrt(q / B.length); }
          const so = Array.from(sd).sort((a, b) => a - b), c2 = document.createElement('canvas'); c2.width = fr.bw * 8; c2.height = fr.bh * 8; const x2 = c2.getContext('2d');
          for (let b = 0; b < nb; b++) { const k = Math.min(255, Math.round(sd[b] * 20)); x2.fillStyle = `rgb(${k},${k},${k})`; x2.fillRect((b % fr.bw) * 8, Math.floor(b / fr.bw) * 8, 8, 8); }
          flick = { frames: B.length, mean: +(so.reduce((a, b) => a + b, 0) / nb).toFixed(2), p50: +so[nb >> 1].toFixed(2), p95: +so[Math.floor(nb * 0.95)].toFixed(2), max: +so[nb - 1].toFixed(1), over3: +(so.filter(v => v > 3).length / nb).toFixed(3), series: ser, url: c2.toDataURL('image/png') }; }
        window.__draw(false); if (!fr) return null; (window.__cells[rowId] ||= {})[col] = fr.canvas;
        const num = x => typeof x === 'number' && isFinite(x) ? +x.toPrecision(4) : null;
        // the place field's own words (render/placeField.ts, the One exposure row): log2 of the light of the place Sam stands in as its eye has it, of the
        // open air on this map (the cells under the sky), and of the place under the camera's look point (null = a wall's cell, or no field on this row)
        let place = null; if (E.place && E.place.ready && E.lookEyeCfg && E.lookEyeCfg[3] < 0) { const f = await E.place.readBack(), W = 100, H = 70, K = W * H, lv = x => x > -500 ? +x.toFixed(2) : null;
          const cell = p => { const i = Math.min(W - 1, Math.max(0, Math.floor(p[0] * 2.5))), j = Math.min(H - 1, Math.max(0, Math.floor(p[2] * 2.5))); return lv(f[j * W + i]); };
          place = { eye: lv(f[K]), open: lv(f[K + 2]), sam: cell(v.sam), look: cell(v.look) }; }
        // the radiance cache at the probe nearest Sam, 1.5 m up (render/rc.ts dice: 7 slabs along y): the light on a face that looks up, on one that looks
        // down, and the mean in the air: what a place's level could be made of
        let dice = null; try { const T = E.rc.dice, nx = T.width, ny = T.height / 7, nz = T.depthOrArrayLayers, bpr = Math.ceil(nx * 8 / 256) * 256, dev = E.device;
          const buf = dev.createBuffer({ size: bpr * T.height * nz, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ }), enc = dev.createCommandEncoder();
          enc.copyTextureToBuffer({ texture: T }, { buffer: buf, bytesPerRow: bpr, rowsPerImage: T.height }, [nx, T.height, nz]); dev.queue.submit([enc.finish()]); await buf.mapAsync(GPUMapMode.READ);
          const h = new Uint16Array(buf.getMappedRange().slice(0)); buf.unmap(); buf.destroy();
          const f16 = x => { const e = (x >> 10) & 31, m = x & 1023, s = x & 0x8000 ? -1 : 1; return s * (e === 0 ? m * 2 ** -24 : e === 31 ? Infinity : (1 + m / 1024) * 2 ** (e - 15)); };
          const i = Math.min(nx - 1, Math.max(0, Math.floor(v.sam[0] * nx / 40))), k = Math.min(nz - 1, Math.max(0, Math.floor(v.sam[2] * nz / 28))), j = Math.min(ny - 1, Math.floor(1.5 * ny / 3));
          const at = slab => { const q = ((k * T.height + slab * ny + j) * (bpr / 2)) + i * 4; return num(0.2126 * f16(h[q]) + 0.7152 * f16(h[q + 1]) + 0.0722 * f16(h[q + 2])); };
          dice = { up: at(2), down: at(3), mean: at(6), n: [nx, ny, nz] }; } catch (e) { dice = String(e).slice(0, 80); }
        // the lights that put light on the floor at Sam's feet, by their unshadowed strength as authored (a fixture is drawn × dayExp by day), each with its reach
        // level in his cell (0 = walled off, 'all' = it carries no mask): which lights the direct estimator weighs in this room
        const lit = (() => { const P = v.sam, ci = Math.floor(P[2]) * 40 + Math.floor(P[0]), L = E.lights, out = [];
          for (const l of L.lights) { if (!l.enabled || !(l.intensity > 0)) continue; let e;
            if (l.kind === 2) e = l.intensity * Math.max(0, -l.dir[1]);
            else { const d = [l.pos[0] - P[0], l.pos[1] - P[1], l.pos[2] - P[2]], r2 = d[0] * d[0] + d[1] * d[1] + d[2] * d[2], r = Math.sqrt(r2); if (r > l.range) continue; const q = r / l.range, w = Math.max(0, 1 - q ** 4) ** 2; e = l.intensity * w / Math.max(r2, l.radius * l.radius) * Math.max(0, d[1] / r); }
            const m = l.kind === 2 ? (L.dirMasks && l.reachDir ? l.reachDir : null) : (l.reach ?? null);
            out.push({ n: l.name, g: l.group, k: l.kind, e: num(e), reach: m ? (m[ci >> 4] >>> ((ci & 15) * 2)) & 3 : 'all' }); }
          return { restir: !!E.settings.restir, dirMasks: !!L.dirMasks, top: out.sort((a, b) => b.e - a.e).slice(0, 8) }; })();
        return { url: fr.canvas.toDataURL('image/png'), ms, gpuErrs: window.__gpuErrs.splice(0, 4), stats: fr.stats, flick,
          at: { hour: window.__hour.id, exposure: num(E.settings.exposure), samLight: num(g.player.irradiance), samVis: num(g.player.visibility), eye: E.lookEyeCfg ? Array.from(E.lookEyeCfg, num) : null, shade: E.lookShadeCfg ? Array.from(E.lookShadeCfg, num) : null, grade: E.lookGradeCfg ? Array.from(E.lookGradeCfg, num) : null, dayExp: num(E.lights.dayExp), place, dice, lit, doors: g.doors.list.map(d => d.def.name + ':' + num(d.angle)).join(' ') } };
      }, [row.id, col, VIEWS[col], SETTLE, D, FLICK, FLICK_EASE]);
      if (!r) { errors.push(`no frame for ${row.id} ${col}`); log(row.id, col, 'NO FRAME'); continue; }
      png(path.join(OUT, 'cells', `${row.id.replace(/[^a-z0-9]+/gi, '-')}_${col}.png`), r.url); delete r.url; if (r.flick) { png(path.join(OUT, 'cells', `${row.id.replace(/[^a-z0-9]+/gi, '-')}_${col}_flicker.png`), r.flick.url); delete r.flick.url; } table[row.id][col] = r;
      if (r.gpuErrs.length) errors.push(...r.gpuErrs);
      log(row.id, col, 'ms', r.ms.join(','), JSON.stringify(r.stats), r.flick ? 'FLICKER ' + JSON.stringify(r.flick) : '', JSON.stringify(r.at), r.gpuErrs.length ? 'GPU ERRORS ' + JSON.stringify(r.gpuErrs) : '');
    }
    await sheet(); fs.writeFileSync(path.join(OUT, 'sheet.json'), JSON.stringify({ rows: ROWS, views: COLS, cells: table, errors }, null, 1)); log('sheet written after', row.id);
  }
  if (EVAL_END) log('eval-end', JSON.stringify(await page.evaluate(js => { try { return (0, eval)(js); } catch (e) { return 'threw: ' + String(e); } }, EVAL_END)));   // (after the last row: the world as the last drawn frame left it)
  // the table, and what the numbers alone call out: wide = p95 over p5 of the shown light, in stops
  const stops = s => Math.log2(Math.max(s.p95, 1e-4) / Math.max(s.p5, 1e-4));
  const call = s => [s.black > 0.5 ? 'mostly black' : '', s.p50 < 0.012 && s.black <= 0.5 ? 'dark' : '', s.burnt > 0.2 ? 'burnt out' : '', stops(s) < 1.6 ? 'flat' : ''].filter(Boolean).join(', ');
  const md = ['# The look sheet', '', `Software adapter, ${QUALITY} at ${VW}×${VH} ×${SCALE}, ${SETTLE} frames a cell: not converged, never seen on a GPU.${SET ? ` Look rows set: ${SET}.` : ''}`, '',
    'A cell: the middle of the shown picture as light (0 to 1), how many stops lie between its darkest and brightest twentieth, and what those numbers call out.', '',
    `| hour | ${COLS.join(' | ')} |`, `|---|${COLS.map(() => '---').join('|')}|`,
    ...ROWS.map(r => `| ${r.id} | ${COLS.map(c => { const s = table[r.id] && table[r.id][c] && table[r.id][c].stats; return s ? `${s.p50.toFixed(3)} · ${stops(s).toFixed(1)} st${call(s) ? ` · **${call(s)}**` : ''}` : 'missing'; }).join(' | ')} |`), ''];
  fs.writeFileSync(path.join(OUT, 'sheet.md'), md.join('\n'));
  log('errors', errors.length, errors.slice(0, 3).join(' | ')); log('done →', path.join(OUT, 'sheet.png'));
  stop(); process.exit(errors.length ? 1 : 0);
})().catch(e => { console.error('died:', e); stop(); process.exit(2); });
