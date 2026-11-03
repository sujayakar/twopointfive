// THE GAME SERVER, CHECKED (src/net/, tools/mp/; docs/agents/net.md): a server process and scripted clients over REAL WebSockets on this machine, plus the
// input rule on its own. Not game code: it starts processes and reads what they print.
//   bun run tools/qa/mpserver-check.ts                    every line below
//   MPSERVER_BREAK=1 bun run tools/qa/mpserver-check.ts   the red path: the seated client flips the last bit of its own man's place after tick 90 — the lines
//                                                         that hold its state words against the server's must FAIL
//   --setup '{"scenario":"?rescue","seed":3}'             another mission (default: the four-man room clear in the office, seed 7)
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
//   lag           a second server, started with the setup record as a control plane hands it over (its seat key, build and game id in the record), its seated
//                 client 60 to 100 ms away each way: every word still the server's, and how its records arrived is printed
//   gone          the stand-in control plane then answers that server's heartbeats 410: it says 'ended' and stops, exit 0
//   two seats     a third server, a room clear of TWO players (`seats=2` in the setup's address, two seat keys): each seated client plays its own man, one of them
//                 60 to 100 ms away, and both match every state word and end on the same one
//   ahead         both of those clients look ahead for their own man each beat (src/net/foresee.ts, as the page does): the look had him moved on the beat his
//                 first record went out, before the game had; held against the game once it reached the same tick, the looks were right to the bar below;
//                 the game they left is still the server's (the line above); what a look cost is printed. Under MPSERVER_BREAK the second client's look
//                 plays no record, and this line must FAIL too
//   worse way     the queue a worse connection is made of (src/net/delay.ts, with a die this check throws): all zero = straight through; under jitter nothing
//                 passes the message in front of it; a lost message is late by a round trip and the resend's wait, and holds the ones behind it; a stall holds everything to its end,
//                 what was on its way too, and one taken back is over at once; a handler that throws loses its own message and strands none behind it
//   bad connection  a fourth server, two seats: one seat on bad wifi (delay, jitter, 3 in 100 resent, one stall of 1.5 s) and one beside the server both
//                 match every state word and end on the same one; the stall was really felt and the server really ran ticks without that seat's record
//   control       the heartbeats reached the stand-in control plane with the game's secret: waiting, then playing, then done
//   refused       a hello with a wrong seat key, another game's id, another wire or (when the server names its build) another build is told why and let go
//   idle          with everybody gone the server ended itself inside its idle time, exit 0
// EXIT 0 = every line passed · 1 = a line failed · 2 = a process could not be started.
import { resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.MPSERVER_BREAK === '1';
const args = process.argv.slice(2), setupArg = args.includes('--setup') ? args[args.indexOf('--setup') + 1] : '{"scenario":"?roomclear&squad4","seed":7}';
let fails = 0;
// THE LOOK AHEAD'S BAR: of a client's CLEAN looks, half or more are right to a millimetre where the game then has the man (asked of a client with ten or
// more). Clean (tools/mp/bot.ts): the server ran every record of it as sent, his legs stayed his own on every tick it crossed, and nobody stood within
// 1.5 m of him. A look is exact when nothing it cannot know happened inside it, and that is most of them (a run's clean looks are often ALL exact). The
// rest are corrections the page eases out: the walk ahead holds the world where the tick has it, and one thing that moves under his cursor or against
// his stride is wrong in every look that crosses it, ten or twenty in a row. So the bar is a COUNT and not an average: it was 'under 5 mm off on
// average' until 2026-11-02, and one such stretch at 20 cm put a run of a hundred exact looks over it one run in three. A look ahead that walks him
// wrong is wrong in every look, and fails this as it failed that.
const AHEAD_EXACT = 0.5;
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
  const fifth = link(), unnamed = link('k2', { build: '' }), capped = fifth.C.byeCode === 'full' && unnamed.C.byeCode === 'build' && unnamed.C.bye.startsWith('this game is on build b1') && S.health().watchers === NET.watchers;
  S.message(B.conn, netText({ k: 'lost', t: 1, w: 'x' })); const deaf = S.lost === 0; S.message(A.conn, netText({ k: 'lost', t: 1, w: 'x' })); S.message(A.conn, netText({ k: 'lost', t: 2, w: 'y' })); const once = S.lost === 1;
  line(waited && plays && junked && quiet && back && slipped && followed && whole && capped && deaf && once,
    `the cores: no tick runs before the seat says it is ready${waited ? '' : ' (NO)'}; then ${own1} of ${n1} ticks ran on the seat's own record and every state word matched${plays ? '' : ' (NO)'}; seven messages that are not records were not taken and nothing threw${junked ? '' : ` (NO${threw ? `: ${threw.split('\n')[0]}` : ''})`}; three seconds with nothing sent left its aim at ${m0} ticks${quiet ? '' : ` (NO: ${A.C.margin})`} and its records were in time again after (${own2} of ${n2})${back ? '' : ' (NO)'}; a server stalled a second let ${S.slips} ticks slip${slipped ? '' : ' (NO)'} and three seconds on ${own3} of ${n3} ticks ran on the seat's own record, sent ${past} ticks past the server's${followed ? '' : ' (NO)'}; a watcher joining at tick ${2 * NET.tapeChunk + 100} over a socket that dropped messages heard all ${B.C.heard} of ${S.tick} ticks in order and matched ${B.C.wordsOk} words${whole ? '' : ` (NO${B.C.broken ? `: ${B.C.broken}` : ''})`}; a watcher too many and a client of no named build were turned away${capped ? '' : ` (NO: "${fifth.C.bye}", "${unnamed.C.bye}")`}; a watcher's "lost" is not counted and a seat's is counted once${deaf && once ? '' : ' (NO)'}`);
}

