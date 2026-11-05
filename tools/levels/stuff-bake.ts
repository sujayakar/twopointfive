// BRING A LEVEL FILE FORWARD: every box says what it WAS drawn as.
// A level file written before 2026-11-04 says nothing about what most of its boxes are made of: the look read that off a paint's name and then off its
// colour. Both readings are gone from the game (src/scene/stuff.ts: a thing is made of what it SAYS), so such a file would draw every box plain. This writes
// the words in, from the old rule kept frozen in ./stuffWas.ts:
//   · each material ROW says what its NAME is made of (the table of names as it is now: stuff.ts rowWord, what the exporter writes too);
//   · each box, prop's part and gear screen's box says its own word ONLY where it was drawn as another thing than its row now says: the 2,200 stand-ins and
//     the 2,100 darkened paints written as numbers, the boxes under a name that is two things, and nothing else.
// The diff of a file is `"stuff"` words and nothing more; what each box is drawn as is the same before and after (tools/qa/stuff-audit.ts counts it).
// A FILE THAT ALREADY SAYS (a row with `stuff`) IS LEFT ALONE: its words are somebody's, and the old rule knows nothing of them.
//   bun run tools/levels/stuff-bake.ts levels/barracks.json …   write each file in place
//   bun run tools/levels/stuff-bake.ts --all                    every levels/*.json
//   … --check       write nothing; exit 1 if a file from before is still to be brought forward
//   … --out <dir>   write the results there instead (same file names)
// A LEVEL WITH A GENERATOR IS NOT MENDED HERE FOR GOOD: its next export writes what the generator says, so a box written as numbers there says its word
// THERE (a kit's paint handed over, stuff.ts tonedAs for a tone of one, or `stuff:` beside the numbers). This is the bridge for a file laid without its
// generator, and the yardstick: a generator's file from before, brought forward, is what the generator must now write.
import { basename, resolve } from 'node:path';
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { wasOf, type Was } from './stuffWas.ts';
import { fromBefore, paintUsers, rowOf, type Data, type Paints } from './paintUsers.ts';
export { fromBefore, paintUsers, rowOf };
const ROOT = resolve(import.meta.dir, '../..');
const { rowWord } = await import(`${ROOT}/src/scene/stuff.ts`);
const { parseLevelData, serializeLevelData, validateLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);

/** what each paint user of a file from before WAS drawn as, in paintUsers' order */
export function wasAll(d: Data): Was[] { const was = wasOf(d.materials); return [...paintUsers(d)].map(({ o }) => { const m = rowOf(d, o); return was(m !== undefined && m !== o.material ? { ...o, material: m } : o); }); }
/** put a word on a box or a part where the format keeps it: after its paint and finish, before its flags (a gear screen's boxes are written key for key as
 *  they stand, so the place is made here; levelData.ts KEYS) */
function say(o: Paints, w: string): void {
  const all = o as Record<string, unknown>, rest: [string, unknown][] = []; delete all.stuff;
  for (const k of Object.keys(all)) if (!['id', 'floor', 'group', 'name', 'c', 'h', 'off', 'yaw', 'rot', 'material', 'albedo', 'emissive', 'rough', 'metal'].includes(k)) { rest.push([k, all[k]]); delete all[k]; }
  all.stuff = w; for (const [k, v] of rest) all[k] = v;
}
/** bring `d` forward in place; returns how many rows and how many boxes and parts now say a word of their own */
export function bake(d: Data): { rows: number; said: number; dropped: number } {
  const was = wasAll(d); let rows = 0, said = 0, dropped = 0;
  for (const [name, m] of Object.entries(d.materials)) { const w = rowWord(name).stuff; if (w !== undefined) { m.stuff = w; rows++; } }
  let k = 0;
  for (const { o } of paintUsers(d)) { const w = was[k++].stuff, m = rowOf(d, o), row = (m !== undefined ? d.materials[m]?.stuff : undefined) ?? 'plain';
    if (w === row) { if (o.stuff !== undefined) { delete o.stuff; dropped++; } } else { say(o, w); said++; } }
  return { rows, said, dropped };
}

if (import.meta.main) {
  const argv = process.argv.slice(2), CHECK = argv.includes('--check'), outAt = argv.indexOf('--out'), OUT = outAt >= 0 ? resolve(argv[outAt + 1]) : null;
  const named = argv.filter((a, i) => !a.startsWith('--') && i !== outAt + 1 || (outAt < 0 && !a.startsWith('--')));
  const files = (argv.includes('--all') ? readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json')).sort().map(f => `${ROOT}/levels/${f}`) : named.map(f => resolve(f))).filter(f => !basename(f).includes('.lint'));   // (a level's lint allowances are no level)
  if (!files.length) { console.error('usage: bun run tools/levels/stuff-bake.ts <level.json …> | --all  [--check] [--out <dir>]'); process.exit(2); }
  if (OUT) mkdirSync(OUT, { recursive: true });
  let todo = 0, bad = 0;
  for (const f of files) {
    const text = readFileSync(f, 'utf8'), d = parseLevelData(text) as Data;
    if (!fromBefore(d)) { console.log(`${basename(f)}: its rows already say what they are made of — left as it is`); if (OUT) writeFileSync(resolve(OUT, basename(f)), text); continue; }
    const canon = serializeLevelData(d as never) === text, st = bake(d), errs = validateLevelData(d as never);
    if (errs.length) { console.error(`${basename(f)}: brought forward it does not validate — ${errs.slice(0, 3).join(' | ')}`); bad++; continue; }
    todo++;
    console.log(`${basename(f)}: ${st.rows} rows say their stuff, ${st.said} boxes and parts say their own${st.dropped ? `, ${st.dropped} dropped a word their row now says` : ''}${canon ? '' : ' — NOT in the canonical text before: more than words will move'}${CHECK ? ' (to do)' : ''}`);
    if (!CHECK) writeFileSync(OUT ? resolve(OUT, basename(f)) : f, serializeLevelData(d as never));
  }
  if (CHECK) console.log(todo ? `${todo} file(s) from before` : 'every file says what its boxes are made of');
  process.exit(bad || (CHECK && todo) ? 1 : 0);
}
