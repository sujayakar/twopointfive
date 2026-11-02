// THE GAME SERVER, CHECKED (src/net/, tools/mp/; docs/agents/net.md): a server process and scripted clients over REAL WebSockets on this machine, plus the
// input rule on its own. Not game code: it starts processes and reads what they print.
//   bun run tools/qa/mpserver-check.ts                    every line below
//   MPSERVER_BREAK=1 bun run tools/qa/mpserver-check.ts   the red path: the seated client flips the last bit of its own man's place after tick 90 — the lines
//                                                         that hold its state words against the server's must FAIL
//   --setup '{"search":"?rescue","seed":3}'               another mission (default: the four-man room clear in the office, seed 7)
// THE LINES:
//   the rule      (no process) a seat's line decides a tick's record: its own in time · a held one with the edges off when none came · a late press lands on the
//                 next tick · an empty seat holds nothing · a second record for one tick is not taken · a record on the wire comes back to the bit
//   the cores     (no process) the server's core and the client's on one made-up clock, a counter for a game: no tick before the seat says ready · records that
//                 are not records are not taken and nothing throws · a client that sends nothing for three seconds is not told to aim further · after the
//                 server stalls a second and lets its clock slip the seat's records are in time again · a watcher that joins late over a socket that drops
//                 messages still gets every tick in order · a fifth watcher, a client of no named build and a watcher's "lost" are turned away
//   stood         the server made its recipe, stood its game and answers /health as `waiting`, no tick run
//   same fight    a seated client and a watcher walk 1200 ticks and every state word (one each half second) is the server's; the seat's man moved and fired
//   late joiner   a watcher that connects six seconds in is sent the run so far and ends on the same words
//   lag           a second server, its seated client 60 to 100 ms away each way: every word still the server's, and how its records arrived is printed
//   control       the heartbeats reached the stand-in control plane with the game's secret: waiting, then playing, then done
//   refused       a hello with a wrong seat key, another game's id, another wire or (when the server names its build) another build is told why and let go
//   idle          with everybody gone the server ended itself inside its idle time, exit 0
// EXIT 0 = every line passed · 1 = a line failed · 2 = a process could not be started.
import { resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.MPSERVER_BREAK === '1';
const args = process.argv.slice(2), setupArg = args.includes('--setup') ? args[args.indexOf('--setup') + 1] : '{"search":"?roomclear&squad4","seed":7}';
let fails = 0;
const line = (ok: boolean, what: string) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); if (!ok) fails++; };
const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));

