// A PAINT'S NAME SAYS WHAT IT IS MADE OF (src/scene/stuff.ts PAINT_STUFF, src/render/gbuffer.ts lookNamed and lookWordOf).
//   bun run tools/qa/stuff-check.ts            exit 0 = every line passes
//   STUFF_BREAK=1 bun run tools/qa/stuff-check.ts     the level's rows are never handed over (the colour rule alone, as the look began): must FAIL
// No game is stood up and no picture is looked at: the look's word for a box is decided on the CPU (lookWordOf) and this asks it. What a named material LOOKS
// like on a desk is a page's still and then a GPU's. The same on both stands: nothing here is a rule of play and no stand's word reaches it.
import { readFileSync, readdirSync } from 'node:fs';
import { ROOT } from './headless.ts';
const { PAINT_STUFF, stuffOfName, BOX_STUFFS, saidFor } = await import(`${ROOT}/src/scene/stuff.ts`);
const { LOOK_NAMES, LOOK_PAINTED, lookStuffOf } = await import(`${ROOT}/src/render/lookTex.ts`);
const { LOOK_OF, LOOK_BY_NAME, lookNamed, lookStuff, lookWordOf } = await import(`${ROOT}/src/render/gbuffer.ts`);
const { GWOT_PALETTE } = await import(`${ROOT}/src/scene/gwotPalette.ts`);
const { parseLevelData, serializeLevelData, validateLevelData, KEYS } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel, levelToData, levelFileOf } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { lookPageWord } = await import(`${ROOT}/src/scene/look.ts`);
const { makeBox, BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const { PropSystem } = await import(`${ROOT}/src/game/props.ts`), { StaticCollision } = await import(`${ROOT}/src/scene/collision.ts`);
const KD = await import(`${ROOT}/src/scene/levelKit.ts`), { newKit, KIT_PALETTE } = KD;
const { Editor } = await import(`${ROOT}/src/editor/editor.ts`), { boxPaint } = await import(`${ROOT}/src/editor/paint.ts`);

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
const wanted = (name: string, albedo: readonly number[]) => { const s = stuffOfName(name); return s === undefined || s === 'mixed' ? lookStuff(albedo) : lookStuffOf(s); };

// 1. every name a level file writes has a row, and every row says a stuff the look has
{ const none = new Map<string, string>(), bad: string[] = [], seen = new Set<string>(), kinds: Record<string, number> = { material: 0, plain: 0, waits: 0, mixed: 0 };
  for (const f of FILES) for (const name of Object.keys(dataOf(f).materials)) { seen.add(name); if (stuffOfName(name) === undefined && !none.has(name)) none.set(name, f); }
  for (const [name, stuff] of Object.entries(PAINT_STUFF) as [string, string][]) { if (stuff === 'plain' || stuff === 'waits' || stuff === 'mixed') kinds[stuff]++; else if (lookStuffOf(stuff) > 0) kinds.material++; else bad.push(`${name}: '${stuff}'`); }
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
  const want: Record<string, string> = { wood: 'wood', woodDark: 'wood', chair: 'fabric', carpet: 'cloth', carpetBlue: 'cloth', wall: 'plaster', concrete: 'block', lino: 'slab', stone: 'slab', metal: 'steel', black: 'plain', cardboard: 'plain' };
  const got = Object.keys(want).map(n => word(boxIn(M[n].albedo), false) & 15), was = Object.keys(want).map(n => plainWord(boxIn(M[n].albedo), false) & 15);
  check(Object.keys(want).every((n, i) => said(got[i]) === want[n]) && was[0] === 0 && was[2] === 0 && was[3] === 0, '4 the office\'s paints: a desk is wood, a chair fabric, a carpet cloth, steel shelving steel, a partition wall plaster, the corridor slab; a bezel is plain because its name says so, and card plain because it waits for a material; by the colour alone the desk, the chair and the carpet were nobody\'s',
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
{ const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8'), wired = main.includes("engine.world.setStuffOf(lookWordOf(b => game.weather.dryPaint(b), q.get('names') === '0' ? null : levelFileOf.get(game.level)?.materials, q.get('names') !== '0' && q.get('words') !== '0'))");
  const L = loadLevel(dataOf('library-hall.json')), kept = levelFileOf.get(L)?.materials, mixedOut = Object.keys(LOOK_BY_NAME).every(n => PAINT_STUFF[n] !== 'mixed') && Object.keys(LOOK_BY_NAME).length < Object.keys(PAINT_STUFF).length;
  check(wired && !!kept && Object.hasOwn(kept, 'wood') && lookPageWord('names', '0') && lookPageWord('words', '0') && mixedOut, '6 the page makes the look\'s word for a box with the paint rows of the level file it stood up (kept beside the level by the loader); `?names=0` hands it none and puts the boxes\' own words off with them, `?words=0` puts the boxes\' own words off alone; both are words of the look page\'s own, so a benchmark asked for with one carries it; the table the audit reads has no mixed name in it',
    `wired ${wired} · ${kept ? Object.keys(kept).length : 0} rows kept for library-hall · look page words: names ${lookPageWord('names', '0')}, words ${lookPageWord('words', '0')} · ${Object.keys(LOOK_BY_NAME).length} names for the audit of ${Object.keys(PAINT_STUFF).length}`); }

// 7. a box says its own word: over its paint's name and its colour, through the level file and back, on a loose prop's part too
{ const D = dataOf('library-hall.json') as any, M = D.materials, bi = D.boxes.findIndex((b: any) => b.group === 'yard.office' && b.material === 'partBlue'), wi = D.boxes.findIndex((b: any) => b.material === 'wood' && b.flags === undefined), fi = D.boxes.findIndex((b: any) => b.material !== undefined && b.flags !== undefined), pi = D.props.findIndex((p: any) => p.kind === 'chair');
  if (bi < 0 || wi < 0 || fi < 0 || pi < 0) check(false, '7 a box says its own word', `the showroom has no car body in the partition's blue, wooden box, flagged box or chair to ask (${bi}, ${wi}, ${fi}, ${pi})`);
  else {
    D.boxes[bi].stuff = 'paint'; D.boxes[wi].stuff = 'plain'; D.boxes[fi].stuff = 'slab'; D.props[pi].parts[0].stuff = 'plaster';
    const text = serializeLevelData(D), back = parseLevelData(text) as any, errs = validateLevelData(back), L = loadLevel(back), word = wordWith(M), namesOff = lookWordOf(undefined, null, false), wordsOff = lookWordOf(undefined, BRK ? null : M, false);
    const car = word(L.boxes[bi], false) & 31, desk = word(L.boxes[wi], false) & 31, carOff = namesOff(L.boxes[bi], false) & 31, deskNamed = wordsOff(L.boxes[wi], false) & 31;
    // the box the GAME makes of a prop's part (game/props.ts Prop), which is also the box the page asks before the first frame
    const seat = word(new PropSystem([L.props[pi]], new StaticCollision(L.boxes), () => {}).all[0].boxes[0], true) & 31;
    const again = levelToData(L, { materials: M }) as any, kept = again.boxes[bi].stuff === 'paint' && again.boxes[wi].stuff === 'plain' && again.props[pi].parts[0].stuff === 'plaster' && again.boxes.every((b: any, i: number) => b.stuff === D.boxes[i].stuff) && again.boxes.some((b: any) => b.stuff === undefined);
    // where it is written: after the finish, before the flags, on a box that has flags; and in the format's own key lists
    const line = text.split('\n').find((l: string) => l.includes(`"id": "${D.boxes[fi].id}"`) || l.includes(`"id":"${D.boxes[fi].id}"`)) ?? '', kb = KEYS.box.indexOf('stuff'), kp = KEYS.part.indexOf('stuff');
    const placed = /"stuff":\s*"slab".*"flags"/.test(line) && !/"stuff".*"(material|albedo|rough|metal)"/.test(line) && KEYS.box[kb - 1] === 'metal' && KEYS.box[kb + 1] === 'flags' && KEYS.part[kp - 1] === 'albedo';
    // refused: on a box, on a prop's part, and on a box of a gear screen's set (a picture's box is the format's own record)
    const KV = FILES.find(f => Object.values((dataOf(f) as any).scenarios?.kitView?.sets ?? {}).some((x: any) => x.boxes?.length));
    const refuse = (w: string, where: 'box' | 'part' | 'set' = 'box') => { const X = dataOf(where === 'set' ? KV! : 'library-hall.json') as any;
      if (where === 'box') X.boxes[0].stuff = w; else if (where === 'part') X.props[0].parts[0].stuff = w; else (Object.values(X.scenarios.kitView.sets).find((x: any) => x.boxes?.length) as any).boxes[0].stuff = w;
      return validateLevelData(X).some((e: string) => e.includes('stuff')); };
    const masonry = (LOOK_NAMES as readonly string[]).slice(LOOK_PAINTED).join(', ');
    check(errs.length === 0 && car === lookStuffOf('paint') && desk === 0 && carOff === lookStuff(M.partBlue.albedo) && deskNamed === lookStuffOf('wood') && seat === lookStuffOf('plaster') && kept && placed
      && refuse('mixed') && refuse('banana') && !refuse('waits') && !refuse('veneer') && refuse('banana', 'part') && !refuse('fabric', 'part') && !!KV && refuse('banana', 'set') && !refuse('steel', 'set')
      && BOX_STUFFS.length === LOOK_PAINTED + 4 && !refuse('glass') && !refuse('rubber') && masonry === 'field stone, mud brick' && refuse('mud brick') && refuse('field stone') && lookStuffOf('mud brick') === 0,
      '7 a box says its own word: the showroom\'s car body, written in the partition\'s blue, says paint and is paint; a wooden box that says plain is plain; a chair\'s part says its own and the box the game makes of it (the box the page asks before the first frame) carries it; the words go through the level file and back and are written only where they were said, after the finish and before the flags; a word the look has no material for is refused when the file is read, on a box, a part and a gear screen\'s box, the stone and brick under a mud wall among them; with the words off a box is its name\'s again, and with names off its colour\'s',
      `car body ${said(car)} (names off: ${said(carOff)}) · the wooden box ${said(desk)} (words off: ${said(deskNamed)}) · the chair's part ${said(seat)} · kept through the file ${kept} · placed ${placed} · refused: mixed ${refuse('mixed')}, banana ${refuse('banana')}, mud brick ${refuse('mud brick')}, a part's ${refuse('banana', 'part')}, a set's ${KV ? refuse('banana', 'set') : 'no file'} (${KV}); waits ${refuse('waits')} · ${errs.length} errors reading it back${errs.length ? `: ${errs.slice(0, 2).join(' | ')}` : ''}`); } }

// 8. a word belongs to the paint it was said with: a piece says it for its own paints, and whoever repaints a box drops it
{ const k = newKit(), n0 = k.boxes.length; KD.desk(k, 2, 2, 1.6, 0.8); const own = k.boxes.slice(n0), n1 = k.boxes.length; KD.desk(k, 6, 2, 1.6, 0.8, 0, KIT_PALETTE.woodDark); const dark = k.boxes.slice(n1), n2 = k.boxes.length;
  KD.desk(k, 2, 6, 1.6, 0.8, 0, KIT_PALETTE.metal); const steel = k.boxes.slice(n2), n3 = k.boxes.length; KD.desk(k, 6, 6, 1.6, 0.8, 0, KIT_PALETTE.white); const white = k.boxes.slice(n3), n4 = k.boxes.length;
  KD.rug(k, 4, 4, 2, 1, [0.3, 0.1, 0.09]); const rug = k.boxes.slice(n4);
  const says = (l: any[], w?: string) => l.length > 0 && l.every((b: any) => b.stuff === w), word = wordWith(Object.fromEntries(Object.entries(KIT_PALETTE).map(([n, a]) => [n, { albedo: a }]))), cls = (b: any) => word(b, false) & 31;
  // the levels as written: no door frame says a word (a frame goes with its leaf, and both wait); in a look's file no box the skin repainted keeps the plain file's word
  let frames = 0, kept = 0, twins = 0, twinWords = 0;
  for (const f of FILES) { const D = dataOf(f) as any; for (const b of D.boxes) if (b.stuff !== undefined && /^doorframe\./.test(b.group ?? '')) frames++;
    if (!f.endsWith('-gwot.json') || !FILES.includes(f.replace('-gwot.json', '.json'))) continue; twins++;
    const P = dataOf(f.replace('-gwot.json', '.json')) as any; D.boxes.forEach((b: any, i: number) => { if (b.stuff === undefined) return; twinWords++; const a = P.boxes[i]; if (a && (a.material !== b.material || JSON.stringify(a.albedo) !== JSON.stringify(b.albedo))) kept++; }); }
  // the editor: painting a box with another paint drops the word, in one command that undoes
  const ED = dataOf('library-hall.json') as any, eb = ED.boxes.find((b: any) => b.material === 'wood'); eb.stuff = 'veneer'; const ed = new Editor(ED, {}); boxPaint(ed, [eb.id], 'concrete');
  const painted = ed.data.boxes.find((b: any) => b.id === eb.id), dropped = painted.material === 'concrete' && painted.stuff === undefined;
  const skin = readFileSync(`${ROOT}/tools/levels/gwotSkin.ts`, 'utf8').includes('b.albedo = gwotAs(m, sim); b.rough = GWOT_FINISH[m]?.rough ?? ROUGH_CLOTH; delete b.stuff;');
  check(says(own, 'veneer') && says(dark, 'veneer') && says(steel) && says(white) && says(rug, 'cloth') && steel.every((b: any) => cls(b) === lookStuffOf('steel')) && own.every((b: any) => cls(b) === lookStuffOf('veneer'))
    && JSON.stringify(saidFor('veneer', [0.3, 0.31, 0.33], KIT_PALETTE.wood, KIT_PALETTE.woodDark)) === '{}' && frames === 0 && kept === 0 && twins > 10 && dropped && skin,
    '8 a word belongs to the paint it was said with: the kit\'s desk says veneer in its own wood paints and nothing as a steel bench or a white table (the paint\'s name speaks: the bench is steel); a rug is cloth in any colour; no door frame says a word; in a look\'s file no box the skin repainted keeps the plain file\'s word, and the skin drops it where it repaints; the editor\'s paint drops it',
    `desk ${own[0]?.stuff}, dark ${dark[0]?.stuff}, as a steel bench ${steel[0]?.stuff ?? 'no word'} (drawn ${said(cls(steel[0]))}), white ${white[0]?.stuff ?? 'no word'} · rug ${rug[0]?.stuff} · ${frames} door frame boxes with a word · ${twins} look files with ${twinWords} words, ${kept} on a repainted box · the editor drops it: ${dropped}`); }

console.log(`=== stuff: ${red ? 'FAILURES above' : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
