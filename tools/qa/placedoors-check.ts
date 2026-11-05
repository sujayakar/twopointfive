// A DOOR, HOWEVER IT HANGS, KEEPS TWO PLACES TWO (src/render/placeField.ts placeMask `gaps` + PLACE.minWide; the look page's One exposure row shows every
// place by its own level, so two places the map takes for one are shown wrong together). The maintainer on his GPU, 11-01, of the pass at midday: "it seems easy
// to get into a state where the exposure is off, like moving between these two rooms": the kitchen was nearly black with Sam in it. Found here, not on a GPU:
// with its yard door wide open the map made the kitchen one place with the open yard (the opening and half the window beside it read as one run five cells
// wide, past a doorway's four: the 0.5 m pier between them was "a post" and one pane a hair under the width a wall piece needs), and a leaf hanging at about
// 65 degrees did the same to the bunk room.
// No GPU: the map is the CPU's (placeMask is pure). What is read: on the pass, with each door swung through its whole travel (every 0.05 rad, the others
// shut) and with every door open at once, no room reaches another room or a cell under the open sky without crossing a doorway cell.
//   bun run tools/qa/placedoors-check.ts   (exit 0 = all pass; PLACEDOORS_BREAK=1 gives the map no doors and the old width, and must fail)
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { placeMask, PLACE, PLACE_W, PLACE_RES } = await import(`${ROOT}/src/render/placeField.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
const BRK = process.env.PLACEDOORS_BREAK === '1'; let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== placedoors: the pass's rooms as the place map has them, every door at every angle${BRK ? ' (no doors told, the old width: must fail)' : ''} ===`);
const l = console.log, w = console.warn; console.log = console.warn = () => { };
const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/pass.json`, 'utf8'))), seed: 1 }); for (let i = 0; i < 10; i++) W.step(1 / 60);
console.log = l; console.warn = w;
const g = W.game, world = g.engine.world;
if (BRK) PLACE.minWide = 0.3;
const GAPS = BRK ? [] : g.doors.list.map((d: any) => { const [hx, hz] = d.def.hinge, a = d.def.closedDir, wd = d.def.width; return [hx, hz, hx + Math.cos(a) * wd, hz + Math.sin(a) * wd]; });
const ROOMS: Record<string, [number, number]> = { tea: [17, 8], kitchen: [25, 8], store: [31, 8], bunk: [31, 13.5] };   // a point well inside each (levels/pass.json)
const cellOf = (x: number, z: number) => Math.floor((z - WORLD.rcMin[2]) * PLACE_RES) * PLACE_W + Math.floor((x - WORLD.rcMin[0]) * PLACE_RES);
/** what the room at `from` reaches over free cells that are not a doorway's: 'sky' and the other rooms' names ('' = it is a place of its own) */
function reaches(from: [number, number]): string {
  const m = placeMask(world.statics, world.dynamics, undefined, GAPS), seen = new Uint8Array(m.length), q = [cellOf(from[0], from[1])]; seen[q[0]] = 1; let sky = false;
  while (q.length) { const k = q.pop()!; if (m[k] & 4) sky = true;
    for (const d of [1, -1, PLACE_W, -PLACE_W]) { const t = k + d; if (t < 0 || t >= m.length || seen[t] || m[t] === 255 || (m[t] & 3) === 1) continue; if (Math.abs(d) === 1 && Math.floor(t / PLACE_W) !== Math.floor(k / PLACE_W)) continue; seen[t] = 1; q.push(t); } }
  return [sky ? 'the open sky' : '', ...Object.entries(ROOMS).filter(([, p]) => p !== from && seen[cellOf(p[0], p[1])]).map(([n]) => n)].filter(Boolean).join(', ');
}
const hang = (name: string, a: number) => { const d = g.doors.byName(name); d.latched = a === 0; d.vel = 0; d.angle = a; };
const settle = () => { for (let i = 0; i < 2; i++) W.step(1 / 60); };
{ for (const o of g.doors.list) hang(o.def.name, 0); settle(); const r = Object.entries(ROOMS).map(([n, p]) => [n, reaches(p)]).filter(([, x]) => x);
  check(r.length === 0 && Object.values(ROOMS).every(p => placeMask(world.statics, world.dynamics, undefined, GAPS)[cellOf(p[0], p[1])] === 2), 'with every door shut each of the four rooms is a place of its own, under a roof', r.map(([n, x]) => `${n} reaches ${x}`).join('; ')); }
{ const bad: string[] = []; let looked = 0;
  for (const d0 of g.doors.list) { for (const o of g.doors.list) hang(o.def.name, 0);
    for (let a = d0.minAuth; a <= d0.maxAuth + 1e-6; a += 0.05) { hang(d0.def.name, Math.abs(a) < 0.02 ? 0 : a); settle(); looked++;
      for (const [n, p] of Object.entries(ROOMS)) { const x = reaches(p); if (x) bad.push(`${d0.def.name} at ${a.toFixed(2)}: ${n} reaches ${x}`); } } }
  check(looked > 300 && bad.length === 0, 'no door, at any angle of its travel, makes a room one place with another room or with the open air', `${looked} door positions looked at, ${bad.length} joined${bad.length ? `: ${bad.slice(0, 4).join('; ')}` : ''}`); }
{ const bad: string[] = [];
  for (const s of [1, -1, 0.8, -0.8, 0.55, -0.55]) { for (const o of g.doors.list) hang(o.def.name, s > 0 ? o.maxAuth * s : -o.minAuth * s); settle(); for (const [n, p] of Object.entries(ROOMS)) { const x = reaches(p); if (x) bad.push(`all at ${s}: ${n} reaches ${x}`); } }
  check(bad.length === 0, 'nor do all seven doors standing open at once, either way', bad.slice(0, 4).join('; ')); }
{ for (const o of g.doors.list) hang(o.def.name, 0); settle(); const m = placeMask(world.statics, world.dynamics, undefined, GAPS); let n = 0; const at: string[] = [];
  // A SHUT DOOR HAS NO DOORWAY BESIDE IT: a doorway's cell is cut off from the room it is in (the field links it weakly), so one left free beside a shut leaf
  // holds a level of its own and is shown as a block of another brightness (the maintainer, 11-01: "weird artifacts to the left of the door")
  for (const d of g.doors.list) { const [hx, hz] = d.def.hinge, a = d.def.closedDir, wd = d.def.width, mx = hx + Math.cos(a) * wd / 2, mz = hz + Math.sin(a) * wd / 2;
    for (let dz = -1.2; dz <= 1.2; dz += 0.2) for (let dx = -1.2; dx <= 1.2; dx += 0.2) { const k = cellOf(mx + dx, mz + dz); if (k >= 0 && k < m.length && m[k] !== 255 && (m[k] & 3) === 1) { n++; if (at.length < 4) at.push(`${d.def.name} at ${(mx + dx).toFixed(1)}, ${(mz + dz).toFixed(1)}`); } } }
  check(n === 0, 'with every door shut no free cell beside a door is a doorway\'s cell', `${n} found${at.length ? `: ${at.join('; ')}` : ''}`); }
process.exit(red ? 1 : 0);
