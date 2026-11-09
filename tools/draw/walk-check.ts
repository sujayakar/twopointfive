// A RUN'S TAPE PLAYS THE RUN (tape/handtape.ts, host/drawplay.ts), with no page: a run of the module's game is played with a walking, shooting, saving hand and commands between its ticks, taped as it
// goes, and the tape is walked on another game of the same module. Every mark is the run's, forward, and again after each seek back.
//   bun run tools/draw/walk-check.ts [level.json …]        exit 0 · 1
//   WALK_BREAK=1 …      one key of the walk is not handed over: the marks must part
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/start-pack.ts';
import { DrawGame } from '../../src/host/drawgame';
import { DrawPlay } from '../../src/host/drawplay';
import { HAND, sampleHand } from '../../src/draw/hand';
import { HandTape, HandWalk, TapeRefused, type TapeRow } from '../../src/tape/handtape';
import { FollowCamera } from '../../src/host/camera';
import { Man, Section, View } from '../../src/draw/sections';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.WALK_BREAK === '1';
const wasm = faceHere(); if (!wasm) { console.error('walk-check: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const instance = new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(wasm))), { env: { face_roll: () => { throw new Error('the game asked a die'); } } });
const [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT), own = { mannequin, mannequinBin, cast, castLook };
let fails = 0; const line = (ok: boolean, what: string, why = ''): void => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${why ? `\n    ${why}` : ''}`); };
const fresh = (): DrawGame => new DrawGame({ exports: instance.exports, bytes: 0, fetchMs: 0, compileMs: 0 });
const WORDS: Record<string, string> = { 'office.json': '?levelFile=office.json', 'compound.json': '?levelFile=compound.json&roomclear&squad4', 'pass.json': '?levelFile=pass.json&roomclear&squad4' }, only = process.argv.slice(2);
const TICKS = 1500;

/** a tape with its rows gone over (`n`: steps before it) */
function changed(bytes: Uint8Array, fn: (r: TapeRow, n: number) => void): Uint8Array {
  const t = new HandTape(HAND.count), w = new HandWalk(bytes, HAND.count); t.start(w.start);
  for (let r = w.next(); r; r = w.next()) { fn(r, t.ticks); if (r.kind === 'step') t.step(r.dt, r.hand, r.keys); else if (r.kind === 'order') t.order(r.bytes); else if (r.kind === 'mark') t.mark(r.mark); else if (r.kind === 'figures') t.figures(r.figures); else if (r.kind === 'try') t.again(); }
  return t.whole();
}
/** a die of the check's own */
function dice(n: number): () => number { return () => { n = Math.imul(n ^ n >>> 15, n | 1); n ^= n + Math.imul(n ^ n >>> 7, n | 61); return ((n ^ n >>> 14) >>> 0) / 4294967296; }; }
for (const [level, words] of Object.entries(WORDS)) { if (only.length && !only.includes(level)) continue;
  const g = fresh(), roll = dice(7), cam = new FollowCamera(), canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 }, D = g.drawn;
  const input = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 360 };
  g.tape = new HandTape(HAND.count); g.commands.god(1); g.commands.difficulty(1);
  g.start({ level: new Uint8Array(readFileSync(resolve(ROOT, 'levels', level))), words, ...own, seed: 11, named: { level, build: 'check' } });
  for (let i = 0; i < TICKS; i++) {
    if (i % 40 === 0) { input.keys.clear(); for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) if (roll() < 0.35) input.keys.add(k); if (roll() < 0.3) { input.buttons = 1; input.clicked = 1; } else input.buttons = 0; }
    if (roll() < 0.02) input.pressed.add(['KeyR', 'KeyC', 'Digit1', 'Digit2', 'KeyQ', 'KeyE', 'KeyF'][Math.floor(roll() * 7)]);
    input.mouseX += (roll() - 0.5) * 30; input.mouseY += (roll() - 0.5) * 30;
    if (i === 200 || i === 700) input.pressed.add('F5');
    if (i === 450) { g.commands.ammo(1); g.tell(); }
    if (i === 620 || i === 1300) g.commands.load('manual.0');
    if (i === 900) g.commands.restart('');
    g.step(1 / 60, sampleHand(g.numbersOf(), input, cam, canvas)); input.pressed.clear(); input.clicked = 0;
    const o = D.at[Section.view]; cam.step(1 / 60, [D.f[o + View.follow], D.f[o + View.follow + 1], D.f[o + View.follow + 2]], null);
  }
  let bytes = g.tape.whole(); const end = g.mark(true), last = g.saves().rows.map(r => r.label).join();
  if (BREAK) bytes = changed(bytes, (r, n) => { if (r.kind === 'step' && n === 100) r.keys = '\nKeyC'; });
  const h = fresh(), P = new DrawPlay(h, bytes); P.start(own); const D2 = h.drawn, clock = (): number => h.drawn.u[h.drawn.at[Section.view] + View.tick];
  let seen = 0, t0 = performance.now(); while (P.tick(seen++ % 3 === 0)) { /* on */ }
  const ms = performance.now() - t0, mine = h.mark(true);
  line(!P.parted && P.at === TICKS && P.total === TICKS && mine.die === end.die && mine.parts === end.parts && mine.state === end.state && h.saves().rows.map(r => r.label).join() === last, `${level}: ${TICKS} ticks walked to the run's own end, every mark its own`,
    `${(bytes.length / 1024).toFixed(0)} KB · walked in ${ms.toFixed(0)} ms${P.parted ? ` · parted at tick ${P.parted.tick}` : ''} · ${end.die} rolls, ${mine.die} walked`);
  // back across a load, the restart and a save, and on to the end again
  for (const to of [1100, 640, 230, 5, 1290]) { P.seek(to); const at = P.at, shown = clock(), stands = (h.tell(), clock()); while (P.tick(false)) { /* on */ } const m = h.mark(true);
    line(at === to && shown === stands && !P.parted && m.die === end.die && m.parts === end.parts && m.state === end.state, `${level}: back to tick ${to}, drawn there, and on to the same end`, P.parted ? `parted at tick ${P.parted.tick}` : `at ${at}, drawn at ${shown} of the game's own ${stands}`); }
  // lost to a panic on the way, it can still be gone back on
  { P.seek(800); try { h.X.face_debug_panic(h.h, 0); } catch { /* lost */ } const was = !!h.lost; P.seek(700); while (P.tick(false)) { /* on */ } const m = h.mark(true); line(was && !h.lost && m.parts === end.parts && m.state === end.state, `${level}: lost to a panic, it goes back to a moment before it and on to the same end`); }
  // watched another way (no fog, a tube), it is the same run; the picture has everybody, and the map and the floor have them seen or not
  { P.seek(0); h.viewAs(true, true); let most = 0; while (P.tick(true)) { const men = new Set<number>(); for (let i = 0, o = D2.at[Section.men]; i < D2.n[Section.men]; i++, o += Man.row) men.add(D2.u[o + Man.id]); most = Math.max(most, men.size); } const m = h.mark(true), bare = D2.u[D2.at[Section.fog]]; h.viewAs(false, null); const fogged = D2.n[Section.men], map = h.map(), floor = h.floor();
    line(!P.parted && m.die === end.die && m.parts === end.parts && m.state === end.state && bare === 0 && most >= fogged && map.men.length >= most && map.men.every(x => Number.isFinite(x.x + x.z + x.aim) && x.name !== undefined) && map.doors.length > 0 && floor.boxes.length > 60 && floor.boxes.length % 6 === 0, `${level}: watched with the fog off and through a tube, it is the same run`, `up to ${most} men drawn, ${fogged} through the fog, ${map.men.length} on the map, ${map.doors.length} doors, ${floor.boxes.length / 6} solid things on the floor`); }
  // played on another module, whose state hashes are its own: it is the run by its die and who stands where, and parts where one of those does
  if (!BREAK) { const at = 720, parts = (fn: (r: TapeRow) => void, same: boolean): DrawPlay => { const p = new DrawPlay(fresh(), changed(bytes, r => { if (r.kind === 'mark') r.mark.parts ^= 1; fn(r); }), same); p.start(own); while (p.tick(false)) { /* on */ } try { h.X.face_drop(p.game.h); } catch { /* */ } return p; }, none = (): void => { /* as it was */ };
    const one = (r: TapeRow, k: number): void => { if (r.kind === 'figures' && r.figures.tick === at) r.figures.men[0].x += k; };
    const other = parts(none, false), same = parts(none, true), near = parts(r => one(r, 0.09), false), far = parts(r => one(r, 0.11), false), die = parts(r => { if (r.kind === 'mark' && r.mark.tick === at) r.mark.die++; }, false), shot = parts(r => { if (r.kind === 'figures' && r.figures.tick === at) r.figures.shots++; }, false), down = parts(r => { if (r.kind === 'figures' && r.figures.tick === at) r.figures.men.at(-1)!.down = !r.figures.men.at(-1)!.down; }, false);
    line(!other.parted && other.words?.tick === 60 && same.parted?.tick === 60 && !near.parted && [far, die, shot, down].every(p => p.parted?.tick === at), `${level}: on another module it is the run where its die, the men's places and the rounds are`, [far, die, shot, down].map(p => p.parted?.why ?? 'not parted').join(' · ')); }
  // refused by name: another build's part, another thing
  { let why = ''; try { new DrawPlay(fresh(), bytes).start({ ...own, cast: castLook }); } catch (e) { why = e instanceof TapeRefused ? e.message : String(e); } let none = ''; try { new DrawPlay(fresh(), new Uint8Array([1, 2, 3])); } catch (e) { none = e instanceof TapeRefused ? e.message : String(e); }
    line(/different version/.test(why) && /isn.t a replay/.test(none), `${level}: a tape of another build's part, and a file that is no tape, are refused in words`, `${why} · ${none}`); }
  // a run lost to a panic: its tape ends there, in words
  { try { g.X.face_debug_panic(g.h, 0); } catch { /* lost */ } const w = new DrawPlay(fresh(), g.ended!.whole()); line(/stopped in/.test(w.ends) && w.total === TICKS, `${level}: a run lost to a panic ends its tape with the panic's words`, w.ends.slice(0, 120)); }
  try { h.X.face_drop(h.h); } catch { /* */ }
}
console.log(fails ? `=== walk: ${fails} FAILED ===` : '=== walk: ALL PASS ===');
process.exit(fails ? 1 : 0);