// ---- the processes
type Proc = { stdout: ReadableStream; stderr: ReadableStream; exited: Promise<number>; kill(): void };
const spawn = (cmd: string[], env: Record<string, string> = {}): Proc => Bun.spawn(cmd, { stdout: 'pipe', stderr: 'pipe', cwd: ROOT, env: { ...process.env, ...env } }) as unknown as Proc;
const text = (s: ReadableStream) => new Response(s).text();
interface Ahead { first: number; firstMoved: boolean; n: number; mean: number; max: number; exact: number; unfree: number; clean: number; cleanMean: number; cleanMax: number; cleanExact: number; worst: { t: number; mm: number; records: number; near: number } | null; fixes: number; fixMax: number; records: number; ms: number }
interface BotOut { ahead?: Ahead; seat: number; confirmed: number; heard: number; wordsOk: number; bad: { t: number; server: string; mine: string } | null; word: string; sent: number; holds: number; skips: number; margin: number; lead: number; broken: string; stalled: number; moved: number; shots: number; bye: string; state: string; stoodMs: number; tickMs: number }
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
const beats: any[] = []; let gone = false;
const control = Bun.serve({ port: 0, hostname: '127.0.0.1', async fetch(req: Request) { try { beats.push(await req.json()); } catch { /* not a heartbeat */ } return gone ? new Response('{"ok":false}', { status: 410 }) : new Response('{"ok":true}'); } });
const PORT = 5400 + Math.floor(performance.now() * 7 + process.pid) % 400, PORT2 = PORT + 1, PORT3 = PORT + 2, PORT4 = PORT + 3;
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
const words = N / 30, clean = (b: BotOut | null, n = N) => !!b && b.confirmed >= n && !b.bad && b.wordsOk >= n / 30 && !b.broken;
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
// (the record as a control plane hands it over: the game's id, the build and the seat keys in it, no MP_GAME and no MP_SEATKEYS)
const record = JSON.stringify({ game: 'check', build: h0.build, ...JSON.parse(setupArg), knobs: {}, seats: [{}], seatKeys: ['seat-one'] });
const env2: Record<string, string> = { ...envOf(PORT2), MP_SETUP: record, MP_IDLE_S: '60' }; delete env2.MP_GAME; delete env2.MP_SEATKEYS;
const srv2 = spawn([process.execPath, 'run', 'tools/mp/server.ts'], env2);
const srv2Err = text(srv2.stderr); void text(srv2.stdout);
if (!await up(PORT2)) { srv2.kill(); console.log(`mpserver-check: the second server did not come up: ${(await srv2Err).trim().split('\n').slice(-3).join(' / ')}`); process.exit(2); }
const [far, near] = await Promise.all([bot(PORT2, ['--key', 'seat-one', '--ticks', String(N), '--lag', '60', '--jitter', '40', ...(BREAK ? ['--break'] : [])]), bot(PORT2, ['--ticks', String(N)])]);
const h2 = await health(PORT2);
line(clean(far) && clean(near) && far!.word === near!.word && !!h2 && h2.lines[0].got > N / 4 && h2.lines[0].own > h2.lines[0].got / 2,   // (of the records that CAME, not of the run's ticks: how many a starved box lets the bot send is the box's)
  `lag: a seat 60 to 100 ms away each way matched ${far?.wordsOk ?? 0} of ${words} words${far?.bad ? ` (FIRST DIFFERENT after tick ${far.bad.t})` : ''} and a watcher beside the server ${near?.wordsOk ?? 0}; of the seat's ${h2?.lines[0].got ?? 0} records ${h2?.lines[0].late ?? 0} came late, ${h2?.lines[0].own ?? 0} ticks ran on its own record and ${h2?.lines[0].held ?? 0} on a held one; it aims ${far?.margin ?? 0} ticks past the server's`);
