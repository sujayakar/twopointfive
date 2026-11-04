// WHAT EVERY BOX OF EVERY LEVEL IS MADE OF, BEFORE AND AFTER (src/scene/stuff.ts: a thing is made of what it says).
//   bun run tools/qa/stuff-audit.ts                         every levels/*.json: as HEAD has it against the working tree (once HEAD's files say their words this
//                                                           compares words with words: give --before a ref from before the rule to see what the colour rule drew)
//   bun run tools/qa/stuff-audit.ts --before <ref|dir>      "before" = that commit's files (git show), or the files of a folder
//   bun run tools/qa/stuff-audit.ts --after <dir>           "after" = the files of a folder (a generator's fresh output, say)
//   bun run tools/qa/stuff-audit.ts barracks compound-gwot  only those
//   … --list 20     name up to 20 boxes of each kind of change (default 4)     … --strict    a box that GAINED a material fails too
// BEFORE is what the game drew a box as: by the old rule (tools/levels/stuffWas.ts, frozen: the box's word, the first row of its colour by name, the colour
// itself exactly and darkened) for a file whose rows say nothing; by today's rule for a file that already says. AFTER is today's rule: the box's own word,
// else its material row's, else plain. A box is matched to itself by its place in the file (by its id where the two files have other boxes).
// EXIT 1 if any box that was drawn as a material is now drawn as another or as none ("zero changes where the colour matched"). A box that was plain and is a
// material now is listed and counted, and fails only with --strict.
// No game is stood up and no picture is looked at: what a box is drawn as is decided on the CPU from the level file alone (the loader puts the word on the
// box, render/gbuffer.ts lookWordOf reads it), and this reads the file as the loader does. The boxes the GAME makes (a door's leaf, debris) are not here.
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
import { drawnAlike, type Was } from '../levels/stuffWas.ts';
import { wasAll } from '../levels/stuff-bake.ts';
import { fromBefore, paintUsers, rowOf } from '../levels/paintUsers.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);

const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const BEFORE = flag('--before') ?? 'HEAD', AFTER = flag('--after') ?? `${ROOT}/levels`, LIST = Number(flag('--list') ?? 4), STRICT = argv.includes('--strict');
const only = argv.filter((a, i) => !a.startsWith('--') && !['--before', '--after', '--list'].includes(argv[i - 1] ?? ''));
const beforeDir = existsSync(resolve(BEFORE)) ? resolve(BEFORE) : null;
const names = readdirSync(AFTER).filter(f => f.endsWith('.json') && !f.includes('.lint') && (!only.length || only.includes(f.replace('.json', '')))).sort();
const textBefore = (f: string): string | null => { try { return beforeDir ? readFileSync(`${beforeDir}/${f}`, 'utf8') : execFileSync('git', ['-C', ROOT, 'show', `${BEFORE}:levels/${f}`], { encoding: 'utf8', maxBuffer: 1 << 28, stdio: ['ignore', 'pipe', 'ignore'] }); } catch { return null; } };

type D = Parameters<typeof wasAll>[0];
/** today's rule over a file: the box's own word, else its row's, else plain */
const nowAll = (d: D): Was[] => [...paintUsers(d)].map(({ o }) => { if (o.stuff !== undefined) return { stuff: o.stuff, by: 'own' as const }; const m = rowOf(d, o), w = m !== undefined ? d.materials[m]?.stuff : undefined; return w !== undefined ? { stuff: w, by: 'name' as const } : { stuff: 'plain', by: 'none' as const }; });
/** a paint user's own name in its file: a box's id, a part by its prop's id and its place */
const keysOf = (d: D): string[] => [...paintUsers(d)].map(({ o, where, kind }) => kind === 'box' ? `box ${(o as { id?: string }).id}` : kind === 'part' ? `part ${(d.props[Number(/props\[(\d+)\]/.exec(where)![1])] as { id?: string }).id}#${/parts\[(\d+)\]/.exec(where)![1]}` : where);
const colourOf = (d: D, o: { material?: string; albedo?: number[] }): string => { const m = rowOf(d, o); return m !== undefined ? m : `[${o.albedo!.join(', ')}]`; };

