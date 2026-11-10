// THE LIGHTS OF EACH CELL (src/render/lights.ts cellsOf; common.wgsl lightsAt), held to what the walk over every light would find: lights of every kind come, go, move, change their range and
// their reach rows for some hundred uploads, and after each one every point asked must find in its cell's bits every light that is in range of it and not walled off from its cell, and no other
// light but by a cell's width. What the buffer was sent is put together the way the card holds it.
//   bun run tools/qa/lightcells-check.ts        exit 0 · 1 a check failed
//   LIGHTCELLS_BREAK=1 …      a light that moves is taken for one that stands: the first line must fail
//   LIGHTCELLS_BREAK=2 …      a reach row that changes is taken for the same row: the first line must fail
import { fake } from './here.ts';
import { LightSet, MAX_LIGHTS, REACH_CELLS_AT, type RtLight } from '../../src/render/lights';
import { REACH_ROW_WORDS } from '../../src/render/reach';
import { WORLD } from '../../src/scene/world';

const BREAK = process.env.LIGHTCELLS_BREAK ?? ''; let seed = 11; const roll = () => { seed = Math.imul(seed ^ seed >>> 15, seed | 1); seed ^= seed + Math.imul(seed ^ seed >>> 7, seed | 61); return ((seed ^ seed >>> 14) >>> 0) / 4294967296; };
const card = new Uint32Array(REACH_CELLS_AT + WORLD.gridW * WORLD.gridH * 2); let wrote = 0;
const queue = { writeBuffer: (_: unknown, at: number, data: ArrayBuffer | Uint32Array, from = 0, size?: number) => { if (!(mine && _ === mine)) return; const bytes = data instanceof ArrayBuffer ? new Uint32Array(data, from, (size ?? data.byteLength) / 4) : data; card.set(bytes, at / 4); wrote++; } };
let mine: unknown = null;
const device = new Proxy(fake('device'), { get: (t, p) => p === 'queue' ? queue : p === 'createBuffer' ? (d: { size: number; label?: string }) => ({ ...d, destroy() { /* none */ } }) : (t as Record<PropertyKey, unknown>)[p] }) as unknown as GPUDevice;
const set = new LightSet(device); mine = set.reachBuf; card.fill(0xffffffff);
const rows = new Map<RtLight, Uint32Array>(), newRow = (): Uint32Array => Uint32Array.from({ length: REACH_ROW_WORDS }, () => roll() < 0.4 ? 0 : (roll() * 4294967296) >>> 0);
const X0 = WORLD.min[0], Z0 = WORLD.min[2], WX = WORLD.gridW * WORLD.gridCell, WZ = WORLD.gridH * WORLD.gridCell, place = (): [number, number, number] => [X0 - 6 + roll() * (WX + 12), roll() * 5, Z0 - 6 + roll() * (WZ + 12)];
const add = (): void => { const l = set.add({ name: 'l', group: 'g', kind: ([0, 0, 1, 1, 3, 2] as const)[Math.floor(roll() * 6)], pos: place(), dir: [0, -1, 0], color: [1, 1, 1], intensity: 5, range: 1 + roll() * 14, innerDeg: 20, outerDeg: 40, radius: 0.1, volumetric: 1, owner: 0, enabled: true, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: 5 } as RtLight); if (roll() < 0.8) rows.set(l, newRow()); };
for (let i = 0; i < 30; i++) add();
let missed = 0, extra = 0, held = 0, asked = 0, differ = 0, quiet = 0, still = 0;
for (let step = 0; step < 300; step++) {
  const changes = step % 5 !== 4;
  if (changes) {
    for (const l of set.lights) { if (roll() < 0.2) l.pos = place(); if (roll() < 0.1) l.range = 1 + roll() * 14; if (roll() < 0.08) l.enabled = !l.enabled; if (roll() < 0.1 && rows.has(l)) rows.set(l, newRow()); }
    if (roll() < 0.3 && set.lights.length < 80) add(); if (roll() < 0.2) set.remove(set.lights[Math.floor(roll() * set.lights.length)]); if (roll() < 0.03) set.forgetSent();
  }
  const S = set as unknown as { cellsFrom: Float64Array; slotLight: RtLight[]; reachSent: Uint32Array; reachSentI: Int32Array; reachData: Uint32Array };
  if (BREAK === '1') S.slotLight.forEach((l, s) => { S.cellsFrom[s * 3] = l.pos[0]; S.cellsFrom[s * 3 + 1] = l.pos[2]; });
  if (BREAK === '2') S.slotLight.forEach((l, s) => { const r = rows.get(l); if (r) S.reachSent.set(r, s * REACH_ROW_WORDS); });
  wrote = 0; const n = set.upload(step / 60, [], { row: l => rows.get(l) }); if (!changes) { still++; if (!wrote) quiet++; }
  for (let i = 0; i < set.cells.length; i++) if (card[REACH_CELLS_AT + i] !== set.cells[i]) differ++;
  for (let k = 0; k < 400; k++) {
    const P = [X0 + roll() * WX, roll() * 3, Z0 + roll() * WZ], cx = Math.floor((P[0] - X0) / WORLD.gridCell), cz = Math.floor((P[2] - Z0) / WORLD.gridCell), ci = cz * WORLD.gridW + cx;
    for (let s = 0; s < MAX_LIGHTS; s++) {
      const bit = (card[REACH_CELLS_AT + ci * 2 + (s >> 5)] >>> (s & 31)) & 1; asked++;
      if (s >= n) { if (bit) extra++; continue; }
      const l = set.lights[set.activeIdx[s]], row = rows.get(l), level = row ? (row[ci >> 4] >>> ((ci & 15) * 2)) & 3 : 3, d = Math.hypot(P[0] - l.pos[0], P[1] - l.pos[1], P[2] - l.pos[2]), gives = level !== 0 && (l.kind === 2 || d <= l.range);
      if (gives) { held++; if (!bit) missed++; } else if (bit && (level === 0 || Math.hypot(P[0] - l.pos[0], P[2] - l.pos[2]) > l.range + Math.SQRT2 * WORLD.gridCell + 0.02)) extra++;
    }
  }
}
let red = 0; const line = (ok: boolean, what: string, got: string): void => { if (!ok) red++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${got}`); };
line(missed === 0 && held > 10000, 'a light that can reach a point is in its cell', `${missed} missed of ${held} (${asked} asked)`);
line(extra === 0, 'and no light is in a cell it cannot reach, nor a slot with no light', `${extra} too many`);
line(differ === 0, 'the card holds the cells the set made', `${differ} words differ`);
line(quiet === still, 'nothing is sent on an upload where no light changed', `${quiet} of ${still} sent nothing`);
console.log(red ? `FAIL — ${red}` : 'PASS'); process.exit(red ? 1 : 0);
