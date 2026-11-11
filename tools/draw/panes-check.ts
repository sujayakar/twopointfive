// A PANE THAT OPENS IS TOLD TO THE DRAWN SMOKE (src/draw/world.ts `holes`): levels played by the hands-off player on the module, and wherever the game says the air goes through a pane that it did not go through
// the frame before, what the smoke's solver has baked there must have been said to be stale. Nothing is drawn.
//   bun run tools/draw/panes-check.ts [level…]        exit 0 · 1 a check failed
//   PANES_BREAK=1 …      no pane is told: it must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/start-pack.ts';
import { fake } from '../qa/here.ts';
import { DrawGame } from '../../src/host/drawgame';
import { FollowCamera } from '../../src/host/camera';
import { sampleHand } from '../../src/draw/hand';
import { DrawWorld } from '../../src/draw/world';
import { Section, View, Word } from '../../src/draw/sections';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.PANES_BREAK === '1', SECS = 120, LEVELS = process.argv.length > 2 ? process.argv.slice(2) : ['glasshouse'];
const wasm = faceHere(); if (!wasm) { console.error('panes-check: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const module = new WebAssembly.Module(new Uint8Array(readFileSync(wasm))), [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT);
let red = 0; const line = (ok: boolean, what: string, got = ''): void => { if (!ok) red++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${got ? `\n    ${got}` : ''}`); };
const device = new Proxy(fake('device'), { get: (t, p) => p === 'createBuffer' ? (d: GPUBufferDescriptor) => ({ size: d.size, destroy() { /* none */ }, getMappedRange: () => new ArrayBuffer(d.size), unmap() { /* none */ } }) : Reflect.get(t, p) }) as GPUDevice;
let opened = 0, told = 0; const missed: string[] = [];

for (const level of LEVELS) {
  const game = new DrawGame({ exports: new WebAssembly.Instance(module).exports, bytes: 0, fetchMs: 0, compileMs: 0 }), D = game.drawn, cam = new FollowCamera(), canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 };
  const input = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 360 };
  const words = `?levelFile=${level}.json&roomclear&squad4&difficulty=hard&seed=7`;
  game.start({ level: new Uint8Array(readFileSync(resolve(ROOT, 'levels', `${level}.json`))), words, mannequin, mannequinBin, cast, castLook, seed: 7 });
  const world = new DrawWorld(device, D); if (BREAK) (world as unknown as { holes(): void }).holes = () => { /* nobody says */ };
  const open = (): Set<number> => { const s = new Set<number>(); for (let i = 0, o = D.at[Section.openPanes]; i < D.n[Section.openPanes]; i++, o += Word.row) s.add(D.u[o + Word.w]); return s; };
  const at = (): void => { const o = D.at[Section.view]; cam.target = [D.f[o + View.follow], D.f[o + View.follow + 1], D.f[o + View.follow + 2]]; cam.rebuild(); };
  at(); game.frame(1, cam.target); D.fresh(); world.from(); world.upload();
  game.commands.god(1); game.commands.ammo(1); game.handsOff({ seed: 10, cap: 400 });
  let was = open(); world.stale.length = 0; const since = new Set<number>();
  for (let t = 0; t < SECS * 60; t++) {
    game.step(1 / 60, sampleHand(game.numbersOf(), input, cam, canvas)); at(); game.frame(0.5, cam.target); D.fresh(); world.from(); world.upload();
    for (const i of world.glassStatics) { const c = world.statics[i].c; if (world.stale.some(s => [0, 1, 2].every(a => s.lo[a] <= c[a] && c[a] <= s.hi[a]))) { since.add(i); told++; } }
    world.stale.length = 0;
    const now = open();
    for (const i of now) if (!was.has(i)) { opened++; if (!since.has(i)) missed.push(`${level} ${i} at ${(t / 60).toFixed(1)} s`); }
    was = now;
  }
}
line(opened > 0, `panes open in these ${LEVELS.length * SECS} s (${opened}; told ${told} times)`);
line(missed.length === 0, 'where the air goes through a pane, the smoke was told', missed.slice(0, 5).join(', '));
process.exit(red ? 1 : 0);
