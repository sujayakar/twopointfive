// POUCH LOOK: the member cards and the loadout screen under the pouches rule (game/kit.ts SLOTS), as the real page draws them in headless Chromium on the
// SOFTWARE WebGPU adapter (SwiftShader — no GPU; the cards are HTML, so what a still shows of THEM is what a browser draws; nothing here says how the frame
// behind them looks on a real GPU). Stills:
//   loadout-issued · loadout-refused · loadout-you · loadout-off(-you)   the loadout screen on the room clear with four
//   loadout-squadkit    the Squad kit row pressed: 'Stun heavy' on all five · loadout-wall  'One wall charge' (only in a build whose wall
//                       charge takes its count from the cards: the button is not there otherwise, and the notes say so)
//   node tools/qa/pouch-look.cjs <outDir>      env: PORT (5199) · QUALITY (verylow) · STILLS=0 (no still is taken: the state lines alone, for a box too
//                                               slow to draw them)
// A STILL THAT THE ADAPTER NEVER DRAWS IS NOT A RED. A still waits for a frame; on the software adapter beside other runs a frame can take longer than any
// sane limit, and the run used to die there with no FAIL line and exit 1 — read as a red of the game. Each still is now tried twice; one that still gives no
// frame is listed at the end as NOT SAVED and the run goes on to the next state line. The exit is 1 for a page error or a step that threw, never for a still.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs');
const path = require('path');
const outDir = process.argv[2] || 'pouch-look';
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow';
const SHOT_MS = 240000;   // (a still waits for a frame, and the software adapter's can take minutes with other runs beside it: Playwright's own 30 s gave no reading at all)
const STILLS = process.env.STILLS !== '0', MISSED = [];
/** one still: tried twice, never thrown — a frame the adapter did not give in time is a still not saved, said at the end */
const still = async (page, o) => {
  const name = path.basename(o.path, '.png'); if (!STILLS) return false;
  for (let k = 0; k < 2; k++) { try { await page.screenshot({ timeout: SHOT_MS, ...o }); return true; } catch (e) { if (!/Timeout|timeout/.test(String(e && e.message || e))) throw e; } }
  MISSED.push(name); return false;
};
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/;
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
fs.mkdirSync(outDir, { recursive: true });
const BASE = `http://localhost:${PORT}/?paused&quality=${QUALITY}&mute&cards`;
(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const R = { errors: [], noise: 0, notes: [] };
  const open = async (q, sel, store) => {
    const page = await browser.newPage({ viewport: { width: 1400, height: 1000 }, deviceScaleFactor: 1 });
    page.setDefaultTimeout(SHOT_MS);   // (a click waits for the element to hold still over two frames: as slow as a still on this adapter, and Playwright's own 30 s ends the run)
    page.on('console', x => { const t = x.type(), s = x.text(); if (t !== 'error' || /Failed to load resource/.test(s)) return; if (NOISE.test(s) && R.noise < 200) R.noise++; else R.errors.push(`${q}: ${s.slice(0, 300)}`); });
    page.on('pageerror', x => R.errors.push(`${q} pageerror: ` + String(x && x.stack || x).slice(0, 500)));
    if (store !== undefined) await page.addInitScript((s) => { try { if (s === null) localStorage.removeItem('twopointsix.kit'); else localStorage.setItem('twopointsix.kit', s); } catch {} }, store);
    await page.goto(BASE + q, { timeout: 240000 }); await page.waitForSelector(sel, { timeout: 240000 });   // (the software adapter boots slowly, slower still beside other work)
    await page.evaluate(async () => { await window.__ready; window.__testVisible = true; });
    return page;
  };
  try {
    // ── the loadout screen (room clear with four), Red One picked
    for (const [name, q] of [['loadout', ''], ['loadout-off', '&noslots']]) {
      const page = await open(`&loadout=room-clear-four${q}`, '#loadout .lo-slot', null); log(`${name}: ready`);
      await page.evaluate(() => window.__kit.reset());
      await page.click('#loadout .lo-slot[data-slot="1"]');
      await page.waitForTimeout(150);
      if (name === 'loadout') { await still(page, { path: path.join(outDir, 'loadout-issued.png') });
        await page.click('#loadout .kc .kc-row[data-field="bangs"] .kc-step:last-of-type'); await page.waitForTimeout(100); await still(page, { path: path.join(outDir, 'loadout-refused.png') });
        R.notes.push('loadout: slots ' + JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('#loadout .lo-pouch')].map(x => x.textContent))) + ' · squad ' + JSON.stringify(await page.evaluate(() => document.querySelector('#loadout .lo-squadp')?.textContent)) + ' · pool ' + JSON.stringify(await page.evaluate(() => document.querySelector('#loadout .lo-pool')?.textContent)) + ' · fits the window: ' + JSON.stringify(await page.evaluate(() => { const b = document.querySelector('#loadout .lo-card')?.getBoundingClientRect(); return b ? { w: Math.round(b.width), h: Math.round(b.height), inW: b.right <= innerWidth && b.left >= 0, inH: b.bottom <= innerHeight && b.top >= 0, scroll: document.querySelector('#loadout .lo-card').scrollHeight > document.querySelector('#loadout .lo-card').clientHeight } : null; })));
      } else { await still(page, { path: path.join(outDir, 'loadout-off.png') }); R.notes.push('loadout-off: foot ' + JSON.stringify(await page.evaluate(() => document.querySelector('#loadout .lo-foot')?.textContent))); }
      if (name === 'loadout') {   // the Squad kit row: one press puts a loadout of the table on all five
        await page.evaluate(() => window.__kit.reset()); R.notes.push('squad kit buttons: ' + JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('#loadout .lo-set')].map(x => x.textContent))));
        await page.click('#loadout .lo-set[data-kit="stun"]'); await page.waitForTimeout(100); await still(page, { path: path.join(outDir, 'loadout-squadkit.png') });
        if (await page.$('#loadout .lo-set[data-kit="wall"]')) { await page.click('#loadout .lo-set[data-kit="wall"]'); await page.click('#loadout .lo-slot[data-slot="1"]'); await page.waitForTimeout(100); await still(page, { path: path.join(outDir, 'loadout-wall.png') }); R.notes.push('loadout-wall: pool ' + JSON.stringify(await page.evaluate(() => document.querySelector('#loadout .lo-pool')?.textContent)) + ' · walls ' + JSON.stringify(await page.evaluate(() => window.__kit.walls(4)))); }
        else R.notes.push('loadout-wall: no "One wall charge" button — this build\'s wall charge takes no count from the cards');
        await page.click('#loadout .lo-set[data-kit="issued"]');
      }
      // your own card too (seven rows under the rule)
      await page.click('#loadout .lo-slot[data-slot="0"]'); await page.waitForTimeout(150); await still(page, { path: path.join(outDir, `${name}-you.png`) });
      await page.close();
    }
  } catch (e) { R.errors.push('threw: ' + String(e && e.stack || e).slice(0, 800)); }
  await browser.close();
  R.stillsMissed = MISSED; R.stills = STILLS;
  fs.writeFileSync(path.join(outDir, 'report.json'), JSON.stringify(R, null, 1));
  for (const n of R.notes) log(n);
  if (!STILLS) log('STILLS=0: no still taken, the state lines above are the whole reading');
  else if (MISSED.length) log(`${MISSED.length} still(s) NOT SAVED, the adapter gave no frame in ${SHOT_MS / 1000} s twice (not a page error): ${MISSED.join(', ')}`);
  log(R.errors.length ? `ERRORS (${R.errors.length}):\n` + R.errors.join('\n') : `no page errors (${R.noise} known software-adapter lines)`);
  process.exit(R.errors.length ? 1 : 0);
})();
