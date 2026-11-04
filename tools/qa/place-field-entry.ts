// THE PLACE FIELD'S TEST PAGE, the half that runs in the browser (tools/qa/place-field-page.cjs bundles this with `bun build` and loads it in a blank page on
// the software WebGPU adapter). ONLY the look prototype's place field (src/render/placeField.ts + shaders/place_field.wgsl): no game, no renderer. It builds a
// small scene of its own (a yard, a room with a door, a darker room behind a shared wall), fills a stand-in for the radiance cache (its slab of light falling on the floor), solves the
// field and hands it back, then walks the eye between the room and the yard.
import { PlaceField, PLACE, PLACE_W, PLACE_H, PLACE_RES } from '../../src/render/placeField';
import { makeBox, BoxFlag, type Box } from '../../src/scene/boxes';
import { WORLD } from '../../src/scene/world';

const b64 = (a: ArrayBufferView): string => { const u = new Uint8Array(a.buffer, a.byteOffset, a.byteLength); let s = ''; for (let i = 0; i < u.length; i += 0x8000) s += String.fromCharCode(...u.subarray(i, i + 0x8000)); return btoa(s); };
/** a float as the 16 bits of a half (round to nearest; the values here are small and positive) */
const half = (() => { const f = new Float32Array(1), u = new Uint32Array(f.buffer); return (v: number): number => { f[0] = v; const x = u[0], s = (x >>> 16) & 0x8000, e = ((x >>> 23) & 0xff) - 112, m = x & 0x7fffff;
  if (e <= 0) { if (e < -10) return s; const mm = (m | 0x800000) >>> (1 - e); return s | ((mm + 0x1000) >>> 13); } if (e >= 31) return s | 0x7c00; return (s | (e << 10) | (m >>> 13)) + ((m & 0x1000) ? 1 : 0); }; })();

interface RunOpts {
  /** the room's door: 'open' = a 1 m gap with its leaf swung clear, 'shut' = the leaf across the gap, 'wide' = a 2.4 m opening and no leaf */
  door: 'open' | 'shut' | 'wide';
  /** room A's wall to the yard has a window in it: a sill, a pane of glass and a lintel, no one of them a wall's height */
  window?: boolean;
  /** the BREAK path: no box is a wall (the map is all clear) */
  noWalls?: boolean;
  place?: Partial<typeof PLACE>;
  /** drawn frames to step after the first (which solves to rest), with the light as it is */
  frames?: number;
}
/** the scene's light: the yard holds 0.1; room A (x 10‥16, z 8‥14) falls from 0.03 at its x = 10 wall to 0.005 at x = 16; room B (x 16‥22) holds 0.001;
 *  one probe in room A (13.5, 10.5) holds nothing, as one inside a crate does */
const YARD = 1.5, A_NEAR = 0.45, A_FAR = 0.075, B_ROOM = 0.015;   // (a grey day's numbers: open ground holds more than 1, which the field's open-air words must count as it counts a dim yard)
const lightAt = (x: number, z: number): number => {
  if (z > 8 && z < 14) { if (x > 10 && x < 16) return A_NEAR * Math.pow(A_FAR / A_NEAR, (x - 10) / 6); if (x > 16 && x < 22) return B_ROOM; }
  return YARD; };
function scene(o: RunOpts): { statics: Box[]; dynamics: Box[] } {
  const wall = (x0: number, z0: number, x1: number, z1: number): Box => makeBox({ c: [(x0 + x1) / 2, 1.3, (z0 + z1) / 2], h: [Math.max((x1 - x0) / 2, 0.1), 1.3, Math.max((z1 - z0) / 2, 0.1)] });
  const gap = o.door === 'wide' ? [11.8, 14.2] : [12.5, 13.5];
  const west = o.window ? [wall(10, 8, 10, 10.4), wall(10, 11.6, 10, 14), makeBox({ c: [10, 0.45, 11], h: [0.1, 0.45, 0.6] }), makeBox({ c: [10, 1.45, 11], h: [0.02, 0.55, 0.6], flags: BoxFlag.Glass | BoxFlag.NoShadow }), makeBox({ c: [10, 2.3, 11], h: [0.1, 0.3, 0.6] })] : [wall(10, 8, 10, 14)];
  const statics = [wall(10, 8, 22, 8), ...west, wall(16, 8, 16, 14), wall(22, 8, 22, 14), wall(10, 14, gap[0], 14), wall(gap[1], 14, 22, 14),
    makeBox({ c: [5, 0.4, 20], h: [1, 0.4, 1] }),                                   // a crate in the yard: not a wall
    makeBox({ c: [13, 2.3, 14], h: [0.5, 0.2, 0.1] }),                              // the door's lintel: not a wall
    makeBox({ c: [16, 2.7, 11], h: [6.1, 0.1, 3.1] })];                             // the two rooms' ceiling: a roof over their cells, so the yard's are the cells under the open sky
  const dynamics: Box[] = [];
  if (o.door === 'shut') dynamics.push(makeBox({ c: [13, 1, 14], h: [0.5, 1, 0.03], flags: BoxFlag.Dynamic }));
  if (o.door === 'open') dynamics.push(makeBox({ c: [12.5, 1, 14.5], h: [0.5, 1, 0.03], yaw: Math.PI / 2, flags: BoxFlag.Dynamic }));
  dynamics.push(makeBox({ c: [13, 0.9, 11], h: [0.25, 0.9, 0.2], flags: BoxFlag.Dynamic }));   // a man in the room: not a wall
  return o.noWalls ? { statics: [], dynamics: [] } : { statics, dynamics };
}

