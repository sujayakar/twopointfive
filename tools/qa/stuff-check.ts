// A PAINT'S NAME SAYS WHAT IT IS MADE OF (src/scene/stuff.ts PAINT_STUFF, src/render/gbuffer.ts lookNamed and lookWordOf).
//   bun run tools/qa/stuff-check.ts            exit 0 = every line passes
//   STUFF_BREAK=1 bun run tools/qa/stuff-check.ts     the level's rows are never handed over (the colour rule alone, as the look began): must FAIL
// No game is stood up and no picture is looked at: the look's word for a box is decided on the CPU (lookWordOf) and this asks it. What a named material LOOKS
// like on a desk is a page's still and then a GPU's. The same on both stands: nothing here is a rule of play and no stand's word reaches it.
import { readFileSync, readdirSync } from 'node:fs';
import { ROOT } from './headless.ts';
const { PAINT_STUFF, stuffOfName } = await import(`${ROOT}/src/scene/stuff.ts`);
const { LOOK_NAMES } = await import(`${ROOT}/src/render/lookTex.ts`);
const { LOOK_OF, LOOK_BY_NAME, lookClassOf, lookNamed, lookStuff, lookWordOf } = await import(`${ROOT}/src/render/gbuffer.ts`);
const { GWOT_PALETTE } = await import(`${ROOT}/src/scene/gwotPalette.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel, levelFileOf } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { lookPageWord } = await import(`${ROOT}/src/scene/look.ts`);
const { makeBox, BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);

const BRK = process.env.STUFF_BREAK === '1';
let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== stuff: a paint's name says what it is made of${BRK ? ' (STUFF_BREAK: the rows are never handed over)' : ''} ===`);
/** the look's word for a box, made with a level's rows handed over as the page hands them */
const wordWith = (rows: any) => lookWordOf(undefined, BRK ? null : rows);
const FILES = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).sort();
const dataOf = (f: string) => parseLevelData(readFileSync(`${ROOT}/levels/${f}`, 'utf8'));
const boxIn = (albedo: readonly number[], flags = 0) => makeBox({ c: [3, 1, 4], h: [0.5, 0.5, 0.5], albedo: [albedo[0], albedo[1], albedo[2]], flags });
const said = (c: number) => c === 0 ? 'plain' : LOOK_NAMES[c - 1];
/** what a row's name should make of its colour: the name's class, or for a mixed name or one with no row, whatever the colour alone gives */
const wanted = (name: string, albedo: readonly number[]) => { const s = stuffOfName(name); return s === undefined || s === 'mixed' ? lookStuff(albedo) : lookClassOf(s); };

// 1. every name a level file writes has a row, and every row says a stuff the look has
{ const none = new Map<string, string>(), bad: string[] = [], seen = new Set<string>(), kinds: Record<string, number> = { material: 0, plain: 0, waits: 0, mixed: 0 };
  for (const f of FILES) for (const name of Object.keys(dataOf(f).materials)) { seen.add(name); if (stuffOfName(name) === undefined && !none.has(name)) none.set(name, f); }
  for (const [name, stuff] of Object.entries(PAINT_STUFF) as [string, string][]) { if (stuff === 'plain' || stuff === 'waits' || stuff === 'mixed') kinds[stuff]++; else if ((LOOK_NAMES as readonly string[]).includes(stuff)) kinds.material++; else bad.push(`${name}: '${stuff}'`); }
  check(none.size === 0 && bad.length === 0 && seen.size > 60 && stuffOfName('constructor') === undefined && stuffOfName('toString') === undefined, '1 every paint name in every level file has a row of its own in the table, and every row says a material the look draws, or plain, or waits, or mixed; a name that is a method of every object has no row',
    `${seen.size} names in ${FILES.length} level files · ${Object.keys(PAINT_STUFF).length} rows: ${kinds.material} a material, ${kinds.plain} plain, ${kinds.waits} waiting, ${kinds.mixed} mixed${none.size ? ` · NO ROW: ${[...none].map(([n, f]) => `${n} (${f})`).join(', ')}` : ''}${bad.length ? ` · not a material: ${bad.join(', ')}` : ''}`); }

