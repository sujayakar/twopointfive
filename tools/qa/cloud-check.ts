// THE HOSTED PAGE'S BACKEND — the check (src/cloud/ · cloud/convex/ · tools/cloud/pull.ts):   bun run tools/qa/cloud-check.ts
// NEEDS the backend's own packages: `npm ci` in cloud/ once (the game itself has none). The backend here is a STAND-IN (cloud/test/standin.ts): the real
// functions and schema of cloud/convex run in the service's test library, behind the same two HTTP routes a deployment answers on. What is real: every
// function, every validator, the wire's bytes. What is not: the service. NEVER RUN AGAINST A REAL DEPLOYMENT FROM THIS TOOL.
//   off        a build with no address has no backend: nothing is made, no id, no request; the address bar can point a visit at this machine and nowhere else;
//              a browser that has made no choice sends nothing
//   backend    settings, tapes and records through the wire: what is written reads back; a piece sent twice is kept once; another browser cannot write into a
//              visit; bad arguments and the internal reads without a key are refused
//   tapes      a run kept in pieces and sent up as it is played reads back out of the backend (pull.ts) as the SAME FILE the browser's own store makes, and a
//              fresh game walks its run tape to the first run's digest; a second round sends nothing; a call that fails loses nothing; a piece past one
//              document goes in parts; a browser that cannot pack sends lines that still read; lost notes ask the backend and send nothing twice
//   identity   a game stepped with its tape going up digests, second by second, as the same game with no backend at all
//   telemetry  frames in, one record a minute of live play with the numbers that were fed; a run's end, an error once, the visit's end; all read back;
//              the report of what died and what threw (tools/cloud/errors.ts)
//   prefs      one browser's settings open in another that is handed its id; a key removed is removed there; machine keys and saves never leave
//   local      a page served from this machine asks its own server for its backend (cloudFind) and takes it as the browser's own choice; the dev
//              server's side (tools/cloud/local.ts) starts `convex dev` with no deploy key from the shell, or uses the backend it is handed
//   notes      what a player wrote on a replay (a kept visit's notes, the F7 shelf's saved windows) goes up as the hand-over format's own lines, only
//              where a line changed; a saved window whose visit is not kept goes up whole, once; pull.ts writes the lines into its file and the
//              reader takes them; the viewer's own way in sends under the same rules
//   files      a file the page saved goes into the backend's file storage and reads back (pull.ts) byte for byte; held while the browser waits for the
//              notice, not held where its switch is off; a failed sending is tried again and kept once; too big, too many, a bad name are refused
// The red path: CLOUD_BREAK=1 in front (src/cloud/tapes.ts drops every third piece on its way up, and a round asked for while one runs is that running
// one; src/cloud/telemetry.ts holds a run's record for the next beat; src/cloud/cloud.ts does not wait for the notice) must
// FAIL `tapes`, `off`, `files` (src/cloud/files.ts sends the bytes and never writes their row), `notes` (src/cloud/notes.ts marks a line as gone unsent)
// and `telemetry` (src/cloud/telemetry.ts reads a run's end off the mission's stage word alone: a house taken writes no record; src/cloud/loadtime.ts
// writes the load's record as it was, with no whole load in it) — and in `off` the lines of the GPU's wait on a frame (never asked) and of the reporter
// (src/cloud/cloud.ts never asks for the reports).
// EXIT 0 all pass · 1 a failure · 2 the backend's packages are not installed.
import { gunzipSync } from 'node:zlib';
import { existsSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
if (!existsSync(`${ROOT}/cloud/node_modules/convex-test`)) { console.error('cloud-check: the backend\'s packages are not installed — run `npm ci` in cloud/ first'); process.exit(2); }
const { standIn } = await import(`${ROOT}/cloud/test/standin.ts`);
const { ReplayBuffer } = await import(`${ROOT}/src/game/replaybuf.ts`);
const F = await import(`${ROOT}/src/replay/format.ts`);
const KP = await import(`${ROOT}/src/replay/kept.ts`);
const KS = await import(`${ROOT}/src/replay/keepstore.ts`);
const { ReplayKeeper } = await import(`${ROOT}/src/replay/keeper.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const { levelFingerprint } = await import(`${ROOT}/src/game/savestate.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { rerun } = await import('./rerun.ts');
const { wire, WireError, toBase64, fromBase64 } = await import(`${ROOT}/src/cloud/wire.ts`);
const { TapeSync, TAPESYNC, memoryNotes } = await import(`${ROOT}/src/cloud/tapes.ts`);
const { PrefSync, prefsNow, skipped } = await import(`${ROOT}/src/cloud/prefs.ts`);
const { Telemetry, TELEMETRY, clean, endStage } = await import(`${ROOT}/src/cloud/telemetry.ts`);
const { FileSync, FILESYNC, fileName } = await import(`${ROOT}/src/cloud/files.ts`);
const { NoteSync } = await import(`${ROOT}/src/cloud/notes.ts`);
const CL = await import(`${ROOT}/src/cloud/cloud.ts`);
const { pullDump, pullFiles } = await import(`${ROOT}/tools/cloud/pull.ts`);
const ER = await import(`${ROOT}/tools/cloud/errors.ts`);

let fails = 0;
const check = (ok: boolean, text: string) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); if (!ok) fails++; };
const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);
const hashBytes = (u: Uint8Array): number => { let h = 2166136261; for (let i = 0; i < u.length; i++) h = Math.imul(h ^ u[i], 16777619); return h >>> 0; };
const refused = async (p: Promise<unknown>): Promise<string> => { try { await p; return ''; } catch (e: any) { return e instanceof WireError && e.refused ? e.message : `NOT A REFUSAL: ${String(e)}`; } };
const PID = 'a1b2c3d4e5f6a7b8c9d0e1f2a3b4', PID2 = 'z9y8x7w6v5u4t3s2r1q0p9o8n7m6', PIDB = 'b0b1b2b3b4b5b6b7b8b9c0c1c2c3';
/** a key store in memory (what localStorage is to the page) */
function memStore(init: Record<string, string> = {}) {
  const m = new Map(Object.entries(init));
  return { get length() { return m.size; }, key: (i: number) => [...m.keys()][i] ?? null, getItem: (k: string) => m.get(k) ?? null, setItem: (k: string, v: string) => { m.set(k, String(v)); }, removeItem: (k: string) => { m.delete(k); }, map: m };
}

function hands(W: any, f: number): void {
  const I = W.input, s = f / 60;
  I.keys.clear();
  if (s > 0.5 && s < 6) I.keys.add('KeyW'); if (s > 3 && s < 5) I.keys.add('KeyD'); if (s > 9 && s < 12) I.keys.add('KeyS');
  I.mouseX = 800 + 300 * Math.sin(s * 0.7); I.mouseY = 500 + 200 * Math.cos(s * 0.5);
  if (f === 400) I.pressed.add('KeyE');
  if (f === 470 || f === 500 || f === 530) { I.clicked |= 1; I.buttons |= 1; } else I.buttons = 0;
  if (f === 700) I.pressed.add('KeyC'); if (f === 120 || f === 900) I.pressed.add('KeyB');
}
interface Rig { W: any; buf: any; K: any; store: any; hashes: string[]; meta: () => any }
/** a game with its tape being kept (a beat every 300 frames, the last one a flush), as tools/qa/replaykeep-probes.ts stands one up; `mid` is called after every frame */
async function run(secs: number, o: { sid?: string; store?: any; gz?: any; mid?: (R: Rig, f: number) => void | Promise<void> } = {}): Promise<Rig> {
  Math.random = mulberry32(0xbeef);
  const W: any = await standUp({ seed: 7 });
  { const r = startFromQuery(W.game, 'roomclear&squad4'); if (r.why) throw new Error(r.why); }
  const buf = new ReplayBuffer(), store = o.store ?? KS.memoryStore(), sid = o.sid ?? 'visit-a';
  const meta = () => ({ url: '?roomclear&squad4', commit: 'abc1234', userAgent: 'probe', fp: levelFingerprint(W.game), session: sid });
  let now = 0;
  const K = new ReplayKeeper(buf, { sid, store, played: () => true, head: () => { const { session: _s, ...m } = meta(); return buf.head(W.game, m); },
    now: () => new Date(Date.UTC(2026, 9, 26, 12, 0, now++)).toISOString(), ...(o.gz !== undefined ? { gz: o.gz } : {}) });
  buf.tape.born(W.game); W.game.replayTap = (dt: number, cmd?: unknown) => { if (cmd) buf.top(W.game, dt, cmd); else buf.tick(W.game, dt, W.cam); };
  const R: Rig = { W, buf, K, store, hashes: [], meta };
  for (let f = 0; f < secs * 60; f++) {
    hands(W, f); W.step(1 / 60);
    if ((f + 1) % 60 === 0) R.hashes.push(hashDigest(digestGame(W.game, true)));
    if ((f + 1) % 300 === 0) { K.beat(); await K.idle(); }
    if (o.mid) await o.mid(R, f);
  }
  await K.flush();
  return R;
}
/** a hand-over file's bytes as records */
const linesOf = (u: Uint8Array) => KP.parseLines(gunzipSync(Buffer.from(u)).toString('utf8'));

const S = standIn();
const W = wire(S.url), ADMIN = wire(S.url, { key: S.key });
console.log('\n=== the hosted page\'s backend (a stand-in: the real functions, not the service) ===');

