// THE PAGE AS A CLIENT OF A GAME SERVER, CHECKED (src/net/page.ts; docs/agents/net.md). Headless Chromium on the software WebGPU adapter opens the game's page
// with ?net= on a real server process (tools/mp/server.ts) started here, is sent to the game's own address, stands its game from the server's recipe, says ready,
// sends its player's records and walks the server's confirmed ticks; a scripted watcher (tools/mp/bot.ts, bun) walks the same game beside it.
//   PORT=5231 node tools/qa/mpclient-page.cjs [--ticks 600] [--setup '{"scenario":"?rescue&levelFile=consulate.json","seed":3}']     (a dev server on that port)
//   MPCLIENT_BREAK=1 …    the red path: after tick 90 the page flips the last bit of its own man's place — its state words must part from the server's and the check FAIL
// PASS = the page was moved to the recipe's address, the seat's key no longer on it · it took the seat and the fight started only after it stood · every state word of N ticks was the
// server's (V8 with the page's engine against JavaScriptCore with none) · the server ran ticks on the page's own records and its man moved · a tab that
// sleeps wakes far behind, catches up and still matches, and its man stood still only when the hands were taken off first · no page error.
// The page is driven by script (frames stepped, nothing drawn but the last): what a PLAYER sees and feels on a GPU is not what this checks.
// exit 0 = pass · 1 = a line failed · 2 = something could not be started
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const { spawn } = require('node:child_process');
const { resolve } = require('node:path');
const ROOT = resolve(__dirname, '../..');
const PORT = process.env.PORT || 5199, BREAK = process.env.MPCLIENT_BREAK === '1', QUALITY = process.env.QUALITY || 'verylow';
const arg = (k, d) => { const i = process.argv.indexOf(`--${k}`); return i >= 0 ? process.argv[i + 1] : d; };
const N = Number(arg('ticks', '600')), setup = JSON.parse(arg('setup', '{"scenario":"?roomclear&squad4","seed":7}'));
const MP = 5800 + (process.pid % 150);
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/;
let fails = 0; const line = (ok, what) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); if (!ok) fails++; };
const sleep = ms => new Promise(r => setTimeout(r, ms));
const health = async () => { try { return await (await fetch(`http://127.0.0.1:${MP}/health`)).json(); } catch { return null; } };

