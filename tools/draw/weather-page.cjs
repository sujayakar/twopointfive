// THE NEW PAGE DRAWS A SNOW LEVEL, A WET NIGHT AND A FIRE: draw.html on the software adapter, stepped by hand, read by what its picture holds, a still of each. Never a GPU's judgement: what
// it holds is that the rows reach the picture and nothing throws. The page's own map of prints, flakes in the ring, the drip lines, a crater, the air's map, a fire's records.
//   PORT=<a dev server> node tools/draw/weather-page.cjs [--out dir]      exit 0 = every check passed, 1 = a FAIL, 2 = the script died
const { chromium } = (() => { try { return require('playwright'); } catch { return require('/opt/node-tools/node_modules/playwright'); } })();
const { mkdirSync } = require('node:fs');
const PORT = process.env.PORT || 5199, at = process.argv.indexOf('--out'), OUT = at > 0 ? process.argv[at + 1] : null;
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--no-sandbox'];
let fails = 0;
const check = (ok, what, got) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${got === undefined ? '' : `   (${JSON.stringify(got)})`}`); if (!ok) fails++; };
async function open(browser, words) {
  const page = await browser.newPage({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 }), errors = [];
  page.on('pageerror', x => errors.push(String(x && x.stack || x).slice(0, 500)));
  page.on('console', m => { const t = m.text(); if (m.type() === 'error' || /Invalid|error:/.test(t)) errors.push(t.slice(0, 300)); });
  await page.goto(`http://localhost:${PORT}/draw.html?paused&quality=verylow&mute&seed=5&${words}`, { timeout: 300000 });
  await page.evaluate(() => window.__ready);
  return { page, errors };
}
const still = async (page, name) => { if (OUT) { mkdirSync(OUT, { recursive: true }); await page.evaluate(() => window.__draw.engine.device.queue.onSubmittedWorkDone()); await page.screenshot({ path: `${OUT}/${name}.png`, timeout: 240000 }); } };
(async () => {
  const browser = await chromium.launch({ headless: true, executablePath: process.env.CHROMIUM || '/usr/bin/chromium', args: FLAGS });
  { // ---- a snow level with a fire
    const { page, errors } = await open(browser, 'levelFile=pass.json');
    const s0 = await page.evaluate(() => { const d = window.__draw; d.run(60, [], 'last'); const w = d.weather, P = d.engine.particles; return { snow: !!w.snow?.field, quads: w.snow?.field?.quads.length ?? 0, tracks: !!w.snow?.tracks, ring: P.rainStats.emitted, fires: d.engine.fires.length, firePos: d.engine.fires[0] ? Array.from(d.engine.fires[0].pos) : null, disc: Array.from(d.engine.weather.nearFade), air: !!d.engine.air, snowW: d.engine.weather.snow }; });
    check(s0.snow && s0.quads > 0, 'the pass: the snow lies in a field baked from the rows', s0);
    check(s0.tracks, 'the pass: a map of prints stands');
    check(s0.ring > 0 || s0.disc[2] > 0, 'the pass: flakes are filed in the view\'s disc', { emitted: s0.ring, disc: s0.disc });
    check(s0.fires > 0, 'the pass: a fire\'s record reaches the composite', s0.fires);
    await still(page, 'pass-1-standing');
    const s1 = await page.evaluate(() => { const d = window.__draw; d.run(180, ['KeyW'], 'last'); const T = d.weather.snow.tracks; let n = 0; for (let i = 0; i < T.data.length; i++) if (T.data[i]) n++; return { prints: T.prints, texels: n, kicked: T.kicked, wantAir: d.engine.airOn }; });
    check(s1.prints > 0 && s1.texels > 0, 'the pass: a walk presses prints into the map', s1);
    await still(page, 'pass-2-walked');
    // (the row that tells it is held by rust/game_start/tests/weather_rows.rs: a frag cannot be thrown by this script's hand)
    // a blast told to the picture where he stands: the crater in the map, the throw in the ring, the shove to the air
    const s3 = await page.evaluate(() => { const d = window.__draw, S = d.weather.snow, a = d.cam.target, T = S.tracks; let n0 = 0; for (let i = 0; i < T.data.length; i++) if (T.data[i]) n0++;
      const ok = S.blasts.at(S.field, T, a[0], 0.2, a[2], 2.5, (x, z, o, s, t) => d.weather.air.puff(x, z, o, s, t), 0); d.run(60, [], 'last'); let n1 = 0; for (let i = 0; i < T.data.length; i++) if (T.data[i]) n1++; return { ok, craters: S.blasts.craters, n0, n1 }; });
    check(s3.ok && s3.craters === 1 && s3.n1 > s3.n0, 'the pass: a blast over the snow leaves its crater', s3);
    await still(page, 'pass-3-after');
    check(errors.length === 0, 'the pass: nothing threw and no GPU error was said', errors.slice(0, 4));
    await page.close();
  }
  { // ---- a wet night
    const { page, errors } = await open(browser, 'levelFile=compound.json&weather=downpour&roomclear&squad4');
    const s = await page.evaluate(() => { const d = window.__draw; d.run(90, [], 'last'); const W = d.engine.weather, R = d.weather.rain, P = d.engine.particles; return { rain: W.rain, wet: W.wet, disc: Array.from(W.nearFade), phase: W.phase ? Array.from(W.phase) : null, drips: R.dripPoints?.length ?? -1, live: P.rainStats.live, emitted: P.rainStats.emitted }; });
    check(s.rain > 0 && s.wet >= 0, 'a downpour: the sky\'s section says rain', s);
    check(s.disc[2] > 0 && s.phase, 'a downpour: the near drops stand in a disc, and the lattice is told', { disc: s.disc, phase: s.phase });
    check(s.drips > 0, 'a downpour: drip lines are found off the statics', s.drips);
    await still(page, 'downpour');
    check(errors.length === 0, 'a downpour: nothing threw and no GPU error was said', errors.slice(0, 4));
    await page.close();
  }
  await browser.close();
  console.log(fails ? `=== weather page: ${fails} FAILED ===` : '=== weather page: ALL PASS ===');
  process.exit(fails ? 1 : 0);
})().catch(e => { console.error(e); process.exit(2); });
