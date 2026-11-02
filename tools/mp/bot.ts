// A SCRIPTED CLIENT OF A GAME SERVER (bun; a process of its own: it stands a game). It connects, stands its game from the server's recipe, sends a seat's records
// when it has a seat's key, walks every confirmed tick and holds its state word against the server's. What tools/qa/mpserver-check.ts runs two and three of.
//   bun run tools/mp/bot.ts --url ws://127.0.0.1:5300 --game local --key k0 --ticks 1200
//   --key K        sit in the seat with that key (none: watch)            --ticks N     leave once N ticks are walked (default 600)
//   --lag MS       hold every message, both ways, that long (default 0)   --jitter MS   and up to this much longer, by its own seeded die
//   --after MS     connect that long after starting (a late joiner)       --break       after tick 90, flip the last bit of its own man's place (the red path)
//   --build NAME   the build it says it is (default: this checkout's, as the server names its own)
// Prints one line: MPBOT {seat, confirmed, heard, wordsOk, bad, word, sent, holds, skips, margin, lead, moved, shots, bye, stoodMs, tickMs}. Exit 0 = it walked its ticks.
import { resolve } from 'node:path';
import type { PlayerCmd } from '../../src/game/cmd.ts';
import { standRecipe, buildName, type Stood } from './stand.ts';

const ROOT = resolve(import.meta.dir, '../..');
const A = process.argv.slice(2), arg = (k: string, d = ''): string => { const i = A.indexOf(k); return i >= 0 ? A[i + 1] ?? '' : d; };
const url = arg('--url', 'ws://127.0.0.1:5300'), game = arg('--game', 'local'), key = A.includes('--key') ? arg('--key') : undefined, want = Number(arg('--ticks', '600'));
const lag = Number(arg('--lag', '0')), jitter = Number(arg('--jitter', '0')), after = Number(arg('--after', '0')), BREAK = A.includes('--break');
const { NetClient } = await import(`${ROOT}/src/net/client.ts`), { NET } = await import(`${ROOT}/src/net/wire.ts`), { sampleCmd } = await import(`${ROOT}/src/game/cmd.ts`);

let die = 0x2545f491; const roll = () => { die = (Math.imul(die, 1664525) + 1013904223) >>> 0; return die / 4294967296; };
/** a message held `lag` + up to `jitter` ms, never overtaking the one before it (a socket keeps its order): one queue a direction, let go by the clock */
function delayed(fn: (text: string) => void): (text: string) => void {
  if (!lag && !jitter) return fn;
  const q: [number, string][] = []; let last = 0;
  setInterval(() => { const now = performance.now(); while (q.length && q[0][0] <= now) fn(q.shift()![1]); }, 1);
  return text => { const at = Math.max(last, performance.now() + lag + roll() * jitter); last = at; q.push([at, text]); };
}

if (after > 0) await new Promise(r => setTimeout(r, after));
const ws = new WebSocket(url);
const client = new NetClient(delayed(text => { if (ws.readyState === 1) ws.send(text); }), { game, build: A.includes('--build') ? arg('--build') : buildName(), ...(key !== undefined ? { key } : {}) });
let stood: Stood | null = null, standing = false, stoodMs = 0, closed = false;
const take = delayed(text => { client.message(text, performance.now()); if (client.welcome && !stood && !standing) { standing = true; const t0 = performance.now(); standRecipe(client.welcome.recipe).then(s => { stood = s; stoodMs = performance.now() - t0; if (client.seat >= 0) client.ready(); }, e => { console.error(`bot.ts: ${(e as Error).message}`); process.exit(1); }); } });
ws.onopen = () => client.hello();
ws.onmessage = (ev: { data: unknown }) => take(String(ev.data));
ws.onclose = () => { closed = true; };
ws.onerror = () => { closed = true; };

// ---- the seat's hands: a walk, the cursor round a circle, shots, an order key. By the beat, so every run of the bot asks the same of its man
let beatN = 0, shots = 0, start: number[] | null = null, tickMs = 0, tickN = 0;
function hands(S: Stood): PlayerCmd {
  const W = S.W, I = W.input, n = beatN++;
  I.keys.clear(); I.pressed.clear(); I.clicked = 0; I.buttons = 0; I.wheel = 0;
  const leg = Math.floor(n / 90) % 4; I.keys.add(['KeyW', 'KeyD', 'KeyS', 'KeyA'][leg]);
  const a = n * 0.05; I.mouseX = W.canvas.clientWidth / 2 + Math.cos(a) * 260; I.mouseY = W.canvas.clientHeight / 2 + Math.sin(a) * 180;
  if (n > 120 && n % 40 < 4) { I.buttons = 1; if (n % 40 === 0) { I.clicked = 1; shots++; } }
  if (n === 200 || n === 500) I.pressed.add('KeyF');
  if (n === 320) I.pressed.add('KeyR');
  return sampleCmd(W.game, I, W.cam, W.canvas);
}
function report(code: number): never {
  const g = stood?.W.game, p = g?.player.char.pos as number[] | undefined;
  console.log('MPBOT ' + JSON.stringify({ seat: client.seat, confirmed: client.confirmed, heard: client.heard, wordsOk: client.wordsOk, bad: client.bad, word: stood?.word() ?? '', sent: client.sent, holds: client.holds, skips: client.skips, margin: client.margin, lead: client.lead, broken: client.broken,
    moved: p && start ? +Math.hypot(p[0] - start[0], p[2] - start[2]).toFixed(3) : 0, shots, bye: client.bye, state: client.state, stoodMs: Math.round(stoodMs), tickMs: tickN ? +(tickMs / tickN).toFixed(2) : 0 }));
  process.exit(code);
}
const t0 = performance.now(); let beats = 0, broke = false;
const loop = setInterval(() => {
  if ((closed || client.broken) && (!stood || !client.behind)) { clearInterval(loop); report(client.confirmed >= want ? 0 : 1); }
  if (!stood) return;
  const S: Stood = stood, g = S.W.game;
  start ??= [...g.player.char.pos];
  // the beat: 1 / hz of this process's own clock
  const due = Math.floor((performance.now() - t0) * NET.hz / 1000) - beats;
  for (let i = 0; i < Math.min(due, 2); i++) { beats++; client.beat(client.seat >= 0 && client.state === 'playing' ? hands(S) : null, performance.now()); }
  if (due > 2) beats += due - 2;
  const a = performance.now();
  const n = client.walk({ step: (cmds: PlayerCmd[]) => { S.step(cmds); S.W.cam.update(1 / NET.hz, g.followPos(), g.aimPoint); }, word: () => S.word() }, Math.min(8, want - client.confirmed));   // (never past the tick asked for: two bots' last words are held against each other)
  if (n) { tickMs += performance.now() - a; tickN += n; }
  if (BREAK && !broke && client.confirmed >= 90) { broke = true; const p = g.player.char.pos as number[], f = new Float64Array([p[0]]), u = new Uint32Array(f.buffer); u[0] ^= 1; p[0] = f[0]; }
  if (client.confirmed >= want) { clearInterval(loop); ws.close(); report(0); }
}, 4);
setTimeout(() => { clearInterval(loop); report(2); }, Number(arg('--timeout', '120')) * 1000);
