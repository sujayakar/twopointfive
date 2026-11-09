// THE RUNNING SMOKE IS THE GAME'S, with no jump in it. What men see through is the game's grid; what the player sees is the solver, bent toward it a little at a time. Read back once a second: the two volumes, and
// what lines of sight let through in each. As a check (software adapter, the solver stepped by hand): a grenade's dust, taken whole as it hangs and thinning for six seconds. With --life: a can through its pour
// and its thinning, every tick drawn, a line a second and stills, for a card.
//   PORT=<a dev server> node tools/draw/smokelife-page.cjs                                   exit 0 = every check passed, 1 = a FAIL, 2 = the script died · LIFE_BREAK=1 = thin smoke is let go of as well: it must fail
//   PORT=… OUT=… [AIM=x,y] node tools/draw/smokelife-page.cjs --life [address words] [seconds] [param=value,…]
const PAGE = require('../qa/page.cjs').page({ name: 'smokelife-page', detail: 'json' }), { check, note } = PAGE, fs = require('node:fs');
const { hands } = require('./smokeread.cjs');
const LIFE = process.argv.includes('--life'), args = process.argv.slice(2).filter(a => a !== '--life'), aim = (process.env.AIM || '.5,.35').split(',').map(Number);
(async () => {
  const context = await PAGE.context({ size: LIFE ? [1280, 720] : [160, 90] }), { page, errors, go } = await PAGE.open({ context, timeout: 3600000, errors: { also: /Invalid|error:/ } });
  const words = args[0] || 'levelFile=office.json&roomclear&squad4';
  await go(`paused&quality=${LIFE ? 'high' : 'verylow'}&adapt=0&mute&cloud=0&noreplay&seed=7&loadbrief=0&${words}`, { timeout: 600000 }); await page.waitForFunction(() => !!window.__ready && !!window.__draw, null, { timeout: 900000 }); await page.evaluate(() => window.__ready);
  await page.evaluate(hands, false);
  await page.evaluate(params => { const S = window.__draw.smoke, C = window.__draw.game.commands; for (const kv of params.split(',').filter(Boolean)) { const [k, v] = kv.split('='); if (k in S.params) S.params[k] = +v; else S.sparse[k] = +v; } window.__draw.run(1, [], 'last'); C.god(1); C.ai(0); }, args[2] || (process.env.LIFE_BREAK === '1' ? 'thinDecay=1.5' : ''));
  // a can, or a grenade, thrown where the cursor is
  const thrown = key => page.evaluate(([key, ax, ay]) => { const d = window.__draw, I = d.input, cv = document.getElementById('view'); I.mouseX = ax * cv.clientWidth; I.mouseY = ay * cv.clientHeight; J.tick(2); I.pressed.add(key); I.keys.add(key); J.tick(5); I.keys.delete(key); J.tick(35); I.buttons |= 1; I.clicked |= 1; J.tick(20); I.buttons = 0; for (let i = 0; i < 480 && !d.smoke.emitters.some(e => e.held && (key === 'Digit5' || e.kind === 'canister')); i++) J.tick(); return d.smoke.emitters.length; }, [key, ...aim]);
  if (LIFE) {
    const secs = +(args[1] || 46), tag = `${words.match(/levelFile=(\w+)/)[1]}-${(args[2] || 'as').replace(/[=,]/g, '_')}`, OUT = process.env.OUT || '/tmp'; note('sources', await thrown('Digit3'));
    for (let s = 1; s <= secs; s++) {
      await page.evaluate(async () => { const E = window.__engine; for (let i = 0; i < 60; i++) { window.__draw.run(1, [], 'all'); if (i % 4 === 3) await E.device.queue.onSubmittedWorkDone(); } });
      if (s === 1 || s % 2 === 0) console.log('LIFE', tag, s, JSON.stringify(await page.evaluate(() => J.read(8))));
      if ([4, 8, 12, 16, 24, 38].includes(s)) { const d = await page.evaluate(async () => { const E = window.__engine, D = window.__draw; D.run(1, [], 'all'); E.render(D.cam, 5, 1 / 60, false); const f = await E.readbackFrame(), u = new Uint8ClampedArray(f.data); for (let i = 0; i < u.length; i += 4) { if (f.bgra) { const x = u[i]; u[i] = u[i + 2]; u[i + 2] = x; } u[i + 3] = 255; } const c = document.createElement('canvas'); c.width = f.w; c.height = f.h; c.getContext('2d').putImageData(new ImageData(u, f.w, f.h), 0, 0); return c.toDataURL('image/jpeg', 0.88); }); fs.writeFileSync(`${OUT}/${tag}-${String(s).padStart(2, '0')}.jpg`, Buffer.from(d.split(',')[1], 'base64')); }
    }
  } else {
    await thrown('Digit5'); const rows = [await page.evaluate(() => { J.tick(60); window.__draw.smoke.anew(); J.tick(); window.__draw.smoke.emitters.length = 0; return J.frame(1 / 30); })];
    for (let s = 1; s <= 6; s++) { rows.push(await page.evaluate(async () => { let r; for (let i = 0; i < 30; i++) { J.tick(2); r = await (i < 29 ? J.step(1 / 30) : J.frame(1 / 30)); } return r; })); const r = rows[s]; note(`${s} s`, JSON.stringify({ game: r.game, drawn: r.drawn, sight: r.sight, blind: r.blind })); }
    const end = rows[6], worst = Math.max(...rows.map(r => r.sight));
    check(rows[0].game > 5 && Math.abs(rows[0].drawn / rows[0].game - 1) < 0.06, 'a grenade\'s dust is taken whole as it hangs', { game: rows[0].game, drawn: rows[0].drawn });
    check(end.game < rows[0].game * 0.6, 'the game\'s thins', { then: rows[0].game, now: end.game });
    check(Math.abs(end.drawn / end.game - 1) < 0.2, 'six seconds on as much of it is drawn as the game has', { game: end.game, drawn: end.drawn, share: +(end.drawn / end.game).toFixed(2) });
    // (what is taken whole is 0.06 off by its billows alone, which are finer than the game's cells)
    check(worst < 0.11 && rows.every(r => r.blind < 0.03), 'a line of sight is the game\'s all the while, but for billows', rows.map(r => [r.sight, r.blind]));
  }
  check(errors.length === 0, 'nothing threw', errors.slice(0, 4));
  await PAGE.end();
})().catch(e => { console.error(e); process.exit(2); });
