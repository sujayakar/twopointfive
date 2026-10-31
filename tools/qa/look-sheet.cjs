// THE LOOK SHEET: the look page's picture at every hour in every kind of place, as ONE contact sheet, so that a change to the day's light or exposure is looked at
// everywhere before anybody is told it is done. The maintainer, 10-31, after finding a fifth day case himself (a lamp in a windowless room that threw no shadow on a
// grey day): "overall i worry that we're finding these cases one by one and not fixing them systematically". One boot of the page in headless Chromium on the
// SOFTWARE adapter (see page-look.cjs for what that adapter can and cannot do: nothing here is seen on a GPU, and a cell is a few frames old, not converged).
// Rows are hours, columns are places on the pass, each place with Sam standing where the column says (the picture is exposed for where he stands):
//   yard       the open yard, Sam in it                    yard-in    the same view with Sam indoors (the yard seen by eyes that are in a room)
//   window     a room with windows, Sam in it              window-out that room with Sam out in the yard (a room seen by eyes that are outdoors)
//   lamp       a room with no window and one bulb, Sam in it
// A row can carry look rows of its own after a plus: `overcast+lamps:0` is the grey day with the Real light values row at 0 (the panel's keys, window.__look).
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
const SETTLE = +flag('--settle', 3), CELL = +flag('--cell', 480), SET = flag('--set', ''), EVAL = flag('--eval', ''), PAGESHOT = flag('--pageshot', '');
const HOURS = ['night', 'dawn', 'midday', 'overcast', 'dusk'];
const VIEWS = {   // levels/pass.json: the tea room has two windows on the yard (x 13 to 21, z under 11), the store (x 28 to 34, z 5 to 11) has none and one bulb at [31, 2.33, 8]
  'yard': { title: 'yard, Sam in it', look: [20, 0, 15.5], yaw: 0, pitch: 60, dist: 13, sam: [20, 0, 17.5] },
  'yard-in': { title: 'yard, Sam indoors', look: [20, 0, 15.5], yaw: 0, pitch: 60, dist: 13, sam: [16, 0, 6.6] },
  'window': { title: 'room with windows, Sam in it', look: [18.4, 0, 8.2], yaw: -90, pitch: 62, dist: 9, sam: [16, 0, 6.6] },
  'window-out': { title: 'room with windows, Sam in the yard', look: [18.4, 0, 8.2], yaw: -90, pitch: 62, dist: 9, sam: [20, 0, 17.5] },
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
const pairs = s => Object.fromEntries(s.split(',').filter(Boolean).map(kv => { const [k, v] = kv.split(':'); return [k, +v]; }));

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
      return { canvas: c, stats: { mean: +mean.toFixed(4), p5: +q(0.05).toFixed(4), p50: +q(0.5).toFixed(4), p95: +q(0.95).toFixed(4), burnt: +(burnt / L.length).toFixed(3), black: +(black / L.length).toFixed(3) } }; };
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
    await page.evaluate(async ([hour, set, v, D]) => {
      const E = window.__engine; window.__look.set(window.__look0); if (set) window.__look.set(set); window.__hour.set(hour); E.settings.debugView = 0;
      window.__stage({ look: v.look, yaw: v.yaw * D, pitch: v.pitch * D, dist: v.dist }, { player: { pos: v.sam, holstered: true } }); window.__park(); if (E.lookEye) E.lookEye.snap = true;
      window.__draw(true); for (let i = 0; i < 3; i++) { window.__runFrames(1, 1 / 30); await window.__done(); } window.__draw(false);
    }, [row.hour, row.set ? pairs(row.set) : null, VIEWS[COLS[0]], D]);
    table[row.id] = {};
    for (const col of COLS) {
      const r = await page.evaluate(async ([rowId, col, v, n, D]) => {
        const E = window.__engine, g = window.__game;
        window.__stage({ look: v.look, yaw: v.yaw * D, pitch: v.pitch * D, dist: v.dist }, { player: { pos: v.sam, holstered: true } }); window.__park(); if (E.lookEye) E.lookEye.snap = true;
        window.__draw(false); window.__runFrames(2, 1 / 60); if (E.lookEye) E.lookEye.snap = true; window.__draw(true); const ms = [];
        for (let i = 0; i < n; i++) { const t0 = performance.now(); window.__runFrames(1, 1 / 600); await window.__done(); ms.push(Math.round(performance.now() - t0)); }
        const fr = await window.__grab(); window.__draw(false); if (!fr) return null; (window.__cells[rowId] ||= {})[col] = fr.canvas;
        const num = x => typeof x === 'number' && isFinite(x) ? +x.toPrecision(4) : null;
        return { url: fr.canvas.toDataURL('image/png'), ms, gpuErrs: window.__gpuErrs.splice(0, 4), stats: fr.stats,
          at: { hour: window.__hour.id, exposure: num(E.settings.exposure), samLight: num(g.player.irradiance), samVis: num(g.player.visibility), eye: E.lookEyeCfg ? Array.from(E.lookEyeCfg, num) : null, shade: E.lookShadeCfg ? Array.from(E.lookShadeCfg, num) : null } };
      }, [row.id, col, VIEWS[col], SETTLE, D]);
      if (!r) { errors.push(`no frame for ${row.id} ${col}`); log(row.id, col, 'NO FRAME'); continue; }
      png(path.join(OUT, 'cells', `${row.id.replace(/[^a-z0-9]+/gi, '-')}_${col}.png`), r.url); delete r.url; table[row.id][col] = r;
      if (r.gpuErrs.length) errors.push(...r.gpuErrs);
      log(row.id, col, 'ms', r.ms.join(','), JSON.stringify(r.stats), JSON.stringify(r.at), r.gpuErrs.length ? 'GPU ERRORS ' + JSON.stringify(r.gpuErrs) : '');
    }
    await sheet(); fs.writeFileSync(path.join(OUT, 'sheet.json'), JSON.stringify({ rows: ROWS, views: COLS, cells: table, errors }, null, 1)); log('sheet written after', row.id);
  }
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
