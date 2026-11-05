// MARKS THAT SIT IN THE SURFACE, on the page (render/decals.ts Decals.sit, shaders/decals.wgsl dkSit; ?marksit=0): what several hand grenades leave on open
// ground of the yard (each one's films of dust and its scorch, laid as the game's records), and stills of the same marks with the word down and up, dry or in the weather asked for.
//   node tools/qa/marksit-page.cjs [--out DIR] [--level gwot-yard.json] [--hour overcast] [--weather rain] [--bursts 5] [--quality high]
//   headless Chromium on the software adapter. It shows that the page boots with the shader, that the word changes what the marks look like and that the
//   picture without it is the picture as it was drawn before. Whether the marks READ as part of the ground is for a GPU: nobody has seen that there.
//   writes DIR/marks-<dry|the weather>-<0|1>.png and DIR/marks.json   (one weather a run: about nine minutes to boot and twenty seconds a frame here)
//   exit 0 = every check held with no page or GPU error · 1 = a check failed or an error · 2 = the script died
//   MARKSIT_BREAK=1: the word is never raised, so the two stills of a pair must come out the same and the check red.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const { spawn } = require('child_process'); const net = require('net'), path = require('path'), fs = require('fs');
const argv = process.argv.slice(2), flag = (n, d) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : d; };
const ROOT = path.resolve(__dirname, '../..'), OUT = path.resolve(flag('--out', 'marksit')), QUALITY = flag('--quality', 'high'), SCALE = +flag('--scale', 0.6), VW = +flag('--w', 640), VH = +flag('--h', 400);
const LEVEL = flag('--level', 'gwot-yard.json'), HOUR = flag('--hour', 'overcast'), WEATHER = flag('--weather', ''), BURSTS = +flag('--bursts', 5), BREAK = process.env.MARKSIT_BREAK === '1';
const FLAGS = ['--js-flags=--no-use-libm-trig-functions --no-use-std-math-pow', '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const freePort = () => new Promise((res, rej) => { const s = net.createServer(); s.unref(); s.on('error', rej); s.listen(0, '127.0.0.1', () => { const p = s.address().port; s.close(() => res(p)); }); });
let srv = null, browser = null; const stop = () => { try { browser && browser.close(); } catch { } try { srv && srv.kill(); } catch { } srv = null; };
process.on('exit', stop); for (const s of ['SIGINT', 'SIGTERM', 'SIGHUP']) process.on(s, () => { stop(); process.exit(130); });

(async () => {
  fs.mkdirSync(OUT, { recursive: true });
  let port = +process.env.PORT || 0;
  if (!port) { port = await freePort(); srv = spawn('bun', ['run', 'tools/serve.ts'], { cwd: ROOT, env: { ...process.env, PORT: String(port) }, stdio: 'ignore' });
    let up = false; for (let i = 0; i < 240 && !up; i++) { try { up = (await fetch(`http://127.0.0.1:${port}/`)).ok; } catch { } if (!up) await new Promise(r => setTimeout(r, 250)); }
    if (!up) throw new Error('the dev server did not come up'); }
  browser = await chromium.launch({ args: FLAGS });
  const errors = [], page = await browser.newPage({ viewport: { width: VW, height: VH }, deviceScaleFactor: 1 });
  page.on('pageerror', e => { errors.push('pageerror: ' + String(e.stack || e).slice(0, 400)); log('PAGEERROR', String(e).slice(0, 300)); });
  page.on('console', m => { const x = m.text(); if (x.startsWith('marks: ')) { log(x); return; } if (m.type() !== 'error' || /Failed to load resource/.test(x)) return; errors.push(x.slice(0, 400)); log('console.error', x.slice(0, 300)); });
  const url = `http://localhost:${port}/?levelFile=${LEVEL}&solo&nofow&seed=1&paused&mute&quality=${QUALITY}${WEATHER ? '&weather=' + WEATHER : ''}`;   // (a page's weather is its boot's: the renderer's half is compiled then)
  log('goto', url); await page.goto(url, { waitUntil: 'commit' });
  for (let i = 0; ; i++) { try { if (await page.evaluate(() => !!window.__ready)) break; } catch { } if (i > 1800) throw new Error('never __ready: ' + errors.join(' | ')); await new Promise(r => setTimeout(r, 500)); }
  const info = await page.evaluate(async ([SCALE, HOUR, BURSTS]) => {
    window.__testVisible = true; await window.__ready; const g = window.__game, E = window.__engine; try { window.__menu.close(); } catch { }
    let at = 'hooks'; try {
    { const real = E.render.bind(E); E.render = (c, t, dt) => real(c, t, dt, false); }   // (every frame is drawn off screen: readbackFrame reads that target)
    window.__gpuErrs = []; E.device.addEventListener('uncapturederror', e => { window.__gpuErrs.push(String(e.error && e.error.message).slice(0, 300)); });
    window.__done = async () => { await E.device.queue.onSubmittedWorkDone(); await new Promise(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => res(); ch.port2.postMessage(0); }); };
    window.__grab = async () => { const fr = await E.readbackFrame(); if (!fr) return null; const d = fr.data; if (fr.bgra) for (let i = 0; i < d.length; i += 4) { const b = d[i]; d[i] = d[i + 2]; d[i + 2] = b; }
      for (let i = 3; i < d.length; i += 4) d[i] = 255;
      window.__last = { w: fr.w, h: fr.h, d: new Uint8Array(d.buffer.slice(d.byteOffset, d.byteOffset + d.length)) };
      const c = document.createElement('canvas'); c.width = fr.w; c.height = fr.h; c.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(d.buffer, d.byteOffset, d.length), fr.w, fr.h), 0, 0); return c.toDataURL('image/png'); };
    E.settings.renderScale = SCALE; g.godMode = true; g.aiEnabled = false; window.__hour.set(HOUR);
    window.__smoke.params.frozen = true;   // (a burst's pall is the smoke solver's, and a step of it takes seconds on the software adapter: the marks are laid at the burst, the pall is not wanted)
    for (let i = 0; i < 2; i++) { window.__runFrames(1, 1 / 60); await window.__done(); }
    at = 'spots'; console.log('marks: ' + at);
    // open ground near where Sam stands: spots on the level's floor with nothing standing within 1.3 m, at least 2.4 m apart, the nearest first
    const P = g.player.char.pos, spots = [], clear = (x, z) => { const h = g.col.raycast([x, 2.6, z], [0, -1, 0], 3); if (!h || Math.abs(2.6 - h.t) > 0.03) return false;
      for (let k = 0; k < 8; k++) { const a = k * Math.PI / 4; if (g.col.raycast([x, 1.2, z], [Math.cos(a), 0, Math.sin(a)], 1.3)) return false; } return true; };
    const cand = []; for (let dx = -9; dx <= 9; dx += 0.6) for (let dz = -9; dz <= 9; dz += 0.6) cand.push([P[0] + dx, P[2] + dz, Math.hypot(dx, dz)]);
    cand.sort((a, b) => a[2] - b[2]);
    for (const c of cand) { if (c[2] < 1.5 || spots.length >= BURSTS) continue; if (spots.some(s => Math.hypot(s[0] - c[0], s[1] - c[1]) < 2.4) || !clear(c[0], c[1])) continue; spots.push([c[0], c[1]]); }
    at = 'bursts'; console.log('marks: ' + at);
    // at each, what a hand grenade leaves, laid as records by floormess.ts's own numbers (a heart of dust and its lobes: CHAOS.mess.burst) and the scorch where it
    // lay; one more scorch on the nearest wall. NOT the blast itself: what it throws and its pall take minutes a frame on the software adapter and are not the marks.
    const B = window.__destr.knobs.mess.burst;
    const H = (x, z, k) => { let h = (Math.imul(Math.round(x * 64), 73856093) ^ Math.imul(Math.round(z * 64), 19349663) ^ Math.imul(k + 1, 2654435761)) >>> 0; h = Math.imul(h ^ (h >>> 15), 2246822519) >>> 0; h = Math.imul(h ^ (h >>> 13), 3266489917) >>> 0; return ((h ^ (h >>> 16)) >>> 0) / 4294967296; };
    const film = (x, z, rad, s, k, heart) => { const a = H(x, z, k + 31) * Math.PI * 2; g.decals.add('damage', [x, 0, z], [0, 1, 0], rad, { param: 6 + Math.min(0.999, Math.max(0.05, s)), look: true, seed: (Math.floor(H(x, z, k) * 2048) & 0x7ff) | (heart ? 0x800 : 0), tangent: [Math.cos(a), 0, Math.sin(a)] }); };
    for (const [x, z] of spots) {
      film(x, z, B.core, B.strength, 0, true); const a0 = H(x, z, 97) * Math.PI * 2;
      for (let k = 0; k < B.lobes; k++) { const j = H(x, z, k * 2 + 1), j2 = H(x, z, k * 2 + 2), a = a0 + k * 2 * Math.PI / B.lobes + (j - 0.5) * 0.6 * 2 * Math.PI / B.lobes, d = B.reach * (0.75 + 0.5 * j2), px = x + Math.sin(a) * d, pz = z + Math.cos(a) * d;
        if (!g.col.raycast([x, 1.2, z], [Math.sin(a), 0, Math.cos(a)], d)) film(px, pz, B.lobe * (0.8 + 0.4 * j), B.strength * (0.6 + 0.3 * j2), k + 1, false); }
      g.decals.add('scorch', [x, 0, z], [0, 1, 0], 0.62 + 0.28 * H(x, z, 5), { param: 1 });
    }
    let wall = null; for (const [x, z] of spots) for (let k = 0; k < 16 && !wall; k++) { const a = k * Math.PI / 8, d = [Math.cos(a), 0, Math.sin(a)], h = g.col.raycast([x, 0.9, z], d, 6); if (h && Math.abs(h.n[1]) < 0.3) wall = { p: [x + d[0] * h.t, 0.9, z + d[2] * h.t], n: h.n }; }
    if (wall) g.decals.add('scorch', wall.p, wall.n, 0.5, { param: 0.9 });
    at = 'stage'; console.log('marks: ' + at);
    const cx = spots.reduce((s, p) => s + p[0], 0) / Math.max(1, spots.length), cz = spots.reduce((s, p) => s + p[1], 0) / Math.max(1, spots.length);
    window.__stage({ look: [cx, 0, cz], yaw: 0, pitch: 62 * Math.PI / 180, dist: 7.5 }, { player: { pos: [P[0], 0, P[2]], holstered: true } });
    at = 'frames'; console.log('marks: ' + at);
    const t0 = performance.now(); for (let i = 0; i < 3; i++) { window.__runFrames(1, 1 / 60); await window.__done(); } const settleMs = Math.round(performance.now() - t0);
    const looks = {}; for (const d of g.decals.list) { const k = d.kind === 'damage' ? 'damage' + Math.floor(d.param) : d.kind; looks[k] = (looks[k] || 0) + 1; }
    return { settleMs, pipelines: window.__boot && window.__boot.pipelines, spots: spots.map(s => s.map(v => +v.toFixed(1))), wall: !!wall, marks: looks, sit: g.decals.sit, weather: g.weather.kind, wetGround: !!g.weather.wetGloss };
    } catch (e) { throw new Error(`at ${at}: ${e && e.stack || e}`); }
  }, [SCALE, HOUR, BURSTS]);
  log('booted', JSON.stringify(info));
  const out = { info, pairs: {} }, fails = [];
  if (info.spots.length < 2) fails.push(`only ${info.spots.length} open spots found for a burst`);
  if (!info.sit) fails.push('the page did not raise the word at boot');
  if (!(info.marks.damage6 > 0)) fails.push('no film of dust was laid');
  if (WEATHER && info.weather !== WEATHER) fails.push(`the page is in ${info.weather}, not ${WEATHER}`);
  for (const wx of [WEATHER || 'dry']) {
    const shots = {};
    for (const sit of [0, 1]) {
      const r = await page.evaluate(async ([wx, on]) => { const g = window.__game;
        g.decals.setSit(on); for (let i = 0; i < 5; i++) { window.__runFrames(1, 1 / 600); await window.__done(); }
        const u = await window.__grab(), L = window.__last; if (!u || !L) return { u: null }; let s = 0; for (let i = 0; i < L.d.length; i += 4) s += 0.2126 * L.d[i] + 0.7152 * L.d[i + 1] + 0.0722 * L.d[i + 2];
        const prev = window.__prev; let diff = -1, changed = -1; if (prev && prev.d.length === L.d.length) { diff = 0; changed = 0; for (let i = 0; i < L.d.length; i += 4) { const e = Math.abs(L.d[i] - prev.d[i]) + Math.abs(L.d[i + 1] - prev.d[i + 1]) + Math.abs(L.d[i + 2] - prev.d[i + 2]); diff += e; if (e > 24) changed++; } diff /= (L.d.length / 4) * 3; changed /= L.d.length / 4; }
        window.__prev = L; return { u, mean: +(s / (L.d.length / 4) / 255).toFixed(4), diff: +diff.toFixed(3), changed: +changed.toFixed(4), weather: g.weather.kind, sit: g.decals.sit }; }, [wx, BREAK ? false : !!sit]);
      if (!r.u) { fails.push(`no picture for ${wx} ${sit}`); continue; }
      const f = path.join(OUT, `marks-${wx}-${sit}.png`); fs.writeFileSync(f, Buffer.from(r.u.split(',')[1], 'base64')); delete r.u; r.file = f; shots[sit] = r; log('shot', wx, sit, JSON.stringify(r));
    }
    out.pairs[wx] = shots;
    // the word must change the marks: over one pixel in a hundred visibly different between the two stills of a pair (frozen frames, the same marks)
    if (shots[1] && !(shots[1].changed > 0.01)) fails.push(`${wx}: the word changed ${(100 * (shots[1].changed || 0)).toFixed(2)} % of the picture`);
  }
  const gpu = await page.evaluate(() => window.__gpuErrs); if (gpu.length) errors.push(...gpu);
  out.errors = errors; out.fails = fails; fs.writeFileSync(path.join(OUT, 'marks.json'), JSON.stringify(out, null, 1));
  for (const f of fails) log('FAIL', f); log('errors', errors.length, errors.slice(0, 3).join(' | '));
  log(fails.length || errors.length ? '=== marksit-page: FAIL ===' : '=== marksit-page: ALL PASS ===');
  stop(); process.exit(fails.length || errors.length ? 1 : 0);
})().catch(e => { console.error('died:', e); stop(); process.exit(2); });
