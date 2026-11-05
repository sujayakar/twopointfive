// EVERY LEVEL ON THE MISSION LIST, COUNTED AS THE LOOK PAGE DRAWS IT (the levels goal, 11-03: "we'll want to dress up all the levels and check their budget").
// One command, no GPU, about a second a level: for each level a drawn row of the list plays on (scene/scenarioList.ts, rows without `offList`), stood up on the
// SHIPPED stand in the default look, with the look page's shapes laid over it as main.ts lays them (scene/geoTry.ts and the three trials) and the mission started
// at the hour its row opens at, it says
//   WHAT THE LEVEL IS MADE OF   boxes, cut boxes and their planes, round things, broken wall tops, glass, tilted boxes, marks, props, doors
//   WHAT LIGHTS IT              lamps by kind, how many burn at that hour, lit panels, openings that let daylight in (render/openings.ts)
//   WHAT THE LOOK MAKES OF IT   how much of the picture is each material and how much is NONE, paint by paint: the to-do list of a level that is being dressed.
//                               A box is made of what it SAYS (Box.stuff, scene/stuff.ts): a material of the look, or a word the look has no map for ('plain',
//                               'waits', 'glass', 'rubber'), or nothing. No colour is asked, so nothing is a material by chance.
// "How much of the picture" is SAMPLED, not summed: one ray a 12.5 cm cell of the level's ground, along the play camera's own line of sight, takes the first
// DRAWN box it meets (a roof is not drawn: the camera sees the floor under it), and that box's word is read as the renderer reads it (render/gbuffer.ts
// lookWordOf: render/lookTex.ts lookStuffOf of Box.stuff). So a big slab under a house counts for the floor that shows and no more.
//
//   bun run tools/qa/level-audit.ts                     every level, a table; a file per level under dist/level-audit/ (not in git)
//   bun run tools/qa/level-audit.ts pass block          these levels only, with each one's paints listed
//   bun run tools/qa/level-audit.ts --paints            … every level, with its paints
//   bun run tools/qa/level-audit.ts --out <dir>         where the files go (<dir>/<level>.json, <dir>/audit.json, <dir>/audit.md)
//   bun run tools/qa/level-audit.ts --plain pass        the level as its FILE has it: none of the look page's shapes laid (what ?geo=0&dress=0&front=0&lorry=0 loads)
//   bun run tools/qa/level-audit.ts --check [level …]   THE BUDGET CHECK: exit 1 when a level is over one of the engine's own caps or one of the bars below
//   LEVELAUDIT_BREAK=1 bun run tools/qa/level-audit.ts --check pass     every bar at a tenth: must FAIL
//   bun run tools/qa/level-audit.ts --counts block      … and the level's `counts` block for its row.json: its baseline row and the row as it stands now
//   bun run tools/qa/level-audit.ts --rows              the list as JSON: each level, the missions on it, the hour and the address its first row opens at
//   bun run tools/qa/level-audit.ts --write-base        writes the baseline (fixtures/level-audit-base.json): the levels BEFORE they are dressed. Not rewritten
//                                                       for a dressed level: it is the "before" of the table page and the bars' "heaviest level today"
//                                                       (its `none` is by the colour rule of 2026-11-03, before a box said what it is made of)
// It sets its own stand (TPS_PROFILE=shipped, TPS_LOOK=1) and raises no switch: nothing here is on a frozen digest. A level is loaded once a process (the
// scenarios' team-start knobs are module state), so the one command runs a child a level, four at a time.
//
// THE BARS (`--check`, AUDIT_BARS). The engine's caps are facts. The rest are PROPOSED, by the task that wrote this, and nobody has ruled on them: the budget
// itself is a frame time on a GPU, which no count answers (tools/qa/level-cost-page.cjs and the page's own pricing run are for that). RED: a count that loads a
// frame is a quarter over the heaviest level the list had before dressing; over a tenth of the picture has no material. SAID,
// NOT RED: a count a quarter over the pass's, the one level the look was timed on (GPU 12.2 to 15.3 ms, 58 to 65 fps on an M4 Max, 11-02): that level wants a GPU.
// So `--check <level>` green is the countable half of "dressed"; `--check` on the whole list is red until every level is, and its red lines are the to-do list.
import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'node:fs';
import { spawn } from 'node:child_process';
import { resolve, dirname } from 'node:path';

