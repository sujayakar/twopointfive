// THE MAP VIEWER'S SEND, ON THE REAL PAGE (src/replay/viewer.ts sendShown; the page's download went on 2026-10-27): headless Chromium, the replay page alone —
// no game is booted, so this takes a minute, not twenty — a replay made headless (tools/qa/replay-make.ts) opened from disk, and a STAND-IN for the backend's
// wire in this file (a few lines of node: every function answers success, a file's start hands out an upload address, the upload keeps the bytes; NOT the
// backend's functions — cloud/test/standin.ts and tools/qa/cloud-page.cjs are those) —
//   A. the backend off (?cloud=0): no Send, no button or link that downloads, and a note typed says once that it stayed in this browser
//   B. a backend on the address: a replay opened from a file is one a notes round never carries, so Send is drawn; a note says to press it; pressed, it reads
//      Sent, and the stand-in holds ONE file of kind 'replay' that is the replay with the note in it (packed on the way: it is past the size that is), its row
//      naming the visit it is a cut of as `of` and no `session`; a note with no words leaves it Sent, a newer note puts Send back
//   C. the stand-in refusing the bytes: Not sent, and the button can be pressed again
//   D. the replays' own word off (?cloudtapes=0): no Send
//   PORT=5311 node tools/qa/replaysend-page.cjs [outDir]      (a dev server on that port: PORT=5311 bun run tools/serve.ts)
//   REPLAYSEND_BREAK=1 … = the stand-in refuses every upload from the start: B must go red (exit 1)
// NOT SEEN HERE: a real backend, a kept visit or a shelf replay that IS carried (no Send: cloud.ts cloudCarries is the backend's side's own, held by its check).
// exit 0 = all of it · 1 = otherwise / a page error
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const http = require('node:http'), fs = require('node:fs'), os = require('node:os'), path = require('node:path'), zlib = require('node:zlib'), cp = require('node:child_process');
const PORT = process.env.PORT || 5199, ROOT = path.resolve(__dirname, '../..');
const OUT = process.argv[2] || fs.mkdtempSync(path.join(os.tmpdir(), 'replaysend-page-'));
let failed = false; const check = (ok, text) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); if (!ok) failed = true; };
const sleep = ms => new Promise(r => setTimeout(r, ms));
// ---- the stand-in for the wire
const calls = [], uploads = [], puts = []; let refuse = process.env.REPLAYSEND_BREAK === '1';
const VISIT = 'visit-of-the-check';
const CORS = { 'Access-Control-Allow-Origin': '*', 'Access-Control-Allow-Headers': '*', 'Access-Control-Allow-Methods': 'POST, GET, OPTIONS' };
const server = http.createServer((req, res) => {
  if (req.method === 'OPTIONS') { res.writeHead(204, CORS); res.end(); return; }
  const chunks = []; req.on('data', c => chunks.push(c)); req.on('end', () => {
    const body = Buffer.concat(chunks);
    if (req.url === '/upload') { if (refuse) { res.writeHead(500, CORS); res.end('refused'); return; } uploads.push(body); res.writeHead(200, { ...CORS, 'Content-Type': 'application/json' }); res.end(JSON.stringify({ storageId: `s${uploads.length}` })); return; }
    let j = null; try { j = JSON.parse(body.toString('utf8')); } catch { /* */ }
    const fn = j && j.path || req.url; calls.push(fn);
    if (fn === 'files:put') puts.push(j.args[0]);
    res.writeHead(200, { ...CORS, 'Content-Type': 'application/json' }); res.end(JSON.stringify({ status: 'success', value: fn === 'files:start' ? { url: `http://127.0.0.1:${server.address().port}/upload` } : null }));
  });
});
(async () => {
  fs.mkdirSync(OUT, { recursive: true });
  const made = path.join(OUT, 'made.json'), mk = cp.spawnSync('bun', ['run', 'tools/qa/replay-make.ts', made, '1800', VISIT], { cwd: ROOT, encoding: 'utf8' });
  if (mk.status !== 0) { console.log(`- FAIL — no replay to open: ${String(mk.stderr).slice(0, 300)}`); process.exit(1); }
  const madeBytes = fs.statSync(made).size;
  await new Promise(r => server.listen(0, '127.0.0.1', r)); const CLOUD = `http://127.0.0.1:${server.address().port}`;
  const browser = await chromium.launch({ headless: true, channel: 'chromium' });
  const errors = []; let downloads = 0;
  const profile = async (query) => {
    const ctx = await browser.newContext({ viewport: { width: 1100, height: 620 }, deviceScaleFactor: 1, acceptDownloads: true }), page = await ctx.newPage();
    page.on('console', m => { if (m.type() === 'error' && !/Failed to load resource/.test(m.text())) errors.push(m.text().slice(0, 300)); });
    page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
    page.on('download', () => { downloads++; });
    await page.goto(`http://localhost:${PORT}/replay.html${query}`, { timeout: 120000 });
    await page.waitForFunction(() => !!window.__viewer, null, { timeout: 60000 });
    await page.setInputFiles('#file', made); await page.waitForFunction(() => !!window.__viewer.file && /#id=\d+/.test(location.hash), null, { timeout: 60000 });
    // (whether Send is drawn is asked of the backend's side once a file is open: `data-asked` says an ask has landed, so nothing here sleeps and hopes)
    await page.waitForFunction(() => document.getElementById('send')?.dataset.asked === '1', null, { timeout: 60000 });
    const send = () => page.evaluate(() => { const b = document.getElementById('send'); return b ? { hidden: b.hidden, text: b.textContent, disabled: b.disabled, title: b.title } : null; });
    const hint = () => page.evaluate(() => document.getElementById('hint')?.textContent || '');
    const note = async (words) => { if (!(await page.$('textarea'))) { await page.evaluate(() => window.__viewer.noteAt(null)); await page.waitForSelector('textarea', { timeout: 20000 }); } await page.fill('textarea', words); };
    const until = async (f, ms = 10000) => { for (let t = 0; t < ms; t += 100) { if (await f()) return true; await sleep(100); } return false; };
    return { ctx, page, send, hint, note, until };
  };
  try {
    // ---- A. the backend off (whatever the server this runs against would find by itself)
    { const P = await profile('?cloud=0'); const s = await P.send();
      const dl = await P.page.evaluate(() => [...document.querySelectorAll('button, a')].filter(e => /download/i.test(e.textContent || '') || e.hasAttribute('download')).map(e => e.textContent));
      check(!!s && s.hidden && dl.length === 0, `with the backend off the viewer draws no Send, and no button or link downloads (${s ? `send hidden ${s.hidden}` : 'no #send'}; ${dl.join(' | ') || 'none'})`);
      await P.note('a note typed with the backend off'); await P.until(async () => /weren't sent/.test(await P.hint()));
      check((await P.hint()) === "Your notes are saved in this browser, but they weren't sent.", `a note that stayed here says so: "${await P.hint()}"`);
      await P.ctx.close(); }
    // ---- B. a backend on the address: Send
    const P = await profile(`?cloud=${encodeURIComponent(CLOUD)}`);
    { const s = await P.send(); check(!!s && !s.hidden && s.text === 'Send ↑' && !s.disabled && s.title === 'Send this replay and your notes to the developer.', `a replay opened from a file, with a backend: Send is drawn (${s ? `"${s.text}", hidden ${s.hidden}` : 'no #send'})`);
      await P.note('a note typed before the send'); await P.until(async () => /Press Send/.test(await P.hint()));
      check((await P.hint()) === 'Your notes are saved in this browser. Press Send to send them.', `the note says how it gets out: "${await P.hint()}"`);
      await P.page.click('#send'); const mid = await P.send(); await P.until(async () => /Sent|Not sent/.test((await P.send()).text), 60000); const s2 = await P.send();
      check(s2.text === 'Sent ✓' && s2.disabled && (mid.text === 'Sending…' || mid.text === 'Sent ✓'), `pressed, it reads "${mid.text}" then "${s2.text}", and cannot be pressed twice over the same file`);
      const reps = puts.filter(p => p.kind === 'replay'), put = reps[0], up = put ? uploads[Number(String(put.storageId).slice(1)) - 1] : null;
      let F = null, meta = {}; if (put && up) { meta = put.meta ? JSON.parse(put.meta) : {}; try { F = JSON.parse((meta.packed === 'gzip' ? zlib.gunzipSync(up) : up).toString('utf8')); } catch { /* */ } }
      check(reps.length === 1 && !!F && F.game === 'twopointsix' && F.kind === 'replay' && F.notes.length === 1 && F.notes[0].text === 'a note typed before the send' && !('session' in meta) && meta.of === VISIT && F.session === VISIT && meta.savedAt === F.savedAt && madeBytes > 256 * 1024 && meta.packed === 'gzip',
        `the stand-in holds ONE file of kind 'replay': the replay (${(madeBytes / 1024).toFixed(0)} KB, packed on the way: ${meta.packed || 'no'}), the note in it, its row naming the visit as 'of' (${meta.of}) and no session (${reps.map(p => p.name).join(' · ') || 'none'}; ${F ? `${F.samples.length} samples, ${F.notes.length} note` : 'unread'}; row: ${Object.keys(meta).join(' ') || 'nothing'})`);
      if (F) { const at = path.join(OUT, 'sent'); fs.mkdirSync(at, { recursive: true }); const file = path.join(at, 'sent.json'); fs.writeFileSync(file, JSON.stringify(F));
        const r = cp.spawnSync('bun', ['run', 'tools/qa/replay-dump.ts', file, '--out', path.join(OUT, 'read')], { cwd: ROOT, encoding: 'utf8' }); let ix = null; try { ix = JSON.parse(fs.readFileSync(path.join(OUT, 'read', 'index.json'), 'utf8')); } catch { /* */ }
        const E = ix ? Object.values(ix.entries || {})[0] : null;
        check(r.status === 0 && !!E && E.notes === 1, `the reader takes what was sent, the note with it (exit ${r.status}, ${E ? `${E.notes} note` : 'no entry'})`); }
      // (a note made and left with no words is not in what a send carries: the button stays Sent)
      await P.page.evaluate(() => window.__viewer.noteAt(null)); await sleep(300); const s0 = await P.send();
      await P.page.evaluate(() => document.activeElement && document.activeElement.blur()); await sleep(300);   // (left with no words, it is gone: the page draws its notes again)
      await P.note('a note typed after the send'); await P.until(async () => (await P.send()).text === 'Send ↑');
      const s3 = await P.send(); check(s0.text === 'Sent ✓' && s0.disabled && s3.text === 'Send ↑' && !s3.disabled, `a note with no words leaves it "${s0.text}", and a newer note puts Send back ("${s3.text}", disabled ${s3.disabled})`); }
    // ---- C. the bytes refused
    { refuse = true; await P.page.click('#send'); await P.until(async () => /Not sent|Sent ✓/.test((await P.send()).text), 120000); const s = await P.send();
      await P.until(async () => (await P.send()).text === 'Send ↑', 6000); const s2 = await P.send(); refuse = false;
      check(s.text === 'Not sent' && s2.text === 'Send ↑' && !s2.disabled, `the backend refusing the bytes: "${s.text}", then "${s2.text}" again (disabled ${s2.disabled})`);
      await P.page.screenshot({ path: path.join(OUT, 'viewer-send.png') }).catch(() => {});
      await P.ctx.close(); }
    // ---- D. the replays' own word off
    { const Q = await profile(`?cloud=${encodeURIComponent(CLOUD)}&cloudtapes=0`); const s = await Q.send();
      check(!!s && s.hidden, `with ?cloudtapes=0 no Send is drawn (hidden ${s && s.hidden})`); await Q.ctx.close(); }
    check(downloads === 0, `nothing on the page asked for a download (${downloads})`);
    check(errors.length === 0, `no page error or console error (${errors.length}${errors.length ? ': ' + errors.slice(0, 3).join(' | ') : ''})`);
  } catch (e) { check(false, `the walk died: ${String(e && e.stack || e).slice(0, 600)}`); }
  await browser.close().catch(() => {}); server.close();
  console.log(`\n=== replaysend-page: ${failed ? 'FAIL' : 'ALL PASS'} ===\n(files: ${OUT})`); process.exit(failed ? 1 : 0);
})();
