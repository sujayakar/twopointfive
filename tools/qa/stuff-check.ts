// A THING IS MADE OF WHAT IT SAYS (src/scene/stuff.ts; src/scene/levelLoad.ts; src/render/gbuffer.ts lookWordOf).
//   bun run tools/qa/stuff-check.ts            exit 0 = every line passes
//   STUFF_BREAK=1 bun run tools/qa/stuff-check.ts     every level file is read with its material rows' words struck out (a file from before 11-04, loaded
//                                                    as it stood): lines 2, 3, 4, 7, 13 and 14 must FAIL
// No game is stood up and no picture is looked at: what a box is drawn as is decided on the CPU (the loader puts a word on the box, lookWordOf reads it) and
// this asks it. What a material LOOKS like on a desk is a page's still and then a GPU's. The same on both stands: nothing here is a rule of play and no
// stand's word reaches it. What every box WAS drawn as before the colour rule went, against what it is now, is tools/qa/stuff-audit.ts.
import { readFileSync, readdirSync } from 'node:fs';
import { ROOT } from './headless.ts';
import { fromBefore, paintUsers, rowOf } from '../levels/paintUsers.ts';
const ST = await import(`${ROOT}/src/scene/stuff.ts`), { PAINT_STUFF, stuffOfName, rowWord, BOX_STUFFS, saidFor, namePaints, paintOf, toned, tonedAs, madeOf, drawnAs, wordFor, worded } = ST;
const { LOOK_NAMES, LOOK_PAINTED, lookStuffOf } = await import(`${ROOT}/src/render/lookTex.ts`);
const GB = await import(`${ROOT}/src/render/gbuffer.ts`), { lookWordOf, lookCoreOf, LOOK_WORD } = GB;
const { GWOT_PALETTE, gwotAs } = await import(`${ROOT}/src/scene/gwotPalette.ts`);
const { parseLevelData, serializeLevelData, validateLevelData, KEYS } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel, levelToData } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { lookPageWord } = await import(`${ROOT}/src/scene/look.ts`);
const { makeBox, wordOf } = await import(`${ROOT}/src/scene/boxes.ts`);
const { PropSystem } = await import(`${ROOT}/src/game/props.ts`), { StaticCollision } = await import(`${ROOT}/src/scene/collision.ts`);
const KD = await import(`${ROOT}/src/scene/levelKit.ts`), { newKit, KIT_PALETTE, KIT_FINISH } = KD;
const { Editor } = await import(`${ROOT}/src/editor/editor.ts`), { boxPaint } = await import(`${ROOT}/src/editor/paint.ts`);

const BRK = process.env.STUFF_BREAK === '1';
let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== stuff: a thing is made of what it says${BRK ? ' (STUFF_BREAK: the material rows\' words are struck out)' : ''} ===`);
const FILES = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).sort();
/** a level file as the check reads it: with STUFF_BREAK its rows say nothing, as a file from before did */
const dataOf = (f: string): any => { const d = parseLevelData(readFileSync(`${ROOT}/levels/${f}`, 'utf8')) as any; if (BRK) for (const m of Object.values(d.materials) as any[]) delete m.stuff; return d; };
const word = lookWordOf(), cls = (b: any, moves = false) => word(b, moves) & (LOOK_WORD.stuffs - 1);
const said = (c: number) => c === 0 ? 'plain' : LOOK_NAMES[c - 1];
const plainIs = (w: string | undefined) => w === undefined || w === 'plain' ? undefined : w;

// 1. every name a level file writes has a row in the table, and every row says a stuff a box can be
{ const none = new Map<string, string>(), bad: string[] = [], seen = new Set<string>(), kinds: Record<string, number> = { material: 0, plain: 0, waits: 0 };
  for (const f of FILES) for (const name of Object.keys(dataOf(f).materials)) { seen.add(name); if (stuffOfName(name) === undefined && !none.has(name)) none.set(name, f); }
  for (const [name, stuff] of Object.entries(PAINT_STUFF) as [string, string][]) { if (stuff === 'plain' || stuff === 'waits') kinds[stuff]++; else if (lookStuffOf(stuff) > 0) kinds.material++; else bad.push(`${name}: '${stuff}'`); }
  check(none.size === 0 && bad.length === 0 && seen.size > 60 && stuffOfName('constructor') === undefined && stuffOfName('toString') === undefined && !(Object.values(PAINT_STUFF) as string[]).includes('mixed'),
    '1 every paint name in every level file has a row of its own in the table, and every row says a material the look draws, or plain, or waits (no name is left to its colour); a name that is a method of every object has no row',
    `${seen.size} names in ${FILES.length} level files · ${Object.keys(PAINT_STUFF).length} rows: ${kinds.material} a material, ${kinds.plain} plain, ${kinds.waits} waiting${none.size ? ` · NO ROW: ${[...none].map(([n, f]) => `${n} (${f})`).join(', ')}` : ''}${bad.length ? ` · not a material: ${bad.join(', ')}` : ''}`); }

// 2. THE FILES SAY IT: every material row of every level file says what it is made of, and a row whose name the table knows says the table's word
{ let rows = 0, silent = 0, other = 0, old = 0; const eg: string[] = [];
  for (const f of FILES) { const d = dataOf(f); if (fromBefore(d)) old++;
    for (const [name, m] of Object.entries(d.materials) as [string, any][]) { rows++; if (m.stuff === undefined) { silent++; if (eg.length < 4) eg.push(`${f} ${name} says nothing`); } else if (rowWord(name).stuff !== undefined && m.stuff !== rowWord(name).stuff) { other++; if (eg.length < 4) eg.push(`${f} ${name} says ${m.stuff}, the table ${rowWord(name).stuff}`); } } }
  check(old === 0 && silent === 0 && other === 0 && rows > 3000, '2 every level file says it itself: each material row says what the paint is made of (plain and waits written out), as the table of names has it; no file is one from before the colour rule went (tools/levels/stuff-bake.ts brings one forward)',
    `${rows} rows in ${FILES.length} files · ${silent} say nothing · ${other} say another word than their name's · ${old} files from before${eg.length ? ` · ${eg.join(' · ')}` : ''}`); }

