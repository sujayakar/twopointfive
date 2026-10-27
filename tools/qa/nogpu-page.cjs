// A START WITH NO GPU (src/gpu/adapter.ts GPU_ASK, the card in src/gpu/device.ts showFatal), on the real page in headless Chromium, software WebGPU adapter.
// `navigator.gpu.requestAdapter` is stubbed before the page's own script runs (?stub=none: nothing for the fast card or for any card, the real answer for the
// software ask · ?stub=first: nothing for the fast card, the real answer after it):
//   1. no card from either ask: the red card with its sentence, the lines of what the machine said and a Copy button that is on top, fits the window and
//      puts the text on the clipboard;
//   2. the same under ?gpuask=0: one ask and the old one-line card;
//   3. an empty first ask: on this box the second answers with the software adapter, which is not a start (a box with a real card would start on it);
//   4. a plain boot: no card, and a recorder's take still downloads through the one door (src/host/savefile.ts).
//   node tools/qa/nogpu-page.cjs        env: PORT (the dev server's, default 5199) · NOGPU_ONLY=card = steps 1 to 3 alone (seconds; step 4 boots the game)
// Nothing here is a machine that really has no adapter, and no real card is ever asked. Exit 0 = every check passed, 1 = one failed or a page did not boot.
//   NOGPU_BREAK=1: every page runs under ?gpuask=0 — must FAIL.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), os = require('os'), path = require('path');
const PORT = process.env.PORT || 5199, BREAK = process.env.NOGPU_BREAK === '1' ? '&gpuask=0' : '', ONLY = process.env.NOGPU_ONLY || '';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let red = 0; const check = (ok, what) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const ctx = await browser.newContext({ viewport: { width: 1000, height: 640 }, deviceScaleFactor: 1, acceptDownloads: true, permissions: ['clipboard-read', 'clipboard-write'] });
  await ctx.addInitScript(() => {
    window.__testVisible = true;
    const how = new URLSearchParams(location.search).get('stub'); if (!how || !navigator.gpu) return;
    const real = navigator.gpu.requestAdapter.bind(navigator.gpu); window.__asked = [];
    navigator.gpu.requestAdapter = o => { window.__asked.push(o?.forceFallbackAdapter ? 'soft' : o?.powerPreference ? 'fast' : 'any');
      if (how === 'none') return Promise.resolve(o?.forceFallbackAdapter ? real(o) : null).then(a => a);
      if (how === 'first') return o?.powerPreference ? Promise.resolve(null) : real(o);
      return real(o); };
  });
  const page = await ctx.newPage();
  const errs = [], infos = []; page.on('pageerror', e => errs.push(String(e).slice(0, 300)));
  page.on('console', m => { if (m.type() === 'info') infos.push(m.text().slice(0, 300)); if (m.type() !== 'error') return; const at = m.location().url || ''; if (!(/\/favicon\.ico$/.test(at) || /^http:\/\/localhost:5174\//.test(at))) errs.push(m.text().slice(0, 300)); });
  const downloads = []; page.on('download', d => downloads.push(d));
  const cardOf = () => page.evaluate(() => { const el = document.getElementById('fatal'); const b = el?.querySelector('button'); const r = b?.getBoundingClientRect();
    return { up: el?.style.display === 'flex', text: el?.innerText ?? '', kids: el?.children.length ?? 0, button: b?.textContent ?? '', top: b && r ? document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2) === b : false, asked: window.__asked ?? [], fits: el ? el.scrollHeight <= el.clientHeight + 1 : false }; });

  // 1. no card from either ask
  await page.goto(`http://localhost:${PORT}/?paused&quality=low&mute&stub=none${BREAK}`, { waitUntil: 'load' });
  await page.waitForFunction(() => document.getElementById('fatal')?.style.display === 'flex', null, { timeout: 60000 });
  let c = await cardOf(); log('card:\n' + c.text);
  check(c.up && c.text.startsWith("The game couldn't start.") && c.kids === 3, 'the card is up with its words, the lines and a button');
  check(c.asked.join() === 'fast,any,soft', `the browser was asked three ways (${c.asked.join()})`);
  check(/asked for the fast card: nothing/.test(c.text) && /asked for any card: nothing/.test(c.text) && /asked for software: \S/.test(c.text) && /WebGL: \S/.test(c.text) && /browser: Mozilla/.test(c.text) && /page: secure address/.test(c.text), 'the lines say what each ask got, WebGL, the browser and the page');
  check(c.button === 'Copy' && c.top && c.fits, 'the Copy button is on top and the card fits the window');
  await page.click('#fatal button'); await page.waitForTimeout(400);
  const clip = await page.evaluate(() => navigator.clipboard.readText()).catch(e => 'ERR ' + e); c = await cardOf();
  check(c.button === 'Copied' && clip.startsWith("The game couldn't start.\nNo WebGPU adapter available.\nasked for the fast card: nothing") && /browser: Mozilla/.test(clip), `Copy puts the first line and the lines on the clipboard (${clip.length} chars)`);
  const only = errs.filter(e => !/No WebGPU adapter available|NoGpuError/.test(e));
  check(only.length === 0, `no other error on the page (${only.length}${only.length ? ': ' + only[0] : ''})`);
  // 2. the switch off: the one line, as it was
  errs.length = 0;
  await page.goto(`http://localhost:${PORT}/?paused&quality=low&mute&stub=none&gpuask=0`, { waitUntil: 'load' });
  await page.waitForFunction(() => document.getElementById('fatal')?.style.display === 'flex', null, { timeout: 60000 });
  c = await cardOf();
  check(c.up && c.kids === 0 && /^Error: No WebGPU adapter available\./.test(c.text) && c.asked.join() === 'fast', `?gpuask=0: one ask and the old card (${c.asked.join()} · ${c.text.slice(0, 60).replace(/\n/g, ' ')})`);
  // 3. first ask empty: on this box the second answers with the software adapter, which is not a start
  await page.goto(`http://localhost:${PORT}/?paused&quality=low&mute&stub=first${BREAK}`, { waitUntil: 'load' });
  await page.waitForFunction(() => document.getElementById('fatal')?.style.display === 'flex' || !!window.__engine, null, { timeout: 120000 }).catch(() => {});
  c = await cardOf(); log('first-empty card:\n' + c.text.slice(0, 700));
  check(c.up ? /asked for any card: .*software adapter/.test(c.text) && c.asked.join() === 'fast,any' : infos.some(l => /came back empty/.test(l)), c.up ? "an empty first ask, a software second: the card, with that adapter named" : 'an empty first ask, a card from the second: the game starts on it');
  if (ONLY === 'card') { await browser.close(); console.log(red ? `${red} FAILED` : 'all pass'); process.exit(red ? 1 : 0); }
  // 4. a plain boot, and a take through the door
  errs.length = 0;
  await page.setViewportSize({ width: 320, height: 180 });
  await page.goto(`http://localhost:${PORT}/?paused&quality=low&mute${BREAK}`, { waitUntil: 'commit' });
  await page.waitForFunction(() => !!window.__ready, null, { timeout: 1500000 }); await page.evaluate(async () => { await window.__ready; }); log('ready');
  check(await page.evaluate(() => document.getElementById('fatal')?.style.display !== 'flex'), 'a plain boot puts no card up');
  await page.evaluate(() => { window.__menu?.close?.(); window.__perfrec.start('door check'); window.__runFrames(6); window.__perfrec.stop(); });
  await page.waitForTimeout(3000);
  check(downloads.length === 1 && /^perf-\d{4}-.*\.json$/.test(downloads[0]?.suggestedFilename() ?? ''), `one file downloaded, named as a take (${downloads.map(d => d.suggestedFilename()).join(', ')})`);
  if (downloads[0]) { const p = path.join(fs.mkdtempSync(path.join(os.tmpdir(), 'nogpu-')), 'take.json'); await downloads[0].saveAs(p); let j = null; try { j = JSON.parse(fs.readFileSync(p, 'utf8')); } catch (e) { /* */ }
    check(!!j && j.version === 1 && Array.isArray(j.frames) && Array.isArray(j.events), `the file is the take: ${fs.statSync(p).size} bytes`); }
  check(!errs.length, `no page or console error (${errs.length}${errs.length ? ': ' + errs[0] : ''})`);
  await browser.close(); console.log(red ? `${red} FAILED` : 'all pass'); process.exit(red ? 1 : 0);
})().catch(e => { console.log('- FAIL — the script died: ' + String(e).slice(0, 400)); process.exit(1); });
