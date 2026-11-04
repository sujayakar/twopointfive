// A PAINT'S NAME SAYS WHAT IT IS MADE OF (src/scene/stuff.ts PAINT_STUFF, src/render/gbuffer.ts lookNames).
//   bun run tools/qa/stuff-check.ts            exit 0 = every line passes
//   STUFF_BREAK=1 bun run tools/qa/stuff-check.ts     the level's rows are never handed over (the colour rule alone, as the look began): must FAIL
// No game is stood up and no picture is looked at: the look's word for a box is decided on the CPU (lookWordOf) and this asks it. What a named material LOOKS
// like on a desk is a page's still and then a GPU's.
import { readFileSync, readdirSync } from 'node:fs';
import { ROOT } from './headless.ts';
const { PAINT_STUFF } = await import(`${ROOT}/src/scene/stuff.ts`);
const { LOOK_NAMES } = await import(`${ROOT}/src/render/lookTex.ts`);
const { LOOK_OF, LOOK_BY_NAME, lookClassOf, lookNames, lookStuff, lookWordOf } = await import(`${ROOT}/src/render/gbuffer.ts`);
const { GWOT_PALETTE } = await import(`${ROOT}/src/scene/gwotPalette.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel, levelFileOf } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { makeBox, BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);

const BRK = process.env.STUFF_BREAK === '1';
let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== stuff: a paint's name says what it is made of${BRK ? ' (STUFF_BREAK: the rows are never handed over)' : ''} ===`);
const hand = (rows: any) => lookNames(BRK ? null : rows);
const FILES = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).sort();
const dataOf = (f: string) => parseLevelData(readFileSync(`${ROOT}/levels/${f}`, 'utf8'));
const word = lookWordOf(), cls = (albedo: readonly number[], flags = 0) => word(makeBox({ c: [3, 1, 4], h: [0.5, 0.5, 0.5], albedo: [albedo[0], albedo[1], albedo[2]], flags }), (flags & BoxFlag.Dynamic) !== 0) & 15;

// 1. every name a level file writes has a row, and every row says a stuff the look has
{ const none = new Map<string, string>(), bad: string[] = []; let names = 0; const seen = new Set<string>();
  for (const f of FILES) for (const name of Object.keys(dataOf(f).materials)) { if (!seen.has(name)) { seen.add(name); names++; } if (!(name in PAINT_STUFF) && !none.has(name)) none.set(name, f); }
  for (const [name, stuff] of Object.entries(PAINT_STUFF)) if (stuff !== 'plain' && stuff !== 'waits' && !(LOOK_NAMES as readonly string[]).includes(stuff as string)) bad.push(`${name}: '${stuff}'`);
  check(none.size === 0 && bad.length === 0 && names > 60, '1 every paint name in every level file has a row in the table (a material, or said to be plain, or said to wait for one), and every row names a material the look draws',
    `${names} names in ${FILES.length} level files · ${Object.keys(PAINT_STUFF).length} rows${none.size ? ` · NO ROW: ${[...none].map(([n, f]) => `${n} (${f})`).join(', ')}` : ''}${bad.length ? ` · not a material: ${bad.join(', ')}` : ''}`); }

// 2. the palette's own rows are what they were, in the order the shader's text is written in
{ const WAS = 'mud:1 mudDark:1 mudLight:1 render:2 whitewash:2 paintCream:2 block:3 tWall:3 dust:4 dirt:4 mudFloor:4 damp:4 gravel:5 slab:6 poplar:7 woodGrey:7 ply:7 galv:8 tin:8 sack:9 sandbag:9 hesco:9 canvas:9 tarpBlue:9 blanketGrey:9 blanketBrown:9 rugRed:9 rugNavy:9 rugCream:9 rugOchre:9 rugGreen:9 rugMadder:9 coyote:9 rust:10 iron:10 paintBlue:11 paintGreen:11 paintRust:11 drumBlue:11 drumRed:11 genYellow:11 truckWhite:11 truckTan:11 truckRed:11 olive:11';
  const now = Object.entries(LOOK_OF).map(([k, v]) => `${k}:${v}`).join(' '); lookNames(null); let moved = 0;
  for (const m of Object.keys(GWOT_PALETTE)) for (const k of [1, 0.7]) { const a = (GWOT_PALETTE as any)[m].map((v: number) => v * k), want = k === 1 ? (LOOK_OF as any)[m] ?? 0 : null; if (want !== null && lookStuff(a) !== want) moved++; }
  check(now === WAS && moved === 0 && Object.keys(GWOT_PALETTE).every(m => m in PAINT_STUFF), '2 the palette\'s rows are drawn as they were: the classes the look\'s shader text is written from are the same, in the same order, every palette row is in the table, and by colour alone each row is its own class',
    now === WAS ? `${Object.keys(LOOK_OF).length} classed rows of ${Object.keys(GWOT_PALETTE).length} · ${moved} moved` : `now: ${now}`); }

