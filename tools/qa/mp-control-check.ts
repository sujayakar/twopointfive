// CO-OP'S CONTROL PLANE — the check (cloud/convex/games.ts · runners.ts · http.ts · tools/mp/runner.ts):   bun run tools/qa/mp-control-check.ts [family …]
// NEEDS the backend's own packages: `npm ci` in cloud/ once. Three things stand here, on this machine, and nothing hosted is touched:
//   the backend   a STAND-IN (cloud/test/standin.ts): the real functions and schema in the service's test library, behind the service's HTTP routes
//   the runner    the REAL local runner (tools/mp/runner.ts), on a port of its own, starting real child processes
//   the servers   STAND-IN game servers (tools/mp/runner-standin.ts): the heartbeat and the seats of a server, and no game
// The clocks are set short (seconds, not minutes) through the same words a deployment's environment sets. The service's own clock is not here: the cron
// that calls the reaper twice a minute is a timer of this file's that calls it four times a second.
//   cap        six pages ask for a game at once where two servers may run: two are made, four are told it is full; a build no runner runs is refused;
//              a seat's key opens its seat on the server and no other key does
//   idle       a server nobody is connected to ends itself: its game is over, its process is gone, and the place it held is free again
//   join       by code: the next free seat, its own key, the same seat again for the same browser; refused for a full game, another build, no such code;
//              a stranger is told no key and no address; both seats connected is 'playing'
//   door       the heartbeat's door refuses what is not a heartbeat, a wrong secret, a game that is not live
//   reaper     servers that do NOT keep the contract are ended by the control plane: one that never says a word, one that goes silent, one that sits
//              with nobody connected, one past its lifetime — each game marked with why, each process stopped by the sweep
//   runner     a runner that is away fails the game; the sweep it missed is made when it is back; a server the runner could not stop holds its place
//              against the cap until it is stopped; a server of ours that no game names is stopped, and one that is another control plane's is left alone
//   hosted     the hosted build's registration: no Modal token, no runner; with one, a runner at the Modal app for this build, and a game of that build
//              goes to Modal's client (which is pointed at nothing here: the start fails, the game says so, the place is freed)
// The red paths: MPCAP_BREAK=1 (cloud/convex/games.ts makes a game without counting) must FAIL `cap`; MPREAP_BREAK=1 (the reaper ends nothing) must FAIL
// `reaper` and `runner` (a sweep that was missed is never asked for again).
// NOT HERE: the service itself (its scheduler, its cron, its transactions under real contention: the test library runs one mutation at a time), a runner
// that makes sandboxes (cloud/convex/modal.ts is reached here and no further: never run against Modal), the real game server, a page.
// EXIT 0 all pass · 1 a failure · 2 the backend's packages are not installed.
import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { wire, WireError } from '../../src/cloud/wire.ts';
import { localRunner } from '../mp/runner.ts';
const ROOT = resolve(import.meta.dir, '../..');
if (!existsSync(`${ROOT}/cloud/node_modules/convex-test`) || !existsSync(`${ROOT}/cloud/node_modules/modal`)) { console.error('mp-control-check: the backend\'s packages are not installed — run `npm ci` in cloud/ first'); process.exit(2); }
// (the clocks, before a function reads them: a server must be heard from in 3 s, is gone after 1.5 s of silence, idles out after 2 s, lives 30 s)
Object.assign(process.env, { MP_START_S: '3', MP_SILENT_S: '1.5', MP_IDLE_S: '2', MP_MAX_S: '30', MP_BEAT_S: '0.25' });
const { standIn } = await import(`${ROOT}/cloud/test/standin.ts`);

const FAMILIES = ['cap', 'idle', 'join', 'door', 'reaper', 'runner', 'hosted'];
const asked = process.argv.slice(2).filter(a => !a.startsWith('-'));
if (asked.some(a => !FAMILIES.includes(a))) { console.error(`mp-control-check: families are ${FAMILIES.join(' · ')}`); process.exit(2); }
const runs = (f: string): boolean => !asked.length || asked.includes(f);

