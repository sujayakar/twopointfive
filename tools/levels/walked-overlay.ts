// WHAT IS LAID BY HAND ON A LEVEL'S GROUND, OVER WHERE THE LEVEL IS WALKED. About twenty-five generators lay paths by hand (snow trodden and cleared, wheel
// ruts, stains run along a lane), and scene/ways.ts levelWays now works out a level's ways from its own doors, routes and starts, which the look's page
// wears into the floor. Before a hand-laid path is retired its lane has to be there: this draws the two over each other and counts.
//   bun run tools/levels/walked-overlay.ts <level …> [--out <dir>] [--px 24]        (--out=<dir> and --px=<n> are read too; any other -- word is refused)
//     <dir>/<level>-walked.png   the level from above, `px` pixels a metre: the walk grid (shut cells dark), every FLAT THING LYING ON THE GROUND filled
//                                in its group's colour, the ways over them as the lines they were traced along (a patrol red, a team blue, door to door
//                                green, the author's own yellow), every door a white square and every start a white cross
//     a table on the page        a row a group of flat things: how many, how many square metres, and HOW MUCH OF IT LIES ON A LANE (a walked cell or
//                                the cell beside one). A group that lies mostly on lanes is a path laid by hand that the rule now draws; one that does
//                                not is a stain, a mat or a spill, or a path the rule does not reach. And the other way, a group: how much of the
//                                lanes lies under it (a floor sheet is under most of its room's; a path the rule draws is under its own lane)
// WHAT COUNTS AS LAID ON THE GROUND: a box of the level's FILE no thicker than 3 cm with its top under 10 cm and its narrow side under 4 m (a path, a rut,
// a stain, a mat; a strip of floor or ground that narrow is a row too, and reads as one), in any group. No list of names: a level's groups say what they are
// (`snow.paths`, `yard.stains`, `lived.mess`), and the reader judges.
// THE LEVEL AND ITS WAYS ARE tools/qa/walked-check.ts pageWays' and its starts that check's startsOf: the file as the page loads it (lived rooms and
// dressing in, the office with its script), so the lanes and starts drawn are the ones the check asks about. A LOOK TWIN asked by name is drawn with ITS
// OWN flat things (the look's bands and stains are in the twin's file) over its plain file's ways: the two stand on one floor (scene/look.ts
// LOOK_LEVELS). <dir> is dist/walked where none is said (git ignores dist/): nothing git keeps is written.
import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { encodePng } from '../qa/png';
import { pageWays, plainOf, startsOf } from '../qa/walked-check';
import { doorMid } from '../../src/scene/ways';
import { parseLevelData } from '../../src/scene/levelData';
import { WORLD } from '../../src/scene/world';

const ROOT = resolve(import.meta.dir, '../..'), argv = process.argv.slice(2), usage = () => { console.error('usage: bun run tools/levels/walked-overlay.ts <level …> [--out <dir>] [--px 4‥64]'); process.exit(2); };
const said: Record<string, string> = {}, words: string[] = [];
for (let i = 0; i < argv.length; i++) { const a = argv[i]; if (!a.startsWith('--')) { words.push(a); continue; } const eq = a.indexOf('='), key = eq < 0 ? a : a.slice(0, eq), v = eq < 0 ? argv[++i] : a.slice(eq + 1);
  if ((key !== '--out' && key !== '--px') || v === undefined || v === '' || v.startsWith('--')) { console.error(`walked-overlay: ${key === '--out' || key === '--px' ? `${key} wants a value` : `no such word ${a}`}`); usage(); } said[key] = v!; }
const OUT = resolve(said['--out'] ?? `${ROOT}/dist/walked`), PX = Number(said['--px'] ?? 24), names = [...new Set(words.map(n => n.replace(/\.json$/, '')))];
if (!names.length || !(Number.isInteger(PX) && PX >= 4 && PX <= 64)) usage();
for (const n of names) if (!existsSync(`${ROOT}/levels/${n}.json`)) { console.error(`walked-overlay: no level ${n}`); process.exit(2); }
mkdirSync(OUT, { recursive: true });

const KIND: Record<string, [number, number, number]> = { patrol: [235, 70, 60], team: [70, 140, 245], doors: [70, 200, 110], hint: [245, 215, 60] };
/** a group's colour: evenly round the wheel, pale, so the ways' four colours read over it */
const hue = (k: number, n: number): [number, number, number] => { const h = (k / Math.max(1, n)) * 6, c = 90, x = c * (1 - Math.abs(h % 2 - 1)), m = 95; const [r, g, b] = h < 1 ? [c, x, 0] : h < 2 ? [x, c, 0] : h < 3 ? [0, c, x] : h < 4 ? [0, x, c] : h < 5 ? [x, 0, c] : [c, 0, x]; return [r + m, g + m, b + m]; };

