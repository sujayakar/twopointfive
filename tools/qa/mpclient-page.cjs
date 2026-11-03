// THE PAGE AS A CLIENT OF A GAME SERVER, CHECKED (src/net/page.ts; docs/agents/net.md). Headless Chromium on the software WebGPU adapter opens the game's page
// with ?net= on a real server process (tools/mp/server.ts) started here, is sent to the game's own address, stands its game from the server's recipe, says ready,
// sends its player's records and walks the server's confirmed ticks; a scripted watcher (tools/mp/bot.ts, bun) walks the same game beside it.
//   PORT=5231 node tools/qa/mpclient-page.cjs [--ticks 600] [--setup '{"scenario":"?rescue&levelFile=consulate.json","seed":3}']     (a dev server on that port)
//   --seats 2             a game of TWO players: the page sits in the SECOND seat (its own man, its own pair, its own records) and the bun client plays the first
//   MPCLIENT_BREAK=1 …    the red path: after tick 90 the page flips the last bit of its own man's place — its state words must part from the server's and the check FAIL
//   MPCLIENT_BREAK=way    the Network page's red path: the check picks the connection as it is where it should pick a long one — the `network page` line must FAIL
//   MPCLIENT_BREAK=ahead  the look ahead's red path: the page is opened with netpredict=0 (its own man drawn where the game has him) — the `ahead` line must FAIL
// PASS = the page was moved to the recipe's address, the seat's key no longer on it · it took the seat and the fight started only after it stood · every state word of N ticks was the
// server's (V8 with the page's engine against JavaScriptCore with none) · the server ran ticks on the page's own records and its man moved · a tab that
// sleeps wakes far behind, catches up and still matches, and its man stood still only when the hands were taken off first · no page error.
// AHEAD (src/net/foresee.ts, host/ahead.ts): on the frame a walk key first goes down the page's look ahead has its own man moved while the game has not moved
// him yet (and how many frames later the game does is printed); through the run he is ahead of the game while he walks; on frames that are DRAWN his palette
// instance, his shadow proxies and the boxes he owns are moved for the render and the state words after those frames are still the server's.
// The page is driven by script (frames stepped, none drawn but the look ahead's two at the end): what a PLAYER sees and feels on a GPU is not what this checks.
// exit 0 = pass · 1 = a line failed · 2 = something could not be started
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const { spawn } = require('node:child_process');
const { resolve } = require('node:path');
const ROOT = resolve(__dirname, '../..');
const PORT = process.env.PORT || 5199, BREAK = process.env.MPCLIENT_BREAK === '1', NO_AHEAD = process.env.MPCLIENT_BREAK === 'ahead', BAD_WAY = process.env.MPCLIENT_BREAK === 'way', QUALITY = process.env.QUALITY || 'verylow';
const arg = (k, d) => { const i = process.argv.indexOf(`--${k}`); return i >= 0 ? process.argv[i + 1] : d; };
const N = Number(arg('ticks', '600')), setup = JSON.parse(arg('setup', '{"scenario":"?roomclear&squad4","seed":7}')), TWO = arg('seats', '1') === '2';
if (TWO) setup.scenario += '&seats=2';
const KEYS = TWO ? ['seat-one', 'seat-two'] : ['seat-one'], MINE = KEYS.length - 1;   // (the page takes the last seat)
const MP = 5800 + (process.pid % 150);
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/;
let fails = 0; const line = (ok, what) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); if (!ok) fails++; };
const sleep = ms => new Promise(r => setTimeout(r, ms));
const health = async () => { try { return await (await fetch(`http://127.0.0.1:${MP}/health`)).json(); } catch { return null; } };