let fails = 0;
const check = (ok: boolean, text: string) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); if (!ok) fails++; };
const refused = async (p: Promise<unknown>): Promise<string> => { try { await p; return ''; } catch (e: any) { return e instanceof WireError && e.refused ? e.message : `NOT A REFUSAL: ${String(e)}`; } };
const until = async (what: () => Promise<boolean> | boolean, ms = 8000): Promise<boolean> => { for (const end = Date.now() + ms; Date.now() < end;) { if (await what()) return true; await Bun.sleep(40); } return false; };
const pid = (n: number): string => `checkpid${String(n).padStart(4, '0')}a1b2c3d4e5f6`;
const BUILD = 'c0ffee123456', RKEY = 'runner-key-of-the-check-0123456789';
const setup = (seats: number) => ({ scenario: 'compound', seed: 7, knobs: { hard: true }, seats: Array.from({ length: seats }, (_, i) => ({ kit: `kit ${i}` })) });

const S = standIn();
const page = wire(S.url), admin = wire(S.url, { key: S.key });
const runner = localRunner({ key: RKEY, build: BUILD, entry: 'tools/mp/runner-standin.ts' });
/** (`deaf.stop`: the runner answers a stop with an error and stops nothing, as one that cannot reach its servers would) */
const deaf = { stop: false };
const serveRunner = (port: number) => Bun.serve({ port, hostname: '127.0.0.1', fetch: async (req: Request) => deaf.stop && req.url.endsWith('/stop') ? new Response('cannot', { status: 500 }) : (await runner.handle(req)) ?? new Response('not found', { status: 404 }) });
let RS = serveRunner(0);
const RPORT = RS.port, RURL = `http://127.0.0.1:${RPORT}/mp/runner`, MINE = `${S.url}/mp/heartbeat`, OWNER = S.url.replace('http://', '').replace(':', '_');
interface Fleet { name: string; url: string; builds: string[]; cap: number; live: number; dirty: number; swept: number; key?: string }
const fleet = () => admin.query<Fleet[]>('runners:list', {});
const mine = async (): Promise<Fleet> => (await fleet()).find(r => r.name === 'check')!;
/** the runner has been swept since anything changed: every place that is not a live game's is free */
const settled = () => until(async () => { const r = await mine(); return r.dirty === r.swept; });
const register = (cap: number) => admin.mutation('runners:register', { name: 'check', url: RURL, key: RKEY, build: BUILD, cap });
const cron = setInterval(() => void admin.mutation('games:reap', {}).catch(() => { /* the next turn asks again */ }), 250);
await register(2); await settled();

/** a family that throws has failed, and the ones after it still run */
const family = async (name: string, body: () => Promise<void>): Promise<void> => {
  if (!runs(name)) return;
  console.log(name);
  try { await body(); } catch (e) { check(false, `${name} stopped: ${String((e as Error)?.message ?? e)}`); }
};
interface View { code: string; state: string; why: string; seats: boolean[]; connected: boolean[]; seat: number; seatKey: string; address: string }
const make = (n: number, seats = 2) => page.mutation<View>('games:make', { pid: pid(n), build: BUILD, setup: setup(seats) });
const get = (n: number, code: string) => page.query<View | null>('games:get', { pid: pid(n), code });
const up = (n: number, code: string) => until(async () => { const g = await get(n, code); return !!g && g.state === 'waiting' && g.address !== ''; });
const over = (n: number, code: string, state: string, why: string, ms = 12000) => until(async () => { const g = await get(n, code); return !!g && g.state === state && g.why === why; }, ms);
const carries = (): number => runner.list().length;
const rows = () => admin.query<{ _id: string; code: string; state: string }[]>('games:list', {});
/** a seat's connection to a server: resolves to the socket once the server has given the seat, to null if it shut the door */
const sit = (address: string, seat: number, key: string): Promise<WebSocket | null> => new Promise(done => {
  const ws = new WebSocket(address); ws.onopen = () => ws.send(JSON.stringify({ seat, key })); ws.onmessage = () => done(ws); ws.onclose = () => done(null); ws.onerror = () => done(null);
});
/** every game made so far is over and nothing runs */
const quiet = async () => await until(async () => carries() === 0 && (await rows()).every(g => g.state === 'ended' || g.state === 'failed'), 15000) && await settled();