// 3. a box that wears a named paint is drawn as its name says, on every level file: static, and moving (a loose prop's part, a door's leaf)
{ const wrong: string[] = []; let rows = 0, byName = 0, files = 0;
  for (const f of FILES) { const M = dataOf(f).materials, first = new Map<string, string>(); files++; hand(M);
    for (const [name, m] of Object.entries(M) as [string, any][]) { const key = m.albedo.join(','); if (first.has(key)) continue; first.set(key, name); rows++;
      const want = lookClassOf(PAINT_STUFF[name]), st = cls(m.albedo), mv = cls(m.albedo, BoxFlag.Dynamic); lookNames(null); const colour = lookStuff(m.albedo); hand(M);
      if (colour !== want) byName++;
      if (st !== want || mv !== want) wrong.push(`${f} ${name}: ${st} standing, ${mv} moving, wanted ${want}`); } }
  lookNames(null);
  check(wrong.length === 0 && byName > 100, '3 on every level file, a box that wears a row\'s paint is drawn as that row\'s name says, standing or moving; many of them are paints the colour rule alone draws as something else',
    `${rows} rows in ${files} files · ${byName} of them differ from what the colour alone gives${wrong.length ? ` · ${wrong.slice(0, 5).join(' · ')}` : ''}`); }

// 4. the office's furniture and floors, by name; and what the colour alone made of them
{ const M = dataOf('office.json').materials, want: Record<string, string> = { wood: 'wood', woodDark: 'wood', chair: 'cloth', carpet: 'cloth', carpetBlue: 'cloth', metal: 'paint', wall: 'plaster', concrete: 'block', lino: 'slab', asphalt: 'gravel', black: 'plain', cardboard: 'waits' };
  hand(M); const got = Object.keys(want).map(n => cls((M as any)[n].albedo)); lookNames(null); const was = Object.keys(want).map(n => lookStuff((M as any)[n].albedo));
  const said = (c: number) => c === 0 ? 'plain' : LOOK_NAMES[c - 1];
  check(Object.keys(want).every((n, i) => got[i] === lookClassOf(want[n])) && was[0] === 0 && was[2] === 0 && was[3] === 0, '4 the office\'s paints: a desk is wood, a chair and a carpet cloth, steel shelving painted steel, a partition plaster, the lot gravel, a bezel plain because its name says so; by the colour alone the desk, the chair and the carpet were nobody\'s',
    Object.keys(want).map((n, i) => `${n} ${said(got[i])}${got[i] !== was[i] ? ` (was ${said(was[i])})` : ''}`).join(' · ')); }

// 5. a row may say its own; a name with no row and no word is the colour's; two rows of one colour: the first; nothing handed over is changed
{ const rows = { accent: { albedo: [0.5, 0.2, 0.6], stuff: 'cloth' }, wood: { albedo: [0.52, 0.4, 0.27], stuff: 'paint' }, nobody: { albedo: [0.31, 0.62, 0.11] }, tWall: { albedo: [0.9, 0.1, 0.1] }, second: { albedo: [0.9, 0.1, 0.1], stuff: 'wood' }, dustLike: { albedo: GWOT_PALETTE.dust } }, before = JSON.stringify(rows);
  const took = hand(rows), a = cls(rows.accent.albedo), w = cls(rows.wood.albedo), n = cls(rows.nobody.albedo), t = cls(rows.tWall.albedo), d = cls(rows.dustLike.albedo as any); lookNames(null);
  check(a === 9 && w === 11 && n === 0 && t === 3 && d === 4 && took === 3 && JSON.stringify(rows) === before, '5 a row that says its own stuff is drawn as it (a level\'s accent is a material, and a row may overrule the table); a name with no row and no word is left to the colour rule; of two rows with one colour the first is taken, as the file\'s writer takes it; the rows handed over are not changed (a name never changes a colour)',
    `accent ${a} · a row saying paint over the table's wood ${w} · unnamed ${n} · two rows one colour ${t} · a palette colour under another name ${d} · ${took} rows taken`); }

// 6. the page: the rows of the level it stands on are handed over before the look's word is, and a word puts it off
{ const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8'), at = main.indexOf("lookNames(q.get('names') === '0' ? null : levelFileOf.get(game.level)?.materials)"), set = main.indexOf('engine.world.setStuffOf(lookWordOf(');
  const L = loadLevel(dataOf('library-hall.json')), kept = levelFileOf.get(L)?.materials;
  check(at > 0 && set > at && !!kept && 'wood' in kept && Object.keys(LOOK_BY_NAME).length === Object.keys(PAINT_STUFF).length, '6 the page hands the look the paint rows of the level file it stood up (kept beside the level by the loader) before the look\'s word for a box is set, and `?names=0` hands it none; the audit\'s table of names is the whole table',
    `handed at ${at}, the word set at ${set} · ${kept ? Object.keys(kept).length : 0} rows kept for library-hall`); }

console.log(`=== stuff: ${red ? 'FAILURES above' : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