// ---- the rule, on its own
{
  const { SeatLine } = await import(`${ROOT}/src/net/timeline.ts`), W = await import(`${ROOT}/src/net/wire.ts`);
  const rec = (k: string[], extra: object = {}) => ({ k, b: 0, m: [10, 20], s: [1280, 720, 1280, 720], v: [1, 0, 2, 0.5, 0.96, 21, 0.6, 16 / 9, 0], ...extra });
  const L = new SeatLine();
  L.put(0, rec(['KeyW'], { p: ['KeyW'] }), 0);
  const t0 = L.take(0, true), t1 = L.take(1, true);
  const own = t0.k.join() === 'KeyW' && t0.p?.join() === 'KeyW', held = t1.k.join() === 'KeyW' && t1.p === undefined && t1.c === undefined;
  L.put(1, rec(['KeyW', 'KeyF'], { p: ['KeyF'], c: 1, w: 3 }), 2);   // late: tick 1 has run
  const t2 = L.take(2, true), t3 = L.take(3, true);
  const late = t2.p?.join() === 'KeyF' && t2.c === 1 && t2.w === 3 && t2.k.includes('KeyF') && t3.p === undefined && t3.c === undefined && L.late === 1;
  const empty = L.take(4, false), idle = empty.k.length === 0 && empty.b === 0 && empty.v[0] === 1;
  L.put(9, rec(['KeyA']), 5); L.put(9, rec(['KeyD']), 5); for (let t = 5; t < 9; t++) L.take(t, true);
  const once = L.take(9, true).k.join() === 'KeyA';
  const far = (() => { const M = new SeatLine(); M.put(5 + W.NET.horizon + 1, rec(['KeyW']), 5); return M.got === 0; })();
  const wheel = (() => { const M = new SeatLine(); M.put(0, rec([], { w: -3 }), 1); M.put(1, rec([], { w: 3 }), 1); const t = M.take(1, true); return t.w === undefined && W.wireOk(t) && !W.wireOk({ ...rec([]), p: 5 }) && !W.wireOk({}) && !W.wireOk({ ...rec([]), v: [1, 2] }); })();
  const odd = { keys: ['KeyW'], pressed: ['KeyE'], buttons: 1, clicked: 1, wheel: -0, mx: 3.25, my: -0, w: 1280, h: 720, pw: 2560, ph: 1440, view: { vp: [], ivp: [], pos: [0, 0, 0], forward: [0, 0, 0], target: [1.1, -0, 2.2], yaw: -0, pitch: 0.1 + 0.2, distance: 21, fov: 0.6, aspect: 16 / 9, aimLead: 0.18 } };
  const back = W.fromWire(W.netParse(W.netText(W.toWire(odd as never))) as never);
  const bits = Object.is(back.my, -0) && Object.is(back.view.yaw, -0) && Object.is(back.view.target[1], -0) && back.view.pitch === 0.1 + 0.2 && back.mx === 3.25 && back.pressed.join() === 'KeyE' && back.clicked === 1 && Number.isNaN(back.view.vp[0]);
  line(own && held && late && idle && once && far && wheel && bits, `the rule: a tick runs on the seat's own record in time${own ? '' : ' (NO)'}, a held one with its edges off when none came${held ? '' : ' (NO)'}, a late press, click and wheel land on the next tick and once${late ? '' : ' (NO)'}, an empty seat holds nothing and keeps its view${idle ? '' : ' (NO)'}, a second record for a tick is not taken${once ? '' : ' (NO)'}, one past the horizon is not kept${far ? '' : ' (NO)'}, a late wheel turn and the tick's own add up${wheel ? '' : ' (NO)'}, a record crosses the wire to the bit${bits ? '' : ' (NO)'}`);
}