await family('cap', async () => {
  const asks = await Promise.all([1, 2, 3, 4, 5, 6].map(n => make(n).then(g => ({ g, why: '' }), (e: Error) => ({ g: null, why: e.message }))));
  const made = asks.filter(a => a.g), full = asks.filter(a => !a.g && /full:/.test(a.why));
  check(made.length === 2 && full.length === 4, `six pages asked at once where two servers may run: ${made.length} games were made and ${full.length} were told it is full`);
  check(made.every(a => /^[BCDFGHJKLMNPQRSTVWXZ]{5}$/.test(a.g!.code) && a.g!.seat === 0 && a.g!.seatKey.length >= 32) && made[0]?.g!.code !== made[1]?.g!.code, 'each game has its own code of five letters, and its maker holds the first seat with a key');
  const hosts = asks.map((a, i) => ({ n: i + 1, g: a.g })).filter(h => h.g);
  check((await Promise.all(hosts.map(h => up(h.n, h.g!.code)))).every(Boolean) && carries() === 2, `both servers were started and heard from: ${carries()} run, and their games wait at an address`);
  check(/full:/.test(await refused(make(7))), 'a seventh page is told it is full while both run');
  check(/build:/.test(await refused(page.mutation('games:make', { pid: pid(8), build: 'another-build', setup: setup(2) }))), 'a page of a build no runner runs is refused');
  check(/seats/.test(await refused(page.mutation('games:make', { pid: pid(8), build: BUILD, setup: setup(3) }))) && /pid/.test(await refused(page.mutation('games:make', { pid: 'short', build: BUILD, setup: setup(2) }))), 'three seats and a pid that is not one are refused');
  const h = hosts[0]; if (h) {
    const g = (await get(h.n, h.g!.code))!;
    const wrong = await sit(g.address, 0, 'not-the-key'), right = await sit(g.address, 0, g.seatKey);
    check(wrong === null && right !== null, 'the server lets the seat\'s key in and shuts the door on another');
    right?.close();
  }
  const f = await fleet();
  check(f.length === 1 && f[0].live === 2 && f[0].cap === 2 && f[0].key === undefined, 'the runner reads back as carrying two of two, and its key is not in the read');
});

await family('idle', async () => {
  const a = await make(11).catch(() => null), n = a ? 11 : 0;
  // (after `cap` the two places are held: the games there idle out here. Alone, this family makes its own)
  const before = await rows();
  check(await quiet(), `servers nobody is connected to ended themselves: ${before.filter(g => g.state !== 'ended' && g.state !== 'failed').length} games are over and no process is left`);
  if (a) check(await over(n, a.code, 'ended', 'server'), 'the game says it ended, and that its server said so');
  const b = await make(12).catch(() => null);
  check(!!b && await up(12, b.code), 'the place is free again: a new game is made and its server answers');
  await quiet();
});

