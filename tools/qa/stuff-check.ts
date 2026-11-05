// A THING IS MADE OF WHAT IT SAYS (src/scene/stuff.ts; src/scene/levelLoad.ts; src/render/gbuffer.ts lookWordOf).
//   bun run tools/qa/stuff-check.ts            exit 0 = every line passes
//   STUFF_BREAK=1 bun run tools/qa/stuff-check.ts     every level file is read with its material rows' words struck out (a file from before 11-04, loaded
//                                                    as it stood): lines 2, 3, 4, 7, 13, 14 and 15 must FAIL
//   COAT_BREAK=1 bun run tools/qa/stuff-check.ts      the table of named coats (scene/stuff.ts NAMED_COATS) is emptied: line 8 must FAIL
// No game is stood up and no picture is looked at: what a box is drawn as is decided on the CPU (the loader puts a word on the box, lookWordOf reads it) and
// this asks it. What a material LOOKS like on a desk is a page's still and then a GPU's. The same on both stands: nothing here is a rule of play and no
// stand's word reaches it. What every box WAS drawn as before the colour rule went, against what it is now, is tools/qa/stuff-audit.ts.
import { readFileSync, readdirSync } from 'node:fs';
import { ROOT } from './headless.ts';
import { fromBefore, paintUsers, rowOf } from '../levels/paintUsers.ts';
const ST = await import(`${ROOT}/src/scene/stuff.ts`), { PAINT_STUFF, stuffOfName, rowWord, BOX_STUFFS, saidFor, namePaints, paintOf, toned, tonedAs, madeOf, drawnAs, wordFor, worded, repainted, repaintWord, NAMED_COATS } = ST;
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
{ const none = new Map<string, string>(), bad: string[] = [], seen = new Set<string>(), kinds: Record<string, number> = { material: 0, plain: 0, waits: 0, hit: 0 };
  for (const f of FILES) for (const name of Object.keys(dataOf(f).materials)) { seen.add(name); if (stuffOfName(name) === undefined && !none.has(name)) none.set(name, f); }
  for (const [name, stuff] of Object.entries(PAINT_STUFF) as [string, string][]) { if (stuff === 'plain' || stuff === 'waits') kinds[stuff]++; else if (lookStuffOf(stuff) > 0) kinds.material++; else if (stuff === 'glass' || stuff === 'rubber') kinds.hit++; else bad.push(`${name}: '${stuff}'`); }   // (glass and rubber: the two the look draws plain and the picture of a round's hit reads — the vehicles' windows and tyres)
  check(none.size === 0 && bad.length === 0 && seen.size > 60 && stuffOfName('constructor') === undefined && stuffOfName('toString') === undefined && !(Object.values(PAINT_STUFF) as string[]).includes('mixed'),
    '1 every paint name in every level file has a row of its own in the table, and every row says a material the look draws, or plain, or waits, or one of the two a round\'s hit reads and the look draws plain, glass and rubber (no name is left to its colour); a name that is a method of every object has no row',
    `${seen.size} names in ${FILES.length} level files · ${Object.keys(PAINT_STUFF).length} rows: ${kinds.material} a material, ${kinds.plain} plain, ${kinds.waits} waiting, ${kinds.hit} glass or rubber${none.size ? ` · NO ROW: ${[...none].map(([n, f]) => `${n} (${f})`).join(', ')}` : ''}${bad.length ? ` · not a material: ${bad.join(', ')}` : ''}`); }

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
  // a repaint: the one helper gives the new paint's word, and a named coat stays under a plain one, a paint or an enamel alike; the skin repaints boxes and props' parts through it and nowhere else
  const coatWas = { ...NAMED_COATS }; if (process.env.COAT_BREAK === '1') for (const k of Object.keys(NAMED_COATS)) delete (NAMED_COATS as any)[k];   // (COAT_BREAK=1: the table of named coats is emptied, so a repaint keeps no coat — this line must FAIL)
  const rp = (was: string | undefined, paint: readonly number[]) => repainted({ albedo: [0.5, 0.5, 0.5] as readonly number[], ...(was ? { stuff: was } : {}) } as any, paint).stuff as string | undefined;
  const coats = `${rp('veneer', GWOT_PALETTE.galv)}, ${rp('steel', GWOT_PALETTE.paintGreen)}, ${rp('carpaint', GWOT_PALETTE.paintGreen)}, ${rp('carpaint', GWOT_PALETTE.galv)}, ${rp('carpaint', GWOT_PALETTE.prodOrange) ?? 'nothing'}, ${rp(undefined, GWOT_PALETTE.paintGreen)}, ${rp('carpaint', madeOf('enamel', [0.4, 0.1, 0.1]))}, ${rp('steel', madeOf('enamel', [0.4, 0.1, 0.12]))}`;
  const skinSrc = readFileSync(`${ROOT}/tools/levels/gwotSkin.ts`, 'utf8'), skin = skinSrc.includes('repainted(b, gwotAs(m, sim))') && skinSrc.includes('repainted(part, next)') && !/delete (b|part)\.stuff/.test(skinSrc) && coats === 'sheet, paint, carpaint, sheet, nothing, paint, carpaint, enamel' && repaintWord('paint', 'paint') === 'paint' && repaintWord('enamel', 'paint') === 'paint' && Object.keys(NAMED_COATS).length >= 1 && Object.keys(NAMED_COATS).every(k => (BOX_STUFFS as string[]).includes(k) && (NAMED_COATS as any)[k].under.includes('paint') && (NAMED_COATS as any)[k].under.includes('enamel') && (NAMED_COATS as any)[k].under.every((c: string) => (BOX_STUFFS as string[]).includes(c)));
  Object.assign(NAMED_COATS, coatWas);
  // a piece broken off a thing is made of what the thing is
  const from = makeBox({ c: [0, 0, 0], h: [1, 1, 1], albedo: [0.2, 0.2, 0.2], stuff: 'veneer' }), copy = makeBox({ c: [0, 0, 0], h: [0.1, 0.1, 0.1], albedo: [0.2, 0.2, 0.2], ...wordOf(from) }), bare = makeBox({ c: [0, 0, 0], h: [0.1, 0.1, 0.1], albedo: [0.2, 0.2, 0.2], ...wordOf(makeBox({ c: [0, 0, 0], h: [1, 1, 1] })) });
  check(says(own, 'veneer') && says(dark, 'veneer') && says(steel, 'steel') && says(white) && says(rug, 'cloth') && JSON.stringify(saidFor('veneer', [0.3, 0.31, 0.33], KIT_PALETTE.wood, KIT_PALETTE.woodDark)) === '{}' && JSON.stringify(saidFor('veneer', [...KIT_PALETTE.wood], KIT_PALETTE.wood, KIT_PALETTE.woodDark)) === '{}' && saidFor('veneer', KIT_PALETTE.wood, KIT_PALETTE.wood).stuff === 'veneer' && dropped && skin && copy.stuff === 'veneer' && bare.stuff === undefined,
    '8 a word belongs to the paint it was said with: the kit\'s desk says veneer in its own wood paints; handed the kit\'s steel it is a steel bench (the paint handed over speaks), handed white laminate it says nothing, and handed a COPY of the wood\'s numbers it says nothing either (the row, not its numbers); a rug is cloth in any colour; the skin repaints through the one helper, which drops a repainted thing\'s old word and gives it its new paint\'s — but a named coat stays under a plain one (a car in another enamel is still car paint; in sheet metal it is sheet, in a plain orange nothing); the editor\'s paint drops a box\'s own word; a piece broken off a thing copies the thing\'s word',
    `desk ${own[0]?.stuff}, dark ${dark[0]?.stuff}, as a steel bench ${steel[0]?.stuff ?? 'no word'}, white ${white[0]?.stuff ?? 'no word'} · rug ${rug[0]?.stuff} · the skin says the new word: ${skin} (veneer → sheet, steel → paint, car paint → paint, → sheet, → plain orange, nothing → paint, car paint → an enamel, steel → an enamel: ${coats}) · the editor drops it: ${dropped} · a broken piece: ${copy.stuff}`); }

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