const BY = ['own', 'name', 'colour', 'darkened', 'none'] as const;
let oldFiles = 0;
const tot = { files: 0, users: 0, same: 0, lost: 0, moved: 0, gained: 0, unmatched: 0, by: Object.fromEntries(BY.map(b => [b, 0])) as Record<string, number>, said: 0, rowSaid: 0, plain: 0 };
const kinds = new Map<string, { n: number; files: Map<string, number>; eg: string[] }>();
const note = (kind: string, f: string, eg: string) => { const k = kinds.get(kind) ?? { n: 0, files: new Map<string, number>(), eg: [] }, name = f.replace('.json', ''); k.n++; k.files.set(name, (k.files.get(name) ?? 0) + 1); if (k.eg.length < LIST) k.eg.push(`${f.replace('.json', '')} ${eg}`); kinds.set(kind, k); };
console.log(`\n=== stuff-audit: what each box is made of, ${beforeDir ? `the files of ${BEFORE}` : `as ${BEFORE} has it`} against ${AFTER === `${ROOT}/levels` ? 'the working tree' : AFTER} ===`);
console.log(`${'level'.padEnd(26)}${'boxes'.padStart(7)}  before by: ${BY.map(b => b.padStart(8)).join('')}   ${'same'.padStart(6)}${'lost'.padStart(6)}${'moved'.padStart(6)}${'gained'.padStart(7)}   after: ${'own'.padStart(6)}${'row'.padStart(7)}${'plain'.padStart(7)}`);
for (const f of names) {
  const t0 = textBefore(f); if (t0 === null) { console.log(`${f.replace('.json', '').padEnd(26)}  (not there before)`); continue; }
  const d0 = parseLevelData(t0) as D, d1 = parseLevelData(readFileSync(`${AFTER}/${f}`, 'utf8')) as D;
  const old = fromBefore(d0); if (old) oldFiles++; const w0 = old ? wasAll(d0) : nowAll(d0), w1 = nowAll(d1), u0 = [...paintUsers(d0)], u1 = [...paintUsers(d1)];
  // the same boxes in the same places, or matched by name
  let pair: (number | undefined)[];
  if (u0.length === u1.length && u0.every((u, i) => u.where === u1[i].where)) pair = u1.map((_, i) => i);
  else { const at = new Map(keysOf(d0).map((k, i) => [k, i] as const)); pair = keysOf(d1).map(k => at.get(k)); }
  const row = { same: 0, lost: 0, moved: 0, gained: 0, by: Object.fromEntries(BY.map(b => [b, 0])) as Record<string, number>, own: 0, rowSaid: 0, plain: 0 };
  u1.forEach((u, i) => {
    const a = w1[i]; if (a.by === 'own') row.own++; else if (a.by === 'name') row.rowSaid++; else row.plain++;
    const j = pair[i]; if (j === undefined) { tot.unmatched++; return; }
    const b = w0[j]; row.by[b.by]++;
    if (drawnAlike(b.stuff, a.stuff)) { row.same++; return; }
    const was = drawnAlike(b.stuff, 'plain'), is = drawnAlike(a.stuff, 'plain'), what = `${b.by === 'none' ? 'plain' : `${b.stuff} by ${b.by === 'colour' || b.by === 'darkened' ? `its colour${b.by === 'darkened' ? ', darkened' : ''} (${b.row})` : b.by === 'name' ? 'its name' : 'its own word'}`} → ${a.stuff}`;
    if (was) row.gained++; else if (is) row.lost++; else row.moved++;
    note(`${was ? 'GAINED' : is ? 'LOST' : 'MOVED'}: ${what}`, f, `${keysOf(d1)[i]} ${colourOf(d1, u.o)}`);
  });
  tot.files++; tot.users += u1.length; tot.same += row.same; tot.lost += row.lost; tot.moved += row.moved; tot.gained += row.gained; for (const b of BY) tot.by[b] += row.by[b]; tot.said += row.own; tot.rowSaid += row.rowSaid; tot.plain += row.plain;
  console.log(`${f.replace('.json', '').padEnd(26)}${String(u1.length).padStart(7)}             ${BY.map(b => String(row.by[b]).padStart(8)).join('')}   ${String(row.same).padStart(6)}${String(row.lost).padStart(6)}${String(row.moved).padStart(6)}${String(row.gained).padStart(7)}          ${String(row.own).padStart(6)}${String(row.rowSaid).padStart(7)}${String(row.plain).padStart(7)}${old ? '' : '   (said before too)'}`);
}
console.log(`${'ALL'.padEnd(26)}${String(tot.users).padStart(7)}             ${BY.map(b => String(tot.by[b]).padStart(8)).join('')}   ${String(tot.same).padStart(6)}${String(tot.lost).padStart(6)}${String(tot.moved).padStart(6)}${String(tot.gained).padStart(7)}          ${String(tot.said).padStart(6)}${String(tot.rowSaid).padStart(7)}${String(tot.plain).padStart(7)}`);
if (kinds.size) { console.log(`\nwhat changed, by kind:`); for (const [k, v] of [...kinds].sort((a, b) => b[1].n - a[1].n)) console.log(`  ${String(v.n).padStart(5)}  ${k}   [${[...v.files].map(([f, n]) => n > 1 ? `${f} ×${n}` : f).join(', ')}]${v.eg.length ? `\n         ${v.eg.join('\n         ')}` : ''}`); }
const bad = tot.lost + tot.moved + (STRICT ? tot.gained : 0);
// (what "before" was: a side whose files already say their words is read by today's rule, so the count is words against words and says nothing of the colour rule)
const against = oldFiles === tot.files ? 'against what the colour rule drew' : oldFiles === 0 ? 'WORDS AGAINST WORDS: the before side already says what its boxes are made of, so this does NOT compare with the old colour rule (give --before a ref from before it)' : `against the colour rule for ${oldFiles} files and words against words for the other ${tot.files - oldFiles}`;
console.log(`\n=== stuff-audit: ${tot.files} files, ${tot.users} boxes and parts · ${tot.same} drawn as before · ${tot.lost} lost their material · ${tot.moved} drawn as another · ${tot.gained} were plain and are a material now${tot.unmatched ? ` · ${tot.unmatched} not there before` : ''} — ${bad ? 'CHANGES above' : 'no box that had a material lost or moved it'}, ${against} ===`);
process.exit(bad ? 1 : 0);