// ---- the two cores against each other, on a clock of this check's own and a counter for a game
{
  const { NetServer } = await import(`${ROOT}/src/net/server.ts`), { NetClient } = await import(`${ROOT}/src/net/client.ts`), { NET, netText } = await import(`${ROOT}/src/net/wire.ts`);
  let now = 0, threw = '';
  const counter = () => ({ n: 0, step() { this.n++; }, word() { return String(this.n); } });
  const game = counter(), S = new NetServer({ now: () => now, step: () => game.step(), word: () => game.word(), ended: () => null }, { game: 'g', build: 'b1', recipe: { seed: 1, search: '?x', scenario: 'roomclear', env: null, knobs: null, tables: {} } as never, seatKeys: ['k'] });
  type Link = { conn: { send(t: string): number; close(): void }; C: InstanceType<typeof NetClient>; pump(): void };
  const links: Link[] = [];
  /** a client and its connection: what it sends reaches the server at once, what the server sends is read at the next pump; `drop` = the socket takes this send or not */
  const link = (key?: string, o: { build?: string; drop?: (n: number) => boolean } = {}): Link => {
    const inbox: string[] = [], host = counter(); let open = true, sends = 0;
    const conn = { send: (t: string) => { if (!open || o.drop?.(++sends)) return 0; inbox.push(t); return t.length; }, close: () => { open = false; } };
    const C = new NetClient((t: string) => S.message(conn, t), { game: 'g', build: o.build ?? 'b1', ...(key !== undefined ? { key } : {}) });
    const L = { conn, C, pump: () => { while (inbox.length) C.message(inbox.shift()!, now); C.walk(host, 1e9); } };
    links.push(L); C.hello(); L.pump(); return L;
  };
  const cmd = { keys: ['KeyW'], pressed: [], buttons: 0, clicked: 0, wheel: 0, mx: 640, my: 360, w: 1280, h: 720, pw: 1280, ph: 720, view: { vp: [], ivp: [], pos: [0, 0, 0], forward: [0, 0, 0], target: [0, 0, 0], yaw: 0, pitch: 0.96, distance: 21, fov: 0.6, aspect: 16 / 9, aimLead: 0 } } as never;
  let beats = 0;
  /** `ms` of the clock: the server advances every ms unless it is stalled, the seat beats at the rate unless its hands are off, everybody reads */
  const run = (ms: number, o: { beat?: boolean; server?: boolean } = {}) => { try { for (let i = 0; i < ms; i++) { now++; if (o.server !== false) S.advance(); for (const L of links) L.pump();
    const due = Math.floor(now * NET.hz / 1000); if (due > beats) { beats = due; if (o.beat !== false) A.C.beat(cmd, now); } } } catch (e) { threw ||= String((e as Error).stack ?? e); } };
  const ownOver = (ms: number, o?: { beat?: boolean; server?: boolean }) => { const o0 = S.lines()[0].own, t0 = S.tick; run(ms, o); return [S.lines()[0].own - o0, S.tick - t0]; };

  const A = link('k');
  run(500); const waited = S.state === 'waiting' && S.tick === 0 && A.C.welcome?.seat === 0;
  A.C.ready(); const [own1, n1] = ownOver(3000), plays = S.state === 'playing' && n1 >= 175 && own1 >= n1 - 10 && A.C.wordsOk === Math.floor(S.tick / NET.wordEvery) && !A.C.bad && A.C.margin <= NET.lead + 2;
  const got0 = S.lines()[0].got;
  for (const junk of [`{"k":"in","t":${S.tick + 5},"c":{}}`, `{"k":"in","t":${S.tick + 5},"c":{"k":5}}`, `{"k":"in","t":${S.tick - 1},"c":{"k":[],"p":5,"b":0,"m":[0,0],"s":[1,1,1,1],"v":[0,0,0,0,0,0,0,0,0]}}`, `{"k":"in","t":"x","c":null}`, '{"k":"in"', 'x'.repeat(NET.maxText + 1), `{"k":"in","t":${S.tick + 6},"c":{"k":[],"b":0,"m":[0,0],"s":[1,1,1,1],"v":[0,0,0,0,0,0,0,0,null]}}`])
    try { S.message(A.conn, junk); } catch (e) { threw ||= String(e); }
  run(300); const junked = !threw && S.lines()[0].got - got0 <= 20 && S.state === 'playing';   // (only the seat's own 18 or so records of those 300 ms were taken)
  const m0 = A.C.margin; run(3000, { beat: false }); const quiet = A.C.margin === m0;
  run(1000); const [own2, n2] = ownOver(1000), back = own2 >= n2 - 6;
  run(1000, { server: false }); const slips0 = S.slips; run(3000); const [own3, n3] = ownOver(1000), slipped = S.slips > slips0 || slips0 > 0, past = A.C.sendTick - S.tick, followed = own3 >= n3 - 6 && A.C.margin <= NET.lead + 6 && past <= NET.lead + 8;   // (in time is not enough: records aimed a second ahead are in time too, and a second late in the hand)
  run(Math.max(0, (2 * NET.tapeChunk + 100 - S.tick) * 1000 / NET.hz));   // (a joiner has more than two whole stretches to be sent)
  const B = link(undefined, { drop: n => n === 3 || n === 4 || n === 9 || (n > 20 && n % 7 === 0) }); run(4000);
  const whole = !B.C.broken && B.C.heard === S.tick && B.C.confirmed === S.tick && !B.C.bad && B.C.wordsOk === Math.floor(S.tick / NET.wordEvery);
  for (let i = 1; i < NET.watchers; i++) link();
  const fifth = link(), unnamed = link('k2', { build: '' }), capped = fifth.C.bye === 'no room to watch' && unnamed.C.bye.startsWith('this game is on build b1') && S.health().watchers === NET.watchers;
  S.message(B.conn, netText({ k: 'lost', t: 1, w: 'x' })); const deaf = S.lost === 0; S.message(A.conn, netText({ k: 'lost', t: 1, w: 'x' })); S.message(A.conn, netText({ k: 'lost', t: 2, w: 'y' })); const once = S.lost === 1;
  line(waited && plays && junked && quiet && back && slipped && followed && whole && capped && deaf && once,
    `the cores: no tick runs before the seat says it is ready${waited ? '' : ' (NO)'}; then ${own1} of ${n1} ticks ran on the seat's own record and every state word matched${plays ? '' : ' (NO)'}; seven messages that are not records were not taken and nothing threw${junked ? '' : ` (NO${threw ? `: ${threw.split('\n')[0]}` : ''})`}; three seconds with nothing sent left its aim at ${m0} ticks${quiet ? '' : ` (NO: ${A.C.margin})`} and its records were in time again after (${own2} of ${n2})${back ? '' : ' (NO)'}; a server stalled a second let ${S.slips} ticks slip${slipped ? '' : ' (NO)'} and three seconds on ${own3} of ${n3} ticks ran on the seat's own record, sent ${past} ticks past the server's${followed ? '' : ' (NO)'}; a watcher joining at tick ${2 * NET.tapeChunk + 100} over a socket that dropped messages heard all ${B.C.heard} of ${S.tick} ticks in order and matched ${B.C.wordsOk} words${whole ? '' : ` (NO${B.C.broken ? `: ${B.C.broken}` : ''})`}; a watcher too many and a client of no named build were turned away${capped ? '' : ` (NO: "${fifth.C.bye}", "${unnamed.C.bye}")`}; a watcher's "lost" is not counted and a seat's is counted once${deaf && once ? '' : ' (NO)'}`);
}