// ---- off
{ const Q = (s: string) => new URLSearchParams(s);
  check(CL.cloudUrl(Q('')) === '' && CL.cloudBoot(Q(''), {} as any) === null, 'off: a build that was given no address has no backend — the boot makes nothing');
  check(CL.cloudUrl(Q('cloud=https://elsewhere.example')) === '' && CL.cloudUrl(Q('cloud=http://10.0.0.5:3210')) === '' && CL.cloudUrl(Q('cloud=http://127.0.0.1.evil.example')) === '', 'off: an address on the address bar that is not this machine is not taken');
  check(CL.cloudUrl(Q('cloud=http://127.0.0.1:3210')) === 'http://127.0.0.1:3210' && CL.cloudUrl(Q('cloud=http://localhost:3210/')) === 'http://localhost:3210', 'off: a loopback address on the address bar is (the page checks\' way in)');
  (globalThis as any).__TPS_CLOUD__ = 'https://built-in.example';
  check(CL.cloudUrl(Q('')) === 'https://built-in.example' && CL.cloudUrl(Q('cloud=0')) === '' && CL.cloudUrl(Q('cloud=off')) === '', 'off: a build given an address has one, and ?cloud=0 puts it off for the visit');
  check(CL.CLOUD.shareDefault === true && CL.cloudSharing(Q('')) === true && CL.cloudSharing(Q('cloud=1')) === true && CL.cloudSharing(Q('cloud=0')) === false, 'off: a browser that has made no choice sends (the default, the maintainer\'s pick); ?cloud=0 never does');
  // THE NOTICE COMES FIRST. A browser that sends by the default alone makes not one request until the title page's line has been on its screen (`told`).
  // (The build's made-up address goes nowhere, or — `toStandIn` — to the stand-in, as a hosted build's own address would to its backend.)
  const calls: string[] = [], realFetch = globalThis.fetch; let toStandIn = false;
  (globalThis as any).fetch = (u: string, ...r: any[]) => { calls.push(String(u)); const built = String(u).startsWith('https://built-in.example'); return built && !toStandIn ? Promise.reject(new Error('no such host')) : (realFetch as any)(built ? S.url + String(u).slice('https://built-in.example'.length) : u, ...r); };
  const deps = { sid: 'visit-boot', store: KS.memoryStore(), build: () => 'abc1234', machine: () => ({ webgpu: 'probe' }), facts: () => ({ scenario: 'x', stage: 'infiltrate', live: true, stats: {}, preset: 'high', scale: 1, auto: true }) };
  type Boot = NonNullable<ReturnType<typeof CL.cloudBoot>>; type Hook = { share: boolean; waits: boolean; told: () => void; link: () => string };
  const play = async (C: Boot) => { for (let i = 0; i < 4000; i++) C.frame(1 / 60, true, 'infiltrate', 4); await C.sync(); await C.telemetry.flush(true, deps.facts()); await new Promise(r => setTimeout(r, 50)); };
  // (a page's listeners, so the boot's own leave-taking and the other tab's word can be fired under bun)
  const heard: Record<string, ((ev: any) => void)[]> = {}, listen = (k: string, f: (ev: any) => void) => { (heard[k] ??= []).push(f); }, fire = (k: string, ev: any = {}) => { for (const f of heard[k] ?? []) f(ev); };
  const G = globalThis as any, had = { document: G.document, window: G.window };
  const page = (on: boolean) => { for (const k of Object.keys(heard)) delete heard[k]; if (on) { G.document = { addEventListener: listen, visibilityState: 'visible' }; G.window = { addEventListener: listen }; } else for (const k of ['document', 'window'] as const) { if (had[k] === undefined) delete G[k]; else G[k] = had[k]; } };
  { const store = memStore(); (globalThis as any).localStorage = store; page(true);
    const C0 = CL.cloudBoot(Q(''), deps)!, H0 = C0.hook() as Hook;
    await play(C0); fire('pagehide'); await new Promise(r => setTimeout(r, 50));
    const before = C0.wire.stats.calls, waited = H0.waits;
    check(!!C0 && H0.share === true && waited && before === 0 && calls.length === 0 && (heard.pagehide?.length ?? 0) > 0 && store.getItem('twopointsix.cloud.told') === null, `off: a browser that has not been shown the notice plays and leaves (its page's own goodbye fired) with ${calls.length} requests made`);
    H0.told(); await new Promise(r => setTimeout(r, 50)); await play(C0);
    check(C0.wire.stats.calls > 0 && calls.length === C0.wire.stats.calls && calls.every(u => u.startsWith('https://built-in.example/api/')) && !H0.waits && store.getItem('twopointsix.cloud.told') === '1', `off: once the notice has been on its screen it sends (${calls.length} requests, all to the build's own backend), and the browser remembers it was told`);
    // …a browser that said no sends nothing, told or not; one that said yes itself does not wait for the notice
    (globalThis as any).localStorage = memStore({ 'twopointsix.cloud.share': '0', 'twopointsix.cloud.told': '1' }); page(true);
    const Cn = CL.cloudBoot(Q(''), { ...deps, sid: 'visit-no' })!; await play(Cn); fire('pagehide'); await new Promise(r => setTimeout(r, 50));
    const storeY = memStore({ 'twopointsix.cloud.share': '1' }); (globalThis as any).localStorage = storeY; page(true);
    const Cy = CL.cloudBoot(Q(''), { ...deps, sid: 'visit-yes' })!, Hy = Cy.hook() as Hook; await new Promise(r => setTimeout(r, 50));
    const yes1 = Cy.wire.stats.calls;
    check(Cn.wire.stats.calls === 0 && !(Cn.hook() as Hook).share && yes1 > 0 && !Hy.waits, `off: a browser whose own row says no sends nothing, and one whose row says yes does not wait for the notice (${yes1} requests)`);
    // …and ANOTHER TAB's no stops this one, also where this page's own row had said yes earlier in the visit
    Hy.share = true; storeY.setItem('twopointsix.cloud.share', '0'); fire('storage', { key: 'twopointsix.cloud.share' });
    const yes2 = Cy.wire.stats.calls; await play(Cy); fire('pagehide'); await new Promise(r => setTimeout(r, 50));
    check((Hy.share as boolean) === false && Cy.wire.stats.calls === yes2 && storeY.getItem('twopointsix.cloud.visits') === '[]', `off: another tab that turns the row off stops this page too (${Cy.wire.stats.calls - yes2} requests after it), and the list of visits that may go is emptied`);
    // …a browser that shares its id is not asked for newer settings at its boot before it was told or chose
    (globalThis as any).localStorage = memStore({ 'twopointsix.cloud.linked': '1', 'twopointsix.cloud.pid': PID2 }); const nA = calls.length;
    const went = await CL.cloudAdopt(Q('')); const nB = calls.length;
    (globalThis as any).localStorage.setItem('twopointsix.cloud.told', '1'); await CL.cloudAdopt(Q(''));
    check(went === false && nB === nA && calls.length > nB, 'off: a browser that shares its id asks the backend nothing at its boot before the notice, and does after');
    (globalThis as any).localStorage = memStore(); page(false);
    check(!CL.cloudChosen(Q('')) && CL.cloudChosen(Q('cloud=1')) && CL.cloudChosen(Q(`cloud=${S.url}`)) && !CL.cloudChosen(Q('cloud=0')) && CL.cloudBoot(Q('cloud=0'), deps) === null && CL.cloudUrl(Q('cloud=OFF')) === '' && CL.cloudUrl(Q('cloud=no')) === '' && !CL.cloudSharing(Q('cloud=No')), 'off: only the browser\'s own yes or a word on the address bar is a choice — ?cloud=0 (off, no, in any case) is a build with no backend');
  }
  // THE FIRST VISIT'S TAPE: kept in the browser from its boot, sent whole once the notice has been shown — and a visit played before the notice was ever
  // on this browser's screen stays in the browser
  { const R1 = await run(12, { sid: 'visit-first' }), store = memStore({ 'twopointsix.cloud.visits': '["visit-before"]' }); (globalThis as any).localStorage = store; toStandIn = true;
    let bootOver = false, bootAsked = 0;
    const C = CL.cloudBoot(Q(''), { ...deps, sid: 'visit-first', store: R1.store, boot: () => { bootAsked++; return bootOver ? { interactive: 2140, pipelinesMs: 1250, pipelines: { total: 97, done: 97, failed: 0 }, slowest: [{ label: 'gather', ms: 800 }], nav: 'reload' } : null; } })!, H = C.hook() as Hook;
    const saidHeld = C.file('bench-first.json', new Blob(['{"fps":60}'], { type: 'application/json' }), 'bench', { bench: '{"scenes":2}', commit: 'abc1234' });   // (a file saved before the notice: held, not sent)
    await C.sync(); const quiet = C.wire.stats.calls, listed = store.getItem('twopointsix.cloud.visits'), heldFiles = C.files.waiting;
    H.told(); await new Promise(r => setTimeout(r, 100)); await C.sync();
    const row = (await ADMIN.query('tapes:list', { pid: C.pid })).find((r: any) => r.sid === 'visit-first'), kept = (await R1.store.rows()).find((r: any) => r.sid === 'visit-first');
    check(quiet === 0 && listed === '["visit-before","visit-first"]' && store.getItem('twopointsix.cloud.visits') === '["visit-first"]' && !!row && !!kept && row.have === kept.pieces && C.tapes!.stats.failed === 0,
      `off: the first visit's tape waits for the notice (${quiet} requests) and then goes up whole (${row?.have} of ${kept?.pieces} pieces); a visit from before the notice is off the list`);
    { const mine = (await ADMIN.query('files:list', {})).filter((r: any) => r.sid === 'visit-first');
      const saidNow = C.file('bench-second.json', new Blob(['{"fps":61}']), 'bench'); await C.sync();
      const Cq = CL.cloudBoot(Q('cloudfiles=0'), { ...deps, sid: 'visit-nofiles' })!; const saidNo = Cq.file('quiet.json', new Blob(['{}']), 'bench'); await Cq.sync();
      const theirs = (await ADMIN.query('files:list', {})).filter((r: any) => r.sid === 'visit-nofiles');
      check(quiet === 0 && heldFiles === 1 && mine.length === 1 && mine[0].name === 'bench-first.json' && mine[0].kind === 'bench' && mine[0].size === 10 && mine[0].build === 'abc1234' && same(JSON.parse(mine[0].meta), { bench: '{"scenes":2}', commit: 'abc1234' }) && C.files.waiting === 0 && theirs.length === 0 && Cq.files.stats.asked === 0
        && saidHeld === false && saidNow === true && saidNo === false,
        `off: a file saved before the notice is held (${heldFiles} waiting, ${quiet} requests) and goes up once it has been shown (${mine[0]?.name}, ${mine[0]?.size} bytes); ?cloudfiles=0 holds and sends none; the page is told "going up now" only of a file that is`); }
    // the boot's record: not asked for before the browser sends, not written before the boot is over, and written once
    const asked0 = bootAsked; bootOver = true; await C.sync(); await C.sync(); await C.sync();
    const boots = await ADMIN.query('telemetry:list', { sid: 'visit-first', kind: 'boot' });
    check(asked0 >= 1 && bootAsked === asked0 + 1 && boots.length === 1 && boots[0].data.pipelinesMs === 1250 && boots[0].data.pipelines.total === 97 && boots[0].data.slowest[0].label === 'gather' && boots[0].data.nav === 'reload', `off: the boot's timeline is one record, written once the boot is over (${boots[0]?.data.pipelinesMs} ms on ${boots[0]?.data.pipelines?.total} shader pipelines, a ${boots[0]?.data.nav})`);
    // THE GPU'S WAIT ON A FRAME (telemetry.ts `lag`): one question out at a time, on live frames only, into the minute's record and the report
    { const ER3 = await import(`${ROOT}/tools/cloud/errors.ts`), turn = async () => { await Promise.resolve(); await Promise.resolve(); await Promise.resolve(); };
      const waits: (() => void)[] = []; let asked = 0; const queue = { onSubmittedWorkDone: () => { asked++; return new Promise<void>(r => waits.push(r)); } };
      const realNow = performance.now.bind(performance); let clock = 1000; (performance as any).now = () => clock;
      try {
        C.frame(1 / 60, false, 'infiltrate', NaN, queue); const paused = asked;                       // a pause card: not asked
        const lf = (n: number) => { for (let i = 0; i < n; i++) C.frame(1 / 60, true, 'infiltrate', NaN, queue); }, every = CL.CLOUD.gpuLagEvery;
        lf(3); const one = asked; clock += 40; waits.shift()?.(); await turn();   // one out: the next two frames ask nothing
        lf(every - 3); const spaced = asked;                                      // its answer is in, and still nothing until `every` frames after it was asked
        lf(1); clock += 3400; waits.shift()?.(); await turn();                    // a frozen stretch
        lf(every); clock += 700; waits.shift()?.(); await turn();
        let threw = 0; const Cbad = CL.cloudBoot(Q(''), { ...deps, sid: 'visit-lag-bad' })!; for (let i = 0; i < 12; i++) Cbad.frame(1 / 60, true, 'infiltrate', NaN, { onSubmittedWorkDone: () => { threw++; throw new Error('not on this browser'); } });
        const live = asked, Coff = CL.cloudBoot(Q('gpulag=0'), { ...deps, sid: 'visit-lag-off' })!; Coff.frame(1 / 60, true, 'infiltrate', NaN, queue); const offAsked = asked - live;
        for (let i = 0; i < 4000; i++) C.frame(1 / 60, true, 'infiltrate', NaN); await C.telemetry.flush();
        const pr = (await ADMIN.query('telemetry:list', { sid: 'visit-first', kind: 'perf' })).find((r: any) => r.data?.lag), L = pr?.data?.lag, M = pr ? ER3.machineLines([pr]) : [], mt = ER3.machineText(M);
        check(every === 4 && paused === 0 && one === 1 && spaced === 1 && live === 3 && threw === 1 && offAsked === 0 && !!L && L.n === 3 && L.worst === 3400 && L.over100 === 2 && L.over500 === 2 && L.over2000 === 1 && Math.abs(L.mean - 1380) < 0.01 && M[0]?.lagWorst === 3400 && M[0]?.lagOver500 === 2 && /the GPU's longest wait on a frame 3400 ms, 2 past half a second/.test(mt),
          `off: the GPU's wait on a frame is asked one at a time, on live frames only and not before ${every} frames after the last question (${live} questions in ${2 * every + 1} live frames, ${paused} on a pause card; a queue that cannot be asked is asked ${threw} time), and rides on the minute's record (${L?.n} waits, the worst ${L?.worst} ms, ${L?.over500} past half a second); the report says it by machine, and ?gpulag=0 asks nothing (${offAsked})`);
      } finally { (performance as any).now = realNow; } }
    // ONE REPORTER (src/cloud/report.ts): any file says what went wrong and where; the page's console is the net under it; each is one of the visit's records
    { const RP = await import(`${ROOT}/src/cloud/report.ts`), ER4 = await import(`${ROOT}/tools/cloud/errors.ts`);
      const printed: unknown[][] = [], fake = { error: (...a: unknown[]) => { printed.push(['error', ...a]); }, warn: (...a: unknown[]) => { printed.push(['warn', ...a]); } };
      const realErr = console.error, realWarn = console.warn; console.error = fake.error; console.warn = fake.warn;
      try {
        RP.reportForget();
        // before anybody asks: held, a few, and no more
        const word = (i: number) => String.fromCharCode(97 + i % 26) + String.fromCharCode(97 + (i / 26 | 0));   // (distinct words: lines that differ only in a number are one report)
        for (let i = 0; i < RP.REPORT.heldWarnings + 9; i++) RP.reportWarning('early.warns', `warned ${word(i)}`);
        for (let i = 0; i < RP.REPORT.held + 5; i++) RP.reportError('early.place', `early ${word(i)}`);
        const heldW = RP.reportHeld().filter((r: any) => r.level === 'warning').length, heldN = RP.reportHeld().length - heldW, firstHeld = RP.reportHeld().find((r: any) => r.level === 'error');
        RP.reportForget(); RP.reportError('boot.early', new Error('said before the backend was up'), { n: 1 });
        const Cr = CL.cloudBoot(Q(''), { ...deps, sid: 'visit-report' })!;   // (its boot asks for the reports: the one that waited comes over)
        const boom = new Error('the buffer was refused'); RP.reportError('gpu.splitrays.buffer', boom, { bytes: 124000000, label: 'rays', all: new Float32Array(100000) }); RP.reportError('gpu.splitrays.buffer', new Error('the buffer was refused'));
        RP.reportError('sized.place', 'a buffer of 1200 bytes at 0x7f3a was refused'); RP.reportError('sized.place', 'a buffer of 96 bytes at 0x11ff was refused');   // one error, whatever its numbers
        RP.reportWarning('air.field', 'not built on this machine'); RP.reportError('other.place', 'the buffer was refused');
        // the net: a console that is tapped prints as before and reports; tapped once; a report prints through the console as it was, so it is not its own echo
        const before = printed.length, undo = RP.tapConsole(console as any), again = RP.tapConsole(console as any);
        const printsIt = function printsIt(): void { console.error('WebGPU error:', 'validation failed in pass', { pass: 'gather' }); }; printsIt();
        console.warn('level: a lamp with no room', new Float32Array(1000000)); RP.reportError('named.place', 'named once');
        const afterTap = printed.slice(before).map(a => a.join(' ')); again(); undo(); const untapped = console.error === fake.error;
        Cr.frame(1 / 60, true, 'infiltrate', NaN); await Cr.telemetry.flush(true, deps.facts());
        const er = (await ADMIN.query('telemetry:list', { sid: 'visit-report', kind: 'error' })).reverse(), by = (w: string) => er.filter((r: any) => r.data.where === w), end = (await ADMIN.query('telemetry:list', { sid: 'visit-report', kind: 'end' }))[0];
        const split = by('gpu.splitrays.buffer'), G4 = ER4.errorGroups(er), text = ER4.errorText(G4);
        check(heldN === RP.REPORT.held && heldW === RP.REPORT.heldWarnings && firstHeld.where === 'early.place' && firstHeld.message === 'early aa' && by('boot.early').length === 1 && by('boot.early')[0].data.data.n === 1 && split.length === 1 && split[0].data.message === 'the buffer was refused' && split[0].data.data.bytes === 124000000 && split[0].data.data.all === 'Float32Array(100000)'
          && Array.isArray(split[0].data.stack) && /^at /.test(split[0].data.stack[0]) && /cloud-check/.test(split[0].data.stack[0]) && !('level' in split[0].data) && by('sized.place').length === 1
          && by('other.place').length === 1 && by('air.field').length === 1 && by('air.field')[0].data.level === 'warning',
          `off: a report is one of the visit's records with where it was said, the top of its stack and the place's own numbers (${er.length} records: ${er.map((r: any) => r.data.where).join(' · ')}); the same words at the same place are one record whatever numbers they carry, at another place another; what was reported before the backend was up comes over (${heldN} errors and ${heldW} warnings at most wait, each with places of their own)`);
        check(untapped && afterTap.length === 3 && /^error WebGPU error: validation failed in pass/.test(afterTap[0]) && /^warn level: a lamp with no room/.test(afterTap[1]) && /^error named once$/.test(afterTap[2]) && by('console.error').length === 1 && by('console.error')[0].data.message === 'WebGPU error: validation failed in pass {"pass":"gather"}' && /printsIt/.test(String(by('console.error')[0].data.stack[0])) && by('console.warn').length === 1 && by('console.warn')[0].data.level === 'warning' && by('console.warn')[0].data.message === 'level: a lamp with no room Float32Array(1000000)' && by('named.place').length === 1 && er.filter((r: any) => r.data.message === 'named once').length === 1,
          `off: the net under it — a tapped console prints as before (${afterTap.length} lines) and each console.error and console.warn is a record under its own where, its stack starting at who printed and a big thing beside the line said by its kind and length; a report prints once and is not its own echo; tapped once, and the way back works`);
        check(G4.length === 7 && G4.findIndex((g: any) => g.kind === 'warning') === 5 && G4.every((g: any, i: number) => i < 5 ? g.kind === 'error' : g.kind === 'warning') && /error at gpu\.splitrays\.buffer, other\.place: the buffer was refused/.test(text) && /warning at air\.field: not built on this machine/.test(text) && !!end && end.data.errors === 8 && end.data.warnings === 2 && ER4.machineLines([...er, ...await ADMIN.query('telemetry:list', { sid: 'visit-report', kind: 'visit' })])[0]?.errors === 6 && ER4.machineLines(er)[0]?.warnings === 2,
          `off: the report reads them back by where they were said, errors before warnings (${G4.length} groups), and the visit's end counts every coming (${end?.data?.errors} errors, the same one twice among them, and ${end?.data?.warnings} warnings)`);
        // WHAT A LINE COSTS: said again, with whatever numbers, it is the report it already was (no stack taken twice); a page makes so many and no more
        { RP.reportForget(); const got: any[] = []; RP.reportSink((r: any) => { got.push(r); });
          for (let i = 0; i < 500; i++) RP.reportError('loop.place', `frame ${i} could not be drawn`);
          const undo2 = RP.tapConsole(console as any); for (let i = 0; i < 500; i++) console.error(`[pipeline] gather: failed at ${i}`); undo2();
          const loop = got.slice(0, 500), tapped = got.slice(500, 1000), n0 = got.length;
          for (let i = 0; i < RP.REPORT.makes + 40; i++) RP.reportError('many.place', `kind ${word(i)} went wrong`);
          const distinct = new Set(got).size;
          check(loop.length === 500 && loop.every(r => r === loop[0]) && tapped.length === 500 && tapped.every(r => r === tapped[0]) && tapped[0].where === 'console.error' && distinct === RP.REPORT.makes && got.length - n0 === RP.REPORT.makes - 2,
            `off: a line said again is the report it already was — ${loop.length} comings of one caught error and ${tapped.length} of one printed line, each with a new number, made ${new Set([...loop, ...tapped]).size} reports — and a page makes ${distinct} at most`); }
        // A LINE MOVED WHOLE prints as console.error(a, b) did, every time; a catch's warning prints at its first coming only; the place's numbers are the record's, never the console's
        { RP.reportForget(); const got: any[] = []; RP.reportSink((r: any) => { got.push(r); }); const p0 = printed.length;
          RP.reportErrorLine('render.dust.twin', 'dust look: the twin did not compile', new Error('line 4: bad')); RP.reportErrorLine('render.dust.twin', 'dust look: the twin did not compile', new Error('line 4: bad'));
          RP.reportWarningLine('gpu.shader.compile', '[WGSL warning] gather:12:3 unused'); RP.reportWarningLine('gpu.shader.compile', '[WGSL warning] gather:14:9 unused');
          for (let i = 0; i < 5; i++) RP.reportWarning('replay.keep.prune', new Error(`the store refused piece ${i}`)); RP.reportError('gpu.pipeline', '[pipeline] gather: failed', { label: 'gather' });
          const out = printed.slice(p0).map(a => a.map(x => x instanceof Error ? x.message : String(x)).join(' '));
          check(same(out, ['error dust look: the twin did not compile line 4: bad', 'error dust look: the twin did not compile line 4: bad', 'warn [WGSL warning] gather:12:3 unused', 'warn [WGSL warning] gather:14:9 unused', 'warn the store refused piece 0', 'error [pipeline] gather: failed'])
            && got.length === 10 && got[0].where === 'render.dust.twin' && got[0].message === 'dust look: the twin did not compile line 4: bad' && got[4].level === 'warning' && got[4] === got[8] && got[9].data.label === 'gather',
            `off: a line moved whole prints as it did, every time (${out.filter(l => /dust look/.test(l)).length} of 2), a catch's warning prints at its first coming only (${out.filter(l => /the store refused/.test(l)).length} line for 5 comings, all 5 counted), and the place's own numbers go to the record and never to the console`); }
        // A BROWSER STILL WAITING FOR THE NOTICE keeps what is reported and sends it once the notice has been shown; one that said no keeps and sends nothing
        const realStore = (globalThis as any).localStorage; CL.cloudForget(); RP.reportForget(); (globalThis as any).localStorage = memStore();
        try {
          RP.reportError('boot.untold', 'said by the boot of a first visit'); const nU = calls.length, Cu = CL.cloudBoot(Q(''), { ...deps, sid: 'visit-report-untold' })!;
          RP.reportError('play.untold', 'said before the notice was read'); await new Promise(r => setTimeout(r, 40)); const quietU = calls.length - nU;
          (Cu.hook() as Hook).told(); await new Promise(r => setTimeout(r, 60)); Cu.frame(1 / 60, true, 'infiltrate', NaN); await Cu.telemetry.flush(true, deps.facts());
          const eu = (await ADMIN.query('telemetry:list', { sid: 'visit-report-untold', kind: 'error' })).map((r: any) => r.data.where).sort();
          CL.cloudForget(); RP.reportForget(); (globalThis as any).localStorage = memStore(); (globalThis as any).localStorage.setItem('twopointsix.cloud.share', '0');
          const Cn = CL.cloudBoot(Q(''), { ...deps, sid: 'visit-report-no' }); RP.reportError('play.no', 'said on a browser that does not send'); (Cn?.hook() as Hook | undefined)?.told(); await new Promise(r => setTimeout(r, 40));
          const en = await ADMIN.query('telemetry:list', { sid: 'visit-report-no' });
          check(quietU === 0 && same(eu, ['boot.untold', 'play.untold']) && en.length === 0,
            `off: a first visit's reports wait for the notice (${quietU} requests before it) and are its records once it has been shown (${eu.join(' · ')}); a browser that said no sends none (${en.length})`);
        } finally { (globalThis as any).localStorage = realStore; CL.cloudForget(); }
      } finally { console.error = realErr; console.warn = realWarn; RP.reportForget(); } }
    // A PAGE THAT DIED says so: on the visit that is up, at once…
    { const live = C.fatal({ message: 'The frame loop threw and has been stopped', stack: 'Error: nope\n    at frame (main.ts:729:9)\n    at tick (loop.ts:40:3)', stage: 'frame' });
      const ok = await live.sent, rec = (await ADMIN.query('telemetry:list', { sid: 'visit-first', kind: 'fatal' }))[0];
      check(!live.waits && ok && !!rec && rec.data.stage === 'frame' && same(rec.data.stack, ['Error: nope', 'at frame (main.ts:729:9)', 'at tick (loop.ts:40:3)']) && typeof rec.data.frames === 'number' && rec.build === 'abc1234', `off: a page that dies on a visit that is up writes one fatal record at once (${rec?.data.stage}: ${rec?.data.message})`);
      // …and where the game NEVER STARTED (no adapter: nothing was booted), from a browser that has not seen the notice: nothing until the card has shown it
      CL.cloudForget(); (globalThis as any).localStorage = memStore(); const n0 = calls.length;
      const report = { message: 'Failed to create WebGPU Context Provider', stack: 'Error: no adapter\n    at initGpu (device.ts:40:9)', stage: 'adapter', data: { adapters: [null, null, null], webgl: 'ANGLE (AMD, Radeon RX 6600)' }, machine: { browser: 'Chrome 155', os: 'Windows', webgl: 'ANGLE (AMD, Radeon RX 6600)' }, build: 'abc1234' };
      const RP2 = await import(`${ROOT}/src/cloud/report.ts`), sayErr = console.error; console.error = () => { /* the check's own line */ }; RP2.reportForget(); RP2.reportError('gpu.adapter.ask', 'no adapter on the first ask'); console.error = sayErr;
      const F = CL.cloudFatal(Q(''), report)!; await new Promise(r => setTimeout(r, 60)); const quiet = calls.length - n0, waited = F?.waits;
      F.told(); const went = await F.sent, pidF = (globalThis as any).localStorage.getItem('twopointsix.cloud.pid');
      const recs = (await ADMIN.query('telemetry:list', { kind: 'fatal' })).filter((r: any) => r.pid === pidF), vis = recs[0] ? await ADMIN.query('telemetry:list', { sid: recs[0].sid, kind: 'visit' }) : [];
      const said = recs[0] ? await ADMIN.query('telemetry:list', { sid: recs[0].sid, kind: 'error' }) : []; RP2.reportForget();
      check(waited === true && quiet === 0 && went && said.length === 1 && said[0].data.where === 'gpu.adapter.ask' && recs.length === 1 && recs[0].sid.startsWith('fatal-') && recs[0].data.stage === 'adapter' && recs[0].data.data.webgl === 'ANGLE (AMD, Radeon RX 6600)' && vis.length === 1 && vis[0].data.machine.os === 'Windows' && vis[0].data.started === false && (globalThis as any).localStorage.getItem('twopointsix.cloud.told') === '1',
        `off: a page that never started waits for the notice on its error card (${quiet} requests) and then writes its visit, what it had reported before it died (${said[0]?.data?.where}) and its fatal record (${recs[0]?.data.message})`);
      // …a browser that was told sends at once; one that said no, ?cloudstats=0 and ?cloud=0 send nothing; a visit says it a few times at most; once a visit is up the word goes to it
      CL.cloudForget(); const F2 = CL.cloudFatal(Q(''), { ...report, message: 'second' })!, went2 = !F2.waits && await F2.sent;
      CL.cloudForget(); const n1 = calls.length, none = [CL.cloudFatal(Q('cloudstats=0'), report), CL.cloudFatal(Q('cloud=0'), report)];
      (globalThis as any).localStorage.setItem('twopointsix.cloud.share', '0'); none.push(CL.cloudFatal(Q(''), report)); (globalThis as any).localStorage.removeItem('twopointsix.cloud.share');
      CL.cloudForget(); const many = [0, 1, 2, 3, 4].map(i => CL.cloudFatal(Q(''), { ...report, message: `many ${i}` })); await Promise.all(many.map(m => m?.sent));
      CL.cloudForget(); page(true); const Cr = CL.cloudBoot(Q(''), { ...deps, sid: 'visit-route' })!; const routed = CL.cloudFatal(Q(''), { message: 'routed', stage: 'device' })!; const went3 = await routed.sent;
      const there = await ADMIN.query('telemetry:list', { sid: 'visit-route', kind: 'fatal' });
      check(went2 && none.every(x => x === null) && many.filter(Boolean).length === CL.FATAL.max && went3 && there.length === 1 && there[0].data.message === 'routed' && !!Cr,
        `off: a told browser's dead page sends at once; ?cloudstats=0, ?cloud=0 and a browser that said no send none; a visit says it ${CL.FATAL.max} times at most; once a visit is up the word goes on its records`);
      CL.cloudForget(); }
    // A PAGE THAT IS NOT A VISIT REPORTS TOO (cloud.ts cloudReports: the replay viewer, the game's page playing a tape): under the records' own words, only
    // in a browser that has read the notice, with no request at all until there is something to say
    { const RP5 = await import(`${ROOT}/src/cloud/report.ts`), ER5 = await import(`${ROOT}/tools/cloud/errors.ts`), realStore = (globalThis as any).localStorage, wait = (ms: number) => new Promise(r => setTimeout(r, ms));
      const quietConsole = { error: () => { /* the check's own lines */ }, warn: () => { /* */ } }, realE = console.error, realW = console.warn, secs = CL.PAGE_REPORTS.secs;
      console.error = quietConsole.error; console.warn = quietConsole.warn; CL.PAGE_REPORTS.secs = 0.05;
      const untap = RP5.tapConsole(quietConsole), G5s = globalThis as any, hadSS = G5s.sessionStorage;   // (the page's tap is taken by a stand-in and undone below: the run's own console is left as it was)
      try {
        const fresh = (keys: Record<string, string> = {}) => { CL.cloudForget(); RP5.reportForget(); (globalThis as any).localStorage = memStore(keys); page(true); };
        const TOLD = { 'twopointsix.cloud.told': '1' };
        // a browser that has not read the notice (this page cannot show it): its reports stay in the page
        fresh(); const n0 = calls.length, untold = await CL.cloudReports(Q(''), { page: 'replay' }); RP5.reportError('replay.viewer.shelf.read', 'said before the notice was ever read'); await wait(200);
        const quiet0 = calls.length - n0, heldU = RP5.reportHeld().length;
        // the words that put the records off, and a browser that said no
        fresh(TOLD); const n1 = calls.length, offs = [await CL.cloudReports(Q('cloud=0'), { page: 'replay' }), await CL.cloudReports(Q('cloudstats=0'), { page: 'tape' })];
        fresh({ ...TOLD, 'twopointsix.cloud.share': '0' }); offs.push(await CL.cloudReports(Q(''), { page: 'replay' })); RP5.reportError('replay.viewer.boot', 'said on a browser that does not send'); await wait(200); const quiet1 = calls.length - n1;
        // a browser that has read it: nothing is asked until a report comes; then the page's own visit record and the report, under a name made for the page
        fresh(TOLD); const n2 = calls.length, on = await CL.cloudReports(Q(''), { page: 'replay', uncaught: true, build: () => 'abc1234' }), twice = await CL.cloudReports(Q(''), { page: 'replay' }); await wait(200); const quiet2 = calls.length - n2;
        RP5.reportWarning('replay.viewer.shelf.read', new Error('the shelf did not open'), { rows: 3 }); RP5.reportWarning('replay.viewer.shelf.read', new Error('the shelf did not open'), { rows: 3 });
        RP5.reportError('replay.viewer.send', 'the sending gave up');
        fire('error', { message: 'thrown on the page', error: new Error('thrown on the page') }); fire('error', { message: 'ResizeObserver loop limit exceeded' }); fire('unhandledrejection', { reason: new Error('a promise nobody caught') });
        await wait(400);
        const pidP = (globalThis as any).localStorage.getItem('twopointsix.cloud.pid'), mine = async () => (await ADMIN.query('telemetry:list', { limit: 2000 })).filter((r: any) => r.pid === pidP);
        const rows = await mine(), vis = rows.filter((r: any) => r.kind === 'visit'), errs = rows.filter((r: any) => r.kind === 'error'), wheres = errs.map((r: any) => r.data.where).sort();
        // the page died: its fatal record goes under the same name
        const Fp = CL.cloudFatal(Q(''), { message: 'the viewer died', stage: 'boot' })!, wentP = !Fp.waits && await Fp.sent, rows2 = await mine(), dead = rows2.filter((r: any) => r.kind === 'fatal');
        // another tab says no: nothing more from here
        (globalThis as any).localStorage.setItem('twopointsix.cloud.share', '0'); fire('storage', { key: 'twopointsix.cloud.share' }); const n3 = calls.length;
        RP5.reportError('replay.viewer.notes.write', 'said after the switch went off'); await wait(200); const quiet3 = calls.length - n3;
        const G5 = ER5.errorGroups(rows2), M5 = ER5.machineLines(rows2), t5 = ER5.errorText(G5), m5 = ER5.machineText(M5);
        check(untold === false && quiet0 === 0 && heldU === 1 && offs.every((x: boolean) => x === false) && quiet1 === 0,
          `page reports: a browser that has not read the notice sends nothing from a page that cannot show it (${quiet0} requests, ${heldU} report kept in the page); ?cloud=0, ?cloudstats=0 and a browser that said no send none (${quiet1} requests)`);
        check(on === true && twice === true && quiet2 === 0 && vis.length === 1 && vis[0].data.page === 'replay' && vis[0].data.started === false && vis[0].sid.startsWith('page-') && vis[0].build === 'abc1234' && errs.every((r: any) => r.sid === vis[0].sid)
          && same(wheres, ['replay.viewer.send', 'replay.viewer.shelf.read', 'unhandledrejection', 'window.error']) && errs.find((r: any) => r.data.where === 'replay.viewer.shelf.read').data.level === 'warning' && errs.find((r: any) => r.data.where === 'replay.viewer.shelf.read').data.data.rows === 3,
          `page reports: a page with nothing to say asks nothing (${quiet2} requests); its first report writes one visit record marked '${vis[0]?.data.page}' under a name of its own and each report once (${wheres.join(' · ')}), an error nothing caught among them`);
        check(wentP === true && dead.length === 1 && dead[0].sid === vis[0]?.sid && dead[0].data.message === 'the viewer died' && quiet3 === 0,
          `page reports: the page's death is a fatal record under the same name (${dead[0]?.data.message}); once another tab says no, nothing more goes (${quiet3} requests)`);
        check(G5.every((g: any) => same(g.pages, ['replay']) && g.visits === 0 && g.pageLoads === 1) && /1 time · 0 visits · 1 other page · 1 browser/.test(t5) && /\n  on a page that is not a visit: replay/.test(t5) && /GPUs: no adapter asked/.test(t5) && M5.length === 1 && M5[0].visits === 0 && M5[0].pages === 1 && /0 visits · 1 other page that reported/.test(m5),
          `page reports: the reader says which page a report came from and does not count the page as a visit (${m5.split('\n')[1]?.trim().slice(0, 60)})`);
        // THE TAB REMEMBERS ITS VISIT'S OFF WORD: the viewer's address has none of its own. A tape's page does not take the mark off; the next visit does
        const ss = memStore(); G5s.sessionStorage = ss;
        fresh(TOLD); await CL.cloudAdopt(Q('cloud=0')); const afterOff = await CL.cloudReports(Q(''), { page: 'replay' });
        fresh(TOLD); await CL.cloudAdopt(Q('cloudstats=0')); await CL.cloudAdopt(Q('playtape=3')); const afterTape = await CL.cloudReports(Q('playtape=3'), { page: 'tape' });
        fresh(TOLD); await CL.cloudAdopt(Q('')); const afterVisit = await CL.cloudReports(Q(''), { page: 'replay' });
        check(afterOff === false && afterTape === false && afterVisit === true,
          `page reports: a page the tab reaches from a visit under ?cloud=0 or ?cloudstats=0 sends nothing (the viewer ${afterOff ? 'sends' : 'does not'}, a tape's page ${afterTape ? 'sends' : 'does not'}); after the next plain visit in the tab it does again`);
      } finally { untap(); console.error = realE; console.warn = realW; CL.PAGE_REPORTS.secs = secs; (globalThis as any).localStorage = realStore; if (hadSS === undefined) delete G5s.sessionStorage; else G5s.sessionStorage = hadSS; CL.cloudForget(); RP5.reportForget(); } }
    toStandIn = false; (globalThis as any).localStorage = memStore(); }
  // …and one pointed at the stand-in sends: hello, the visit's record, the settings
  (globalThis as any).localStorage = memStore({ 'twopointsix.options': '{"hints":false}' });
  const C1 = CL.cloudBoot(Q(`cloud=${S.url}&cloudtapes=0`), deps);
  await new Promise(r => setTimeout(r, 300)); await C1?.sync();
  const got = await W.query('prefs:get', { pid: C1?.pid ?? PID });
  check(!!C1 && /^[a-z0-9]{24,64}$/.test(C1.pid) && !!got && same(JSON.parse(got.data), { 'twopointsix.options': '{"hints":false}' }) && C1.tapes !== null && C1.tapes.stats.rounds === 0, `off: pointed at this machine's backend it sends — the settings are there under an id of ${C1?.pid.length} characters — and ?cloudtapes=0 holds the tapes alone`);
  // WHICH VISITS MAY GO UP: the page lists a visit while it has sent its tapes since its boot, and never lists one it did not see boot sending
  { (globalThis as any).localStorage = memStore();
    const list = (): string[] => JSON.parse((globalThis as any).localStorage.getItem('twopointsix.cloud.visits') ?? '[]');
    const Ca = CL.cloudBoot(Q(`cloud=${S.url}`), { ...deps, sid: 'visit-a' })!, a1 = list(), Ha = Ca.hook() as { share: boolean };
    Ha.share = false; const a2 = list(), off1 = Ha.share; Ha.share = true; const a3 = list();
    CL.cloudBoot(Q(`cloud=${S.url}&cloudtapes=0`), { ...deps, sid: 'visit-b' }); const b1 = list();
    const Cc = CL.cloudBoot(Q('cloud=0'), { ...deps, sid: 'visit-c' }), c1 = list();
    (globalThis as any).localStorage.setItem('twopointsix.cloud.share', '0'); const Cd = CL.cloudBoot(Q(''), { ...deps, sid: 'visit-d' }), d1 = list();
    CL.cloudBoot(Q(`cloud=${S.url}`), { ...deps, sid: 'visit-e' }); const e1 = list();
    check(same(a1, ['visit-a']) && off1 === false && same(a2, []) && same(a3, []) && same(b1, []) && Cc === null && same(c1, []) && !!Cd && same(d1, []) && same(e1, ['visit-e']),
      `off: a visit is on the list of tapes that may go while it has sent since its boot (${a1.join()}); turned off it is off the list for good, also once turned on again; a visit under ?cloudtapes=0, under ?cloud=0 or in a browser that said no is never on it (${e1.join()} after five visits)`); }
  // (the boots above keep their timers: the build's own made-up address stays unreachable for the rest of the run)
  (globalThis as any).fetch = (u: string, ...r: any[]) => String(u).startsWith('https://built-in.example') ? Promise.reject(new Error('no such host')) : (realFetch as any)(u, ...r); delete (globalThis as any).__TPS_CLOUD__; delete (globalThis as any).localStorage;
}

