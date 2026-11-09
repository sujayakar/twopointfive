// THE HANDS-OFF PLAYER (rust/game_run hands_off.rs), with no page: a benchmark's scenes (host/benchScenes.ts) played from their own addresses as the page would play them, through a camera that
// follows the man as the page's does. Each is taken, or is being taken when its seconds are up: orders given, rounds fired, defenders down.
//   bun run tools/draw/hands-check.ts [scene…] [--trace] [--seeds 1,2,3]      exit 0 · 1 and each that was not · 2 no such scene
//   HANDS_BREAK=1 …      nobody plays the seat: every scene must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/start-pack.ts';
import { DrawGame } from '../../src/host/drawgame';
import { sampleHand } from '../../src/draw/hand';
import { FollowCamera } from '../../src/host/camera';
import { BENCH_SCENES } from '../../src/host/benchScenes';
import { benchSceneSearch } from '../../src/ui/benchrun';
import { Section, View } from '../../src/draw/sections';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.HANDS_BREAK === '1', args = process.argv.slice(2), trace = args.includes('--trace'), at = args.indexOf('--seeds');
const seeds = at >= 0 ? args[at + 1].split(',').map(Number) : null, only = args.filter((a, i) => !a.startsWith('--') && (at < 0 || i !== at + 1));
const wasm = faceHere(); if (!wasm) { console.error('hands-check: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const instance = new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(wasm))), { env: { face_roll: () => { throw new Error('the game asked a die'); } } });
const [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT);
const bytes = (f: string): Uint8Array => new Uint8Array(readFileSync(resolve(ROOT, 'levels', f)));
let fails = 0;
{ const stray = only.filter(id => !BENCH_SCENES.some(s => s.id === id)); if (stray.length) { console.error(`hands-check: no scene ${stray.join(', ')}`); process.exit(2); } }
for (const S of BENCH_SCENES.filter(s => !only.length || only.includes(s.id))) for (const seed of seeds ?? [S.seed]) {
  const words = `?${benchSceneSearch({ ...S, seed }, 0).replace(/^\?/, '')}`, file = new URLSearchParams(words).get('levelFile') ?? 'office.json';
  const game = new DrawGame({ exports: instance.exports, bytes: 0, fetchMs: 0, compileMs: 0 }), D = game.drawn, cam = new FollowCamera(), input = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 360 }, canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 };
  const follow = (): [number, number, number] => { const o = D.at[Section.view] + View.follow; return [D.f[o], D.f[o + 1], D.f[o + 2]]; };
  if (S.hard) game.commands.difficulty(1);
  game.start({ level: bytes(file), words, mannequin, mannequinBin, cast, castLook, seed });
  cam.aspect = 1280 / 720; cam.target = follow(); cam.rebuild();
  if (S.god) game.commands.god(1);
  if (!BREAK) game.handsOff({ ...S.plan, seed, cap: S.secs });
  game.tell(); const left0 = game.hands()?.left ?? 0, t0 = performance.now(); let ms = 0, ticks = 0, words1 = '';
  try {
    for (; ticks < S.secs * 60 + 60 && !game.hands()?.end; ticks++) { const keys = sampleHand(game.numbersOf(), input, cam, canvas); game.step(1 / 60, keys); ms += game.ms.step; const o = D.at[Section.view] + View.aim; cam.easeYaw(1 / 60, false); cam.step(1 / 60, follow(), [D.f[o], D.f[o + 1], D.f[o + 2]]); D.fresh(); }
  } catch (e) { words1 = String((e as Error).message).slice(0, 300); }
  // the run's own end, as a card of it has it: the verdict's lines, with every man
  let carded = ''; if (!game.lost) { for (let i = 0; i < 600 && !game.verdict(); i++) game.step(1 / 60, sampleHand(game.numbersOf(), input, cam, canvas)); const V = game.verdict(), E = game.end().verdict;
    if (!V !== !E) carded = `the card has ${E ? 'an end' : 'none'}, the game ${V ? 'one' : 'none'}`; else if (V && E && !(E.won === V.won && E.word === V.word && E.why === V.why && Math.abs(E.secs - V.secs) < 1e-6 && E.approach === V.approach && E.lost === V.lost && E.of === V.of && E.foes.down === V.foesDown && E.foes.of === V.foesOf && E.void === V.void && E.clean === V.clean && E.men.length === E.of && E.men.filter(m => m.state !== 'up').length === E.lost && E.men.every(m => m.name && m.path.length >= 3))) carded = `the card's end is not the game's: ${JSON.stringify(V)} against ${JSON.stringify({ ...E, men: E.men.map(m => [m.name, m.state, m.path.length]) })}`; }
  const h = game.lost ? null : game.hands(), ok = !carded && !words1 && !!h && (h.end === 'taken' || ((h.end === 'cap' || !h.end) && h.given >= 3 && h.shotAt > 0 && h.left < left0));
  if (!ok) fails++;
  if (carded) console.log(`    ${carded}`); console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${S.id} (${S.scn}, seed ${seed}): ${h ? `${h.end ?? 'cut'} at ${h.t.toFixed(0)} s, ${h.given} orders, ${h.up} of ${h.of} up, ${h.left} defenders left of ${left0}` : 'nobody played'}${words1 ? `\n    ${words1}` : ''} · ${(ms / Math.max(1, ticks)).toFixed(2)} ms a tick, ${((performance.now() - t0) / 1000).toFixed(0)} s`);
  if (trace && h) console.log(game.handsTrace());
  try { if (game.h) game.X.face_drop(game.h); } catch { /* lost */ }
}
console.log(`=== hands: ${fails ? `${fails} FAILED` : 'ALL PASS'} ===`);
process.exit(fails ? 1 : 0);