const HERE = dirname(new URL(import.meta.url).pathname), ROOT_DIR = resolve(HERE, '../..');
const argv = process.argv.slice(2);
const flag = (w: string) => argv.includes(w), valueOf = (w: string) => { const i = argv.indexOf(w); return i >= 0 ? argv[i + 1] : undefined; };
const OUT = resolve(valueOf("--out") ?? `${ROOT_DIR}/dist/level-audit`), PLAIN = flag('--plain'), CHECK = flag('--check'), BRK = process.env.LEVELAUDIT_BREAK === '1';
const named = argv.filter((a, i) => !a.startsWith('--') && argv[i - 1] !== '--out' && argv[i - 1] !== '--one' && argv[i - 1] !== '--base');

import { AUDIT_BARS, countPiece } from './level-count.ts';   // (the bars, and the one count a level and a single piece of the kit share)
const BASE = resolve(valueOf('--base') ?? `${HERE}/fixtures/level-audit-base.json`);

if (flag('--rows')) { const { scenarioQuery } = await import(`${ROOT_DIR}/src/scene/scenarios.ts`); console.log(JSON.stringify((await listed()).map(r => ({ level: r.level, file: r.file, missions: r.rows.map((e: any) => e.id), hour: r.rows[0].hour ?? 'night', address: scenarioQuery(r.rows[0]) })))); process.exit(0); }   // (the page tools read the list from here: level-cost-page.cjs)
if (valueOf('--one') !== undefined) await one(valueOf('--one')!); else await all();

// ---------------------------------------------------------------------------------------------------------------------------------------------------------------
/** the rows the list draws, a level each: the level's stem → its rows (the office has two) */
async function listed(): Promise<{ level: string; file: string | null; rows: any[] }[]> {
  const { listedLevels } = await import(`${ROOT_DIR}/src/scene/scenarioList.ts`);   // (the one answer to which levels the list draws)
  return (listedLevels() as { level: string; rows: any[] }[]).map(({ level, rows }) => ({ level, file: rows[0].level ?? null, rows }));
}