// 3. THE LOADER PUTS IT ON THE BOX and the look reads the box and nothing else: on every file, every box and prop's part is its own word, else its row's
{ let n = 0, off = 0, drawn = 0, withWord = 0, own = 0; const eg: string[] = [];
  for (const f of FILES) { const d = dataOf(f), L = loadLevel(d, { lived: true, dressing: true }) as any;
    const want = new Map<string, string | undefined>(); for (const { o, where, kind } of paintUsers(d)) { if (kind === 'set') continue; const m = rowOf(d, o); want.set(where, plainIs(o.stuff ?? (m !== undefined ? d.materials[m].stuff : undefined))); if (o.stuff !== undefined) own++; }
    L.boxes.forEach((b: any, i: number) => { n++; const w = want.get(`boxes[${i}]`); if (b.stuff !== undefined) withWord++; if (b.stuff !== w) { off++; if (eg.length < 4) eg.push(`${f} box ${i}: ${b.stuff ?? 'nothing'}, the file ${w ?? 'nothing'}`); } if (cls(b) !== lookStuffOf(w)) drawn++; });
    L.props.forEach((p: any, i: number) => p.parts.forEach((q: any, j: number) => { n++; const w = want.get(`props[${i}].parts[${j}]`); if (q.stuff !== undefined) withWord++; if (q.stuff !== w) { off++; if (eg.length < 4) eg.push(`${f} prop ${i} part ${j}: ${q.stuff ?? 'nothing'}, the file ${w ?? 'nothing'}`); } })); }
  check(off === 0 && drawn === 0 && n > 30000 && withWord > n * 0.6 && own > 3000, '3 the loader puts the word on the box: on every level file each box and each prop\'s part carries its own word, else its material row\'s, and nothing where that is plain; the look\'s word for a box is that stuff\'s number; most boxes have a material, and thousands say their own (the paints written as numbers)',
    `${n} boxes and parts in ${FILES.length} files · ${withWord} with a material · ${own} say their own in the file · ${off} not what the file says · ${drawn} drawn as something else${eg.length ? ` · ${eg.join(' · ')}` : ''}`); }

// 4. the office's own paints, by name, as the game loads them; and what the office's own surfaces say over them (a place somebody keeps: officeGen.ts)
{ const d = dataOf('office.json'), L = loadLevel(d, { lived: true, dressing: true }) as any, at = (name: string, own = false) => L.boxes[d.boxes.findIndex((b: any) => b.material === name && (b.stuff !== undefined) === own)];
  const want: Record<string, string> = { wood: 'wood', woodDark: 'wood', carpet: 'cloth', carpetBlue: 'cloth', wall: 'plaster', concrete: 'block', lino: 'slab', stone: 'slab', metal: 'steel', black: 'plain', cardboard: 'plain', asphalt: 'plain' };
  // (a name whose every box in the office says its own word has no box to ask: its ROW is asked, which is what any other box of that paint would be)
  const got = Object.keys(want).map(n => { const b = at(n); return b ? said(cls(b)) : d.materials[n] ? said(lookStuffOf(d.materials[n].stuff)) : 'no such row'; });
  const kept: Record<string, string> = { carpet: 'carpet', carpetBlue: 'carpet', carpetRed: 'carpet', wall: 'skim', wallDark: 'skim', concrete: 'cast panel', lino: 'enamel', stone: 'paver', red: 'enamel' };
  const own = Object.keys(kept).map(n => { const b = at(n, true); return b ? said(cls(b)) : 'no such box'; });
  check(Object.keys(want).every((n, i) => got[i] === want[n]) && Object.keys(kept).every((n, i) => own[i] === kept[n]), '4 the office\'s paints: a bench is wood, a carpet cloth, steel shelving steel, a partition wall plaster, the corridor slab; a bezel is plain because its name says so, card plain because it waits for a material; the car park plain because no stuff the look has is tarmac. And the office\'s own floors and walls say what a kept place is made of: a loop carpet, skimmed partitions, a shell cast in panels, a waxed sheet for lino, stone laid in tiles, red enamel',
    `${Object.keys(want).map((n, i) => `${n} ${got[i]}`).join(' · ')} · its own: ${Object.keys(kept).map((n, i) => `${n} ${own[i]}`).join(' · ')}`); }