// ---- backend
{ const u = new Uint8Array(1000); for (let i = 0; i < u.length; i++) u[i] = (i * 31 + 7) & 255;
  check(same([...fromBase64(toBase64(u))], [...u]) && toBase64(new Uint8Array([1])) === 'AQ==' && toBase64(new Uint8Array([1, 2])) === 'AQI=', 'backend: bytes ride the wire as base64 and come back the same');
  const h = await W.mutation('prefs:hello', { pid: PID }), p1 = await W.mutation('prefs:put', { pid: PID, data: JSON.stringify({ 'twopointsix.options': 'a' }) }), g = await W.query('prefs:get', { pid: PID });
  check(h.rev === 0 && p1.rev === 1 && g.rev === 1 && g.data === '{"twopointsix.options":"a"}' && (await W.query('prefs:get', { pid: PID2 })) === null, 'backend: settings saved under an id read back under it, counted, and under no other');
  const r1 = await refused(W.mutation('prefs:put', { pid: 'short', data: '{}' })), r2 = await refused(W.mutation('prefs:put', { pid: PID, data: '[1]' })), r3 = await refused(W.mutation('prefs:put', { pid: PID, data: JSON.stringify({ k: 'x'.repeat(600 * 1024) }) }));
  check(/not an id/.test(r1) && /not a map/.test(r2) && /longer than/.test(r3), `backend: an id that is not one, settings that are not a map of texts and settings past the bound are refused (${r1} · ${r2} · ${r3.slice(0, 40)})`);
  const row = { startedAt: '2026-10-26T12:00:00Z', lastAt: '2026-10-26T12:00:05Z', scenario: 's', url: '?x', seconds: 5, pieces: 2, bytes: 10 };
  const body = (s: string) => new TextEncoder().encode(s), part = (seq: number, s: string, at = row.lastAt, p = 0, n = 1) => ({ seq, part: p, parts: n, at, body: body(s) });
  const a = await W.mutation('tapes:put', { pid: PIDB, sid: 'v1', row, chunks: [part(-1, 'head-a'), part(0, 'zero'), part(1, 'one')] });
  const b = await W.mutation('tapes:put', { pid: PIDB, sid: 'v1', row, chunks: [part(0, 'ZERO-AGAIN'), part(1, 'one')] });
  const c = await ADMIN.query('tapes:chunks', { sid: 'v1', seq: -1 }), text = (x: any) => new TextDecoder().decode(x.body);
  check(a.have === 2 && b.have === 2 && c.chunks.length === 3 && c.next === null && same(c.chunks.map(text), ['head-a', 'zero', 'one']), 'backend: a piece sent twice is kept once, as it first came');
  await W.mutation('tapes:put', { pid: PIDB, sid: 'v1', row, chunks: [part(-1, 'head-old', '2026-10-26T11:00:00Z')] });
  await W.mutation('tapes:put', { pid: PIDB, sid: 'v1', row, chunks: [part(-1, 'head-', '2026-10-26T12:00:09Z', 0, 2), part(-1, 'new', '2026-10-26T12:00:09Z', 1, 2)] });
  // (an older head of MORE parts: none of them is written, its tail included)
  await W.mutation('tapes:put', { pid: PIDB, sid: 'v1', row, chunks: [0, 1, 2].map(i => part(-1, `stale${i}`, '2026-10-26T11:30:00Z', i, 3)) });
  const c2 = await ADMIN.query('tapes:chunks', { sid: 'v1', seq: -1 });
  check(same(c2.chunks.map(text), ['head-', 'new', 'zero', 'one']) && c2.next === null, 'backend: the head is the last one sent — an older one does not take its place, not even with its tail, a newer one does, part by part');
  // a piece of several parts counts once its last part is here; a long tape reads back a bounded page at a time, a page's end inside a piece
  { const h1 = await W.mutation('tapes:put', { pid: PIDB, sid: 'v3', row, chunks: [part(0, 'a', row.lastAt, 0, 3), part(0, 'b', row.lastAt, 1, 3)] }), h2 = await W.mutation('tapes:put', { pid: PIDB, sid: 'v3', row, chunks: [part(0, 'c', row.lastAt, 2, 3), ...[1, 2, 3].flatMap(s => [0, 1, 2].map(i => part(s, `${s}.${i}`, row.lastAt, i, 3)))] });
    const pages: string[][] = []; for (let from: any = { seq: -1, part: 0 }; from;) { const r = await ADMIN.query('tapes:chunks', { sid: 'v3', ...from }); pages.push(r.chunks.map(text)); from = r.next; }
    check(h1.have === 0 && h2.have === 4 && pages.length === 2 && pages[0].length === 8 && same(pages.flat(), ['a', 'b', 'c', '1.0', '1.1', '1.2', '2.0', '2.1', '2.2', '3.0', '3.1', '3.2']), `backend: a piece counts once its last part is here, and a tape reads back out a bounded page at a time (${pages.map(p => p.length).join(' + ')} chunks)`); }
  const r4 = await refused(W.mutation('tapes:put', { pid: PID2, sid: 'v1', row, chunks: [part(2, 'theirs')] })), hv = await W.query('tapes:have', { pid: PID2, sids: ['v1', 'none'] }), hv1 = await W.query('tapes:have', { pid: PIDB, sids: ['v1'] });
  check(/another browser/.test(r4) && same(hv, {}) && hv1.v1 === 2, 'backend: another browser cannot write into a visit, nor learn what is kept of it');
  const r5 = await refused(W.mutation('tapes:put', { pid: PIDB, sid: 'v2', row, chunks: [{ seq: 0, part: 0, parts: 1, at: 'x', body: new Uint8Array(800 * 1024) }] })), r6 = await refused(W.mutation('tapes:put', { pid: PIDB, sid: 'bad sid!', row, chunks: [] })), r7 = await refused(W.mutation('tapes:put', { pid: PIDB, sid: 'v2', row: { ...row, pieces: 1.5 }, chunks: [] }));
  check(/larger than/.test(r5) && /not an id/.test(r6) && /whole number/.test(r7), 'backend: a chunk past one document, a visit name that is not one and a count that is not one are refused');
  const r8 = await refused(W.query('tapes:list', {})), r9 = await refused(W.query('tapes:chunks', { sid: 'v1', seq: -1 })), r10 = await refused(W.query('telemetry:list', {})), r11 = await refused(W.query('prefs:players', {}));
  const L = await ADMIN.query('tapes:list', {}), P = await ADMIN.query('prefs:players', {});
  check([r8, r9, r10, r11].every(r => /Could not find public function/.test(r)) && L.length === 3 && L.some((x: any) => x.sid === 'visit-first') && L.some((x: any) => x.sid === 'v1' && x.pid === PIDB) && P.some((x: any) => x.pid === PID), 'backend: the reads back out answer to the deploy key and to nobody else');
  const t = await W.mutation('telemetry:put', { pid: PIDB, sid: 'v1', build: 'abc', events: [{ at: 5, kind: 'visit', data: { a: 1 } }, { at: 6, kind: 'run', scenario: 's', data: null }] }), tl = await ADMIN.query('telemetry:list', { sid: 'v1' });
  const r12 = await refused(W.mutation('telemetry:put', { pid: PIDB, sid: 'v1', events: [{ at: 1, kind: 'x', data: { big: 'y'.repeat(20000) } }] }));
  const r13 = await refused(W.mutation('telemetry:put', { pid: PIDB, sid: 'v1', events: [{ at: 1, kind: 'x', data: { blob: new Uint8Array(300000) } }] })), r14 = await refused(W.mutation('telemetry:put', { pid: PID2, sid: 'v1', events: [{ at: 1, kind: 'x', data: null }] }));
  check(t.kept === 2 && tl.length === 2 && tl[0].kind === 'run' && tl[0].scenario === 's' && tl[1].data.a === 1 && tl[1].build === 'abc' && /larger than/.test(r12) && /not plain JSON/.test(r13) && /another browser/.test(r14), 'backend: records are kept as sent and read back newest first; one past the bound, one carrying bytes and one under another browser\'s visit are refused');
}