// 2. the palette's own rows are what they were, in the order the shader's text is written in
{ const WAS = 'mud:1 mudDark:1 mudLight:1 render:2 whitewash:2 paintCream:2 block:3 tWall:3 dust:4 dirt:4 mudFloor:4 damp:4 gravel:5 slab:6 poplar:7 woodGrey:7 ply:7 galv:8 tin:8 sack:9 sandbag:9 hesco:9 canvas:9 tarpBlue:9 blanketGrey:9 blanketBrown:9 rugRed:9 rugNavy:9 rugCream:9 rugOchre:9 rugGreen:9 rugMadder:9 coyote:9 rust:10 iron:10 paintBlue:11 paintGreen:11 paintRust:11 drumBlue:11 drumRed:11 genYellow:11 truckWhite:11 truckTan:11 truckRed:11 olive:11';
  const now = Object.entries(LOOK_OF).map(([k, v]) => `${k}:${v}`).join(' '), moved = Object.keys(GWOT_PALETTE).filter(m => lookStuff((GWOT_PALETTE as any)[m]) !== ((LOOK_OF as any)[m] ?? 0));
  check(now === WAS && moved.length === 0, '2 the palette\'s rows are drawn as they were: the classes the look\'s shader text is written from are the same, in the same order, and by colour alone each palette row is its own class',
    now === WAS ? `${Object.keys(LOOK_OF).length} classed rows of ${Object.keys(GWOT_PALETTE).length} · ${moved.length} moved${moved.length ? `: ${moved.join(', ')}` : ''}` : `now: ${now}`); }

// 3. every level file: a box that wears a row's paint is drawn as the FIRST row of that colour says (the row the file's writer names it by), standing or moving
{ const wrong: string[] = []; let rows = 0, byName = 0, later = 0;
  for (const f of FILES) { const M = dataOf(f).materials as Record<string, { albedo: number[] }>, word = wordWith(M), first = new Map<string, string>();
    for (const [name, m] of Object.entries(M)) { const key = m.albedo.join(','), head = first.get(key) ?? name; if (!first.has(key)) first.set(key, name); else later++;
      const want = wanted(head, m.albedo), st = word(boxIn(m.albedo), false) & 15, mv = word(boxIn(m.albedo, BoxFlag.Dynamic), true) & 15; rows++;
      if (want !== lookStuff(m.albedo)) byName++;
      if (st !== want || mv !== want) wrong.push(`${f} ${name}${head !== name ? ` (written as ${head})` : ''}: ${said(st)} standing, ${said(mv)} moving, wanted ${said(want)}`); } }
  check(wrong.length === 0 && byName > 100, '3 on every level file, a box that wears a row\'s paint is drawn as the first row of that colour says, standing or moving (a loose prop\'s part); a mixed name and a name with no row are the colour\'s; many rows are paints the colour alone draws as something else',
    `${rows} rows in ${FILES.length} files (${later} of them a later row of a colour another has) · ${byName} differ from what the colour alone gives${wrong.length ? ` · ${wrong.slice(0, 5).join(' · ')}` : ''}`); }

// 4. the office's own paints, by name; and what the colour alone made of them
{ const M = dataOf('office.json').materials as Record<string, { albedo: number[] }>, word = wordWith(M), plainWord = lookWordOf();
  const want: Record<string, string> = { wood: 'wood', woodDark: 'wood', chair: 'cloth', carpet: 'cloth', carpetBlue: 'cloth', wall: 'plaster', concrete: 'block', lino: 'slab', stone: 'slab', black: 'plain', metal: 'plain', cardboard: 'plain' };
  const got = Object.keys(want).map(n => word(boxIn(M[n].albedo), false) & 15), was = Object.keys(want).map(n => plainWord(boxIn(M[n].albedo), false) & 15);
  check(Object.keys(want).every((n, i) => said(got[i]) === want[n]) && was[0] === 0 && was[2] === 0 && was[3] === 0, '4 the office\'s paints: a desk is wood, a chair and a carpet cloth, a partition wall plaster, the corridor slab; a bezel is plain because its name says so, and steel and card plain because they wait for a material; by the colour alone the desk, the chair and the carpet were nobody\'s',
    Object.keys(want).map((n, i) => `${n} ${said(got[i])}${got[i] !== was[i] ? ` (was ${said(was[i])})` : ''}`).join(' · ')); }