// 5. NOTHING ASKS A COLOUR: the same numbers are whatever their box says; a box that says nothing is plain whatever its colour; rain, a tone, a repaint change nothing
{ const mud = [...GWOT_PALETTE.mud], a = makeBox({ c: [3, 1, 4], h: [0.5, 0.5, 0.5], albedo: mud }), b = makeBox({ c: [3, 1, 4], h: [0.5, 0.5, 0.5], albedo: mud, stuff: 'wood' }), c = makeBox({ c: [3, 1, 4], h: [0.5, 0.5, 0.5], albedo: mud, stuff: 'mud wall' });
  const before = cls(c); c.albedo = [mud[0] * 0.5, mud[1] * 0.5, mud[2] * 0.52]; const wet = cls(c); c.albedo = [0.9, 0.1, 0.9]; const pink = cls(c);
  const srcs = ['src/render/gbuffer.ts', 'src/scene/levelKit.ts', 'src/scene/officeGen.ts', 'src/scene/levelLoad.ts', 'src/main.ts'].map(f => readFileSync(`${ROOT}/${f}`, 'utf8')).join('\n');
  const wgsl = readdirSync(`${ROOT}/src/render/shaders`).filter(f => f.endsWith('.wgsl')).map(f => readFileSync(`${ROOT}/src/render/shaders/${f}`, 'utf8')).join('\n');
  const gone = ['lookStuff', 'lookNamed', 'lookNeedPaints', 'classWgsl', 'LOOK_OF', 'LOOK_BY_NAME', 'LOOK_MATCH', 'FINISH_BY_ALBEDO', 'surfTryConfig'].filter(n => new RegExp(`\\b${n}\\b`).test(srcs) || (GB as any)[n] !== undefined);
  const shader = ['lookClass', 'surfClass', 'surfaceTry'].filter(n => new RegExp(`\\b${n}\\s*\\(`).test(wgsl)).concat(/\bSURF_TRY\b/.test(wgsl) ? ['SURF_TRY'] : []);
  const noPalette = !/gwotPalette/.test(readFileSync(`${ROOT}/src/render/gbuffer.ts`, 'utf8'));
  check(cls(a) === 0 && cls(b) === lookStuffOf('wood') && before === lookStuffOf('mud wall') && wet === before && pink === before && gone.length === 0 && shader.length === 0 && noPalette && (ST as any).BY_COLOUR === undefined,
    '5 nothing asks a colour what it is made of: a box in the mud wall\'s exact paint that says nothing is plain, one that says wood is wood; a box that says mud wall is still that soaked dark by rain and repainted pink; the renderer\'s colour match, the name-by-colour table, the kits\' finish-by-colour table and the surfaces try\'s classifier are gone from the code and from every shader, and the renderer does not import the palette',
    `mud's paint, no word: ${said(cls(a))} · saying wood: ${said(cls(b))} · mud wall dry ${said(before)}, wet ${said(wet)}, pink ${said(pink)} · still in the code: ${gone.join(', ') || 'none'} · in a shader: ${shader.join(', ') || 'none'}`); }

// 6. the page: the look's word is made from the boxes alone, and the two words that gave the colour rule back are no longer the look page's
{ const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8') + readFileSync(`${ROOT}/src/engine.ts`, 'utf8'), wired = main.includes('this.world.setStuffOf(lookWordOf(), lookCoreOf())') && !/setStuffOf\(lookWordOf\(\)\)/.test(main);
  const core = lookCoreOf(), cut = makeBox({ c: [0, 1, 0], h: [1, 1, 0.2], albedo: [0.4, 0.4, 0.4], stuff: 'block' }); const own = core(cut); cut.cutCoreStuff = 'slab'; const other = core(cut); cut.cutCoreStuff = 'plain' as any; const none = core(cut);
  check(wired && !lookPageWord('names', '0') && !lookPageWord('words', '0') && !/q\.get\('names'\)|q\.get\('words'\)|q\.has\('surf'\)/.test(main) && own === lookStuffOf('block') && other === lookStuffOf('slab') && none === 0,
    '6 the page makes the look\'s word from what each box says, with no rows and no paint handed over; `?names=0`, `?words=0` and `?surf` are read nowhere; an old break\'s cut faces are the stuff of its box, or of its core where the box says one',
    `wired ${wired} · look page words: names ${lookPageWord('names', '0')}, words ${lookPageWord('words', '0')} · an old cut: ${said(own)}, with a core of slab ${said(other)}, of plain ${said(none)}`); }

