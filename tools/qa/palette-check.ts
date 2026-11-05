// A LEVEL IN OTHER CLOTHES — the check behind the three things that let a level wear a palette that is not the kit's own:
//   · the kit is handed its paints by its host (src/scene/gwotKit.ts GwotHost.paints; src/scene/gwotPalette.ts paintsOf), so a piece is written once;
//   · a look file's skin is handed the same (tools/levels/gwotSkin.ts Skin.of);
//   · a file keeps only the rows of such a palette that something in it wears (tools/levels/export.ts PALETTE; tools/levels/wornRows.ts).
// No level wears a second palette today and no palette of one is in the build: the check makes a small one of its own, with names nothing else has.
//   bun run tools/qa/palette-check.ts          exit 0 = every line holds · 1 = one does not
//   PALETTE_BREAK=1   the second paints say nothing of what they do (each is laid as itself):          line 1 must FAIL
//   PALETTE_BREAK=2   the skin's second arm is not handed the second paints (it lays the kit's own):   line 2 must FAIL
//   PALETTE_BREAK=3   the exporter's helper keeps every row:                                           line 3 must FAIL
// The same on both stands: nothing here is a rule of play, and no game is stood up. Never seen on a GPU: this asks what is laid, not how it looks.
import { readdirSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import type { Vec3 } from '../../src/math/vec';
import { BoxFlag, type Box } from '../../src/scene/boxes';
import { GWOT_PALETTE, paintsOf, type GwotMat, type KitPaints } from '../../src/scene/gwotPalette';
import { sayPaint, type BoxStuff } from '../../src/scene/stuff';
import { officeKit } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice } from '../../src/scene/gwotKit';
import { watchPieces } from '../../src/scene/library';
import { findCarLamps } from '../../src/game/carlamps';
import { parseLevelData, type LevelData } from '../../src/scene/levelData';
import { loadLevel } from '../../src/scene/levelLoad';
import { skinLevel } from '../levels/gwotSkin';
import { dropUnworn } from '../levels/wornRows';
import { lines } from './kit-stand';

const BREAK = process.env.PALETTE_BREAK ?? '';
console.log(`\n=== palette: a level in other clothes${BREAK ? ` (PALETTE_BREAK=${BREAK})` : ''} ===`);
const { check, failed } = lines();
const key = (a: readonly number[]) => a.join(',');

// THE SECOND PALETTE OF THIS CHECK: five rows nothing else names, and every row of the kit wearing one of them (by the row's place in the kit's list)
const ROWS: Record<string, Vec3> = { 'check chalk': [0.61, 0.6, 0.57], 'check clay': [0.42, 0.2, 0.13], 'check ink': [0.05, 0.09, 0.22], 'check zinc': [0.4, 0.41, 0.42], 'check reed': [0.5, 0.44, 0.3] };
const FINISH: Record<string, { rough: number; metal?: boolean }> = { 'check chalk': { rough: 0.9 }, 'check clay': { rough: 0.8 }, 'check ink': { rough: 0.5 }, 'check zinc': { rough: 0.4, metal: true }, 'check reed': { rough: 0.95 } };
const WORDS: Record<string, BoxStuff> = { 'check chalk': 'paint', 'check clay': 'paint', 'check ink': 'paint', 'check zinc': 'steel', 'check reed': 'fabric' };
const KIT = Object.keys(GWOT_PALETTE) as GwotMat[], NAMES = Object.keys(ROWS);
const OF = Object.fromEntries(KIT.map((m, i) => [m, NAMES[i % NAMES.length]])) as Record<GwotMat, string>;
const SECOND: KitPaints = paintsOf(ROWS, FINISH, OF, WORDS);
if (BREAK === '1') for (const m of KIT) sayPaint(SECOND.paints[m], OF[m], { stuff: WORDS[OF[m]], ...FINISH[OF[m]] });   // (the red path: laid as itself, doing nothing said)
const kitKeys = new Set(KIT.map(m => key(GWOT_PALETTE[m])));