// ---------------------------------------------------------------------------------------------------------------------------------------------------------------
/** ONE LEVEL, in a process of its own: its counts as one line of JSON on stdout (the last line: the game logs above it) */
async function one(level: string) {
  process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_LOOK ??= '1';   // (before the harness is imported: it reads the stand once)
  const quiet = <T>(fn: () => T): T => { const l = console.log, w = console.warn, i = console.info; console.log = console.warn = console.info = () => { }; try { return fn(); } finally { console.log = l; console.warn = w; console.info = i; } };
  const { ROOT, standUp } = await import('./headless.ts');
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`), { buildLevel } = await import(`${ROOT}/src/scene/level.ts`);
  const { lookLevelFile } = await import(`${ROOT}/src/scene/look.ts`), { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
  const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`), { WORLD } = await import(`${ROOT}/src/scene/world.ts`), { cutConfig } = await import(`${ROOT}/src/scene/cut.ts`);
  const { geoTryLevel } = await import(`${ROOT}/src/scene/geoTry.ts`), { dressFront } = await import(`${ROOT}/src/scene/frontTry.ts`), { dressLorry, kitLorry } = await import(`${ROOT}/src/scene/lorryTry.ts`);
  const { dressPass } = await import(`${ROOT}/src/scene/passTry.ts`), { seatPosts } = await import(`${ROOT}/src/scene/lookMend.ts`);
  const { skyPlan } = await import(`${ROOT}/src/scene/placePlan.ts`), { planOpenings, openAreas, LOOK_OPENINGS } = await import(`${ROOT}/src/scene/openings.ts`);   // (loaded by a built path, as every check here loads the game, so the typecheck does not see a module that moves: RUN the audit after a merge)
  const { LOOK_NAMES, lookStuffOf } = await import(`${ROOT}/src/render/lookTex.ts`), { snowPaintOf } = await import(`${ROOT}/src/render/snowField.ts`);
  const { GWOT_PALETTE } = await import(`${ROOT}/src/scene/gwotPalette.ts`), { MAX_LIGHTS } = await import(`${ROOT}/src/render/lights.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`), { applyHour, hourOf } = await import(`${ROOT}/src/game/hours.ts`);
  const rows = (await listed()).find(r => r.level === level);
  if (!rows) { console.log(JSON.stringify({ level, error: 'no drawn row of the mission list plays on this level' })); process.exit(2); }
  const row = rows.rows[0], hour: string = row.hour ?? 'night';

  // ---- the level, as the page loads it: the look's file where it has one, then the look page's shapes over it
  const file = rows.file ? lookLevelFile(rows.file) : null, text = file ? readFileSync(`${ROOT}/levels/${file}`, 'utf8') : null;
  const src: any = text ? JSON.parse(text) : JSON.parse(readFileSync(`${ROOT}/levels/office.json`, 'utf8'));
  const lv: any = quiet(() => text ? loadLevel(parseLevelData(text)) : buildLevel());
  const fileBoxes = lv.boxes.length;
  const shapes = { rounds: 0, walls: 0, front: 0, lorry: 0, lorryKit: 0, pass: 0, seated: 0 };
  if (!PLAIN) quiet(() => { cutConfig.on = true; cutConfig.swap = false;
    const gt = geoTryLevel(lv, skyPlan(lv.boxes)); shapes.rounds = gt.rounds; shapes.walls = gt.walls;
    shapes.front = dressFront(lv);
    if (/^pass\b/.test(rows.file ?? '')) { shapes.lorry = dressLorry(lv); if (shapes.lorry) shapes.lorryKit = kitLorry(lv); shapes.pass = dressPass(lv); }
    shapes.seated = seatPosts(lv); });

  // ---- the game on it, the mission started, at the hour its row opens at
  const W: any = await quietAsync(() => standUp({ level: lv, seed: 1 }));
  const g = W.game, started = quiet(() => startFromQuery(g, scenarioQuery(row)));
  quiet(() => { applyHour(g, hour as any); for (let i = 0; i < 8; i++) W.step(1 / 60); });
  const world = g.engine.world, statics: any[] = world.statics, dynamics: any[] = world.dynamics ?? [], boxes = [...statics, ...dynamics];
  const drawn = (b: any) => !(b.flags & BoxFlag.NoPrimary), lit = (b: any) => b.emissive[0] + b.emissive[1] + b.emissive[2] > 0;

  // ---- what each paint is called in the level's own file (a box's `material`, a prop's part), by its colour
  const WRITTEN = '(written as a colour)';
  const keyOf = (a: ArrayLike<number>) => `${Math.round(a[0] * 4096)},${Math.round(a[1] * 4096)},${Math.round(a[2] * 4096)}`;
  const namesOf = new Map<string, Set<string>>(), groupsOf = new Map<string, Map<string, number>>(), mats: Record<string, { albedo?: number[] }> = src.materials ?? {};
  const say = (a: number[] | undefined, n: string) => { if (!a) return; const k = keyOf(a); if (!namesOf.has(k)) namesOf.set(k, new Set()); namesOf.get(k)!.add(n); };
  for (const b of src.boxes ?? []) { const a = b.albedo ?? mats[b.material]?.albedo; say(a, b.material ?? WRITTEN); if (a) { const k = keyOf(a), gm = groupsOf.get(k) ?? new Map<string, number>(); gm.set(b.group ?? '', (gm.get(b.group ?? '') ?? 0) + 1); groupsOf.set(k, gm); } }
  for (const p of src.props ?? []) for (const part of p.parts ?? []) say(part.albedo ?? mats[part.material]?.albedo, part.material ?? WRITTEN);
  for (const n of Object.keys(GWOT_PALETTE)) say((GWOT_PALETTE as any)[n], n);   // (a piece the page lays by hand is painted from the palette)
  const paintOf = (b: any): ArrayLike<number> => (b.aniso === true && g.weather?.dryPaint?.(b)) || b.albedo;   // (rain darkens a slab in place: its paint's NAME is looked up by its dry colour. What it is made of is its word, which no rain changes)
  const stuffName = (c: number) => c > 0 ? LOOK_NAMES[c - 1] as string : 'none';

  // ---- the picture, sampled: one ray a cell of ground along the play camera's line of sight
  type Paint = { key: string; albedo: number[]; names: string[]; /** the file's groups its boxes are in, the most boxes first */ groups: string[]; /** what the look draws it as: a material's name, 'snow', or the word it says that the look has no map for ('plain', 'waits', 'glass', 'rubber'), or 'none' where it says nothing */ stuff: string; verdict: string; cells: number; boxes: number; tilted: number };
  const paints = new Map<string, Paint>(), kinds: Record<string, number> = {}; let cells = 0, tiltedCells = 0;
  /** a row a paint AND a word: two boxes of one colour that say two things are two lines of the to-do list */
  const paintRow = (b: any): Paint => { const a = paintOf(b), colour = keyOf(a), word: string | undefined = b.stuff, snow = !!snowPaintOf(b), key = `${snow ? 'snow' : word ?? ''}|${colour}`; let p = paints.get(key);
    if (!p) { const cls = lookStuffOf(word), stuff = snow ? 'snow' : cls > 0 ? stuffName(cls) : word ?? 'none';
      // the verdict: a material of the look (or snow, which the snow field draws) is done; a word the look has no map for is SAID and drawn flat; no word is the to-do
      const verdict = snow ? 'snow' : cls > 0 ? 'material' : word !== undefined ? `says '${word}': drawn flat` : 'none';
      p = { key, albedo: [a[0], a[1], a[2]].map(v => +v.toFixed(4)), names: [...(namesOf.get(colour) ?? [])], groups: [...(groupsOf.get(colour) ?? [])].sort((x, y) => y[1] - x[1]).slice(0, 6).map(([gn, n]) => `${gn || '(no group)'} ${n}`), stuff, verdict, cells: 0, boxes: 0, tilted: 0 }; paints.set(key, p); }
    return p; };
  for (const b of boxes) if (drawn(b)) { const p = paintRow(b); p.boxes++; if (b.rot) p.tilted++; }
  { const cam = W.cam, f = cam.forward ?? null;
    // the line of sight: from the camera's eye to what it looks at (the follow camera on the player's spawn); straight down where a camera has neither
    let d: number[] = [0, -1, 0];
    const eye = cam.eye ?? cam.pos ?? cam.position, at = cam.target;
    if (f && Math.abs(f[1]) > 0.2) d = [f[0], f[1], f[2]]; else if (eye && at) { const v = [at[0] - eye[0], at[1] - eye[1], at[2] - eye[2]], n = Math.hypot(v[0], v[1], v[2]); if (n > 1e-6 && v[1] < 0) d = v.map(x => x / n); }
    const step = 0.125, x0 = WORLD.rcMin[0], z0 = WORLD.rcMin[2], x1 = x0 + WORLD.rcSize[0], z1 = z0 + WORLD.rcSize[2], back = 40;
    for (let z = z0 + step / 2; z < z1; z += step) for (let x = x0 + step / 2; x < x1; x += step) {
      const ro: [number, number, number] = [x - d[0] * back, -d[1] * back, z - d[2] * back];
      const hit = world.raycast(ro, d as [number, number, number], 200, BoxFlag.NoPrimary, 0, true); if (!hit) continue;
      const b = hit.index < statics.length ? statics[hit.index] : dynamics[hit.index - statics.length]; if (!b) continue;
      cells++; const kind = (b.flags & BoxFlag.Glass) ? 'glass' : lit(b) ? 'lit' : null;
      if (kind) { kinds[kind] = (kinds[kind] ?? 0) + 1; continue; }
      const p = paintRow(b); p.cells++; if (b.rot && p.verdict === 'material') tiltedCells++;
    }
    (shapes as any).sight = d.map(v => +v.toFixed(3));
  }
  const share = (n: number) => cells ? +(n / cells).toFixed(4) : 0;
  const byStuff: Record<string, number> = {}; let bare = 0, said = 0;   // bare = what the look draws flat · said = the part of it that says a word of its own
  for (const p of paints.values()) { byStuff[p.stuff] = (byStuff[p.stuff] ?? 0) + p.cells; if (p.verdict !== 'material' && p.verdict !== 'snow') { bare += p.cells; if (p.verdict !== 'none') said += p.cells; } }
  for (const k in kinds) byStuff[k] = kinds[k];

  // ---- the lamps, as they stand at that hour; the openings daylight comes in at
  const lamps: any[] = g.engine.lights.lights ?? [], KIND = ['point', 'spot', 'directional', 'area'];
  const on = (l: any) => l.enabled && l.intensity > 0, lampKinds: Record<string, number> = {}, onKinds: Record<string, number> = {};
  for (const l of lamps) { const k = KIND[l.kind] ?? String(l.kind); lampKinds[k] = (lampKinds[k] ?? 0) + 1; if (on(l)) onKinds[k] = (onKinds[k] ?? 0) + 1; }
  const openings: any[] = quiet(() => { const o = planOpenings(statics); openAreas(o, dynamics); return o; });

  const counted = countPiece({ statics, moving: dynamics });
  const out = {
    level, file: file ?? 'office.json (bundled)', missions: rows.rows.map((e: any) => e.id), address: scenarioQuery(row), hour, hourStood: hourOf(g), started: started?.why ?? 'yes', plain: PLAIN,
    boxes: { file: fileBoxes, statics: statics.length, moving: dynamics.length, all: boxes.length, ...(({ boxes: _n, glowing: _g, ...rest }) => rest)(counted.all), cap: WORLD.maxBoxes },
    shapes, props: (src.props ?? []).length, doors: g.doors?.list?.length ?? (src.doors ?? []).length,
    lamps: { all: lamps.length, on: lamps.filter(on).length, kinds: lampKinds, onKinds, cap: MAX_LIGHTS, /** lit panels and signs: the area lamps (what the 11-02 page's column counted) */ panels: lampKinds.area ?? 0, panelsOn: onKinds.area ?? 0 },
    glowing: counted.all.glowing,
    openings: { found: openings.length, open: openings.filter(o => o.area >= LOOK_OPENINGS.small).length, keep: LOOK_OPENINGS.keep, litAtOnce: LOOK_OPENINGS.lit, area: +openings.reduce((s, o) => s + o.area, 0).toFixed(1) },
    picture: { cells, share: Object.fromEntries(Object.entries(byStuff).sort((a, b) => b[1] - a[1]).map(([k, n]) => [k, share(n)])), none: share(bare), saidFlat: share(said), tiltedNoMap: share(tiltedCells) },
    paints: [...paints.values()].sort((a, b) => b.cells - a.cells || b.boxes - a.boxes).map(p => ({ ...p, share: share(p.cells) })),
  };
  console.log('\n@@AUDIT@@' + JSON.stringify(out));
  process.exit(0);

  async function quietAsync<T>(fn: () => Promise<T>): Promise<T> { const l = console.log, w = console.warn, i = console.info; console.log = console.warn = console.info = () => { }; try { return await fn(); } finally { console.log = l; console.warn = w; console.info = i; } }
}

