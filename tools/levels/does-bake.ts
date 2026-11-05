// BRING A LEVEL FILE FORWARD: every box says what it DID.
// A level file written before 2026-11-05 says nothing of what its boxes are to a round, a charge and a blast: the rules read that off each box's colour.
// They read the box's word now (src/scene/materials.ts, Box.does), so such a file would stand with every box on the default row: no wall a charge cuts,
// no cover, nothing a round goes through. This writes the words in, from the old rule kept frozen in ./doesWas.ts:
//   · each material ROW says what its colour did (the row of the table of materials);
//   · each box, shape's piece and prop's part says its own word ONLY where its colour did another thing than its row now says.
// The diff of a file is `"does"` words and nothing more, and every box does what it did (tools/qa/does-audit.ts counts it).
// A FILE THAT ALREADY SAYS (a row or a box with `does`) IS LEFT ALONE: its words are somebody's, and the old rule knows nothing of them.
//   bun run tools/levels/does-bake.ts levels/some.json …   write each file in place
//   bun run tools/levels/does-bake.ts --all                 every levels/*.json
//   … --check       write nothing; exit 1 if a file from before is still to be brought forward (tools/qa/does-check.ts asks the same of every file)
// A LEVEL WITH A GENERATOR IS NOT MENDED HERE: its export writes the words (tools/levels/export.ts). This is the bridge for a file laid without one: a
// level saved from the editor before that day, a file of a branch cut before it.
import { basename, resolve } from 'node:path';
import { readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { wordWas } from './doesWas.ts';
const ROOT = resolve(import.meta.dir, '../..');
const { DOES_DEFAULT } = await import(`${ROOT}/src/scene/materials.ts`);
const { parseLevelData, serializeLevelData, validateLevelData, KEYS } = await import(`${ROOT}/src/scene/levelData.ts`);

type Paints = { material?: string; albedo?: number[]; does?: string };
type Data = { materials: Record<string, { albedo: number[]; does?: string }>; boxes: (Paints & { shape?: { with?: Paints[] } })[]; props: { parts: Paints[] }[] };
/** a file from before says nothing of what anything does */
export const fromBeforeDoes = (d: Data): boolean => !Object.values(d.materials).some(m => m.does !== undefined) && !d.boxes.some(b => b.does !== undefined) && !d.props.some(p => p.parts.some(q => q.does !== undefined));
/** put the key where the format keeps it (levelData.ts KEYS: straight after `stuff`) */
function keyed<T extends object>(o: T, order: readonly string[]): T { const out: Record<string, unknown> = {}; for (const k of order) if ((o as Record<string, unknown>)[k] !== undefined) out[k] = (o as Record<string, unknown>)[k]; for (const k of Object.keys(o)) if (!(k in out)) out[k] = (o as Record<string, unknown>)[k]; for (const k of Object.keys(o)) delete (o as Record<string, unknown>)[k]; return Object.assign(o, out); }
export function bake(d: Data): { rows: number; said: number } {
  let rows = 0, said = 0;
  for (const m of Object.values(d.materials)) { const w = wordWas(m.albedo); if (w !== DOES_DEFAULT) { m.does = w; keyed(m, KEYS.material); rows++; } }
  const one = (o: Paints, order: readonly string[]) => {
    if (o.material === undefined && o.albedo === undefined) return;   // (a shape's piece that wears its box's paint does what its box does)
    const row = o.material !== undefined ? d.materials[o.material] : undefined, colour = o.albedo ?? row!.albedo, w = wordWas(colour), rows = row?.does ?? DOES_DEFAULT;
    if (w !== rows) { o.does = w; keyed(o, order); said++; }
  };
  for (const b of d.boxes) { one(b, KEYS.box); for (const w of b.shape?.with ?? []) one(w, [...KEYS.box, 'off', 'round', 'arc', 'cut', 'old']); }
  for (const p of d.props) for (const q of p.parts) one(q, KEYS.part);
  return { rows, said };
}
if (import.meta.main) {
  const argv = process.argv.slice(2), CHECK = argv.includes('--check');
  const files = argv.includes('--all') ? readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).sort().map(f => `${ROOT}/levels/${f}`) : argv.filter(a => !a.startsWith('--')).map(a => resolve(a));
  if (!files.length) { console.error('usage: bun run tools/levels/does-bake.ts <level.json …> | --all   [--check]'); process.exit(2); }
  let owed = 0;
  for (const f of files) {
    const d = parseLevelData(readFileSync(f, 'utf8')) as Data;
    if (!fromBeforeDoes(d)) { console.log(`${basename(f)}: it already says what its boxes do — left as it is`); continue; }
    owed++; const st = bake(d), errs = validateLevelData(d as never);
    if (errs.length) { console.error(`${basename(f)}: ${errs.length} error(s) after the words went in — ${errs.slice(0, 3).join(' | ')}`); process.exit(1); }
    console.log(`${basename(f)}: FROM BEFORE — ${st.rows} rows and ${st.said} boxes, pieces and parts ${CHECK ? 'would say' : 'now say'} what they did`);
    if (!CHECK) writeFileSync(f, serializeLevelData(d as never));
  }
  process.exit(CHECK && owed ? 1 : 0);
}