// ---- tapes
{ const notes = memoryNotes(); let sync: any = null, midCalls = 0;
  const R = await run(42, { sid: 'visit-a', mid: async (R, f) => { sync ??= new TapeSync(W, R.store, { pid: PID, notes }); if ((f + 1) % 600 === 0) { await sync.round(); midCalls = sync.stats.calls; } } });
  await sync.round();
  const calls = sync.stats.calls; await sync.round(); await sync.round();
  check(midCalls > 0 && sync.stats.pieces === 9 && sync.stats.failed === 0 && sync.stats.calls === calls, `tapes: 42 s of play went up as it was played — ${sync.stats.pieces} pieces in ${calls} calls, ${(sync.stats.bytes / 1024).toFixed(0)} KB — and two more rounds sent nothing`);
  const D = await pullDump(ADMIN, { pid: PID, since: '2026-10-26T12:00:00.000Z' }), got = D ? linesOf(D.bytes) : { lines: [], bad: 1 };
  const V = KP.visitsOf(got.lines).get('visit-a'), A = V ? KP.assemble(V) : null;
  const local = await KS.readVisit(R.store, 'visit-a'), B = local ? KP.assemble(local) : null;
  check(!!D && got.bad === 0 && got.lines[0]?.k === 'dump' && !!A && A.pieces === 9 && !A.missing.length, `tapes: read back out of the backend it is one hand-over file of ${got.lines.length} records, nine pieces, none missing (${D?.name})`);
  check(!!A && !!B && same(A.file, B.file) && F.checkReplay(A.file) === '', 'tapes: the replay file put together from the backend is the one this browser\'s own store makes, and passes the replay\'s own check');
  if (A) { const Rr = await rerun(A.file, { toN: 2400 });
    check(A.tapeTicks === 42 * 60 && !Rr.refuse && !!Rr.report && !Rr.report.drift && !!Rr.W && hashDigest(digestGame(Rr.W.game, true)) === R.hashes[39], `tapes: a fresh game walks the tape from the backend to step 2400 and digests as the first run did there${Rr.refuse ? ` — REFUSED: ${Rr.refuse}` : ''}`); }
  const rowB = (await ADMIN.query('tapes:list', { pid: PID })).find((r: any) => r.sid === 'visit-a'), rowL = (await R.store.rows())[0];
  check(!!rowB && rowB.have === 9 && rowB.pieces === rowL.pieces && rowB.seconds === rowL.seconds && rowB.scenario === rowL.scenario && rowB.commit === 'abc1234', `tapes: the visit's row on the backend is the store's own (${rowB?.seconds} s, ${rowB?.scenario})`);
  // the notes lost: the backend is asked, nothing goes twice
  const again = new TapeSync(W, R.store, { pid: PID, notes: memoryNotes() }); await again.round();
  check(again.stats.pieces === 0 && again.stats.failed === 0, `tapes: a browser that lost its notes asks the backend what it has and sends no piece twice (${again.stats.pieces} pieces)`);
  // a call that fails loses nothing: the wire is down for the first rounds, then up
  { let down = true; const flaky = wire(S.url, { fetch: ((u: any, i: any) => down ? Promise.reject(new Error('offline')) : fetch(u, i)) as any });
    const S2 = new TapeSync(flaky, R.store, { pid: PID2, notes: memoryNotes() }); await S2.round(); await S2.round(); const failed = S2.stats.failed; down = false; await S2.round();
    // (the same store under another browser's id is another browser's visit of the same name: the backend refuses it, which is the refusal path — sent as nothing, held up nothing)
    check(failed >= 2 && S2.stats.pieces === 9, `tapes: with the backend unreachable a round fails and holds its place (${failed} failed); a call the backend refuses is not asked for ever (${S2.stats.lastError.slice(0, 60)})`); }
  // …and one that fails mid-visit picks up where it stopped
  { const R2 = await run(12, { sid: 'visit-b' }); let n = 0;
    const flaky = wire(S.url, { fetch: ((u: any, i: any) => (/mutation/.test(String(u)) && ++n === 1) ? Promise.reject(new Error('dropped')) : fetch(u, i)) as any });
    const was = TAPESYNC.parts; TAPESYNC.parts = 2;
    const S3 = new TapeSync(flaky, R2.store, { pid: PID, notes: memoryNotes() }); await S3.round(); const first = S3.stats.pieces; await S3.round(); TAPESYNC.parts = was;
    const D2 = await pullDump(ADMIN, { pid: PID }), V2 = D2 ? KP.visitsOf(linesOf(D2.bytes).lines).get('visit-b') : null, A2 = V2 ? KP.assemble(V2) : null, L2 = await KS.readVisit(R2.store, 'visit-b');
    check(first === 0 && S3.stats.failed === 1 && !!A2 && !A2.missing.length && same(A2.file, KP.assemble(L2!)!.file), `tapes: a call dropped mid-visit is sent again next round and the tape is whole (${S3.stats.pieces} pieces after ${S3.stats.calls} calls)`); }
  // a piece past one document goes in parts; a browser that cannot pack sends lines that still read
  { const R3 = await run(12, { sid: 'visit-c', gz: null }); const was = TAPESYNC.chunk; TAPESYNC.chunk = 3000;
    const S4 = new TapeSync(W, R3.store, { pid: PID, notes: memoryNotes(), gz: null }); await S4.round(); TAPESYNC.chunk = was;
    const D3 = await pullDump(ADMIN, { pid: PID }), V3 = D3 ? KP.visitsOf(linesOf(D3.bytes).lines).get('visit-c') : null, A3 = V3 ? KP.assemble(V3) : null, L3 = await KS.readVisit(R3.store, 'visit-c');
    check(S4.stats.parts > S4.stats.pieces * 2 && !!A3 && !A3.missing.length && same(A3.file, KP.assemble(L3!)!.file) && D3!.visits.length === 3, `tapes: pieces cut into parts (${S4.stats.parts} parts for ${S4.stats.pieces} pieces) and sent unpacked read back whole, beside the other visits (${D3?.visits.length} in the file)`); }
  // A ROUND ASKED FOR WHILE ONE RUNS is one more round after it: the running one read the store before a run's end was filed (ui/replaykey.ts onEndKept)
  { const M = KS.memoryStore(), rowOf = (pieces: number, lastAt: string) => ({ sid: 'ends', startedAt: '2026-10-26T13:00:00.000Z', lastAt, scenario: 'x', url: '?x', seconds: 5 * pieces, pieces, bytes: 10 * pieces });
    await M.put(rowOf(1, '2026-10-26T13:00:05.000Z'), 0, 'zero', 'head'); await M.put(rowOf(2, '2026-10-26T13:00:10.000Z'), 1, 'the end');
    let n = 0, open!: () => void; const gate = new Promise<void>(r => { open = r; });
    const late = { rows: async () => { if (++n === 1) { await gate; return []; } return M.rows(); }, head: (s: string) => M.head(s), piecesFrom: (s: string, f: number) => M.piecesFrom(s, f) } as any;   // (the first round finds a store with nothing in it yet, and is held there)
    const S5 = new TapeSync(W, late, { pid: PID, notes: memoryNotes(), gz: null });
    const a = S5.round(), b = S5.round(), c = S5.round(); open(); await a; const afterA = S5.stats.pieces, hvA = (await W.query('tapes:have', { pid: PID, sids: ['ends'] })).ends ?? 0; await c; const rounds = S5.stats.rounds;
    const hv = (await W.query('tapes:have', { pid: PID, sids: ['ends'] })).ends; await S5.round();
    check(a !== b && b === c && afterA === 0 && hvA === 0 && hv === 2 && rounds === 2 && S5.stats.rounds === 3 && S5.stats.failed === 0, `tapes: a round asked for while one runs is one more round after it, however many ask (3 asks, ${rounds} rounds) — the running one had read the store empty and sent ${afterA} pieces, the one after it left the backend with ${hv}; asked with none running it is one round (${S5.stats.rounds - rounds})`); }
  // THE NOTE IS THE NEXT PIECE NUMBER, and a visit that has not moved is not read: a store with a hole (a piece it refused), and a visit begun again under its
  // name with this browser's notes lost (the backend has 0..3, the store holds 4..6)
  { const M = KS.memoryStore(), reads: number[] = [], spyStore = { rows: () => M.rows(), head: (s: string) => M.head(s), piecesFrom: (s: string, f: number) => { reads.push(f); return M.piecesFrom(s, f); } };
    const rowOf = (sid: string, pieces: number, lastAt: string) => ({ sid, startedAt: '2026-10-26T13:00:00.000Z', lastAt, scenario: 'x', url: '?x', seconds: 5 * pieces, pieces, bytes: 10 * pieces });
    const seqs = async (sid: string) => { const out: number[] = []; for (let from: any = { seq: -1, part: 0 }; from;) { const r = await ADMIN.query('tapes:chunks', { sid, ...from }); out.push(...r.chunks.map((c: any) => c.seq)); from = r.next; } return out; };
    await M.put(rowOf('hole', 1, '2026-10-26T13:00:05.000Z'), 0, 'zero', 'head'); await M.put(rowOf('hole', 3, '2026-10-26T13:00:15.000Z'), 2, 'two');
    const N = memoryNotes(), SH = new TapeSync(W, spyStore, { pid: PID, notes: N, gz: null }); await SH.round(); const first = reads.length; for (let i = 0; i < 5; i++) await SH.round(); const idle = reads.length - first;
    await M.put(rowOf('hole', 4, '2026-10-26T13:00:20.000Z'), 3, 'three'); await SH.round(); const hv = await W.query('tapes:have', { pid: PID, sids: ['hole'] });
    check(same(await seqs('hole'), [-1, 0, 2, 3]) && first === 1 && idle === 0 && reads[reads.length - 1] === 3 && hv.hole === 4 && N.get().hole.n === 4, `tapes: a visit with a hole in its numbers goes up as it is (${(await seqs('hole')).join(' ')}), is not read again while its row does not move (${idle} reads in 5 rounds), and the next piece is asked for by its number (from ${reads[reads.length - 1]})`);
    // (a piece filed under a moment a round already saw — the keeper puts the row as it stands, so two pieces can share one — still goes up)
    await M.put(rowOf('hole', 5, '2026-10-26T13:00:20.000Z'), 4, 'four'); await SH.round(); const late = await seqs('hole');
    check(same(late, [-1, 0, 2, 3, 4]) && N.get().hole.n === 5, `tapes: a piece filed under a moment the last round already saw is sent all the same (${late.join(' ')})`);
    for (let q = 0; q < 4; q++) await W.mutation('tapes:put', { pid: PID, sid: 'again', row: (({ sid: _s, ...r }) => r)(rowOf('again', 4, '2026-10-26T13:00:20.000Z')), chunks: [{ seq: q, part: 0, parts: 1, at: 'a', body: new TextEncoder().encode(`old${q}`) }] });
    for (let q = 4; q < 7; q++) await M.put(rowOf('again', q + 1, `2026-10-26T13:01:0${q}.000Z`), q, `new${q}`, 'head-again');   // (the row's count is the last number plus one: it goes on through a visit begun again)
    const SA = new TapeSync(W, spyStore, { pid: PID, notes: memoryNotes(), gz: null, only: (sid: string) => sid === 'again' }); await SA.round();
    check(same(await seqs('again'), [-1, 0, 1, 2, 3, 4, 5, 6]) && SA.stats.pieces === 3, `tapes: a visit begun again under its name, in a browser that lost its notes, sends the pieces the backend has not (${SA.stats.pieces} of them: the backend had four, the store held 4 to 6)`); }
  // only the visits the page lists go up (those played start to end while the browser sent): one that is not listed is never asked about, read or sent
  { const rows = await R.store.rows(), one = rows[0].sid, asked: string[] = [];
    const spy = { ...W, query: (p: string, a: any, o?: any) => { if (p === 'tapes:have') asked.push(...a.sids); return W.query(p, a, o); } } as typeof W;
    const S5 = new TapeSync(spy, R.store, { pid: PID, notes: memoryNotes(), only: (sid: string) => sid !== one }); await S5.round(); const unlisted = asked.length;
    const S6 = new TapeSync(spy, R.store, { pid: PID, notes: memoryNotes(), may: () => false }); await S6.round(); const quiet = asked.length;
    const S7 = new TapeSync(spy, R.store, { pid: PID, notes: memoryNotes(), only: (sid: string) => sid === one }); await S7.round();
    check(unlisted === 0 && S5.stats.calls === 0 && S5.stats.rounds === 1 && quiet === 0 && S6.stats.rounds === 0 && asked.includes(one), `tapes: a visit that is not on the page's list is not asked about or sent (${unlisted} asked, ${S5.stats.calls} calls) where the same store with it listed is, and a browser that does not send does not read its store at all`); }
}

// ---- identity
{ const plain = await run(20, { sid: 'visit-plain' });
  let sync: any = null;
  const sent = await run(20, { sid: 'visit-sent', mid: async (R, f) => { sync ??= new TapeSync(W, R.store, { pid: PID, notes: memoryNotes() }); if ((f + 1) % 300 === 0) await sync.round(); } });
  check(sync.stats.pieces > 0 && same(plain.hashes, sent.hashes), `identity: 20 s with the tape going up every 5 s (${sync.stats.pieces} pieces) digest, second by second, as 20 s with no backend`);
}

// the switch turned off in the MIDDLE of a round: no further call of that round goes, and the rest goes once it is on again
{ const R5 = await run(12, { sid: 'visit-stop' }), was = TAPESYNC.call; TAPESYNC.call = 4096;
  let on = true; const spy = { ...W, mutation: async (p: string, a: any, o?: any) => { const r = await W.mutation(p, a, o); if (p === 'tapes:put') on = false; return r; } } as typeof W;
  const S9 = new TapeSync(spy, R5.store, { pid: PIDB, notes: memoryNotes(), may: () => on }); await S9.round();
  const one = S9.stats.calls; on = true; const S10 = new TapeSync(W, R5.store, { pid: PIDB, notes: memoryNotes() }); await S10.round(); TAPESYNC.call = was;
  const row = (await ADMIN.query('tapes:list', { pid: PIDB })).find((r: any) => r.sid === 'visit-stop'), kept = (await R5.store.rows())[0];
  check(one === 1 && S10.stats.calls >= 2 && !!row && row.have === kept.pieces, `tapes: sending turned off in the middle of a round stops it there (${one} call went, ${S10.stats.calls} more were waiting), and the tape is whole once it is on again`);
}