// 12. A HOUSE FRONT'S AND A YARD'S PIECES SAY THEIR WORDS (scene/gwotKit.ts: what is done to a wall's face, and the round things a yard keeps; they were
//     the look page's hand-built trials until 11-05 and are the kit's now, laid by the pass's generator): every box they make in a paint somebody named
//     says that paint's stuff, and so does every piece of TRIM that wears a named paint of its own (a log's pale end is wood, a pipe's bore metal: a
//     shape's piece is of its box's stuff unless it says another). Asked of the generator, where a paint is still the row somebody named
{ const { build } = await import(`${ROOT}/tools/levels/pass.ts`), { dressLorry } = await import(`${ROOT}/src/scene/lorryTry.ts`);
  const g = build(), B = g.level.boxes as any[], mine = B.filter((_, i) => g.groups[i] === 'front' || g.groups[i] === 'yard'), named = (a: any) => { const w = paintOf(a)?.stuff; return w !== undefined && w !== 'plain'; };
  const silent = mine.filter(b => named(b.albedo) && b.stuff === undefined), trim = mine.flatMap(b => (b.shape?.with ?? []) as any[]).filter(p => p.albedo), trimSilent = trim.filter(p => named(p.albedo) && p.stuff === undefined);
  // (THE PIPES ARE ASKED WHERE THEY STAND: the pass laid them until 11-05 and the showroom still does; a pipe's bore is the one piece of trim that says metal)
  const hall = ((await import(`${ROOT}/tools/levels/library-hall.ts`)).build().level.boxes as any[]).flatMap(b => (b.shape?.with ?? []) as any[]).filter(p => p.albedo), hallSilent = hall.filter(p => named(p.albedo) && p.stuff === undefined);
  const said = mine.filter(b => b.stuff !== undefined).length, words = new Set(mine.map(b => b.stuff ?? 'plain')), trimWords = new Set(trim.map(p => p.stuff ?? 'plain')), hallWords = new Set(hall.map(p => p.stuff ?? 'plain'));
  const lorry = dressLorry(loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/pass.json`, 'utf8')), { lived: true, dressing: true }) as any);
  // (the lorry's trial makes nothing since the level's lorry says its own shapes, scene/carShapes.ts: lorryTry.ts dressLorry stands down)
  check(mine.length > 120 && silent.length === 0 && trimSilent.length === 0 && said > 0.75 * mine.length && ['wood', 'paint', 'mud wall', 'cloth', 'metal'].every(w => words.has(w)) && ['wood', 'cloth'].every(w => trimWords.has(w)) && hallSilent.length === 0 && ['wood', 'metal'].every(w => hallWords.has(w)) && lorry === 0,
    '12 a house front\'s and a yard\'s pieces say their words: on the pass every box the kit\'s wall-face and yard pieces make in a named paint says what that paint is made of (wood, paint, mud wall, cloth and metal among them), and every piece of trim in a named paint of its own says that paint\'s (a log\'s end wood, a rolled blanket\'s ties cloth), and so does the showroom\'s trim (a pipe\'s bore metal: the pipes stand there); the lorry\'s trial stands down, the level\'s lorry saying its own shapes',
    `the pass's front and yard: ${mine.length} boxes, ${said} say a word (${[...words].sort().join(', ')}), ${silent.length} in a named paint and silent · their trim in a paint of its own: ${trim.length} pieces (${[...trimWords].sort().join(', ')}), ${trimSilent.length} silent · the showroom's trim: ${hall.length} pieces (${[...hallWords].sort().join(', ')}), ${hallSilent.length} silent · the lorry's trial made ${lorry}`); }

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

// 15. NO WALL AND NO DOOR FRAME IS CLOTH BY A BORROWED PAINT: a generator or a skin that paints a wall or a frame in a cloth's paint for its COLOUR gets the
//     cloth with it (the depot's hut walls in the tan partition paint, the consulate's frames in a blanket's brown: drawn as weave, and a round knocked tufts
//     out of them, until 11-05). Such a thing says its own word over the paint (scene/stuff.ts saidAs; a skin's Said, tools/levels/gwotSkin.ts). A wall of
//     SACKS is cloth and meant: the sandbag's and the hesco's own paints. Asked of a level's own boxes in its `walls.` and `doorframe.` groups, by the row
//     each wears (rowOf), and of A GEAR SCREEN'S SET (scenarios.kitView: no groups there, so a box that stands as a wall: from the floor to 1.8 m, thin, long,
//     casting a shadow). A set's box names its rows 'look|plain' and IS ASKED AS THE LOOK WEARS IT, its first-named row: a stand-in on its fallback row (the
//     compound's house in the tan, in the plain file) is not asked and is NOT to be mended: a set is drawn only under the kit view, where the plain file draws
//     no stuff and nobody fires, and a word for each row would be a change to the set's format that nothing needs.
//     NOT ASKED YET, on purpose: the look's own painted bands on a wall's face (a dado, a wainscot: decals no ray meets). Two look files lay theirs in a
//     cloth's paint and are leaving with the setting they were made for; the band's line joins this one with whatever replaces them.
//     RED PATH: a wall written here in the tan partition paint must be found, and with STUFF_BREAK (the rows say nothing) it is not.
{ const CLOTHS = ['cloth', 'fabric', 'carpet'], SACKS = ['sandbag', 'hesco'];
  const standsAsWall = (o: any) => Array.isArray(o.c) && Array.isArray(o.h) && o.c[1] - o.h[1] <= 0.1 && o.c[1] + o.h[1] >= 1.8 && Math.min(o.h[0], o.h[2]) <= 0.15 && Math.max(o.h[0], o.h[2]) >= 0.5 && !(o.flags ?? []).includes('NoShadow');
  /** the walls and frames of a level file that are cloth and are not sacks: what each is, and how many; and its sets' walls, asked as the look wears them */
  const clothWalls = (d: any): { asked: number; sacks: number; sets: number; standins: number; bad: Map<string, number> } => { let asked = 0, sacks = 0, sets = 0, standins = 0; const bad = new Map<string, number>();
    const ask = (o: any, what: string) => { const row = rowOf(d, o), w = o.stuff ?? (row !== undefined ? d.materials[row]?.stuff : undefined); if (!CLOTHS.includes(w)) return;
      if (o.stuff === undefined && row !== undefined && SACKS.includes(row)) sacks++; else { const k = `${what} in ${row ?? 'a paint written as numbers'} says ${w}`; bad.set(k, (bad.get(k) ?? 0) + 1); } };
    for (const b of d.boxes as any[]) { const g: string = b.group ?? ''; if (!g.startsWith('walls.') && !g.startsWith('doorframe.')) continue; asked++; ask(b, g.split('.')[0]); }
    for (const { o, kind } of paintUsers(d)) { if (kind !== 'set' || !standsAsWall(o)) continue; const m = (o as any).material as string | undefined;
      if (m?.includes('|') && rowOf(d, o) !== m.split('|')[0]) { standins++; continue; } sets++; ask(o, 'a set\'s wall'); }
    return { asked, sacks, sets, standins, bad }; };
  let asked = 0, sacks = 0, sets = 0, standins = 0; const bad: string[] = [];
  for (const f of FILES) { const r = clothWalls(dataOf(f)); asked += r.asked; sacks += r.sacks; sets += r.sets; standins += r.standins; for (const [k, n] of r.bad) bad.push(`${n} × ${f} ${k}`); }
  // (the red path: a hut wall as the depot's stood, a frame as the consulate's stood, a wall of sacks, a wall that says its own word over the tan; and a set:
  //  a wall in the tan as its first-named row is found, the same wall on its fallback row is a stand-in, a line of washing is no wall)
  const rows = { partTan: { albedo: [0.55, 0.47, 0.35], stuff: 'cloth' }, sandbag: { albedo: [0.45, 0.39, 0.27], stuff: 'cloth' } } as Record<string, any>; if (BRK) for (const m of Object.values(rows)) delete m.stuff;
  const wall = { c: [5, 1.5, 7], h: [4, 1.5, 0.12] };
  const made = clothWalls({ materials: rows, props: [], boxes: [{ group: 'walls.huts', material: 'partTan' }, { group: 'doorframe.front', albedo: [0.27, 0.18, 0.12], stuff: 'cloth' }, { group: 'walls.exterior', material: 'sandbag' },
    { group: 'walls.huts', material: 'partTan', stuff: 'stucco' }, { group: 'cubicles', material: 'partTan' }],
    scenarios: { kitView: { sets: { yard: { boxes: [{ ...wall, material: 'partTan|wall' }, { ...wall, material: 'mud|partTan' }, { c: [3, 1.3, 5], h: [0.2, 0.34, 0.02], albedo: [0.14, 0.2, 0.45], stuff: 'fabric', flags: ['NoShadow'] }] } } } } });
  // … and the skin's own Said, read off the one file that wears it: every frame of the consulate in the varnish's paint says the varnish's stuff and finish
  const { VARNISH } = await import(`${ROOT}/tools/levels/gwotSkin.ts`), brown = GWOT_PALETTE[VARNISH.paint], inBrown = (b: any) => b.albedo !== undefined && [0, 1, 2].every(c => Math.abs(b.albedo[c] - brown[c]) <= 0.0025);
  const frames = (dataOf('consulate-gwot.json').boxes as any[]).filter(b => (b.group ?? '').startsWith('doorframe.') && inBrown(b)), varnished = frames.filter(b => b.stuff === VARNISH.stuff && b.rough === VARNISH.rough).length;
  check(bad.length === 0 && asked > 5000 && sacks > 0 && sets > 0 && made.asked === 4 && made.sacks === 1 && made.sets === 1 && made.standins === 1 && made.bad.size === 3 && frames.length > 40 && varnished === frames.length,
    '15 no wall and no door frame is cloth by a borrowed paint: in every level file no box of a `walls.` or `doorframe.` group is cloth, fabric or carpet, but for a wall of sacks in the sandbag\'s or the hesco\'s own paint; nor is a wall of a gear screen\'s set, asked as the look wears it (a stand-in on its fallback row is not asked); a hut wall in the tan partition paint, a frame that says cloth and a set\'s wall in the tan are found, a wall of sacks and a wall that says its own word are not; the consulate\'s frames wear the varnish\'s paint and say its stuff and finish (a skin\'s Said)',
    `${asked} wall and frame boxes in ${FILES.length} files · ${sacks} of sacks · ${sets} walls of sets asked, ${standins} stand-ins not asked · made up: ${made.bad.size} of 3 found, ${made.sacks} of sacks, ${made.standins} stand-in · the consulate's frames in the varnish's paint: ${frames.length}, ${varnished} say ${VARNISH.stuff} at ${VARNISH.rough}${bad.length ? ` · CLOTH: ${bad.join(' · ')}` : ''}`); }

// 16. A WORD AND ITS FINISH ARE SAID WITH THE PAINT, NEVER WRITTEN BESIDE IT BY HAND: scene/stuff.ts is the one way a paint says what it is made of
//     (`madeOf` for a colour written as numbers, `saidAs` for a kit's row that is another thing here, each with the thing's finish where it has its own).
//     A FINDER OVER THE SOURCE, line by line, of every module under tools/levels that lays boxes, and of the office's generator (scene/officeGen.ts):
//       · a literal that writes `stuff: '<word>'` beside a paint it names (a kit's row, numbers written out, a paint that has said its word), or with no
//         paint of its own at all (a word kept apart to spread into boxes, or handed in a call's options beside the paint);
//       · a `rough` or `metal` written beside a paint that has said its word (a helper's call, or one of the paints named once: PANEL_GREY, CHAIR_GREY,
//         KERB_GREY, RACK_BLACK, the office's K table), in the same literal or in the options that follow it in the call.
//     NOT ASKED: a piece that says its word for whatever paint it is HANDED (`albedo, stuff: 'cloth'`: a rug is cloth in any colour), the level format's
//     own data written as data (a gear screen's set: a literal with an `id`), a skin's Said (a literal with a `paint`), and the modules that ARE the way
//     or only read files (livedKit, livedPieces, gwotSkin; export, library, libraryDraw, paintUsers, stuff-bake, stuffWas).
//     WHAT A FINDER OVER LINES CANNOT SEE, said so that nobody takes its green for more: a call split over lines; a finish assigned afterwards
//     (`b.rough = …`); and a generator that keeps a paint record of its own and passes `stuff: P.stuff` on at every call (white-roof.ts, 11-05: owed
//     by its owner). RED PATH: eight made-up lines, five that must be found and three that must not; and the step before this line was red on 17.
{ const HELP = /\b(?:madeOf|saidAs|drawnAs)\((?:[^(){}]|\([^()]*\)|\{[^{}]*\})*\)/g, NAMED = /\b(?:PANEL_GREY|CHAIR_GREY|KERB_GREY|RACK_BLACK)\b|\bK\.[a-z]\w*\b(?!\()/g;
  /** every `{…}` of a line, innermost first, each with what is nested in it struck out: its own keys */
  const literals = (code: string): string[] => { const out: string[] = []; for (let s = code, n = 0; n < 60; n++) { const m = /\{[^{}]*\}/.exec(s); if (!m) break; out.push(m[0]); s = s.slice(0, m.index) + '⟨⟩' + s.slice(m.index + m[0].length); } return out; };
  /** what a line of a generator writes by hand beside a paint: nothing, or what it is */
  const byHand = (line: string): string[] => { const code = line.replace(/\s\/\/ .*$/, ''), out = new Set<string>(); if (/^\s*(\/\/|\*|\/\*)/.test(code)) return [];
    const said = code.replace(HELP, '§').replace(NAMED, '§');
    if (/§\s*,\s*(?:\[[^\[\]]*\]\s*,\s*)?\{[^{}]*\b(?:rough|metal):/.test(said)) out.add('a finish in the options beside a paint that has said its word');
    for (const lit of literals(said)) { if (/\bid: '/.test(lit) || /\bpaint: '/.test(lit)) continue; const word = /\bstuff: '/.test(lit);
      if (word && /\balbedo: (?:[A-Z]\w*\.\w+|\[[-\d., ]+\]|§)/.test(lit)) out.add('a word beside a paint named at the call');
      else if (word && !/\balbedo\b/.test(lit)) out.add('a word in a literal with no paint of its own');
      if (/\balbedo: §/.test(lit) && /\b(?:rough|metal):/.test(lit)) out.add('a finish beside a paint that has said its word'); }
    return [...out]; };
  const NOT = ['livedKit.ts', 'livedPieces.ts', 'gwotSkin.ts', 'export.ts', 'library.ts', 'libraryDraw.ts', 'paintUsers.ts', 'stuff-bake.ts', 'stuffWas.ts'];
  const files = [...readdirSync(`${ROOT}/tools/levels`).filter(f => f.endsWith('.ts') && !NOT.includes(f)).sort().map(f => `tools/levels/${f}`), 'src/scene/officeGen.ts'];
  const found: string[] = []; let lines = 0;
  for (const f of files) readFileSync(`${ROOT}/${f}`, 'utf8').split('\n').forEach((l, i) => { lines++; for (const w of byHand(l)) found.push(`${f}:${i + 1} ${w}`); });
  const madeUp = ["  add({ c: [1, 1, 1], h: [1, 1, 1], albedo: C.woodDark, stuff: 'veneer' });", "  const PHONE = { rough: 0.92, stuff: 'metal' } as const;", "  floor(14, 4, 22, 10, madeOf('paver', [0.2, 0.21, 0.24]), { rough: 0.35 });",
    "  const rack = add({ c: [x, 1, z], h: [0.3, 1, 0.5], albedo: drawnAs('metal', [0.06, 0.06, 0.07]), rough: 0.3, ...mk(marks(slots.map(sl => ({ v: sl.y })))) });", "  add({ c: [9, 1, 5], h: [0.5, 1, 0.5], albedo: CHAIR_GREY, rough: 0.4 });",
    "  add({ c: [x, 0.018, z], h: [w / 2, 0.006, d / 2], yaw, albedo, stuff: 'cloth' });", "  floor(14, 4, 22, 10, madeOf('paver', [0.2, 0.21, 0.24], { rough: 0.35 }));   // { rough: 0.3, stuff: 'metal' }",
    "    { id: 'cn_rug_a', group: 'cn.bunk', c: [22.7, 0.44, 11.62], h: [0.93, 0.04, 0.4], albedo: [0.27, 0.18, 0.12], stuff: 'cloth' },"].map(l => byHand(l).length ? 1 : 0);
  check(found.length === 0 && files.length > 60 && lines > 15000 && madeUp.join('') === '11111000',
    '16 a word and its finish are said with the paint (scene/stuff.ts madeOf, saidAs), never written beside it by hand: in every module that lays a level\'s boxes no line writes a word beside a paint it names or in a literal with no paint of its own, or a finish beside a paint that has said its word; a piece that says its word for whatever paint it is handed, a set\'s data and a skin\'s Said are not asked',
    `${files.length} modules, ${lines} lines · ${found.length} by hand${found.length ? `: ${found.slice(0, 30).join(' · ')}` : ''} · made up: ${madeUp.join(' ')} (the first five found, the last three not)`); }

console.log(`=== stuff: ${red ? 'FAILURES above' : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