(async () => {
  const srv = spawn('bun', ['run', 'tools/mp/server.ts'], { cwd: ROOT, stdio: ['ignore', 'pipe', 'pipe'], env: { ...process.env, MP_PORT: String(MP), MP_BUILD: '', MP_IDLE_S: '600', MP_SETUP: JSON.stringify({ game: 'check', ...setup, seatKeys: ['seat-one'] }) } });
  let srvErr = ''; srv.stderr.on('data', d => { srvErr += d; }); srv.stdout.on('data', () => {});
  let h0 = null; for (let i = 0; i < 200 && !h0; i++) { h0 = await health(); if (!h0) await sleep(100); }
  if (!h0) { srv.kill(); console.log(`mpclient-page: the server did not come up: ${srvErr.trim().split('\n').slice(-2).join(' / ')}`); process.exit(2); }
  const bot = spawn('bun', ['run', 'tools/mp/bot.ts', '--url', `ws://127.0.0.1:${MP}`, '--game', 'check', '--build', '', '--ticks', String(N), '--timeout', '900'], { cwd: ROOT, stdio: ['ignore', 'pipe', 'pipe'] });
  let botOut = ''; bot.stdout.on('data', d => { botOut += d; }); const botDone = new Promise(r => bot.on('exit', r));

  let browser = null, out; const errs = [];
  try {
    browser = await chromium.launch({ args: FLAGS }); const page = await browser.newPage({ viewport: { width: 480, height: 270 } });
    page.on('console', m => { if (m.type() === 'error' && !NOISE.test(m.text())) errs.push(m.text().slice(0, 200)); }); page.on('pageerror', e => errs.push('PAGEERROR ' + String(e).slice(0, 300)));
    const first = `http://localhost:${PORT}/?net=${encodeURIComponent(`ws://127.0.0.1:${MP}`)}&netgame=check&netkey=seat-one&paused&mute&quality=${QUALITY}`;
    await page.goto(first, { waitUntil: 'load' });
    await page.waitForFunction(() => new URLSearchParams(location.search).has('seed'), null, { timeout: 60000 });
    const moved = await page.evaluate(() => location.search);
    let hWait = null; for (let i = 0; i < 600; i++) { hWait = await health(); if (hWait && hWait.seats[0]) break; await sleep(50); }   // (asked the moment the moved page has said hello: it has the seat and its game is nowhere near stood — the server must be waiting, no tick run)
    await page.evaluate(() => window.__ready);
    out = await page.evaluate(async ({ N, BREAK }) => {
      window.__testVisible = true;
      const g = window.__game, start = [...g.player.char.pos], t0 = performance.now(); let broke = false, n = 0;
      const S = () => window.__net.state;
      while (S().confirmed < N && !S().bad && !S().broken && !S().bye && performance.now() - t0 < 600000) {
        const leg = Math.floor(n / 9) % 4, keys = [['KeyW', 'KeyD', 'KeyS', 'KeyA'][leg]]; n++;
        window.__mouse(240 + Math.cos(n * 0.3) * 120, 135 + Math.sin(n * 0.3) * 80); if (n % 6 === 0) window.__click(0);
        window.__runFrames(10, 1 / 60, keys, 'none');
        if (BREAK && !broke && S().confirmed >= 90) { broke = true; const p = g.player.char.pos, f = new Float64Array([p[0]]), u = new Uint32Array(f.buffer); u[0] ^= 1; p[0] = f[0]; }
        await new Promise(r => setTimeout(r, 0));
      }
      // A TAB THAT SLEEPS, twice, the walk key down as it goes: first with nothing said (the server holds the seat's keys: the man walks on), then with the
      // hands taken off as a hidden tab or a card does (one record: he stays). Each time the page wakes far behind and catches up to the server's word.
      const main = { confirmed: S().confirmed, wordsOk: S().wordsOk, secs: +((performance.now() - t0) / 1000).toFixed(1) };
      const yieldNow = () => new Promise(r => setTimeout(r, 0)), at = () => [g.player.char.pos[0], g.player.char.pos[2]];
      const level = async keys => { const t = performance.now(); let far = false; while ((S().behind > 2 || S().far) && !S().bad && performance.now() - t < 180000) { window.__runFrames(10, 1 / 60, keys, 'none'); far = far || S().far; await yieldNow(); } return far; };
      const walk = async (key, n) => { const a = at(); for (let i = 0; i < n; i++) { window.__runFrames(10, 1 / 60, [key], 'none'); await yieldNow(); } await level([key]); const b = at(); return Math.hypot(b[0] - a[0], b[1] - a[1]); };
      const nap = async rest => {
        let key = 'KeyW', room = -1; for (const k of ['KeyW', 'KeyD', 'KeyS', 'KeyA']) { const d = await walk(k, 3); if (d > room) { room = d; key = k; } }   // (the way he has room to walk: a wall in front of him would say nothing)
        await walk(key, 4);
        const a = at(), down = !!g.player.down; if (rest) window.__net.client.rest(performance.now());
        await new Promise(r => setTimeout(r, 3500));
        const behind = S().behind, far = await level([]), b = at();
        return { behind, far, key, down, walked: +Math.hypot(b[0] - a[0], b[1] - a[1]).toFixed(2) };
      };
      const naps = BREAK || S().bad || S().broken || S().bye ? null : { held: await nap(false), rest: await nap(true), words: S().wordsOk, heard: S().heard, bad: !!S().bad };
      const p = g.player.char.pos;
      return { naps, main, state: S(), moved: +Math.hypot(p[0] - start[0], p[2] - start[2]).toFixed(3), scenario: window.__scenarios.current()?.kind ?? null, fatal: (document.getElementById('fatal') || {}).textContent || '', line: (document.getElementById('netline') || {}).textContent || '' };
    }, { N, BREAK });
    const hEnd = await health(), S = out.state, M = out.main, words = Math.floor(N / 30);
    line(new URLSearchParams(moved).has('roomclear') === /roomclear/.test(setup.scenario) && new URLSearchParams(moved).get('seed') === String(setup.seed) && new URLSearchParams(moved).has('net') && !new URLSearchParams(moved).has('netkey'), `moved: the page was sent to the game's own address, the seat's key kept off it (${moved.slice(0, 120)})`);
    line(!!hWait && hWait.state === 'waiting' && hWait.tick === 0 && hWait.seats[0] === true, `ready: while the page stood its game the server held the seat and waited (${hWait?.state}, tick ${hWait?.tick})`);
    line(S.confirmed >= N && !S.bad && M.wordsOk >= words && !S.broken && !out.fatal, `same fight: the page walked ${M.confirmed} ticks in ${M.secs} s and matched ${M.wordsOk} of ${Math.floor(M.confirmed / 30)} state words${S.bad ? ` (FIRST DIFFERENT after tick ${S.bad.t}: server ${S.bad.server}, its own ${S.bad.mine})` : ''}${S.broken ? ` (${S.broken})` : ''}; a confirmed tick cost ${S.walkMs.toFixed(1)} ms here${out.line ? `; the page said "${out.line}"` : ''}`);
    line(S.seat === 0 && S.sent > 20 && !!hEnd && hEnd.lines[0].own > 20 && out.moved > 0.5, `its own man: seat ${S.seat}, ${S.sent} records sent, the server ran ${hEnd?.lines[0].own} ticks on them (${hEnd?.lines[0].late} late, ${hEnd?.lines[0].held} on a held one), the man moved ${out.moved} m; it aims ${S.margin} ticks past the server's`);
    if (out.naps) { const Z = out.naps;
      const up = !Z.held.down && !Z.rest.down;   // (a man shot down walks nowhere: then only the waking is asked)
      line(Z.held.far && Z.rest.far && !Z.bad && !S.far && (!up || (Z.held.walked > 1 && Z.rest.walked < Z.held.walked / 3)), `a tab that sleeps: ${up ? `asleep 3.5 s with a walk key down (${Z.held.key}) and nothing said, the server walked the man on ${Z.held.walked} m; with the hands taken off first (${Z.rest.key}), ${Z.rest.walked} m` : 'the man was down by then, so how far he walks on was NOT asked'}; each time the page woke ${Z.held.behind} and ${Z.rest.behind} ticks behind, caught up with the mixer shut and still matched every state word (${Z.words} by tick ${Z.heard})`); }
    line(errs.length === 0, `no page error${errs.length ? `: ${errs.slice(0, 3).join(' | ')}` : ''}`);
    await page.close();
    await Promise.race([botDone, sleep(20000)]);
    const bl = botOut.split('\n').find(x => x.startsWith('MPBOT ')), B = bl ? JSON.parse(bl.slice(6)) : null;
    line(!!B && B.confirmed >= N && !B.bad && B.wordsOk >= words, `beside it: a watcher on bun walked the same ${B?.confirmed ?? 0} ticks and matched ${B?.wordsOk ?? 0} words`);
  } catch (e) { line(false, `the page could not be driven: ${String(e && e.stack || e).slice(0, 400)}${errs.length ? ` · page errors: ${errs.slice(0, 3).join(' | ')}` : ''}`); }
  try { await browser?.close(); } catch { /* gone */ } try { bot.kill(); } catch { /* gone */ } srv.kill();
  console.log(fails ? `mpclient-page: ${fails} line${fails === 1 ? '' : 's'} FAILED${BREAK ? ' (MPCLIENT_BREAK=1: the same-fight line must, and its-own-man may: a page out of step stops sending)' : ''}` : 'mpclient-page: every line passed');
  process.exit(fails ? 1 : 0);
})().catch(e => { console.log(`mpclient-page: ${String(e && e.stack || e).slice(0, 400)}`); process.exit(2); });