await family('join', async () => {
  const a = await make(21); await up(21, a.code);
  const b = await page.mutation<View>('games:join', { pid: pid(22), code: a.code.toLowerCase(), build: BUILD });
  const host = (await get(21, a.code))!;
  check(b.seat === 1 && b.seatKey.length >= 32 && b.seatKey !== host.seatKey && b.address === host.address && host.address.startsWith('ws://'), 'a second browser joins by the code: the next seat, a key of its own, the same address');
  const again = await page.mutation<View>('games:join', { pid: pid(22), code: a.code, build: BUILD });
  check(again.seat === 1 && again.seatKey === b.seatKey, 'the same browser joining again has the same seat and key');
  check(/full:/.test(await refused(page.mutation('games:join', { pid: pid(23), code: a.code, build: BUILD }))), 'a third browser is told every seat is taken');
  check(/build:/.test(await refused(page.mutation('games:join', { pid: pid(22), code: a.code, build: 'another-build' }))), 'a page of another build than the game is refused');
  check(/code:/.test(await refused(page.mutation('games:join', { pid: pid(23), code: 'ZZZZZ', build: BUILD }))) && (await get(23, 'ZZZZZ')) === null, 'a code no game has is refused, and reads as no game');
  const stranger = (await get(23, a.code))!;
  check(stranger.seat === -1 && stranger.seatKey === '' && stranger.address === '' && stranger.seats.every(Boolean), 'a browser that holds no seat is told the game is full, and neither a key nor the address');
  const w0 = await sit(host.address, 0, host.seatKey), w1 = await sit(host.address, 1, b.seatKey);
  check(!!w0 && !!w1 && await until(async () => { const g = await get(21, a.code); return g!.state === 'playing' && g!.connected.every(Boolean); }), 'with both seats connected the game reads as playing, both connected');
  w1?.close();
  check(await until(async () => { const g = await get(21, a.code); return g!.state === 'waiting' && g!.connected[0] && !g!.connected[1]; }), 'one of them gone, it waits again and says which seat is connected');
  await Bun.sleep(3200);
  check((await get(21, a.code))!.state === 'waiting' && carries() === 1, 'a game somebody is connected to is not idle: it still runs past the idle clock');
  w0?.close();
  check(await over(21, a.code, 'ended', 'server') && await quiet(), 'both gone, it idles out and its server is gone');
});

await family('door', async () => {
  const a = await make(31, 1); await up(31, a.code);
  const id = (await rows()).find(g => g.code === a.code)!._id;
  const beat = async (body: unknown): Promise<number> => (await fetch(MINE, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: typeof body === 'string' ? body : JSON.stringify(body) })).status;
  const b = { game: id, secret: 'not-the-secret', state: 'ended', tick: 0, seats: [false] };
  check(await beat(b) === 403 && (await get(31, a.code))!.state === 'waiting', 'a heartbeat with another secret is refused, and ends nothing');
  check(await beat({ ...b, game: 'no-such-game' }) === 410, 'a heartbeat for a game that is not there is told it is gone');
  check(await beat('{') === 400 && await beat({ ...b, tick: 'soon' }) === 400 && await beat({ game: id }) === 400, 'what is not a heartbeat is refused');
  check(/Could not find/.test(await refused(page.mutation('games:heard', { ...b })).then(m => m.replace('NOT A REFUSAL: ', ''))) && /Could not find/.test(await refused(page.mutation('games:reap', {})).then(m => m.replace('NOT A REFUSAL: ', ''))), 'a page cannot call the heartbeat\'s mutation or the reaper');
  check(await quiet(), 'left alone, the game idles out');
});

