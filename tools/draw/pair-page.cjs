// THE TWO PAGES SIDE BY SIDE: the game's page and draw.html at the same address, die, tick and camera; a still of each, how far the two pictures are apart, the renderer's own numbers that are not the same (`apart`), and what each costs. Stepped by hand, nothing played.
//   PORT=<a dev server> node tools/draw/pair-page.cjs --out <dir> [--size 1280x720] [--quality high] [--places office,pass] [--time] [--ticks 180]
// --act "Digit5 run:20 down run:15 up draw:90" (--as <a name for the still>): keys and the mouse before the still, the same on both pages (a code is tapped; down and up are the mouse; run and draw are ticks).
// --play <minutes>: instead, one place (--places pass) played on each page by the same scripted hand (walks, shots, throws, orders; he cannot be hurt, his rounds are not counted), and every half minute
//   where a frame of the page's goes: its update by parts, its render, the HUD, the heap.
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
      const words = `${PLACES[name]}&seed=7&quality=${QUALITY}&adapt=0&mute&loadbrief=0&cloud=0&noreplay${has('time') || has('play') ? '' : '&paused'}`, { p, errors, ctx } = await open(page, words);
      if (has('play')) {
        const r = await p.evaluate(async minutes => {
          const D = window.__draw, G = window.__game, canvas = document.getElementById('view'), mid = a => a.length ? +a.slice().sort((x, y) => x - y)[a.length >> 1].toFixed(2) : null, hi = a => a.length ? +a.slice().sort((x, y) => x - y)[Math.floor(a.length * 0.95)].toFixed(2) : null;
          let n = 77; const roll = () => { n = Math.imul(n ^ n >>> 15, n | 1); n ^= n + Math.imul(n ^ n >>> 7, n | 61); return ((n ^ n >>> 14) >>> 0) / 4294967296; };
          const god = () => { if (D) { D.game.commands.god(1); D.game.commands.ammo(1); } else { G.godMode = true; G.infiniteAmmo = true; } }; god();
          const key = (type, code) => window.dispatchEvent(new KeyboardEvent(type, { code, key: code, bubbles: true, cancelable: true })), held = new Set(), hold = codes => { for (const c of held) if (!codes.includes(c)) { key('keyup', c); held.delete(c); } for (const c of codes) if (!held.has(c)) { key('keydown', c); held.add(c); } };
          const mouse = (type, x, y, button = 0) => (type === 'mouseup' ? window : canvas).dispatchEvent(new MouseEvent(type, { clientX: x, clientY: y, button, buttons: type === 'mousedown' ? 1 : 0, bubbles: true, cancelable: true }));
          const tap = async c => { key('keydown', c); await new Promise(r => setTimeout(r, 60)); key('keyup', c); };
          const begin = performance.now(), until = begin + minutes * 60000, buckets = []; let B = null, nextHand = 0, lastTicks = -1;
          while (performance.now() < until) {
            await new Promise(r => requestAnimationFrame(r)); const now = performance.now();
            if (!B || now - B.t0 >= 30000) { B = { t0: now, frames: 0, drawn: 0, parts: {} }; buckets.push(B); }
            const put = (k, v) => { if (Number.isFinite(v)) (B.parts[k] ??= []).push(v); }, ticks = D ? D.game.ticks : window.__rust.ticks; B.frames++;
            if (ticks !== lastTicks) { const ran = lastTicks < 0 ? 1 : Math.max(1, ticks - lastTicks); lastTicks = ticks; B.drawn++; put('ticks', ran); put('perTick', (D ? D.told.update : window.__perfrec.cpuUpdate) / ran);
              if (D) { put('update', D.told.update); put('step', D.game.ms.step); put('rows', D.game.ms.draw); put('rest', D.told.update - D.game.ms.step - D.game.ms.draw); put('render', D.told.render); put('hud', D.told.hud); }
              else { const L = window.__rust.last, P = window.__perfrec; put('update', P.cpuUpdate); put('step', L.step); put('export', L.export); put('apply', L.apply); put('out', L.out); put('dress', L.dress); put('host', L.host); put('rest', P.cpuUpdate - L.step - L.export - L.apply - L.out - L.dress - L.host); put('render', P.cpuRender); } }
            B.heap = performance.memory ? Math.round(performance.memory.usedJSHeapSize / 1048576) : null; B.wasm = D ? Math.round(D.game.memory.buffer.byteLength / 1048576) : window.__rust.memory ? Math.round(window.__rust.memory / 1048576) : null;
            if (now >= nextHand) { nextHand = now + 900; const keys = ['KeyW', 'KeyA', 'KeyS', 'KeyD'].filter(() => roll() < 0.35); hold(keys);
              const x = 0.2 + 0.6 * roll(), y = 0.2 + 0.5 * roll(), X = x * canvas.clientWidth, Y = y * canvas.clientHeight; mouse('mousemove', X, Y); const what = roll();
              if (what < 0.5) { await tap('Digit2'); mouse('mousedown', X, Y); setTimeout(() => mouse('mouseup', X, Y), 400); } else if (what < 0.62) { await tap(['Digit3', 'Digit4', 'Digit5'][Math.floor(roll() * 3)]); mouse('mousedown', X, Y); setTimeout(() => mouse('mouseup', X, Y), 350); } else if (what < 0.75) await tap('KeyQ'); else if (what < 0.82) await tap('KeyF'); else if (what < 0.88) await tap('KeyE'); god(); }
          }
          hold([]); return buckets.map((b, i) => ({ min: i / 2, fps: Math.round(b.frames / 30), drawn: Math.round(b.drawn / 30), heap: b.heap, wasm: b.wasm, ...Object.fromEntries(Object.entries(b.parts).map(([k, a]) => [k, mid(a)])), update95: hi(b.parts.update || []) }));
        }, Number(arg('play', 5)));
        rows.push({ name, side, play: r, errors: errors.length }); log(name, side, errors.slice(0, 2).join(' | ')); for (const b of r) log('  ', JSON.stringify(b));
      } else if (has('time')) {
        const r = await p.evaluate(async () => {
          const wait = ms => new Promise(r => setTimeout(r, ms)), E = window.__engine, T = E.timer, gpu = [], page = [], own = window.__draw ? window.__draw.told : null; await wait(8000);
          const until = performance.now() + 12000; let last = -1; while (performance.now() < until) { if (T.resolved !== last) { last = T.resolved; const ms = T.unionMs(); if (ms > 0) gpu.push(ms); } if (own) page.push(own.update + own.render + own.hud); else if (window.__perfrec) page.push(window.__perfrec.cpuUpdate + window.__perfrec.cpuRender); await new Promise(r => requestAnimationFrame(r)); }
          const mid = a => a.length ? a.sort((x, y) => x - y)[a.length >> 1] : NaN, boot = (() => { try { const s = window.__boot && (window.__boot.summary ? window.__boot.summary() : window.__boot); return s && JSON.parse(JSON.stringify(s)); } catch { return null; } })();
          return { gpu: mid(gpu), page: mid(page), n: page.length, heap: performance.memory ? performance.memory.usedJSHeapSize / 1048576 : NaN, wasm: window.__draw ? window.__draw.game.memory.buffer.byteLength / 1048576 : NaN, boot, res: `${E.width}x${E.height} of ${E.outWidth}x${E.outHeight}` };
        });
        rows.push({ name, side, ...r, errors: errors.length }); log(name, side, JSON.stringify(r).slice(0, 500), errors.slice(0, 2).join(' | '));
      } else {
        // (every tick drawn at the end of it, the way a person's page draws one: what accumulates over frames has the same frames)
        const r = await p.evaluate(async ([ticks, frames, act]) => {
          const E = window.__engine, cam = window.__draw ? window.__draw.cam : window.__cam; window.__runFrames(ticks, 1 / 60, [], 'none');
          // (an act first: the player's own keys and mouse, the same on both pages, and the second or two it takes, every tick of it drawn)
          if (act) { const canvas = document.getElementById('view'), X = 0.62 * canvas.clientWidth, Y = 0.36 * canvas.clientHeight, key = (t, code) => window.dispatchEvent(new KeyboardEvent(t, { code, key: code, bubbles: true, cancelable: true })), mouse = t => (t === 'mouseup' ? window : canvas).dispatchEvent(new MouseEvent(t, { clientX: X, clientY: Y, button: 0, buttons: t === 'mousedown' ? 1 : 0, bubbles: true, cancelable: true }));
            const run = async (n, drawn) => { for (let i = 0; i < n; i++) { window.__runFrames(1, 1 / 60, [], drawn ? true : 'none'); if (drawn) await E.device.queue.onSubmittedWorkDone(); } }, tap = async c => { key('keydown', c); await run(1); key('keyup', c); await run(1); };
            mouse('mousemove'); await run(2); for (const step of act.split(' ')) { const [what, n] = step.split(':'); if (what === 'down') mouse('mousedown'); else if (what === 'up') mouse('mouseup'); else if (what === 'run') await run(+n); else if (what === 'draw') await run(+n, true); else await tap(what); } }
          for (let i = 0; i < frames; i++) { window.__runFrames(1, 1 / 60, [], true); await E.device.queue.onSubmittedWorkDone(); }
          const men = window.__draw ? (window.__draw.game.map ? window.__draw.game.map().men.map(m => [m.x, m.z]) : []) : [window.__game.player.char, ...window.__game.guards.map(g => g.char), ...window.__game.civilians.map(g => g.char)].map(c => [Math.round(c.pos[0] * 100) / 100, Math.round(c.pos[2] * 100) / 100]);
          E.render(cam, 5, 1 / 60, false); const f = await E.readbackFrame(); let o = ''; const B = 0x8000; for (let i = 0; i < f.data.length; i += B) o += String.fromCharCode.apply(null, f.data.subarray(i, i + B));
          return { w: f.w, h: f.h, bgra: f.bgra, b: btoa(o), men, cam: { t: [...cam.target], yaw: cam.yaw, pitch: cam.pitch, d: cam.distance, fov: cam.fov }, res: `${E.width}x${E.height}`, set: ((o = {}) => { for (const [of, t] of [['settings', E.settings], ['smokeDraw', E.smokeDraw], ['passes', E.passes]]) for (const [k, v] of Object.entries(t)) if (typeof v === 'number' || typeof v === 'boolean') o[`${of}.${k}`] = typeof v === 'number' ? Math.fround(v) : v; return o; })(), maps: { ...E.gbuffer.lookBy }, hud: ['equip', 'interact', 'objective', 'log', 'meters'].map(id => `${id}: ${(document.getElementById(id).innerText || '').replace(/\s+/g, ' ')}`) };
        }, [TICKS, FRAMES, arg('act', '')]);
        pair[side] = r; log(name, side, r.w, r.h, JSON.stringify(r.cam), errors.slice(0, 3).join(' | '));
      }
      await ctx.close();
    }
    if (pair.old && pair.new) {
      const a = Buffer.from(pair.old.b, 'base64'), b = Buffer.from(pair.new.b, 'base64'); let sum = 0, over = 0, n = 0, worst = 0; const cell = {};
      for (let i = 0; i < Math.min(a.length, b.length); i += 4) { const d = (Math.abs(a[i] - b[i]) + Math.abs(a[i + 1] - b[i + 1]) + Math.abs(a[i + 2] - b[i + 2])) / 3; sum += d; n++; if (d > 24) { over++; const x = (i / 4) % pair.old.w, y = Math.floor(i / 4 / pair.old.w), k = `${Math.floor(x / pair.old.w * 8)},${Math.floor(y / pair.old.h * 6)}`; cell[k] = (cell[k] || 0) + 1; } worst = Math.max(worst, d); }
      // the two and what is between them, as one picture: the game's page, draw.html, and four times the difference
      { const pic = await (await browser.newContext()).newPage(); const url = await pic.evaluate(async ([o, n, w, h, bgra]) => {
          const px = s => { const t = atob(s), u = new Uint8ClampedArray(t.length); for (let i = 0; i < t.length; i++) u[i] = t.charCodeAt(i); if (bgra) for (let i = 0; i < u.length; i += 4) { const x = u[i]; u[i] = u[i + 2]; u[i + 2] = x; } for (let i = 3; i < u.length; i += 4) u[i] = 255; return u; };
          const a = px(o), b = px(n), d = new Uint8ClampedArray(a.length); for (let i = 0; i < a.length; i++) d[i] = i % 4 === 3 ? 255 : Math.abs(a[i] - b[i]) * 4;
          const c = document.createElement('canvas'); c.width = w * 3; c.height = h; const x = c.getContext('2d'); [a, b, d].forEach((p, k) => x.putImageData(new ImageData(p, w, h), k * w, 0)); return c.toDataURL('image/jpeg', 0.88);
        }, [pair.old.b, pair.new.b, pair.old.w, pair.old.h, pair.old.bgra]); writeFileSync(`${OUT}/${name}${arg('act', '') ? '-' + arg('as', 'act') : ''}.jpg`, Buffer.from(url.split(',')[1], 'base64')); await pic.context().close(); }
      const moved = !pair.new.men.length ? -1 : pair.old.men.filter((m, i) => !pair.new.men[i] || Math.hypot(m[0] - pair.new.men[i][0], m[1] - pair.new.men[i][1]) > 0.02).length;
      const apart = [...new Set([...Object.keys(pair.old.set), ...Object.keys(pair.new.set)])].filter(k => pair.old.set[k] !== pair.new.set[k]).map(k => `${k} ${pair.old.set[k]} | ${pair.new.set[k]}`);
      const row = { name, mean: +(sum / n).toFixed(2), over24: +(100 * over / n).toFixed(2), worst: +worst.toFixed(0), men: `${pair.old.men.length}/${pair.new.men.length}`, menApart: moved, camApart: +Math.hypot(...pair.old.cam.t.map((v, i) => v - pair.new.cam.t[i])).toFixed(4), hud: pair.old.hud.every((h, i) => h === pair.new.hud[i]), apart, maps: `${JSON.stringify(pair.old.maps)} ${JSON.stringify(pair.new.maps)}`, where: Object.entries(cell).sort((x, y) => y[1] - x[1]).slice(0, 6).map(([k, v]) => `${k}:${v}`).join(' ') };
      rows.push(row); log(JSON.stringify(row)); if (!row.hud) pair.old.hud.forEach((h, i) => { if (h !== pair.new.hud[i]) log(`  old ${h}\n  new ${pair.new.hud[i]}`); });
    }
  }
  writeFileSync(`${OUT}/rows.json`, JSON.stringify(rows, null, 1));
  await browser.close();
})().catch(e => { console.error(e); process.exit(2); });
