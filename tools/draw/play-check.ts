// EVERY MISSION ON THE PAGE THAT HOLDS NO GAME, with no page: the module is started from each row's own address as the page starts it, handed a walking hand, stepped,
// drawn between ticks, saved, loaded, lost to a planted panic and stood again, restarted and stepped on. A panic in a section is an abort, so each is met here first.
//   bun run tools/draw/play-check.ts [mission id…]        exit 0 · 1 and each mission that stopped, with its words · 2 no such mission
//   PLAY_BREAK=1 …      a load is asked of a key nothing is kept under: the line that says the load goes back must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/character.ts';
import { DrawGame } from '../../src/host/drawgame';
import { sampleHand } from '../../src/draw/hand';
import { FollowCamera } from '../../src/host/camera';
import { Section, Head, View } from '../../src/draw/sections';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.PLAY_BREAK === '1';
const { missionRows } = await import(`${ROOT}/tools/desk.ts`) as typeof import('../desk.ts'), LISTED = missionRows();
const wasm = faceHere(); if (!wasm) { console.error('play-check: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const instance = new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(wasm))));
const [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT), only = process.argv.slice(2);
let fails = 0; const line = (ok: boolean, what: string, why = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${why ? `\n    ${why}` : ''}`); };
const bytes = (f: string): Uint8Array => new Uint8Array(readFileSync(resolve(ROOT, 'levels', f)));

{ const stray = only.filter(id => !LISTED.some(m => m.id === id)); if (stray.length) { console.error(`play-check: no mission ${stray.join(', ')}`); process.exit(2); } }
/** from above nobody on the floor is the hand's to walk */
const SKY = new Set(['tut-sky']);
/** …and every level's file with no mission named, as an address with nothing on it but the level opens it */
const PLAIN = [...new Set(LISTED.map(m => m.level))].map(f => ({ id: `${f.replace('.json', '')} (no mission)`, words: `?levelFile=${f}` }));
for (const m of [...LISTED.map(m => ({ id: m.id, words: m.link })), ...(only.length ? [] : [{ id: 'the plain address', words: '?seed=1' }, ...PLAIN])]) { if (only.length && !only.includes(m.id)) continue;
  const { words } = m, file = new URLSearchParams(words).get('levelFile') ?? 'office.json';
  const game = new DrawGame({ exports: instance.exports, bytes: 0, fetchMs: 0, compileMs: 0 }), D = game.drawn, cam = new FollowCamera(), input = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 300 }, canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 };
  let at = 'the start', warmMs = 0, worst = 0, sum = 0, n = 0, boxes = 0;
  const where = (): [number, number] => { const o = D.at[Section.view] + View.at; return [D.f[o], D.f[o + 2]]; }, stood = (): number => D.u[D.at[Section.view] + View.stood];
  const run = (ticks: number, what: string): void => { at = what; for (let i = 0; i < ticks; i++) {
    const keys = sampleHand(game.numbersOf(), input, cam, canvas); game.step(1 / 60, keys); input.pressed.clear(); input.clicked = 0;
    const o = D.at[Section.view], f: [number, number, number] = [D.f[o + View.follow], D.f[o + View.follow + 1], D.f[o + View.follow + 2]]; cam.step(1 / 60, f, null);
    for (const a of [0.5, 1]) { game.frame(a, cam.target); D.fresh(); }
    const h = D.at[Section.head]; boxes = Math.max(boxes, D.u[h + Head.count]); sum += game.ms.draw; n++; worst = Math.max(worst, game.ms.draw);
    if (D.u[h + Head.count] !== D.n[Section.geo] || D.n[Section.view] !== 1 || D.n[Section.sky] !== 1) throw new Error('the sections do not agree');
    if (!game.at(game.view().me) || !game.head(game.view().me)) throw new Error('the HUD is told of a man who is not drawn'); } };
  try {
    game.start({ level: bytes(file), words, mannequin, mannequinBin, cast, castLook, seed: 5 });
    { const o = D.at[Section.view]; cam.target = [D.f[o + View.follow], D.f[o + View.follow + 1], D.f[o + View.follow + 2]]; cam.rebuild(); }
    // the ticks a page plays and throws away before the first: the game that plays is the game that was started
    at = 'the ticks thrown away'; let cold = true; { const was = game.mark(), t0 = performance.now(); await game.warm(36, (N, i) => { input.keys.clear(); if (i >= 12) input.keys.add('KeyW'); input.buttons = i >= 24 ? 1 : 0; return sampleHand(N, input, cam, canvas); }, async () => { /* */ }); input.keys.clear(); input.buttons = 0; const is = game.mark();
      cold = JSON.stringify(was) === JSON.stringify(is) && game.ticks === 0; const which = stood(); run(1, 'the first tick'); cold &&= stood() === which; warmMs = Math.max(warmMs, performance.now() - t0); }
    game.commands.god(1);
    run(60, 'a second standing');
    input.pressed.add('F5'); run(2, 'a save by hand'); const saved = game.savesTold(), hand = game.saves().hand;
    const a = where(); input.keys.add('KeyW'); run(90, 'a walk'); input.keys.clear(); const b = where();
    input.buttons = 1; input.clicked = 1; run(30, 'a shot'); input.buttons = 0;
    const s0 = stood(); game.commands.load(BREAK ? 'manual.9' : 'manual.0'); run(2, 'a load'); const put = game.savesTold(), c = where(), loaded = stood() !== s0, walked = Math.hypot(b[0] - a[0], b[1] - a[1]) > 0.3;
    // a panic: the game of that call is gone, and it stands again from the newest kept, with its saves
    at = 'a panic'; let label: string | null = null; try { game.face.debugPanic(game.h); } catch { label = game.recover(true); }
    run(20, 'the frames after a panic'); const e = where(), up = label !== null && (/^tut-/.test(file) || (label === 'by hand' && Math.hypot(e[0] - a[0], e[1] - a[1]) < 0.6));
    // …and a second time, from the start of the run, with the saves still kept
    at = 'a second panic'; let first: string | null = null; try { game.face.debugPanic(game.h); } catch { first = game.recover(false); }
    run(3, 'the frames after the second'); const z = where(), kept = game.kept(), start = first === '' && Math.hypot(z[0] - a[0], z[1] - a[1]) < 2 && (/^tut-/.test(file) || kept === 'by hand');
    // what a card of the run is drawn from: its record, how it ended, the map's floor and marks
    at = 'the card of it'; const card = game.debrief(), E = game.end(), carded = card.rows !== undefined && E.floor.rows.length === E.floor.w * E.floor.h && /[01]/.test(E.floor.rows) && E.marks.some(m => m.cls === 'spawn') && E.marks.every(m => Number.isFinite(m.x + m.z) && m.title.length > 2);
    const s1 = stood(); game.commands.restart(''); run(120, 'a restart'); const again = stood() !== s1;
    // (a tutorial keeps its own checkpoints and takes none by hand)
    const taught = /^tut-/.test(file), back = loaded && Math.hypot(c[0] - a[0], c[1] - a[1]) < 0.1;
    const told = !hand ? taught : saved.tag === 'CHECKPOINT SAVED' && put.tag === 'CHECKPOINT LOADED' && put.lines[0] === 'checkpoint loaded — by hand' && !game.savesTold().lines.length;   // (…and the player is told of it, once)
    const went = walked || SKY.has(m.id);
    line(again && cold && went && up && start && told && carded && (taught || back), `${m.id}: started, walked${walked ? '' : ' (nowhere)'}, shot, ${taught ? 'taught' : 'saved and went back'}, stood again after a panic, restarted`, `${cold ? '' : 'the game after the ticks thrown away is not the game before them · '}${boxes} boxes · thrown away in ${warmMs.toFixed(0)} ms · sections ${(sum / n).toFixed(2)} ms a tick, worst ${worst.toFixed(1)}${start ? '' : ` · from the start: ${first}, at ${z.map(x => x.toFixed(1))}, kept ${kept}`}${up ? '' : ` · after the panic: ${label}, at ${e.map(x => x.toFixed(1))}`}${again ? '' : ' · the restart was not told'}${carded ? '' : ` · the card: ${JSON.stringify([E.marks, E.floor.w, E.floor.h])}`}${told ? '' : ` · what was said of the saves: ${JSON.stringify([hand, saved, put])}`}${taught || back ? '' : ` · the load left him at ${c.map(x => x.toFixed(1))}, saved at ${a.map(x => x.toFixed(1))}`}`);
  } catch (e) { line(false, `${m.id}: stopped in ${at}`, String((e as Error)?.message ?? e).slice(0, 400)); }
  try { if (game.h) game.face.drop(game.h); } catch { /* lost to its panic */ }
}
// WHAT A MACHINE KEEPS is what a game is staged by: a hard one's tables are another's than a standard one's, and a standard one's are its own again
if (!only.length) { const tables = (kept: string): string => { const g = new DrawGame({ exports: instance.exports, bytes: 0, fetchMs: 0, compileMs: 0 }), F = g.face;
    g.start({ level: bytes('depot.json'), words: '?levelFile=depot.json&roomclear&squad4', kept, mannequin, mannequinBin, cast, castLook, seed: 5 });
    const w = F.stateWords(g.h), off = F.stateNames().map((n, i) => `${n} ${w[i]}`);
    F.drop(g.h); return off.filter(x => /^k /.test(x)).join(); };
  const standard = tables(''), hard = tables('{"hard":true}'), lever = tables('{"levers":{"ai":"hard"}}'), refused = tables('{"levers":{"ai":"nonsense"}}'), after = tables('');
  line(standard === after && standard === refused && hard !== standard && lever !== standard && lever !== hard, 'hard, and one lever of it, are what a game is staged by; a word that is no rung is nothing, and the next game is its own'); }
// TWO GAMES IN ONE MODULE, and one of them lost to a panic: it is still that game, whatever else has been started since, and the other is untouched
if (!only.length) { const two = [0, 1].map(() => new DrawGame({ exports: instance.exports, bytes: 0, fetchMs: 0, compileMs: 0 })), [A, B] = two, cam = new FollowCamera(), canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 }, input = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 360 }, start = (g: DrawGame, level: string): void => g.start({ level: bytes(level), words: `?levelFile=${level}&roomclear&squad4`, mannequin, mannequinBin, cast, castLook, seed: 5 });
  const step = (g: DrawGame, n: number): void => { cam.rebuild(); for (let i = 0; i < n; i++) g.step(1 / 60, sampleHand(g.numbersOf(), input, cam, canvas)); }, doors = (g: DrawGame): number => g.map().doors.length, way = (g: DrawGame): boolean => { const D = g.map().doors; return !!g.face.way(g.h, [g.map().men[0].x, 0, g.map().men[0].z], [D[0].hx + 1, 0, D[0].hz + 1], 6000); };
  start(A, 'block.json'); step(A, 60); const a0 = doors(A), ways0 = way(A); try { A.face.debugPanic(A.h); } catch { /* lost */ }
  start(B, 'compound.json'); step(B, 60); const b0 = doors(B), b1 = B.face.stateWords(B.h).join(), label = A.recover(false), a1 = doors(A), apart = A.h !== B.h && B.face.stateWords(B.h).join() === b1; step(A, 60); step(B, 60);
  line(label === '' && a0 === a1 && a0 !== b0 && apart && doors(B) === b0 && (!ways0 || way(A)), 'of two games in one module the one lost to a panic stands again as itself, and the other goes on', `${a0} and ${a1} doors, the other ${b0}; numbers ${A.h} and ${B.h}`);
  for (const g of two) g.face.drop(g.h); }
// A KNOB BY ITS NAME: every one turned to what it is turned to is no change, one turned is another game, and turned back it is the same again
if (!only.length) { const g = new DrawGame({ exports: instance.exports, bytes: 0, fetchMs: 0, compileMs: 0 }), F = g.face;
  g.start({ level: bytes('depot.json'), words: '?levelFile=depot.json&roomclear&squad4', mannequin, mannequinBin, cast, castLook, seed: 5 });
  const word = (): string => F.stateWords(g.h).join();
  const send = (f: () => void): void => { f(); F.commands(g.h, g.commands.take()); };
  const all = g.knobs(), was = word(); send(() => { for (const k of all) g.commands.knob(k.name, k.value); }); const same = word() === was;
  const one = all.find(k => /pressure\.on$/.test(k.name)) ?? all[0]; send(() => g.commands.knob(one.name, one.yes ? 1 - one.value : one.value + 1)); const turned = word() !== was && g.knobs().find(k => k.name === one.name)!.value !== one.value;
  send(() => g.commands.knob(one.name, one.value)); send(() => g.commands.knob('no.such', 1));
  line(all.length > 1000 && same && turned && word() === was, `${all.length} knobs by name: turned to what they are is no change, one turned is, and turned back it is not`); F.drop(g.h); }
console.log(fails ? `=== play: ${fails} FAILED ===` : '=== play: ALL PASS ===');
process.exit(fails ? 1 : 0);