await family('reaper', async () => {
  const t0 = Date.now();
  // (a child takes the environment as it is when the runner starts it, which is a moment after `make` answers: hold the words until the server is listed)
  const held = async (n: number, words: Record<string, string>, seats = 2) => { Object.assign(process.env, { STANDIN_STUBBORN: '1', ...words }); try { const c = carries(), g = await make(n, seats); await until(() => carries() > c); return g; } finally { for (const k of ['STANDIN_STUBBORN', ...Object.keys(words)]) delete process.env[k]; } };
  const dumb = await held(41, { STANDIN_MUTE_S: '0' });
  const still = await held(42, {});
  check(await up(42, still.code) && carries() === 2, 'two servers that will not end themselves are up (one of them never says a word)');
  check(await over(41, dumb.code, 'failed', 'never started') && await over(42, still.code, 'ended', 'idle'), 'the one never heard from is failed as never started; the one that sits with nobody connected is ended as idle');
  check(await until(() => carries() === 0), `the sweep stopped both processes (${((Date.now() - t0) / 1000).toFixed(1)} s after they were asked for)`);
  const hush = await held(43, { STANDIN_MUTE_S: '1' });
  check(await over(43, hush.code, 'failed', 'silent') && await until(() => carries() === 0), 'a server that was heard from and then went silent is failed as silent, and stopped');
  process.env.MP_MAX_S = '2';
  const old = await held(44, {}, 1); await up(44, old.code);
  const g = (await get(44, old.code))!, ws = await sit(g.address, 0, g.seatKey);
  check(!!ws && await over(44, old.code, 'ended', 'lifetime') && await until(() => carries() === 0), 'a server past its lifetime with a player still connected is ended as past its lifetime, and stopped');
  process.env.MP_MAX_S = '30';
  const again = await make(45).catch(() => null);
  check(!!again && await up(45, again.code), 'every place they held is free again');
  await quiet();
});

await family('runner', async () => {
  RS.stop(true);
  const a = await make(51);
  check(await until(async () => { const g = await get(51, a.code); return g!.state === 'failed' && g!.why === 'start failed'; }), 'with the runner away, a game asked for is failed and says the start failed');
  const told = (await get(52, a.code))!, kept = (await admin.query<{ code: string; detail?: string }[]>('games:list', {})).find(g => g.code === a.code);
  check(told.why === 'start failed' && !!kept?.detail && !JSON.stringify(told).includes(kept.detail), 'what the runner said of it is kept for a key\'s read and is in no page\'s answer');
  await Bun.sleep(600);
  const f = await mine();
  check(f.live === 0 && f.dirty !== f.swept, 'no game of it is live, and the runner is marked as not swept');
  RS = serveRunner(RPORT);
  check(await settled(), 'the runner back, the reaper\'s next turn makes the sweep that was missed');
  // (the cap is on SERVERS: a game over whose server the runner did not stop still holds its place)
  deaf.stop = true; process.env.STANDIN_STUBBORN = '1';
  const b1 = await make(53), b2 = await make(54);
  await until(() => carries() === 2); delete process.env.STANDIN_STUBBORN;
  check(await over(53, b1.code, 'ended', 'idle') && await over(54, b2.code, 'ended', 'idle') && carries() === 2, 'two games are ended as idle by the reaper, and a runner that cannot stop its servers still carries both');
  check(/full:/.test(await refused(make(55))), 'a new game is refused while they run: a server that was not stopped holds its place');
  deaf.stop = false;
  check(await until(() => carries() === 0) && await settled() && !!(await make(55).catch(() => null)), 'the runner able again, the next sweep stops both and a game can be made');
  await quiet();
  process.env.STANDIN_STUBBORN = '1';
  const body = { build: BUILD, setup: '{"seatKeys":[]}', secret: 's', idleS: 60, maxS: 60 };
  const ours = await runner.start({ ...body, game: 'a-game-nobody-made', control: MINE, owner: OWNER });
  const theirs = await runner.start({ ...body, game: 'another-planes-game', control: 'http://127.0.0.1:9/mp/heartbeat', owner: 'another.plane' });
  delete process.env.STANDIN_STUBBORN;
  await register(2);   // (a runner that registers is swept)
  check(await until(() => !runner.list().some(s => s.id === ours.id)), 'a server of ours that no game names is stopped by the sweep');
  await Bun.sleep(600);
  check(runner.list().some(s => s.id === theirs.id), 'a server that is another control plane\'s is left alone');
  const noKey = await fetch(`${RURL}/list`, { method: 'POST', body: '{}' }), wrongKey = await fetch(`${RURL}/start`, { method: 'POST', headers: { Authorization: 'Bearer guess' }, body: '{}' });
  const otherBuild = await fetch(`${RURL}/start`, { method: 'POST', headers: { Authorization: `Bearer ${RKEY}` }, body: JSON.stringify({ ...body, build: 'another-build', game: 'x', control: MINE, owner: OWNER }) });
  check(noKey.status === 401 && wrongKey.status === 401 && otherBuild.status === 409 && runner.list().length === 1, `the runner answers nobody without its key, and starts no server of another build (${noKey.status} ${wrongKey.status} ${otherBuild.status} ${runner.list().length})`);
});