// ---- telemetry
{ let t = 1000; const passes = new Map([['composite', 9], ['gather', 2], ['restir', 1.5], ['smoke-solve', 0.25]]), asked = { n: 0 };
  const T = new Telemetry(W, { pid: PID, sid: 'visit-t', build: () => 'abc1234', now: () => t++, passes: () => { asked.n++; return passes; }, render: ['composite', 'post'] });
  const facts = { scenario: 'Room clear', stage: 'infiltrate', live: true, stats: { elapsed: 61, shots: 3, kills: 1 }, preset: 'high', scale: 0.8, auto: true, sharp: true, shed: false, res: [1536, 864] as [number, number], out: [1920, 1080] as [number, number] };
  let factsAsked = 0; const F = () => { factsAsked++; return facts; }, Fdone = () => ({ ...facts, stage: 'done' });
  T.add('visit', { machine: { webgpu: 'probe gpu', threads: 8, screen: [1440, 900] }, query: 'roomclear', '$bad': 1, _bad: 2 });
  const frames = TELEMETRY.perfSecs * 60 + 200;
  // (a 60 ms frame every 400; the GPU's reading is new two frames in three: 6 ms, and 20 ms once in 300)
  for (let i = 0; i < frames; i++) T.frame(i % 400 === 399 ? 0.060 : 1 / 60, true, 'infiltrate', i % 3 === 2 ? NaN : i % 300 === 1 ? 20 : 6, F);
  const askedLive = asked.n, factsLive = factsAsked;
  for (let i = 0; i < 300; i++) T.frame(1 / 60, false, 'infiltrate', 6, F);   // (a pause card: not play)
  T.away(); T.frame(5, true, 'infiltrate', 6, F);   // (a tab that slept: the page said it went out of sight)
  T.frame(3, true, 'infiltrate', 6, F); T.frame(1 / 60, true, 'infiltrate', 6, F);   // (a FREEZE with the tab up, and the frame after it)
  T.error(new Error('boom')); T.error(new Error('boom')); T.error('other');
  T.frame(1 / 60, true, 'done', NaN, Fdone);
  await T.flush(); await T.flush(true, Fdone());
  const rows = (await ADMIN.query('telemetry:list', { sid: 'visit-t' })).reverse(), kinds = rows.map((r: any) => r.kind), perf = rows.filter((r: any) => r.kind === 'perf'), runRow = rows.find((r: any) => r.kind === 'run'), end = rows.find((r: any) => r.kind === 'end'), visit = rows.find((r: any) => r.kind === 'visit');
  check(same(kinds, ['visit', 'perf', 'error', 'error', 'run', 'perf', 'end']) && rows.every((r: any) => r.build === 'abc1234' && r.pid === PID), `telemetry: a visit's records read back in order (${kinds.join(' · ')})`);
  const p = perf[0]?.data;
  check(!!p && p.frames > 3500 && p.frames < 3700 && p.over50 === Math.floor(p.frames / 400) && p.over100 === 0 && p.p50 === 17 && p.worst === 60 && p.preset === 'high' && p.scale === 0.8 && perf[0].scenario === 'Room clear', `telemetry: a minute of live play is one record with the numbers that were fed (${p?.frames} frames, median ${p?.p50} ms, ${p?.over50} past 50 ms, worst ${p?.worst} ms)`);
  { const g = p?.gpu, timed = Math.round(p.frames * 2 / 3), looks = Math.floor(timed / TELEMETRY.gpuEvery);
    check(!!g && Math.abs(g.frames - timed) <= 1 && g.p50 === 6 && g.worst === 20 && g.mean > 6 && g.mean < 6.1 && g.looks === looks && askedLive === looks + Math.floor((frames - p.frames) * 2 / 3 / TELEMETRY.gpuEvery) && same(g.passes, [['gather', 2], ['restir', 1.5], ['smoke-solve', 0.25]]) && same(g.render, [['composite', 9]]) && same(p.res, [1536, 864]) && same(p.out, [1920, 1080]),
      `telemetry: the minute's record carries the GPU's time from the numbers it was handed (${g?.frames} readings, median ${g?.p50} ms, worst ${g?.worst} ms, ${g?.passes?.length} compute passes by mean, the ${g?.render?.length} render pass listed apart) and the picture's size`);
    check(factsLive === 1 && asked.n === askedLive, `telemetry: a frame asks for nothing — the game's facts were asked ${factsLive} time in ${frames} frames (the minute's record), the pass times once in ${TELEMETRY.gpuEvery} readings, and not at all on a pause card`);
    const Tn = new Telemetry(W, { pid: PID, sid: 'visit-nogpu', now: () => t++ }); for (let i = 0; i < frames; i++) Tn.frame(1 / 60, true, 'infiltrate', 0, F); await Tn.flush();
    const pn = (await ADMIN.query('telemetry:list', { sid: 'visit-nogpu', kind: 'perf' }))[0]?.data;
    check(!!pn && pn.gpu === null && pn.frames > 3500, 'telemetry: an adapter with no pass timings has frame times and no GPU record');
    // (the percentiles and the table's top: 80 ms one reading in ten, 3 ms the rest)
    const Tc = new Telemetry(W, { pid: PID, sid: 'visit-gpuslow', now: () => t++ }); for (let i = 0; i < frames; i++) Tc.frame(1 / 60, true, 'infiltrate', i % 10 === 0 ? 80 : 3, F); await Tc.flush();
    const gc = (await ADMIN.query('telemetry:list', { sid: 'visit-gpuslow', kind: 'perf' }))[0]?.data.gpu;
    check(!!gc && gc.p50 === 3 && gc.p95 === 50 && gc.p99 === 50 && gc.worst === 80 && Math.abs(gc.mean - 10.7) < 0.05 && gc.looks === 0 && same(gc.passes, []), `telemetry: the GPU's percentiles are read off a table that tops out at 50 ms (median ${gc?.p50}, 95th ${gc?.p95}, worst ${gc?.worst} ms), and with no pass times handed in there are none`);
    // an error met while the browser does not send is not counted: its next coming, once it sends, is a record
    let sends = false; const Te = new Telemetry(W, { pid: PID, sid: 'visit-err', now: () => t++, may: () => sends }); Te.error(new Error('early')); sends = true; Te.error(new Error('early')); await Te.flush();
    const er = await ADMIN.query('telemetry:list', { sid: 'visit-err', kind: 'error' });
    check(er.length === 1 && er[0].data.message === 'early', 'telemetry: an error first met before the browser sends is a record when it comes again'); }
  check(!!runRow && runRow.data.stage === 'done' && runRow.data.stats.shots === 3 && !!end && end.data.frames === frames + 2 && end.data.errors === 3 && p.stalls === 0 && perf[1].data.stalls === 1 && perf[1].data.stall === 3000 && p.sharp === true && p.shed === false && Math.abs(end.data.played - (p.secs + perf[1].data.secs)) < 0.01, `telemetry: the run's end carries its tally, the visit's end what was played (${end?.data.frames} frames, ${end?.data.errors} errors) — a pause card and a slept tab are not play, and a ${perf[1]?.data.stall} ms freeze with the tab up is counted by itself`);
  check(!!visit && visit.data.machine.webgpu === 'probe gpu' && !('$bad' in visit.data) && !('_bad' in visit.data) && same(clean({ a: NaN, b: 'x'.repeat(900), c: () => 1 }), { a: null, b: 'x'.repeat(400) }), 'telemetry: a record is a bag of plain keys, finite numbers and short words');
  const T2 = new Telemetry(W, { pid: PID, sid: 'visit-quiet', may: () => false }); T2.add('visit', {}); for (let i = 0; i < 4000; i++) T2.frame(1 / 60, true, 'infiltrate', 6, F); await T2.flush(true, facts);
  check(T2.stats.calls === 0 && (await ADMIN.query('telemetry:list', { sid: 'visit-quiet' })).length === 0, 'telemetry: a browser that does not send keeps no records');
  // A SQUAD'S MISSION ENDS BY ITS OWN RULE, and the mission's stage word never moves: the page hands `frame` the verdict's end (`endStage`).
  // On a real game: a room clear, every defender down. One run record, with the verdict in brief and the display's rate.
  { const { VERDICT } = await import(`${ROOT}/src/game/verdict.ts`), { killGuard } = await import(`${ROOT}/src/game/combat.ts`), { hostileTo } = await import(`${ROOT}/src/game/teams.ts`);
    const von = VERDICT.on; VERDICT.on = true;
    const l = console.log, wn = console.warn; let G: any, word0 = '', wordEnd = '';
    const Tr = new Telemetry(W, { pid: PID, sid: 'visit-squad', now: () => t++ });
    try {
      console.log = console.warn = () => { };
      const R: any = await standUp({ seed: 7 }); G = R.game;
      { const r = startFromQuery(G, 'roomclear&squad4'); if (r.why) throw new Error(r.why); }
      const over = () => { const V = G.verdict.result; return V && { kind: V.kind, won: V.won, word: V.word, secs: V.secs, lost: V.lost, of: V.of, foesDown: V.foes.down, foesOf: V.foes.of }; };
      const Fg = () => ({ ...facts, stage: G.mission.stage, stats: G.mission.stats, hz: 120, over: over() });
      const tick = (n: number) => { for (let i = 0; i < n; i++) { R.step(1 / 60); Tr.frame(1 / 60, true, endStage(G.mission.stage, G.verdict.result), NaN, Fg); } };
      tick(5); { const p = G.player.char.pos; G.teleportPlayer([p[0] + 3, 0, p[2]]); } tick(5); word0 = G.mission.stage;
      for (const h of G.guards.filter((x: any) => x.team !== 'operative' && hostileTo(x, 'player') && x.state !== 'dead')) killGuard(G, h, [1, 0, 0], false, 'shot', 'player');
      tick(240); wordEnd = G.mission.stage;
    } finally { console.log = l; console.warn = wn; VERDICT.on = von; }
    await Tr.flush();
    const rr = await ADMIN.query('telemetry:list', { sid: 'visit-squad', kind: 'run' }), d = rr[0]?.data;
    check(G.roomClear.end === 'taken' && wordEnd === word0 && wordEnd !== 'done' && rr.length === 1 && d.stage === 'done' && d.over.won === true && d.over.word === 'CLEAR' && d.over.kind === 'roomclear' && d.over.foesDown === d.over.foesOf && d.over.foesOf > 0 && d.hz === 120 && typeof d.stats.shots === 'number',
      `telemetry: a house taken is one run record though the mission's stage word never moved (it stayed '${wordEnd}'): ${rr.length} record, ${d?.stage}, the verdict ${d?.over?.word} with ${d?.over?.foesDown} of ${d?.over?.foesOf} defenders down, the display at ${d?.hz} Hz`); }
  // A LOAD'S RECORD (src/cloud/loadtime.ts): the whole load and its stages from the boot's own tables and the browser's count of what came down
  { const LT = await import(`${ROOT}/src/cloud/loadtime.ts`), ER2 = await import(`${ROOT}/tools/cloud/errors.ts`);
    const sum = (gpuReady?: number) => ({ marks: { script: 310, gpu: 460, engine: 500, level: 590, gltf: 900, game: 1320, pipelines: 9000, hooks: 9010, interactive: 9020, firstSubmit: 9100, ...(gpuReady ? { gpuReady } : {}) }, spans: { beforeScript: 300, gpu: 150, level: 85, gltf: 400, gameCtor: 410, pipelines: 8500, pipelines_gather: 6100, firstFrameCpu: 40, firstFrameGpu: 330, toInteractive: 9020 },
      pipelines: { total: 97, done: 97, failed: 0 }, shaderText: { after: 123456 }, slowest: [{ label: 'gather', ms: 6100 }, { label: 'restir', ms: 900 }, { label: 'post', ms: 300 }, { label: 'fxaa', ms: 10 }], maxRafGap: 2400, maxLongTask: 800 });
    const O = 'https://game.example', perfAt = (now: number, type = 'navigate') => ({ now: () => now, getEntriesByType: (t: string) => t === 'navigation' ? [{ type, responseEnd: 80, transferSize: 2000, decodedBodySize: 5000 }] : t === 'resource' ? [
      { name: `${O}/main.js`, initiatorType: 'script', startTime: 90, responseEnd: 290, duration: 200, transferSize: 0, decodedBodySize: 2100000 },
      { name: `${O}/small.js`, initiatorType: 'script', startTime: 90, responseEnd: 120, duration: 30, transferSize: 900, decodedBodySize: 4000 },
      { name: `${O}/assets/chars.gltf`, initiatorType: 'fetch', startTime: 500, responseEnd: 880, duration: 380, transferSize: 700000, decodedBodySize: 3000000 },
      { name: 'https://backend.example/api/mutation', initiatorType: 'fetch', startTime: 600, responseEnd: 700, duration: 100, transferSize: 300, decodedBodySize: 0 },
      { name: `${O}/levels/late.json`, initiatorType: 'fetch', startTime: 20000, responseEnd: 20100, duration: 100, transferSize: 5000, decodedBodySize: 9000 }] : [] });
    const ss = new Map<string, string>(), tab = { getItem: (k: string) => ss.get(k) ?? null, setItem: (k: string, v: string) => { ss.set(k, v); } }; LT.tabLoadForget();
    const early = LT.bootRecord(sum(), perfAt(9500), false, ['splitrays'], O), noCard = LT.bootRecord({ ...sum(9430), marks: { script: 310 } }, perfAt(9500), true, [], O), late = LT.bootRecord(sum(), perfAt(9020 + LT.LOADTIME.waitMs + 1), false, [], O), going = LT.bootRecord(sum(), perfAt(9500), true, [], O);
    LT.tabLoadForget();
    const b: any = LT.bootRecord(sum(9430), perfAt(9500, 'reload'), false, ['splitrays', 'compactshaders'], O, 'Room clear', tab), bAgain: any = LT.bootRecord(sum(9430), perfAt(9500), false, [], O, '', tab);
    LT.tabLoadForget(); const b2: any = LT.bootRecord(sum(9430), perfAt(9500), false, [], O, 'Room clear', tab); LT.tabLoadForget(); const b3: any = LT.bootRecord(sum(9430), perfAt(9500), false, [], O, '', { getItem: () => { throw new Error('no storage'); }, setItem: () => { } }); LT.tabLoadForget();
    check(early === null && noCard === null && !!late && !('whole' in late) && late.interactive === 9020 && !('shown' in late) && !!going && !('whole' in going) && ER.loadLines([{ sid: 's', pid: 'p', at: 1, kind: 'boot', data: late }])[0]?.toCard === true && !!b && b.whole === 9430 && b.shown === 9430 && b.interactive === 9020 && b.script === 300 && b.device === 150 && b.level === 85 && b.gltf === 400 && b.game === 410 && b.pipelinesMs === 8500 && b.slowest.length === 3 && b.nav === 'reload' && b.marks.gltf === 900 && b.spans.pipelines_gather === 6100 && same(b.rows, ['splitrays', 'compactshaders']) && b.nth === 1 && bAgain.nth === 1 && b.mission === 'Room clear' && b2.nth === 2 && b3.nth === 1 && b3.mission === '',
      `telemetry: a load's record is the whole load (${b?.whole} ms to the first picture, the start card at ${b?.interactive}) with each stage, the boot's two tables whole, the shader rows it started with, the mission it opened on and which load of the tab it is (a page counts once; the next page is ${b2?.nth}); none before the first picture is back unless a minute has passed or the page is going, and such a record says no whole load: the report reads it to its start card only`);
    check(!!b && same(b.net, { html: 80, script: 200, scriptWire: 0, scriptSize: 2100000, files: 3, wire: 702900, size: 5109000, last: 880 }) && same(LT.loadNet({ now: () => 0, getEntriesByType: () => { throw new Error('no'); } }, 1, O), {}),
      `telemetry: what came down for the load is the browser's own count of this page's files up to the load's end (${b?.net?.files} files, ${b?.net?.wire} bytes over the wire, the bundle from the cache), never another site's and never a name; a browser that says nothing gives nothing`);
    // A LOAD STARTED ON THE FIRST FRAME'S SHADERS is not over at its first picture: the mark of its later programs rides on the load's record where it
    // is there by then, and is a small record of its own where it comes after; the record is never held for it
    { const withLater = (g: number, later?: number) => { const x = sum(g); return { ...x, marks: { ...x.marks, ...(later ? { laterReady: later } : {}) } }; };
      const before: any = LT.bootRecord(withLater(9430), perfAt(9500), false, ['firstframe'], O), withIt: any = LT.bootRecord(withLater(9430, 9480), perfAt(9500), false, ['firstframe'], O); LT.tabLoadForget();
      const none = LT.laterRecord(withLater(9430)), came: any = LT.laterRecord(withLater(9430, 31250));
      // through a visit: asked on the records' beat (with what the visit's records already carry), written once, and never again where the load's own record carried it
      const realStore = (globalThis as any).localStorage; (globalThis as any).localStorage = memStore(); CL.cloudForget();
      const Q = (x: string) => new URLSearchParams(x), deps = { store: null, build: () => 'abc1234', machine: () => ({ webgpu: 'probe' }), facts: () => ({ scenario: 'x', stage: 'infiltrate', live: true, stats: {}, preset: 'high', scale: 1, auto: true }) };
      let ans: unknown = null, asks = 0; const Cl = CL.cloudBoot(Q(`cloud=${S.url}`), { ...deps, sid: 'visit-later', boot: () => before, later: (have: readonly string[]) => { asks++; return ans && LT.laterRecord(ans as any, have); } })!;
      await new Promise(r => setTimeout(r, 200)); await Cl.sync(); const asked0 = asks; ans = withLater(9430, 31250); await Cl.sync(); await Cl.sync(); const asked1 = asks;
      CL.cloudForget(); (globalThis as any).localStorage = memStore(); let asks2 = 0; const Cc = CL.cloudBoot(Q(`cloud=${S.url}`), { ...deps, sid: 'visit-later-carried', boot: () => withIt, later: (have: readonly string[]) => { asks2++; return LT.laterRecord(withLater(9430, 31250), have); } })!;
      await new Promise(r => setTimeout(r, 200)); await Cc.sync(); await Cc.sync(); CL.cloudForget(); (globalThis as any).localStorage = realStore;
      const lrows: any[] = []; for (const sid of ['visit-later', 'visit-later-carried']) lrows.push(...await ADMIN.query('telemetry:list', { sid }));
      const laters = lrows.filter(r => r.kind === 'later'), LLl = ER.loadLines(lrows), ltext = ER.loadText(LLl);
      check(!!before && !('later' in before) && before.whole === 9430 && withIt.later === 9480 && none === null && came.later === 31250 && came.after === 22230
        && asked0 >= 1 && asked1 > asked0 && asks2 >= 1 && laters.length === 1 && laters[0].sid === 'visit-later' && laters[0].data.later === 31250 && /the last program built behind the start in at/.test(ltext)
        && /never said they were in/.test(ER.loadText(ER.loadLines([{ sid: 'n', pid: 'p', at: 1, kind: 'boot', data: before }]))) && !/never said/.test(ltext) && !/never said/.test(ER.loadText(ER.loadLines([{ sid: 'n', pid: 'p', at: 1, kind: 'boot', data: { ...before, rows: ['compactshaders'] } }]))),
        `telemetry: a load started on the first frame's shaders writes its record at its first picture (${before?.whole} ms) and says when its later programs were in — on the record where that is known by then (${withIt?.later}), else in one small record of the visit once it is (${laters[0]?.data.later} ms, written once however often it is asked: ${asked1} times here); the report says of such a load with neither that its later programs never said they were in`); }
    // …AND SO FOR WHAT ELSE THE LOAD LEARNS LATE: the moment a mission's START went live (the boot's mark `settled`) and the build of the programs with the
    // cut rule (the page's own late fact). Each goes once: on the load's record where it is known by then, else in a `later` record with what is new
    { const withMarks = (o: Record<string, number>) => { const x = sum(9430); return { ...x, marks: { ...x.marks, ...o } }; };
      const onIt: any = LT.bootRecord(withMarks({ laterReady: 9300, settled: 9410 }), perfAt(9500), false, ['firstframe'], O), before: any = LT.bootRecord(withMarks({}), perfAt(9500), false, ['firstframe'], O); LT.tabLoadForget();
      const cut = { ms: 4100, pipelines: 21, owners: 9 };
      const justCut: any = LT.laterRecord(withMarks({ laterReady: 9300, settled: 9410 }), ['later', 'settled'], { cut }), nothing = LT.laterRecord(withMarks({ laterReady: 9300, settled: 9410 }), ['later', 'settled', 'cut'], { cut }), notYet = LT.laterRecord(withMarks({}), [], { cut: undefined });
      const realStore = (globalThis as any).localStorage; (globalThis as any).localStorage = memStore(); CL.cloudForget();
      const Q = (x: string) => new URLSearchParams(x), deps = { store: null, build: () => 'abc1234', machine: () => ({ webgpu: 'probe' }), facts: () => ({ scenario: 'x', stage: 'infiltrate', live: true, stats: {}, preset: 'high', scale: 1, auto: true }) };
      let marks: Record<string, number> = {}, more: Record<string, unknown> = {};
      const Cs = CL.cloudBoot(Q(`cloud=${S.url}`), { ...deps, sid: 'visit-settled', boot: () => before, later: (have: readonly string[]) => LT.laterRecord(withMarks(marks), have, more) })!;
      await new Promise(r => setTimeout(r, 200)); await Cs.sync();
      marks = { laterReady: 11020 }; await Cs.sync();
      marks = { laterReady: 11020, settled: 12500 }; more = { cut }; await Cs.sync(); await Cs.sync();
      CL.cloudForget(); (globalThis as any).localStorage = realStore;
      const srows: any[] = await ADMIN.query('telemetry:list', { sid: 'visit-settled' }), slaters = srows.filter(r => r.kind === 'later').sort((a, b) => Number('settled' in a.data) - Number('settled' in b.data)), sl = ER.loadLines(srows)[0], stext = ER.loadText(ER.loadLines(srows));
      check(onIt?.later === 9300 && onIt?.settled === 9410 && !('settled' in (before ?? {})) && same(justCut, { cut }) && nothing === null && notYet === null
        && slaters.length === 2 && slaters[0].data.later === 11020 && slaters[0].data.after === 2000 && Object.keys(slaters[0].data).length === 2 && slaters[1].data.settled === 12500 && slaters[1].data.settledAfter === 3480 && slaters[1].data.cut?.ms === 4100 && Object.keys(slaters[1].data).length === 3
        && sl?.later === 11020 && sl?.settled === 12500 && sl?.cutMs === 4100 && /START live at 13 s · the cut programs built in 4\.1 s/.test(stext),
        `telemetry: a load says when its mission's START went live and how long the cut programs took — on the load's record where the mark is there by then (${onIt?.settled} ms), else in the visit's later records, each fact once (${slaters.length} records: ${slaters.map((r: any) => Object.keys(r.data).join('+')).join(' · ')}); the report reads them back (START live at ${sl?.settled} ms, cut programs ${sl?.cutMs} ms)`); }
    // A LEAVE'S CALL THAT DID NOT COME BACK MAY STILL HAVE GOT THERE (the browser finishes it without the page), and the page, which cannot know, sends
    // its records again at the leave's second event: the backend keeps the same record once (same visit, moment, kind and words), and the report reads
    // one that was written twice before that once. A going flush with no facts (a lost device) leaves the open stretch to the leave that has them
    { let clock = 5000, calls = 0, fails = true;
      const lossy = { ...W, mutation: async (path: string, args: any, o?: { keepalive?: boolean }) => { calls++; const r = await W.mutation(path, args, o); if (fails && o?.keepalive) throw new WireError('telemetry:put: the page went', false); return r; } };
      const Td = new Telemetry(lossy as any, { pid: PID, sid: 'visit-twice', now: () => clock++ });
      for (let i = 0; i < 600; i++) Td.frame(1 / 60, true, 'infiltrate', 3, F);
      await Td.flush(true); const noFacts: any[] = await ADMIN.query('telemetry:list', { sid: 'visit-twice' });
      await Td.flush(true, facts); await Td.flush(true, facts); fails = false; await Td.flush();
      const drows: any[] = await ADMIN.query('telemetry:list', { sid: 'visit-twice' }), ends = drows.filter(r => r.kind === 'end').length, perfs = drows.filter(r => r.kind === 'perf').length, twice = [...drows, ...drows.filter(r => r.kind === 'end')];
      const again: any = await W.mutation('telemetry:put', { pid: PID, sid: 'visit-twice', events: drows.slice(0, 2).map(r => ({ at: r.at, kind: r.kind, data: Object.fromEntries(Object.entries(r.data).reverse()) })) });
      check(calls >= 4 && noFacts.filter(r => r.kind === 'perf').length === 0 && noFacts.filter(r => r.kind === 'end').length === 1 && ends === 1 && perfs === 1 && again?.kept === 0
        && ER.once(twice).length === drows.length && ER.once(drows).length === drows.length,
        `telemetry: a leave's records whose call did not come back are sent again by the page (${calls} calls) and kept once by the backend (${ends} end, ${perfs} perf; the same records with their keys in another order: ${again?.kept} kept); a going flush with no facts writes no stretch (${noFacts.filter(r => r.kind === 'perf').length}) and the leave after it does; the report reads a record written twice once (${twice.length} → ${ER.once(twice).length})`); }
    // WHERE THE QUALITY STOOD AND WHEN IT MOVED: the minute's record ends on a preset and a scale, and the ladder moves inside the minute. The page's word
    // for its rung and the ladder's own last line are asked a few times a second on live frames; each change of either is a move with its second of play
    // (marked where it was made while play was held), the first `moves` of them, and the report lays a visit's stretches out in order
    { let word = 'high 0.85 1632x918 sharp', why = 'auto: starting at high — calibrating', asks = 0, throws = false;
      const Tm = new Telemetry(W, { pid: PID, sid: 'visit-moves', now: () => t++, rung: () => { asks++; if (throws) throw new Error('no'); return [word, why]; } });
      const play = (secs: number, live = true) => { for (let i = 0; i < Math.round(secs * 60); i++) Tm.frame(1 / 60, live, 'infiltrate', 3, F); };
      play(3); word = 'medium 0.55 1056x594 shed'; why = 'calibrated (boot): 10.0 ms compute at high → medium ×0.55'; play(6.5);
      throws = true; play(1); throws = false;                                              // (a word that could not be read makes no move)
      play(2, false); word = 'low 0.6 1152x648 sharp'; why = 'preset low'; play(2, false); play(47.5);   // (changed with a menu up: marked, at the second play stood on)
      why = 'adaptive: frame pacing, not load'; play(2.5);                                 // (the stretch closes at a minute of play; the ladder's line alone is a move too)
      for (let i = 0; i < TELEMETRY.moves + 3; i++) { word = `low 0.${i + 10} 1x1`; play(0.5); }
      await Tm.flush(true, facts); const played = 3 + 6.5 + 1 + 47.5 + 2.5 + (TELEMETRY.moves + 3) * 0.5;
      const mrows: any[] = (await ADMIN.query('telemetry:list', { sid: 'visit-moves' })).filter((r: any) => r.kind === 'perf').sort((a: any, b: any) => a.at - b.at), g1 = mrows[0]?.data.rung, g2 = mrows[1]?.data.rung, ML = ER.moveLines(mrows), mt = ER.moveText(ML);
      const Tq = new Telemetry(W, { pid: PID, sid: 'visit-still', now: () => t++, rung: () => ['high 1 1920x1080 pinned', 'preset high'] }); for (let i = 0; i < 600; i++) Tq.frame(1 / 60, true, 'infiltrate', 3, F); await Tq.flush(true, facts);
      const still: any[] = (await ADMIN.query('telemetry:list', { sid: 'visit-still' })).filter((r: any) => r.kind === 'perf');
      const junk = ER.moveLines([{ sid: 'j', pid: 'p', at: 1, kind: 'perf', data: { secs: 5, rung: { from: 'x', moves: [null, 3, [1, 'y'], ['z']] } } }]);
      check(mrows.length === 2 && g1?.from === 'high 0.85 1632x918 sharp' && g1.why === 'auto: starting at high — calibrating' && g1.moves?.length === 3
        && Math.abs(g1.moves[0][0] - 3) < 0.3 && g1.moves[0][1] === 'medium 0.55 1056x594 shed' && /^calibrated \(boot\)/.test(g1.moves[0][2]) && g1.moves[0].length === 3
        && Math.abs(g1.moves[1][0] - 10.5) < 0.1 && g1.moves[1][1] === 'low 0.6 1152x648 sharp' && g1.moves[1][3] === 1 && Math.abs(g1.moves[2][0] - 58) < 0.3 && g1.moves[2][1] === 'low 0.6 1152x648 sharp' && /frame pacing/.test(g1.moves[2][2])
        && g2?.from === 'low 0.6 1152x648 sharp' && g2.moves?.length === TELEMETRY.moves && g2.more === 3 && asks <= played * 4 + 12 && asks > played * 3
        && ML.length === 1 && ML[0].stretches.length === 2 && /stretch 1, Room clear \(60 s of play\): began on high 0\.85 1632x918 sharp  \[auto: starting at high — calibrating\]/.test(mt) && /\n    10\.5 s \(while play was held\) → low 0\.6 1152x648 sharp  \[preset low\]/.test(mt) && /\n    and 3 more/.test(mt)
        && still[0]?.data.rung?.from === 'high 1 1920x1080 pinned' && !('moves' in still[0].data.rung) && ER.moveLines(still).length === 0 && /no visit's quality moved/.test(ER.moveText([]))
        && junk.length === 1 && junk[0].stretches[0].moves.length === 1 && junk[0].stretches[0].moves[0].to === 'y',
        `telemetry: a minute's record says where the quality stood as it began, with the ladder's line, and each move with its second of play and why (${g1?.from} → ${(g1?.moves ?? []).map((m: any) => `${m[0]} s ${m[1]}${m[3] ? ' (held)' : ''}`).join(' → ')}): a change made with play held is marked, a line of the ladder alone is a move, a word that could not be read is none; the first ${TELEMETRY.moves} moves of a stretch and a count of the rest (${g2?.more} more); asked a few times a second, not every frame (${asks} asks in ${Math.round(played * 60)} live frames); the report lays a visit's stretches out in order and reads only entries that are moves`); }
    // THE MAIN THREAD'S TIME ON A FRAME: the loop's own timings (the game's update, the engine's render, the whole from the frame's start, game ticks)
    // are handed in on every played frame and the minute's record carries each as mean / p50 / p95 / worst — what tells a slow script from a slow GPU.
    // The update is read on the frames that ran a tick (a fast display draws frames between ticks)
    { let u = 4, r = 3, w = 9, tk = 1, asks = 0, throws = false, per = NaN;   // (`per`: the ms the frame spent inside the page's timed GPU calls, NaN where the page keeps no clock on them)
      const Tc = new Telemetry(W, { pid: PID, sid: 'visit-cpu', now: () => t++, cpu: (out: Float64Array) => { asks++; if (throws) throw new Error('no'); out[0] = u; out[1] = r; out[2] = w; out[3] = tk; out[4] = per; } });
      const play = (n: number, live = true, dt = 1 / 60) => { for (let i = 0; i < n; i++) Tc.frame(dt, live, 'infiltrate', 3, F); };
      play(150); per = -1; play(150); per = 2; u = 0.1; w = 5; tk = 0; play(400); u = 4; w = 9; tk = 1;   // (a fast display: more frames between ticks than on them. The page's call clock is on from here — before it there was none, then a share below nothing: neither is a reading)
      u = 30; r = 5; w = 41.3; tk = 3; per = 3.5; play(60); r = 3; per = 2;               // (slow frames: three ticks each, the update the long part)
      per = 50; play(30, false); per = 2;                                                // (a menu up: not played, not asked)
      throws = true; play(5); throws = false;                                             // (a frame the page could not time is left out whole)
      u = 2; r = 1; w = 500; tk = 1; play(1, true, 1.5);                                  // (after a freeze, and longer than the table: counted, its bin the last, its worst itself)
      u = 3; r = -1; w = NaN; play(2);                                                    // (no whole = not a played frame)
      u = 90; r = 1; w = 95; tk = 3; Tc.away(); play(1, true, 30); u = 4; r = 3; w = 9; tk = 1;   // (the first frame after the tab was away runs its catch-up ticks: not a played frame)
      await Tc.flush(true, facts);
      const crow: any = (await ADMIN.query('telemetry:list', { sid: 'visit-cpu' })).find((x: any) => x.kind === 'perf'), c = crow?.data.cpu, near = (a: number, b: number) => Math.abs(a - b) < 0.001;   // (a record's numbers go up at three places)
      const plain: any = (await ADMIN.query('telemetry:list', { sid: 'visit-still' })).find((x: any) => x.kind === 'perf');
      const mine = { ...crow, pid: 'pc' }, CM = crow ? ER.machineLines([mine, { ...mine, data: { ...mine.data, cpu: { n: 5, update: { n: 5, p50: 20 }, render: { p50: 6 }, whole: { p50: 31 }, ticks: { mean: 2 } } } }, { ...mine, data: { ...mine.data, cpu: 'junk' } }]) : [], ct = ER.machineText(CM);
      const none = ER.machineText(ER.machineLines(plain ? [{ ...plain, pid: 'pq' }] : []));
      const Tk = new Telemetry(W, { pid: PID, sid: 'visit-calls-small', now: () => t++, cpu: (out: Float64Array) => { out[0] = 1; out[1] = 1; out[2] = 2; out[3] = 1; out[4] = 0.2; } }); for (let i = 0; i < 400; i++) Tk.frame(1 / 60, true, 'infiltrate', 3, F); await Tk.flush(true, facts);
      const small: any = (await ADMIN.query('telemetry:list', { sid: 'visit-calls-small' })).find((x: any) => x.kind === 'perf')?.data.cpu?.calls;
      const Tn = new Telemetry(W, { pid: PID, sid: 'visit-noticks', now: () => t++, cpu: (out: Float64Array) => { out[0] = 7; out[1] = 2; out[2] = 3; out[3] = 0; } }); for (let i = 0; i < 400; i++) Tn.frame(1 / 60, true, 'infiltrate', 3, F); await Tn.flush(true, facts);
      const ntRow: any = (await ADMIN.query('telemetry:list', { sid: 'visit-noticks' })).find((x: any) => x.kind === 'perf'), nt: any = ntRow?.data.cpu, noClock = ER.machineText(ER.machineLines(ntRow ? [{ ...ntRow, pid: 'pn' }] : []));
      check(!!c && c.n === 761 && asks === 768 && crow.data.frames === 767 && crow.data.stalls === 1
        && c.update.n === 361 && c.update.p50 === 4 && c.update.p95 === 30 && c.update.worst === 30 && near(c.update.mean, (300 * 4 + 60 * 30 + 2) / 361)
        && c.render.p50 === 3 && c.render.p95 === 5 && c.render.worst === 5 && near(c.render.mean, (700 * 3 + 60 * 5 + 1) / 761)
        && c.whole.p50 === 5 && c.whole.p95 === 41.5 && c.whole.worst === 500 && near(c.whole.mean, (300 * 9 + 400 * 5 + 60 * 41.3 + 500) / 761)
        && c.ticks.max === 3 && near(c.ticks.mean, (300 + 180 + 1) / 761)
        && c.calls?.n === 461 && c.calls.p50 === 2 && c.calls.p95 === 3.5 && c.calls.worst === 3.5 && near(c.calls.mean, (401 * 2 + 60 * 3.5) / 461) && nt && !('calls' in nt)
        && small?.n === 400 && small.p50 === 0.2 && near(small.mean, 0.2) && /the main thread 3\.0 ms a frame/.test(noClock) && !/GPU's calls/.test(noClock)
        && !!plain && !('cpu' in plain.data)
        && nt?.n === 400 && nt.update.n === 0 && !('p50' in nt.update) && nt.whole.p50 === 3 && nt.ticks.mean === 0
        && CM.length === 1 && CM[0].cpuWhole === 5 && CM[0].cpuUpdate === 4 && CM[0].cpuRender === 3 && /the main thread 5\.0 ms a frame \(render 3\.0 ms; update 4\.0 ms on the frames that ran a game tick, 0\.6 ticks a frame\) · inside the GPU's calls 2\.(19|20) ms a frame on average and 3\.50 ms at the 95th, of a whole frame's 10\.1 ms on average and 41\.5 ms at the 95th \(the 0\.2 min that timed them\)/.test(ct) && !/the main thread/.test(none),
        `telemetry: a minute's record says how long the main thread took on a frame, from the loop's own timings (${c?.n} played frames: whole ${c?.whole.p50} ms median, ${c?.whole.p95} at the 95th, ${c?.whole.worst} worst; render ${c?.render.p50} / ${c?.render.p95}; update ${c?.update.p50} / ${c?.update.p95} on the ${c?.update.n} frames that ran a game tick, so a fast display's frames between ticks do not make it nothing; ${c?.ticks.mean.toFixed(2)} game ticks a frame, ${c?.ticks.max} at the most): asked on played frames only (${asks} asks), a frame after a freeze is counted, one that could not be timed or was held still is left out, a stretch with no tick says so (${nt?.update.n}), a page with no timings writes none; where the page times its GPU calls, the ms a played frame spent inside them (${c?.calls?.p50} ms median, ${c?.calls?.p95} at the 95th, on ${c?.calls?.n} frames: none where the page kept no clock or handed a share below nothing; a fifth of a millisecond reads as itself, ${small?.p50}), and the report sets it beside the whole frame of the same minutes; the report prints a machine's medians`); }
    // THE MACHINE'S POWER: the browser is asked once whether the machine is on its battery, and every minute's record, a run's and the visit's say it
    // as it stands when they are written — a laptop on its battery can run the same build far slower, and no record said which it was
    { let asks = 0; const batt = { charging: false, level: 0.62 }, ask = CL.powerAsker({ getBattery: () => { asks++; return Promise.resolve(batt); } });
      const first = ask(); await Promise.resolve(); await Promise.resolve(); const then = ask(); batt.level = 0.4; const lower = ask(); batt.charging = true; const plugged = ask(); batt.charging = false;
      const noneB = CL.powerAsker({}), refuse = CL.powerAsker({ getBattery: () => Promise.reject(new Error('no')) }), throws = CL.powerAsker({ getBattery: () => { throw new Error('no'); } }); noneB(); refuse(); throws(); await Promise.resolve(); await Promise.resolve();
      const Tp = new Telemetry(W, { pid: PID, sid: 'visit-power', now: () => t++, power: ask });
      for (let i = 0; i < 3700; i++) Tp.frame(1 / 60, true, 'infiltrate', 3, F); batt.charging = true; for (let i = 0; i < 600; i++) Tp.frame(1 / 60, true, i < 599 ? 'infiltrate' : 'done', 3, F); await Tp.flush(true, facts);
      const prow: any[] = (await ADMIN.query('telemetry:list', { sid: 'visit-power' })).sort((a: any, b: any) => a.at - b.at), pp = prow.filter(r => r.kind === 'perf'), pr = prow.find(r => r.kind === 'run');
      // (…and through the page's own wiring: a visit whose stats go asks the browser once and its visit record says the word; one with the stats off never asks)
      const Qp = (x: string) => new URLSearchParams(x), depsP = { store: null, build: () => 'abc1234', machine: () => ({ webgpu: 'probe' }), facts: () => ({ scenario: 'x', stage: 'infiltrate', live: true, stats: {}, preset: 'high', scale: 1, auto: true }) };
      const nav: any = (globalThis as any).navigator, hadB = nav?.getBattery, realLS = (globalThis as any).localStorage; let navAsks = 0; if (nav) nav.getBattery = () => { navAsks++; return Promise.resolve({ charging: false, level: 0.5 }); };
      const visitPower = async (sid: string, q: string) => { CL.cloudForget(); (globalThis as any).localStorage = memStore(); const C = CL.cloudBoot(Qp(q), { ...depsP, sid }); await new Promise(r => setTimeout(r, 200)); await C?.sync(); await C?.sync(); return (await ADMIN.query('telemetry:list', { sid })).find((r: any) => r.kind === 'visit')?.data; };
      const vOn = await visitPower('visit-power-on', `cloud=${S.url}`), asksOn = navAsks, vOff = await visitPower('visit-power-off', `cloud=${S.url}&cloudstats=0`), asksOff = navAsks - asksOn;
      CL.cloudForget(); (globalThis as any).localStorage = realLS; if (nav) { if (hadB === undefined) delete nav.getBattery; else nav.getBattery = hadB; }
      const PL = ER.machineLines(pp.map(r => ({ ...r, pid: 'pw' }))), pt = ER.machineText(PL), allIn = ER.machineText(ER.machineLines([{ ...pp[1], pid: 'pz' }]));
      const still: any = (await ADMIN.query('telemetry:list', { sid: 'visit-still' })).find((x: any) => x.kind === 'perf');
      check(first === '' && then === 'on battery (62 %)' && lower === 'on battery (40 %)' && plugged === 'plugged in' && asks === 1 && noneB() === '' && refuse() === '' && throws() === ''
        && vOn?.power === 'on battery (50 %)' && vOn.machine?.webgpu === 'probe' && asksOn === 1 && !vOff && asksOff === 0
        && pp.length === 2 && pp[0].data.power === 'on battery (40 %)' && pp[1].data.power === 'plugged in' && pr?.data.power === 'plugged in' && !!still && !('power' in still.data)
        && PL.length === 1 && Math.abs(PL[0].batteryMin - 1) < 0.01 && PL[0].batteryLow === 40 && / · on battery for 1\.0 of 1\.2 min \(down to 40 %\)/.test(pt) && / · plugged in$/.test(allIn),
        `telemetry: a minute's record, a run's and the visit's say the machine's power as it stood (${pp.map(r => r.data.power).join(' → ')}; the run: ${pr?.data.power}): the browser is asked once (${asks}) and read as it stands after, a browser with no answer says nothing; a visit's own record says it (${vOn?.power}, ${asksOn} question) and a visit with its stats off never asks (${asksOff}); the report says how much of a machine's play was on battery and how low it read`); }
    // THE KEPT PAGE's words on a load's record (the shaders-once-a-visit work writes them: `kept`, `made`, `shell`, `shellNav`): the report tells the load
    // that made the page from a mission start on it, adds the shell's ms to the first, and says how many programs each compiled
    { const base: any = LT.bootRecord(sum(9430), perfAt(9500), false, ['compactshaders'], O); LT.tabLoadForget();
      const row = (sid: string, data: any) => ({ sid, pid: 'pk', at: 1, kind: 'boot', data });
      const KL = ER.loadLines([row('k1', { ...base, kept: 1, shell: 420, shellNav: 'reload', calls: { n: 310, ms: 1840, mb: 212.4, worst: 620, what: 'writeTexture', by: { writeTexture: [12, 900, 180.2], submit: [40, 700, 0] } }, made: { pipelines: 119, modules: 40, asked: 119 } }), row('k2', { ...base, kept: 2, whole: 1100, shown: 1100, waited: 180, made: { pipelines: 0, modules: 0, asked: 119, labels: [] } }),
        row('k3', { ...base, kept: 3, whole: 1300, shown: 1300, waited: 2900, made: { pipelines: 2, modules: 1, asked: 119, labels: ['gather', 'cutA'] } }), row('p1', base)]), kt = ER.loadText(KL);
      const first = KL.find((l: any) => l.kept === 'first'), start = KL.find((l: any) => l.kept === 'start'), plain = KL.find((l: any) => l.kept === '');
      check(KL.length === 3 && !!first && first.nav === 'reload' && first.whole === 9850 && first.shell === 420 && first.made === 119 && !!start && start.loads === 2 && start.whole === 1100 && start.made === 0 && same(start.labels, ['gather', 'cutA']) && !!plain && plain.whole === 9430 && plain.made === null && KL[2] === start
        && /1 reload that made the kept page · .* whole load 9\.8 s \(the shell page 0\.4 s of it\) · compiled 119 of 119 programs/.test(kt) && /2 mission starts on the kept page \(no page load\) · .* to its first picture 1\.1 s · compiled 0 \(2 at the most\) of 119 programs \(gather, cutA\)/.test(kt)
        && first.callsMs === 1840 && first.callWorst === 620 && start.callsMs === null && first.callsMost === 1840 && /1 reload that made the kept page[^\n]* · inside GPU calls 1\.8 s, the longest single call of any of them 620 ms \(writeTexture\)/.test(kt) && !/2 mission starts[^\n]*inside GPU calls/.test(kt)
        && start.waited === 180 && start.waitedMost === 2900 && first.waited === null && /2 mission starts on the kept page .* · waited 0\.2 s for the GPU to finish the mission before \(2\.9 s at the longest\)/.test(kt) && !/1 reload that made the kept page[^\n]*waited/.test(kt),
        `telemetry: the report reads the kept page's words — the load that made the page with its shell's time in the whole (${first?.whole} ms, ${first?.made} of ${first?.asked} programs compiled), a mission start on it apart (${start?.loads} starts, ${start?.whole} ms, ${start?.made} compiled) with how long it waited for the GPU to finish the mission before (${start?.waited} ms, ${start?.waitedMost} at the longest), a load's time inside its GPU calls where it was timed (${first?.callsMs} ms, the longest ${first?.callWorst}), a whole page as before`); }
    // through the records and out again: the report's loads, by browser, kind of load and shader rows; a record from before reads to the start card
    const Tb = new Telemetry(W, { pid: PID2, sid: 'visit-load', now: () => t++ }); Tb.add('visit', { machine: { webgpu: 'probe gpu', browser: 'Chrome 155', os: 'Windows' }, query: 'compactshaders=1&splitrays=0' }); Tb.add('boot', b); Tb.add('boot', { ...b, whole: 9830, shown: 9830 }); Tb.add('boot', b2); Tb.add('boot', { ...b2, nth: 3 }); Tb.add('boot', { ...b3, mission: '' });
    Tb.add('boot', { interactive: 7000, script: 250, device: 100, pipelinesMs: 6500, nav: 'navigate' }); await Tb.flush();
    const lr: any[] = []; for (const kind of ['visit', 'boot']) lr.push(...await ADMIN.query('telemetry:list', { sid: 'visit-load', kind }));
    const LL = ER2.loadLines(lr), text = ER2.loadText(LL), re = LL.find((l: any) => l.nav === 'reload'), old = LL.find((l: any) => l.toCard);
    const later = LL.find((l: any) => l.nav === 'later'), first = LL.find((l: any) => l.nav === 'first');
    check(LL.length === 4 && !!re && re.loads === 2 && re.rows === 'compactshaders + splitrays' && re.whole === 9430 && re.pipelinesMs === 8500 && re.cached === 2 && re.wire === 702900 && re.missions === 2 && !!old && old.nav === 'navigate' && old.whole === 7000 && old.rows === 'by the address alone: compactshaders' && !!later && later.loads === 2 && later.missions === 2 && later.rows === '' && !!first && first.loads === 1 && first.missions === 0
      && /2 reloads, 2 into a mission · rows compactshaders \+ splitrays · whole load 9\.4 s · page script at 0\.3 s · device 0\.1 s · level 0\.1 s · characters 0\.4 s · game 0\.4 s · shaders 8\.5 s · first picture at 9\.4 s · 703 kB over the wire/.test(text) && /1 first load of a tab · no shader row on · whole load/.test(text) && /2 later loads in a tab, 2 into a mission · no shader row on/.test(text) && /1 load that was not a reload \(first of a tab or a mission start: not recorded\) · rows by the address alone: compactshaders · to the start card 7\.0 s/.test(text),
      `telemetry: the report reads loads by browser, kind of load (a tab's first, a later one such as a mission start, a reload) and shader rows (${LL.length} lines: ${LL.map((l: any) => `${l.loads} ${l.nav}`).join(' · ')}), and a record from before the whole load was in it reads to the start card with the rows its address asked for`); }
  { // the words alone: a verdict over a live word is the end; an end that moves to the other end is the same run; the minute's record carries the rate
    const Tw = new Telemetry(W, { pid: PID, sid: 'visit-ends', now: () => t++ }), Fh = () => ({ ...facts, hz: 144 });
    const say = (stage: string, v: { won: boolean } | null) => Tw.frame(1 / 60, true, endStage(stage, v), NaN, Fh);
    say('drive', null); say('failed', null); say('failed', { won: true }); say('infiltrate', null); say('infiltrate', { won: false }); say('infiltrate', { won: false });
    for (let i = 0; i < TELEMETRY.perfSecs * 60 + 10; i++) say('infiltrate', { won: false });
    await Tw.flush();
    const rw = (await ADMIN.query('telemetry:list', { sid: 'visit-ends' })).reverse(), runs = rw.filter((r: any) => r.kind === 'run').map((r: any) => r.data.stage), pf = rw.find((r: any) => r.kind === 'perf')?.data;
    check(endStage('drive', null) === 'drive' && endStage('exfil', undefined) === 'exfil' && same(runs, ['failed', 'failed']) && pf?.hz === 144 && !('hz' in p),
      `telemetry: a run's end is the verdict's where there is one and the mission's word where there is none; failed and then won on one run is one record (${runs.join(' · ')}), and a minute's record carries the display's rate once it has been seen (${pf?.hz} Hz; none before)`); }
  { // A RUN'S RECORD GOES WHEN THE RUN ENDS, not at the records' next beat: nothing asks for a flush here
    const Tr = new Telemetry(W, { pid: PID, sid: 'visit-runnow', now: () => t++ }); Tr.frame(1 / 60, true, 'infiltrate', NaN, () => facts); Tr.frame(1 / 60, true, 'done', NaN, () => facts);
    let there = false; for (let i = 0; i < 40 && !there; i++) { await new Promise(r => setTimeout(r, 50)); there = (await ADMIN.query('telemetry:list', { sid: 'visit-runnow' })).some((x: any) => x.kind === 'run'); }
    check(there, `telemetry: a run's record goes up when the run ends, with no flush asked (${there ? 'on the backend within two seconds' : 'not there after two seconds'}) — a tab shut a few seconds after a win used to take it along`); }
  // the report (tools/cloud/errors.ts): what died and what threw, grouped, with the machines it happened on
  { const rows: any[] = []; for (const kind of ['fatal', 'error', 'visit', 'boot', 'perf']) rows.push(...await ADMIN.query('telemetry:list', { kind, limit: 2000 }));
    const G = ER.errorGroups(rows), amd = G.find((g: any) => g.message === 'Failed to create WebGPU Context Provider'), boom = G.find((g: any) => g.message === 'boom'), firstError = G.findIndex((g: any) => g.kind === 'error');
    const twice = ER.errorGroups([{ sid: 'a', pid: 'p1', at: 1, kind: 'error', data: { message: 'Buffer 0x7f3a at https://x.example/a.js:10:4 is 123456 long' } }, { sid: 'b', pid: 'p2', at: 2, kind: 'error', data: { message: 'Buffer 0x11ff at https://x.example/a.js:99:1 is 654321 long' } }]);
    const M = ER.machineLines(rows), text = ER.errorText(G);
    check(!!amd && amd.kind === 'fatal' && same(amd.stages, ['adapter']) && same(amd.gpus, ['no WebGPU adapter · ANGLE (AMD, Radeon RX 6600)']) && amd.browsers === 1 && same(amd.builds, ['abc1234']) && amd.stack[1] === 'at initGpu (device.ts:40:9)' && !!boom && boom.kind === 'error' && G.slice(0, firstError).every((g: any) => g.kind === 'fatal') && same(G.slice(firstError).map((g: any) => g.kind), [...G.filter((g: any) => g.kind === 'error').map(() => 'error'), ...G.filter((g: any) => g.kind === 'warning').map(() => 'warning')])
      && twice.length === 1 && twice[0].count === 2 && twice[0].browsers === 2 && /THE PAGE DIED \(adapter\): Failed to create WebGPU Context Provider/.test(text) && M[0].fatals >= 5 && M.some((m: any) => m.frameP50 === 17 && m.preset === 'high'),
      `telemetry: the report groups what died and what threw (${G.filter((g: any) => g.kind === 'fatal').length} dead pages first, then ${G.filter((g: any) => g.kind === 'error').length} errors, then ${G.filter((g: any) => g.kind === 'warning').length} warnings), each with its builds and GPUs; two messages that differ in a number are one; a line a machine carries its frame time`); }
  // a player who switches tabs: the first real visits wrote an `end` at every switch and a record of a tenth of a second of play
  { const T3 = new Telemetry(W, { pid: PID, sid: 'visit-tabs' }), live = (n: number) => { for (let i = 0; i < n; i++) T3.frame(1 / 60, true, 'infiltrate', 6, F); };
    const kindsNow = async () => (await ADMIN.query('telemetry:list', { sid: 'visit-tabs' })).reverse().map((r: any) => r.kind);
    live(600); await T3.flush(true, facts, true); const k1 = await kindsNow();                    // ten seconds, then out of sight
    T3.back(); await T3.flush(true, facts, true); const k2 = await kindsNow();                    // back and out again with nothing played
    T3.back(); live(60); await T3.flush(true, facts, true); const k3 = await kindsNow();          // a second of play, out of sight
    T3.back(); await T3.flush(true, facts, false); const k4 = await kindsNow();                   // the page goes
    check(same(k1, ['perf', 'end']) && same(k2, k1) && same(k3, ['perf', 'end', 'end']) && same(k4, ['perf', 'end', 'end', 'perf']),
      `telemetry: a tab that goes out of sight twice with nothing played writes one end, and a second of play waits for the page to go (${k4.join(' · ')})`); }
}