// ---------------------------------------------------------------------------------------------------------------------------------------------------------------
/** EVERY LEVEL: a child each, four at a time; the table, the files, and under --check the bars */
async function all() {
  const list = await listed(), want = named.length ? named : list.map(r => r.level), unknown = want.filter(n => !list.some(r => r.level === n));
  if (unknown.length) { console.error(`level-audit: not on the mission list: ${unknown.join(', ')} (the list has ${list.map(r => r.level).join(' ')})`); process.exit(2); }
  const child = (level: string) => new Promise<any>(res => { const p = spawn('bun', ['run', `${HERE}/level-audit.ts`, '--one', level, ...(PLAIN ? ['--plain'] : [])], { cwd: ROOT_DIR, env: process.env }); let o = '', e = '';
    p.stdout.on('data', (d: unknown) => { o += String(d); }); p.stderr.on('data', (d: unknown) => { e += String(d); });
    p.on('close', (code: number | null) => { const at = o.lastIndexOf('@@AUDIT@@'); if (at < 0) res({ level, error: `the level did not stand (exit ${code}): ${(e || o).trim().split('\n').slice(-3).join(' | ').slice(0, 400)}` }); else res(JSON.parse(o.slice(at + 9).split('\n')[0])); }); });
  const got: any[] = new Array(want.length); let next = 0;
  await Promise.all(Array.from({ length: Math.min(4, want.length) }, async () => { while (next < want.length) { const i = next++; got[i] = await child(want[i]); } }));
  mkdirSync(OUT, { recursive: true });
  for (const r of got) writeFileSync(`${OUT}/${r.level}${PLAIN ? '.plain' : ''}.json`, JSON.stringify(r, null, 1) + '\n');

  const pct = (v: number) => `${(100 * v).toFixed(v < 0.0995 ? 1 : 0)}%`;
  const head = ['level', 'hour', 'boxes', 'moving', 'drawn', 'cut', 'planes', 'round', 'bites', 'glass', 'lamps on/all', 'lit panels on/all', 'glowing', 'openings', 'props', 'no material', 'most of the picture'];
  const line = (r: any) => r.error ? [r.level, 'DID NOT STAND: ' + r.error] : [r.level, r.hour, r.boxes.all, r.boxes.moving, r.boxes.drawn, r.boxes.cut, r.boxes.planes, r.shapes.rounds + r.shapes.pass, r.shapes.walls, r.boxes.glass, `${r.lamps.on}/${r.lamps.all}`, `${r.lamps.panelsOn}/${r.lamps.panels}`, r.glowing, r.openings.found, r.props,
    pct(r.picture.none), Object.entries(r.picture.share as Record<string, number>).filter(([k]) => k !== 'none' && k !== 'plain' && k !== 'waits').slice(0, 4).map(([k, v]) => `${k} ${pct(v)}`).join(', ')];
  const table = [head, ...got.map(line)].map(c => c.map(String));
  const wide = head.map((_, i) => Math.max(...table.map(c => (c[i] ?? '').length)));
  console.log(`\n=== level-audit: ${got.length} level${got.length === 1 ? '' : 's'} as the look page draws ${got.length === 1 ? 'it' : 'them'}${PLAIN ? ' (--plain: none of the look page\'s shapes laid)' : ''}, each at the hour its mission opens at ===`);
  for (const c of table) console.log(c.map((s, i) => i < 2 || i === c.length - 1 ? s.padEnd(wide[i]) : s.padStart(wide[i])).join('  '));
  console.log(`(no GPU: counted, never seen. "round" = round things, by rule and by hand; "lit panels" = area lamps; "glowing" = drawn boxes that emit; "openings" = where daylight comes in.\n "no material" is a share of the picture from above, one ray a 12.5 cm cell along the play camera's line of sight: what the look draws flat, because the box says\n nothing of what it is made of, or says a word the look has no map for ('plain', 'waits', 'glass', 'rubber').)`);
  if (named.length || flag('--paints')) for (const r of got) { if (r.error) continue;
    console.log(`\n--- ${r.level}: its paints, the most of the picture first (${r.paints.length} paints; ${pct(r.picture.none)} of the picture has no material) ---`);
    for (const p of r.paints.filter((p: any) => p.cells > 0 || p.verdict !== 'material').slice(0, flag('--paints') && !named.length ? 12 : 40))
      console.log(`  ${pct(p.share).padStart(5)}  ${String(p.boxes).padStart(4)} boxes  ${p.stuff.padEnd(11)} ${(p.names.join(' / ') || '(no name)').slice(0, 34).padEnd(34)} ${p.albedo.join(' ').padEnd(22)} ${p.verdict === 'material' || p.verdict === 'snow' ? '' : p.verdict}${p.verdict !== 'material' && p.verdict !== 'snow' ? `   in: ${p.groups.join(', ') || 'nothing of the file (laid at run time, a prop or a man\'s kit)'}` : ''}`); }
  writeFileSync(`${OUT}/audit${PLAIN ? '.plain' : ''}.json`, JSON.stringify({ made: 'tools/qa/level-audit.ts', plain: PLAIN, levels: got.map(r => r.error ? r : { ...r, paints: undefined }) }, null, 1) + '\n');
  writeFileSync(`${OUT}/audit${PLAIN ? '.plain' : ''}.md`, `| ${head.join(' | ')} |\n|${head.map(() => '---').join('|')}|\n${got.map(r => `| ${line(r).join(' | ')} |`).join('\n')}\n`);
  console.log(`\nwritten: ${OUT}/<level>${PLAIN ? '.plain' : ''}.json (a level's paints are in its own file), audit${PLAIN ? '.plain' : ''}.json, audit${PLAIN ? '.plain' : ''}.md`);
  const stood = got.filter(r => !r.error);
  // ---- what moved since the baseline (the levels as they stood before anyone dressed them: tools/qa/fixtures/level-audit-base.json)
  const slim = (r: any) => ({ boxes: r.boxes.all, moving: r.boxes.moving, drawn: r.boxes.drawn, planes: r.boxes.planes, round: r.shapes.rounds + r.shapes.pass, bites: r.shapes.walls, lampsOn: r.lamps.on, lamps: r.lamps.all, panels: r.lamps.panels,
    glowing: r.glowing, openings: r.openings.found, props: r.props, none: r.picture.none });
  if (flag('--write-base')) { if (PLAIN || named.length || stood.length !== got.length) { console.error('level-audit --write-base: every level of the list, with the look\'s shapes, and all of them standing'); process.exit(2); }
    writeFileSync(BASE, JSON.stringify({ what: 'tools/qa/level-audit.ts --write-base: every level of the mission list as the look page drew it BEFORE it was dressed (the look as frozen for the join, 2026-11-03). The budget check\'s "heaviest level today" and the table page\'s "before" are read from here; a dressed level does not rewrite its row. Its `none` was counted by the colour rule of that day, before a box said what it is made of: a before to read beside today\'s, not a number to hold a level to by a tenth of a percent.', levels: Object.fromEntries(stood.map(r => [r.level, slim(r)])) }, null, 1) + '\n');
    console.log(`baseline written: ${BASE}`); process.exit(0); }
  const base: Record<string, ReturnType<typeof slim>> | null = existsSync(BASE) ? JSON.parse(readFileSync(BASE, 'utf8')).levels : null;
  if (base && !PLAIN) { const moved = stood.flatMap(r => { const b = base[r.level], n: any = slim(r); if (!b) return [`${r.level}: no row in the baseline (a new level)`];
      const d = Object.keys(n).filter(key => (b as any)[key] !== n[key]).map(key => key === 'none' ? `${key} ${pct((b as any)[key])} → ${pct(n[key])}` : `${key} ${(b as any)[key]} → ${n[key]}`); return d.length ? [`${r.level}: ${d.join(', ')}`] : []; });
    const fell = got.length - stood.length;   // (a level that did not stand has no counts to set against the baseline: say so, where "nothing moved" would read as all being well)
    console.log(moved.length ? `\nsince the baseline (before any dressing):\n  ${moved.join('\n  ')}${fell ? `\n  (${fell} of ${got.length} levels did not stand and are not compared)` : ''}` : fell ? `\nsince the baseline (before any dressing): NOT COMPARED for ${fell} of ${got.length} levels, which did not stand${stood.length ? '; nothing moved on the rest' : ''}` : '\nsince the baseline (before any dressing): nothing moved'); }
  // `--counts`: a level's row for the table page (tools/qa/level-table.ts): its baseline row and the row as it stands now, as a dressing task's row.json keeps them
  if (flag('--counts')) for (const r of stood) console.log(`\n"counts" for ${r.level}'s row.json:\n${JSON.stringify({ before: base?.[r.level] ?? null, after: slim(r) })}`);
  if (!CHECK) { process.exit(stood.length === got.length ? 0 : 1); }

  // ---- THE BUDGET CHECK
  let red = got.length !== stood.length; const calls: string[] = [];
  const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
  console.log(`\n=== level-audit --check: the engine's caps, the heaviest level of the list before dressing, and every surface a material${BRK ? ' (every bar at a tenth: must fail)' : ''} ===`);
  for (const r of got) if (r.error) check(false, `${r.level} stands`, r.error);
  if (!base) { check(false, `the baseline is there (${BASE})`, 'bun run tools/qa/level-audit.ts --write-base, on the levels before they are dressed'); process.exit(1); }
  const k = BRK ? 0.1 : 1, H = AUDIT_BARS.headroom, X = AUDIT_BARS.overHeaviest, all0 = Object.entries(base);
  const caps = stood.flatMap(r => ([[r.boxes.all, H * r.boxes.cap, 'boxes', r.boxes.cap], [r.lamps.on, H * r.lamps.cap, 'lamps burning', r.lamps.cap], [r.openings.found, r.openings.keep - 0.5, 'openings found (at the cap: the smallest were dropped)', r.openings.keep], [r.boxes.glass, 24, 'panes of glass', 24]] as [number, number, string, number][])
    .filter(([n, bar]) => n > bar * k).map(([n, , w, cap]) => `${r.level}: ${n} ${w}, the cap is ${cap}`));
  check(caps.length === 0, `the engine's own caps: no level is within a tenth of its boxes (${stood[0]?.boxes.cap}) or of the lamps that can burn at once (${stood[0]?.lamps.cap}), and none has as many openings as are kept (${stood[0]?.openings.keep}: at the cap the smallest lose their light) or more panes than a lamp can shine through (24)`, caps.join('; '));
  const bars: [string, keyof ReturnType<typeof slim>][] = [['boxes drawn', 'drawn'], ['boxes that move', 'moving'], ['cut planes', 'planes'], ['lamps burning at its hour', 'lampsOn'], ['boxes that glow', 'glowing']];
  for (const [what, key] of bars) { const [top, topN] = all0.reduce((m, [lv, b]) => (b[key] as number) > m[1] ? [lv, b[key] as number] : m, ['', 0] as [string, number]), bar = Math.max(1, topN) * X * k;
    const over = stood.filter(r => (slim(r)[key] as number) > bar).map(r => `${r.level} ${slim(r)[key]}`);
    check(over.length === 0, `${what}: no level asks more than ${X} × what the heaviest level of the list asked before dressing (${top}, ${topN}) — a PROPOSED bar, not ruled`, over.join(', '));
    const pass0 = base.pass?.[key] as number | undefined; if (pass0 !== undefined) { const more = stood.filter(r => r.level !== 'pass' && (slim(r)[key] as number) > AUDIT_BARS.overPass * Math.max(1, pass0)).map(r => `${r.level} ${slim(r)[key]}`); if (more.length) calls.push(`${what}, over ${AUDIT_BARS.overPass} × the pass's ${pass0}: ${more.join(', ')}`); } }
  const bare = stood.filter(r => r.picture.none > AUDIT_BARS.bare * k).map(r => `${r.level} ${pct(r.picture.none)}`);
  check(bare.length === 0, `every surface is a material: no level keeps over ${pct(AUDIT_BARS.bare)} of its picture with none (a PROPOSED bar; the pass has ${base.pass ? pct(base.pass.none) : '?'})`, bare.length ? `${bare.join(', ')} — its paints: bun run tools/qa/level-audit.ts <level>` : '');
  if (calls.length) console.log(`\nNOT RED, FOR A GPU: the look was timed on one level, the pass (GPU 12.2 to 15.3 ms on an M4 Max, 11-02). These ask more of a frame than it does:\n  ${calls.join('\n  ')}`);
  console.log(red ? '=== level-audit: FAILED ===' : '=== level-audit: ALL PASS ===');
  process.exit(red ? 1 : 0);
}