await family('hosted', async () => {
  for (const k of ['MODAL_TOKEN_ID', 'MODAL_TOKEN_SECRET']) delete process.env[k];
  const A = 'abcdef012345', B = 'bcdef0123456';
  check(await admin.mutation('runners:hosted', { build: A }) === 'no Modal token' && !(await fleet()).some(r => r.name === 'modal'), 'a deployment with no Modal token says so and has no hosted runner');
  // (a token that is nobody's, and Modal's address pointed at a port of this machine where nothing listens: the calls are made and none can land.
  // A server is given longer to be heard from than Modal's client takes to give up, so that it is the start that fails the game)
  Object.assign(process.env, { MODAL_TOKEN_ID: 'ak-check', MODAL_TOKEN_SECRET: 'as-check', MODAL_SERVER_URL: 'http://127.0.0.1:9', MP_START_S: '60', MP_CAP: '0' });
  check(/build:/.test(await refused(admin.mutation('runners:hosted', { build: 'not a commit; rm' }))), 'a build that is not a commit is refused');
  const modal = async () => (await fleet()).find(r => r.name === 'modal');
  check(await admin.mutation('runners:hosted', { build: A }) === 'modal' && (await modal())?.cap === 0 && /full:/.test(await refused(page.mutation('games:make', { pid: pid(60), build: A, setup: setup(2) }))), 'with a token the hosted runner is registered; a cap of none in the environment is none, and no game is made');
  delete process.env.MP_CAP;
  await admin.mutation('runners:hosted', { build: B, cap: 3 });
  const m = await modal();
  check(!!m && m.url === 'modal:twopointsix-mp' && m.cap === 3 && JSON.stringify(m.builds) === JSON.stringify([B, A]), 'the next deploy reads back as the Modal app, three servers at most, its build first and the one before it still served');
  await until(async () => { const r = await modal(); return !!r && r.cap - r.live - (r.dirty - r.swept) > 0; }, 1000);   // (its sweeps cannot land here either: the places they hold are counted)
  const g = await page.mutation<View>('games:make', { pid: pid(61), build: A, setup: setup(2) }).catch((e: Error) => e.message);
  const failed = typeof g !== 'string' && await until(async () => (await get(61, g.code))!.state === 'failed', 30000);
  const kept = typeof g === 'string' ? undefined : (await admin.query<{ code: string; why?: string; detail?: string }[]>('games:list', {})).find(x => x.code === g.code);
  check(failed && kept?.why === 'start failed' && /modal\.client/i.test(kept.detail ?? ''), `a game of the older build goes to Modal's client; with Modal out of reach its start fails, and the kept detail is that client's own (${typeof g === 'string' ? g : `${kept?.why}: ${(kept?.detail ?? '').slice(0, 60)}`})`);
  check((await modal())!.live === 0, 'and no game of it is live');
  await admin.mutation('runners:unregister', { name: 'modal' });
  for (const k of ['MODAL_TOKEN_ID', 'MODAL_TOKEN_SECRET', 'MODAL_SERVER_URL']) delete process.env[k];
  process.env.MP_START_S = '3';
});

clearInterval(cron);
runner.stopAll(); RS.stop(true); S.stop();
await until(() => carries() === 0, 5000);
console.log(fails ? `mp-control-check: ${fails} FAILED` : 'mp-control-check: all pass');
process.exit(fails ? 1 : 0);