let device: GPUDevice | null = null; const gpuErrors: string[] = [];
async function boot(): Promise<{ adapter: string }> {
  const ad = await navigator.gpu.requestAdapter(); if (!ad) throw new Error('no WebGPU adapter');
  device = await ad.requestDevice(); device.addEventListener('uncapturederror', e => { gpuErrors.push(String((e as GPUUncapturedErrorEvent).error?.message).slice(0, 400)); });
  const info = (ad as unknown as { info?: { vendor?: string; architecture?: string; description?: string } }).info;
  return { adapter: info ? `${info.vendor ?? ''} ${info.architecture ?? ''} ${info.description ?? ''}`.trim() : '?' };
}

async function run(o: RunOpts) {
  const dev = device!; const saved = { ...PLACE }; Object.assign(PLACE, o.place ?? {});
  // the cache's stand-in: a metre a cell, three cells high, seven slabs; only the third (the light on a face that looks up: what falls on the floor) is filled
  const nx = Math.round(WORLD.rcSize[0]), ny = 4, nz = Math.round(WORLD.rcSize[2]), tex = dev.createTexture({ label: 'dice stand-in', size: [nx, ny * 7, nz], dimension: '3d', format: 'rgba16float', usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST });
  const fill = (dim: number): void => { const px = new Uint16Array(nx * ny * 7 * nz * 4);
    for (let k = 0; k < nz; k++) for (let i = 0; i < nx; i++) { const x = WORLD.rcMin[0] + i + 0.5, z = WORLD.rcMin[2] + k + 0.5;
      const inWall = x === 13.5 && z === 10.5, v = half(inWall ? 0 : lightAt(x, z) * (z > 8 && z < 14 && x > 10 && x < 16 ? dim : 1));
      for (let j = ny * 2; j < ny * 3; j++) { const q = ((k * ny * 7 + j) * nx + i) * 4; px[q] = px[q + 1] = px[q + 2] = v; px[q + 3] = 0; } }   // (this slab's fourth number is what a portal took out of the face to send itself, which the field adds back: rc.wgsl rcDice. None here)
    dev.queue.writeTexture({ texture: tex }, px, { bytesPerRow: nx * 8, rowsPerImage: ny * 7 }, [nx, ny * 7, nz]); };
  fill(1);
  const view = tex.createView(), place = new PlaceField(dev, view); await place.whenReady;
  const { statics, dynamics } = scene(o);
  const one = (dt: number, sam: [number, number] | null, snap: boolean): void => { place.setBoxes(statics, dynamics); const enc = dev.createCommandEncoder(); place.step(enc, view, dt, sam, snap); dev.queue.submit([enc.finish()]); };
  const DT = 1 / 60, ROOM: [number, number] = [13, 11], OUT: [number, number] = [5, 20], K = PLACE_W * PLACE_H;
  const t0 = performance.now(); one(DT, ROOM, true); await dev.queue.onSubmittedWorkDone(); const msSettle = performance.now() - t0;
  const rest = await place.readBack();
  for (let i = 0; i < (o.frames ?? 0); i++) one(DT, ROOM, false);
  const later = await place.readBack();
  // the eye: from the room out into the yard (it closes), then from the yard into the room (it opens), 0.35 s each
  const walk = async (from: [number, number], to: [number, number]): Promise<number[]> => { one(DT, from, true); const a = (await place.readBack())[K]; for (let i = 0; i < 21; i++) one(DT, to, false); const b = (await place.readBack())[K]; for (let i = 0; i < 600; i++) one(DT, to, false); return [a, b, (await place.readBack())[K]]; };
  const closing = await walk(ROOM, OUT), opening = await walk(OUT, ROOM);
  // the light changes: room A's lamp goes down to a tenth, with nothing told to the field: how far the room's level has followed after 0.5 s and after 3 s
  one(DT, ROOM, true); fill(0.1); const follow: number[] = [];
  for (let i = 1; i <= 180; i++) { one(DT, ROOM, false); if (i === 30 || i === 180) { const f = await place.readBack(); follow.push(f[Math.floor((11 - WORLD.rcMin[2]) * PLACE_RES) * PLACE_W + Math.floor((13 - WORLD.rcMin[0]) * PLACE_RES)]); } }
  const t1 = performance.now(); for (let i = 0; i < 30; i++) one(DT, ROOM, false); await dev.queue.onSubmittedWorkDone(); const msFrame = (performance.now() - t1) / 30;
  const out = { w: PLACE_W, h: PLACE_H, res: PLACE_RES, min: [WORLD.rcMin[0], WORLD.rcMin[2]], rest: b64(rest), later: b64(later), mask: b64(place.mask), closing, opening, follow, sky: rest[K + 2], mid: rest[K + 3], msSettle, msFrame, drawn: place.drawn, swept: place.swept, place: { ...PLACE }, gpuErrors: gpuErrors.slice() };
  place.destroy(); tex.destroy(); Object.assign(PLACE, saved); return out;
}
(window as unknown as { __place: unknown }).__place = { boot, run, light: { YARD, A_NEAR, A_FAR, B_ROOM } };
