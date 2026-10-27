// THE SAVE DOOR ON A REAL PAGE (src/host/savefile.ts: a saved file goes to the backend and no download is made), Chromium on the software adapter, against a
// STAND-IN backend on this machine (cloud/test/standin.ts: `npm ci` in cloud/ first). Three pages of the dev server, booted side by side:
//   A  ?cloud=<the stand-in>: an F9 take makes NO download, the stand-in has the file, the recorder knows it is being sent and then that it arrived, and the
//      notes say so; the benchmark card's line for each way a run can go, its move to "were sent", and Close waiting for a file that is on its way
//      (the card is put up by hand: a whole ?bench run is tools/qa/bench-page.cjs);
//   B  ?downloads=1&cloud=0: the door as it was, one download and the old note;
//   C  ?cloud=0: nobody listens. No download, the notes say the take stopped and then that it was saved nowhere (no shot server here), and the card
//      says the results weren't sent, with no notice line.
//   PORT=<dev server> node tools/qa/savedoor-page.cjs      (three boots of the game: many minutes on the software adapter)
// Exit 0 = every check passed, 1 = one failed or a page did not boot.   SAVEDOOR_BREAK=1: page A is opened under ?downloads=1 as well — must FAIL.
// Nothing here is a real card, a real backend or a real network: never seen on a GPU.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const path = require('node:path'), cp = require('node:child_process');
const PORT = process.env.PORT || 5199, CPORT = Number(process.env.CPORT || 3219), ROOT = path.resolve(__dirname, '../..'), BREAK = process.env.SAVEDOOR_BREAK === '1' ? '&downloads=1' : '';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const CLOUD = `http://127.0.0.1:${CPORT}`, KEY = 'page-check-key';
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let red = 0; const check = (ok, what) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
const sleep = ms => new Promise(r => setTimeout(r, ms));
const admin = async (kind, fn, args) => { const r = await fetch(`${CLOUD}/api/${kind}`, { method: 'POST', headers: { 'Content-Type': 'application/json', Authorization: `Convex ${KEY}` }, body: JSON.stringify({ path: fn, format: 'convex_encoded_json', args: [args] }) }); const j = await r.json(); if (j.status !== 'success') throw new Error(JSON.stringify(j).slice(0, 300)); return j.value; };
const heard = async () => (await (await fetch(`${CLOUD}/__stats`)).json()).calls;
const open = async (ctx, q) => { const page = await ctx.newPage(); const errs = [], downloads = [];
  page.on('pageerror', e => errs.push(String(e).slice(0, 300)));
  page.on('console', m => { if (m.type() !== 'error') return; const at = m.location().url || ''; if (!(/\/favicon\.ico$/.test(at) || /^http:\/\/localhost:5174\//.test(at) || /Failed to load resource/.test(m.text()))) errs.push(m.text().slice(0, 300)); });
  page.on('download', d => downloads.push(d));
  await page.goto(`http://localhost:${PORT}/?paused&quality=low&mute${q}`, { waitUntil: 'commit' });
  return { page, errs, downloads, ready: (async () => { await page.waitForFunction(() => !!window.__ready, null, { timeout: 1500000 }); await page.evaluate(async () => { await window.__ready; }); })() }; };
const take = page => page.evaluate(() => { const P = window.__perfrec, notes = [], old = P.hudNote; P.hudNote = s => { if (!/^● REC/.test(s)) notes.push(s); try { old(s); } catch (e) { /* */ } };
  window.__menu?.close?.(); P.start('door check'); window.__runFrames(6); P.stop(); window.__notes = notes; return { notes: [...notes], went: P.lastSent }; });
(async () => {
  const standin = cp.spawn('bun', ['run', 'test/standin.ts', String(CPORT)], { cwd: path.join(ROOT, 'cloud'), env: { ...process.env, STANDIN_KEY: KEY }, stdio: 'ignore' });
  let up = false; for (let i = 0; i < 60; i++) { try { await heard(); up = true; break; } catch { await sleep(250); } }
  check(up, 'the stand-in backend is up'); if (!up) { standin.kill(); process.exit(1); }
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const ctx = await browser.newContext({ viewport: { width: 320, height: 180 }, deviceScaleFactor: 1, acceptDownloads: true });
  await ctx.addInitScript(() => { window.__testVisible = true; });
  try {
    const A = await open(ctx, `&cloud=${encodeURIComponent(CLOUD)}${BREAK}`), B = await open(ctx, '&downloads=1&cloud=0'), C = await open(ctx, '&cloud=0');
    // ---- A: a backend that sends
    await A.ready; log('A ready');
    const c = await A.page.evaluate(() => window.__cloud ? { share: window.__cloud.share } : null);
    check(!!c && c.share === true, `A: the page has a backend and sends (${JSON.stringify(c)})`);
    const a = await take(A.page);
    let rows = []; for (let i = 0; i < 40; i++) { await sleep(500); try { rows = await admin('query', 'files:list', {}); } catch (e) { log(String(e)); } if (rows.length) break; }
    check(A.downloads.length === 0, `A: no file was downloaded (${A.downloads.length})`);
    check(a.went === 'sent' && a.notes.join(' | ') === 'The recording is being sent to the developer.', `A: the recorder knows it is being sent and the note says so (${a.went}: ${a.notes.join(' | ')})`);
    await sleep(2000); const a2 = await A.page.evaluate(() => ({ notes: window.__notes, went: window.__perfrec.lastSent }));
    check(a2.went === 'arrived' && a2.notes.join(' | ') === 'The recording is being sent to the developer. | The recording was sent to the developer.', `A: once the file is on the backend the recorder knows it arrived and a second note says so (${a2.went}: ${a2.notes.join(' | ')})`);
    check(rows.length === 1 && rows[0].kind === 'perf' && /^perf-.*\.json$/.test(rows[0].name) && rows[0].size > 500, `A: the backend has the take (${rows.map(r => `${r.kind} ${r.name} ${r.size}`).join(' · ')})`);
    // the card, put up by hand with each answer
    const said = await A.page.evaluate(() => { const V = window.__bench.deps.view, out = [];
      for (const w of [{ result: 'sent', take: 'sent' }, { result: 'held', take: 'held' }, { result: false, take: false }, { result: 'sent', take: false }]) { V.show('a report', () => {}, w); out.push([document.getElementById('bench-said')?.textContent ?? '(no line)', !!document.querySelector('#bench-card .cloudnote')]); }
      document.getElementById('bench-card')?.remove(); return out; });
    log(JSON.stringify(said));
    const upd = await A.page.evaluate(() => { const V = window.__bench.deps.view; V.show('a report', () => {}, { result: 'sent', take: 'sent' }); const a = document.getElementById('bench-said').textContent; V.update({ result: 'arrived', take: 'arrived' }); const b = document.getElementById('bench-said').textContent; document.getElementById('bench-card')?.remove(); V.update({ result: false, take: false }); return [a, b]; });
    check(upd[0] === 'Your results are being sent to the developer.' && upd[1] === 'Your results were sent to the developer.', `A: the card's line follows the files to their end, and an update with no card up is harmless (${upd.join(' / ')})`);
    // Close waits for a file still on its way; a second press leaves at once
    const cl1 = await A.page.evaluate(async () => { const V = window.__bench.deps.view; let closed = 0, free; const idle = () => new Promise(r => { free = r; });
      V.show('a report', () => { closed++; }, { result: 'sent', take: 'sent', idle }); const B = document.querySelectorAll('#bench-card button')[1];
      B.click(); const mid = [B.textContent, !!document.getElementById('bench-card'), closed]; free(true); await new Promise(r => setTimeout(r, 50)); const end = [!!document.getElementById('bench-card'), closed];
      V.show('a report', () => { closed++; }, { result: 'sent', take: 'sent', idle }); const B2 = document.querySelectorAll('#bench-card button')[1]; B2.click(); B2.click(); const twice = [!!document.getElementById('bench-card'), closed]; free(true); await new Promise(r => setTimeout(r, 50));
      V.show('a report', () => { closed++; }, { result: 'arrived', take: 'arrived', idle }); document.querySelectorAll('#bench-card button')[1].click(); const plain = [!!document.getElementById('bench-card'), closed];
      V.show('a report', () => { closed++; }, { result: 'held', take: 'held', idle }); const B4 = document.querySelectorAll('#bench-card button')[1]; B4.click(); const held = [B4.textContent, !!document.getElementById('bench-card'), closed]; free(false); await new Promise(r => setTimeout(r, 50)); held.push(!!document.getElementById('bench-card'), closed);
      return { mid, end, twice, plain, held }; });
    log(JSON.stringify(cl1));
    check(cl1.mid[0] === 'Sending…' && cl1.mid[1] === true && cl1.mid[2] === 0 && cl1.end[0] === false && cl1.end[1] === 1, 'A: Close on a card whose files are on their way reads Sending… and keeps the page until they have gone, then closes');
    check(cl1.twice[0] === false && cl1.twice[1] === 2 && cl1.plain[0] === false && cl1.plain[1] === 3, 'A: a second press leaves at once (and closes once), and a card with nothing on its way closes on the first press');
    check(cl1.held[0] === 'Sending…' && cl1.held[1] === true && cl1.held[2] === 3 && cl1.held[3] === false && cl1.held[4] === 4, 'A: Close also asks before leaving a card whose files are held (they go up once the notice has been read), and leaves when told nothing is going');
    const real = await A.page.evaluate(async () => { const t0 = performance.now(), ok = await window.__bench.deps.idle(15000); return [ok, Math.round(performance.now() - t0)]; });
    check(real[0] === true && real[1] < 5000, `A: the page's own idle seam answers once nothing is on its way (${real.join(', ')} ms)`);
    check(said[0][0] === 'Your results are being sent to the developer.' && said[1][0] === 'Your results will be sent to the developer.' && /^Your results weren't sent, so press Copy/.test(said[2][0]) && /The full recording wasn't sent\.$/.test(said[3][0]), 'A: the card\'s line for sent, held, not sent, and a take refused alone');
    check(!A.errs.length, `A: no page or console error (${A.errs.length}${A.errs.length ? ': ' + A.errs[0] : ''})`);
    // ---- B: downloads put back, nobody listening
    await B.ready; log('B ready');
    const b = await take(B.page); await sleep(3000);
    check(B.downloads.length === 1 && /^perf-.*\.json$/.test(B.downloads[0]?.suggestedFilename() ?? '') && b.went === false && /^REC saved — \d+ frames$/.test(b.notes.join(' | ')) && (await B.page.evaluate(() => window.__saveUp.on)) === false, `B (?downloads=1): the take is downloaded once and the note is the old one (${B.downloads.map(d => d.suggestedFilename()).join(', ')}: ${b.notes.join(' | ')})`);
    check(!B.errs.length, `B: no page or console error (${B.errs.length}${B.errs.length ? ': ' + B.errs[0] : ''})`);
    // ---- C: nobody listening, a page on this machine
    await C.ready; log('C ready');
    const c3 = await take(C.page); await sleep(3000); const cNotes = await C.page.evaluate(() => window.__notes);
    check(await C.page.evaluate(() => typeof window.__saveUp === 'object' && window.__saveUp.on === true), "C: the door's table is on the page for a developer");
    check(C.downloads.length === 0 && c3.went === false && /^REC stopped with \d+ frames\. \| The recording wasn't saved, because no shot server is up\.$/.test(cNotes.join(' | ')), `C (?cloud=0, localhost): no download, and the notes say it stopped and then that it was saved nowhere (${cNotes.join(' | ')})`);
    const lone = await C.page.evaluate(() => { window.__bench.deps.view.show('a report', () => {}, { result: false, take: false }); const t = document.getElementById('bench-said')?.textContent, n = !!document.querySelector('#bench-card .cloudnote'); document.getElementById('bench-card')?.remove(); return [t, n]; });
    check(/^Your results weren't sent, so press Copy/.test(lone[0] ?? '') && lone[1] === false && (await C.page.evaluate(() => window.__cloud)) === null, `C: the card on a page with no backend says not sent and has no notice line (${JSON.stringify(lone)})`);
    check(!C.errs.length, `C: no page or console error (${C.errs.length}${C.errs.length ? ': ' + C.errs[0] : ''})`);
  } finally { await browser.close().catch(() => {}); standin.kill(); }
  console.log(red ? `${red} FAILED` : 'all pass'); process.exit(red ? 1 : 0);
})().catch(e => { console.log('- FAIL — the script died: ' + String(e).slice(0, 400)); process.exit(1); });
