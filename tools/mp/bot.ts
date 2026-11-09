// A SCRIPTED CLIENT OF A GAME SERVER (bun; a process of its own: it stands a game). It connects, stands its game from the server's start, sends a seat's records
// when it has a seat's key, walks every confirmed tick and holds its state word against the server's. What tools/qa/mpserver-check.ts runs two and three of.
//   bun run tools/mp/bot.ts --url ws://127.0.0.1:5300 --game local --key k0 --ticks 1200
//   --key K        sit in the seat with that key (none: watch)            --ticks N     leave once N ticks are walked (default 600)
//   --lag MS       hold every message, both ways, that long (default 0)   --jitter MS   and up to this much longer, by its own seeded die
//   --loss PCT     that share of messages waits for a resend, and holds those behind it (src/net/delay.ts wayMs)
//   --stall MS     once, after tick 150: nothing passes either way for that long (`stalled` in the result: the longest wait between two messages down, ms)
//   --after MS     connect that long after starting (a late joiner)       --break       after tick 90, step one record with another walk key (the red path)
//   --build NAME   the build it says it is (default: this checkout's, as the server names its own)
//   --name NAME    what its player calls himself (the hello's `name`; `names` in the result: every seat's as the server last said them)
//   --predict      look ahead for its own man each beat, as the page does (src/net/foresee.ts), and say how that went: `ahead` in the result —
//                  `first` the beats from its first record sent to a look ahead that had him moved (and whether the game had moved him by then), `n` looks
//                  held against the tick they ended on once the game got there, `mean` and `max` how far off they were (m), `exact` how many within a
//                  millimetre, `fixes` and `fixMax` the corrections, `records` the records a look played (mean), `ms` what one cost
//   --no-ahead     (the red path of that) the look ahead plays no record
//   --why          with --predict: every clean look that was off by a millimetre or more, a line each on stderr (where, by how much, his state)
// Prints one line: MPBOT {seat, confirmed, heard, wordsOk, bad, word, watchOk, watchOff, watchFirst (the whole game's word, watched: src/net/words.ts foldWhole), sent, holds, skips, margin, lead, moved, shots, bye, stoodMs, tickMs}. Exit 0 = it walked its ticks.
// (`moved`: the length of the way its man went, m, tick by tick.)
import { NET, type WireCmd } from '../../src/net/wire.ts';
import { NetClient } from '../../src/net/client.ts';
import { Foresee, FORESEE } from '../../src/net/foresee.ts';
import { onWay, wayOff } from '../../src/net/delay.ts';
import { buildName, standStart, type Stood } from './stand.ts';

const A = process.argv.slice(2), arg = (k: string, d = ''): string => { const i = A.indexOf(k); return i >= 0 ? A[i + 1] ?? '' : d; };
const url = arg('--url', 'ws://127.0.0.1:5300'), game = arg('--game', 'local'), key = A.includes('--key') ? arg('--key') : undefined, want = Number(arg('--ticks', '600'));
const lag = Number(arg('--lag', '0')), jitter = Number(arg('--jitter', '0')), loss = Number(arg('--loss', '0')), stall = Number(arg('--stall', '0')), after = Number(arg('--after', '0')), BREAK = A.includes('--break'), PREDICT = A.includes('--predict'), WHY = A.includes('--why');
FORESEE.on = PREDICT; if (A.includes('--no-ahead')) FORESEE.max = 0;

let die = 0x2545f491; const roll = () => { die = (Math.imul(die, 1664525) + 1013904223) >>> 0; return die / 4294967296; };
/** this client's way to the server, as the page's own (src/net/delay.ts: one queue a direction, never out of order; its die is the bot's seeded one) */
const WAY = { ...wayOff(), lagMs: lag, jitterMs: jitter, lossPct: loss };