// ---- the processes
type Proc = { stdout: ReadableStream; stderr: ReadableStream; exited: Promise<number>; kill(): void };
const spawn = (cmd: string[], env: Record<string, string> = {}): Proc => Bun.spawn(cmd, { stdout: 'pipe', stderr: 'pipe', cwd: ROOT, env: { ...process.env, ...env } }) as unknown as Proc;
const text = (s: ReadableStream) => new Response(s).text();
interface BotOut { seat: number; confirmed: number; heard: number; wordsOk: number; bad: { t: number; server: string; mine: string } | null; word: string; sent: number; holds: number; skips: number; margin: number; lead: number; broken: string; moved: number; shots: number; bye: string; state: string; stoodMs: number; tickMs: number }
async function bot(port: number, extra: string[]): Promise<BotOut | null> {
  const p = spawn([process.execPath, 'run', 'tools/mp/bot.ts', '--url', `ws://127.0.0.1:${port}`, '--game', 'check', ...extra]);
  const [out, err] = await Promise.all([text(p.stdout), text(p.stderr), p.exited]);
  const l = out.split('\n').find(x => x.startsWith('MPBOT '));
  if (!l) { console.log(`  (a client printed no result: ${(err || out).trim().split('\n').slice(-2).join(' / ')})`); return null; }
  return JSON.parse(l.slice(6)) as BotOut;
}
const health = async (port: number): Promise<any> => { try { return await (await fetch(`http://127.0.0.1:${port}/health`)).json(); } catch { return null; } };
async function up(port: number, tries = 150): Promise<any> { for (let i = 0; i < tries; i++) { const h = await health(port); if (h) return h; await sleep(100); } return null; }
/** one hello on a raw socket: the server's answer ('welcome', or the bye's words) */
function knock(port: number, hello: object): Promise<string> {
  return new Promise(done => { const ws = new WebSocket(`ws://127.0.0.1:${port}`); let said = '';
    ws.onopen = () => ws.send(JSON.stringify(hello));
    ws.onmessage = (ev: { data: unknown }) => { const m = JSON.parse(String(ev.data)); if (m.k === 'welcome') { said = 'welcome'; ws.close(); } else if (m.k === 'bye') said = `bye: ${m.why}`; };
    ws.onclose = () => done(said || 'closed with no word'); ws.onerror = () => done(said || 'no connection');
    setTimeout(() => { try { ws.close(); } catch { /* gone */ } done(said || 'no answer'); }, 5000); });
}