// 7. a box says its own word: over its row's, through the level file and back, on a loose prop's part too; a row may say one; what cannot be said is refused
{ const D = dataOf('library-hall.json') as any, bi = D.boxes.findIndex((b: any) => b.material === 'concrete' && b.stuff === undefined), wi = D.boxes.findIndex((b: any) => b.material === 'wood' && b.flags === undefined && b.stuff === undefined), fi = D.boxes.findIndex((b: any) => b.material !== undefined && b.flags !== undefined && b.stuff === undefined), pi = D.props.findIndex((p: any) => p.kind === 'chair'), ii = D.boxes.findIndex((b: any) => b.albedo !== undefined && b.stuff === undefined);
  if (bi < 0 || wi < 0 || fi < 0 || pi < 0 || ii < 0) check(false, '7 a box says its own word', `the showroom has no concrete box, wooden box, flagged box, inline box or chair to ask (${bi}, ${wi}, ${fi}, ${ii}, ${pi})`);
  else {
    D.boxes[bi].stuff = 'paint'; D.boxes[wi].stuff = 'plain'; D.boxes[fi].stuff = 'slab'; D.props[pi].parts[0].stuff = 'plaster'; D.boxes[ii].stuff = 'enamel';
    const text = serializeLevelData(D), back = parseLevelData(text) as any, errs = validateLevelData(back), L = loadLevel(back, { lived: true, dressing: true }) as any;   // (every box of the file: the lived rooms and the dressing too)
    const car = cls(L.boxes[bi]), desk = cls(L.boxes[wi]), inl = cls(L.boxes[ii]);
    // the box the GAME makes of a prop's part (game/props.ts Prop), which is also the box the page asks before the first frame
    const seat = cls(new PropSystem([L.props[pi]], new StaticCollision(L.boxes), () => {}).all[0].boxes[0], true);
    // back to a file: a word is written only where the box is another thing than its row says; 'plain' is written over a row that says something
    const again = levelToData(L, { materials: back.materials }) as any, same = again.boxes.every((b: any, i: number) => b.stuff === back.boxes[i].stuff) && again.props.every((p: any, i: number) => p.parts.every((q: any, j: number) => q.stuff === back.props[i].parts[j].stuff));
    const kept = again.boxes[bi].stuff === 'paint' && again.boxes[wi].stuff === 'plain' && again.props[pi].parts[0].stuff === 'plaster' && again.boxes[ii].stuff === 'enamel' && again.boxes.some((b: any) => b.stuff === undefined && b.material !== undefined && back.materials[b.material].stuff !== 'plain');
    // where it is written: after the finish, before the flags, on a box that has flags; and in the format's own key lists, a material row's too
    const line = text.split('\n').find((l: string) => l.includes(`"id": "${D.boxes[fi].id}"`) || l.includes(`"id":"${D.boxes[fi].id}"`)) ?? '', kb = KEYS.box.indexOf('stuff'), kp = KEYS.part.indexOf('stuff'), km = KEYS.material.indexOf('stuff');
    const placed = /"stuff":\s*"slab".*"flags"/.test(line) && !/"stuff".*"(material|albedo|rough|metal)"/.test(line) && KEYS.box[kb - 1] === 'metal' && KEYS.box[kb + 1] === 'flags' && KEYS.part[kp - 1] === 'albedo' && KEYS.material[km - 1] === 'metal' && KEYS.material[km + 1] === 'notes';
    // refused: on a box, on a prop's part, on a material row, and on a box of a gear screen's set (a picture's box is the format's own record)
    const KV = FILES.find(f => Object.values((dataOf(f) as any).scenarios?.kitView?.sets ?? {}).some((x: any) => x.boxes?.length));
    const refuse = (w: string, where: 'box' | 'part' | 'set' | 'row' = 'box') => { const X = dataOf(where === 'set' ? KV! : 'library-hall.json') as any;
      if (where === 'box') X.boxes[0].stuff = w; else if (where === 'part') X.props[0].parts[0].stuff = w; else if (where === 'row') X.materials.wood.stuff = w; else (Object.values(X.scenarios.kitView.sets).find((x: any) => x.boxes?.length) as any).boxes[0].stuff = w;
      return validateLevelData(X).some((e: string) => e.includes('stuff')); };
    const masonry = (LOOK_NAMES as readonly string[]).slice(LOOK_PAINTED).join(', ');
    check(errs.length === 0 && car === lookStuffOf('paint') && desk === 0 && inl === lookStuffOf('enamel') && seat === lookStuffOf('plaster') && same && kept && placed
      && refuse('mixed') && refuse('banana') && !refuse('waits') && !refuse('veneer') && refuse('banana', 'part') && !refuse('fabric', 'part') && refuse('banana', 'row') && !refuse('veneer', 'row') && refuse('mud brick', 'row') && !!KV && refuse('banana', 'set') && !refuse('steel', 'set')
      && BOX_STUFFS.length === LOOK_PAINTED + 4 && !refuse('glass') && !refuse('rubber') && masonry === 'field stone, mud brick' && refuse('mud brick') && refuse('field stone') && lookStuffOf('mud brick') === 0,
      '7 a box says its own word: a concrete box that says paint is paint; a wooden box that says plain is plain; a box written as numbers that says enamel is enamel; a chair\'s part says its own and the box the game makes of it (the box the page asks before the first frame) carries it; the words go through the level file and back, written only where the box is another thing than its row says, after the finish and before the flags; a word the look has no material for is refused when the file is read, on a box, a part, a material row and a gear screen\'s box, the stone and brick under a mud wall among them (glass and rubber, which the picture of a hit reads, are words a box may say)',
      `the concrete box ${said(car)} · the wooden box ${said(desk)} · the inline box ${said(inl)} · the chair's part ${said(seat)} · through the file and back the same ${same}, kept ${kept} · placed ${placed} · refused: mixed ${refuse('mixed')}, banana ${refuse('banana')}, mud brick ${refuse('mud brick')}, a part's ${refuse('banana', 'part')}, a row's ${refuse('banana', 'row')}, a set's ${KV ? refuse('banana', 'set') : 'no file'} (${KV}); waits ${refuse('waits')} · ${errs.length} errors reading it back${errs.length ? `: ${errs.slice(0, 2).join(' | ')}` : ''}`); } }

