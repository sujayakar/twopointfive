// THE PAGE AS A CLIENT OF A GAME SERVER, CHECKED (src/host/drawnet.ts, src/net/door.ts; docs/agents/net.md). Headless Chromium on the software WebGPU adapter opens the page with ?net= on
// a real server process (tools/mp/server.ts) started here; a scripted client (tools/mp/bot.ts, bun) walks the same game beside it. Both stand the game from the start the server's welcome carries.
//   PORT=5231 node tools/qa/mpclient-page.cjs [--ticks 600] [--setup '{"scenario":"?rescue&levelFile=consulate.json","seed":3}']     (a dev server on that port)
//   --seats 2             a game of TWO players: the page sits in the SECOND seat and the bun client plays the first (one seat: the page plays, the bun client watches)
//   --words DIR           every machine's words of the whole game, one a part, after each word tick, a file a machine in DIR (tools/mp/words-diff.ts says which part two of them differ in)
//   MPCLIENT_BREAK=1 …    the red path: after tick 90 the page steps one record with another walk key than the server ran — its state words must part from the server's
//   MPCLIENT_BREAK=way    the check picks the connection as it is where it should pick a long one — the `network page` line must fail, and no other
//   MPCLIENT_BREAK=ahead  the page is opened with netpredict=0 (its own man drawn where the game has him) — the `ahead` line must fail, and no other
// THE LINES:
//   moved         the page was sent to the game's own address, the seat's key no longer on it
//   ready         the server held the seat and ran no tick while the page stood its game
//   same fight    every state word of the run is the server's
//   every part    beside it, the word of EVERY part of the game is the server's on every word tick (watched: src/net/words.ts)
//   its own man   the page shows its own seat's man, the server ran ticks on its records, and he moved
//   ahead         on the tick the first record with a walk key down went out, the look ahead had his man moved and the game had not; walking, he is ahead; a drawn frame has him where the look has him
//   sleeps        a tab asleep with a key down is walked on by the server, with its hands off it is not; each time the page wakes far behind, catches up and still matches
//   network page  the debugger's rows make the page's connection worse and better, live
// The page is driven by script, two frames of it drawn: what a PLAYER sees and feels on a GPU is not what this checks.
// exit 0 = pass · 1 = a line failed · 2 = something could not be started
const { page: pageRun, NOISE } = require('./page.cjs'), PAGE = pageRun({ name: 'mpclient-page' }), line = PAGE.check;
const { spawn } = require('node:child_process');
const { resolve } = require('node:path');
const fs = require('node:fs');
const ROOT = resolve(__dirname, '../..');
const BREAK = process.env.MPCLIENT_BREAK === '1', NO_AHEAD = process.env.MPCLIENT_BREAK === 'ahead', BAD_WAY = process.env.MPCLIENT_BREAK === 'way', QUALITY = process.env.QUALITY || 'verylow';
const arg = (k, d) => { const i = process.argv.indexOf(`--${k}`); return i >= 0 ? process.argv[i + 1] : d; };
const WORDS = arg('words', ''); if (WORDS) { fs.mkdirSync(WORDS, { recursive: true }); for (const f of fs.readdirSync(WORDS)) if (f.endsWith('.words')) fs.unlinkSync(`${WORDS}/${f}`); }
const N = Number(arg('ticks', '600')), setup = JSON.parse(arg('setup', '{"scenario":"?roomclear&squad4","seed":7}')), TWO = arg('seats', '1') === '2';
if (TWO) setup.scenario += '&seats=2';
const KEYS = TWO ? ['seat-one', 'seat-two'] : ['seat-one'], MINE = KEYS.length - 1;   // (the page takes the last seat)
const MP = 5800 + (process.pid % 150);
const sleep = ms => new Promise(r => setTimeout(r, ms));
const health = async () => { try { return await (await fetch(`http://127.0.0.1:${MP}/health`)).json(); } catch { return null; } };

