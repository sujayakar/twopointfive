#!/usr/bin/env bun
// NO RULE OF THE GAME READS A COLOUR TO KNOW WHAT A THING IS (src/scene/materials.ts: the rules read a box's word, Box.does).
//   bun run tools/qa/does-check.ts                    exit 0 = all pass, 1 = a line failed (named)
//   TPS_PROFILE=shipped bun run tools/qa/does-check.ts   the same on the page's stand (run both: the stands differ in which rules are up)
//   DOES_BREAK=1 bun run tools/qa/does-check.ts       the red path: the levels are stood up with every box's word taken off in place of its colour
//                                                     changed, as a build that knew a thing by its colour would see a repainted level — line 3 must FAIL
// WHAT IS HELD:
//   1 the table   every word of the table of materials that has a `does` is one a level file may say; the default row is there; a row's `wet` names a
//                 row; the words the table marks as drawn are the look's own list of materials, no more and no fewer (render/lookTex.ts LOOK_NAMES that
//                 have a number); every paint name that says what it does (scene/stuff.ts PAINT_DOES) names a row
//   2 the source  nothing under src/ names one of the old colour readers (paletteName, surfOf, paintStandsFor, steelGrey, gwotSimOf, the stand-in list);
//                 no file of the renderer asks what a box does but the snow field, which is one model the picture and the game share; and none of the
//                 rules' own files (the wall engine, a round through a wall, a wall charge, the furniture blast, the window bars, the snow bog, the
//                 wind's snow test) imports the look's table, the table of paint names or a palette
//   3 the proof   a level is stood up three times: as its file has it; with EVERY colour of every box and every prop's part moved by 0.004 a channel
//                 (the maintainer's experiment of 2026-11-05: it took the office from 18 breakable walls, 56 pieces of cover and 137 things a round
//                 goes through to none); and with every colour replaced outright by another (the same new colour wherever the old one was the same: two boxes in one paint stay in one). Each time the game is asked everything its rules make of
//                 a material: the wall engine's walls and cover with each one's stuff, what a round goes through and how thick, the outer walls a
//                 charge may cut, the furniture a blast breaks, the destruction index's paper, screens, housings and clutter, which boxes are lying
//                 snow, what a round knocks out of every static (and of the lot's slab soaked), and how each part of every prop breaks. The three answers must be the same, line for line
//   4 the files   no level file is from before the rule: each says what its boxes do, on a row or on a box (one that says nothing would stand with every
//                 box on the default row: no wall a charge cuts, no cover, nothing a round goes through, exactly what the red path shows.
//                 tools/levels/does-bake.ts brings such a file forward)
// A level and its look twin are held to the same words by tools/qa/look-check.ts; that every box's word is what its colour once said, by does-audit.ts.
// NOT HELD HERE, AND STILL TRUE OF THE GAME: two rules ask a PAINT something that is no reading of a material. Which slabs are one round thing is asked
// of their wearing the same paint (scene/round.ts, geoTry.ts: why line 3 gives one new colour a paint, not a box); and a vehicle's lamps are found by
// a signature a generator leaves, a size and a paint together (game/carlamps.ts), so a car's lamp repainted is no lamp to a round.
// Nothing here looks at a picture.
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT, standUp } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { MATERIALS, DOES_WORDS, LOOKS_WORDS, DOES_DEFAULT, doesWordOk, doesOfWord, doesRows } = await import(`${ROOT}/src/scene/materials.ts`);
const { LOOK_NAMES, lookStuffOf } = await import(`${ROOT}/src/render/lookTex.ts`);
const { PAINT_DOES } = await import(`${ROOT}/src/scene/stuff.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { fromBeforeDoes } = await import('../levels/does-bake.ts');
const { doesWet } = await import(`${ROOT}/src/scene/materials.ts`);
const { thinThing } = await import(`${ROOT}/src/game/wallpen.ts`);
const { outerWall } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { surfAt } = await import(`${ROOT}/src/game/particleFx.ts`);
const { snowPaintOf } = await import(`${ROOT}/src/render/snowField.ts`);
const { breakClassOf } = await import(`${ROOT}/src/game/furnblast.ts`);

export async function doesCheck(): Promise<boolean> {
const BREAK = process.env.DOES_BREAK === '1';
let ok = true; const check = (pass: boolean, what: string, detail = '') => { console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); if (!pass) ok = false; };
console.log(`\n=== does-check: no rule reads a colour to know what a thing is (stand: ${process.env.TPS_PROFILE === 'shipped' ? 'shipped' : 'frozen'})${BREAK ? ' (DOES_BREAK: the words taken off, as a build that read colours would see a repaint)' : ''} ===`);

// 1. the table
{ const rows = doesRows(), drawn = (LOOK_NAMES as readonly string[]).filter(n => lookStuffOf(n) > 0).sort(), marked = [...LOOKS_WORDS].sort();
  const wetOff = DOES_WORDS.filter((w: string) => { const wet = doesOfWord(w).wet; return wet !== undefined && !doesWordOk(wet); });
  const namesOff = Object.entries(PAINT_DOES as Record<string, string>).filter(([, w]) => !doesWordOk(w)).map(([n]) => n);
  const flat = rows.rows.every((r: number[]) => r.length === rows.cols.length && r.every(v => Number.isInteger(v) && v >= 0));
  check(DOES_WORDS.length >= 30 && DOES_WORDS.includes(DOES_DEFAULT) && wetOff.length === 0 && namesOff.length === 0 && JSON.stringify(drawn) === JSON.stringify(marked) && flat && Object.keys(MATERIALS).length === new Set([...DOES_WORDS, ...LOOKS_WORDS]).size,
    '1 the table of materials: every row that says what a thing does is a word a file may say, the default row among them; a wet word names a row; the rows marked as drawn are the look\'s own materials, each of them and no other; every paint name says a row; the columns go out as whole numbers',
    `${Object.keys(MATERIALS).length} rows · ${DOES_WORDS.length} say what a thing does (default '${DOES_DEFAULT}') · ${marked.length} are drawn, the look has ${drawn.length}${JSON.stringify(drawn) === JSON.stringify(marked) ? '' : ` (not the same: ${drawn.filter((n: string) => !marked.includes(n)).concat(marked.filter((n: string) => !drawn.includes(n))).join(', ')})`} · ${Object.keys(PAINT_DOES).length} paint names${namesOff.length ? `, ${namesOff.join(', ')} name no row` : ''}${wetOff.length ? ` · wet words that are no row: ${wetOff.join(', ')}` : ''} · ${rows.cols.length} columns as numbers`); }

// 2. the source
{ const walk = (dir: string): string[] => readdirSync(dir, { withFileTypes: true }).flatMap(e => e.isDirectory() ? walk(`${dir}/${e.name}`) : /\.(ts|wgsl)$/.test(e.name) ? [`${dir}/${e.name}`] : []);
  const files = walk(`${ROOT}/src`), rel = (f: string) => f.slice(ROOT.length + 1), text = new Map(files.map(f => [f, readFileSync(f, 'utf8')]));
  const OLD = /\b(paletteName|surfOf|paintStandsFor|steelGrey|gwotSimOf|GWOT_SIM|GWOT_SIM_NAMES|SNOW_SIM)\b\s*[(\[=:,;]/;   // (a call, a table or an import of one: a name told in a comment is history)
  const code = (t: string) => t.split('\n').filter(l => !/^\s*(\/\/|\*|\/\*)/.test(l)).map(l => l.replace(/\/\/.*$/, '')).join('\n');
  const old = files.filter(f => OLD.test(code(text.get(f)!))).map(rel);
  const asks = /from '\.\.?\/(scene\/)?materials'/, renderAsks = files.filter(f => rel(f).startsWith('src/render/') && asks.test(text.get(f)!)).map(rel).filter(f => f !== 'src/render/snowField.ts');
  const RULES = ['wallbreak', 'wallpen', 'wallcharge', 'furnblast', 'winbars', 'snowBog', 'airSim'].map(n => `${ROOT}/src/game/${n}.ts`), LOOKS = /from '\.\.\/(render\/lookTex|scene\/stuff|scene\/gwotPalette|scene\/gwotKit)'/;
  const ruleLooks = RULES.filter(f => !existsSync(f) || LOOKS.test(text.get(f) ?? '')).map(rel), ruleAsks = RULES.filter(f => existsSync(f) && (asks.test(text.get(f)!) || /snowPaintOf|breakClassOf/.test(text.get(f)!))).length;   // (the table itself, or the one function of another rule file that reads it: the snow test, the break class)
  check(old.length === 0 && renderAsks.length === 0 && ruleLooks.length === 0 && ruleAsks === RULES.length,
    '2 the source: nothing under src/ calls an old colour reader or keeps the stand-in list; no file of the renderer asks what a box does but the snow field; none of the rules\' own files imports the look\'s table, the paint names or a palette, and each asks the table of materials',
    `${files.length} files · old readers in ${old.length}${old.length ? `: ${old.join(', ')}` : ''} · renderer files that ask: ${renderAsks.join(', ') || 'none'} · rule files that import the look: ${ruleLooks.join(', ') || 'none'} · ${ruleAsks} of ${RULES.length} rule files ask what a box does`); }

// 3. the proof
type B = { albedo: number[]; flags: number; does?: string; c: number[]; h: number[] };
const quiet = async <T>(f: () => Promise<T>): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await f(); } finally { console.log = log; console.error = err; console.warn = warn; } };
/** everything the rules make of a material on a level, as lines of text, after `paint` has been at every box and every prop's part */
async function answers(file: string, paint: (o: { albedo: number[]; does?: string }, i: number) => void): Promise<{ lines: string[]; counts: Record<string, number> }> {
  const lvl = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')));
  let k = 0; for (const b of lvl.boxes) paint(b, k++); for (const p of lvl.props) for (const q of p.parts) paint(q, k++);
  const was = DESTRUCTION.on; DESTRUCTION.on = true;
  try { const W: any = await quiet(() => standUp({ level: lvl })), g = W.game, d = g.destr, boxes: B[] = g.col.boxes;
    const walls = d.walls.list.map((X: any) => `${boxes.indexOf(X.box)}:${X.kind}:${X.surf}`).sort();
    const thin: string[] = [], outer: string[] = [], snow: string[] = [], hit: string[] = [];
    boxes.forEach((b, i) => { const t = thinThing(g, i); if (t) thin.push(`${i}:${t.stuff}:${t.thick.toFixed(3)}`); if (outerWall(g, i)) outer.push(String(i)); const s = snowPaintOf(b); if (s) snow.push(`${i}:${s}`); hit.push(surfAt(g, b)); });
    const items = [...d.items].map(([i, x]: [number, any]) => `${i}:${x.kind}`).sort(), furn = JSON.stringify(d.furn.inventory().list);
    const lot = boxes.find(b => b.c[1] < 0 && b.c[1] + b.h[1] > -0.01 && b.h[0] > 18 && b.h[2] > 12);   // (the one slab the weather soaks in place: weather.ts setWet's own test)
    const parts = (g.props.props as any[]).flatMap(p => p.boxes.map((b: B) => breakClassOf(b)));   // (a prop a blast takes apart is broken part by part: furnblast.ts)
    return { lines: [`walls ${walls.join(' ')}`, `through ${thin.join(' ')}`, `outer ${outer.join(' ')}`, `furniture ${furn}`, `index ${items.join(' ')}`, `snow ${snow.join(' ')}`, `hits ${hit.join(' ')}`, `lot ${lot ? `${boxes.indexOf(lot)}: ${surfAt(g, lot)} now, ${doesWet(lot).surf} soaked` : 'none'}`, `props ${parts.join(' ')}`],
      counts: { walls: walls.filter((w: string) => w.includes(':wall:')).length, cover: walls.filter((w: string) => w.includes(':cover:')).length, through: thin.length, outer: outer.length, furniture: (JSON.parse(furn) as unknown[]).length, index: items.length, snow: snow.length } };
  } finally { DESTRUCTION.on = was; }
}
{ const LEVELS = ['office.json', 'barracks.json', 'barracks-gwot.json', 'depot-gwot.json', 'white-roof.json', 'pass.json'].filter(f => existsSync(resolve(ROOT, 'levels', f)));
  // (another colour for each colour: two boxes in one paint stay in one paint. Which slabs are one round thing is asked of their paint being the SAME
  //  paint — scene/round.ts, geoTry.ts — which is a question about the paint and no reading of a material; a colour a box of its own would undo it)
  const h = (a: number[], c: number) => { let x = 0x811c9dc5; for (const ch of `${a.join(',')}|${c}`) { x ^= ch.charCodeAt(0); x = Math.imul(x, 0x01000193) >>> 0; } x ^= x >>> 15; x = Math.imul(x, 0x2c1b3c6d) >>> 0; x ^= x >>> 12; return (x >>> 0) / 4294967296; };
  const bad: string[] = [], said: string[] = []; let total = { walls: 0, cover: 0, through: 0 };
  for (const f of LEVELS) {
    const asIs = await answers(f, () => { });
    // (the red path: the words go and the colours stay, which is what moving the colours did to a build that knew a thing by its colour)
    const moved = await answers(f, BREAK ? o => { o.does = undefined; } : o => { o.albedo = [o.albedo[0] + 0.004, o.albedo[1] + 0.004, o.albedo[2] + 0.004]; });
    const other = await answers(f, BREAK ? o => { o.does = undefined; } : o => { o.albedo = [h(o.albedo, 0), h(o.albedo, 1), h(o.albedo, 2)]; });
    const off = (X: { lines: string[] }) => asIs.lines.map((l, k) => l === X.lines[k] ? '' : l.split(' ')[0]).filter(Boolean);
    const a = off(moved), b = off(other); if (a.length || b.length) bad.push(`${f}: ${a.length ? `moved by 0.004: ${a.join(', ')} differ (walls ${asIs.counts.walls} → ${moved.counts.walls}, cover ${asIs.counts.cover} → ${moved.counts.cover}, a round goes through ${asIs.counts.through} → ${moved.counts.through})` : ''}${a.length && b.length ? ' · ' : ''}${b.length ? `replaced outright: ${b.join(', ')} differ` : ''}`);
    total = { walls: total.walls + asIs.counts.walls, cover: total.cover + asIs.counts.cover, through: total.through + asIs.counts.through };
    said.push(`${f.replace('.json', '')}: ${asIs.counts.walls} walls, ${asIs.counts.cover} cover, ${asIs.counts.through} a round goes through, ${asIs.counts.outer} outer walls, ${asIs.counts.furniture} of furniture, ${asIs.counts.index} in the index, ${asIs.counts.snow} of snow`);
  }
  check(bad.length === 0 && LEVELS.length >= 4 && total.walls > 0 && total.cover > 0 && total.through > 0,
    `3 the proof: on ${LEVELS.length} levels, with every colour moved by 0.004 a channel and with every colour replaced outright, the game makes of every box what it made of it: the same walls and cover of the same stuff, the same things a round goes through at the same thickness, the same outer walls, furniture, index, snow, and the same thing knocked out of every static by a round`,
    `${said.join('\n    ')}${bad.length ? `\n    NOT SO: ${bad.join('\n    ')}` : ''}`); }

// 4. the files
{ const files = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).sort();
  const before = files.filter(f => fromBeforeDoes(JSON.parse(readFileSync(`${ROOT}/levels/${f}`, 'utf8'))));
  check(before.length === 0 && files.length > 40, '4 no level file is from before the rule: every one says what its boxes do',
    `${files.length} files${before.length ? ` · FROM BEFORE (bun run tools/levels/does-bake.ts <file>, or export it again): ${before.join(', ')}` : ''}`); }

console.log(ok ? '=== does-check: ALL PASS ===' : '=== does-check: FAILED ===');
return ok;
}
if (import.meta.main) process.exit((await doesCheck()) ? 0 : 1);