if (after > 0) await new Promise(r => setTimeout(r, after));
const ws = new WebSocket(url);
const client = new NetClient(onWay(WAY, (text: string) => { if (ws.readyState === 1) ws.send(text); }, roll), { game, build: A.includes('--build') ? arg('--build') : buildName(), ...(key !== undefined ? { key } : {}), ...(A.includes('--name') ? { name: arg('--name') } : {}) });
let stood: Stood | null = null, standing = false, stoodMs = 0, closed = false;
const down: (f: () => void) => void = onWay(WAY, (f: () => void) => f(), roll);   // (what comes down the socket, its close too, in the order it came)
/** the longest wait between two messages down once the game ran (ms), and when the last one came */
let gapMax = 0, gapAt = 0, stalledOnce = false;
const take = (text: string): void => down(() => { const t = performance.now(); if (gapAt && client.confirmed > 60) gapMax = Math.max(gapMax, t - gapAt); gapAt = t; client.message(text, t); if (client.welcome && !stood && !standing) { standing = true; const t0 = performance.now(); standStart(client.welcome.start).then(s => { stood = s; stoodMs = performance.now() - t0; if (client.seat > 0) VIEW = { yaw: 1.7, pitch: 0.9, distance: 33, fov: 0.6, w: 1920, h: 1080 }; if (client.seat >= 0) { s.show(client.seat); client.ready(); } }, e => { console.error(`bot.ts: ${(e as Error).message}`); process.exit(1); }); } });
ws.onopen = () => client.hello();
ws.onmessage = (ev: { data: unknown }) => take(String(ev.data));
ws.onclose = () => down(() => { closed = true; });
ws.onerror = () => down(() => { closed = true; });

// ---- the seat's hands: a walk, the cursor round a circle, shots, an order key. By the beat, so every run of the bot asks the same of its man
// (OFF THE WALL: a leg of the square held into a wall takes the hold, and on it he has no look ahead by rule and a lone key only sidles him. The keys nearest
//  square away from it are held for half a second and the square goes on a leg later; he never stands a run out with his legs not his own)
const WAYS: [string[], number, number][] = [[['KeyW'], 1, 0], [['KeyS'], -1, 0], [['KeyD'], 0, 1], [['KeyA'], 0, -1], [['KeyW', 'KeyD'], 1, 1], [['KeyW', 'KeyA'], 1, -1], [['KeyS', 'KeyD'], -1, 1], [['KeyS', 'KeyA'], -1, -1]];
/** the seat whose man this client plays, or whom a watcher watches */
const seatN = (): number => Math.max(0, client.seat);
/** ITS CAMERA: on its man where he is drawn, looking one way. What a look ahead, a walk key and the cursor are read through is this, as a page's is the player's. (A second
 *  player's is turned another way, further off, in another window: what a machine shows is no number of the game's) */