// the stand-in control plane: counts the heartbeats
const beats: any[] = [];
const control = Bun.serve({ port: 0, hostname: '127.0.0.1', async fetch(req: Request) { try { beats.push(await req.json()); } catch { /* not a heartbeat */ } return new Response('ok'); } });
const PORT = 5400 + Math.floor(performance.now() * 7 + process.pid) % 400, PORT2 = PORT + 1;
const envOf = (port: number): Record<string, string> => ({ MP_PORT: String(port), MP_GAME: 'check', MP_SETUP: setupArg, MP_SEATKEYS: 'seat-one', MP_IDLE_S: '3', MP_SECRET: 'the-secret', MP_CONTROL: `http://127.0.0.1:${control.port}/beat` });

const srv = spawn([process.execPath, 'run', 'tools/mp/server.ts'], envOf(PORT));
const srvOut = text(srv.stdout), srvErr = text(srv.stderr);
const h0 = await up(PORT);
if (!h0) { srv.kill(); console.log(`mpserver-check: the server did not come up: ${(await srvErr).trim().split('\n').slice(-3).join(' / ')}`); process.exit(2); }
line(h0.state === 'waiting' && h0.tick === 0 && h0.seats.length === 1 && h0.seats[0] === false, `stood: the server made its recipe, stood its game and answers /health — ${h0.state}, tick ${h0.tick}, ${h0.seats.length} seat${h0.seats.length === 1 ? '' : 's'}, build ${h0.build || '(none)'}`);

{ const B = h0.build as string, wrongKey = await knock(PORT, { k: 'hello', proto: 1, build: B, game: 'check', key: 'not-a-key' }), wrongGame = await knock(PORT, { k: 'hello', proto: 1, build: B, game: 'another' }), wrongWire = await knock(PORT, { k: 'hello', proto: 99, build: B, game: 'check' }), wrongBuild = await knock(PORT, { k: 'hello', proto: 1, build: 'not-this-build', game: 'check' });
  const ok = wrongKey.startsWith('bye: no seat') && wrongGame.startsWith('bye: this server runs another') && wrongWire.startsWith('bye: this server speaks') && (!h0.build || wrongBuild.startsWith('bye: this game is on build'));
  const still = await health(PORT);
  line(ok && still?.state === 'waiting' && still.tick === 0, `refused: a wrong seat key ("${wrongKey}"), another game's id ("${wrongGame}"), another wire ("${wrongWire}")${h0.build ? ` and another build ("${wrongBuild}")` : ''} are each told why and let go; the server still waits at tick ${still?.tick}`); }

const N = 1200;
const [seat, watcher, joiner] = await Promise.all([
  bot(PORT, ['--key', 'seat-one', '--ticks', String(N), ...(BREAK ? ['--break'] : [])]),
  bot(PORT, ['--ticks', String(N)]),
  bot(PORT, ['--ticks', String(N), '--after', '6000']),
]);
const words = N / 30, clean = (b: BotOut | null) => !!b && b.confirmed >= N && !b.bad && b.wordsOk >= words && !b.broken;
line(clean(seat) && clean(watcher) && seat!.word === watcher!.word && seat!.moved > 1 && seat!.shots > 0,
  `same fight: over ${N} ticks the seated client matched ${seat?.wordsOk ?? 0} of ${words} state words${seat?.bad ? ` (FIRST DIFFERENT after tick ${seat.bad.t}: server ${seat.bad.server}, its own ${seat.bad.mine})` : ''} and the watcher ${watcher?.wordsOk ?? 0}${watcher?.bad ? ` (first different after tick ${watcher.bad.t})` : ''}; both end on ${seat?.word}${seat && watcher && seat.word !== watcher.word ? ` ≠ ${watcher.word}` : ''}; the seat's man moved ${seat?.moved ?? 0} m and fired ${seat?.shots ?? 0} time${seat?.shots === 1 ? '' : 's'} (${seat?.sent ?? 0} records sent, a tick ${seat?.tickMs ?? 0} ms here)`);
