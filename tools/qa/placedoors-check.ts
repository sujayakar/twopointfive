// A DOOR, HOWEVER IT HANGS, KEEPS TWO PLACES TWO (src/scene/placePlan.ts placeMask `gaps` + PLACE.minWide; the look page's One exposure row shows every
// place by its own level, so two places the map takes for one are shown wrong together). The maintainer on his GPU, 11-01, of the pass at midday: "it seems easy
// to get into a state where the exposure is off, like moving between these two rooms": the kitchen was nearly black with Sam in it. Found here, not on a GPU:
// with its yard door wide open the map made the kitchen one place with the open yard (the opening and half the window beside it read as one run five cells
// wide, past a doorway's four: the 0.5 m pier between them was "a post" and one pane a hair under the width a wall piece needs), and a leaf hanging at about
// 65 degrees did the same to the bunk room.
// No GPU: the map is the CPU's (placeMask is pure). What is read: on the pass, with each door swung through its whole travel (every 0.05 rad, the others
// shut) and with every door open at once, no room reaches another room or a cell under the open sky without crossing a doorway cell.
// DOORS OPEN BY DEGREES IN THE MAP (the last two lines): a leaf is never a wall where it hangs, and what the map says of each door (placeMask `steps`).
// It is the only map since 11-04: the one that stamped a leaf as wall cells where it stood is gone with its row.
//   bun run tools/qa/placedoors-check.ts   (exit 0 = all pass; PLACEDOORS_BREAK=1 gives the map no doors and the old width, and must fail;
//                                             DOORSBY_BREAK=1 tells the map every door stands wide open however its leaf hangs, and must fail)
import { readFileSync, readdirSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { apart, WAY } = await import(`${ROOT}/src/scene/wayThrough.ts`);
const { placeMask, doorSteps, doorJambs, doorwayOpen, leafOutlines, PlaceMap, DOOR_STEPS, PLACE, PLACE_W, PLACE_RES } = await import(`${ROOT}/src/scene/placePlan.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
const BRK = process.env.PLACEDOORS_BREAK === '1', BRK_BY = process.env.DOORSBY_BREAK === '1'; let red = false;
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
/** the map as the level stands (DOORSBY_BREAK=1: every door told wide open) */
const byDeg = (): Uint8Array => placeMask(world.statics, world.dynamics, undefined, GAPS, BRK_BY ? GAPS.map(() => DOOR_STEPS) : doorSteps(world.statics, world.dynamics, GAPS));
function reaches(from: [number, number]): string {
  const m = byDeg(), seen = new Uint8Array(m.length), q = [cellOf(from[0], from[1])]; seen[q[0]] = 1; let sky = false;
  while (q.length) { const k = q.pop()!; if (m[k] & 4) sky = true;
    for (const d of [1, -1, PLACE_W, -PLACE_W]) { const t = k + d; if (t < 0 || t >= m.length || seen[t] || m[t] === 255 || (m[t] & 3) === 1) continue; if (Math.abs(d) === 1 && Math.floor(t / PLACE_W) !== Math.floor(k / PLACE_W)) continue; seen[t] = 1; q.push(t); } }
  return [sky ? 'the open sky' : '', ...Object.entries(ROOMS).filter(([, p]) => p !== from && seen[cellOf(p[0], p[1])]).map(([n]) => n)].filter(Boolean).join(', ');
}
const hang = (name: string, a: number) => { const d = g.doors.byName(name); d.latched = a === 0; d.vel = 0; d.angle = a; };
const settle = () => { for (let i = 0; i < 2; i++) W.step(1 / 60); };
{ for (const o of g.doors.list) hang(o.def.name, 0); settle(); const r = Object.entries(ROOMS).map(([n, p]) => [n, reaches(p)]).filter(([, x]) => x);
  check(r.length === 0 && Object.values(ROOMS).every(p => byDeg()[cellOf(p[0], p[1])] === 2), 'with every door shut each of the four rooms is a place of its own, under a roof', r.map(([n, x]) => `${n} reaches ${x}`).join('; ')); }
{ const bad: string[] = []; let looked = 0;
  for (const d0 of g.doors.list) { for (const o of g.doors.list) hang(o.def.name, 0);
    for (let a = d0.minAuth; a <= d0.maxAuth + 1e-6; a += 0.05) { hang(d0.def.name, Math.abs(a) < 0.02 ? 0 : a); settle(); looked++;
      for (const [n, p] of Object.entries(ROOMS)) { const x = reaches(p); if (x) bad.push(`${d0.def.name} at ${a.toFixed(2)}: ${n} reaches ${x}`); } } }
  check(looked > 300 && bad.length === 0, 'no door, at any angle of its travel, makes a room one place with another room or with the open air', `${looked} door positions looked at, ${bad.length} joined${bad.length ? `: ${bad.slice(0, 4).join('; ')}` : ''}`); }
{ const bad: string[] = [];
  for (const s of [1, -1, 0.8, -0.8, 0.55, -0.55, 0]) { for (const o of g.doors.list) hang(o.def.name, s > 0 ? o.maxAuth * s : -o.minAuth * s); settle(); for (const [n, p] of Object.entries(ROOMS)) { const x = reaches(p); if (x) bad.push(`all at ${s}: ${n} reaches ${x}`); } }
  check(bad.length === 0, 'nor do all seven doors standing open at once, either way', bad.slice(0, 4).join('; ')); }
{ for (const o of g.doors.list) hang(o.def.name, 0); settle(); const m = byDeg(); let n = 0; const at: string[] = [];
  // A SHUT DOOR HAS NO DOORWAY BESIDE IT: a doorway's cell is cut off from the room it is in, so one left free beside a shut leaf was a block of another
  // brightness (the maintainer, 11-01: "weird artifacts to the left of the door")
  for (const d of g.doors.list) { const [hx, hz] = d.def.hinge, a = d.def.closedDir, wd = d.def.width, mx = hx + Math.cos(a) * wd / 2, mz = hz + Math.sin(a) * wd / 2;
    for (let dz = -1.2; dz <= 1.2; dz += 0.2) for (let dx = -1.2; dx <= 1.2; dx += 0.2) { const k = cellOf(mx + dx, mz + dz); if (k >= 0 && k < m.length && m[k] !== 255 && (m[k] & 3) === 1) { n++; if (at.length < 4) at.push(`${d.def.name} at ${(mx + dx).toFixed(1)}, ${(mz + dz).toFixed(1)}`); } } }
  check(n === 0, 'with every door shut no free cell beside a door is a doorway\'s cell', `${n} found${at.length ? `: ${at.join('; ')}` : ''}`); }
// ---- DOORS OPEN BY DEGREES: a leaf is never a wall where it hangs, a doorway's cells carry how open it is, and a shut door is a wall from jamb to jamb.
// The maintainer on his GPU, 11-04, of a door opened on a dark room: "some odd brightness swinging": a leaf swinging through its travel was stamped as wall
// cells wherever it stood, so the map's walls moved under the field as it swung.
console.log(`--- by degrees`);
{ // THE WALLS STAND STILL WHILE A DOOR SWINGS, and how open the map says the door is follows the swing
  const wallsOf = (m: Uint8Array) => { let h = 0, n = 0; for (let k = 0; k < m.length; k++) if (m[k] === 255) { n++; h = (Math.imul(h, 31) + k) | 0; } return `${n}:${h}`; };
  const bad: string[] = [], kept = new PlaceMap(); let newSets = 0, doors = 0, late = 0, least = DOOR_STEPS, keptOff = 0, keptDrawn = 0, looked = 0;
  g.doors.list.forEach((d0: any, n: number) => { for (const o of g.doors.list) hang(o.def.name, 0); settle(); doors++;
    const now = new Set<string>(), shutWalls = byDeg(); let first = 9, top = 0;
    for (const sg of [1, -1]) { let last = 0, topDir = 0, firstDir = 9;
      for (let a = 0; a <= (sg > 0 ? d0.maxAuth : -d0.minAuth) + 1e-6; a += 0.05) { hang(d0.def.name, a * sg); settle();
        const m = byDeg(), st = GAPS[n] ? doorSteps(world.statics, world.dynamics, GAPS)[n] : -1;
        if (st < last) bad.push(`${d0.def.name} at ${(a * sg).toFixed(2)}: ${st} of ${DOOR_STEPS} after ${last}`); last = st; topDir = Math.max(topDir, st); if (st > 0) firstDir = Math.min(firstDir, a);
        // THE MAP THE ENGINE KEEPS (PlaceMap.update: its jambs kept with the walls, its steps with the leaves, its key by the steps) is the fresh one
        if (!BRK_BY) { looked++; if (kept.update(world.statics, world.dynamics, GAPS)) keptDrawn++; for (let k = 0; k < m.length; k++) if (kept.mask[k] !== m[k]) { keptOff++; break; } }
        if (st > 0) now.add(wallsOf(m));
        // what the map itself carries: every free cell that was wall with the door shut is this door's, a doorway's cell, and says the same step; no other cell says any
        let own = 0, other = 0; for (let k = 0; k < m.length; k++) { if (m[k] === 255) continue; const s5 = (m[k] >> 3) & 31; if (shutWalls[k] === 255) { own++; if ((m[k] & 3) !== 1 || s5 !== st) bad.push(`${d0.def.name} at ${(a * sg).toFixed(2)}: a cell of its line says ${m[k]}`); } else if (s5) other++; }
        if ((st > 0) !== (own > 0)) bad.push(`${d0.def.name} at ${(a * sg).toFixed(2)}: ${st} of ${DOOR_STEPS} open and ${own} cells of its line free`);
        if (st > 0 && other) bad.push(`${d0.def.name} at ${(a * sg).toFixed(2)}: ${other} cells off its line say how open`); }
      first = Math.max(first === 9 ? 0 : first, firstDir); top = Math.min(top || DOOR_STEPS, topDir); }   // (the later of the two ways to open, the lesser of the two ends)
    newSets += now.size; if (first > 0.35) late++; least = Math.min(least, top); });
  // … and it follows a wall that goes (the first door's jambs taken away: its line ends in the open, and the kept jambs are found again) and comes back
  { for (const o of g.doors.list) hang(o.def.name, 0); hang(g.doors.list[0].def.name, 0.6); settle(); const [hx, hz, fx, fz] = GAPS[0] ?? [0, 0, 0, 0];
    const fewer = world.statics.filter((b: any) => Math.min(Math.hypot(b.c[0] - hx, b.c[2] - hz), Math.hypot(b.c[0] - fx, b.c[2] - fz)) > Math.hypot(b.h[0], b.h[2]) + 0.2);
    for (const st of [fewer, world.statics, fewer, world.statics]) { kept.update(st, world.dynamics, GAPS);
      const fresh = placeMask(st, world.dynamics, undefined, GAPS); looked++; for (let k = 0; k < fresh.length; k++) if (kept.mask[k] !== fresh[k]) { keptOff++; break; } }
    if (fewer.length === world.statics.length) keptOff++; }
  // HOW OPEN, PINNED: the pass's tea room door, a leaf of 0.86 m in a shell wall
  const tea = g.doors.list.findIndex((d: any) => d.def.name === 'tea'), J = doorJambs(world.statics, GAPS), share = (a: number) => { for (const o of g.doors.list) hang(o.def.name, 0); hang('tea', a); settle(); return GAPS[tea] ? doorwayOpen(GAPS[tea], leafOutlines(world.dynamics), J[tea]) : -1; };
  const sh = [0.2, 0.25, 0.5, 1.0, 1.57].map(share), pins = sh[0] === 0 && sh[1] < 0.03 && sh[2] > 0.24 && sh[2] < 0.32 && sh[3] > 0.68 && sh[3] < 0.78 && sh[4] > 0.95;
  check(doors > 0 && newSets === doors && bad.length === 0 && late === 0 && least === DOOR_STEPS && pins && keptOff === 0 && keptDrawn > doors * 10 && keptDrawn < looked * 0.8,
    'by degrees the map\'s walls stand still while a door swings, and its doorway says how open it is',
    `${doors} doors, each swung both ways from shut: the wall cells are ${newSets === doors ? 'one set a door' : `${newSets} sets`} over every open position; a shut door's line is wall and an open one's cells are doorway cells that all carry its step, which never falls as the door opens, is over 0 within 0.35 rad (the leaf's edge has first to leave the wall's own depth) and is ${least} of ${DOOR_STEPS}, a doorway whole, at both ends of every door's travel. The tea room's door is open by ${sh.map(x => x.toFixed(2)).join(', ')} of its doorway at 0.2, 0.25, 0.5, 1.0 and 1.57 rad (pinned: shut while its edge is in the wall's depth, about a quarter at 0.5, three quarters at 1). The map the engine keeps was the fresh one at all ${looked} positions, after a wall went and came back, twice (${keptOff} wrong), drawn ${keptDrawn} times${bad.length ? `. WRONG: ${bad.slice(0, 4).join('; ')}` : ''}`); }
{ // EVERY LEVEL'S DOORS: shut is shut, the walls of a level with its doors shut are the walls it had, a door at either end of its travel is open, and every
  // door's line ends at something a leaf shuts against
  const l0 = console.log, w0 = console.warn; let levels = 0, doors = 0, ends = 0; const bad: string[] = [], bare: string[] = [], low: string[] = [];
  for (const name of readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).map(f => f.replace(/\.json$/, ''))) {
    console.log = console.warn = () => { }; let X: any; try { X = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8'))), seed: 1 }); for (let i = 0; i < 4; i++) X.step(1 / 60); } catch { console.log = l0; console.warn = w0; continue; }
    const gg = X.game, wd = gg.engine.world, list: any[] = gg.doors.list; gg.aiEnabled = false; levels++; doors += list.length;
    const gaps = BRK ? [] : list.map((d: any) => { const [hx, hz] = d.def.hinge, a = d.def.closedDir, w = d.def.width; return [hx, hz, hx + Math.cos(a) * w, hz + Math.sin(a) * w]; });
    const put = (f: (d: any) => number) => { for (const d of list) { const a = f(d); d.latched = a === 0; d.vel = 0; d.angle = a; } X.step(1 / 60); X.step(1 / 60); return BRK_BY ? gaps.map(() => DOOR_STEPS) : doorSteps(wd.statics, wd.dynamics, gaps); };
    // (shut = no door says it is open, and every cell whose middle lies on a door's line, from jamb to jamb, is a wall's)
    const shut = put(() => 0), a = placeMask(wd.statics, wd.dynamics, undefined, gaps, shut); let n = 0;
    for (const [ax, az, bx, bz] of gaps) { const len = Math.hypot(bx - ax, bz - az), ux = (bx - ax) / len, uz = (bz - az) / len; for (let s = 0.1; s < len - 0.05; s += 0.1) { const k = Math.floor((az + uz * s - WORLD.rcMin[2]) * PLACE_RES) * PLACE_W + Math.floor((ax + ux * s - WORLD.rcMin[0]) * PLACE_RES); if (k >= 0 && k < a.length && a[k] !== 255) n++; } }
    if (n || shut.some((s: number) => s !== 0) || shut.length !== list.length) bad.push(`${name}: shut reads ${shut.filter((s: number) => s).length} doors open, ${n} points of a shut door's line in a free cell`);
    const hi = put(d => d.maxAuth), lo = put(d => d.minAuth); list.forEach((d: any, i: number) => { for (const [st, at] of [[hi[i], d.maxAuth], [lo[i], d.minAuth]]) if (Math.abs(at) > 1.2 && !(st >= DOOR_STEPS - 1)) low.push(`${name} ${d.def.name} at ${at.toFixed(2)}: ${st}`); });
    const J = doorJambs(wd.statics, gaps); gaps.forEach((gp: number[], i: number) => { for (let e = 0; e < 4; e += 2) { ends++; if (!J[i].some((o: any) => apart(o, [[gp[e], gp[e + 1]], [gp[e], gp[e + 1]]]) <= WAY.play)) bare.push(`${name} ${list[i].def.name}`); } });
    console.log = l0; console.warn = w0; }
  check(levels >= 50 && doors > 400 && bad.length === 0 && low.length === 0 && bare.length <= ends * 0.01,
    'on every level a shut door is shut and its line a wall from jamb to jamb, a door at either end of its travel is a doorway whole or a step under, and a door\'s line ends at something a leaf shuts against',
    `${levels} levels, ${doors} doors; ${bad.length} wrong shut${bad.length ? `: ${bad.slice(0, 3).join('; ')}` : ''}; ${low.length} low at an end${low.length ? `: ${low.slice(0, 3).join('; ')}` : ''}; ${bare.length} of ${ends} line ends with nothing there${bare.length ? `: ${bare.slice(0, 6).join('; ')}` : ''}`); }
process.exit(red ? 1 : 0);