let VIEW = { yaw: 0.4, pitch: 0.96, distance: 21, fov: 0.6, w: 1280, h: 720 };
let target: ArrayLike<number> | null = null;
function offWall(S: Stood): string[] | null {
  const w = S.ahead(seatN(), []).wall; if (!w) return null;
  const f = [-Math.sin(VIEW.yaw), -Math.cos(VIEW.yaw)], away = ([, a, r]: [string[], number, number]): number => ((f[0] * a - f[1] * r) * w[0] + (f[1] * a + f[0] * r) * w[1]) / Math.hypot(a, r);
  return WAYS.reduce((a, b) => away(b) > away(a) ? b : a)[0];
}
let off: { keys: string[]; till: number } | null = null, later = 0;
let beatN = 0, shots = 0, start: number[] | null = null, tickMs = 0, tickN = 0;
function hands(S: Stood): WireCmd {
  const n = beatN++, V = VIEW, c: WireCmd = { k: [], b: 0, m: [0, 0], s: [V.w, V.h, V.w, V.h], v: [target![0], target![1], target![2], V.yaw, V.pitch, V.distance, V.fov, V.w / V.h, 0] };
  if (off && n >= off.till) off = null;
  if (!off) { const keys = offWall(S); if (keys) { off = { keys, till: n + 30 }; later++; } }
  const leg = (Math.floor(n / 90) + later + seatN()) % 4; c.k = [...off?.keys ?? [['KeyW', 'KeyD', 'KeyS', 'KeyA'][leg]]];   // (a second seat walks the same square a side on)
  const a = n * 0.05 + seatN() * 2; c.m = [V.w / 2 + Math.cos(a) * 260, V.h / 2 + Math.sin(a) * 180];
  if (n > 120 && n % 40 < 4) { c.b = 1; if (n % 40 === 0) { c.c = 1; shots++; } }
  if (n === 200 || n === 500) c.p = ['KeyF'];
  if (n === 320) c.p = ['KeyR'];
  return c;
}
function report(code: number): never {
  console.log('MPBOT ' + JSON.stringify({ seat: client.seat, names: client.names, confirmed: client.confirmed, heard: client.heard, wordsOk: client.wordsOk, bad: client.bad, word: stood?.word() ?? '', watchOk: client.watchOk, watchOff: client.watchOff, watchFirst: client.watchFirst, sent: client.sent, holds: client.holds, skips: client.skips, margin: client.margin, lead: client.lead, broken: client.broken, stalled: Math.round(gapMax),
    moved: +walked.toFixed(3), shots, bye: client.bye, state: client.state, stoodMs: Math.round(stoodMs), tickMs: tickN ? +(tickMs / tickN).toFixed(2) : 0,
    ...(PREDICT ? { ahead: { first: seen.first, firstMoved: seen.gameMoved, n: seen.n, mean: seen.n ? +(seen.sum / seen.n).toFixed(4) : 0, max: +seen.max.toFixed(4), exact: seen.exact, unfree: seen.unfree, clean: seen.clean, cleanMean: seen.clean ? +(seen.cleanSum / seen.clean).toFixed(4) : 0, cleanMax: +seen.cleanMax.toFixed(4), cleanExact: seen.cleanExact, worst: seen.worst, fixes: fore.fixes, fixMax: +fore.fixMax.toFixed(4), records: seen.looks ? +(seen.recs / seen.looks).toFixed(1) : 0, ms: +fore.ms.toFixed(2) } } : {}) }));
  process.exit(code);
}
const t0 = performance.now(); let beats = 0, broke = false, walked = 0, at: number[] | null = null;   // (walked: the length of the way its man went, walk by walk — where a square walked by the beat ends is the box's)
// ---- the look ahead, held against the game: where a look said he would stand after its last record, kept by that tick until the game has run it
const fore = new Foresee(), said = new Map<number, { pos: number[]; heard: number; from: number }>(), heldBy: number[] = [], lostBy: number[] = [];   // (…and lostBy[t]: ticks up to and with t that ended with his legs not his own, or with him down)   // (heldBy[t]: ticks up to and with t the server ran on a held record for this seat)
const seen = { first: -1, gameMoved: false, sentAt: -1, n: 0, sum: 0, max: 0, exact: 0, looks: 0, recs: 0, unfree: 0, clean: 0, cleanSum: 0, cleanMax: 0, cleanExact: 0, worst: null as null | { t: number; mm: number; records: number; near: number } };
const APART = 1.5;
function met(S: Stood, t: number): void {   // (the game has just run tick t)
  const me = S.ahead(seatN(), []);
  lostBy[t] = (lostBy[t - 1] ?? 0) + (!me.down && me.free ? 0 : 1);
  const was = said.get(t + 1); if (!was) return; said.delete(t + 1);
  const want = was.pos, p = me.at, e = Math.hypot(p[0] - want[0], p[1] - want[1], p[2] - want[2]);
  seen.n++; seen.sum += e; if (e > seen.max) seen.max = e; if (e < 0.001) seen.exact++;
  // (a CLEAN look: every record it played that the server had not said yet was then run as sent, his legs stayed his own on every tick it crossed (a
  //  kick, a door, a hit that drops him begun INSIDE the look is the game's to say, and the walk ahead plays none of it), and nobody stands within APART
  //  of him — the walk ahead holds everybody else where the tick has them, so a teammate at his shoulder moves him as no look could know. Such a look is
  //  right to the bar; any other is a correction, eased on the page, and is counted in the whole alone)
  const near = me.near;
  if (heldBy[t] === (heldBy[was.heard - 1] ?? 0) && lostBy[t] === (lostBy[was.from - 1] ?? 0) && near > APART) { seen.clean++; seen.cleanSum += e; if (e < 0.001) seen.cleanExact++;
    if (WHY && e > 0.001) console.error(`why ${JSON.stringify({ t, mm: Math.round(e * 1000), from: was.from, heard: was.heard, near: +near.toFixed(2), at: p, want })}`);
    if (e > seen.cleanMax) { seen.cleanMax = e; seen.worst = { t, mm: Math.round(e * 1000), records: t + 1 - was.from, near: +near.toFixed(2) }; } }
}
function lookAhead(S: Stood): void {
  const F = fore.look(client.seat >= 0 ? (cmds: WireCmd[]) => S.ahead(client.seat, cmds) : null, client);
  if (client.sent && seen.sentAt < 0) seen.sentAt = beats;
  if (!F) return;
  seen.looks++; seen.recs += F.n; if (!F.free) seen.unfree++; else if (F.n) said.set(F.end, { pos: F.pos, heard: client.heard, from: client.confirmed });   // (a look with his legs not his own walked nothing: nobody is ahead, and nothing is held against the game)
  if (seen.first < 0 && seen.sentAt >= 0 && Math.hypot(F.pos[0] - start![0], F.pos[2] - start![2]) > 0.001) { seen.first = beats - seen.sentAt; const p = S.ahead(seatN(), []).at; seen.gameMoved = Math.hypot(p[0] - start![0], p[2] - start![2]) > 0.001; }
  for (const k of said.keys()) if (k <= client.confirmed) said.delete(k);
}
const loop = setInterval(() => {
  if ((closed || client.broken) && (!stood || !client.behind)) { clearInterval(loop); report(client.confirmed >= want ? 0 : 1); }
  if (!stood) return;
  const S: Stood = stood;
  start ??= S.ahead(seatN(), []).at; target ??= start;
  // the beat: 1 / hz of this process's own clock
  const due = Math.floor((performance.now() - t0) * NET.hz / 1000) - beats;
  for (let i = 0; i < Math.min(due, 2); i++) { beats++; client.beat(client.seat >= 0 && client.state === 'playing' ? hands(S) : null, performance.now()); }
  if (due > 2) beats += due - 2;
  const a = performance.now();
  const led = () => { const F = PREDICT ? fore.last : null; target = F ? F.to.pos : S.ahead(seatN(), []).at; };   // (its camera is on its man where he is drawn, as the page's is: src/host/drawpage.ts `lead`)
  // (--break, the red path: ONE record stepped with another walk key than the server ran, after tick 90. The man here is then a hair from where the
  //  server has him, and nothing but the state word can say so)
  const n = client.walk({ step: (cmds: WireCmd[]) => { const t = client.confirmed; heldBy[t] = client.heldRun;
    if (BREAK && !broke && t >= 90) { broke = true; const i = Math.max(0, client.seat), c = cmds[i]; cmds[i] = { ...c, k: c.k.includes('KeyA') ? ['KeyD'] : ['KeyA'] }; }
    S.step(cmds); if (PREDICT) met(S, t); }, word: () => S.word(), watch: () => S.watch() }, Math.min(8, want - client.confirmed));   // (never past the tick asked for: two bots' last words are held against each other)
  if (n) { { const p = S.ahead(seatN(), []).at; if (at) walked += Math.hypot(p[0] - at[0], p[2] - at[2]); at = [p[0], p[1], p[2]]; } led(); }
  if (n) { tickMs += performance.now() - a; tickN += n; }
  if (PREDICT && due > 0) { lookAhead(S); if (!n) led(); }
  if (stall > 0 && !stalledOnce && client.confirmed >= 150) { stalledOnce = true; WAY.stallUntil = performance.now() + stall; }
  if (client.confirmed >= want) { clearInterval(loop); ws.close(); report(0); }
}, 4);
setTimeout(() => { clearInterval(loop); report(2); }, Number(arg('--timeout', '120')) * 1000);
