// EVERY MISSION ON THE PAGE THAT HOLDS NO GAME, with no page: the module is started from each row's own address as draw.html starts it, handed a walking hand, stepped,
// drawn between ticks, saved, loaded, restarted and stepped on. A panic in a section is an abort, so each is met here first.
//   bun run tools/draw/play-check.ts [mission id…]        exit 0 · 1 and each mission that stopped, with its words · 2 no such mission
//   PLAY_BREAK=1 …      a load is asked of a key nothing is kept under: the line that says the load goes back must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/start-pack.ts';
import { DrawGame } from '../../src/host/drawgame';
import { sampleHand } from '../../src/draw/hand';
import { FollowCamera } from '../../src/game/camera';
import { Section, Head, View } from '../../src/draw/sections';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.PLAY_BREAK === '1';
const { ALL_MISSIONS, pickSearch } = await import(`${ROOT}/src/scene/scenarioList.ts`), { lookLevelFile, LOOK } = await import(`${ROOT}/src/scene/look.ts`);
LOOK.on = true;
const wasm = faceHere(); if (!wasm) { console.error('play-check: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const instance = new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(wasm))), { env: { face_roll: () => { throw new Error('the game asked a die'); } } });
const [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT), only = process.argv.slice(2);
let fails = 0; const line = (ok: boolean, what: string, why = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${why ? `\n    ${why}` : ''}`); };
const bytes = (f: string): Uint8Array => { try { return new Uint8Array(readFileSync(resolve(ROOT, 'levels', lookLevelFile(f)))); } catch { return new Uint8Array(readFileSync(resolve(ROOT, 'levels', f))); } };

{ const stray = only.filter(id => !(ALL_MISSIONS as { id: string }[]).some(m => m.id === id)); if (stray.length) { console.error(`play-check: no mission ${stray.join(', ')}`); process.exit(2); } }
/** from above nobody on the floor is the hand's to walk */
const SKY = new Set(['tut-sky']);
for (const m of ALL_MISSIONS as { id: string; title: string }[]) { if (only.length && !only.includes(m.id)) continue;
  const words: string = pickSearch(m, ''), file = new URLSearchParams(words).get('levelFile') ?? 'office.json';
  const game = new DrawGame({ exports: instance.exports, bytes: 0, fetchMs: 0, compileMs: 0 }), D = game.drawn, cam = new FollowCamera(), input = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 300 }, canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 };
  let at = 'the start', worst = 0, sum = 0, n = 0, boxes = 0;
  const where = (): [number, number] => { const o = D.at[Section.view] + View.at; return [D.f[o], D.f[o + 2]]; }, stood = (): number => D.u[D.at[Section.view] + View.stood];
  const run = (ticks: number, what: string): void => { at = what; for (let i = 0; i < ticks; i++) {
    const keys = sampleHand(game.numbersOf(), input, cam, canvas); game.step(1 / 60, keys); input.pressed.clear(); input.clicked = 0;
    const o = D.at[Section.view], f: [number, number, number] = [D.f[o + View.follow], D.f[o + View.follow + 1], D.f[o + View.follow + 2]]; cam.step(1 / 60, f, null);
    for (const a of [0.5, 1]) { game.frame(a, cam.target); D.fresh(); }
    const h = D.at[Section.head]; boxes = Math.max(boxes, D.u[h + Head.count]); sum += game.ms.draw; n++; worst = Math.max(worst, game.ms.draw);
    if (D.u[h + Head.count] !== D.n[Section.geo] || D.n[Section.view] !== 1 || D.n[Section.sky] !== 1) throw new Error('the sections do not agree'); } };
  try {
    game.start({ level: bytes(file), words, mannequin, mannequinBin, cast, castLook, seed: 5 });
    { const o = D.at[Section.view]; cam.target = [D.f[o + View.follow], D.f[o + View.follow + 1], D.f[o + View.follow + 2]]; cam.rebuild(); }
    game.commands.god(1);
    run(60, 'a second standing');
    input.pressed.add('F5'); run(2, 'a save by hand');
    const a = where(); input.keys.add('KeyW'); run(90, 'a walk'); input.keys.clear(); const b = where();
    input.buttons = 1; input.clicked = 1; run(30, 'a shot'); input.buttons = 0;
    const s0 = stood(); game.commands.load(BREAK ? 'manual.9' : 'manual.0'); run(2, 'a load'); const c = where(), loaded = stood() !== s0, walked = Math.hypot(b[0] - a[0], b[1] - a[1]) > 0.3;
    const s1 = stood(); game.commands.restart(''); run(120, 'a restart'); const again = stood() !== s1;
    // (a tutorial keeps its own checkpoints and takes none by hand)
    const taught = /^tut-/.test(file), back = loaded && Math.hypot(c[0] - a[0], c[1] - a[1]) < 0.1;
    const went = walked || SKY.has(m.id);
    line(again && went && (taught || back), `${m.id}: started, walked${walked ? '' : ' (nowhere)'}, shot, ${taught ? 'taught' : 'saved and went back'}, restarted`, `${boxes} boxes · sections ${(sum / n).toFixed(2)} ms a tick, worst ${worst.toFixed(1)}${again ? '' : ' · the restart was not told'}${taught || back ? '' : ` · the load left him at ${c.map(x => x.toFixed(1))}, saved at ${a.map(x => x.toFixed(1))}`}`);
  } catch (e) { line(false, `${m.id}: stopped in ${at}`, String((e as Error)?.message ?? e).slice(0, 400)); }
  try { if (game.h) game.X.face_drop(game.h); } catch { /* lost to its panic */ }
}
// WHAT IS COMMANDED BEFORE A GAME IS MADE is what it is staged by: a hard one's tables are another's than a standard one's, and a standard one's are its own again
if (!only.length) { const tables = (f: (g: DrawGame) => void): string => { const g = new DrawGame({ exports: instance.exports, bytes: 0, fetchMs: 0, compileMs: 0 }), X = g.X; f(g);
    g.start({ level: bytes('depot.json'), words: '?levelFile=depot.json&roomclear&squad4', mannequin, mannequinBin, cast, castLook, seed: 5 });
    X.face_state_words(g.h); const p = X.face_out_ptr() >>> 0, w = new Uint32Array(g.memory.buffer.slice(p, p + (X.face_out_len() >>> 0))), off: string[] = [];
    for (let i = 0; i < X.face_state_n(); i++) { X.face_state_name(i); off.push(`${new TextDecoder().decode(new Uint8Array(g.memory.buffer, X.face_out_ptr() >>> 0, X.face_out_len() >>> 0))} ${w[i]}`); }
    X.face_drop(g.h); return off.filter(x => /^k /.test(x)).join(); };
  const standard = tables(() => { }), hard = tables(g => g.commands.difficulty(1)), lever = tables(g => g.commands.lever('ai', 'hard')), refused = tables(g => g.commands.lever('ai', 'nonsense')), after = tables(() => { });
  line(standard === after && standard === refused && hard !== standard && lever !== standard && lever !== hard, 'hard, and one lever of it, are what a game is staged by; a word that is no rung is nothing, and the next game is its own'); }
console.log(fails ? `=== play: ${fails} FAILED ===` : '=== play: ALL PASS ===');
process.exit(fails ? 1 : 0);