gone = true;   // (the bots have left: the seat's leaving is a change, its heartbeat is answered 410)
const code2 = await Promise.race([srv2.exited, sleep(15000).then(() => -1)]); if (code2 === -1) srv2.kill();
{ const last = beats.filter(b => b.game === 'check').pop();
  line(code2 === 0 && last?.state === 'ended' && /no such game/.test(last?.done ?? ''), `gone: told its game is gone (a heartbeat answered 410) the server stopped${code2 === 0 ? '' : ' — NOT inside 15 s'}, exit ${code2 === -1 ? 'never' : code2}; its last heartbeat said ${last?.state ?? 'nothing'} ("${last?.done ?? ''}")`); }

// ---- two seats: its own server, a room clear of two players; both clients look ahead for their own man
gone = false;   // (the stand-in control plane knows this game again)
{ const N2 = 600, env3: Record<string, string> = { ...envOf(PORT3), MP_SETUP: '{"scenario":"?roomclear&squad4&seats=2","seed":7}', MP_SEATKEYS: 'seat-one,seat-two' };
  const srv3 = spawn([process.execPath, 'run', 'tools/mp/server.ts'], env3); const srv3Err = text(srv3.stderr); void text(srv3.stdout);
  const h3 = await up(PORT3);
  if (!h3) { srv3.kill(); console.log(`mpserver-check: the two seat server did not come up: ${(await srv3Err).trim().split('\n').slice(-3).join(' / ')}`); process.exit(2); }
  const [one, two] = await Promise.all([
    bot(PORT3, ['--key', 'seat-one', '--ticks', String(N2), '--predict', '--lag', '60', '--jitter', '40', ...(BREAK ? ['--break'] : [])]),
    bot(PORT3, ['--key', 'seat-two', '--ticks', String(N2), '--predict', ...(BREAK ? ['--no-ahead'] : [])])]);
  const h3e = await health(PORT3), L3 = (h3e?.lines ?? []) as { got: number; own: number }[];
  line(h3.seats.length === 2 && clean(one, N2) && clean(two, N2) && one!.word === two!.word && one!.seat === 0 && two!.seat === 1 && one!.moved > 1 && two!.moved > 1 && L3.length === 2 && L3.every(l => l.own > 0),
    `two seats: a room clear of two players, each client in its own seat${h3.seats.length === 2 ? '' : ` (the server stood ${h3.seats.length} seat)`}: over ${N2} ticks the first, 60 to 100 ms away, matched ${one?.wordsOk ?? 0} of ${N2 / 30} state words${one?.bad ? ` (FIRST DIFFERENT after tick ${one.bad.t})` : ''} and the second ${two?.wordsOk ?? 0}${two?.bad ? ` (first different after tick ${two.bad.t})` : ''}; they end on ${one?.word}${one && two && one.word !== two.word ? ` ≠ ${two.word}` : ''}; their men went ${one?.moved ?? 0} m and ${two?.moved ?? 0} m; the server ran ${L3.map(l => l.own).join(' and ')} ticks on the seats' own records`);
  const A = [one?.ahead, two?.ahead], said = (a: Ahead | undefined) => a ? `moved ${a.first < 0 ? 'NEVER' : a.first === 0 ? 'on the beat of his first record' : `${a.first} beat${a.first === 1 ? '' : 's'} after his first record`}${a.firstMoved ? ' (the game had moved him already)' : ''}, ${a.n} looks held against the game: ${(a.mean * 1000).toFixed(1)} mm off on average, ${(a.max * 1000).toFixed(0)} mm at most, ${a.exact} within a millimetre${a.unfree ? ` (and ${a.unfree} looks walked nothing: his legs were not his own)` : ''}; ${a.clean} of them CLEAN (every record run as sent, his legs his own all the way, nobody within 1.5 m of him): ${a.cleanExact} right to a millimetre, ${(a.cleanMean * 1000).toFixed(1)} mm off on average, ${(a.cleanMax * 1000).toFixed(0)} mm at most${a.worst && a.worst.mm >= 1 ? ` (the worst after tick ${a.worst.t}: ${a.worst.records} records on, the nearest body ${a.worst.near} m off)` : ''}; ${a.fixes} correction${a.fixes === 1 ? '' : 's'} past a centimetre (the longest ${(a.fixMax * 1000).toFixed(0)} mm); ${a.records} records a look, ${a.ms} ms each` : 'no look ahead reported';
  line(A.every(a => !!a && a.first >= 0 && a.first <= 1 && !a.firstMoved && a.n >= 20) && A.some(a => a!.clean >= 20) && A.every(a => a!.clean < 10 || a!.cleanExact >= a!.clean * AHEAD_EXACT),
    `ahead: each client's own man by its look ahead — the first (60 to 100 ms away): ${said(A[0])} · the second (beside the server): ${said(A[1])} · the bar: half or more of a client's clean looks right to a millimetre (asked of a client with ten or more; one of the two must have twenty)`);
  const c3 = await Promise.race([srv3.exited, sleep(12000).then(() => -1)]); if (c3 === -1) srv3.kill(); }

