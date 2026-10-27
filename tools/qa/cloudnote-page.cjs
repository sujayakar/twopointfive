// THE NOTICE LINE ON A CARD THAT IS NOT THE TITLE (src/ui/cloudnote.ts, its `off` way in: a benchmark's result card, the error card of a page that never
// started), in a real browser:   node tools/qa/cloudnote-page.cjs
// The module is bundled by itself and put on an empty page: no game, no server, no GPU. What it holds: the line counts as SEEN only after two seconds in the
// window with nothing over it; a line under another element, or scrolled out of the window, never does; the title's own way still wants the live title.
// EXIT 0 all pass · 1 a failure.
const { chromium } = require('playwright');
const { execFileSync } = require('node:child_process');
const path = require('node:path');
const ROOT = path.resolve(__dirname, '../..');
let fails = 0; const check = (ok, text) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); if (!ok) fails++; };
(async () => {
  const js = execFileSync('bun', ['build', path.join(ROOT, 'src/ui/cloudnote.ts'), '--target', 'browser'], { encoding: 'utf8', maxBuffer: 1 << 24 });
  const browser = await chromium.launch(); const page = await browser.newPage({ viewport: { width: 900, height: 600 } });
  await page.setContent('<body style="margin:0"><div id="a"></div><div id="b" style="position:relative"></div><div style="height:2000px"></div><div id="c"></div><div id="d"></div></body>');
  // (the bundle is a module: its export line is dropped and the one function is put on the window)
  await page.addScriptTag({ type: 'module', content: js.replace(/export\s*\{[^}]*\};?/g, '') + '\n;window.CN = { cloudNote };' });
  await page.waitForFunction(() => !!window.CN, null, { timeout: 5000 }).catch(() => {});
  const has = await page.evaluate(() => typeof window.CN?.cloudNote === 'function');
  if (!has) { console.log('- FAIL — the module did not load on the page'); await browser.close(); process.exit(1); }
  await page.evaluate(() => {
    window.T = { a: 0, b: 0, c: 0, d: 0 }; const t0 = performance.now(), at = k => () => { window.T[k] = performance.now() - t0; };
    CN.cloudNote(document.getElementById('a'), undefined, { told: at('a') });                               // in the window, on top
    CN.cloudNote(document.getElementById('b'), undefined, { told: at('b') });                               // under another element
    const over = document.createElement('div'); over.style.cssText = 'position:absolute;inset:0;background:#000'; document.getElementById('b').appendChild(over);
    CN.cloudNote(document.getElementById('c'), undefined, { told: at('c') });                               // under the fold
    window.__cloud = { share: true, told: at('d') }; CN.cloudNote(document.getElementById('d'), () => {});  // the title's way, and no live title
  });
  await page.waitForTimeout(1400); const early = await page.evaluate(() => window.T.a);
  await page.waitForTimeout(2600); const T = await page.evaluate(() => window.T);
  const words = await page.evaluate(() => document.querySelector('#a .cloudnote')?.textContent ?? '');
  check(early === 0 && T.a >= 2000 && T.a < 3400, `a card that is not the title: the line is seen after two seconds on top in the window (${Math.round(T.a)} ms), not before`);
  check(T.b === 0 && T.c === 0, 'a line under another element, and one scrolled out of the window, are never seen');
  check(T.d === 0, 'the title\'s own line still wants the live title');
  check(/sends your replays, stats, settings and saved files/.test(words), `the line reads as on the title ("${words.slice(0, 48)}…")`);
  await browser.close();
  console.log(fails ? `\n${fails} FAILED` : '\nall pass'); process.exit(fails ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
