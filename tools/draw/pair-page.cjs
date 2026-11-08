// THE TWO PAGES SIDE BY SIDE: the game's page and draw.html at the same address, die, tick and camera; a still of each, how far the two pictures are apart, and what each costs. Stepped by hand, nothing played.
//   PORT=<a dev server> node tools/draw/pair-page.cjs --out <dir> [--size 1280x720] [--quality high] [--places office,pass] [--time] [--ticks 180]
// --time: instead of stills, each page's own frames for a while: GPU ms of its passes, its own ms, the page's memory and the time to its first picture. A card's numbers only.
const { chromium } = (() => { try { return require('playwright'); } catch { return require('/opt/node-tools/node_modules/playwright'); } })();
const { mkdirSync, writeFileSync } = require('node:fs');
const arg = (n, d) => { const i = process.argv.indexOf(`--${n}`); return i > 0 ? process.argv[i + 1] : d; }, has = n => process.argv.includes(`--${n}`);
const PORT = process.env.PORT || 5199, OUT = arg('out', '/tmp/pair'), [W, H] = arg('size', '1280x720').split('x').map(Number), QUALITY = arg('quality', 'high'), TICKS = Number(arg('ticks', 180)), FRAMES = Number(arg('frames', 60));
const PLACES = { killhouse: 'levelFile=killhouse.json&roomclear&squad4', office: 'nightbreach', compound: 'levelFile=compound.json&roomclear&squad4&weather=rain', barracks: 'levelFile=barracks.json&roomclear&squad4&hour=midday', pass: 'levelFile=pass.json&roomclear&squad4', consulate: 'levelFile=consulate.json&rescue&squad=4' };
const want = arg('places', Object.keys(PLACES).join(',')).split(','), FLAGS = ['--no-sandbox', '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
for (const n of want) if (!PLACES[n]) { console.error(`pair-page: no place ${n} (${Object.keys(PLACES).join(', ')})`); process.exit(2); }
const t0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - t0) / 1000).toFixed(0)}s]`, ...a);
(async () => {
  mkdirSync(OUT, { recursive: true });
  const browser = await chromium.launch({ headless: true, executablePath: process.env.CHROMIUM || '/usr/bin/chromium', args: FLAGS }), rows = [];
  const open = async (page, words) => {
    const ctx = await browser.newContext({ viewport: { width: W, height: H }, deviceScaleFactor: 1 }), p = await ctx.newPage(), errors = []; p.setDefaultTimeout(900000);
    p.on('pageerror', e => errors.push(String(e && e.stack || e).slice(0, 400))); p.on('console', m => { if (m.type() === 'error') errors.push(m.text().slice(0, 200)); });
    await p.goto(`http://localhost:${PORT}/${page}?${words}`, { timeout: 0 });
    await p.waitForFunction(() => !!window.__ready && (!!window.__draw || !!window.__cam), null, { timeout: 900000 }); await p.evaluate(() => window.__ready);
    return { p, errors, ctx };
  };
  for (const name of want) {
    const pair = {};
    for (const [side, page] of [['old', 'index.html'], ['new', 'draw.html']]) {
      const words = `${PLACES[name]}&seed=7&quality=${QUALITY}&adapt=0&mute&loadbrief=0&cloud=0&noreplay${has('time') ? '' : '&paused'}`, { p, errors, ctx } = await open(page, words);
      if (has('time')) {
        const r = await p.evaluate(async () => {
          const wait = ms => new Promise(r => setTimeout(r, ms)), E = window.__engine, T = E.timer, gpu = [], page = [], own = window.__draw ? window.__draw.told : null; await wait(8000);
          const until = performance.now() + 12000; let last = -1; while (performance.now() < until) { if (T.resolved !== last) { last = T.resolved; const ms = T.unionMs(); if (ms > 0) gpu.push(ms); } if (own) page.push(own.update + own.render + own.hud); else if (window.__perfrec) page.push(window.__perfrec.cpuUpdate + window.__perfrec.cpuRender); await new Promise(r => requestAnimationFrame(r)); }
          const mid = a => a.length ? a.sort((x, y) => x - y)[a.length >> 1] : NaN, boot = window.__boot && (window.__boot.marks || window.__boot.summary && window.__boot.summary().marks);
          return { gpu: mid(gpu), page: mid(page), n: page.length, heap: performance.memory ? performance.memory.usedJSHeapSize / 1048576 : NaN, wasm: window.__draw ? window.__draw.game.memory.buffer.byteLength / 1048576 : NaN, boot, res: `${E.width}x${E.height} of ${E.outWidth}x${E.outHeight}` };
        });
        rows.push({ name, side, ...r, errors: errors.length }); log(name, side, JSON.stringify(r).slice(0, 500), errors.slice(0, 2).join(' | '));
      } else {
        // (every tick drawn at the end of it, the way a person's page draws one: what accumulates over frames has the same frames)
        const r = await p.evaluate(async ([ticks, frames]) => {
          const E = window.__engine, cam = window.__draw ? window.__draw.cam : window.__cam; window.__runFrames(ticks, 1 / 60, [], 'none'); for (let i = 0; i < frames; i++) { window.__runFrames(1, 1 / 60, [], true); await E.device.queue.onSubmittedWorkDone(); }
          const men = window.__draw ? window.__draw.game.map().men.map(m => [m.x, m.z]) : [window.__game.player.char, ...window.__game.guards.map(g => g.char), ...window.__game.civilians.map(g => g.char)].map(c => [Math.round(c.pos[0] * 100) / 100, Math.round(c.pos[2] * 100) / 100]);
          E.render(cam, 5, 1 / 60, false); const f = await E.readbackFrame(); let o = ''; const B = 0x8000; for (let i = 0; i < f.data.length; i += B) o += String.fromCharCode.apply(null, f.data.subarray(i, i + B));
          return { w: f.w, h: f.h, bgra: f.bgra, b: btoa(o), men, cam: { t: [...cam.target], yaw: cam.yaw, pitch: cam.pitch, d: cam.distance, fov: cam.fov }, set: { exposure: E.settings.exposure, res: `${E.width}x${E.height}` }, hud: ['equip', 'interact', 'objective', 'log', 'meters'].map(id => `${id}: ${(document.getElementById(id).innerText || '').replace(/\s+/g, ' ')}`) };
        }, [TICKS, FRAMES]);
        pair[side] = r; writeFileSync(`${OUT}/${name}-${side}.raw.json`, JSON.stringify(r)); log(name, side, r.w, r.h, JSON.stringify(r.cam), errors.slice(0, 3).join(' | '));
      }
      await ctx.close();
    }
    if (pair.old && pair.new) {
      const a = Buffer.from(pair.old.b, 'base64'), b = Buffer.from(pair.new.b, 'base64'); let sum = 0, over = 0, n = 0, worst = 0; const cell = {};
      for (let i = 0; i < Math.min(a.length, b.length); i += 4) { const d = (Math.abs(a[i] - b[i]) + Math.abs(a[i + 1] - b[i + 1]) + Math.abs(a[i + 2] - b[i + 2])) / 3; sum += d; n++; if (d > 24) { over++; const x = (i / 4) % pair.old.w, y = Math.floor(i / 4 / pair.old.w), k = `${Math.floor(x / pair.old.w * 8)},${Math.floor(y / pair.old.h * 6)}`; cell[k] = (cell[k] || 0) + 1; } worst = Math.max(worst, d); }
      const moved = pair.old.men.filter((m, i) => !pair.new.men[i] || Math.hypot(m[0] - pair.new.men[i][0], m[1] - pair.new.men[i][1]) > 0.02).length;
      const row = { name, mean: +(sum / n).toFixed(2), over24: +(100 * over / n).toFixed(2), worst: +worst.toFixed(0), men: `${pair.old.men.length}/${pair.new.men.length}`, menApart: moved, camApart: +Math.hypot(...pair.old.cam.t.map((v, i) => v - pair.new.cam.t[i])).toFixed(4), hud: pair.old.hud.every((h, i) => h === pair.new.hud[i]), where: Object.entries(cell).sort((x, y) => y[1] - x[1]).slice(0, 6).map(([k, v]) => `${k}:${v}`).join(' ') };
      rows.push(row); log(JSON.stringify(row)); if (!row.hud) pair.old.hud.forEach((h, i) => { if (h !== pair.new.hud[i]) log(`  old ${h}\n  new ${pair.new.hud[i]}`); });
    }
  }
  writeFileSync(`${OUT}/rows.json`, JSON.stringify(rows, null, 1));
  await browser.close();
})().catch(e => { console.error(e); process.exit(2); });