// ---- prefs
{ const A = memStore({ 'twopointsix.options': '{"vol":{"master":0.4}}', 'twopointsix.keys': '{"grab":"KeyG"}', 'twopointsix.findme': '{"rim":1}', 'twopointsix.quality': 'high', 'twopointsix.autoPreset': 'low', 'twopointsix.textsize': '2',
    'twopointsix.cloud.pid': PID2, 'twopointsix.save.auto.0': '{"big":1}', 'twopointsix.editorLevel': '{}', 'twopointsix.kept.shelfSent': '4', 'other.site': 'x', 'twopointsix.huge': 'x'.repeat(70000) });
  const B = memStore({ 'twopointsix.options': '{"old":1}', 'twopointsix.sneak': '{"on":false}', 'twopointsix.quality': 'verylow', 'twopointsix.save.auto.0': '{"theirs":1}' });
  const PA = new PrefSync(W, A, { pid: PID2 }), PB = new PrefSync(W, B, { pid: PID2 });
  await PA.look(); const puts = PA.stats.puts; await PA.look(); await PA.look();
  const kept = JSON.parse((await W.query('prefs:get', { pid: PID2 })).data);
  check(puts === 1 && PA.stats.puts === 1 && same(Object.keys(kept), ['twopointsix.findme', 'twopointsix.keys', 'twopointsix.options']), `prefs: the settings go up once and not again until one changes — and only settings (${Object.keys(kept).length} keys: no machine pick, no save, no id, nothing of another site)`);
  const changed = await PB.take();
  check(changed && same(prefsNow(B), prefsNow(A)) && B.getItem('twopointsix.sneak') === null && B.getItem('twopointsix.quality') === 'verylow' && B.getItem('twopointsix.save.auto.0') === '{"theirs":1}', 'prefs: a browser handed the id opens with them — its own stray settings gone, its machine pick and its saves untouched');
  A.setItem('twopointsix.options', '{"vol":{"master":0.9}}'); A.removeItem('twopointsix.keys'); await PA.look();
  const again = await PB.take(), none = await PB.take();
  check(PA.stats.puts === 2 && again && !none && B.getItem('twopointsix.options') === '{"vol":{"master":0.9}}' && B.getItem('twopointsix.keys') === null, 'prefs: a setting changed and one removed in the first browser are so in the second, and taking twice changes nothing');
  check(skipped('twopointsix.cloud.rev') && skipped('twopointsix.netquiet.visit') && skipped('unrelated') && !skipped('twopointsix.loadout') && !skipped('twopointsix.autosave'), 'prefs: the keys that never leave are this family\'s own, one visit\'s marks and anything that is not the game\'s');
  const PQ = new PrefSync(W, A, { pid: PID2, may: () => false }); A.setItem('twopointsix.options', 'quiet'); await PQ.look();
  check(PQ.stats.puts === 0, 'prefs: a browser that does not send saves nothing there');
}

