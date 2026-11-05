// WHAT EVERY BOX OF EVERY LEVEL DOES, BY ITS WORD AGAINST BY ITS COLOUR (src/scene/materials.ts: the rules read a box's word).
//   bun run tools/qa/does-audit.ts                 every levels/*.json as it stands: each box's word (its own, else its material row's, else the default row)
//                                                  against what the OLD rule made of its colour (tools/levels/doesWas.ts, frozen)
//   bun run tools/qa/does-audit.ts barracks …      only those          … --list 12   name up to 12 boxes of each kind of difference (default 4)
//   … --stamp     first give every box that says nothing the word its colour had, as the exporter does (what a file would say once exported)
//   … --before <ref>   compare WORDS instead: each box's word in the files of that commit against its word now (after the colours changed and the
//                      colour rule can no longer be asked: a look's paints lost their hair's shift). A box is matched by its place in the file.
// THE COLUMNS: every one a rule reads (materials.ts Does). Three of them the destruction index read by TONE and by the dead screen's colour, and only of a
// box it could have looked at, so they are compared only there: `paper` of a small thin drawn-only sheet off the floor, `casing` of small clutter at desk
// height (a monitor's foot) or a thin slab within 8 cm of a dead screen and six boxes of it in the file (its housing), `screen` of a slab no thicker
// than 24 mm. Elsewhere no rule could tell. `steel` (a plain grey: a cabinet) the blast asked only of a box that could be a carcass, 25 cm to 2.2 m
// through and not tilted, and it is compared there: a look's skin has always let a floor's finish differ in it from the plain level's.
// THE LOT, WET: the one slab the weather soaks (game/weather.ts) was read by its halved colour while soaked, and of the ground a rule asks only what a
// round knocks out of it: its word's `wet` row against that, `surf` alone.
// A PAINT OF THE LOOK ON A THING THAT DOES SOMETHING ELSE cannot be asked this way any more. Until 2026-11-05 such a box wore its paint a hair off (the name
// it stood for in the offset), and on the files of that day (the commit that first wrote the words) this audit asked every box of every file and found
// none. Since then a look's skin and gwotAs lay the paint's own numbers and the box says its word, so its colour says only what the paint's NAME says:
// those boxes are counted and not judged here. They are held two other ways: `--before <that commit>` (each box says the word it said) and
// tools/qa/look-check.ts (a level and its look twin say the same word of every box).
// EXIT 1 if any box does another thing by its word than by its colour. No game is stood up: what a box does is on the box once the file is loaded.
import { execFileSync } from 'node:child_process';
import { readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
import { wasOf, wasWetOf, wordWas, lookRowWas, differs, stampDoes } from '../levels/doesWas.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const { doesOf, doesWet, DOES_DEFAULT } = await import(`${ROOT}/src/scene/materials.ts`);
const { LIVED } = await import(`${ROOT}/src/scene/lived.ts`); const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`); LIVED.on = true; DESTRUCTION.on = true;   // (every box of the file: the lived things and the dressing with the rest)

const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const LIST = Number(flag('--list') ?? 4), STAMP = argv.includes('--stamp'), BEFORE = flag('--before');
const only = argv.filter((a, i) => !a.startsWith('--') && !['--list', '--before'].includes(argv[i - 1] ?? ''));
const names = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint') && (!only.length || only.includes(f.replace('.json', '')))).sort();
type B = { c: number[]; h: number[]; albedo: number[]; emissive: number[]; flags: number; does?: string; name?: string };
const load = (text: string) => loadLevel(parseLevelData(text)) as { boxes: B[]; props: { name: string; parts: { albedo: number[]; does?: string }[] }[] };

const kinds = new Map<string, { n: number; files: Map<string, number>; eg: string[] }>();
const note = (kind: string, f: string, eg: string) => { const k = kinds.get(kind) ?? { n: 0, files: new Map<string, number>(), eg: [] }, name = f.replace('.json', ''); k.n++; k.files.set(name, (k.files.get(name) ?? 0) + 1); if (k.eg.length < LIST) k.eg.push(`${name} ${eg}`); kinds.set(kind, k); };
const tot = { files: 0, boxes: 0, parts: 0, said: 0, bad: 0, lots: 0, standIns: 0, words: new Map<string, number>() };
/** a paint of the look, to the number, on a thing that says it does another thing than the paint's name: the colour cannot say (see the head) */
const standIn = (o: { albedo: number[]; does?: string }): boolean => lookRowWas(o.albedo) && (o.does ?? DOES_DEFAULT) !== wordWas(o.albedo);
console.log(`\n=== does-audit: what each box does, ${BEFORE ? `its word at ${BEFORE} against its word now` : `by its word against by its colour${STAMP ? ' (every silent box first given the word of its colour, as the exporter does)' : ''}`} ===`);
for (const f of names) {
  const lvl = load(readFileSync(`${ROOT}/levels/${f}`, 'utf8')); tot.files++;
  if (BEFORE) {
    let t0: string; try { t0 = execFileSync('git', ['-C', ROOT, 'show', `${BEFORE}:levels/${f}`], { encoding: 'utf8', maxBuffer: 1 << 28, stdio: ['ignore', 'pipe', 'ignore'] }); } catch { console.log(`${f}: not there at ${BEFORE}`); continue; }
    const was = load(t0);
    if (was.boxes.length !== lvl.boxes.length || was.props.length !== lvl.props.length) { note(`ANOTHER LEVEL: ${was.boxes.length} boxes and ${was.props.length} props before, ${lvl.boxes.length} and ${lvl.props.length} now`, f, ''); tot.bad++; continue; }
    lvl.boxes.forEach((b, i) => { tot.boxes++; const a = was.boxes[i].does ?? DOES_DEFAULT, n = b.does ?? DOES_DEFAULT; tot.words.set(n, (tot.words.get(n) ?? 0) + 1); if (a !== n) { tot.bad++; note(`${a} → ${n}`, f, `box ${i} [${b.albedo.join(', ')}]`); } });
    lvl.props.forEach((p, i) => p.parts.forEach((q, j) => { tot.parts++; const a = was.props[i].parts[j].does ?? DOES_DEFAULT, n = q.does ?? DOES_DEFAULT; if (a !== n) { tot.bad++; note(`a prop's part: ${a} → ${n}`, f, `${p.name}#${j}`); } }));
    continue;
  }
  if (STAMP) stampDoes(lvl);
  const seenBox = (b: B) => !(b.flags & (BoxFlag.Glass | BoxFlag.NoPrimary));
  const screens = lvl.boxes.map((b, i) => seenBox(b) && Math.min(b.h[0], b.h[1], b.h[2]) <= 0.012 && wasOf(b.albedo).screen ? i : -1).filter(i => i >= 0);   // (the index's screens and televisions, by the colour: a housing was looked for behind each)
  lvl.boxes.forEach((b, i) => {
    tot.boxes++; if (b.does !== undefined) tot.said++; tot.words.set(b.does ?? DOES_DEFAULT, (tot.words.get(b.does ?? DOES_DEFAULT) ?? 0) + 1);
    // what the destruction index could have looked at (game/destruction.ts index, as it read until the rules took words)
    const h = b.h, maxH = Math.max(h[0], h[1], h[2]), minH = Math.min(h[0], h[1], h[2]), bottom = b.c[1] - h[1], lit = b.emissive[0] + b.emissive[1] + b.emissive[2] > 0;
    const seen = seenBox(b), ghost = (b.flags & BoxFlag.NoShadow) !== 0;
    const housing = h[2] <= 0.05 && screens.some(si => si !== i && Math.abs(si - i) <= 6 && Math.hypot(lvl.boxes[si].c[0] - b.c[0], lvl.boxes[si].c[1] - b.c[1], lvl.boxes[si].c[2] - b.c[2]) < 0.08);
    const paper = seen && ghost && h[1] <= 0.03 && maxH >= 0.08 && maxH <= 0.2 && bottom > 0.3 && !lit;
    const clutter = seen && !ghost && maxH <= 0.17 && h[0] * h[1] * h[2] * 8 <= 0.02 && bottom >= 0.7 && bottom <= 1.6 && !lit;
    const carcass = !(b as { rot?: unknown }).rot && minH * 2 >= 0.25 && maxH * 2 <= 2.2;   // (furnblast.ts steel: STEEL_CARCASS_M, STEEL_CARCASS_MOST)
    const d = differs(doesOf(b), wasOf(b.albedo), { paper, casing: seen && (clutter || housing), screen: seen && minH <= 0.012 }).filter(x => carcass || !x.startsWith('steel'));
    if (d.length && standIn(b)) tot.standIns++;
    else if (d.length) { tot.bad++; note(`${b.does ?? `(${DOES_DEFAULT})`}: ${d.join(' · ')}`, f, `box ${i}${b.name ? ` '${b.name}'` : ''} [${b.albedo.join(', ')}] at (${b.c.map(v => v.toFixed(2)).join(', ')}) half (${h.join(', ')})`); }
  });
  lvl.props.forEach(p => p.parts.forEach((q, j) => { tot.parts++; if (q.does !== undefined) tot.said++; const d = differs(doesOf(q), wasOf(q.albedo), { paper: false, casing: false, screen: false }); if (d.length && standIn(q)) tot.standIns++; else if (d.length) { tot.bad++; note(`a prop's part, ${q.does ?? `(${DOES_DEFAULT})`}: ${d.join(' · ')}`, f, `${p.name}#${j} [${q.albedo.join(', ')}]`); } }));
  // the lot: the one big slab whose top is the ground the cars stand on (weather.ts setWet's own test)
  const lot = lvl.boxes.find(b => b.c[1] < 0 && b.c[1] + b.h[1] > -0.01 && b.h[0] > 18 && b.h[2] > 12);
  if (lot) { tot.lots++; const d = differs(doesWet(lot), wasWetOf(lot.albedo), { paper: false, casing: false, screen: false }).filter(x => x.startsWith('surf')); if (d.length) { tot.bad++; note(`THE LOT WHILE SOAKED, ${lot.does ?? `(${DOES_DEFAULT})`}: ${d.join(' · ')}`, f, `[${lot.albedo.join(', ')}]`); } }
}
for (const [k, v] of [...kinds].sort((a, b) => b[1].n - a[1].n)) { console.log(`\n  ${String(v.n).padStart(5)}  ${k}   [${[...v.files].map(([n, c]) => c > 1 ? `${n} ×${c}` : n).join(', ')}]`); for (const e of v.eg) if (e.trim()) console.log(`           ${e}`); }
console.log(`\n${tot.files} files · ${tot.boxes} boxes and ${tot.parts} props' parts${BEFORE ? '' : ` · ${tot.said} say a word, the rest take the default row · ${tot.lots} lots asked wet · ${tot.standIns} in a paint of the look that do another thing than the paint's name says, not judged by their colour`}`);
if (!BEFORE || !tot.bad) console.log(`words: ${[...tot.words].sort((a, b) => b[1] - a[1]).map(([w, n]) => `${w} ${n}`).join(' · ')}`);
console.log(tot.bad ? `=== does-audit: FAIL — ${tot.bad} ${BEFORE ? 'say another word than before' : 'do another thing by word than by colour'} ===` : `=== does-audit: PASS — ${BEFORE ? 'every box says the word it said' : 'every box does by its word what it did by its colour'} ===`);
process.exit(tot.bad ? 1 : 0);