for (const name of names) {
  const plain = plainOf(name), { data: plainData, level, nav, ways: A } = pageWays(plain), data = plain === name ? plainData : parseLevelData(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8'));
  const X0 = WORLD.min[0], Z0 = WORLD.min[2], WM = WORLD.max[0] - X0, HM = WORLD.max[2] - Z0, W = Math.round(WM * PX), H = Math.round(HM * PX), img = new Uint8Array(W * H * 4);
  const put = (x: number, y: number, c: readonly number[], a = 1) => { if (x < 0 || y < 0 || x >= W || y >= H) return; const o = (y * W + x) * 4; for (let k = 0; k < 3; k++) img[o + k] = Math.round(img[o + k] * (1 - a) + c[k] * a); img[o + 3] = 255; };
  // the walk grid
  for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) { const i = Math.floor((X0 + (x + 0.5) / PX) / A.cell), j = Math.floor((Z0 + (y + 0.5) / PX) / A.cell), shut = i < 0 || j < 0 || i >= A.w || j >= A.h || nav.blocked[j * A.w + i]; put(x, y, shut ? [26, 27, 30] : [58, 60, 64]); }
  // what lies flat on the ground, by group (the file's own `group` of each box: the loader keeps the boxes in the file's order, and drops none of these)
  const flats = (data.boxes as any[]).filter(b => 2 * b.h[1] <= 0.03 && b.c[1] + b.h[1] <= 0.1 && 2 * Math.min(b.h[0], b.h[2]) < 4);
  const groups = [...new Set(flats.map(b => String(b.group ?? '(no group)')))].sort();
  // a lane: a walked cell, or the cell beside one
  const walked = new Uint8Array(A.w * A.h), lane = new Uint8Array(A.w * A.h);
  for (let k = 0; k < walked.length; k++) walked[k] = A.patrol[k] + A.team[k] + A.hint[k] > 0 ? 1 : 0;
  // (door to door outside is a way too; the `doors` count also holds every door's own threshold, which is no way, so those ways are laid here from their
  // lines, at the rule's own step over free cells: a copy of scene/ways.ts walk's `put`, there until the rule hands the two apart)
  for (const p of A.paths as { kind: string; points: number[][] }[]) if (p.kind === 'doors') for (let s = 0; s + 1 < p.points.length; s++) { const a = p.points[s], b = p.points[s + 1], n = Math.max(1, Math.ceil(Math.hypot(b[0] - a[0], b[2] - a[2]) / 0.2));
    for (let t = 0; t <= n; t++) { const i = Math.floor((a[0] + (b[0] - a[0]) * t / n) / A.cell), j = Math.floor((a[2] + (b[2] - a[2]) * t / n) / A.cell); if (i >= 0 && j >= 0 && i < A.w && j < A.h && !nav.blocked[j * A.w + i]) walked[j * A.w + i] = 1; } }
  for (let j = 0; j < A.h; j++) for (let i = 0; i < A.w; i++) { let on = 0; for (let dj = -1; dj <= 1 && !on; dj++) for (let di = -1; di <= 1; di++) { const u = i + di, v = j + dj; if (u >= 0 && v >= 0 && u < A.w && v < A.h && walked[v * A.w + u]) { on = 1; break; } } lane[j * A.w + i] = on && !nav.blocked[j * A.w + i] ? 1 : 0; }
  const rows = groups.map(g => ({ group: g, n: 0, m2: 0, on: 0, of: 0, boxes: [] as ((x: number, z: number) => boolean)[] }));
  for (const b of flats) { const gi = groups.indexOf(String(b.group ?? '(no group)')), row = rows[gi], col = hue(gi, groups.length), yaw = b.yaw ?? 0, cs = Math.cos(yaw), sn = Math.sin(yaw);
    row.n++; row.m2 += 4 * b.h[0] * b.h[2];
    const inside = (x: number, z: number) => { const dx = x - b.c[0], dz = z - b.c[2]; return Math.abs(cs * dx - sn * dz) <= b.h[0] && Math.abs(sn * dx + cs * dz) <= b.h[2]; };
    row.boxes.push(inside); const r = Math.hypot(b.h[0], b.h[2]);
    for (let y = Math.max(0, Math.floor((b.c[2] - r - Z0) * PX)); y <= Math.min(H - 1, Math.ceil((b.c[2] + r - Z0) * PX)); y++) for (let x = Math.max(0, Math.floor((b.c[0] - r - X0) * PX)); x <= Math.min(W - 1, Math.ceil((b.c[0] + r - X0) * PX)); x++)
      if (inside(X0 + (x + 0.5) / PX, Z0 + (y + 0.5) / PX)) put(x, y, col, 0.55);
    // counted in the box's OWN frame, on a step a quarter of the walk grid's or finer and never fewer than two across either side: a rut 8 cm wide and
    // a path turned a few degrees are counted as what they are
    const nu = Math.max(2, Math.ceil(2 * b.h[0] / (A.cell / 4))), nv = Math.max(2, Math.ceil(2 * b.h[2] / (A.cell / 4))), da = 4 * b.h[0] * b.h[2] / (nu * nv);   // (each point stands for its share of the box's area: a group's share is by area, summed over its boxes)
    for (let u = 0; u < nu; u++) for (let v = 0; v < nv; v++) { const lx = ((u + 0.5) / nu * 2 - 1) * b.h[0], lz = ((v + 0.5) / nv * 2 - 1) * b.h[2], x = b.c[0] + cs * lx + sn * lz, z = b.c[2] - sn * lx + cs * lz;
      const i = Math.floor(x / A.cell), j = Math.floor(z / A.cell); if (i < 0 || j < 0 || i >= A.w || j >= A.h) continue; row.of += da; if (lane[j * A.w + i]) row.on += da; } }
  // the ways, as lines
  const line = (a: number[], b: number[], c: readonly number[]) => { const n = Math.max(1, Math.ceil(Math.hypot(b[0] - a[0], b[2] - a[2]) * PX)); for (let t = 0; t <= n; t++) { const x = Math.round((a[0] + (b[0] - a[0]) * t / n - X0) * PX), y = Math.round((a[2] + (b[2] - a[2]) * t / n - Z0) * PX); for (let dy = -1; dy <= 1; dy++) for (let dx = -1; dx <= 1; dx++) put(x + dx, y + dy, c, 0.8); } };
  for (const kind of ['doors', 'team', 'patrol', 'hint']) for (const p of A.paths as { kind: string; points: number[][] }[]) if (p.kind === kind) for (let s = 0; s + 1 < p.points.length; s++) line(p.points[s], p.points[s + 1], KIND[kind]);
  for (const d of level.doors) { const c = doorMid(d), x = Math.round((c[0] - X0) * PX), y = Math.round((c[2] - Z0) * PX); for (let dy = -3; dy <= 3; dy++) for (let dx = -3; dx <= 3; dx++) put(x + dx, y + dy, Math.max(Math.abs(dx), Math.abs(dy)) === 3 ? [20, 20, 20] : [250, 250, 250]); }
  for (const st of startsOf(plain, level)) { const x = Math.round((st.v[0] - X0) * PX), y = Math.round((st.v[2] - Z0) * PX); for (let k = -6; k <= 6; k++) for (const w of [-1, 0, 1]) { put(x + k, y + w, [250, 250, 250]); put(x + w, y + k, [250, 250, 250]); } }
  writeFileSync(`${OUT}/${name}-walked.png`, encodePng(img, W, H));
  // the table
  // …and how much of the lanes' own floor lies under a group: the same lanes the other way, by area (sixteen points a cell)
  const cells: number[] = []; for (let k = 0; k < lane.length; k++) if (lane[k]) cells.push(k);   // (a lane, as above: a walked cell or the free cell beside one)
  const pct = (a: number, b: number) => b ? `${Math.round(100 * a / b)}%` : '-', N = 4;
  const laneShare = (boxes: ((x: number, z: number) => boolean)[]) => { let n = 0; for (const k of cells) { const i = k % A.w, j = (k - i) / A.w; for (let u = 0; u < N; u++) for (let v = 0; v < N; v++) { const x = (i + (u + 0.5) / N) * A.cell, z = (j + (v + 0.5) / N) * A.cell; if (boxes.some(f => f(x, z))) n++; } } return pct(n, cells.length * N * N); };
  console.log(`\n${name}: ${A.paths.length} ways (${['patrol', 'team', 'doors', 'hint'].map(k => `${k} ${A.paths.filter((p: { kind: string }) => p.kind === k).length}`).join(', ')}) · ${cells.length} cells of lane (${A.cell} m; walked, or beside one that is) · ${OUT}/${name}-walked.png${plain === name ? '' : ` (its own flat things over the ways of ${plain})`}`);
  console.log('  what lies flat on the ground, a group                boxes     m²   of it on a lane   of the lanes under it');
  for (const r of rows.sort((a, b) => b.m2 - a.m2)) console.log(`  ${r.group.padEnd(50)} ${String(r.n).padStart(5)} ${r.m2.toFixed(1).padStart(6)}   ${pct(r.on, r.of).padStart(15)}   ${laneShare(r.boxes).padStart(21)}`);
  if (!rows.length) console.log('  (nothing)');
}