// ---- files
{ const SID = 'visit-files', when = Date.UTC(2026, 9, 27, 5, 0, 0);
  const report = new Blob([JSON.stringify({ kind: 'bench', scenes: [{ name: 'yard', fps: 58.2 }, { name: 'hall', fps: 61 }], machine: { webgpu: 'probe gpu' } })], { type: 'application/json' });
  const packed = new Uint8Array(300 * 1024); { const r = mulberry32(0xf11e); for (let i = 0; i < packed.length; i++) packed[i] = (r() * 256) | 0; }
  let t = when; const FS = new FileSync(W, { pid: PID, sid: SID, build: () => 'abc1234', now: () => t++ });
  const took = FS.add('twopointsix-bench 2026-10-27.json', report, 'bench', { commit: 'abc1234', bench: '{"fps":58.2}' }), tooMuch = FS.add('facts.json', new Blob(['{}']), 'bench', { big: 'x'.repeat(FILESYNC.metaMax) }); FS.add('C:\\saves\\run 7.ndjson.gz', new Blob([packed], { type: 'application/gzip' }), 'replay');
  await FS.round(); await FS.round();
  const rows = (await ADMIN.query('files:list', { pid: PID })).filter((r: any) => r.sid === SID && r.name !== 'facts.json').reverse(), facts = (await ADMIN.query('files:list', { pid: PID })).find((r: any) => r.sid === SID && r.name === 'facts.json');
  check(rows.length === 2 && rows[0].name === 'twopointsix-bench 2026-10-27.json' && rows[0].kind === 'bench' && rows[0].type.startsWith('application/json') && rows[0].size === report.size && rows[0].build === 'abc1234' && rows[0].at === when
    && rows[1].name === 'run 7.ndjson.gz' && rows[1].kind === 'replay' && rows[1].size === packed.length && FS.stats.sent === 3 && FS.waiting === 0 && FS.stats.failed === 0
    && took === 'now' && tooMuch === 'now' && same(JSON.parse(rows[0].meta), { commit: 'abc1234', bench: '{"fps":58.2}' }) && rows[1].meta === undefined && !!facts && facts.meta === undefined,
    `files: two saved files are two rows with the names, kinds and sizes they were saved under, the facts beside one kept with it and facts too long left off (${rows.map((r: any) => `${r.name} ${r.size}`).join(' · ')})`);
  const got = (await pullFiles(ADMIN, { pid: PID })).filter((f: any) => f.row.sid === SID && f.row.name !== 'facts.json');
  check(got.length === 2 && !!got[0].bytes && new TextDecoder().decode(got[0].bytes!) === await report.text() && !!got[1].bytes && same([...got[1].bytes!.subarray(0, 64)], [...packed.subarray(0, 64)]) && got[1].bytes!.length === packed.length && hashBytes(got[1].bytes!) === hashBytes(packed)
    && got.every((f: any) => /^2026-10-27T05-00-00-00\d-visit-files-[A-Za-z0-9._-]+$/.test(f.saveAs)),
    `files: both read back out of the backend byte for byte (${got.map((f: any) => f.bytes?.length ?? 'none').join(' · ')} bytes), under names that are safe on a disk (${got[0]?.saveAs})`);
  check(same((await ADMIN.query('files:list', { kind: 'bench', since: when })).filter((r: any) => r.sid === SID).map((r: any) => r.name).sort(), ['facts.json', 'twopointsix-bench 2026-10-27.json']) && (await ADMIN.query('files:list', { pid: PID, since: when + 5 })).filter((r: any) => r.sid === SID).length === 0, 'files: the list reads by kind and by time');
  // held and not held
  { let may = false, on = true; const w2 = wire(S.url), F2 = new FileSync(w2, { pid: PID, sid: 'visit-files-wait', may: () => may, on: () => on });
    const saidHeld = F2.add('r0.json', new Blob(['{"i":0}']), 'bench'); for (let i = 1; i < FILESYNC.hold + 1; i++) F2.add(`r${i}.json`, new Blob([`{"i":${i}}`]), 'bench');
    await F2.round(); const quiet = w2.stats.calls, held = F2.waiting, let1 = F2.stats.dropped;
    may = true; await F2.round(); const sent = F2.stats.sent, names = (await ADMIN.query('files:list', { pid: PID })).filter((r: any) => r.sid === 'visit-files-wait').map((r: any) => r.name).sort();
    on = false; const calls3 = w2.stats.calls; const saidOff = F2.add('late.json', new Blob(['{}']), 'bench'); await F2.round(); on = true; const saidLevel = F2.add('yard.json', new Blob(['{}']), 'level'); await F2.round(); on = false;
    check(quiet === 0 && held === FILESYNC.hold && let1 === 1 && sent === FILESYNC.hold && names.length === FILESYNC.hold && names[0] === 'r1.json' && w2.stats.calls === calls3 && F2.waiting === 0 && saidHeld === 'held' && saidOff === null && saidLevel === null,
      `files: a browser that waits holds its files with no request made (${held} held of ${FILESYNC.hold + 1}, the oldest let go) and sends them when it may (${sent}); one whose switch is off is handed none, and a level from the editor is never held`);
    on = true; may = false; F2.add('held.json', new Blob(['{}']), 'bench'); await F2.round(); const heldNow = F2.waiting; on = false; await F2.round();
    check(heldNow === 1 && F2.waiting === 0 && w2.stats.calls === calls3, 'files: a file held when the switch goes off is let go, not sent'); }
  // a sending that fails on the way is tried again, and the file is kept once
  { let fail = 2; const F3 = new FileSync(W, { pid: PID, sid: 'visit-files-retry', fetch: ((u: any, o: any) => fail-- > 0 ? Promise.reject(new Error('line dropped')) : fetch(u, o)) as typeof fetch });
    F3.add('again.json', new Blob(['{"again":true}'], { type: 'application/json' }), 'bench');
    await F3.round(); const w1 = F3.waiting; await F3.round(); const w2 = F3.waiting; await F3.round(); await F3.round();
    const kept = (await ADMIN.query('files:list', { pid: PID })).filter((r: any) => r.sid === 'visit-files-retry');
    check(w1 === 1 && w2 === 1 && F3.waiting === 0 && F3.stats.failed === 2 && F3.stats.sent === 1 && kept.length === 1, `files: a sending that failed ${F3.stats.failed} times on the way is tried again and the file is kept once`); }
  // the bounds: the page's own, and the backend's for a caller that is not the page
  { const max = FILESYNC.max; FILESYNC.max = 1000; const w4 = wire(S.url), F4 = new FileSync(w4, { pid: PID, sid: 'visit-files-big' });
    F4.add('big.bin', new Blob([new Uint8Array(1001)]), 'replay'); F4.add('empty.bin', new Blob([]), 'replay'); await F4.round(); FILESYNC.max = max;
    const a = { pid: PID, sid: 'visit-files-big', kind: 'bench' };
    const r1 = await refused(W.mutation('files:start', { ...a, name: 'x.bin', size: max + 1 })), r2 = await refused(W.mutation('files:start', { ...a, name: '../x.bin', size: 10 })), r3 = await refused(W.mutation('files:start', { ...a, pid: PID2, sid: SID, name: 'x.bin', size: 10 }));
    const r5 = await refused(W.mutation('files:put', { ...a, name: 'x.bin', type: 'text/plain', at: 1, storageId: rows[0]?.storageId }));
    check(F4.stats.dropped === 2 && w4.stats.calls === 0 && /size/.test(r1) && /name/.test(r2) && /another browser/.test(r3) && /already a file/.test(r5),
      `files: a file past the bound or empty stays the player's alone (${w4.stats.calls} requests); the backend refuses one too big, a name with a folder in it, another browser's visit and a stored file claimed twice`);
    let listed = ''; try { await W.query('files:list', {}); listed = 'ANSWERED'; } catch (e: any) { listed = String(e.message); }
    check(listed !== 'ANSWERED' && fileName('C:\\a\\b/c d(1).json') === 'c d(1).json' && fileName('../../.env') === 'env' && fileName('ünï.json') === 'n_.json', `files: the list is a deploy key's read (a page is told: ${listed.slice(0, 60)}), and a saved file's name is cut to plain characters with no folders`);
    // a visit keeps so many
    const F5 = new FileSync(W, { pid: PID, sid: 'visit-files-many' }); const hold = FILESYNC.hold; FILESYNC.hold = 100;
    for (let i = 0; i < 66; i++) F5.add(`m${i}.json`, new Blob(['{}']), 'bench'); await F5.round(); FILESYNC.hold = hold;
    const many = (await ADMIN.query('files:list', { pid: PID, limit: 1000 })).filter((r: any) => r.sid === 'visit-files-many').length;
    check(many === 64 && F5.stats.sent === 64 && F5.stats.dropped === 2 && F5.waiting === 0, `files: a visit keeps ${many} files and the rest are refused and let go`); }
  // NOTHING IS DOWNLOADED ANY MORE: whoever hands a file over hears how it ended (once, and only of a file that was taken)
  { const ends: string[] = [], told = (n: string) => (ok: boolean) => { ends.push(`${n}:${ok}`); };
    const F6 = new FileSync(W, { pid: PID, sid: 'visit-files-ends' });
    const t1 = F6.add('good.json', new Blob(['{"a":1}']), 'bench', undefined, told('good')), t2 = F6.add('level.json', new Blob(['{}']), 'level', undefined, told('level')), t3 = F6.add('empty.json', new Blob([]), 'bench', undefined, told('empty'));
    await F6.round();
    const F7 = new FileSync(W, { pid: PID, sid: 'visit-files-many' }); F7.add('one-too-many.json', new Blob(['{}']), 'bench', undefined, told('full')); await F7.round();
    let on = true; const F8 = new FileSync(W, { pid: PID, sid: 'visit-files-held', on: () => on, may: () => false }), t8 = F8.add('held.json', new Blob(['{}']), 'bench', undefined, told('held'));
    await F8.round(); const waits = F8.waiting === 1 && !ends.some(e => e.startsWith('held')); on = false; await F8.round();
    const rowsEnds = (await ADMIN.query('files:list', { pid: PID, limit: 1000 })).filter((r: any) => r.sid === 'visit-files-ends').map((r: any) => r.name);
    check(t1 === 'now' && t2 === null && t3 === null && t8 === 'held' && waits && same(ends, ['good:true', 'full:false', 'held:false']) && same(rowsEnds, ['good.json']),
      `files: the saver hears how a file ended — it arrived, the backend refused it, the switch went off while it waited (${ends.join(' · ')}) — and nothing of one that was never taken`); }
  // the page's whole answer (Cloud.save) and the one-file way in for a page with no game (cloudFile)
  { CL.cloudForget(); const store = memStore(); (globalThis as any).localStorage = store; const Qs = (s: string) => new URLSearchParams(s), here = `cloud=${S.url}`;
    const offA = await CL.cloudFile(Qs(''), 'a.json', new Blob(['{}']), 'bench'), offB = await CL.cloudFile(Qs(`${here}&cloudfiles=0`), 'a.json', new Blob(['{}']), 'bench'), offC = await CL.cloudFile(Qs('cloud=0'), 'a.json', new Blob(['{}']), 'bench'), pidNone = store.getItem('twopointsix.cloud.pid');
    const sent = await CL.cloudFile(Qs(here), 'viewer-save.ndjson.gz', new Blob(['a replay, its notes in it']), 'replay', { of: 'viewer' }), lvl = await CL.cloudFile(Qs(here), 'yard.json', new Blob(['{}']), 'level');
    const rowsP = await ADMIN.query('files:list', { pid: store.getItem('twopointsix.cloud.pid') });
    check(offA === 'off' && offB === 'off' && offC === 'off' && pidNone === null && sent === 'sent' && lvl === 'no' && rowsP.length === 1 && rowsP[0].name === 'viewer-save.ndjson.gz' && /^page-/.test(rowsP[0].sid),
      `files: a page with no game sends one file and waits for its end (${sent}, filed under ${String(rowsP[0]?.sid).slice(0, 5)}…); with no backend, under ?cloud=0 or ?cloudfiles=0 it says off and makes no id; a level stays the player's (${lvl})`);
    const depsL = { sid: 'visit-save', store: null, build: () => 'abc1234', machine: () => ({ webgpu: 'probe' }), facts: () => ({ scenario: 'x', stage: 'infiltrate', live: true, stats: {}, preset: 'high', scale: 1, auto: true }) } as any;
    const C = CL.cloudBoot(Qs(here), depsL)!; let arrived: boolean | null = null;
    const s1 = C.save('take.json', new Blob(['{"t":1}']), 'perf', undefined, (ok: boolean) => { arrived = ok; }), s2 = C.save('yard.json', new Blob(['{}']), 'level'), s3 = C.save('none.json', new Blob([]), 'perf');
    const viaLive = await CL.cloudFile(Qs(here), 'via-live.json', new Blob(['{}']), 'bench'); await C.sync();
    const Coff = CL.cloudBoot(Qs(`${here}&cloudfiles=0`), { ...depsL, sid: 'visit-save-off' })!, s4 = Coff.save('q.json', new Blob(['{}']), 'perf');
    const rowsL = (await ADMIN.query('files:list', { pid: C.pid })).filter((r: any) => r.sid === 'visit-save').map((r: any) => r.name).sort();
    check(s1 === 'sent' && s2 === 'no' && s3 === 'no' && s4 === 'off' && arrived === true && viaLive === 'sent' && same(rowsL, ['take.json', 'via-live.json']),
      `files: the page's own answer says what became of a save (${s1} · a level ${s2} · an empty one ${s3} · under ?cloudfiles=0 ${s4}), tells the saver it arrived, and a one-file save on the game's page goes under the visit that is up (${rowsL.join(', ')})`);
    const asksYes = await CL.cloudSendsFiles(Qs(here)), asked0 = Coff.files.stats.asked, asksOff = await CL.cloudSendsFiles(Qs(`${here}&cloudfiles=0`)); CL.cloudForget();
    const pageYes = await CL.cloudSendsFiles(Qs(here)), pageOff = await CL.cloudSendsFiles(Qs(`${here}&cloudfiles=0`)), pageNone = await CL.cloudSendsFiles(Qs(''));
    check(asksYes === false && asksOff === false && Coff.files.stats.asked === asked0 && pageYes && !pageOff && !pageNone, 'files: a page can ask whether this browser would send a saved file before it draws a button — the visit that is up answers by its own words, a page with no game by the address — and asking sends and counts nothing');
    // a replay is a replay whichever road it takes
    const cutOwn = C.save('f7-own.json', new Blob(['{"k":"replay"}']), 'replay', { session: 'visit-save' }), cutGone = C.save('f7-gone.json', new Blob(['{"k":"replay"}']), 'replay', { session: 'visit-played-with-sending-off' }), cutFile = C.file('f7-gone2.json', new Blob(['{}']), 'f7', { session: 'visit-played-with-sending-off' });
    const Ct = CL.cloudBoot(Qs(`${here}&cloudtapes=0`), { ...depsL, sid: 'visit-save-notapes' })!, noTapes = Ct.save('r.json', new Blob(['{}']), 'replay'), benchStill = Ct.save('b.json', new Blob(['{}']), 'bench');
    CL.cloudForget(); const pageNamed = await CL.cloudFile(Qs(here), 'p.json', new Blob(['{}']), 'replay', { session: 'visit-of-another-browser' }), pageWord = await CL.cloudFile(Qs(`${here}&cloudtapes=0`), 'p.json', new Blob(['{}']), 'replay');
    await C.sync(); const rowsR = (await ADMIN.query('files:list', { pid: C.pid })).filter((r: any) => r.sid === 'visit-save' && r.kind !== 'perf' && r.kind !== 'bench').map((r: any) => r.name);
    check(cutOwn === 'sent' && cutGone === 'off' && cutFile === false && noTapes === 'off' && benchStill === 'sent' && pageNamed === 'off' && pageWord === 'off' && same(rowsR, ['f7-own.json']),
      `files: a replay goes by the replays' word too — a window cut from a visit whose tape may go is sent (${cutOwn}), one cut from a visit played with the sending off is not (${cutGone}), and under ?cloudtapes=0 no replay goes while a benchmark's file still does (${benchStill})`);
    (globalThis as any).localStorage = memStore(); CL.cloudForget(); }
  // A LONG TAKE IS PACKED on its way (with nothing downloaded, a file past the bound would be kept nowhere); and a page about to leave can wait for what is going
  { const big = JSON.stringify({ kind: 'perf', frames: Array.from({ length: 60000 }, (_, i) => ({ f: i, ms: 16.6 + (i % 7) * 0.25, gpu: 9.5, stage: 'infiltrate' })) }), raw = new Blob([big], { type: 'application/json' });
    const ends: string[] = [], told = (n: string) => (ok: boolean) => { ends.push(`${n}:${ok}`); };
    const FP = new FileSync(W, { pid: PID, sid: 'visit-files-packed' }); FP.add('perf-long.json', raw, 'perf', { commit: 'abc1234' }, told('long')); FP.add('small.json', new Blob(['{"a":1}'], { type: 'application/json' }), 'bench');
    const idle = await FP.settle(20000);
    const FN = new FileSync(W, { pid: PID, sid: 'visit-files-packed', pack: null }); FN.add('plain-long.json', raw, 'perf'); await FN.round();
    const max = FILESYNC.max; FILESYNC.max = 2000; const FB = new FileSync(W, { pid: PID, sid: 'visit-files-packed' }); const tb = FB.add('too-long.json', raw, 'perf', undefined, told('toolong')); await FB.round(); FILESYNC.max = max;
    const rowsK = (await ADMIN.query('files:list', { pid: PID, limit: 1000 })).filter((r: any) => r.sid === 'visit-files-packed').sort((a: any, b: any) => a.name < b.name ? -1 : 1);
    const got = (await pullFiles(ADMIN, { pid: PID, limit: 1000 })).filter((f: any) => f.row.sid === 'visit-files-packed'), long = got.find((f: any) => f.row.name === 'perf-long.json.gz');
    const rk = rowsK.find((r: any) => r.name === 'perf-long.json.gz'), mk = rk ? JSON.parse(rk.meta) : {};
    check(idle && same(rowsK.map((r: any) => r.name), ['perf-long.json.gz', 'plain-long.json', 'small.json']) && rk.type === 'application/gzip' && rk.size < raw.size / 5 && mk.packed === 'gzip' && mk.commit === 'abc1234' && mk.rawBytes === String(raw.size)
      && !!long?.bytes && new TextDecoder().decode(long.bytes) === big && /perf-long\.json$/.test(long.saveAs) && tb === 'now' && same(ends, ['long:true', 'toolong:false']),
      `files: a long take goes up packed (${(raw.size / 1024).toFixed(0)} KB as ${((rk?.size ?? 0) / 1024).toFixed(0)} KB) and the pull writes it out as it was saved; a small file and a browser with no packer send theirs as they are; one past the bound even packed is let go and its saver told`);
    // what a review of the packing found: a packer that fails, a long name, a file too big to hold, bytes that unpack to far more than a page may send
    { const ends2: string[] = [], told2 = (n: string) => (ok: boolean) => { ends2.push(`${n}:${ok}`); };
      const FT = new FileSync(W, { pid: PID, sid: 'visit-files-pack2', pack: () => Promise.reject(new Error('no packer after all')) }); FT.add('unpacked-after-all.json', raw, 'perf', undefined, told2('plain'));
      const longName = `${'take-'.repeat(22)}xxxxx.json`; FT.add(longName, raw, 'perf'); await FT.settle(20000);   // (120 characters)
      const hb = FILESYNC.holdBytes, rm = FILESYNC.rawMax; FILESYNC.holdBytes = raw.size; FILESYNC.rawMax = raw.size;
      const FQ = new FileSync(W, { pid: PID, sid: 'visit-files-pack2', may: () => false }); FQ.add('older.json', new Blob(['{"a":1}']), 'bench', undefined, told2('older')); const tBig = FQ.add('fills-the-hold.json', raw, 'perf', undefined, told2('fills')), kept = FQ.waiting;
      const tOver = FQ.add('past-the-hold.json', new Blob([big, big], { type: 'application/json' }), 'perf', undefined, told2('over')); FILESYNC.holdBytes = hb; FILESYNC.rawMax = rm;
      const FL = new FileSync(W, { pid: PID, sid: 'visit-files-pack2' }); FL.add(longName, raw, 'perf'); await FL.settle(20000);
      const rows2 = (await ADMIN.query('files:list', { pid: PID, limit: 1000 })).filter((r: any) => r.sid === 'visit-files-pack2').map((r: any) => r.name).sort();
      // a row that says it is packed, whose bytes unpack past the bound: the pull leaves it as it came
      const rmx = FILESYNC.rawMax; FILESYNC.rawMax = 1000; const bomb = (await pullFiles(ADMIN, { pid: PID, limit: 1000 })).find((f: any) => f.row.sid === 'visit-files-packed' && f.row.name === 'perf-long.json.gz'); FILESYNC.rawMax = rmx;
      check(same(ends2, ['plain:true', 'older:false']) && tBig === 'held' && kept === 1 && tOver === null && longName.length === 120 && same(rows2, [`${longName.slice(0, 112)}.json.gz`, longName, 'unpacked-after-all.json']) && !!bomb?.bytes && bomb.bytes.length < raw.size / 5 && /\.json\.gz$/.test(bomb.saveAs),
        `files: a packer that fails sends the file as it is; a long name keeps its head under .gz; a file that fills the hold lets go of the older one, never of itself, and one past it is not taken; bytes that unpack past the bound are written as they came (${rows2.length} rows)`); }
    // a page with no rounds of its own: two files handed over while the backend is away both hear their end
    { CL.cloudForget(); (globalThis as any).localStorage = memStore(); const tries = FILESYNC.tries, again = FILESYNC.againMs; FILESYNC.tries = 2; FILESYNC.againMs = 30;
      const away = new URLSearchParams('cloud=http://127.0.0.1:1'), t0 = Date.now();
      const both = await Promise.race([Promise.all([CL.cloudFile(away, 'a.json', new Blob(['{}']), 'bench'), CL.cloudFile(away, 'b.json', new Blob(['{}']), 'bench')]), new Promise<string[]>(r => setTimeout(() => r(['never', 'never']), 15000))]);
      FILESYNC.tries = tries; FILESYNC.againMs = again; (globalThis as any).localStorage = memStore(); CL.cloudForget();
      check(same(both, ['failed', 'failed']), `files: two files handed over on a page with no game while the backend is away both hear that they failed (${both.join(', ')} after ${Date.now() - t0} ms)`); }
    let may = false; const FH = new FileSync(W, { pid: PID, sid: 'visit-files-idle', may: () => may }); FH.add('held.json', new Blob(['{}']), 'bench'); const t0 = Date.now(), heldIdle = await FH.settle(20000), quick = Date.now() - t0 < 2000; may = true; const thenIdle = await FH.settle(20000);
    const down = new FileSync(wire('http://127.0.0.1:1'), { pid: PID, sid: 'visit-files-idle' }); down.add('lost.json', new Blob(['{}']), 'bench'); const downIdle = await down.settle(20000);
    const none = await new FileSync(W, { pid: PID, sid: 'visit-files-idle' }).settle(20000);
    check(!heldIdle && quick && thenIdle && FH.waiting === 0 && !downIdle && down.waiting === 1 && none, 'files: a page about to leave can wait for what is going — true once all has gone, at once with nothing waiting, and false without waiting where a file is held for the notice or its sending failed'); }
}

