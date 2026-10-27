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
//   telemetry  frames in, one record a minute of live play with the numbers that were fed; a run's end, an error once, the visit's end; all read back
//   prefs      one browser's settings open in another that is handed its id; a key removed is removed there; machine keys and saves never leave
// The red path: CLOUD_BREAK=1 in front (src/cloud/tapes.ts drops every third piece on its way up; src/cloud/cloud.ts does not wait for the notice) must
// FAIL `tapes` and `off`.
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
const { Telemetry, TELEMETRY, clean } = await import(`${ROOT}/src/cloud/telemetry.ts`);
const CL = await import(`${ROOT}/src/cloud/cloud.ts`);
const { pullDump } = await import(`${ROOT}/tools/cloud/pull.ts`);

let fails = 0;
const check = (ok: boolean, text: string) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); if (!ok) fails++; };
const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);
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
    await C.sync(); const quiet = C.wire.stats.calls, listed = store.getItem('twopointsix.cloud.visits');
    H.told(); await new Promise(r => setTimeout(r, 100)); await C.sync();
    const row = (await ADMIN.query('tapes:list', { pid: C.pid })).find((r: any) => r.sid === 'visit-first'), kept = (await R1.store.rows()).find((r: any) => r.sid === 'visit-first');
    check(quiet === 0 && listed === '["visit-before","visit-first"]' && store.getItem('twopointsix.cloud.visits') === '["visit-first"]' && !!row && !!kept && row.have === kept.pieces && C.tapes!.stats.failed === 0,
      `off: the first visit's tape waits for the notice (${quiet} requests) and then goes up whole (${row?.have} of ${kept?.pieces} pieces); a visit from before the notice is off the list`);
    // the boot's record: not asked for before the browser sends, not written before the boot is over, and written once
    const asked0 = bootAsked; bootOver = true; await C.sync(); await C.sync(); await C.sync();
    const boots = await ADMIN.query('telemetry:list', { sid: 'visit-first', kind: 'boot' });
    check(asked0 >= 1 && bootAsked === asked0 + 1 && boots.length === 1 && boots[0].data.pipelinesMs === 1250 && boots[0].data.pipelines.total === 97 && boots[0].data.slowest[0].label === 'gather' && boots[0].data.nav === 'reload', `off: the boot's timeline is one record, written once the boot is over (${boots[0]?.data.pipelinesMs} ms on ${boots[0]?.data.pipelines?.total} shader pipelines, a ${boots[0]?.data.nav})`);
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

S.stop();
console.log(fails ? `\n${fails} FAILED` : '\nall pass');
process.exit(fails ? 1 : 0);