// 8. a word belongs to the paint it was said with: a piece says it for its own paints, a kit's add gives a box its paint's, and whoever repaints a box does not keep the old
{ const k = newKit(), n0 = k.boxes.length; KD.desk(k, 2, 2, 1.6, 0.8); const own = k.boxes.slice(n0), n1 = k.boxes.length; KD.desk(k, 6, 2, 1.6, 0.8, 0, KIT_PALETTE.woodDark); const dark = k.boxes.slice(n1), n2 = k.boxes.length;
  KD.desk(k, 2, 6, 1.6, 0.8, 0, KIT_PALETTE.metal); const steel = k.boxes.slice(n2), n3 = k.boxes.length; KD.desk(k, 6, 6, 1.6, 0.8, 0, KIT_PALETTE.white); const white = k.boxes.slice(n3), n4 = k.boxes.length;
  KD.rug(k, 4, 4, 2, 1, [0.3, 0.1, 0.09]); const rug = k.boxes.slice(n4);
  const says = (l: any[], w?: string) => l.length > 0 && l.every((b: any) => b.stuff === w);
  // the editor: painting a box with another paint drops the box's own word (the new paint's row speaks), in one command that undoes
  const ED = dataOf('library-hall.json') as any, eb = ED.boxes.find((b: any) => b.material === 'wood'); eb.stuff = 'veneer'; const ed = new Editor(ED, {}); boxPaint(ed, [eb.id], 'concrete');
  const painted = ed.data.boxes.find((b: any) => b.id === eb.id), dropped = painted.material === 'concrete' && painted.stuff === undefined;
  const skin = readFileSync(`${ROOT}/tools/levels/gwotSkin.ts`, 'utf8').includes('b.albedo = gwotAs(m, sim); b.rough = GWOT_FINISH[m]?.rough ?? ROUGH_CLOTH; delete b.stuff; worded(b);');
  // a piece broken off a thing is made of what the thing is
  const from = makeBox({ c: [0, 0, 0], h: [1, 1, 1], albedo: [0.2, 0.2, 0.2], stuff: 'veneer' }), copy = makeBox({ c: [0, 0, 0], h: [0.1, 0.1, 0.1], albedo: [0.2, 0.2, 0.2], ...wordOf(from) }), bare = makeBox({ c: [0, 0, 0], h: [0.1, 0.1, 0.1], albedo: [0.2, 0.2, 0.2], ...wordOf(makeBox({ c: [0, 0, 0], h: [1, 1, 1] })) });
  check(says(own, 'veneer') && says(dark, 'veneer') && says(steel, 'steel') && says(white) && says(rug, 'cloth') && JSON.stringify(saidFor('veneer', [0.3, 0.31, 0.33], KIT_PALETTE.wood, KIT_PALETTE.woodDark)) === '{}' && JSON.stringify(saidFor('veneer', [...KIT_PALETTE.wood], KIT_PALETTE.wood, KIT_PALETTE.woodDark)) === '{}' && saidFor('veneer', KIT_PALETTE.wood, KIT_PALETTE.wood).stuff === 'veneer' && dropped && skin && copy.stuff === 'veneer' && bare.stuff === undefined,
    '8 a word belongs to the paint it was said with: the kit\'s desk says veneer in its own wood paints; handed the kit\'s steel it is a steel bench (the paint handed over speaks), handed white laminate it says nothing, and handed a COPY of the wood\'s numbers it says nothing either (the row, not its numbers); a rug is cloth in any colour; the skin drops a repainted box\'s old word and gives it its new paint\'s; the editor\'s paint drops a box\'s own word; a piece broken off a thing copies the thing\'s word',
    `desk ${own[0]?.stuff}, dark ${dark[0]?.stuff}, as a steel bench ${steel[0]?.stuff ?? 'no word'}, white ${white[0]?.stuff ?? 'no word'} · rug ${rug[0]?.stuff} · the skin says the new word: ${skin} · the editor drops it: ${dropped} · a broken piece: ${copy.stuff}`); }