(async () => {
  const srv = spawn('bun', ['run', 'tools/mp/server.ts'], { cwd: ROOT, stdio: ['ignore', 'pipe', 'pipe'], env: { ...process.env, MP_PORT: String(MP), MP_BUILD: '', MP_IDLE_S: '600', MP_GRACE_S: '900', MP_SETUP: JSON.stringify({ game: 'check', ...setup, seatKeys: KEYS }) } });
  let srvErr = ''; srv.stderr.on('data', d => { srvErr += d; }); srv.stdout.on('data', () => {});
  let h0 = null; for (let i = 0; i < 200 && !h0; i++) { h0 = await health(); if (!h0) await sleep(100); }
  if (!h0) { srv.kill(); console.log(`mpclient-page: the server did not come up: ${srvErr.trim().split('\n').slice(-2).join(' / ')}`); process.exit(2); }
  const bot = spawn('bun', ['run', 'tools/mp/bot.ts', '--url', `ws://127.0.0.1:${MP}`, '--game', 'check', '--build', '', '--ticks', String(N), '--timeout', '900', ...(TWO ? ['--key', 'seat-one'] : [])], { cwd: ROOT, stdio: ['ignore', 'pipe', 'pipe'] });
  let botOut = ''; bot.stdout.on('data', d => { botOut += d; }); const botDone = new Promise(r => bot.on('exit', r));

  let browser = null, out; const errs = [];
  try {
    browser = await chromium.launch({ args: FLAGS }); const page = await browser.newPage({ viewport: { width: 480, height: 270 } });
    page.on('console', m => { if (process.env.MPCLIENT_SAY && m.text().startsWith('[mpc]')) console.log('  ' + m.text()); if (m.type() === 'error' && !NOISE.test(m.text())) errs.push(m.text().slice(0, 200)); }); page.on('pageerror', e => errs.push('PAGEERROR ' + String(e).slice(0, 300)));
    const first = `http://localhost:${PORT}/?net=${encodeURIComponent(`ws://127.0.0.1:${MP}`)}&netgame=check&netkey=${KEYS[MINE]}&paused&mute&quality=${QUALITY}${NO_AHEAD ? '&netpredict=0' : ''}`;
    await page.goto(first, { waitUntil: 'load' });
    await page.waitForFunction(() => new URLSearchParams(location.search).has('seed'), null, { timeout: 60000 });
    const moved = await page.evaluate(() => location.search);
    let hWait = null; for (let i = 0; i < 600; i++) { hWait = await health(); if (hWait && hWait.seats[MINE]) break; await sleep(50); }   // (asked the moment the moved page has said hello: it has the seat and its game is nowhere near stood — the server must be waiting, no tick run)
    await page.evaluate(() => window.__ready);
    out = await page.evaluate(async ({ N, BREAK, NO_AHEAD, PICK }) => {
      window.__testVisible = true;
      const g = window.__game, start = [...g.player.char.pos], t0 = performance.now(); let broke = false;
      const S = () => window.__net.state;
      // THE KEY GOES DOWN: level and still first; then ONE frame with a walk key — the look ahead has him moved, the game has not had the record yet
      const pos = () => [...g.player.char.pos], gone = (a, b) => Math.hypot(a[0] - b[0], a[2] - b[2]);
      console.log('[mpc] stood', JSON.stringify(S()).slice(0, 300));
      // (level first, by bursts with no key down — he has not walked yet, so every record on its way holds nothing — and the key's frame at once: a page that
      //  is ticks behind looks ahead through the ticks it has not walked before it reaches its own newest record)
      { const t = performance.now(); while ((S().state !== 'playing' || !S().sent || S().behind > 2 || S().far) && performance.now() - t < 120000) { window.__runFrames(10, 1 / 60, [], 'none'); await new Promise(r => setTimeout(r, 0)); } }
      console.log('[mpc] level', S().confirmed, S().behind, S().sent, JSON.stringify(S().ahead));
      // (the first way he has room to walk: a key that walks him into the wall he stands at moves him nowhere, by the look ahead or by the game, and says nothing.
      //  A page's record goes out on the client's own beat, not on every frame, and the server's answer comes by the socket: so frame by frame, each one
      //  yielding, the frame the FIRST record with the key down went out is the one held to the bar, and the frames and milliseconds until the game moved him)
      const tick0 = () => new Promise(r => setTimeout(r, 0)), moving = () => (S().ahead ? S().ahead.move : 0);
      const settle = async () => { const t = performance.now(); let q = pos(), still = 0; while ((still < 3 || S().behind > 3 || S().pending > 12 || S().far) && performance.now() - t < 30000) { window.__runFrames(10, 1 / 60, [], 'none'); const r = pos(); still = gone(r, q) < 1e-6 && moving() < 1e-6 ? still + 1 : 0; q = r; await tick0(); } };
      let key = { code: '', rec: -1, ahead: -1, game: 0, pending: 0, behind: 0, later: -1, laterMs: -1 };
      for (const code of ['KeyW', 'KeyD', 'KeyS', 'KeyA']) {
        await settle(); const p0 = pos(), s0 = S().sent; let at = 0;
        key = { code, rec: -1, ahead: -1, game: 0, pending: 0, behind: S().behind, later: -1, laterMs: -1 };
        for (let i = 0; i < 240 && key.later < 0; i++) {
          window.__runFrames(1, 1 / 60, [code], 'none'); const mv = gone(pos(), p0);
          if (key.rec < 0 && S().sent > s0) { key.rec = i; at = performance.now(); key.ahead = moving(); key.game = mv; key.pending = S().pending; }
          else if (key.rec >= 0 && mv > 1e-4) { key.later = i - key.rec; key.laterMs = Math.round(performance.now() - at); }
          await tick0();
        }
        console.log('[mpc] tried', JSON.stringify(key), JSON.stringify(S().ahead));
        if (key.ahead > 1e-4 || key.later >= 0) break;
      }
      console.log('[mpc] key', JSON.stringify(key));
      const ahead = { samples: 0, on: 0, most: 0, drawn: 0, moved: 0, men: 0, caps: 0, boxes: 0, lights: 0, walls: 0, gave: null };
      const base = S().confirmed, w0 = S().wordsOk, tm = performance.now(); let n = 0;
      const even = async keys => { const t = performance.now(); while ((S().behind > 3 || S().far) && !S().bad && !S().broken && !S().bye && performance.now() - t < 180000) { window.__runFrames(10, 1 / 60, keys, 'none'); await tick0(); } };
      while (S().confirmed < base + N && !S().bad && !S().broken && !S().bye && performance.now() - tm < 600000) {
        const leg = Math.floor(n / 9) % 4, keys = [['KeyW', 'KeyD', 'KeyS', 'KeyA'][leg]]; n++;
        window.__mouse(240 + Math.cos(n * 0.3) * 120, 135 + Math.sin(n * 0.3) * 80); if (n % 6 === 0) window.__click(0);
        window.__runFrames(10, 1 / 60, keys, 'none');
        if (!S().far) { const a = S().ahead; ahead.samples++; if (a && a.move > 0.02) ahead.on++; if (a && a.move > ahead.most) ahead.most = a.move; }
        if (n % 20 === 0) console.log('[mpc] main', n, S().confirmed, S().behind, S().far, S().wordsOk, JSON.stringify(S().bad), JSON.stringify(S().ahead), Math.round(performance.now() - tm));
        if (BREAK && !broke && S().confirmed >= base + 90) { broke = true; const p = g.player.char.pos, f = new Float64Array([p[0]]), u = new Uint32Array(f.buffer); u[0] ^= 1; p[0] = f[0]; }
        await tick0();
      }
      const rowsOf = () => { const R = {}; for (const r of document.querySelectorAll('#panel .row')) { const l = r.querySelector('label'); if (l) R[l.textContent.trim()] = r; } return R; };
      const pend = async loops => { const v = []; for (let i = 0; i < loops; i++) { window.__runFrames(10, 1 / 60, [['KeyW', 'KeyD', 'KeyS', 'KeyA'][Math.floor(i / 9) % 4]], 'none'); if (!S().far) v.push(S().pending); await tick0(); } v.sort((a, b) => a - b); return v.length ? v[v.length >> 1] : -1; };
      const until = async ok => { const t = performance.now(); while (!ok() && performance.now() - t < 10000) { window.__runFrames(10, 1 / 60, [], 'none'); await tick0(); } };   // (an eased number given the time to get there, whatever the machine's pace)
      // THE LOOK AHEAD'S BOX on the debugger's Network page (src/ui/netrows.ts), while the fight is young: off, he is drawn where the game has him; on again, ahead.
      // (Asked here and not with the page's other rows below, which need no fight: those come after the drawn frames, minutes of the server's time later.)
      const boxed = { asked: false, down: false, off: null, on: 0 };
      if (BREAK !== true && !NO_AHEAD && !S().bad && !S().broken && !S().bye) {
        window.__panel.show('Network'); const row = rowsOf()['Draw your own man ahead'], box = row && row.querySelector('input');
        boxed.down = !!g.player.down || !!S().outcome;   // (a man shot down, a mission that is over: nobody is ahead by rule, and the box's two looks are not asked)
        if (box && !boxed.down) { boxed.asked = true; box.checked = false; box.dispatchEvent(new Event('change')); await pend(6); boxed.off = S().ahead; box.checked = true; box.dispatchEvent(new Event('change')); await pend(12); boxed.on = S().ahead ? S().ahead.n : 0; boxed.down = !!g.player.down || !!S().outcome; }
        window.__panel.toggle();
      }
      // A TAB THAT SLEEPS, twice, the walk key down as it goes: first with nothing said (the server holds the seat's keys: the man walks on), then with the
      // hands taken off as a hidden tab or a card does (one record: he stays). Each time the page wakes far behind and catches up to the server's word.
      const main = { base, confirmed: S().confirmed, wordsOk: S().wordsOk, words: S().wordsOk - w0, secs: +((performance.now() - tm) / 1000).toFixed(1) };
      const yieldNow = () => new Promise(r => setTimeout(r, 0)), at = () => [g.player.char.pos[0], g.player.char.pos[2]];
      const level = async keys => { const t = performance.now(); let far = false; while ((S().behind > 2 || S().far) && !S().bad && performance.now() - t < 180000) { window.__runFrames(10, 1 / 60, keys, 'none'); far = far || S().far; await yieldNow(); } return far; };
      const walk = async (key, n) => { const a = at(); for (let i = 0; i < n; i++) { window.__runFrames(10, 1 / 60, [key], 'none'); await yieldNow(); } await level([key]); const b = at(); return Math.hypot(b[0] - a[0], b[1] - a[1]); };
      // (the floor for the walk: the way a nap walks is back over floor he has just walked, so a wall at his shoulder says nothing. He walks each way two
      //  seconds until one gives him two metres; the nap's key is the one that takes him back)
      const BACK = { KeyW: 'KeyS', KeyS: 'KeyW', KeyA: 'KeyD', KeyD: 'KeyA' };
      const nap = async rest => {
        let out = 'KeyW', room = -1; for (const k of ['KeyW', 'KeyD', 'KeyS', 'KeyA']) { const d = await walk(k, 12); if (d > room) { room = d; out = k; } if (d >= 2) break; }
        const key = BACK[out]; await walk(key, 2);
        const a = at(), down = !!g.player.down; if (rest) window.__net.client.rest(performance.now());
        await new Promise(r => setTimeout(r, 3500));
        const behind = S().behind, far = await level([]), b = at();
        return { behind, far, key, down, room: +room.toFixed(2), walked: +Math.hypot(b[0] - a[0], b[1] - a[1]).toFixed(2) };
      };
      const naps = BREAK === true || S().bad || S().broken || S().bye ? null : { held: await nap(false), rest: await nap(true), words: S().wordsOk, heard: S().heard, bad: !!S().bad };
      // TWO FRAMES DRAWN, last of the fight's own asks (the software adapter is a minute at a drawn frame with every core, and the page falls thousands of ticks behind after
      // each: everything else is asked first, early in the fight). He walks until the look has him ahead, the frame is drawn with him moved, and the ticks
      // after it read the world as the game has it: the state words after the draws say so.
      const on = () => !S().bad && !S().broken && !S().bye;
      // (HIS BACK TO A WALL: a nap's held key walks him on for seconds, and a key held into a wall takes the hold — player.ts wallPress. On it he has no look
      //  ahead by rule, and a lone key within WALLSNAP.alongDeg of the wall's line only sidles him, so where the camera stands across that line no one key
      //  takes him off. The keys nearest square away from the wall do, as a player's two fingers would)
      const WAYS = [[['KeyW'], 1, 0], [['KeyS'], -1, 0], [['KeyD'], 0, 1], [['KeyA'], 0, -1], [['KeyW', 'KeyD'], 1, 1], [['KeyW', 'KeyA'], 1, -1], [['KeyS', 'KeyD'], -1, 1], [['KeyS', 'KeyA'], -1, -1]];
      const offWall = () => { const w = g.player.wall; if (!w || !window.__cam) return null; const B = window.__cam.planarBasis(); let best = null, top = -2;
        for (const [ks, f, r] of WAYS) { const x = B.fwd[0] * f + B.right[0] * r, z = B.fwd[2] * f + B.right[2] * r, d = (x * w.n[0] + z * w.n[2]) / Math.hypot(x, z); if (d > top) { top = d; best = ks; } }
        return best; };
      for (let d = 0; d < 2 && BREAK !== true && on(); d++) {
        let keys = ['KeyW'], tries = 0;
        while (tries < 60 && on() && (S().far || moving() <= 0.02)) { const off = offWall(); if (off) ahead.walls++; keys = off ?? [['KeyW', 'KeyD', 'KeyS', 'KeyA'][Math.floor(tries / 4) % 4]]; window.__runFrames(10, 1 / 60, keys, 'none'); tries++; await tick0(); }
        // (he never got ahead in 60 tries: WHY is kept. A man shot down and a mission that is over are states with no look ahead by rule — foresee.ts, and a
        //  card that freezes the page takes the hands off — so the drawn frames are then not asked, as the sleeping tab's walk is not of a man who is down)
        if (!on() || S().far || !(moving() > 0.02)) { ahead.gave = { down: !!g.player.down, over: !!S().outcome, far: S().far, state: S().state, behind: S().behind, pending: S().pending, tick: S().confirmed, tries,
          // (what else takes his legs from him or the camera off him: said, so a red line names its cause — player.ts legsOwn, foresee.ts `his`)
          legs: ['takedown', 'holding', 'kick', 'picking', 'wanding', 'coding', 'wall', 'dragging', 'haul', 'doorMove', 'crouch'].filter(k => g.player[k]).concat(g.puppet ? ['puppet'] : [], g.touring ? ['touring'] : []).join(' '),
          cam: (() => { try { return g.followPos() === g.player.char.pos; } catch { return null; } })(), look: S().ahead }; break; }
        window.__runFrames(1, 1 / 60, keys, true);
        const L = window.__net.ahead && window.__net.ahead.last; ahead.drawn++; if (L && L.on) { ahead.moved++; ahead.men = Math.max(ahead.men, L.men); ahead.caps = Math.max(ahead.caps, L.caps); ahead.boxes = Math.max(ahead.boxes, L.boxes); ahead.lights = Math.max(ahead.lights, L.lights); }
        await tick0(); await even(keys);
      }
      // THE DEBUGGER'S NETWORK PAGE (src/ui/netrows.ts): its rows are moved as a hand moves them (the list's own change event, the box's, the button's click)
      // while he walks, and the page plays on under each. AFTER the drawn frames, so that nothing here costs the lines above a man still on his feet. A long connection is picked and his inputs wait longer for their answer; the look ahead is put
      // off and on; the connection is cut for two seconds; then all of it is put back. Every state word is still the server's at the end (the lines below).
      const netrows = BREAK === true || S().bad || S().broken || S().bye ? null : await (async () => {
        window.__panel.show('Network'); const R = rowsOf(), sel = R['Connection'] && R['Connection'].querySelector('select'), box = R['Draw your own man ahead'] && R['Draw your own man ahead'].querySelector('input'), cut = R['Stall'] && R['Stall'].querySelectorAll('button')[1];   // (the long one)
        const o = { rows: !!sel && !!box && !!cut && !!R['Delay each way (ms)'] && !!R['Jitter (ms)'] && !!R['Loss (%)'], kinds: sel ? [...sel.options].map(x => x.textContent) : [], before: -1, far: -1, held0: -1, held: -1, heldBack: -1, margins: [], stallText: cut ? cut.textContent : '', tried: null, slider: '', off: null, on: null, stalled: -1, back: -1, backTried: null, kept: '' };
        if (!o.rows) return o;
        await even([]); o.before = await pend(18); o.held0 = S().heldMs; o.margins.push(S().margin);
        sel.value = String(PICK); sel.dispatchEvent(new Event('change')); o.tried = window.__net.tried; window.__panel.refresh(); o.slider = R['Delay each way (ms)'].querySelector('input').value;
        await pend(18); o.far = await pend(18); await until(() => S().heldMs >= 70); o.held = S().heldMs; o.margins.push(S().margin);
        try { o.kept = sessionStorage.getItem('tps.net.try') || ''; } catch { /* keeps nothing */ }   // (written once a row rests)
        const t0 = performance.now(); cut.click(); let gap = 0, hl = S().heard, since = t0;   // (the longest stretch with no new tick heard from the server)
        while (performance.now() - t0 < 4200) { window.__runFrames(10, 1 / 60, ['KeyW'], 'none'); const t = performance.now(), hn = S().heard; if (hn !== hl) { hl = hn; since = t; } else gap = Math.max(gap, t - since); await tick0(); }
        o.stalled = Math.round(gap); await even([]);
        sel.value = '0'; sel.dispatchEvent(new Event('change')); o.backTried = window.__net.tried; await even([]); await pend(18); o.back = await pend(18); await until(() => S().heldMs < 5); o.heldBack = S().heldMs; o.margins.push(S().margin);
        window.__panel.toggle();
        return o;
      })();
      const after = { confirmed: S().confirmed, wordsOk: S().wordsOk };
      const p = g.player.char.pos;
      return { naps, main, key, ahead, after, netrows, boxed, mine: g.seats.indexOf(g.localSeat), state: S(), moved: +Math.hypot(p[0] - start[0], p[2] - start[2]).toFixed(3), scenario: window.__scenarios.current()?.kind ?? null, fatal: (document.getElementById('fatal') || {}).textContent || '', line: (document.getElementById('netline') || {}).textContent || '' };
    }, { N, BREAK, NO_AHEAD, PICK: BAD_WAY ? 0 : 3 });
    const hEnd = await health(), S = out.state, M = out.main, words = Math.floor(N / 30);
    line(new URLSearchParams(moved).has('roomclear') === /roomclear/.test(setup.scenario) && new URLSearchParams(moved).get('seed') === String(setup.seed) && new URLSearchParams(moved).has('net') && !new URLSearchParams(moved).has('netkey'), `moved: the page was sent to the game's own address, the seat's key kept off it (${moved.slice(0, 120)})`);
    line(!!hWait && hWait.state === 'waiting' && hWait.tick === 0 && hWait.seats[MINE] === true, `ready: while the page stood its game the server held the seat and waited (${hWait?.state}, tick ${hWait?.tick})`);
    line(M.confirmed >= M.base + N && !S.bad && M.words >= words && M.wordsOk >= Math.floor(M.confirmed / 30) && !S.broken && !out.fatal, `same fight: the page walked ${M.confirmed} ticks (the last ${M.confirmed - M.base} in ${M.secs} s, walking and firing) and matched ${M.wordsOk} of ${Math.floor(M.confirmed / 30)} state words${S.bad ? ` (FIRST DIFFERENT after tick ${S.bad.t}: server ${S.bad.server}, its own ${S.bad.mine})` : ''}${S.broken ? ` (${S.broken})` : ''}; a confirmed tick cost ${S.walkMs.toFixed(1)} ms here${out.line ? `; the page said "${out.line}"` : ''}`);
    line(S.seat === MINE && S.sent > 20 && !!hEnd && hEnd.lines[MINE].own > 20 && out.moved > 0.5 && out.mine === MINE, `its own man: seat ${S.seat}${TWO ? ` of ${hEnd?.lines.length} (the page shows seat ${out.mine}'s man)` : ''}, ${S.sent} records sent, the server ran ${hEnd?.lines[MINE].own} ticks on them (${hEnd?.lines[MINE].late} late, ${hEnd?.lines[MINE].held} on a held one), the man moved ${out.moved} m; it aims ${S.margin} ticks past the server's`);
    { const K = out.key, A = out.ahead, a = S.ahead;
      line(K.rec >= 0 && K.ahead > 1e-4 && K.game < 1e-6 && K.later > 0 && A.on > A.samples / 2 && ((A.drawn === 2 && A.moved === A.drawn && A.men === 1 && A.caps === 1 && A.boxes > 0) || (!!A.gave && (A.gave.down || A.gave.over) && A.moved === A.drawn)) && !S.bad && out.after.wordsOk >= Math.floor(out.after.confirmed / 30),
        `ahead: on the frame the first record with a walk key down went out (${K.code}, the first way he has room) the look ahead had his man ${(K.ahead * 1000).toFixed(1)} mm on${K.ahead > 1e-4 ? '' : ' (NOT MOVED)'} and the game ${K.game < 1e-6 ? 'had not moved him' : `HAD moved him ${(K.game * 1000).toFixed(1)} mm`} (${K.pending} record${K.pending === 1 ? '' : 's'} on their way, the page ${K.behind} tick${K.behind === 1 ? '' : 's'} behind); the game moved him ${K.later < 0 ? 'NEVER in 240 frames' : `${K.laterMs} ms later`}; walking, he was ahead of the game in ${A.on} of ${A.samples} looks (${(A.most * 100).toFixed(0)} cm at most); ${A.gave ? `after ${A.drawn} drawn frame${A.drawn === 1 ? '' : 's'} he could not be got ahead for the next (${A.gave.down ? 'his man was DOWN by then' : A.gave.over ? 'the mission was OVER by then' : `NOT down and the mission not over: the page ${A.gave.behind} ticks behind${A.gave.far ? ', catching up' : ''}, ${A.gave.pending} records on their way, ${A.gave.state}; his legs: ${A.gave.legs || 'his own by every flag'}; the camera ${A.gave.cam === false ? 'NOT on him' : 'on him'}; the last look ${A.gave.look ? `played ${A.gave.look.n} records and moved him ${(A.gave.look.move * 1000).toFixed(0)} mm` : 'none'}`}, tick ${A.gave.tick}), so ${A.gave.down || A.gave.over ? 'the rest were NOT asked' : 'this FAILS'}; ` : ''}${A.walls ? `he was taken off a wall first (${A.walls} bursts of the keys square away from it); ` : ''}of ${A.drawn} drawn frames ${A.moved} drew him moved (${A.men} palette instance, ${A.caps} set of shadow proxies, ${A.boxes} boxes, ${A.lights} light${A.lights === 1 ? '' : 's'}) and every state word after them matched (${out.after.wordsOk} by tick ${out.after.confirmed})${a ? `; ${a.fixes} correction${a.fixes === 1 ? '' : 's'} past a centimetre (the longest ${(a.fixMax * 1000).toFixed(0)} mm), a look ${a.ms.toFixed(2)} ms` : ''}`); }
    if (out.netrows) { const W = out.netrows, T = W.tried || {}, B = W.backTried || {}, X = out.boxed;
      line(W.rows && T.lagMs === 80 && T.jitterMs === 15 && T.lossPct === 0 && W.slider === '80' && /"lagMs":80/.test(W.kept) && W.held0 < 5 && W.held >= 70 && W.heldBack < 5 && (NO_AHEAD || (!X.asked && X.down) || (X.asked && X.off === null && (X.on > 0 || X.down))) && W.stalled >= 2500 && B.lagMs === 0 && B.jitterMs === 0 && B.lossPct === 0 && !S.bad && !S.broken,
        `network page: ${W.rows ? `the debugger's Network page has its rows (${W.kinds.length} kinds of connection). Picking "${W.kinds[BAD_WAY ? 0 : 3]}" set this page's connection to ${T.lagMs} ms each way, up to ${T.jitterMs} ms more and ${T.lossPct}% resent (the delay's own row then read ${W.slider}; kept for the tab: ${W.kept ? 'yes' : 'NO'}), and the server's messages, which had waited ${W.held0.toFixed(0)} ms in the page's queue, then waited ${W.held.toFixed(0)} ms${W.held >= 70 ? '' : ' (NO LONGER A WAY)'}; ${NO_AHEAD ? 'the look ahead\'s box was left alone on this path' : !X.asked ? (X.down ? 'his man was down or the mission over by then, so the look ahead\'s box was NOT asked' : 'the look ahead\'s box was NOT FOUND') : `earlier, in the fight, with the look ahead's box off he was drawn ${X.off === null ? 'where the game has him' : 'STILL AHEAD'}, and on again ${X.on} ticks ahead${X.on > 0 || !X.down ? '' : ' (he was down by then)'}`}; the stall's button ("${W.stallText}") kept the server unheard for ${W.stalled} ms and the page caught up; back on "${W.kinds[0]}" they waited ${W.heldBack.toFixed(0)} ms (his inputs on their way through the three: ${W.before}, ${W.far}, ${W.back}, the page aiming ${W.margins.join(', ')} ticks past the server's; on this box the aim is the starved page's own and says little)` : 'the debugger has NO Network page with its rows'}`); }
    if (out.naps) { const Z = out.naps;
      const up = !Z.held.down && !Z.rest.down;   // (a man shot down walks nowhere: then only the waking is asked)
      line(Z.held.far && Z.rest.far && !Z.bad && !S.far && (!up || (Z.held.walked > 1 && Z.rest.walked < Z.held.walked / 3)), `a tab that sleeps: ${up ? `asleep 3.5 s with a walk key down (${Z.held.key}, back over ${Z.held.room} m he had just walked) and nothing said, the server walked the man on ${Z.held.walked} m; with the hands taken off first (${Z.rest.key}), ${Z.rest.walked} m` : 'the man was down by then, so how far he walks on was NOT asked'}; each time the page woke ${Z.held.behind} and ${Z.rest.behind} ticks behind, caught up with the mixer shut and still matched every state word (${Z.words} by tick ${Z.heard})`); }
    line(errs.length === 0, `no page error${errs.length ? `: ${errs.slice(0, 3).join(' | ')}` : ''}`);
    await page.close();
    await Promise.race([botDone, sleep(20000)]);
    const bl = botOut.split('\n').find(x => x.startsWith('MPBOT ')), B = bl ? JSON.parse(bl.slice(6)) : null;
    line(!!B && B.confirmed >= N && !B.bad && B.wordsOk >= words, `beside it: a ${TWO ? 'client in the first seat' : 'watcher'} on bun walked the same ${B?.confirmed ?? 0} ticks and matched ${B?.wordsOk ?? 0} words`);
  } catch (e) { line(false, `the page could not be driven: ${String(e && e.stack || e).slice(0, 400)}${errs.length ? ` · page errors: ${errs.slice(0, 3).join(' | ')}` : ''}`); }
  try { await browser?.close(); } catch { /* gone */ } try { bot.kill(); } catch { /* gone */ } srv.kill();
  console.log(fails ? `mpclient-page: ${fails} line${fails === 1 ? '' : 's'} FAILED${BREAK ? ' (MPCLIENT_BREAK=1: the same-fight line must and the ahead line with it, and its-own-man may: a page out of step stops sending)' : NO_AHEAD ? ' (MPCLIENT_BREAK=ahead: the ahead line must, and no other)' : BAD_WAY ? ' (MPCLIENT_BREAK=way: the network page line must, and no other)' : ''}` : 'mpclient-page: every line passed');
  process.exit(fails ? 1 : 0);
})().catch(e => { console.log(`mpclient-page: ${String(e && e.stack || e).slice(0, 400)}`); process.exit(2); });