/** in the page: the hands of the check, and what it reads */
async function drive({ N, BREAK, NO_AHEAD, PICK, WORDS }) {
  const D = window.__draw, G = D.game, net = window.__net, C = net.client, S = () => net.state;
  const tick0 = () => new Promise(r => setTimeout(r, 0)), step = (n, keys, drawn = 'none') => D.run(n, keys, drawn);
  const man = () => net.man(), pos = () => man().at, gone = (a, b) => Math.hypot(a[0] - b[0], a[2] - b[2]), moving = () => (S().ahead ? S().ahead.move : 0);
  const over = () => !!(S().bad || S().broken || S().bye), said = (...a) => console.log('[mpc]', ...a);
  const out = { words: [], names: '' };
  // what the client's walk is handed, wrapped: the red path's one record, and the words kept for a file
  let broke = false, base = Infinity;
  const walk = C.walk;
  C.walk = (host, max) => walk.call(C, { step: cmds => { if (BREAK && !broke && C.confirmed >= base + 90) { broke = true; const i = Math.max(0, C.seat), c = cmds[i]; cmds = cmds.slice(); cmds[i] = { ...c, k: c.k.includes('KeyA') ? ['KeyD'] : ['KeyA'] }; } host.step(cmds); }, word: () => host.word(),
    watch: () => { if (WORDS) { const w = net.whole(); out.names ||= w.names.join(' '); out.words.push(`${C.confirmed} ${w.words.map(x => x.toString(36)).join(' ')}`); } return host.watch(); } }, max);
  // (a square, its legs by THE GAME'S ticks: a driven page steps as fast as the box lets it, and a record goes out on the client's own beat)
  const start = pos(), LEGS = ['KeyW', 'KeyD', 'KeyS', 'KeyA'], leg = () => LEGS[Math.floor(S().confirmed / 90) % 4];
  // LEVEL WITH ITS SERVER, as level as a driven page gets: ten ticks of one turn take their time, and the server confirms a tick every sixtieth of a second of it
  let owed = 0; const turn = keys => { const a = performance.now(); step(10, keys); owed = Math.min(8, Math.ceil((performance.now() - a) * 60 / 1000)); };
  const uneven = () => S().behind > 2 + owed || S().far;
  const level = async keys => { const t = performance.now(); let far = false; owed = 0; while (uneven() && !over() && performance.now() - t < 180000) { turn(keys); far = far || S().far; await tick0(); } return far; };
  // AT REST: hands off, level, and nothing he held still on its way
  const rested = async () => { const t = performance.now(); let q = pos(), n = 0; owed = 0; while ((n < 3 || uneven()) && !over() && performance.now() - t < 30000) { turn([]); const r = pos(); n = gone(q, r) < 1e-6 && moving() < 1e-6 ? n + 1 : 0; q = r; await tick0(); } };
  // OFF A WALL: a key held into a wall takes the hold, and on it he has no look ahead by rule. The keys nearest square away from it take him off
  const WAYS = [[['KeyW'], 1, 0], [['KeyS'], -1, 0], [['KeyD'], 0, 1], [['KeyA'], 0, -1], [['KeyW', 'KeyD'], 1, 1], [['KeyW', 'KeyA'], 1, -1], [['KeyS', 'KeyD'], -1, 1], [['KeyS', 'KeyA'], -1, -1]];
  const offWall = () => { const w = man().wall; if (!w) return null; const B = D.cam.planarBasis(); let best = null, top = -2;
    for (const [ks, f, r] of WAYS) { const x = B.fwd[0] * f + B.right[0] * r, z = B.fwd[2] * f + B.right[2] * r, d = (x * w[0] + z * w[1]) / Math.hypot(x, z); if (d > top) { top = d; best = ks; } }
    return best; };
  const stood = async () => { await rested(); for (let i = 0; i < 40 && !over(); i++) { const off = offWall(); if (!off) return; turn(off); await tick0(); if (!offWall()) await rested(); } };

  // ---- the key goes down
  { const t = performance.now(); while ((S().state !== 'playing' || !S().sent || uneven()) && !over() && performance.now() - t < 120000) { turn([]); await tick0(); } }
  said('level', S().confirmed, S().behind, S().sent);
  let key = { code: '', rec: -1, ahead: -1, game: 0, pending: 0, behind: 0, laterMs: -1 };
  for (const code of LEGS) {   // (the first way he has room to walk: a key that walks him into the wall he stands at says nothing)
    await stood(); const p0 = pos(), s0 = S().sent; let at = 0;
    key = { code, rec: -1, ahead: -1, game: 0, pending: 0, behind: S().behind, laterMs: -1 };
    for (let i = 0; i < 240 && key.laterMs < 0 && !over(); i++) {
      step(1, [code]); const mv = gone(pos(), p0);
      if (key.rec < 0 && S().sent > s0) { key.rec = i; at = performance.now(); key.ahead = moving(); key.game = mv; key.pending = S().pending; }
      else if (key.rec >= 0 && mv > 1e-4) key.laterMs = Math.round(performance.now() - at);
      await tick0();
    }
    said('tried', JSON.stringify(key));
    if (key.ahead > 1e-4 || key.laterMs >= 0) break;
  }

  // ---- the fight: a square walked, the cursor round him, rounds fired
  const ahead = { samples: 0, on: 0, most: 0, drawn: 0, off: 0, led: 0, gave: null };
  base = S().confirmed; const w0 = S().wordsOk, t1 = performance.now(); let shot = 0, told = 0;
  while (S().confirmed < base + N && !over() && performance.now() - t1 < 600000) {
    const I = D.input, t = S().confirmed - base; I.mouseX = 240 + Math.cos(t * 0.03) * 120; I.mouseY = 135 + Math.sin(t * 0.03) * 80; if (t >= shot) { shot = t + 40; I.buttons |= 1; I.clicked |= 1; } else if (t >= shot - 36) I.buttons &= ~1;
    step(10, [leg()]);
    if (!S().far) { const a = S().ahead; ahead.samples++; if (a && a.move > 0.02) ahead.on++; if (a && a.move > ahead.most) ahead.most = a.move; }
    if (t >= told) { told = t + 120; said('fight', S().confirmed, S().behind, S().wordsOk, JSON.stringify(S().bad)); }
    await tick0();
  }
  D.input.buttons = 0;
  out.main = { base, confirmed: S().confirmed, wordsOk: S().wordsOk, words: S().wordsOk - w0, secs: +((performance.now() - t1) / 1000).toFixed(1), walkMs: S().walkMs };

  // ---- the debugger's Network page, its rows moved as a hand moves them
  const rowsOf = () => { const R = {}; for (const r of document.querySelectorAll('#panel .row')) { const l = r.querySelector('label'); if (l) R[l.textContent.trim()] = r; } return R; };
  const pend = async loops => { const v = []; for (let i = 0; i < loops; i++) { step(10, [leg()]); if (!S().far) v.push(S().pending); await tick0(); } v.sort((a, b) => a - b); return v.length ? v[v.length >> 1] : -1; };
  const until = async ok => { const t = performance.now(); while (!ok() && performance.now() - t < 10000) { step(10, []); await tick0(); } };   // (an eased number given the time to get there)
  out.netrows = BREAK || over() ? null : await (async () => {
    window.__panel.show('Network'); const R = rowsOf(), sel = R['Connection'] && R['Connection'].querySelector('select'), box = R['Draw your own man ahead'] && R['Draw your own man ahead'].querySelector('input'), cut = R['Stall'] && R['Stall'].querySelectorAll('button')[1];   // (the long one)
    const o = { rows: !!sel && !!box && !!cut && !!R['Delay each way (ms)'] && !!R['Jitter (ms)'] && !!R['Loss (%)'], kinds: sel ? [...sel.options].map(x => x.textContent) : [], boxed: false, down: false, off: null, on: 0, held: [], inputs: [], tried: null, slider: '', kept: '', stalled: -1, backTried: null };
    if (!o.rows) return o;
    // (a man shot down, a mission that is over: nobody is ahead by rule, and the box's two looks are not asked)
    o.down = man().down || !!S().outcome;
    if (!o.down && !NO_AHEAD) { o.boxed = true; box.checked = false; box.dispatchEvent(new Event('change')); await pend(6); o.off = S().ahead; box.checked = true; box.dispatchEvent(new Event('change')); await pend(12); o.on = S().ahead ? S().ahead.n : 0; o.down = man().down || !!S().outcome; }
    await level([]); o.inputs.push(await pend(18)); o.held.push(S().heldMs);
    sel.value = String(PICK); sel.dispatchEvent(new Event('change')); o.tried = net.tried; window.__panel.refresh(); o.slider = R['Delay each way (ms)'].querySelector('input').value;
    await pend(18); o.inputs.push(await pend(18)); await until(() => S().heldMs >= 70); o.held.push(S().heldMs);
    try { o.kept = sessionStorage.getItem('tps.net.try') || ''; } catch { /* keeps nothing */ }   // (written once a row rests)
    const t0 = performance.now(); cut.click(); let gap = 0, hl = S().heard, since = t0;   // (the longest stretch with no new tick heard from the server)
    while (performance.now() - t0 < 4200) { step(10, ['KeyW']); const t = performance.now(), hn = S().heard; if (hn !== hl) { hl = hn; since = t; } else gap = Math.max(gap, t - since); await tick0(); }
    o.stalled = Math.round(gap); await level([]);
    sel.value = '0'; sel.dispatchEvent(new Event('change')); o.backTried = net.tried; await level([]); await pend(18); o.inputs.push(await pend(18)); await until(() => S().heldMs < 5); o.held.push(S().heldMs);
    window.__panel.toggle();
    return o;
  })();

  // ---- a tab that sleeps, twice, the walk key down as it goes: with nothing said the server holds his keys; with the hands taken off first (one record, as a hidden tab sends) he stays
  // (the floor for it is the floor of his last walk, back the way he came: a key held into whatever stands there walks him nowhere, whatever the server holds)
  const go = async k => { await stood();
    const a = pos(), t0 = performance.now(), c0 = S().confirmed; let d = 0, since = -1;
    while (d < 3 && S().confirmed - (since < 0 ? c0 + 120 : since) < 48 && !over() && performance.now() - t0 < 30000) { turn([k]); await tick0(); const m = gone(pos(), a); if (m > d + 0.05) { d = m; since = S().confirmed; } }
    await rested(); return gone(pos(), a); };
  const BACK = { KeyW: 'KeyS', KeyS: 'KeyW', KeyA: 'KeyD', KeyD: 'KeyA' };
  const nap = async rest => {
    let way = '', room = -1, last = ''; for (const k of ['KeyW', 'KeyS', 'KeyD', 'KeyA']) { const d = await go(k); last = k; if (d > room) { room = d; way = k; } if (d >= 2.5) break; }
    if (last !== way) { await go(BACK[way]); room = await go(way); }
    await stood();
    // (turned back, and asleep at once: not walked back to the wall he started from. A page that hitched is catching up with its hands off the keys: asleep once the key has gone out)
    const k = BACK[way], t0 = performance.now(), out = () => { const r = C.pending[C.pending.length - 1]; return !!r && r[1].k.includes(k); };
    for (let i = 0; (i < 2 || S().far || !out()) && performance.now() - t0 < 30000; i++) { step(10, [k]); await tick0(); }
    const a = pos(), down = man().down; if (rest) C.rest(performance.now());
    // (past the last tick he said anything for, half a second let him come to a stand: what the server does with a man nobody speaks for)
    const lastSaid = Math.max(C.confirmed, ...C.pending.map(q => q[0])), was = C.walk; let quiet = null;
    C.walk = (host, max) => was.call(C, { step: c => { const t = C.confirmed; host.step(c); if (!quiet && t >= lastSaid + 30) quiet = pos(); }, word: () => host.word(), watch: () => host.watch() }, max);
    await new Promise(r => setTimeout(r, 3500));
    const behind = S().behind, far = await level([]), b = pos(); C.walk = was;
    return { behind, far, key: k, down, room: +room.toFixed(2), walked: +gone(b, a).toFixed(2), after: quiet ? +gone(b, quiet).toFixed(2) : -1 };
  };
  out.naps = BREAK || over() ? null : { held: await nap(false), rest: await nap(true), words: S().wordsOk, heard: S().heard };

  // ---- two frames drawn, last of what is asked of the fight (a drawn frame is long on the software adapter, and the page is far behind after it): the drawn man is where the look ahead has him
  for (let d = 0; d < 2 && !BREAK && !over(); d++) {
    let keys = ['KeyW'], tries = 0; const t0 = performance.now();
    // (a turn spent catching up is not a try: a page far behind draws no man ahead, by rule)
    while (tries < 60 && performance.now() - t0 < 240000 && !over() && (S().far || moving() <= 0.02)) { const far = S().far; keys = offWall() ?? [leg()]; turn(keys); if (!far) tries++; await tick0(); }
    if (over() || S().far || !(moving() > 0.02)) { const m = man(); ahead.gave = { down: m.down, over: !!S().outcome, free: m.free, wall: !!m.wall, far: S().far, behind: S().behind, pending: S().pending, tick: S().confirmed, tries }; break; }
    step(1, keys, 'last');
    const a = S().ahead, m = man(), at = G.at(m.id);
    if (a && at) { ahead.drawn++; ahead.off = Math.max(ahead.off, gone(at, a.pos)); ahead.led = Math.max(ahead.led, gone(at, m.at)); }
    await tick0(); await level(keys);
  }
  await level([]);
  const m = man(), state = S();
  return Object.assign(out, { key, ahead, state, shown: G.view().me === m.id, moved: +gone(pos(), start).toFixed(3), fatal: getComputedStyle(document.getElementById('fatal')).display !== 'none' ? document.getElementById('fatal').textContent : '', said: (document.getElementById('netline') || {}).textContent || '' });
}

