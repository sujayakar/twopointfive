// A GAME OF TWO ON THE PAGE THAT DRAWS (host/drawnet.ts), with no page and no socket: the server's core and a second player stand the module's game (tools/mp/modstand.ts), the first player is the page's own half over
// a door with a wire of a check's, on a clock of a check's. The words of the whole game are the server's at every word tick, through a restart, a stretch asleep and bunched ticks; his man is drawn where the
// look ahead has him, stands still where nothing was confirmed, and is blended over every tick of a walk.
//   bun run tools/draw/net-check.ts        exit 0 · 1
//   NET_BREAK=1 …      one tick is stepped on a record that is not the server's: the words must part
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/start-pack.ts';
import { freshStart, standStart } from '../mp/modstand.ts';
import { drawEnd } from '../../src/host/drawend';
import { DrawGame } from '../../src/host/drawgame';
import { sampleHand } from '../../src/draw/hand';
import { DrawNet } from '../../src/host/drawnet';
import { shownAs } from '../../src/host/myname';
import { NetDoor } from '../../src/net/door';
import { FORESEE } from '../../src/net/foresee';
import { NetClient } from '../../src/net/client';
import { NetServer, type Conn } from '../../src/net/server';
import { FIRST, type WireCmd } from '../../src/net/wire';
import { FollowCamera } from '../../src/host/camera';
import { Man, Section, View } from '../../src/draw/sections';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.NET_BREAK === '1';
let now = 1000; performance.now = () => now;
let fails = 0; const line = (ok: boolean, what: string, why = ''): void => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${why ? `\n    ${why}` : ''}`); };
const start = freshStart({ search: '?levelFile=compound.json&roomclear&squad4&seats=2', seed: 7 });

// ---- the server, and the wire: what is sent arrives two of the clock's ticks later
const S = await standStart(start), WAY = 2 * 1000 / 60, wire: { at: number; go: () => void }[] = [], later = (go: () => void): void => { wire.push({ at: now + WAY, go }); };
const server = new NetServer({ now: () => now, step: c => S.step(c), word: () => S.word(), watch: () => S.watch(), ended: () => S.ended() }, { game: 'g', build: '', start, seatKeys: ['a', 'b'] });
const joined = (c: NetClient, keep: { conn: Conn | null }): void => { keep.conn = { send: t => { later(() => c.message(t, now)); return 1; }, close: () => { /* */ } }; };

// ---- the second player
const B = await standStart(start), bWire = { conn: null as Conn | null }, b = new NetClient(t => later(() => server.message(bWire.conn!, t)), { game: 'g', build: '', key: 'b', name: 'Grace' }); joined(b, bWire);
B.show(1);

// ---- the first: the page's half
const wasm = faceHere(); if (!wasm) { console.error('net-check: no module of these sources'); process.exit(1); }
const instance = new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(wasm))), { env: { face_roll: () => { throw new Error('the game asked a die'); } } });
const [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT);
const game = new DrawGame({ exports: instance.exports, bytes: 0, fetchMs: 0, compileMs: 0 }), D = game.drawn, aWire = { conn: null as Conn | null };
const door = new NetDoor('', 'g', 'a', 'Ada'); (door as unknown as { ws: unknown }).ws = { readyState: 1, send: (t: string) => later(() => server.message(aWire.conn!, t)), close: () => { /* */ } }; joined(door.client, aWire);
FORESEE.on = true;
let deaf = 0; const net = new DrawNet(door, start, game, () => { deaf++; });
net.start({ level: new Uint8Array(readFileSync(resolve(ROOT, 'levels', start.file))), mannequin, mannequinBin, cast, castLook });
// (the ticks the page plays and throws away before the first, as a page does: the server's game and the second player's are played none, and every word below must still be theirs)
{ const eye = new FollowCamera(), none = new Set<string>(), walk = new Set(['KeyW']), o = D.at[Section.view] + View.follow; eye.target = [D.f[o], D.f[o + 1], D.f[o + 2]]; eye.rebuild();
  await game.warm(36, (N, i) => sampleHand(N, { keys: i >= 12 ? walk : none, pressed: none, buttons: i >= 24 ? 1 : 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 300 }, eye, { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 }), async () => { /* */ }); }
if (BREAK) { const n = (net as unknown as { net: { step(c: WireCmd[]): void; ticks: number } }).net, step = n.step.bind(n); n.step = c => step(n.ticks === 200 ? [{ ...c[0], k: ['KeyS'] }, c[1]] : c); }

const cam = new FollowCamera(), canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 }, input = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 300 };
const row = (): number => { for (let i = 0, o = D.at[Section.men]; i < D.n[Section.men]; i++, o += Man.row) if (D.u[o + Man.id] === D.u[D.at[Section.view] + View.man]) return o; return -1; };
const two = (o: number): [number, number] => [D.f[o], D.f[o + 2]], far = (p: number[], q: number[]): number => Math.hypot(p[0] - q[0], p[1] - q[1]), stood = (): number => D.u[D.at[Section.view] + View.stood];
let seen = 0, asleep = false, stall = false, off = false, level = 0, worst = 0, led = 0, held = 0, rewound = 0, moved = 0, spread = 0, spreads = 0; const bKeys = ['KeyW', 'KeyA', 'KeyS', 'KeyD'];
/** a tick of every machine's own clock */
function beat(): void {
  now += 1000 / 60; while (wire.length && wire[0].at <= now) wire.shift()!.go();
  if (!stall) server.advance();
  b.beat({ ...FIRST, k: [bKeys[Math.floor(b.sent / 50) % 4]], v: [B.ticks ? 2 : 0, 0, 12, 0, 0.96, 21, 0.6, 16 / 9, 0] }, now); b.walk({ step: c => B.step(c), word: () => B.word(), watch: () => B.watch() });
  if (asleep) return;
  const was = game.ticks, before = row() >= 0 ? two(row() + Man.at) : null;
  if (net.tick(input, cam, canvas, off, () => { seen++; })) level++;
  input.pressed.clear(); input.clicked = 0;
  const o = D.at[Section.view], L = net.lead, at: [number, number, number] = [D.f[o + View.follow], D.f[o + View.follow + 1], D.f[o + View.follow + 2]]; if (L) for (let i = 0; i < 3; i++) at[i] += L.by[i];
  cam.step(1 / 60, at, L?.aim ?? null);
  if (door.far || game.ticks === 0) return;
  game.frame(1, cam.target); const m = row(), F = door.fore.last; if (m < 0) return;
  const drawn = two(m + Man.feet), stands = two(m + Man.at);
  if (F && F.id === D.u[m + Man.id]) { worst = Math.max(worst, far(drawn, [F.to.pos[0], F.to.pos[2]])); led = Math.max(led, far(drawn, stands)); }
  // a page tick in which nothing was confirmed: the men of it stand where the last tick left them, however far the frame is into the next
  if (game.ticks === was) { held++; game.frame(0.1, cam.target); for (let i = 0, q = D.at[Section.men]; i < D.n[Section.men]; i++, q += Man.row) if (q !== row()) rewound = Math.max(rewound, far(two(q + Man.feet), two(q + Man.at))); }
  else { game.frame(0.1, cam.target); for (let i = 0, q = D.at[Section.men]; i < D.n[Section.men]; i++, q += Man.row) if (q !== row()) moved = Math.max(moved, far(two(q + Man.feet), two(q + Man.at))); }
  // two ticks in one: half way is half way over both
  if (game.ticks === was + 2 && before && !F) { game.frame(0.5, cam.target); const mid = [(before[0] + stands[0]) / 2, (before[1] + stands[1]) / 2]; spread = Math.max(spread, far(two(m + Man.feet), mid)); spreads++; }
}
const c = door.client; c.hello(); b.hello(); for (let i = 0; i < 20; i++) beat(); c.ready(); b.ready();
const run = (n: number): void => { for (let i = 0; i < n; i++) beat(); };
run(120); input.keys.add('KeyW'); run(240); input.keys.clear(); input.keys.add('KeyD'); run(120);
// the server a few ticks late, again and again
for (let i = 0; i < 12; i++) { stall = true; run(4); stall = false; run(8); }
// bunched: every third beat the page misses two, with no look ahead, so that what is drawn is the game's
FORESEE.on = false; for (let i = 0; i < 60; i++) { asleep = true; run(2); asleep = false; run(1); } FORESEE.on = true;
input.keys.clear(); run(60);
const walked = seen, parted = c.bad ? `parted at tick ${c.bad.t}` : '';
line(!c.bad && !c.broken && c.wordsOk > 20 && c.watchOk === c.wordsOk && c.watchOff === 0 && !b.bad, 'every word tick, the words of the whole game are the server\'s, on the page\'s half and on the second player\'s', parted || `${c.wordsOk} word ticks, ${c.sent} records sent, ${c.heldRun} ticks ran on a held one`);
line(led > 0.1 && worst < 0.02, 'his man is drawn where the look ahead has him', `at most ${led.toFixed(2)} m ahead of the game; ${(worst * 1000).toFixed(1)} mm off the look`);
line(held > 5 && moved > 0.01 && rewound < 1e-4, 'where nothing was confirmed the others stand still', `${held} such ticks, ${(rewound * 1000).toFixed(2)} mm, where between two that were they are up to ${(moved * 1000).toFixed(0)} mm apart`);
line(spreads > 5 && spread < 0.02, 'two ticks in one of the page\'s are blended over both', `${spreads} of them, ${(spread * 1000).toFixed(1)} mm off half way`);
// a restart by a key: every machine's, and the picture begins anew
{ const s = stood(), ok = c.wordsOk; input.keys.add('ShiftLeft'); input.pressed.add('Enter'); input.keys.add('Enter'); run(3); input.keys.clear(); run(90); line(!c.bad && c.wordsOk > ok && stood() !== s, 'a restart inside the game is every machine\'s, and nothing is drawn through it', `${c.wordsOk - ok} word ticks after it`); }
// asleep for four seconds, and back
{ const ok = c.wordsOk, was = seen, ticks = game.ticks; asleep = true; run(240); asleep = false; let n = 0; while (c.behind > 2 && n++ < 100) beat(); run(30);
  line(!c.bad && deaf === 1 && level === 1 && !door.far && c.wordsOk > ok && seen - was < game.ticks - ticks, 'a page that slept catches up over ticks nobody sees, and is level again', `${game.ticks - ticks} ticks, ${seen - was} of them seen`); }
// his hands off under a card
{ input.keys.add('KeyW'); run(30); off = true; run(20); const p = game.view().me, a = game.at(p)!; run(30); const z = game.at(p)!; line(far([a[0], a[2]], [z[0], z[2]]) < 0.02 && !c.bad, 'under a card his hands are off, and the game goes on', `${walked} ticks seen in all before it`); }
// what a card calls the players
{ const E = drawEnd(game, {} as never, c.names), own = [1, 2, 3].map(n => E.name(0, `Player ${n}`)), mate = E.name(0, 'Grey'), twin = shownAs('Player 1', ['Ada', 'ada']), posed = shownAs('Player 1', ['player 2', 'Grace']), one = shownAs('Player', ['Ada']);
  line(own.join() === 'Ada,Grace,Player 3' && mate === 'Grey' && twin === 'Player 1' && posed === 'Player 1' && one === 'Ada', 'a card calls a player what he picked, over the game\'s own word for his seat; a name that tells nobody apart is not used', `${own.join(', ')} \u00b7 ${mate} \u00b7 ${twin} \u00b7 ${posed} \u00b7 ${one}`); }
console.log(fails ? `=== net: ${fails} FAILED ===` : '=== net: ALL PASS ===');
process.exit(fails ? 1 : 0);