// ---- a worse connection (src/net/delay.ts): the queue's own rules with a die this check throws, then a seat that plays over a bad one
{ const { onWay, wayOff } = await import('../../src/net/delay.ts');
  const run = async (W: ReturnType<typeof wayOff>, rolls: number[], n: number, gapMs: number, mid?: (i: number) => void) => {
    const out: { i: number; at: number }[] = [], sentAt: number[] = []; let r = 0; const t0 = performance.now();
    const put = onWay<number>(W, i => out.push({ i, at: performance.now() - t0 }), () => rolls[r++ % rolls.length]);
    for (let i = 0; i < n; i++) { mid?.(i); sentAt.push(performance.now() - t0); put(i); if (gapMs) await sleep(gapMs); }
    const sync = out.length; await sleep(700); return { out, sentAt, sync };
  };
  const off = await run(wayOff(), [0.5], 20, 0);
  // (jitter by a die that swings from none to all of it: a later message is due BEFORE the one in front of it, and must not pass it)
  const jit = await run({ ...wayOff(), lagMs: 20, jitterMs: 80 }, [0.95, 0.05, 0.6, 0.0], 40, 2);
  // (loss: the die loses message 10 alone — rolls come in pairs, the jitter's then the loss's)
  const lossRolls: number[] = []; for (let i = 0; i < 90; i++) lossRolls.push(0, i === 10 ? 0 : 0.99);
  const los = await run({ ...wayOff(), lagMs: 10, lossPct: 50, resendMs: 180 }, lossRolls, 90, 4);
  // (a stall: what is already on its way waits too — message 9, sent a moment before it — and one taken back early is over at once)
  const W4 = { ...wayOff(), lagMs: 10 }; const stl = await run(W4, [0], 130, 4, i => { if (i === 10) W4.stallUntil = performance.now() + 300; });
  const W5 = { ...wayOff(), lagMs: 10 }; const lift = await run(W5, [0], 130, 4, i => { if (i === 10) W5.stallUntil = performance.now() + 5000; if (i === 50) W5.stallUntil = 0; });
  // (a handler that throws: its own message is gone with it and the ones behind still come, with nothing more sent to wake the queue. The throw comes out
  // of the queue's timer, so nobody is there to catch it but the process: it is caught here for these 300 ms and counted)
  const thr = { got: [] as number[], seen: 0, other: '' };
  { const W6 = { ...wayOff(), lagMs: 20 }, put = onWay<number>(W6, i => { if (i === 1) throw new Error('thrown on purpose by the worse way line'); thr.got.push(i); }, () => 0);
    const see = (e: unknown): void => { if (/thrown on purpose/.test(String(e))) thr.seen++; else thr.other ||= String(e).slice(0, 160); };
    process.on('uncaughtException', see); for (let i = 0; i < 4; i++) put(i); await sleep(300); process.off('uncaughtException', see); }
  const thrOk = thr.seen === 1 && thr.got.join() === '0,2,3' && !thr.other;
  const inOrder = (o: { i: number }[], n: number) => o.length === n && o.every((m, k) => m.i === k);
  const late = (x: { out: { i: number; at: number }[]; sentAt: number[] }, i: number) => x.out[i] ? x.out[i].at - x.sentAt[i] : -1;
  const jitOk = inOrder(jit.out, 40) && jit.out.every((m, k) => m.at - jit.sentAt[k] >= 19);
  const losOk = inOrder(los.out, 90) && late(los, 9) < 120 && late(los, 10) >= 200 && late(los, 11) >= 190 && late(los, 89) < 120;
  const stlOk = inOrder(stl.out, 130) && late(stl, 5) < 120 && late(stl, 9) >= 250 && late(stl, 10) >= 290 && late(stl, 129) < 120;
  const liftOk = inOrder(lift.out, 130) && late(lift, 10) >= 100 && late(lift, 10) < 1500 && late(lift, 129) < 120;
  line(off.sync === 20 && inOrder(off.out, 20) && jitOk && losOk && stlOk && liftOk && thrOk,
    `worse way: with every number at zero ${off.sync} of 20 messages went straight through; under jitter 40 came out ${inOrder(jit.out, 40) ? 'in the order they went in' : 'OUT OF ORDER'}, none sooner than its delay; a lost message came ${late(los, 10).toFixed(0)} ms late and the one behind it ${late(los, 11).toFixed(0)} ms (it waits for the resend), the last ${late(los, 89).toFixed(0)} ms; a stall of 300 ms held message 10 for ${late(stl, 10).toFixed(0)} ms and message 9, already on its way, for ${late(stl, 9).toFixed(0)} ms, and the last came ${late(stl, 129).toFixed(0)} ms late; a stall of 5 s taken back early let message 10 go after ${late(lift, 10).toFixed(0)} ms; of four messages whose second made the handler throw, ${thr.got.length} came (${thr.got.join(', ') || 'none'}: ${thrOk ? 'the rest, by the queue\'s own timer' : 'THE ONES BEHIND IT WERE LEFT WAITING'}) and the throw was seen ${thr.seen} time${thr.seen === 1 ? '' : 's'}${thr.other ? ` — ANOTHER ERROR CAME UP MEANWHILE: ${thr.other}` : ''}`);
  const N4 = 600, env4: Record<string, string> = { ...envOf(PORT4), MP_SETUP: '{"scenario":"?roomclear&squad4&seats=2","seed":7}', MP_SEATKEYS: 'seat-one,seat-two' };
  const srv4 = spawn([process.execPath, 'run', 'tools/mp/server.ts'], env4); const srv4Err = text(srv4.stderr); void text(srv4.stdout);
  if (!await up(PORT4)) { srv4.kill(); console.log(`mpserver-check: the bad connection's server did not come up: ${(await srv4Err).trim().split('\n').slice(-3).join(' / ')}`); process.exit(2); }
  const [bad, good] = await Promise.all([
    bot(PORT4, ['--key', 'seat-one', '--ticks', String(N4), '--predict', '--lag', '15', '--jitter', '60', '--loss', '3', '--stall', '1500', ...(BREAK ? ['--break'] : [])]),
    bot(PORT4, ['--key', 'seat-two', '--ticks', String(N4), '--predict'])]);
  const h4 = await health(PORT4), L4 = h4?.lines[0] as { got: number; late: number; held: number } | undefined;
  line(clean(bad, N4) && clean(good, N4) && bad!.word === good!.word && bad!.stalled >= 1400 && !!L4 && L4.held > 60 && bad!.moved > 1,
    `bad connection: a seat on bad wifi (15 to 75 ms each way, 3 in 100 messages resent, and once nothing at all for 1.5 s) matched ${bad?.wordsOk ?? 0} of ${N4 / 30} state words${bad?.bad ? ` (FIRST DIFFERENT after tick ${bad.bad.t})` : ''} and the seat beside the server ${good?.wordsOk ?? 0}; they end on ${bad?.word}${bad && good && bad.word !== good.word ? ` ≠ ${good.word}` : ''}; its longest wait for the server was ${bad?.stalled ?? 0} ms; of its ${L4?.got ?? 0} records ${L4?.late ?? 0} came late and the server ran ${L4?.held ?? 0} ticks on a held one; its man still went ${bad?.moved ?? 0} m${bad?.broken ? ` — BROKEN: ${bad.broken}` : ''}`);
  const c4 = await Promise.race([srv4.exited, sleep(12000).then(() => -1)]); if (c4 === -1) srv4.kill(); }
control.stop(true);

console.log(fails ? `mpserver-check: ${fails} line${fails === 1 ? '' : 's'} FAILED${BREAK ? ' (MPSERVER_BREAK=1: the state-word lines must)' : ''}` : 'mpserver-check: every line passed');
process.exit(fails ? 1 : 0);