// 9. A PAINT IS KNOWN BY THE ROW HANDED OVER, never by its numbers: a kit's finish and word come with the palette's own array; a copy of its numbers is nobody's
{ const k = newKit(), C = KIT_PALETTE, i0 = KD.add(k, { c: [1, 1, 1], h: [0.2, 0.2, 0.2], albedo: C.metal }), i1 = KD.add(k, { c: [2, 1, 1], h: [0.2, 0.2, 0.2], albedo: [...C.metal] as any }), i2 = KD.add(k, { c: [3, 1, 1], h: [0.2, 0.2, 0.2], albedo: C.metal, rough: 0.9 }), i3 = KD.add(k, { c: [4, 1, 1], h: [0.2, 0.2, 0.2], albedo: C.metal, stuff: 'plain' });
  const B = k.boxes, f = KIT_FINISH.metal;
  const t = toned(C.wood, 0.6), t2 = toned(t, 0.5), far = tonedAs(GWOT_PALETTE.slab, [0.01, 0.01, 0.01]), nobody = toned([0.5, 0.4, 0.3], 0.5);
  const standIn = gwotAs('mud', 'wall'), own = gwotAs('mud', 'concrete'), plastic = gwotAs('plasticWhite', 'wall');
  const lit = madeOf('cloth', [0.3, 0.1, 0.09]), acc = drawnAs('metal', [0.08, 0.08, 0.09], { rough: 0.4 }), two = { a: [0.2, 0.3, 0.4], b: [0.2, 0.3, 0.4] } as Record<string, number[]>; namePaints({ wood: two.a, carpet: two.b });
  const part = worded({ albedo: C.chair as number[] }), partPlain = worded({ albedo: C.chair as number[], stuff: 'plain' as any });
  check(B[i0].rough === f.rough && B[i0].metal === true && B[i0].stuff === 'steel' && B[i1].rough === undefined && B[i1].metal === undefined && B[i1].stuff === undefined && B[i2].rough === 0.9 && B[i2].metal === undefined && B[i2].stuff === 'steel' && B[i3].stuff === undefined && B[i3].rough === f.rough
    && paintOf(C.wood)?.name === 'wood' && paintOf([...C.wood]) === undefined && paintOf(t)?.stuff === 'wood' && paintOf(t)?.rough === undefined && paintOf(t2)?.stuff === 'wood' && paintOf(far)?.stuff === 'slab' && paintOf(nobody) === undefined
    && paintOf(standIn)?.stuff === 'mud wall' && standIn !== GWOT_PALETTE.mud && own === GWOT_PALETTE.mud && paintOf(plastic)?.stuff === 'plastic' && paintOf(lit)?.stuff === 'cloth' && paintOf(acc)?.stuff === 'metal' && paintOf(acc)?.rough === 0.4
    && paintOf(two.a)?.stuff === 'wood' && paintOf(two.b)?.stuff === 'cloth' && part.stuff === 'fabric' && !('stuff' in partPlain) && wordFor(C.black).stuff === undefined && wordFor(C.black, 'veneer').stuff === 'veneer',
    '9 a paint is known by the row handed over: a box made with the kit\'s steel takes the steel\'s finish and is steel; a COPY of the same numbers is nobody\'s paint, matte and plain; a finish or a word said on the call wins (plain said aloud is no word); a paint toned keeps its word and not its finish, however dark; a stand-in is the stuff of the row it stands in for; numbers written out say their word beside them; two paints with the same numbers are two paints',
    `kit steel: rough ${B[i0].rough}, metal ${B[i0].metal}, ${B[i0].stuff} · its copy: rough ${B[i1].rough}, ${B[i1].stuff ?? 'no word'} · wood toned 0.6 and again 0.5: ${paintOf(t)?.stuff}, ${paintOf(t2)?.stuff} · slab toned to near black: ${paintOf(far)?.stuff} · a stand-in of mud: ${paintOf(standIn)?.stuff}, of white plastic: ${paintOf(plastic)?.stuff} · two paints, one colour: ${paintOf(two.a)?.stuff}, ${paintOf(two.b)?.stuff}`); }

// 10. PICTURE ONLY: nothing the simulation makes of anything reads a box's word. In src/game it is read in at most one place, the picture of a hit (that rule joins
//     after this one: until it does, nowhere), and said or copied in three
{ const dir = `${ROOT}/src/game`, lines: string[] = [];
  for (const f of readdirSync(dir).filter(f => f.endsWith('.ts') && f !== 'wallpen.ts')) readFileSync(`${dir}/${f}`, 'utf8').split('\n').forEach((l, i) => { const code = l.replace(/\/\/.*$/, '').replace(/^\s*\*.*$/, ''); if (/\.stuff\b/.test(code)) lines.push(`${f}:${i + 1}:${code}`); });
  const reads = lines.filter(l => !/\bstuff: \w+(\.\w+)*\.stuff\b|\.stuff = \w+(\.\w+)*\.stuff\b|\bstuff: paintOf\(albedo\)\?\.stuff\b|\.stuff = paintOf\(albedo\)\?\.stuff\b|\.stuff = '[\w ]+'/.test(l)), hit = reads.filter(l => l.startsWith('particleFx.ts:') && l.includes('export function saidSurf('));
  const files = [...new Set(lines.map(l => l.split(':')[0]))].sort().join(', ');
  check(reads.length === hit.length && hit.length <= 1 && lines.length >= 4, '10 picture only: in the game\'s code a box\'s word is READ in at most one place, what a round\'s hit looks like (particleFx.ts saidSurf: sparks, chips, the mark), and otherwise only said outright or copied from a thing to what is made of it (a prop\'s part to its box, a wall to its pieces, a named colour to a gun\'s part); no rule of a round, a charge, a blast or a man\'s senses asks it',
    `${lines.length} lines in ${files} · ${reads.length} read it${reads.length > hit.length ? `: ${reads.map(l => l.slice(0, 90)).join(' | ')}` : hit.length ? ' (the hit\'s picture)' : ' (the hit picture\'s rule is not in this build)'}`); }

