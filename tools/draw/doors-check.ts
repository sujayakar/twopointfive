// EVERY DOOR'S STEP IS KEPT RIGHT (src/scene/placePlan.ts PlaceMap: a door measured again only when a leaf that hangs in it has moved): levels played by the hands-off
// player on the module, drawn a tick or two at a time as a page draws them, the map held after every frame to the same map made of every door measured afresh
// whenever the map's own two keys say that a wall or a leaf has moved, and a wall taken out of the level now and then. What a frame of it costs, beside the afresh, is printed.
//   bun run tools/draw/doors-check.ts [level…]        exit 0 · 1 a check failed
//   DOORS_BREAK=1 …      a static that goes is not told: the map must differ
//   DOORS_BREAK=2 …      a leaf that moves is not told: the steps must differ
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/character.ts';
import { fake } from '../qa/here.ts';
import { DrawGame } from '../../src/host/drawgame';
import { FollowCamera } from '../../src/host/camera';
import { sampleHand } from '../../src/draw/hand';
import { DrawWorld } from '../../src/draw/world';
import { doorways } from '../../src/draw/reach';
import { Section, View } from '../../src/draw/sections';
import { PlaceMap, doorJambs, doorSteps, leafKey, placeKey, placeMask, placeWall, type PlaceGap } from '../../src/scene/placePlan';
import { BoxFlag } from '../../src/scene/boxes';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.DOORS_BREAK ?? '', SECS = 45, ASKED = process.argv.slice(2);
const ALL = ['office', 'compound', 'warehouse'], LEVELS = ALL.filter(l => !ASKED.length || ASKED.includes(l));
{ const stray = ASKED.filter(l => !ALL.includes(l)); if (stray.length) { console.error(`doors-check: no level ${stray.join(', ')} (${ALL.join(', ')})`); process.exit(2); } }
const wasm = faceHere(); if (!wasm) { console.error('doors-check: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const module = new WebAssembly.Module(new Uint8Array(readFileSync(wasm))), [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT);
let red = 0; const line = (ok: boolean, what: string, got = ''): void => { if (!ok) red++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${got ? `\n    ${got}` : ''}`); };
const device = new Proxy(fake('device'), { get: (t, p) => p === 'createBuffer' ? (d: GPUBufferDescriptor) => ({ size: d.size, destroy() { /* none */ }, getMappedRange: () => new ArrayBuffer(d.size), unmap() { /* none */ } }) : p === 'queue' ? { writeBuffer() { /* nowhere */ } } : t[p] }) as GPUDevice;
const same = (a: ArrayLike<number>, b: ArrayLike<number>): boolean => { if (a.length !== b.length) return false; for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return false; return true; };
const total = { swings: 0, steps: 0, masks: 0, kept: 0, afresh: 0, taken: 0, still: [] as string[] };

for (const level of LEVELS) {
  const words = level === 'office' ? '?roomclear&difficulty=hard&seed=7' : `?levelFile=${level}.json&roomclear&squad4&difficulty=hard&seed=7`;
  const game = new DrawGame({ exports: new WebAssembly.Instance(module).exports, bytes: 0, fetchMs: 0, compileMs: 0 }), D = game.drawn, cam = new FollowCamera(), canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 };
  const input = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 360 };
  game.start({ level: new Uint8Array(readFileSync(resolve(ROOT, 'levels', `${level}.json`))), words, mannequin, mannequinBin, cast, castLook, seed: 7 });
  const world = new DrawWorld(device, D), map = new PlaceMap(), gaps: PlaceGap[] = [];
  { const o = D.at[Section.view]; cam.target = [D.f[o + View.follow], D.f[o + View.follow + 1], D.f[o + View.follow + 2]]; cam.rebuild(); }
  game.frame(1, cam.target); D.fresh(); world.from(); world.upload(); for (const d of doorways(D)) gaps.push([d.hinge[0], d.hinge[1], d.hinge[0] + Math.cos(d.closedDir) * d.width, d.hinge[1] + Math.sin(d.closedDir) * d.width]);
  game.commands.god(1); game.commands.ammo(1); game.handsOff({ seed: 10, cap: 400 });
  let walls = NaN, leaves = NaN, afresh: number[] = [], jambs: ReturnType<typeof doorJambs> = [], frames = 0, swings = 0, steps = 0, masks = 0, taken = 0, takeAt = 600; const mine = { swing: [] as number[], rest: [] as number[] }, old = { swing: [] as number[], rest: [] as number[] };
  for (let t = 0; t < SECS * 60;) {
    const n = 1 + (frames++ % 3 === 2 ? 1 : 0); for (let k = 0; k < n; k++, t++) { const keys = sampleHand(game.numbersOf(), input, cam, canvas); game.step(1 / 60, keys); const o = D.at[Section.view]; cam.step(1 / 60, [D.f[o + View.follow], D.f[o + View.follow + 1], D.f[o + View.follow + 2]], null); }
    game.frame(0.4 + 0.5 * ((frames * 7) % 10) / 10, cam.target); D.fresh(); world.from(); world.upload();
    if (t >= takeAt) { takeAt += 900; const g = gaps[(taken * 5) % gaps.length], box = world.statics.filter(b => placeWall(b) !== 0).sort((x, y) => Math.hypot(x.c[0] - g[0], x.c[2] - g[1]) - Math.hypot(y.c[0] - g[0], y.c[2] - g[1]))[0];
      if (box) { box.flags |= BoxFlag.NoPrimary | BoxFlag.NoShadow; if (BREAK !== '1') world.staticsTold++; taken++; } }
    // (afresh: every door, whenever a wall or a leaf has moved, as the map's own two keys say)
    const t0 = performance.now(), wk = placeKey(world.statics, gaps), lk = leafKey(world.dynamics), moved = lk !== leaves || wk !== walls; if (wk !== walls) jambs = doorJambs(world.statics, gaps); if (moved) afresh = doorSteps(world.statics, world.dynamics, gaps, jambs); const t1 = performance.now();
    if (moved) { swings++; leaves = lk; walls = wk; total.afresh += gaps.length; }
    if (BREAK === '2' && swings > 3) map.doors.update = () => { /* nobody says */ };
    const t2 = performance.now(); map.update(world.statics, world.dynamics, gaps, world.staticsTold); const t3 = performance.now();
    (moved ? mine.swing : mine.rest).push(t3 - t2); (moved ? old.swing : old.rest).push(t1 - t0);
    if (!same(map.doors.steps, afresh)) steps++;
    if (!same(map.mask, placeMask(world.statics, world.dynamics, undefined, gaps, afresh))) masks++;
  }
  if (world.statics.length < 200) line(false, `${level}'s statics are here`, `${world.statics.length}`);
  total.swings += swings; total.steps += steps; total.masks += masks; total.kept += map.doors.measured; total.taken += taken; if (swings < 60) total.still.push(level);
  const mean = (a: number[]): string => (a.length ? a.reduce((x, y) => x + y, 0) / a.length : 0).toFixed(3);
  console.log(`${level}: ${gaps.length} doors, ${swings} of ${frames} frames a leaf or a wall moved in; ms a frame with one ${mean(mine.swing)} (the keys and every door ${mean(old.swing)}), with none ${mean(mine.rest)} (${mean(old.rest)}); the map drawn again ${map.drawn} times, ${taken} walls taken out, the statics told ${world.staticsTold} times`);
}
line(total.still.length === 0, `doors swung on every level (${total.swings} frames)`, total.still.join(', '));
line(total.steps === 0, 'every door\'s step is what it is measured afresh, after every frame', `${total.steps} frames differ`);
line(total.masks === 0, 'the map of the level is, cell for cell, the one drawn of the steps measured afresh', `${total.masks} frames differ`);
line(total.kept * 4 < total.afresh, 'a door nobody moved is not measured', `${total.kept} doors measured, ${total.afresh} afresh`);
process.exit(red ? 1 : 0);