// 1. THE KIT OVER A HOST THAT HANDS IT PAINTS: EVERY CALL the two showrooms make of a piece of the kit (the library's rule is that every piece stands in
//    one: tools/levels/library.ts), laid alone on a stand in the kit's palette and again over the second paints. So the piece added tomorrow is asked
const calls: { name: string; args: unknown[] }[] = [], kitPieces = new Set<string>();
{ let depth = 0; watchPieces({ kit(kit, names) { if (kit === 'gwotKit') for (const n of names) kitPieces.add(n); }, open(kit, name, args) { if (depth++ === 0 && kit === 'gwotKit') calls.push({ name, args: [...args] }); }, close() { depth--; } });
  try { for (const g of ['library-hall', 'gwot-yard']) (await import(`../levels/${g}.ts`)).build(); } finally { watchPieces(null); } }
const layOne = (c: { name: string; args: unknown[] }, paints?: KitPaints) => { const K = officeKit({ groups: [] }), G = gwotKit({ ...gwotOfOffice(K), ...(paints ? { paints } : {}) }) as unknown as Record<string, (...a: unknown[]) => unknown>;
  K.grp('stand'); G[c.name](...c.args); return { boxes: K.boxes as Box[], props: K.props }; };
const shapeOf = (b: Box) => JSON.stringify(b.shape ? { ...b.shape, with: b.shape.with?.map(t => ({ off: t.off, h: t.h, round: t.round, arc: t.arc, cut: t.cut })) } : null);
{ const moved: string[] = [], does: string[] = [], left: string[] = [], threw: string[] = [], lamps: string[] = [], names = new Set<string>(), words = new Set<string>(); let asked = 0, leaned = 0, nBoxes = 0, said = 0, lenses = 0;
  for (const c of calls) { let P: ReturnType<typeof layOne>;
    try { P = layOne(c); } catch { leaned++; continue; }   // (a call that takes a thing another call made does not lay alone: not asked, counted)
    let S: ReturnType<typeof layOne>; try { S = layOne(c, SECOND); } catch (e) { threw.push(`${c.name}: ${String((e as Error).message).split('\n')[0].slice(0, 90)}`); continue; }
    asked++; names.add(c.name); nBoxes += S.boxes.length;
    const lp = findCarLamps(P.boxes), ls = findCarLamps(S.boxes), lens = new Set(ls.map(l => l.box)); lenses += ls.length;
    if (lp.length !== ls.length) lamps.push(`${c.name}: ${lp.length} lamps the game finds in the kit's palette, ${ls.length} over the second paints`);
    if (P.boxes.length !== S.boxes.length) { moved.push(`${c.name}: ${P.boxes.length} boxes, then ${S.boxes.length}`); continue; }
    for (let i = 0; i < P.boxes.length; i++) { const a = P.boxes[i], b = S.boxes[i];
      if (key(a.c) !== key(b.c) || key(a.h) !== key(b.h) || a.yaw !== b.yaw || a.flags !== b.flags || shapeOf(a) !== shapeOf(b)) moved.push(`${c.name} box ${i}`);
      if (a.does !== b.does) does.push(`${c.name} box ${i} did ${a.does ?? 'nothing said'}, does ${b.does ?? 'nothing said'}`); if (b.does !== undefined) { said++; words.add(b.does); }
      if (lens.has(i)) continue;   // (a lens is laid in the lamp's own paint in any clothes: the game finds it by that)
      if (kitKeys.has(key(b.albedo))) left.push(`${c.name} box ${i}`); for (const t of b.shape?.with ?? []) if (t.albedo && kitKeys.has(key(t.albedo))) left.push(`a trim of ${c.name} box ${i}`); }
    if (P.props.length !== S.props.length) moved.push(`${c.name}: ${P.props.length} loose things, then ${S.props.length}`);
    else P.props.forEach((p, k) => p.parts.forEach((part, j) => { const q = S.props[k].parts[j] as typeof part | undefined; if (!q) { moved.push(`${c.name} loose thing ${k} part ${j}`); return; }
      if ((part as { does?: string }).does !== (q as { does?: string }).does) does.push(`${c.name} loose thing ${k} (${p.kind}) part ${j}`); if (kitKeys.has(key(q.albedo))) left.push(`a part of ${c.name}'s loose thing ${k}`); })); }
  const unasked = [...kitPieces].filter(n => !names.has(n));   // (a piece of the kit no showroom call laid alone both ways)
  check(!moved.length && !does.length && !left.length && !threw.length && !lamps.length && !unasked.length && kitPieces.size > 60 && said > 500 && words.size >= 6 && lenses >= 8,
    `1 the kit over a host that hands it paints: every call the showrooms make of a piece lays, box for box and part for part as in the kit's own palette; every box and every loose thing's part DOES what it did; the game finds the same lamps; and but for a lamp's lens nothing is left in a row of the kit's`,
    `${asked} calls of ${names.size} of the kit's ${kitPieces.size} pieces laid alone both ways${unasked.length ? ` · NOT ASKED: ${unasked.join(', ')}` : ''} (${leaned} that take another call's thing not asked), ${nBoxes} boxes · ${said} boxes say what they do, in ${words.size} words · ${lenses} lenses found over the second paints${threw.length ? ` · DO NOT LAY OVER THE SECOND PAINTS: ${threw.slice(0, 4).join(' · ')}` : ''}${moved.length ? ` · MOVED: ${moved.slice(0, 5).join(', ')}` : ''}${does.length ? ` · ${does.length} DO ANOTHER THING: ${does.slice(0, 4).join(' · ')}` : ''}${lamps.length ? ` · LAMPS: ${lamps.slice(0, 3).join(' · ')}` : ''}${left.length ? ` · ${left.length} LEFT IN THE KIT'S PAINT: ${left.slice(0, 6).join(', ')}` : ''}`); }

// 2. A LOOK FILE'S SKIN IN THE SECOND PAINTS: the skin's shared rules over every plain level that has a look file, once in the kit's palette, once in the second
{ const LV = resolve(import.meta.dir, '../../levels'), plains = readdirSync(LV).filter(f => f.endsWith('-gwot.json')).map(f => f.replace(/-gwot\.json$/, '')).sort();
  const open = (name: string) => { const data = parseLevelData(readFileSync(resolve(LV, `${name}.json`), 'utf8')) as LevelData; return { level: loadLevel(data, { lived: true, dressing: true } as never), n: 0 }; };
  const bad: string[] = []; let asked = 0, repainted = 0, decals = 0, laidInKit = 0;
  for (const name of plains) { const A = open(name), B = open(name), O = open(name), n0 = A.level.boxes.length;   // (O: the plain level, untouched)
    const groups = (parseLevelData(readFileSync(resolve(LV, `${name}.json`), 'utf8')) as LevelData).boxes.map(b => (b as { group?: string }).group ?? '');
    if (groups.length !== n0) { bad.push(`${name}: ${groups.length} groups for ${n0} boxes (the file's lived or dressed boxes are not its own list): not asked`); continue; }
    const sa = skinLevel(A.level, [...groups], {}), sb = skinLevel(B.level, [...groups], BREAK === '2' ? {} : { of: SECOND });
    if (sa.repainted !== sb.repainted || sa.kept !== sb.kept) bad.push(`${name}: ${sa.repainted} repainted and ${sa.kept} kept in the kit's palette, ${sb.repainted} and ${sb.kept} in the second`);
    for (let i = 0; i < n0; i++) { const a = A.level.boxes[i], b = B.level.boxes[i]; asked++;
      if (key(a.c) !== key(b.c) || key(a.h) !== key(b.h) || a.flags !== b.flags) bad.push(`${name} box ${i} moved`);
      if (a.does !== b.does) bad.push(`${name} box ${i} did ${a.does}, does ${b.does}`);
      const was = key(O.level.boxes[i].albedo), painted = key(a.albedo) !== was;   // (the first skin repainted it)
      if (painted !== (key(b.albedo) !== was)) bad.push(`${name} box ${i}: ${painted ? 'the first skin repaints it and the second does not' : 'the second skin repaints it and the first does not'}`);
      if (painted && kitKeys.has(key(b.albedo))) bad.push(`${name} box ${i} is in the kit's paint under the second skin`);
      if (!painted && kitKeys.has(was)) laidInKit++; }
    for (let i = n0; i < B.level.boxes.length; i++) { decals++; if (kitKeys.has(key(B.level.boxes[i].albedo))) bad.push(`${name}: a decal of the second skin (box ${i}) is in a row of the kit's`); if (!(B.level.boxes[i].flags & BoxFlag.NoShadow)) bad.push(`${name}: the second skin added a box that is no decal`); }
    A.level.props.forEach((p, k) => p.parts.forEach((part, j) => { const q = B.level.props[k]?.parts[j], was = O.level.props[k]?.parts[j]; if (!q || !was || (part as { does?: string }).does !== (q as { does?: string }).does) bad.push(`${name} loose thing ${k} part ${j}`); else if (key(part.albedo) !== key(was.albedo) && kitKeys.has(key(q.albedo))) bad.push(`${name}: a part of loose thing ${k} is in the kit's paint under the second skin`); }));
    repainted += sb.repainted; }
  check(!bad.length && plains.length >= 10 && repainted > 2000, `2 a look file's skin handed the second paints repaints the boxes the first skin repaints and no others, every box and part does what it did, and nothing it paints or lays is left in a row of the kit's`,
    `${plains.length} plain levels that have a look file, ${asked} boxes asked, ${repainted} repainted and ${decals} decals laid in the second paints · LEFT AS LAID, and so in the kit's paint under any skin: ${laidInKit} boxes a plain file's own generator laid with the kit (a level that wears a second palette lays those over a host that hands the kit its paints: line 1)${bad.length ? ` · ${bad.length} NOT SO: ${bad.slice(0, 6).join(' · ')}` : ''}`); }

// 3. OF A PALETTE A FILE CARRIES ONLY THE ROWS IT WEARS
{ const row = (a: Vec3) => ({ albedo: a }), pal = { red: row([0.5, 0.1, 0.1]), green: row([0.1, 0.5, 0.1]), blue: row([0.1, 0.1, 0.5]), gold: row([0.5, 0.4, 0.1]), grey: row([0.4, 0.4, 0.4]) };
  const data = { materials: { wall: row([0.6, 0.6, 0.6]), ...pal, grey2: row([0.3, 0.3, 0.3]) },
    boxes: [{ id: 'a', c: [0, 0, 0], h: [1, 1, 1], material: 'red' }, { id: 'b', c: [3, 0, 0], h: [1, 1, 1], albedo: [0.2, 0.2, 0.2], shape: { with: [{ off: [0, 0, 0], h: [0.1, 0.1, 0.1], material: 'green' }] } }],
    props: [{ id: 'p', kind: 'crate', x: 0, z: 0, parts: [{ off: [0, 0, 0], h: [0.2, 0.2, 0.2], material: 'blue' }] }],
    scenarios: { kitView: { ways: [{ add: { boxes: [{ c: [0, 0, 0], h: [1, 1, 1], material: 'wall|gold' }] } }] } } } as unknown as LevelData;
  // (`grey2`: the file's own material in other numbers than the palette's row of that name, worn by nothing: it is not the palette's, and stays)
  const out = dropUnworn(data, { ...pal, grey2: row([0.9, 0.9, 0.9]) }), kept = Object.keys(data.materials);
  check(out.join() === 'grey' && kept.join() === 'wall,red,green,blue,gold,grey2', `3 of a palette a file keeps the rows a box, a shape's piece, a prop's part or its scenarios' own boxes wear, drops the row nothing wears, and never touches a material that is no row of the palette, by name or by a shared name in other numbers`,
    `dropped: ${out.join(', ') || 'none'} · kept: ${kept.join(', ')}`); }

console.log(failed() ? `=== palette: ${failed()} FAILED ===` : '=== palette: ALL PASS ===');
process.exit(failed() ? 1 : 0);
