// NO REACH MASK IS MADE IN THE FRAME THAT FIRST SHOWS ITS LIGHT (src/render/reachKeep.ts): storms played by the hands-off player on the module, drawn a tick or two at a
// time as a page draws them, with the keeper stood as the picture stands it and no GPU. Every stroke's mask is held, byte for byte, to what is worked out whole of the
// level as it stands, and the cells decided in the frame it shows in are counted.
//   bun run tools/draw/reach-check.ts [level…]        exit 0 · 1 a check failed
//   REACH_BREAK=1 …      nobody says what is coming: the line that counts the cells must fail
//   REACH_BREAK=2 …      a static that moves is not told: the line that holds the bytes must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/start-pack.ts';
import { fake } from '../qa/here.ts';
import { DrawGame } from '../../src/host/drawgame';
import { FollowCamera } from '../../src/host/camera';
import { sampleHand } from '../../src/draw/hand';
import { DrawWorld } from '../../src/draw/world';
import { DrawLights } from '../../src/draw/lights';
import { DrawReach, doorways } from '../../src/draw/reach';
import { Section, View } from '../../src/draw/sections';
import { ReachKeep } from '../../src/render/reachKeep';
import { REACH_WORK } from '../../src/render/reach';
import { BoxFlag, stopsSky, type Box } from '../../src/scene/boxes';
import type { RtLight, LightSet } from '../../src/render/lights';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.REACH_BREAK ?? '', SECS = 75, ASKED = process.argv.slice(2);
const ALL = ['office', 'compound', 'block', 'records', 'mousehole', 'warehouse'], LEVELS = ALL.filter(l => !ASKED.length || ASKED.includes(l));
{ const stray = ASKED.filter(l => !ALL.includes(l)); if (stray.length) { console.error(`reach-check: no level ${stray.join(', ')} (${ALL.join(', ')})`); process.exit(2); } }
const wasm = faceHere(); if (!wasm) { console.error('reach-check: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const module = new WebAssembly.Module(new Uint8Array(readFileSync(wasm))), [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT);
let red = 0; const line = (ok: boolean, what: string, got = ''): void => { if (!ok) red++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${got ? `\n    ${got}` : ''}`); };
const device = new Proxy(fake('device'), { get: (t, p) => p === 'createBuffer' ? (d: GPUBufferDescriptor) => ({ size: d.size, destroy() { /* none */ }, getMappedRange: () => new ArrayBuffer(d.size), unmap() { /* none */ } }) : p === 'queue' ? { writeBuffer() { /* nowhere */ } } : t[p] }) as GPUDevice;
/** the statics that stop a light of the sky, as one number (what the world's own count has to move with) */
const sig = (statics: readonly Box[]): number => { let h = 0; const buf = new Float64Array(1), bits = new Uint32Array(buf.buffer), add = (v: number): void => { buf[0] = v; h = (Math.imul(h ^ bits[0], 0x9E3779B1) + bits[1]) | 0; };
  for (const b of statics) { if (!stopsSky(b.flags)) continue; for (let k = 0; k < 3; k++) { add(b.c[k]); add(b.h[k]); } add(b.yaw ?? 0); add(b.round ? 1 : 0); add(b.cut ? 1 : 0); if (b.rot) for (const v of b.rot) add(v); } return h; };
const total = { strokes: 0, cells: 0, ms: 0, differ: 0, walls: 0, untold: 0, dry: [] as string[], shows: [] as string[] };

for (const level of LEVELS) {
  const words = level === 'office' ? '?roomclear&difficulty=hard&seed=7' : `?levelFile=${level}.json&roomclear&squad4&difficulty=hard&seed=7`;
  const game = new DrawGame({ exports: new WebAssembly.Instance(module).exports, bytes: 0, fetchMs: 0, compileMs: 0 }), D = game.drawn, cam = new FollowCamera(), canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 };
  const input = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 360 };
  game.start({ level: new Uint8Array(readFileSync(resolve(ROOT, 'levels', `${level}.json`))), words, mannequin, mannequinBin, cast, castLook, seed: 7 });
  const set = { lights: [] as RtLight[], ir: [] as RtLight[] } as unknown as LightSet, world = new DrawWorld(device, D), keep = new ReachKeep(world, set), lights = new DrawLights(set, D);
  { const o = D.at[Section.view]; cam.target = [D.f[o + View.follow], D.f[o + View.follow + 1], D.f[o + View.follow + 2]]; cam.rebuild(); }
  game.frame(1, cam.target); D.fresh(); world.upload(); lights.from();
  const reach = new DrawReach({ reach: keep, lights: set }, world, D, doorways(D));
  if (BREAK === '2') Object.defineProperty(world, 'moved', { get: () => 0, set() { /* nobody tells */ }, configurable: true });
  if (BREAK === '1') keep.coming = () => { /* nobody says */ };
  game.commands.god(1); game.commands.ammo(1); game.handsOff({ seed: 10, cap: 400 });
  const seen = new Set<RtLight>(), boot = keep.peek().solid; let was = sig(world.statics), said = world.moved, untold = 0, cells = 0, ms = 0, differ = 0, strokes = 0, walls = 0, frames = 0, taken = 0, takeAt = 900; const spends: number[] = [];
  for (let t = 0; t < SECS * 60;) {
    const n = 1 + (frames++ % 3 === 2 ? 1 : 0); for (let k = 0; k < n; k++, t++) { const keys = sampleHand(game.numbersOf(), input, cam, canvas); game.step(1 / 60, keys); const o = D.at[Section.view]; cam.step(1 / 60, [D.f[o + View.follow], D.f[o + View.follow + 1], D.f[o + View.follow + 2]], null); }
    game.frame(0.4 + 0.5 * ((frames * 7) % 10) / 10, cam.target); D.fresh();
    lights.from(); const c0 = REACH_WORK.cells, t0 = performance.now(); reach.from(); const spent = performance.now() - t0, made = REACH_WORK.cells - c0;
    for (const l of set.lights) { if (l.kind !== 2 || !l.transient || seen.has(l)) continue; seen.add(l); strokes++; cells += made; ms = Math.max(ms, spent); if (keep.peek().solid !== boot) walls++;
      const want = keep.wholeOf(l), got = keep.row(l); if (!want || !got || want.length !== got.length || want.some((w, i) => w !== got[i])) differ++; }
    // (some seconds in, and again: a ceiling slab goes, then a wall, as a blast leaves them: what a mask in hand stands on is moved from under it)
    if (t >= takeAt) { takeAt += 1500; const box = world.statics.filter(b => stopsSky(b.flags) && (taken % 2 ? b.h[1] > 1 && Math.min(b.h[0], b.h[2]) < 0.2 : b.c[1] - b.h[1] > 1.9)).sort((x, y) => y.h[0] * y.h[2] - x.h[0] * x.h[2])[0];
      if (box) { box.flags |= BoxFlag.NoShadow; world.moved++; if (taken % 2) world.walls.push([box.c[0] - box.h[0] - 1, box.c[2] - box.h[2] - 1, box.c[0] + box.h[0] + 1, box.c[2] + box.h[2] + 1]); taken++; } }
    world.upload();
    { const now = sig(world.statics); if (now !== was && world.moved === said) untold++; was = now; said = world.moved; }
    const k0 = performance.now(); keep.tickTimed(8); const k = performance.now() - k0; spends.push(k);
  }
  total.strokes += strokes; total.cells += cells; total.ms = Math.max(total.ms, ms); total.differ += differ; total.walls += walls; total.untold += untold; if (strokes < 3) total.dry.push(level);
  total.shows.push(`${level}: ${strokes} strokes, ${cells} cells decided in their frames, worst ${ms.toFixed(1)} ms, ${walls} on walls other than the level's, ${frames} frames, ${taken} taken out of the statics, statics moved ${world.moved} times; the keeper's frame: mean ${(spends.reduce((x, y) => x + y, 0) / spends.length).toFixed(2)} ms, ${spends.filter(x => x > 1).length} over 1 ms, ${spends.filter(x => x > 3).length} over 3, worst ${Math.max(...spends).toFixed(1)}`);
  console.log(total.shows.at(-1));
}
line(total.dry.length === 0, `every level had strokes (${total.strokes})`, total.dry.join(', '));
line(total.cells === 0, 'no cell is decided in the frame a stroke shows in', `${total.cells} cells, worst frame ${total.ms.toFixed(1)} ms`);
line(total.differ === 0, 'every stroke\'s mask is, byte for byte, what is worked out whole of the level as it stands', `${total.differ} differ of ${total.strokes}`);
line(total.untold === 0, 'the world counts every move of a static that stops a light of the sky', `${total.untold} frames had one and did not`);
process.exit(red ? 1 : 0);