// 11. A DOOR'S LEAF IS MADE OF WHAT ITS PAINT IS (scene/lookTidy.ts lookDoors): every outside leaf in the level's accent paint is paint and every inside one in
//     the palette's wood is wood, whatever shade the leaf's own name gave it; a level's row of bare numbers says nothing unless it says a word for that side
{ const { lookDoors } = await import(`${ROOT}/src/scene/lookTidy.ts`), { LOOK_DOORS } = await import(`${ROOT}/src/scene/look.ts`);
  const wears = [makeBox({ c: [0, 1, 0], h: [1, 1, 0.1], albedo: GWOT_PALETTE.paintBlue })];
  const leaves = (n: number) => Array.from({ length: n }, (_, i) => ({ def: { name: `door ${i}`, exterior: i % 2 === 0, steel: i === 1 }, box: makeBox({ c: [i, 1, 0], h: [0.45, 1, 0.03], albedo: [0.3, 0.3, 0.3], stuff: 'veneer' }) }));
  const of = (D: ReturnType<typeof leaves>, f: (d: ReturnType<typeof leaves>[number]) => boolean) => [...new Set(D.filter(f).map(d => d.box.stuff ?? 'plain'))].sort().join('/');
  const own = leaves(40), bare = leaves(40), sided = leaves(40), one = leaves(40);
  lookDoors(own, wears); lookDoors(bare, wears, { out: [0.6, 0.42, 0.04] }); lookDoors(sided, wears, LOOK_DOORS['records.json']); lookDoors(one, wears, LOOK_DOORS['white-roof.json']);
  const out = (d: { def: { exterior?: boolean; steel?: boolean } }) => !!d.def.exterior, inn = (d: { def: { exterior?: boolean; steel?: boolean } }) => !d.def.exterior && !d.def.steel;
  const shades = new Set(own.filter(out).map(d => d.box.albedo[2].toFixed(4))).size;
  check(of(own, out) === 'paint' && of(own, inn) === 'plain/wood' && shades > 10 && own[1].box.stuff === 'veneer' && of(bare, out) === 'plain' && of(bare, inn) === 'plain/wood'
    && of(sided, out) === 'plain' && of(sided, inn) === 'plain/wood' && typeof LOOK_DOORS['records.json'].stuff === 'object' && of(one, d => !d.def.steel) === 'enamel',
    '11 a door\'s leaf is made of what its paint is: every outside leaf in the level\'s accent paint is paint whatever its own shade, an inside one in the palette\'s wood is wood and one with old paint faded on it is plain; a steel door is left as it stood; a level\'s row of bare numbers makes its outside leaves plain, a row that says wood for the inside makes those wood, and a row with one word makes every leaf that',
    `no row: outside ${of(own, out)} in ${shades} shades, inside ${of(own, inn)}, the steel door ${own[1].box.stuff} · a row of numbers: outside ${of(bare, out)}, inside ${of(bare, inn)} · the land office's row: outside ${of(sided, out)}, inside ${of(sided, inn)} · the white roof's row: ${of(one, d => !d.def.steel)}`); }