// ---- notes
{ const note = (id: string, text: string) => ({ id, r: 12.5, t: 12.5, n: 750, pos: [3.25, -1.5], text, at: '2026-10-27T05:30:00.000Z' });
  const rows: any[] = [{ sid: 'visit-n1', lastAt: '2026-10-27T05:20:00.000Z', pieces: 3, notes: [note('n1', 'he stops short of the door here')] }, { sid: 'visit-n2', lastAt: '2026-10-27T05:25:00.000Z', pieces: 2 }, { sid: 'visit-private', lastAt: '2026-10-27T05:26:00.000Z', pieces: 2, notes: [note('np', 'not to go')] }];
  const shelf: any[] = [{ id: 1, savedAt: '2026-10-27T05:19:50.000Z', seconds: 30, why: 'F7', scenario: 'Room clear', session: 'visit-n1', n1: 1800, notes: [note('s1', 'the stack breaks up')] }, { id: 2, savedAt: '2026-10-27T04:00:00.000Z', seconds: 30, why: 'F7', scenario: 'The depot', session: 'visit-gone', notes: [] }, { id: 3, savedAt: '2026-10-27T05:27:00.000Z', seconds: 30, why: 'F7', scenario: 'The depot', session: 'visit-private', notes: [note('sp', 'not to go either')] }];
  const window2 = { v: 1, seconds: 30, scenario: 'The depot', notes: [] }, fileAsked: number[] = [];
  let marks: Record<string, string> = {}, may = true; const wn = wire(S.url), FSn = new FileSync(W, { pid: PIDB, sid: 'visit-notes-files' });
  const NS = new NoteSync(wn, { rows: async () => rows, shelf: async () => shelf, shelfFile: async (id: number) => { fileAsked.push(id); return id === 2 ? window2 : null; } },
    { pid: PIDB, marks: { get: () => marks, set: (m: Record<string, string>) => { marks = m; } }, may: () => may, only: (sid: string) => sid !== 'visit-private', file: (n: string, b: Blob, k: string, m?: Record<string, string>) => FSn.add(n, b, k, m) === 'now', now: () => Date.UTC(2026, 9, 27, 5, 31) });
  const listed = async () => (await ADMIN.query('notes:list', { pid: PIDB })).map((r: any) => ({ key: r.key, line: JSON.parse(r.text) })).sort((a: any, b: any) => a.key < b.key ? -1 : 1);
  may = false; await NS.round(); const quiet = wn.stats.calls; may = true;
  await NS.round(); await FSn.round(); const L1 = await listed(), calls1 = wn.stats.calls, up = (await ADMIN.query('files:list', { pid: PIDB })).filter((r: any) => r.sid === 'visit-notes-files');
  check(quiet === 0 && same(L1.map((x: any) => x.key), ['f7:visit-gone:2026-10-27T04-00-00.000Z', 'f7:visit-n1:2026-10-27T05-19-50.000Z', 'visit:visit-n1']) && L1[2].line.k === 'notes' && L1[2].line.notes[0].text === 'he stops short of the door here' && L1[1].line.k === 'f7' && L1[1].line.n === 1800 && L1[1].line.notes[0].id === 's1' && L1[0].line.notes.length === 0,
    `notes: a kept visit's notes and the shelf's saved windows go up as the hand-over format's own lines (${L1.map((x: any) => x.line.k).join(' · ')}); a visit with none, and one whose tape may not go with the window saved out of it, send none`);
  check(same(fileAsked, [2]) && up.length === 1 && up[0].kind === 'f7' && JSON.parse(up[0].meta).session === 'visit-gone' && up[0].size === JSON.stringify(window2).length, `notes: a saved window whose visit is not kept goes up whole through the saved files (${up[0]?.name}, ${up[0]?.size} bytes); one whose visit is kept is not read`);
  await NS.round(); const calls2 = wn.stats.calls;
  rows[0].notes = [note('n1', 'he stops short of the door here, twice')]; await NS.round(); const calls3 = wn.stats.calls, L3 = await listed();
  rows[0].notes = []; await NS.round(); const L4 = await listed(), D = await pullDump(ADMIN, { pid: PIDB }), got = D ? linesOf(D.bytes).lines : [];
  check(calls2 === calls1 && same(fileAsked, [2]) && calls3 === calls2 + 1 && L3[2].line.notes[0].text.endsWith('twice') && L4[2].line.notes.length === 0, `notes: a round with nothing changed asks nothing; a note that changed is one call; notes taken off again empty the backend's line`);
  rows[0].notes = [note('n1', 'back again')]; await NS.round(); const D2 = await pullDump(ADMIN, { pid: PIDB }), got2 = D2 ? linesOf(D2.bytes).lines : [], V = KP.visitsOf(got2).get('visit-n1');
  check(!!D && got.filter((l: any) => l.k === 'notes').length === 0 && got.filter((l: any) => l.k === 'f7').length === 2 && !!D2 && D2.notes === 3 && got2[0].k === 'dump' && got2[0].shelf === 2 && !!V && V.notes.length === 1 && V.notes[0].text === 'back again' && got2.some((l: any) => l.k === 'f7' && l.session === 'visit-n1' && l.notes[0].text === 'the stack breaks up'),
    `notes: the pull writes them into its hand-over file as the browser's own file had them (${got2.map((l: any) => l.k).join(' · ')}) and the reader puts the note on its visit`);
  const b = { pid: PIDB, items: [{ key: 'visit:x', at: 1, text: '{"k":"notes","sid":"x","notes":[]}' }] };
  const r1 = await refused(W.mutation('notes:put', { ...b, items: [{ ...b.items[0], key: 'tape:x' }] })), r2 = await refused(W.mutation('notes:put', { ...b, items: [{ ...b.items[0], text: 'not json' }] })), r3 = await refused(W.mutation('notes:put', { ...b, items: [{ ...b.items[0], text: '{"k":"piece"}' }] })), r4 = await refused(W.query('notes:list', {}));
  check(/key/.test(r1) && /JSON/.test(r2) && /notes line/.test(r3) && r4 === '' ? false : /key/.test(r1) && /JSON/.test(r2) && /notes line/.test(r3), 'notes: a key that is not a note\'s, a text that is not JSON and a line of another kind are refused');
  let listedByPage = ''; try { await W.query('notes:list', {}); listedByPage = 'ANSWERED'; } catch (e: any) { listedByPage = String(e.message); }
  // the viewer's own way in (a page that boots no backend's side): one round under the same rules
  CL.cloudForget(); const storeV = memStore({ 'twopointsix.cloud.visits': '["visit-v1"]' }); (globalThis as any).localStorage = storeV;
  const vrows = { rows: async () => [{ sid: 'visit-v1', lastAt: '2026-10-27T05:40:00.000Z', pieces: 1, notes: [note('v1', 'typed in the viewer')] }, { sid: 'visit-v0', lastAt: '2026-10-27T05:00:00.000Z', pieces: 1, notes: [note('v0', 'an old visit that was never to go')] }] } as any;
  const no1 = await CL.cloudNotes(new URLSearchParams('cloud=0'), vrows), no2 = await CL.cloudNotes(new URLSearchParams(`cloud=${S.url}&cloudtapes=0`), vrows), pidV0 = storeV.getItem('twopointsix.cloud.pid');
  const yes = await CL.cloudNotes(new URLSearchParams(`cloud=${S.url}`), vrows), pidV = storeV.getItem('twopointsix.cloud.pid'), LV = (await ADMIN.query('notes:list', { pid: pidV })).map((r: any) => r.key);
  check(listedByPage !== 'ANSWERED' && !no1 && !no2 && pidV0 === null && yes && same(LV, ['visit:visit-v1']), `notes: the list is a deploy key's read; the viewer's way in sends a note typed there (${LV.join(', ')}), nothing under ?cloud=0 or ?cloudtapes=0, and nothing of a visit that may not go`);
  { CL.cloudForget(); const storeC = memStore({ 'twopointsix.cloud.visits': '["visit-v1"]' }); (globalThis as any).localStorage = storeC; const here = `cloud=${S.url}`, n0 = S.stats.calls;
    const mine = await CL.cloudCarries(new URLSearchParams(here), 'visit-v1'), file = await CL.cloudCarries(new URLSearchParams(here), 'visit-from-a-file'), none = await CL.cloudCarries(new URLSearchParams(here), undefined);
    const offT = await CL.cloudCarries(new URLSearchParams(`${here}&cloudtapes=0`), 'visit-v1'), off0 = await CL.cloudCarries(new URLSearchParams('cloud=0'), 'visit-v1'), noBackend = await CL.cloudCarries(new URLSearchParams(''), 'visit-v1');
    check(mine && !file && !none && !offT && !off0 && !noBackend && S.stats.calls === n0 && storeC.getItem('twopointsix.cloud.pid') === null, 'notes: a page can ask whether a visit\'s notes go up by themselves — yes for a visit of this browser whose tape may go, no for a replay from a file, under ?cloudtapes=0 or ?cloud=0, or with no backend — and asking sends nothing and makes no id'); }
  (globalThis as any).localStorage = memStore(); CL.cloudForget();
}

// ---- local: a build served from this machine asks its own server what backend it runs, and the dev server starts one beside itself
{ const Q = (s: string) => new URLSearchParams(s), G = globalThis as any, realFetch = globalThis.fetch;
  const LC = await import(`${ROOT}/tools/cloud/local.ts`), { mkdtempSync, mkdirSync, writeFileSync } = await import('node:fs'), { tmpdir } = await import('node:os');
  let asks = 0, says = S.url;
  G.fetch = (u: string, ...r: any[]) => String(u) === '/qa/cloud' ? (asks++, Promise.resolve(new Response(says))) : (realFetch as any)(u, ...r);
  const ask = async (host: string, said: string) => { CL.cloudFound(null); says = said; G.location = { hostname: host }; await CL.cloudFind(); await CL.cloudFind(); return CL.cloudUrl(Q('')); };
  CL.cloudForget(); G.localStorage = memStore();
  const hosted = await ask('game.example', S.url), asksHosted = asks;
  const bad = [await ask('localhost', '<!doctype html><title>no such page</title>'), await ask('localhost', 'http://10.0.0.5:3210'), await ask('localhost', '')];
  const hostedUrl = await ask('localhost', 'https://hosted.example/'), hostedLocal = CL.cloudLocal(Q('')), hostedChosen = CL.cloudChosen(Q(''));
  asks = 0; const mine = await ask('localhost', S.url + '/'), once = asks;
  check(hostedUrl === 'https://hosted.example' && !hostedLocal && !hostedChosen, 'local: a hosted address the dev server names is the page\'s backend but not its own machine\'s — the notice comes first there, as on a hosted build');
  check(hosted === '' && asksHosted === 0 && bad.every(u => u === '') && mine === S.url && once === 1 && CL.cloudLocal(Q('')) && CL.cloudChosen(Q('')) && CL.cloudSharing(Q('')),
    `local: a page served from this machine asks its server once and takes the backend it names (${mine === S.url ? 'the stand-in' : mine || 'nothing'}) as the browser's own choice; a page from anywhere else never asks, and an answer that is not an address is not taken`);
  const offNow = CL.cloudUrl(Q('cloud=0')), other = CL.cloudUrl(Q('cloud=http://127.0.0.1:1')), otherLocal = CL.cloudLocal(Q('cloud=http://127.0.0.1:1'));
  G.__TPS_CLOUD__ = 'https://built-in.example'; asks = 0; const built = await ask('localhost', S.url), builtAsks = asks, builtLocal = CL.cloudLocal(Q('')); delete G.__TPS_CLOUD__;
  check(offNow === '' && other === 'http://127.0.0.1:1' && !otherLocal && built === 'https://built-in.example' && builtAsks === 0 && !builtLocal,
    'local: ?cloud=0 still puts it off, an address on the address bar still wins, and a build given an address keeps it and never asks');
  // a note typed in the viewer of a local build goes up with no notice read (the viewer's way in asks the server itself)
  CL.cloudFound(null); says = S.url; G.location = { hostname: '127.0.0.1' }; const storeL = memStore({ 'twopointsix.cloud.visits': '["visit-l1"]' }); G.localStorage = storeL;
  const rowsL = { rows: async () => [{ sid: 'visit-l1', lastAt: '2026-10-27T14:40:00.000Z', pieces: 1, notes: [{ id: 'l1', t: 3, pos: null, text: 'typed on a local build', at: '2026-10-27T14:40:00.000Z' }] }] } as any;
  const wentL = await CL.cloudNotes(Q(''), rowsL), keysL = (await ADMIN.query('notes:list', { pid: storeL.getItem('twopointsix.cloud.pid') })).map((r: any) => r.key);
  check(wentL && same(keysL, ['visit:visit-l1']) && storeL.getItem('twopointsix.cloud.told') !== '1', `local: a note typed in the viewer of a local build is on its backend with no notice to read (${keysL.join(', ') || 'nothing'})`);
  CL.cloudFound(null); delete G.location; G.fetch = realFetch; G.localStorage = memStore(); CL.cloudForget();
  // the dev server's side (tools/cloud/local.ts): what it starts, with what, and when it hands the address out
  const dir = mkdtempSync(`${tmpdir()}/cloud-local-`), runs: { cmd: string[]; env: Record<string, string | undefined> }[] = [];
  const kills: (string | undefined)[] = []; let dies = false;
  const spawn = (cmd: string[], o: { env: Record<string, string | undefined> }) => { runs.push({ cmd, env: o.env }); return { exited: dies ? Promise.resolve(1) : new Promise<number>(() => { /* stays up */ }), kill(sig?: string) { kills.push(sig); } }; };
  check(LC.keyUrl('prod:quiet-otter-12|c2VjcmV0') === 'https://quiet-otter-12.convex.cloud' && LC.keyUrl('dev:a-b-3|x') === 'https://a-b-3.convex.cloud' && LC.keyUrl('preview:team:project|x') === '' && LC.keyUrl('0123abcd|x') === '' && LC.keyUrl('prod:Bad_Name|x') === '' && LC.keyUrl('prod:a.evil.example/|x') === '' && LC.keyUrl(undefined) === '',
    'local: a hosted deployment\'s key names its own address for a read given no address; a preview key, a local key and a name that is not a deployment\'s give none');
  const quiet = console.log; console.log = () => { /* the tool's own lines */ };
  const envAll = { TPS_LOCAL_CLOUD: '1', CONVEX_DEPLOY_KEY: 'prod:name|secret', CONVEX_DEPLOYMENT: 'prod:name', CONVEX_URL: 'https://name.convex.cloud', PATH: process.env.PATH };
  const given = LC.localCloud({ TPS_DEV_CLOUD: S.url + '/' }, dir, spawn), junk = LC.localCloud({ TPS_DEV_CLOUD: 'ftp://nowhere' }, dir, spawn), unasked = LC.localCloud({ CONVEX_DEPLOY_KEY: 'prod:name|secret', TPS_CLOUD_URL: 'https://name.convex.cloud' }, dir, spawn), unset = LC.localCloud(envAll, dir, spawn);
  const none0 = runs.length;
  mkdirSync(`${dir}/node_modules/.bin`, { recursive: true }); writeFileSync(`${dir}/node_modules/.bin/convex`, '');
  const up = LC.localCloud(envAll, dir, spawn), early = up?.url(), keyNone = LC.localKey(dir);
  mkdirSync(`${dir}/.convex/local/default`, { recursive: true }); writeFileSync(`${dir}/.convex/local/default/config.json`, JSON.stringify({ ports: { cloud: Number(new URL(S.url).port) }, adminKey: S.key }));
  let late = ''; for (let i = 0; i < 40 && !late; i++) { await Bun.sleep(150); late = up?.url() ?? ''; }
  up?.stop(); dies = true; const again = LC.localCloud(envAll, dir, spawn); await Bun.sleep(1600); const stale = again?.url();
  console.log = quiet; const run = runs[0], keyNow = LC.localKey(dir);
  check(given?.url() === S.url && junk === null && unasked === null && unset === null && none0 === 0, 'local: the dev server uses a backend it is handed (TPS_DEV_CLOUD) and starts nothing; with no word — the hosted build\'s own address in the shell is not one — it starts nothing and names none; asked for one before cloud/ is set up it says so and starts nothing');
  check(run.cmd.slice(1).join(' ') === 'dev --tail-logs disable' && run.env.CONVEX_AGENT_MODE === 'anonymous' && !('CONVEX_DEPLOY_KEY' in run.env) && !('CONVEX_DEPLOYMENT' in run.env) && !('CONVEX_URL' in run.env) && run.env.PATH === process.env.PATH,
    'local: asked for one it starts `convex dev` with no account, and never with a deploy key or a chosen deployment from the shell');
  check(early === '' && keyNone === null && late === S.url && keyNow?.url === S.url && keyNow?.key === S.key, `local: the address is handed out only once the backend answers (${late ? 'it did' : 'it never did'}), and a read takes its address and key from where the tool keeps them`);
  check(stale === '' && runs.length === 2 && same(kills, ['SIGINT']), `local: a backend that answers but that this start did not bring up (its convex dev stopped at once) is not handed out, and stopping the dev server interrupts the one it started (${kills.join() || 'no signal'})`);
}

S.stop();
console.log(fails ? `\n${fails} FAILED` : '\nall pass');
process.exit(fails ? 1 : 0);