line(clean(joiner) && joiner!.word === watcher?.word, `late joiner: a watcher that connected six seconds in was sent the run so far and matched ${joiner?.wordsOk ?? 0} of ${words} words, ending on ${joiner?.word}`);
const hEnd = await health(PORT);
if (hEnd) console.log(`  (the server: tick ${hEnd.tick}, ${hEnd.slips} tick${hEnd.slips === 1 ? '' : 's'} slipped, the seat's records: ${hEnd.lines[0].got} taken, ${hEnd.lines[0].late} late, ${hEnd.lines[0].own} ticks on its own record, ${hEnd.lines[0].held} on a held one)`);

// ---- idle: everybody has left; the server ends itself
const code = await Promise.race([srv.exited, sleep(12000).then(() => -1)]);
if (code === -1) srv.kill();
line(code === 0, `idle: with everybody gone the server ended itself${code === 0 ? '' : ` — NOT inside 12 s (idle time 3 s)${code > 0 ? `, exit ${code}` : ''}`}, exit ${code === -1 ? 'never' : code}${code === 0 ? ` ("${(await srvOut).trim().split('\n').pop()?.replace(/^\[[^\]]*\] /, '')}")` : ''}`);
{ const mine = beats.filter(b => b.game === 'check' && b.secret === 'the-secret'), states = mine.map(b => b.done ? 'done' : b.state), firstPlay = states.indexOf('playing');
  line(mine.length >= 3 && states[0] === 'waiting' && firstPlay > 0 && states[states.length - 1] === 'done' && mine.every(b => typeof b.tick === 'number' && Array.isArray(b.seats)),
    `control: ${mine.length} heartbeat${mine.length === 1 ? '' : 's'} reached the stand-in control plane with the game's secret: ${[...new Set(states)].join(', then ')}`); }

// ---- lag: its own server (a game is one run)
beats.length = 0;
const srv2 = spawn([process.execPath, 'run', 'tools/mp/server.ts'], envOf(PORT2));
const srv2Err = text(srv2.stderr); void text(srv2.stdout);
if (!await up(PORT2)) { srv2.kill(); console.log(`mpserver-check: the second server did not come up: ${(await srv2Err).trim().split('\n').slice(-3).join(' / ')}`); process.exit(2); }
const [far, near] = await Promise.all([bot(PORT2, ['--key', 'seat-one', '--ticks', String(N), '--lag', '60', '--jitter', '40', ...(BREAK ? ['--break'] : [])]), bot(PORT2, ['--ticks', String(N)])]);
const h2 = await health(PORT2);
line(clean(far) && clean(near) && far!.word === near!.word && !!h2 && h2.lines[0].own > N / 2,
  `lag: a seat 60 to 100 ms away each way matched ${far?.wordsOk ?? 0} of ${words} words${far?.bad ? ` (FIRST DIFFERENT after tick ${far.bad.t})` : ''} and a watcher beside the server ${near?.wordsOk ?? 0}; of the seat's ${h2?.lines[0].got ?? 0} records ${h2?.lines[0].late ?? 0} came late, ${h2?.lines[0].own ?? 0} ticks ran on its own record and ${h2?.lines[0].held ?? 0} on a held one; it aims ${far?.margin ?? 0} ticks past the server's`);
await Promise.race([srv2.exited, sleep(12000)]); srv2.kill();
control.stop(true);

console.log(fails ? `mpserver-check: ${fails} line${fails === 1 ? '' : 's'} FAILED${BREAK ? ' (MPSERVER_BREAK=1: the state-word lines must)' : ''}` : 'mpserver-check: every line passed');
process.exit(fails ? 1 : 0);
