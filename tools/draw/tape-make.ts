// A RUN MADE FOR ITS SOUNDS, by a scripted player with keys and a cursor over the module, taped: what no kept run is sure to hold (cans thrown, a lamp shot out, a breaker worked). A script walks by the floor's own
// way, points where he is told and presses; it reads where he stands and nothing else.
//   bun run tools/draw/tape-make.ts <script> <out.tape.gz>      exit 0 · 1 no such script, or it could not be done
import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { gzipSync } from 'node:zlib';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/start-pack.ts';
import { DrawGame, type HandsOffPlan } from '../../src/host/drawgame';
import { askOf } from '../../src/draw/ask';
import { HAND, sampleHand } from '../../src/draw/hand';
import { Section, View } from '../../src/draw/sections';
import { FollowCamera } from '../../src/host/camera';
import { HandTape } from '../../src/tape/handtape';
import type { Vec3 } from '../../src/math/vec';

interface Bot { wait(secs: number): void; press(code: string, secs?: number): void; hold(codes: string[], secs: number): void; aim(p: Vec3): void; click(p: Vec3, button?: number, secs?: number): void; go(to: Vec3, secs?: number): void; at(): Vec3; say(words: string): void }
interface Script { level: string; words: string; god?: boolean; noAi?: boolean; hands?: HandsOffPlan; play?(B: Bot): void }
const SCRIPTS: Record<string, Script> = {
  // a stun, a smoke and a frag at his own feet and ahead of him, and the lobby's lamps shot out: nobody thinks, and he cannot be hurt
  cans: { level: 'office.json', words: '?roomclear&squad4&freshstart=0&seed=1', god: true, noAi: true, play(B) {
    const p = B.at(), ahead = (m: number): Vec3 => [p[0] + m, 0.1, p[2]];
    B.wait(1); B.press('KeyN', 1); B.press('KeyN', 1); B.press('KeyV', 1);
    for (const [key, m] of [['Digit4', 3], ['Digit3', 5], ['Digit5', 9]] as const) { B.press(key, 2.5); B.click(ahead(m), 1, 0.1); B.wait(key === 'Digit4' ? 9 : 5); }
    B.press('Digit1', 2); for (const lamp of [[5.6, 2.2, 23.9], [8, 2.6, 20], [6, 2.6, 18]] as Vec3[]) for (let i = 0; i < 3; i++) { B.click(lamp, 1, 0.1); B.wait(0.4); }
    B.wait(2);
  } },
  // floors taken by a player nobody holds: orders, doors, rooms fought; on the first the breaker is thrown before any
  block: { level: 'block.json', words: '?levelFile=block.json&roomclear=1&squad4&seed=1', god: true, hands: { seed: 1, cap: 75, breaker: true }, play(B) { B.wait(77); } },
  compound: { level: 'compound.json', words: '?levelFile=compound.json&roomclear=6&squad4&seed=6', god: true, hands: { seed: 6, cap: 75 }, play(B) { B.wait(77); } },
};
const ROOT = resolve(import.meta.dir, '../..'), [name, to] = process.argv.slice(2), S = SCRIPTS[name]; if (!S || !to) { console.error(`tape-make: <${Object.keys(SCRIPTS).join(' | ')}> <out>`); process.exit(1); }
const wasm = faceHere(); if (!wasm) { console.error('tape-make: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const g = new DrawGame({ exports: new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(wasm))), { env: { face_roll: () => { throw new Error('the game asked a die'); } } }).exports, bytes: 0, fetchMs: 0, compileMs: 0 }), D = g.drawn;
const [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT), T = g.tape = new HandTape(HAND.count), seed = Number(new URLSearchParams(S.words).get('seed'));
g.start({ level: new Uint8Array(readFileSync(resolve(ROOT, 'levels', S.level))), words: S.words, mannequin, mannequinBin, cast, castLook, seed, named: { level: S.level, build: '' } });
const cam = new FollowCamera(), I = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 360 }, canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 }, ask = askOf(g), v3 = (o: number): Vec3 => [D.f[o], D.f[o + 1], D.f[o + 2]], view = (): number => D.at[Section.view];
cam.aspect = 1280 / 720; cam.target = v3(view() + View.follow); cam.rebuild();
if (S.god) g.commands.god(1); if (S.noAi) g.commands.ai(0); if (S.hands) g.handsOff(S.hands); g.tell();
const step = (): void => { g.step(1 / 60, sampleHand(g.numbersOf(), I, cam, canvas)); I.pressed.clear(); I.clicked = 0; D.fresh(); cam.easeYaw(1 / 60, false); cam.step(1 / 60, v3(view() + View.follow), v3(view() + View.aim)); };
const B: Bot = {
  wait: secs => { for (let i = 0; i < secs * 60; i++) step(); },
  press: (code, secs = 0.1) => { I.pressed.add(code); I.keys.add(code); step(); I.keys.delete(code); B.wait(secs); },
  hold: (codes, secs) => { for (const c of codes) { I.keys.add(c); I.pressed.add(c); } B.wait(secs); for (const c of codes) I.keys.delete(c); },
  aim: p => { const [x, y] = cam.project(p, 1280, 720); I.mouseX = Math.max(2, Math.min(1278, x)); I.mouseY = Math.max(2, Math.min(718, y)); },
  click: (p, button = 1, secs = 0.1) => { for (let i = 0; i < 20; i++) { B.aim(p); step(); } I.clicked |= button; I.buttons |= button; for (let i = 0; i < secs * 60; i++) { B.aim(p); step(); } I.buttons &= ~button; step(); },
  go: (p, secs = 20) => { const way = ask.way(B.at(), p, 6000) ?? [p]; let left = secs * 60;
    for (const q of way) while (left-- > 0) { const a = B.at(), dx = q[0] - a[0], dz = q[2] - a[2], d = Math.hypot(dx, dz); if (d < 0.35) break; const { fwd, right } = cam.planarBasis(), f = (dx * fwd[0] + dz * fwd[2]) / d, r = (dx * right[0] + dz * right[2]) / d; for (const [k, on] of [['KeyW', f > 0.38], ['KeyS', f < -0.38], ['KeyD', r > 0.38], ['KeyA', r < -0.38]] as const) if (on) I.keys.add(k); else I.keys.delete(k); step(); }
    for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) I.keys.delete(k); if (left <= 0) B.say(`did not get to ${p[0]}, ${p[2]}: at ${B.at().map(v => v.toFixed(1))}`); },
  at: () => v3(view() + View.at), say: words => console.log('  ' + words),
};
S.play?.(B); T.end(''); const gz = gzipSync(T.whole(), { level: 9 }); writeFileSync(to, gz); const t = g.tally(), h = g.hands();
console.log(`${name}: ${T.ticks} ticks, ${(gz.length / 1024).toFixed(0)} KB · ${t.stage}${h ? ` · ${h.given} orders, ${h.left} defenders left, ended ${h.end}` : ''}`);
