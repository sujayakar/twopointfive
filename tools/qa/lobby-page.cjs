// HOST OR JOIN A CO-OP GAME, ON THE PAGE (src/ui/coop.ts, src/net/lobby.ts; docs/agents/net.md "the lobby"). Two windows of headless Chromium on the software
// WebGPU adapter, a dev server started here whose backend is a STAND-IN on this machine (cloud/test/standin.ts: `npm ci` in cloud/ first) — the way
// `bun run dev` hands its pages the backend it started — and the dev server's own runner starting a REAL game server (tools/mp/server.ts):
//   1. the title has a Co-op row; Host a game opens the starting page with only the missions two can play, and its block says so
//   2. the block makes a game: the window is sent to the game's own address, the seat's key kept off it; the backend holds one live game of two seats
//      whose setup is the mission with the four and two seats on it
//   3. the host's window stands in the first seat while the server waits, and shows the code and the link
//      …and THE DOOR: the same window sent to the link (/?join=<CODE>) comes back into its own seat (a tab is one player), the second seat still free
//   4. a second window TYPES the code on the title and is sent to the same game, in the second seat
//   5. SAME FIGHT: both windows walk the server's ticks for some seconds and match every state word; each shows its own seat's man, each walks him, and
//      the other window's game has him moved too; the invite is gone. THE WALK BEGINS AS SOON AS BOTH STAND: the server plays from that moment at its
//      own pace, and a fight nobody plays for ten minutes can leave a man down
//   6. a third window that opens the LINK is told the game is full
//   PORT=5351 [CLOUD_PORT=3251] node tools/qa/lobby-page.cjs [--ticks 300]        (five page loads, two games side by side: a quarter of an hour on this adapter)
//   LOBBY_BREAK=1 …   the red path: the second window is opened with coop=0 on its address (the page as it was before co-op: no row to type a code into, a
//                    link not read) — the join line and the same-fight line must FAIL
// The windows are driven by script (frames stepped, none drawn): what two PLAYERS see and feel on a GPU is not what this checks. NOT HERE: Convex's own
// local backend, the hosted backend and its sandboxes, two machines. exit 0 = pass · 1 = a line failed · 2 = something could not be started
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const path = require('node:path'), cp = require('node:child_process'), fs = require('node:fs');
const ROOT = path.resolve(__dirname, '../..');
const PORT = process.env.PORT || 5351, CPORT = Number(process.env.CLOUD_PORT) || 3251, BREAK = process.env.LOBBY_BREAK === '1', QUALITY = process.env.QUALITY || 'verylow';
const arg = (k, d) => { const i = process.argv.indexOf(`--${k}`); return i >= 0 ? process.argv[i + 1] : d; };
const N = Number(arg('ticks', '300')), NAV = Math.max(60000, Number(process.env.NAV_MS) || 480000), BASE = `http://localhost:${PORT}/`, CLOUD = `http://127.0.0.1:${CPORT}`, KEY = 'page-check-key', WORDS = 'paused&mute&quality=' + QUALITY;
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage|Failed to load resource/;
const T0 = Date.now(), log = (...a) => { if (process.env.LOBBY_SAY) console.log(`  [${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a); };
let fails = 0; const line = (ok, what) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); if (!ok) fails++; };
const sleep = ms => new Promise(r => setTimeout(r, ms));
const until = async (f, ms = 20000) => { const t = Date.now(); for (;;) { const v = await f().catch(() => null); if (v || Date.now() - t > ms) return v; await sleep(250); } };
const admin = async (fn, args) => { const r = await fetch(`${CLOUD}/api/query`, { method: 'POST', headers: { 'Content-Type': 'application/json', Authorization: `Convex ${KEY}` }, body: JSON.stringify({ path: fn, format: 'convex_encoded_json', args: [args] }) }); const j = await r.json(); if (j.status !== 'success') throw new Error(`${fn}: ${JSON.stringify(j).slice(0, 200)}`); return j.value; };

(async () => {
  if (!fs.existsSync(path.join(ROOT, 'cloud/node_modules/convex-test'))) { console.log('lobby-page: the backend\'s packages are not installed — run `npm ci` in cloud/ first'); process.exit(2); }
  const kids = [], stopKids = () => { for (const k of kids) try { k.kill(); } catch { /* gone */ } }; process.on('exit', stopKids);
  const free = async url => { try { await fetch(url, { signal: AbortSignal.timeout(800) }); return false; } catch { return true; } };
  if (!(await free(`${CLOUD}/__stats`)) || !(await free(`${BASE}qa/commit`))) { console.log(`lobby-page: port ${CPORT} or ${PORT} is already answering: stop what is there and start again`); process.exit(2); }
  // (THE CLOCKS, by the words a deployment's environment sets: on this adapter a window is minutes at a page load, and a server nobody has connected to for a minute ends itself)
  kids.push(cp.spawn('bun', ['run', 'test/standin.ts', String(CPORT)], { cwd: path.join(ROOT, 'cloud'), env: { ...process.env, STANDIN_KEY: KEY, MP_IDLE_S: '900', MP_START_S: '300', MP_SILENT_S: '300' }, stdio: 'ignore' }));
  { const env = { ...process.env, PORT: String(PORT), TPS_DEV_CLOUD: CLOUD, MP_CLOUD_KEY: KEY }; delete env.TPS_LOCAL_CLOUD; delete env.MP_SERVER;
    kids.push(cp.spawn('bun', ['run', 'tools/serve.ts'], { cwd: ROOT, env, stdio: 'ignore' })); }
  const done = async (code, browser) => { try { await browser?.close(); } catch { /* gone */ } stopKids(); await sleep(500); process.exit(code); };
  await until(async () => (await fetch(`${CLOUD}/__stats`)).ok); const build = await until(async () => { const r = await fetch(`${BASE}qa/commit`); return r.ok ? (await r.text()).trim() : ''; }, 60000);
  if (!build || !(await until(async () => (await admin('runners:list', {})).some(r => r.builds.includes(build)), 30000))) { console.log('lobby-page: the dev server did not come up as its backend\'s runner'); return done(2); }
  const browser = await chromium.launch({ args: FLAGS }).catch(() => null); if (!browser) { console.log('lobby-page: the browser did not start'); return done(2); }
  const ctx = await browser.newContext({ viewport: { width: 640, height: 360 } }), errs = [];   // (ONE browser, two of its windows: they share what a browser keeps, and each is its own tab)
  const open = async name => { const p = await ctx.newPage(); p.on('console', m => { if (m.type() === 'error' && !NOISE.test(m.text())) errs.push(`${name}: ${m.text().slice(0, 200)}`); }); p.on('pageerror', e => errs.push(`${name}: PAGEERROR ${String(e).slice(0, 300)}`)); return p; };
  const inGame = (p, ms) => p.waitForFunction(() => { const q = new URLSearchParams(location.search); return q.has('seed') && q.has('net'); }, null, { timeout: ms }).then(() => true, () => false);
  // (the address is the game's before the game's page is: wait for the page itself, then for its boot, then for its client)
  const stood = async p => { await p.waitForFunction(() => !!window.__ready, null, { timeout: NAV }); await p.evaluate(() => window.__ready); await p.waitForFunction(() => !!window.__net && !!window.__game, null, { timeout: 60000 }); };
  const title = async p => { await p.goto(`${BASE}?${WORDS}`, { waitUntil: 'domcontentloaded', timeout: 180000 }); return p.waitForSelector('#menu [data-coop="open"]', { timeout: 120000 }).then(() => true, () => false); };
  try {
    // 1 · the title's row and the host's starting page
    const A = await open('host');
    const row = await title(A); let list = { rows: 0, can: 0, lessons: 0, block: '', kit: '', head: '' };
    if (row) { await A.click('#menu [data-coop="open"]'); await A.click('#menu [data-coop="host"]'); await A.waitForSelector('#missions.one.on', { timeout: 20000 }); await sleep(800);
      list = await A.evaluate(() => { const vis = e => !!e && getComputedStyle(e).display !== 'none', M = document.getElementById('missions');
        const can = window.__coop ? window.__coop.hostable() : [];
        return { rows: M.querySelectorAll('.mrow').length, can: can.length, lessons: can.filter(r => r.group === 'tutorial' || !/^(roomclear|nightbreach)$/.test(r.kind)).length,
          block: [...M.querySelectorAll('[data-act="deploy"]')].filter(vis).map(b => b.textContent).join('|'), kit: [...M.querySelectorAll('[data-act="kit"], .mstep[data-step="loadout"]')].filter(vis).length, head: (M.querySelector('.msteps') || { previousElementSibling: null }).previousElementSibling?.textContent || '' }; }); }
    line(row && list.rows > 0 && list.rows === list.can && list.lessons === 0 && list.block === 'Host game' && list.kit === 0 && /co-op/i.test(list.head), `the title has a Co-op row; Host a game opens the starting page ("${list.head}") on ${list.rows} missions, every one a mission two can play, with no kit step and "${list.block}" on its block`);

    // 2 · the block makes the game, and the window goes through the door to it
    if (row) await A.click('#missions [data-act="deploy"]');
    const hosted = row && await inGame(A, NAV), qa = new URL(A.url()).searchParams, code = qa.get('join') || '';
    const games = await admin('games:list', {}).catch(() => []), G = games.find(g => g.code === code) || null, S = G ? new URLSearchParams(G.setup.scenario) : new URLSearchParams();
    line(hosted && /^[BCDFGHJKLMNPQRSTVWXZ]{5}$/.test(code) && !qa.has('netkey') && qa.get('seats') === '2' && !!G && G.holders.length === 2 && G.holders[0] !== '' && G.holders[1] === '' && S.get('seats') === '2' && S.get('squad') === '4' && S.has('roomclear'),
      `host: the block made game ${code || '(none)'} and the window was sent to its address, the seat's key kept off it; the backend holds it with ${G ? G.holders.length : 0} seats, the host in the first, for ${G ? G.setup.scenario : '?'}`);

    // 3 · the host stands, waits, and shows the code and the link
    let inv = { state: '', seat: -1, shown: '', link: '', vis: false };
    const invite = p => p.evaluate(() => { const c = document.getElementById('coopinvite'), S = window.__net.state; return { state: S.state, seat: S.seat, vis: !!c && c.style.display !== 'none', shown: c?.querySelector('[data-coop="shown"]')?.textContent || '', link: c?.querySelector('[data-coop="link"]')?.textContent || '' }; });
    if (hosted) { await stood(A); await until(async () => (await invite(A)).vis, 20000); inv = await invite(A); }
    line(inv.state === 'waiting' && inv.seat === 0 && inv.vis && inv.shown === code && inv.link === `${BASE}?join=${code}`, `invite: the host's window stands in seat ${inv.seat} while the server waits (${inv.state}) and shows the code ${inv.shown} and the link ${inv.link}`);

    if (hosted && !BREAK) { // (the door: one more load of the host's window, while the server still waits at tick 0)
      const key0 = await A.evaluate(() => window.__net.state.seat);
      await A.goto(`${BASE}?join=${code}&${WORDS}`, { waitUntil: 'domcontentloaded', timeout: 180000 });
      const back = await inGame(A, NAV), qd = new URL(A.url()).searchParams; let d = { state: '', seat: -1, vis: false, shown: '' };
      if (back) { await stood(A); await until(async () => (await invite(A)).vis, 20000); d = await invite(A); }
      const g2 = (await admin('games:list', {}).catch(() => [])).find(g => g.code === code) || null;
      line(back && key0 === 0 && qd.get('net') === qa.get('net') && qd.get('netgame') === qa.get('netgame') && qd.get('join') === code && !qd.has('netkey') && d.state === 'waiting' && d.seat === 0 && d.vis && d.shown === code && !!g2 && g2.holders[0] === G.holders[0] && g2.holders[1] === '',
        `door: the host's window, sent to the link, ${back ? `came back to the same game in seat ${d.seat} (${d.state}), the invite up again, and the second seat is still ${g2 && g2.holders[1] === '' ? 'free' : 'TAKEN'}` : 'did NOT come back to the game'}`); }

    // 4 · the second window types the code on the title
    const B = await open('join'); let typed = false;
    if (BREAK) { await B.goto(`${BASE}?join=${code}&coop=0&${WORDS}`, { waitUntil: 'domcontentloaded', timeout: 180000 }); }
    else if (await title(B)) { await B.click('#menu [data-coop="open"]'); await B.focus('#menu [data-coop="code"]'); await B.keyboard.type(code.toLowerCase()); typed = (await B.inputValue('#menu [data-coop="code"]')) === code; await B.keyboard.press('Enter'); }
    const joined = hosted && await inGame(B, BREAK ? 60000 : NAV), qb = new URL(B.url()).searchParams;
    if (joined) await stood(B);
    const seatB = joined ? await B.evaluate(() => ({ seat: window.__net.state.seat, mine: window.__game.seats.indexOf(window.__game.localSeat) })) : { seat: -1, mine: -1 };
    line(joined && typed && qb.get('net') === qa.get('net') && qb.get('netgame') === qa.get('netgame') && !qb.has('netkey') && seatB.seat === 1 && seatB.mine === 1, `join: a second window typed the code on the title's Co-op row${typed ? '' : ' (THE FIELD DID NOT TAKE IT)'} and ${joined ? `was sent to the same game, in seat ${seatB.seat} (it shows seat ${seatB.mine}'s man)` : 'was NOT sent to the game'}`);

    // 5 · the same fight
    if (joined) {
      const drive = (p, keys) => p.evaluate(keys => { window.__testVisible = true; window.__runFrames(10, 1 / 60, keys, 'none'); const g = window.__game, S = window.__net.state; return { state: S.state, confirmed: S.confirmed, behind: S.behind, far: S.far, ok: S.wordsOk, bad: !!S.bad, broken: S.broken, bye: S.bye, men: g.seats.map(s => [...s.view.player.char.pos]), alive: g.seats.map(s => s.view.player.char.alive !== false), mission: window.__roomclear ? window.__roomclear.state : '' }; }, keys);
      const gone = (a, b) => Math.hypot(a[0] - b[0], a[2] - b[2]), dead = s => s.bad || s.broken || s.bye;
      let a = await drive(A, []), b = await drive(B, []); const t1 = Date.now();
      while ((a.state !== 'playing' || b.state !== 'playing' || a.behind > 3 || b.behind > 3) && !dead(a) && !dead(b) && Date.now() - t1 < 240000) { a = await drive(A, []); b = await drive(B, []); }
      log('level', JSON.stringify({ a: [a.state, a.confirmed, a.behind], b: [b.state, b.confirmed, b.behind] }));
      const start = a.men, at = { tick: a.confirmed, mission: a.mission, alive: a.alive }, base = Math.max(a.confirmed, b.confirmed), t2 = Date.now(); let n = 0;
      // (how far each man GOT from where he stood, not where the walk left him: the keys go round a square, and a square can end where it began)
      const moved = [0, 0, 0, 0], reach = () => { [gone(a.men[0], start[0]), gone(a.men[1], start[1]), gone(b.men[0], start[0]), gone(b.men[1], start[1])].forEach((d, i) => { if (d > moved[i]) moved[i] = d; }); };
      const free = at.mission === 'live' && at.alive.every(Boolean);   // (a man who is down, or a mission that is over, walks nowhere: the line then says so, not '0.0 m')
      while ((a.confirmed < base + N || b.confirmed < base + N) && !dead(a) && !dead(b) && Date.now() - t2 < 600000) { const k = [['KeyW', 'KeyD', 'KeyS', 'KeyA'][Math.floor(n / 6) % 4]]; n++; a = await drive(A, k); b = await drive(B, [k[0] === 'KeyW' ? 'KeyS' : k[0] === 'KeyS' ? 'KeyW' : k[0] === 'KeyA' ? 'KeyD' : 'KeyA']); reach(); if (n % 10 === 0) log('fight', a.confirmed, b.confirmed, a.ok, b.ok); }
      const t3 = Date.now(); while ((a.behind > 3 || b.behind > 3 || Math.abs(a.confirmed - b.confirmed) > 30) && !dead(a) && !dead(b) && Date.now() - t3 < 120000) { a = await drive(A, []); b = await drive(B, []); }
      const h = await fetch(String(qa.get('net')).replace(/^ws/, 'http') + '/health').then(r => r.json(), () => null), mineA = await A.evaluate(() => window.__game.seats.indexOf(window.__game.localSeat)), up = await A.evaluate(() => !!document.getElementById('coopinvite'));
      reach();
      line(a.confirmed >= base + N && b.confirmed >= base + N && !dead(a) && !dead(b) && a.ok >= Math.floor(a.confirmed / 30) && b.ok >= Math.floor(b.confirmed / 30) && a.ok > 0 && !!h && h.seats.length === 2 && h.seats.every(Boolean) && h.tick >= base + N,
        `same fight: the host's window walked ${a.confirmed} ticks and matched ${a.ok} of ${Math.floor(a.confirmed / 30)} state words, the second ${b.confirmed} and ${b.ok} of ${Math.floor(b.confirmed / 30)}${a.bad || b.bad ? ' (A WORD DIFFERED)' : ''}${a.broken || b.broken ? ` (${a.broken || b.broken})` : ''}; the server is at tick ${h ? h.tick : '?'} with ${h ? h.seats.filter(Boolean).length : 0} of ${h ? h.seats.length : '?'} seats connected`);
      line(mineA === 0 && seatB.mine === 1 && moved.every(d => d > 0.3) && gone(start[0], start[1]) > 0.1 && !up,
        `two players: the host's window shows seat ${mineA}'s man and the second seat ${seatB.mine}'s; each walked its own (as far as ${moved[0].toFixed(1)} m and ${moved[3].toFixed(1)} m from where he stood) and the other window's game has him moved too (${moved[2].toFixed(1)} m and ${moved[1].toFixed(1)} m); the invite is ${up ? 'STILL UP' : 'gone'}${free ? '' : ` — AT THE WALK'S START (tick ${at.tick}) the mission was '${at.mission}' and the men alive were ${at.alive.join(', ')}: NOBODY WAS FREE TO WALK, which is this run's clock (the walk began too late), not the lobby`}`);
    } else line(false, 'same fight: the second window is not in the game, so nothing was walked');
    // 6 · a third window opens the link (after the walk: a page load is minutes here, and the fight does not wait for it)
    let full = '';
    if (joined) { const C = await open('third'); await C.goto(`${inv.link || BASE}&${WORDS}`, { waitUntil: 'domcontentloaded', timeout: 180000 });
      full = await until(async () => C.evaluate(() => { const l = document.getElementById('coopline'); return l && !/Joining|starting/i.test(l.textContent) ? l.textContent : ''; }), 30000) || ''; const left = new URL(C.url()).searchParams.has('join'); await C.close();
      line(/already has two players/.test(full) && !left, `full: a third window that opened the link was told "${full}", and the code is off its address`); }
    line(errs.length === 0, `no page error${errs.length ? `: ${errs.slice(0, 3).join(' | ')}` : ''}`);
  } catch (e) { line(false, `the windows could not be driven: ${String(e && e.stack || e).slice(0, 400)}${errs.length ? ` · page errors: ${errs.slice(0, 3).join(' | ')}` : ''}`); }
  console.log(fails ? `lobby-page: ${fails} line${fails === 1 ? '' : 's'} FAILED${BREAK ? ' (LOBBY_BREAK=1: the join line and the same-fight line must)' : ''}` : 'lobby-page: every line passed');
  return done(fails ? 1 : 0, browser);
})().catch(e => { console.log(`lobby-page: ${String(e && e.stack || e).slice(0, 400)}`); process.exit(2); });
