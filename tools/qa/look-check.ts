#!/usr/bin/env bun
// THE DEFAULT LOOK'S CHECK (src/scene/look.ts) — `bun run tools/qa/look-check.ts`: a level's file in the look is the plain file in other clothes, and nothing else.
// For every pair in LOOK_LEVELS:
//   1. both files are what their generators give now (tools/levels/export.ts --check);
//   2. every box of the plain file is in the look's file at the same place in the list, with the same centre, size, yaw, flags and group — only its paint
//      (albedo, rough, metal) may differ; every box the look adds comes after them, is in the group 'look', is a NoShadow decal (no ray and no collision
//      query ever returns one) and starts at the floor (so game/wallbreak.ts, which lets drawn-only things that hang off a broken thing fall, never asks it);
//   3. every box DOES the same thing in both (src/scene/materials.ts, Box.does: its own word, else its material row's, else the default row), whatever
//      it is painted — all a round, a charge, a blast or the destruction index reads of a material;
//   4. props: same kind, place, yaw, mass and parts — only a part's paint may differ;
//   5. everything else in the file (lights, switchables, doors, routes, devices, marks, plans, scenarios, surfaces, sky …) is the plain file's, key for key.
//   6. the game stood up on each: the destruction's three indexes (destruction.ts paper / screens / clutter, wallbreak.ts walls + cover, furnblast.ts groups — all
//      found by SHAPE and by what a box does) list the same boxes in both; and the lot's slab says the same word, so it is the same wet;
// And the palette itself: no two rows of it are one colour, and no level outside the look carries one.
// Exit 0 = all pass, 1 = a check failed. Also `bun run tools/qa/probes.ts look` (in all-quick): lookCheck() below.
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
export async function lookCheck(): Promise<boolean> {
await import('./headless.ts');   // (the game modules are one import cycle: enter it the way every probe does)
const { LOOK_LEVELS } = await import(`${ROOT}/src/scene/look.ts`);
const { GWOT_PALETTE, SNOW_PALETTE, lookPaintOf } = await import(`${ROOT}/src/scene/gwotPalette.ts`);
const { DOES_DEFAULT, doesOfWord } = await import(`${ROOT}/src/scene/materials.ts`);
/** what a box or a part of a file does: its own word, else its material row's, else the default row (as the loader writes it onto the box) */
const doesIn = (o: any, mats: any): string => o.does ?? (o.material !== undefined ? mats[o.material]?.does : undefined) ?? DOES_DEFAULT;
const { standUp } = await import('./headless.ts');
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { FLAT_STEP, flatClashes, isFlat } = await import(`${ROOT}/tools/levels/flatLayers.ts`);
/** the three destruction indexes of a file, as text */
async function indexesOf(file: string): Promise<string> {
  const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { };
  try { const was = DESTRUCTION.on; DESTRUCTION.on = true;
    try { const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8'))) }); const d = W.game.destr, boxes = W.game.col.boxes;
      const items = [...d.items].map(([i, x]: [number, any]) => `${i}:${x.kind}`).sort().join(' ');
      const walls = d.walls.list.map((B: any) => `${boxes.indexOf(B.box)}:${B.kind}`).sort().join(' ');
      const furn = JSON.stringify(d.furn.inventory().list);
      return `items ${items}\nwalls ${walls}\nfurn ${furn}`; } finally { DESTRUCTION.on = was; } }
  finally { console.log = log; console.error = err; console.warn = warn; }
}

let ok = true; const check = (pass: boolean, what: string) => { console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${what}`); if (!pass) ok = false; };
const J = (f: string) => JSON.parse(readFileSync(resolve(ROOT, 'levels', f), 'utf8'));
const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);
const albedoOf = (b: any, mats: any): number[] => b.albedo ?? mats[b.material]?.albedo;
/** the generator of a level file, by name: tools/levels/<stem>.ts, or the stem without its dashes (block-squads.json is tools/levels/blocksquads.ts) */
const genOf = (file: string): string | undefined => { const stem = file.replace(/\.json$/, ''); return [stem, stem.replace(/-/g, '')].find(n => existsSync(resolve(ROOT, 'tools/levels', `${n}.ts`))); };
const PAINT = new Set(['albedo', 'material', 'rough', 'metal', 'stuff', 'does']);   // (what a box says it is made of is part of its paint: the skin drops it with the paint it repaints. `does` is NOT, and is held equal below: but a file writes it on the box or leaves it to the box's row, and the two files name different rows)

// the palette
{ const seen = new Map<string, string>(); let clash = 0;
  for (const [m, a] of [...Object.entries(GWOT_PALETTE), ...Object.entries(SNOW_PALETTE)] as [string, number[]][]) { const k = a.join(','), o = seen.get(k); if (o && o !== m) clash++; seen.set(k, m); }
  check(clash === 0, `the palette: ${seen.size} paints, no two of them one colour (${clash} clashes): a level file names a box's paint by its numbers`);
  // (a level BUILT in the kit carries the palette on purpose — its generator imports the kit; any other level carrying one of its colours does so by accident)
  const lookFiles = new Set<string>(Object.values(LOOK_LEVELS as Record<string, string>)); let strays: string[] = [], built: string[] = [];
  // (built in the kit = its generator imports the kit, or imports a generator beside it that does: the training yards share tools/levels/train-ground.ts)
  const KIT = /gwotKit|gwotSkin|gwotPalette/;
  const inKit = (f: string): boolean => {
    const name = genOf(f), gen = name && resolve(ROOT, 'tools/levels', `${name}.ts`); if (!gen) return false;
    const src = readFileSync(gen, 'utf8'); if (KIT.test(src)) return true;
    return [...src.matchAll(/from '\.\/([\w-]+)'/g)].some(m => { const dep = resolve(ROOT, 'tools/levels', `${m[1]}.ts`); return existsSync(dep) && KIT.test(readFileSync(dep, 'utf8')); });
  };
  for (const f of readdirSync(resolve(ROOT, 'levels'))) { if (!f.endsWith('.json') || lookFiles.has(f)) continue; const d = J(f); let n = 0;
    for (const m of Object.values(d.materials ?? {}) as any[]) if (lookPaintOf(m.albedo) !== undefined) n++; for (const b of d.boxes ?? []) if (b.albedo && lookPaintOf(b.albedo) !== undefined) n++;
    if (n) (inKit(f) ? built : strays).push(`${f} ×${n}`); }
  check(strays.length === 0, `no level outside the look carries one of its colours${strays.length ? ` — ${strays.join(', ')}` : ''} (the picture asks a level whether it wears the look by its paints: scene/lookTidy.ts; built in the kit, on purpose: ${built.map(x => x.split(' ')[0]).join(', ') || 'none'})`);
}

for (const [plain, look] of Object.entries(LOOK_LEVELS as Record<string, string>)) {
  console.log(`\n${plain} ↔ ${look}`);
  { const gen = genOf(plain) ?? plain.replace(/\.json$/, '');
    const p = Bun.spawnSync(['bun', 'run', resolve(ROOT, 'tools/levels/export.ts'), gen, '--check'], { cwd: ROOT });   // (the exporter checks a level's file in the look with it)
    check(p.exitCode === 0 && p.stdout.toString().includes(look), `levels/${plain} and levels/${look} are what their generators give now (export ${gen} --check: exit ${p.exitCode})`); }
  const A = J(plain), B = J(look); const nA = A.boxes.length;
  let moved = 0, repainted = 0, reclassed: string[] = [];
  for (let i = 0; i < nA; i++) {
    const a = A.boxes[i], b = B.boxes[i]; if (!b) { moved++; continue; }
    const keys = new Set([...Object.keys(a), ...Object.keys(b)]); let diff = false, paint = false;
    // (a shape's TRIM wears paint as a box does — scene/shapes.ts ShapePiece — and a file names that paint as it names a box's: a row where it has the
    // row, numbers where it has not. So two shapes that differ only in what their pieces wear differ in paint, and in nothing else)
    const bare = (sh: any) => sh?.with ? { ...sh, with: sh.with.map((w: any) => Object.fromEntries(Object.entries(w).filter(([k]) => !PAINT.has(k)))) } : sh;
    for (const k of keys) { if (same(a[k], b[k])) continue; if (PAINT.has(k) || (k === 'shape' && same(bare(a[k]), bare(b[k])))) paint = true; else diff = true; }
    if (diff) { moved++; continue; }
    const da = doesIn(a, A.materials), db = process.env.LOOK_BREAK && i === 0 ? 'plant' : doesIn(b, B.materials);   // (asked of EVERY box, repainted or not)
    if (da !== db) reclassed.push(`${a.id} [${a.group}] ${da} → ${db}`);
    if (!paint) continue;
    const pa = albedoOf(a, A.materials), pb = albedoOf(b, B.materials); if (same(pa, pb)) continue; repainted++;
  }
  check(moved === 0, `every one of the plain file's ${nA} boxes is in the look's file at the same index with the same centre, size, yaw, flags, group and markings (${moved} differ in more than paint)`);
  check(reclassed.length === 0, `${repainted} boxes wear another paint, and every box of the ${nA} does the same thing in both files to a round, a charge and a blast (the same word of the table of materials)${reclassed.length ? ` — not so: ${reclassed.slice(0, 4).join(' · ')}` : ''}`);
  const extra = B.boxes.slice(nA), badExtra = extra.filter((b: any) => b.group !== 'look' || !same(b.flags, ['NoShadow']));
  check(badExtra.length === 0, `the look adds ${extra.length} boxes after them, every one a NoShadow decal in the group 'look'${badExtra.length ? ` — not so: ${badExtra.slice(0, 3).map((b: any) => b.id).join(' ')}` : ''}`);
  { const off = extra.filter((b: any) => b.c[1] - b.h[1] > 0.06);   // (game/wallbreak.ts judges a drawn-only static that is OFF the floor when what it touches breaks — it falls, as debris with a sound; one that starts at the floor is never asked)
    check(off.length === 0, `every one of them starts at the floor, so the destruction never judges one (${off.length} hang off it${off.length ? `: ${off.slice(0, 3).map((b: any) => b.id).join(' ')}` : ''})`); }
  { // (tools/levels/flatLayers.ts: 20 µm between two decals that cross is nothing to the depth buffer at the camera's distance — they fought for every pixel on a GPU)
    const paint = (b: any) => albedoOf(b, B.materials) ?? [0, 0, 0], thin = (b: any) => !(b.flags ?? []).some((f: string) => f === 'NoPrimary' || f === 'Glass') && b.c[1] + b.h[1] <= 0.05 && b.c[1] + b.h[1] > 0.0105;
    const fl = extra.filter(isFlat), cl = process.env.LOOK_BREAK ? flatClashes(fl, paint, 1) : flatClashes(fl, paint, undefined, B.boxes.slice(0, nA).filter(thin));
    check(cl.length === 0, `of its ${fl.length} flat decals no two of different paint that overlap have their top faces under ${(FLAT_STEP * 1000).toFixed(1)} mm apart, nor one and a thin flat of the level: the picture can order them (${cl.length} cannot be${cl.length ? `: ${cl.slice(0, 3).map(([p, q]: any[]) => `${p.id} × ${q.id}`).join(' ')}` : ''})`); }
  const bare = (p: any) => ({ ...p, parts: p.parts.map((x: any) => Object.fromEntries(Object.entries(x).filter(([k]) => !PAINT.has(k)))) });
  const propsSame = A.props.length === B.props.length && A.props.every((p: any, i: number) => same(bare(p), bare(B.props[i])));
  let partsOff = 0; if (propsSame) A.props.forEach((p: any, i: number) => p.parts.forEach((x: any, j: number) => { if (doesIn(x, A.materials) !== doesIn(B.props[i].parts[j], B.materials)) partsOff++; }));
  check(propsSame && partsOff === 0, `the ${A.props.length} props are the same bodies (kind, place, yaw, mass, parts) — only a part's paint may differ, and each part does what it did (${partsOff} do not)`);
  const rest = [...new Set([...Object.keys(A), ...Object.keys(B)])].filter(k => !['boxes', 'props', 'materials', 'name', 'meta'].includes(k)); const off = rest.filter(k => !same(A[k], B[k]));
  { const ia = await indexesOf(plain), ib = await indexesOf(look); const la = ia.split('\n'), lb = ib.split('\n'); const offI = la.map((l, k) => l === lb[k] ? '' : l.split(' ')[0]).filter(Boolean);
    check(ia === ib, `the destruction's indexes list the same boxes in both (${la.map(l => `${l.split(' ')[0]} ${l.split(/[ ,]/).length - 1}`).join(' · ')})${offI.length ? ` — differ: ${offI.join(' ')}` : ''}`); }
  { // (the ONE slab the storm darkens in place — weather.ts setWet finds it by this shape; every other slab is wetted for the renderer only, Box.wetK)
    const i = A.boxes.findIndex((b: any) => b.c[1] < 0 && b.c[1] + b.h[1] > -0.01 && b.h[0] > 18 && b.h[2] > 12);
    if (i < 0) check(true, 'no lot slab for the storm to darken in place');
    else { const wa = doesIn(A.boxes[i], A.materials), wb = doesIn(B.boxes[i], B.materials), wet = (w: string) => doesOfWord(doesOfWord(w).wet ?? w).surf;
      check(wa === wb, `the lot's slab (${A.boxes[i].id}) says the same word in both, so the storm wets it the same (${wa}: ${doesOfWord(wa).surf} to a round dry, ${wet(wa)} soaked · ${wb})`); } }
  { let metalOff = 0; for (let i = 0; i < nA; i++) { const a = A.boxes[i], b = B.boxes[i]; if (a.c[1] - a.h[1] <= 0.1 && a.c[1] + a.h[1] >= 2.2 && !same(a.metal ?? A.materials[a.material]?.metal ?? false, b.metal ?? B.materials[b.material]?.metal ?? false)) metalOff++; }
    check(metalOff === 0, `no full-height box changed its metal flag (${metalOff} did)`); }
  check(off.length === 0, `everything else is the plain file's, key for key (${rest.join(' ')})${off.length ? ` — differs: ${off.join(' ')}` : ''}`);
}
return ok;
}
if (import.meta.main) { const ok = await lookCheck(); console.log(ok ? '\n=== look-check: ALL PASS ===' : '\n=== look-check: FAILED ==='); process.exit(ok ? 0 : 1); }