// 12. THE LOOK PAGE'S HAND-BUILT PIECES SAY THEIR WORDS (scene/frontTry.ts, lorryTry.ts, passTry.ts: boxes made at boot, in no level file): every box
//     they make or repaint in a paint somebody named says that paint's stuff or a word of its own, and most of what they make is a material
{ const { dressFront } = await import(`${ROOT}/src/scene/frontTry.ts`), { dressLorry, kitLorry } = await import(`${ROOT}/src/scene/lorryTry.ts`), { dressPass } = await import(`${ROOT}/src/scene/passTry.ts`);
  const stand = (f: string, dress: (L: any) => void) => { const L = loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${f}`, 'utf8')), { lived: true, dressing: true }) as any, was = new Map<any, any>(L.boxes.map((b: any) => [b, b.albedo])); dress(L);
    const made = L.boxes.filter((b: any) => was.get(b) !== b.albedo), silent = made.filter((b: any) => paintOf(b.albedo)?.stuff !== undefined && b.stuff === undefined), words = new Set(made.map((b: any) => b.stuff ?? 'plain'));
    return { made: made.length, silent: silent.length, said: made.filter((b: any) => b.stuff !== undefined).length, words }; };
  const pass = stand('pass.json', L => { dressFront(L); if (dressLorry(L)) kitLorry(L); dressPass(L); });
  // (the lorry's trial makes nothing here since the level's lorry says its own shapes, scene/carShapes.ts: lorryTry.ts dressLorry stands down, and the
  // 136 boxes are the house front's and the yard's — its sheet steel went with it)
  check(pass.made > 120 && pass.silent === 0 && pass.said > 0.75 * pass.made && ['wood', 'paint', 'mud wall', 'cloth'].every(w => pass.words.has(w)),
    '12 the look page\'s hand-built pieces say their words: on the pass every box the house front and the yard\'s pieces make or repaint in a named paint says what that paint is made of (wood, paint, mud wall and cloth among them); the lorry\'s trial stands down, the level\'s lorry saying its own shapes',
    `the pass: ${pass.made} made or repainted, ${pass.said} say a word (${[...pass.words].sort().join(', ')}), ${pass.silent} in a named paint and silent`); }

// 13. THE EDITOR KEEPS A PAINT'S WORD (src/editor/paint.ts): editing a paint's colour or finish leaves what it is made of, a copy of a paint is made of the same,
//     the word can be set and cleared and a word that is no stuff is refused; a blank document's own paints say theirs
{ const P = await import(`${ROOT}/src/editor/paint.ts`), ed = new Editor(dataOf('office.json') as any, {}), M = () => ed.data.materials as Record<string, { stuff?: string; rough?: number }>;
  const was = M().wood.stuff; P.materialSet(ed, 'wood', { rough: 0.5 }); const kept = M().wood.stuff; P.materialPreset(ed, 'wood', P.FINISH_PRESETS[0].key); const preset = M().wood.stuff;
  const copy = P.materialDuplicate(ed, 'carpet'), copied = M()[copy].stuff; P.materialSet(ed, copy, { stuff: 'carpet' }); const set = M()[copy].stuff; P.materialSet(ed, copy, { stuff: null }); const cleared = M()[copy].stuff;
  let refused = false; try { P.materialSet(ed, copy, { stuff: 'banana' }); } catch { refused = true; }
  const plainWas = M().black.stuff; P.materialSet(ed, 'black', { rough: 0.3 }); const plainKept = M().black.stuff;
  const fresh = P.materialCreate(ed, 'newpaint'), blank = Editor.blank('t').materials as Record<string, { stuff?: string }>;
  check(was === 'wood' && kept === 'wood' && preset === 'wood' && copied === M().carpet.stuff && copied !== undefined && set === 'carpet' && cleared === undefined && refused && plainWas === 'plain' && plainKept === 'plain' && M()[fresh].stuff === undefined
    && blank.wood.stuff === 'wood' && blank.wall.stuff === 'plaster' && blank.metal.stuff === 'steel' && validateLevelData(ed.data as never).length === 0,
    '13 the editor keeps a paint\'s word: a change of roughness and a finish preset leave the wood paint wood, a copy of the carpet paint is made of what the carpet is, the word is set and cleared by name and a word that is no stuff is refused; a paint that says plain still says it after an edit (as the exporters write it); a new paint says nothing until told; a blank document\'s own paints say what their names are made of',
    `wood: ${was} → after a roughness edit ${kept}, after a preset ${preset} · the carpet's copy: ${copied}, set ${set}, cleared ${cleared ?? 'nothing'} · banana refused ${refused} · blank: wood ${blank.wood.stuff}, wall ${blank.wall.stuff}, metal ${blank.metal.stuff}, asphalt ${blank.asphalt.stuff ?? 'nothing'}`); }

// 14. A SHAPE'S PIECE SAYS ITS WORD AS A BOX DOES (scene/shapes.ts, a box's `shape.with`): its own, else its paint's row's, else nothing; the box stood up for
//     it carries the word; the file keeps only a word that is another thing than the row's, and a word that is no stuff is refused
{ const { shapeLevel } = await import(`${ROOT}/src/scene/shapes.ts`);
  const D = dataOf('office.json') as any, M = D.materials, bi = D.boxes.findIndex((b: any) => b.material === 'wood' && b.rot === undefined && b.markings === undefined && b.shape === undefined && !(b.flags ?? []).includes('Glass')), b = D.boxes[bi];
  const up = b.h[1] + 0.02;
  b.shape = { with: [{ off: [0, up, 0], h: [0.05, 0.02, 0.05], material: 'metal' }, { off: [0.12, up, 0], h: [0.05, 0.02, 0.05], material: 'metal', stuff: 'plastic' }, { off: [-0.12, up, 0], h: [0.05, 0.02, 0.05], albedo: [0.31, 0.27, 0.4] }, { off: [0, up, 0.12], h: [0.05, 0.02, 0.05], material: 'wood', stuff: 'plain' }] };
  const errs = validateLevelData(D), L = loadLevel(D) as any, said = (L.boxes[bi].shape.with as any[]).map(w => w.stuff ?? 'nothing');
  shapeLevel(L.boxes, true); const stood = (L.boxes as any[]).filter(x => x.piece === true && x.of === bi).map(x => x.stuff ?? 'nothing'); shapeLevel(L.boxes, false);
  const back = (levelToData(L, { materials: M }) as any).boxes[bi].shape.with.map((w: any) => w.stuff ?? 'nothing');
  const bad = JSON.parse(JSON.stringify(D)); bad.boxes[bi].shape.with[0].stuff = 'banana'; const refused = validateLevelData(bad).some((e: string) => e.includes('banana'));
  const row: string = M.metal?.stuff ?? 'nothing';   // (what the file's own row says the kit's metal is made of)
  check(errs.length === 0 && row !== 'nothing' && said.join() === [row, 'plastic', 'nothing', 'nothing'].join() && stood.join() === said.join() && back.join() === ['nothing', 'plastic', 'nothing', 'plain'].join() && refused,
    '14 a shape\'s piece says its word as a box does: a piece in a named paint is made of what that paint\'s row says, one that says its own word is that, one written as numbers and one that says plain over a row are nothing; the box stood up for each carries the same; the file keeps only the word that is another thing than the row\'s; a word that is no stuff is refused',
    `the row: ${row} · loaded: ${said.join(', ')} · stood up: ${stood.join(', ')} · written back: ${back.join(', ')} · banana refused ${refused} · ${errs.length} errors${errs.length ? `: ${errs.slice(0, 2).join(' | ')}` : ''}`); }

console.log(`=== stuff: ${red ? 'FAILURES above' : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
