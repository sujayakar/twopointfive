// A START WITH NO GPU (src/gpu/adapter.ts GPU_ASK, the card in src/gpu/device.ts showFatal), on the real page in headless Chromium, software WebGPU adapter.
// `navigator.gpu.requestAdapter` is stubbed before the page's own script runs (?stub=none: nothing for the fast card or for any card, the real answer for the
// software ask · ?stub=first: nothing for the fast card, the real answer after it):
//   0. ?stub=wasm: WebAssembly gone before the page's script: the red card says so in one line and asks for another browser;
//   1. no card from either ask: the red card with its sentence, the lines of what the machine said and a Copy button that is on top, fits the window and
//      puts the text on the clipboard;
//   2. the same under ?gpuask=0: one ask and the old one-line card;
//   3. an empty first ask: on this box the second answers with the software adapter, which is not a start (a box with a real card would start on it);
//   4. a plain boot: no card, and a recorder's take goes out through the one door with no download (src/host/savefile.ts): nobody listens on this page (?cloud=0),
//      so the recorder knows it went nowhere.
//   node tools/qa/nogpu-page.cjs        env: PORT (the dev server's, default 5199) · NOGPU_ONLY=card = steps 0 to 3 alone (seconds; step 4 boots the game), wasm = step 0
// Nothing here is a machine that really has no adapter, and no real card is ever asked. Exit 0 = every check passed, 1 = one failed or a page did not boot.
//   NOGPU_BREAK=1: every page runs under ?gpuask=0 — must FAIL.
const { page: pageRun, NOT_THE_PAGE } = require('./page.cjs'), PAGE = pageRun({ name: 'nogpu-page' }), { check, note: log } = PAGE;
const BREAK = process.env.NOGPU_BREAK === '1' ? '&gpuask=0' : '', ONLY = process.env.NOGPU_ONLY || '';
(async () => {
  const { page, errors: errs, ready } = await PAGE.open({ size: [1000, 640], options: { acceptDownloads: true, permissions: ['clipboard-read', 'clipboard-write'] }, errors: { skipAt: NOT_THE_PAGE }, init: [() => {
    window.__testVisible = true;
    const how = new URLSearchParams(location.search).get('stub'); if (how === 'wasm') { delete window.WebAssembly; return; } if (!how || !navigator.gpu) return;
    const real = navigator.gpu.requestAdapter.bind(navigator.gpu); window.__asked = [];
    navigator.gpu.requestAdapter = o => { window.__asked.push(o?.forceFallbackAdapter ? 'soft' : o?.powerPreference ? 'fast' : 'any');
      if (how === 'none') return Promise.resolve(o?.forceFallbackAdapter ? real(o) : null).then(a => a);
      if (how === 'first') return o?.powerPreference ? Promise.resolve(null) : real(o);
      return real(o); };
  }] });
  const infos = []; page.on('console', m => { if (m.type() === 'info') infos.push(m.text().slice(0, 300)); });
  const downloads = []; page.on('download', d => downloads.push(d));
  const cardOf = () => page.evaluate(() => { const el = document.getElementById('fatal'); const b = el?.querySelector('button'); const r = b?.getBoundingClientRect();
    return { up: el?.style.display === 'flex', text: el?.innerText ?? '', kids: el?.children.length ?? 0, button: b?.textContent ?? '', top: b && r ? document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2) === b : false, asked: window.__asked ?? [], fits: el ? el.scrollHeight <= el.clientHeight + 1 : false }; });

  // 0. no WebAssembly (the line is the maintainer's pick)
  await page.goto(PAGE.url('paused&quality=low&mute&stub=wasm'), { waitUntil: 'load' });
  await page.waitForFunction(() => document.getElementById('fatal')?.style.display === 'flex', null, { timeout: 60000 });
  { const w = await cardOf(); log('card:\n' + w.text); check(w.text.startsWith("This game needs WebAssembly, and it's turned off in this browser. Try another browser."), 'with no WebAssembly the card says so and nothing else is tried'); }
  if (ONLY === 'wasm') { await PAGE.end(); return; }

  // 1. no card from either ask
  await page.goto(PAGE.url(`paused&quality=low&mute&stub=none${BREAK}`), { waitUntil: 'load' });
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
  await page.goto(PAGE.url('paused&quality=low&mute&stub=none&gpuask=0'), { waitUntil: 'load' });
  await page.waitForFunction(() => document.getElementById('fatal')?.style.display === 'flex', null, { timeout: 60000 });
  c = await cardOf();
  check(c.up && c.kids === 0 && /^Error: No WebGPU adapter available\./.test(c.text) && c.asked.join() === 'fast', `?gpuask=0: one ask and the old card (${c.asked.join()} · ${c.text.slice(0, 60).replace(/\n/g, ' ')})`);
  // 3. first ask empty: on this box the second answers with the software adapter, which is not a start
  await page.goto(PAGE.url(`paused&quality=low&mute&stub=first${BREAK}`), { waitUntil: 'load' });
  await page.waitForFunction(() => document.getElementById('fatal')?.style.display === 'flex' || !!window.__engine, null, { timeout: 120000 }).catch(() => {});
  c = await cardOf(); log('first-empty card:\n' + c.text.slice(0, 700));
  check(c.up ? /asked for any card: .*software adapter/.test(c.text) && c.asked.join() === 'fast,any' : infos.some(l => /came back empty/.test(l)), c.up ? "an empty first ask, a software second: the card, with that adapter named" : 'an empty first ask, a card from the second: the game starts on it');
  if (ONLY === 'card') await PAGE.end(PAGE.fails ? `${PAGE.fails} FAILED` : 'all pass');
  // 4. a plain boot, and a take through the door
  errs.length = 0;
  await page.setViewportSize({ width: 320, height: 180 });
  await page.goto(PAGE.url(`paused&quality=low&mute&cloud=0${BREAK}`), { waitUntil: 'commit' });   // (cloud=0: nobody listens at the door, whatever backend this dev server runs)
  await ready(); log('ready');
  check(await page.evaluate(() => document.getElementById('fatal')?.style.display !== 'flex'), 'a plain boot puts no card up');
  await page.evaluate(() => { window.__menu?.close?.(); window.__perfrec.start('door check'); window.__runFrames(6); window.__perfrec.stop(); });
  await page.waitForTimeout(3000);
  const take = await page.evaluate(() => { const D = window.__lastPerfDump; return { went: window.__perfrec.lastSent, ok: !!D && D.version === 1 && Array.isArray(D.frames) && Array.isArray(D.events), frames: D?.frames?.length ?? -1 }; });
  check(downloads.length === 0 && take.ok && take.went === false, `the take was made (${take.frames} frames), no file was downloaded (${downloads.length}), and with nobody listening the recorder knows it wasn't sent (${take.went})`);
  check(!errs.length, `no page or console error (${errs.length}${errs.length ? ': ' + errs[0] : ''})`);
  await PAGE.end(PAGE.fails ? `${PAGE.fails} FAILED` : 'all pass');
})().catch(e => { console.log('- FAIL — the script died: ' + String(e).slice(0, 400)); process.exit(1); });