(async () => {
  const srv = spawn('bun', ['run', 'tools/mp/server.ts'], { cwd: ROOT, stdio: ['ignore', 'pipe', 'pipe'], env: { ...process.env, ...(WORDS ? { MP_WORDS: `${resolve(WORDS)}/server` } : {}), MP_PORT: String(MP), MP_BUILD: '', MP_IDLE_S: '600', MP_GRACE_S: '900', MP_SETUP: JSON.stringify({ game: 'check', ...setup, seatKeys: KEYS }) } });
  let srvErr = ''; srv.stderr.on('data', d => { srvErr += d; }); srv.stdout.on('data', () => {});
  let h0 = null; for (let i = 0; i < 200 && !h0; i++) { h0 = await health(); if (!h0) await sleep(100); }
  if (!h0) { srv.kill(); console.log(`mpclient-page: the server did not come up: ${srvErr.trim().split('\n').slice(-2).join(' / ')}`); process.exit(2); }
  const bot = spawn('bun', ['run', 'tools/mp/bot.ts', '--url', `ws://127.0.0.1:${MP}`, '--game', 'check', '--build', '', '--ticks', String(N), '--timeout', '900', ...(TWO ? ['--key', 'seat-one'] : [])], { cwd: ROOT, stdio: ['ignore', 'pipe', 'pipe'], env: { ...process.env, ...(WORDS ? { MP_WORDS: `${resolve(WORDS)}/bun` } : {}) } });
  let botOut = ''; bot.stdout.on('data', d => { botOut += d; }); const botDone = new Promise(r => bot.on('exit', r));

  const errs = [];
  try {
    const { page, go } = await PAGE.open({ size: [480, 270], timeout: 600000, errors: { into: errs, skip: NOISE } });
    if (process.env.MPCLIENT_SAY) page.on('console', m => { if (m.text().startsWith('[mpc]')) console.log('  ' + m.text()); });
    await go(`net=${encodeURIComponent(`ws://127.0.0.1:${MP}`)}&netgame=check&netkey=${KEYS[MINE]}&paused&mute&quality=${QUALITY}${NO_AHEAD ? '&netpredict=0' : ''}`, { waitUntil: 'load' });
    await page.waitForFunction(() => new URLSearchParams(location.search).has('seed'), null, { timeout: 60000 });
    const moved = new URLSearchParams(await page.evaluate(() => location.search));
    let hWait = null; for (let i = 0; i < 600; i++) { hWait = await health(); if (hWait && hWait.seats[MINE]) break; await sleep(50); }   // (asked the moment the moved page has said hello: its game is nowhere near stood)
    await page.waitForFunction(() => !!window.__ready && !!window.__draw && !!window.__net); await page.evaluate(() => window.__ready);
    const out = await page.evaluate(drive, { N, BREAK, NO_AHEAD, PICK: BAD_WAY ? 0 : 3, WORDS: !!WORDS });
    const hEnd = await health(), S = out.state, M = out.main, words = Math.floor(N / 30), differ = S.bad ? ` (FIRST DIFFERENT after tick ${S.bad.t}: server ${S.bad.server}, its own ${S.bad.mine})` : '';
    line(moved.has('roomclear') === /roomclear/.test(setup.scenario) && moved.get('seed') === String(setup.seed) && moved.has('net') && !moved.has('netkey'), `moved: the page was sent to the game's own address, the seat's key kept off it (?${moved.toString().slice(0, 120)})`);
    line(!!hWait && hWait.state === 'waiting' && hWait.tick === 0 && hWait.seats[MINE] === true, `ready: while the page stood its game the server held the seat and waited (${hWait?.state}, tick ${hWait?.tick})`);
    line(M.confirmed >= M.base + N && !S.bad && !S.broken && M.words >= words && M.wordsOk >= Math.floor(M.confirmed / 30) && !out.fatal,
      `same fight: the page walked ${M.confirmed} ticks (the last ${M.confirmed - M.base} in ${M.secs} s, walking and firing) and matched ${M.wordsOk} of ${Math.floor(M.confirmed / 30)} state words${differ}${S.broken ? ` (${S.broken})` : ''}; to the end of the check ${S.wordsOk}; a confirmed tick cost ${M.walkMs.toFixed(1)} ms here${out.said ? `; the page said "${out.said}"` : ''}`);
    line(S.watchOk > 0 && S.watchOff === 0, `every part: the word of the whole game was the server's on ${S.watchOk} of ${S.watchOk + S.watchOff} word ticks${S.watchOff ? ` (FIRST DIFFERENT after tick ${S.watchFirst}: --words DIR, then tools/mp/words-diff.ts, names the parts)` : ''}`);
    line(S.seat === MINE && out.shown && S.sent > 20 && !!hEnd && hEnd.lines[MINE].own > 20 && out.moved > 0.5, `its own man: seat ${S.seat}${TWO ? ` of ${hEnd?.lines.length}` : ''}, ${out.shown ? 'and it shows its own man' : 'and it shows ANOTHER man'}; ${S.sent} records sent, the server ran ${hEnd?.lines[MINE].own} ticks on them (${hEnd?.lines[MINE].late} late, ${hEnd?.lines[MINE].held} on a held one), the man moved ${out.moved} m; it aims ${S.margin} ticks past the server's`);
    { const K = out.key, A = out.ahead, g = A.gave, rule = g && (g.down || g.over), a = S.ahead;
      line(K.rec >= 0 && K.ahead > 1e-4 && K.game < 1e-6 && K.laterMs >= 0 && A.on > A.samples / 2 && (A.drawn === 2 || (rule && A.drawn < 2)) && (A.drawn === 0 || (A.off < 0.02 && A.led > 0.01)),
        `ahead: on the tick the first record with a walk key down went out (${K.code}) the look ahead had his man ${(K.ahead * 1000).toFixed(1)} mm on and the game ${K.game < 1e-6 ? 'had not moved him' : `HAD moved him ${(K.game * 1000).toFixed(1)} mm`} (${K.pending} on their way, ${K.behind} ticks behind); the game moved him ${K.laterMs < 0 ? 'NEVER' : `${K.laterMs} ms later`}; walking, he was ahead in ${A.on} of ${A.samples} looks (${(A.most * 100).toFixed(0)} cm at most); ${A.drawn} drawn frame${A.drawn === 1 ? ' has' : 's have'} him within ${(A.off * 1000).toFixed(1)} mm of the look, as far as ${(A.led * 100).toFixed(0)} cm from where the game has him${g ? `; he could not be got ahead for the next (${g.down ? 'DOWN by then' : g.over ? 'the mission OVER by then' : `legs his own: ${g.free}, on a wall: ${g.wall}, far: ${g.far}, ${g.behind} behind, ${g.pending} on their way`}, tick ${g.tick}, ${g.tries} tries)${rule ? ', so the rest were NOT asked' : ''}` : ''}${a ? `; ${a.fixes} correction${a.fixes === 1 ? '' : 's'} past a centimetre (the longest ${(a.fixMax * 1000).toFixed(0)} mm), a look ${a.ms.toFixed(2)} ms` : ''}`); }
    if (out.naps) { const Z = out.naps, up = !Z.held.down && !Z.rest.down;   // (a man shot down walks nowhere: then only the waking is asked)
      line(Z.held.far && Z.rest.far && !S.bad && !S.far && (!up || (Z.held.walked > 0.5 && Z.rest.after >= 0 && Z.rest.after < 0.05)), `sleeps: ${up ? `asleep 3.5 s with a walk key down (${Z.held.key}, back over ${Z.held.room} m he had just walked) and nothing said, the server walked the man on ${Z.held.walked} m (${Z.held.after} of them past the last record he sent and half a second); with the hands taken off first (${Z.rest.key}, his records on their way took him ${Z.rest.walked} m), ${Z.rest.after} m past that` : 'the man was down by then, so how far he walks on was NOT asked'}; each time the page woke ${Z.held.behind} and ${Z.rest.behind} ticks behind, ${Z.held.far && Z.rest.far && !S.far ? 'caught up over ticks nobody saw' : 'did NOT catch up unseen'} and still matched every state word (${Z.words} by tick ${Z.heard})`); }
    if (out.netrows) { const W = out.netrows, T = W.tried || {}, B = W.backTried || {};
      line(W.rows && T.lagMs === 80 && T.jitterMs === 15 && T.lossPct === 0 && W.slider === '80' && /"lagMs":80/.test(W.kept) && W.held[0] < 5 && W.held[1] >= 70 && W.held[2] < 5 && (NO_AHEAD || (!W.boxed && W.down) || (W.off === null && (W.on > 0 || W.down))) && W.stalled >= 2500 && B.lagMs === 0 && B.jitterMs === 0 && B.lossPct === 0,
        `network page: ${W.rows ? `the debugger's Network page has its rows (${W.kinds.length} kinds of connection). Picking "${W.kinds[BAD_WAY ? 0 : 3]}" set this page's connection to ${T.lagMs} ms each way, up to ${T.jitterMs} ms more and ${T.lossPct}% resent (the delay's own row then read ${W.slider}; kept for the tab: ${W.kept ? 'yes' : 'NO'}), and the server's messages waited ${W.held.map(x => x.toFixed(0)).join(', ')} ms in the page's queue before, on it and after; ${NO_AHEAD ? 'the look ahead\'s box was left alone on this path' : !W.boxed ? 'his man was down or the mission over by then, so the look ahead\'s box was NOT asked' : `with the look ahead's box off he was drawn ${W.off === null ? 'where the game has him' : 'STILL AHEAD'}, and on again ${W.on} ticks ahead`}; the long stall kept the server unheard for ${W.stalled} ms and the page caught up (his inputs on their way through the three: ${W.inputs.join(', ')})` : 'the debugger has NO Network page with its rows'}`); }
    line(errs.length === 0, `no page error${errs.length ? `: ${errs.slice(0, 3).join(' | ')}` : ''}`);
    if (WORDS) { if (out.names) fs.writeFileSync(`${WORDS}/page.0.words`, `names ${out.names}\n${out.words.join('\n')}\n`); console.log(`  (--words: the page's words of ${out.words.length} word ticks, the server's and the bun client's, are in ${WORDS})`); }
    await page.close();
    await Promise.race([botDone, sleep(20000)]);
    const bl = botOut.split('\n').find(x => x.startsWith('MPBOT ')), B = bl ? JSON.parse(bl.slice(6)) : null;
    line(!!B && B.confirmed >= N && !B.bad && B.wordsOk >= words && B.watchOff === 0, `beside it: a ${TWO ? 'client in the first seat' : 'watcher'} on bun walked the same ${B?.confirmed ?? 0} ticks and matched ${B?.wordsOk ?? 0} words, and every part of the game on ${B?.watchOk ?? 0}`);
  } catch (e) { line(false, `the page could not be driven: ${String(e && e.stack || e).slice(0, 400)}${errs.length ? ` · page errors: ${errs.slice(0, 3).join(' | ')}` : ''}`); }
  await PAGE.close(); try { bot.kill(); } catch { /* gone */ } srv.kill();
  await PAGE.end(PAGE.fails ? `mpclient-page: ${PAGE.fails} line${PAGE.fails === 1 ? '' : 's'} FAILED${BREAK ? ' (MPCLIENT_BREAK=1: the same-fight line must, the every-part line and the ahead line with it)' : NO_AHEAD ? ' (MPCLIENT_BREAK=ahead: the ahead line must, and no other)' : BAD_WAY ? ' (MPCLIENT_BREAK=way: the network page line must, and no other)' : ''}` : 'mpclient-page: every line passed');
})().catch(e => { console.log(`mpclient-page: ${String(e && e.stack || e).slice(0, 400)}`); process.exit(2); });