// 5. one name, more than one thing: the things that wear another thing's numbers are not drawn as it
{ const H = dataOf('library-hall.json'), word = wordWith(H.materials), L = loadLevel(H), file = levelFileOf.get(L)!.boxes as any[];
  const car = file.map((b, i) => ({ b, i })).filter(({ b }) => b.group === 'yard.office' && b.material === 'partBlue').map(({ i }) => word(L.boxes[i], false) & 15);
  const leaf = word(boxIn([0.2, 0.22, 0.25], BoxFlag.Dynamic), true) & 15, iron = word(boxIn(H.materials.asphalt.albedo), false) & 15, stucco = wordWith(dataOf('compound.json').materials)(boxIn(dataOf('compound.json').materials.partTan.albedo), false) & 15;
  const rows = { trim: { albedo: [0.9, 0.1, 0.1] }, wood: { albedo: [0.9, 0.1, 0.1] }, constructor: { albedo: GWOT_PALETTE.mud }, carpet: { albedo: [0.2, 0.7, 0.3] }, tWall: { albedo: [0.2, 0.7, 0.3] } }, before = JSON.stringify(rows), w2 = wordWith(rows);
  const trim = w2(boxIn(rows.trim.albedo), false) & 15, proto = w2(boxIn(GWOT_PALETTE.mud as any), false) & 15, firstWins = w2(boxIn(rows.carpet.albedo), false) & 15, taken = lookNamed(BRK ? null : rows).size;
  check(car.length >= 2 && car.every(c => c === lookStuff(H.materials.partBlue.albedo)) && leaf === lookStuff([0.2, 0.22, 0.25]) && iron === lookStuff(H.materials.asphalt.albedo) && stucco === lookStuff(dataOf('compound.json').materials.partTan.albedo)
    && trim === 0 && proto === 1 && firstWins === 9 && taken === 1 && JSON.stringify(rows) === before,
    '5 one name, more than one thing: the showroom\'s car body (written as the blue partition), an outside door\'s leaf (the window frame\'s numbers), the lived kit\'s iron (the car park\'s) and a hut\'s stucco (the tan partition\'s) are what the colour alone makes of them, not what the name\'s other thing is; a colour whose first row has no row in the table is the colour\'s whatever a later row says; of two named rows the first is taken; a row named like a method is no name; nothing handed over is changed',
    `car body ${car.map(said).join(', ')} · outside leaf ${said(leaf)} · iron fitting ${said(iron)} · stucco ${said(stucco)} · unnamed first, wood second: ${said(trim)} · a row called constructor in mud's colour: ${said(proto)} · carpet first, tWall second: ${said(firstWins)} · ${taken} colour taken by name of the three`); }

// 6. the page: the rows of the level it stands on go into the look's word, a word puts it off and a benchmark carries that word
{ const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8'), wired = main.includes("engine.world.setStuffOf(lookWordOf(b => game.weather.dryPaint(b), q.get('names') === '0' ? null : levelFileOf.get(game.level)?.materials))");
  const L = loadLevel(dataOf('library-hall.json')), kept = levelFileOf.get(L)?.materials, mixedOut = Object.keys(LOOK_BY_NAME).every(n => PAINT_STUFF[n] !== 'mixed') && Object.keys(LOOK_BY_NAME).length < Object.keys(PAINT_STUFF).length;
  check(wired && !!kept && Object.hasOwn(kept, 'wood') && lookPageWord('names', '0') && mixedOut, '6 the page makes the look\'s word for a box with the paint rows of the level file it stood up (kept beside the level by the loader), `?names=0` hands it none, and the word is one of the look page\'s own, so a benchmark asked for with it carries it; the table the audit reads has no mixed name in it',
    `wired ${wired} · ${kept ? Object.keys(kept).length : 0} rows kept for library-hall · names is a look page word: ${lookPageWord('names', '0')} · ${Object.keys(LOOK_BY_NAME).length} names for the audit of ${Object.keys(PAINT_STUFF).length}`); }

console.log(`=== stuff: ${red ? 'FAILURES above' : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
