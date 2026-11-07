// THE SWITCH LIST (tools/qa/switches.json): every switch row of the knob tables, its value on the page and its kind. It is WRITTEN, not kept by hand:
//   bun run tools/qa/switch-list.ts make <the port's list>    writes tools/qa/switches.json from the list the Rust port translates from (port-list.json)
//   bun run tools/qa/switch-list.ts same <the port's list>    exit 0 = the file is that list's rows, no more and no fewer · 1 = it is not (the rows that differ
//                                                             are named: one `new` added here is among them until that list has it)
//   bun run tools/qa/switch-list.ts new                       a NEW switch (a yes/no row of the knob tables that is on no list) is added as an open decision
//                                                             at its value on the page: what "a new rule is a switch" means here. Needs no list from outside.
// The port's list (project files, rebuild/rust-sim/port/blitz/port-list.json) is the source: a row changes kind THERE and this file is made again. It carries
// more (who writes a row, why it is open); this file keeps what a tool reads: `row`, `page`, `kind`. Its readers: tools/qa/fold.ts (the settled rows held at
// `page`) and the Rust host's own count of folded rows. A row added by `new` is not in the port's list until its keeper puts it there: `same` names it.
// What holds this file to the GAME is tools/qa/fold-check.ts (every settled row is in the tables and at `page` on the page's stand; every yes/no row of the
// tables is on the list).
import { readFileSync, writeFileSync } from 'node:fs';
import { SWITCH_LIST, SWITCH_KINDS, unlistedYesNo, type SwitchRow } from './fold.ts';

const KINDS: readonly string[] = SWITCH_KINDS;
const WHAT = "Every switch row of the knob tables: its value on the page and its kind. settled = a constant of the game at `page` (tools/qa/fold.ts holds it there); open decision = stays a switch; player's or run-time = stays a variable (`page` = where a fresh page starts). Written by tools/qa/switch-list.ts from the port's switch list (project files, rebuild/rust-sim/port/blitz/port-list.json): do not edit by hand.";
/** the file's text: its head, then one row a line */
function fileText(from: Record<string, unknown>, list: readonly SwitchRow[]): string {
  const rows = list.map(r => { if (!KINDS.includes(r.kind)) throw new Error(`${r.row}: a kind this file does not know ('${r.kind}')`); return { row: r.row, page: r.page, kind: r.kind }; });
  const head = { what: WHAT, from, counts: Object.fromEntries(KINDS.map(k => [k, rows.filter(r => r.kind === k).length])) };
  return `{\n${Object.entries(head).map(([k, v]) => `  ${JSON.stringify(k)}: ${JSON.stringify(v)},`).join('\n')}\n  "rows": [\n${rows.map(r => `    ${JSON.stringify(r)}`).join(',\n')}\n  ]\n}\n`;
}
/** …from the port's list (which names the branch and the step it was built on; this file says only when it was made and how often corrected) */
export const switchListText = (port: { made?: string; changed?: string[]; rows: SwitchRow[] }): string => fileText({ made: port.made ?? '', corrections: (port.changed ?? []).length }, port.rows);

if (import.meta.main) {
  const [mode, path] = process.argv.slice(2);
  if (mode === 'new' && !path) {
    const { standUp, ROOT } = await import('./headless.ts'), { STAND, BY_HAND } = await import('./stand.ts'), { knobRegistry, NOT_KNOBS } = await import(`${ROOT}/src/game/knobs.ts`);
    if (STAND !== 'shipped' || BY_HAND.length) { console.error(`switch-list.ts new: a row's value on the page is read on the page's stand as it opens (no TPS_PROFILE, no word of the stand by hand${BY_HAND.length ? `: this run has ${BY_HAND.join(' ')}` : ''})`); process.exit(2); }
    const log = console.log; console.log = () => {}; try { await standUp(); } finally { console.log = log; }
    const file = JSON.parse(readFileSync(SWITCH_LIST, 'utf8')) as { from: Record<string, unknown>; rows: SwitchRow[] };
    const added: SwitchRow[] = unlistedYesNo(knobRegistry() as Record<string, unknown>, file.rows, NOT_KNOBS).map(r => ({ row: r.row, page: r.value, kind: 'open decision' }));
    if (!added.length) { console.log('tools/qa/switches.json: every yes/no row of the knob tables is on the list already'); process.exit(0); }
    writeFileSync(SWITCH_LIST, fileText({ ...file.from, added_here: [...((file.from.added_here as string[] | undefined) ?? []), ...added.map(r => r.row)] }, [...file.rows, ...added]));
    console.log(`tools/qa/switches.json: ${added.length} new row${added.length > 1 ? 's' : ''} added as an open decision at the page's value: ${added.map(r => `${r.row}=${String(r.page)}`).join(', ')}`);
    console.log("  tell the keeper of the port's switch list (the switch cleanup): `switch-list.ts same` is red until that list has them too");
    process.exit(0);
  }
  if (!['make', 'same'].includes(mode) || !path) { console.error("usage: bun run tools/qa/switch-list.ts make|same <the port's list (port-list.json)>   ·   bun run tools/qa/switch-list.ts new"); process.exit(2); }
  const port = JSON.parse(readFileSync(path, 'utf8')) as { made?: string; changed?: string[]; rows: SwitchRow[] }, want = switchListText(port);
  let have = ''; try { have = readFileSync(SWITCH_LIST, 'utf8'); } catch { /* no file */ }
  if (mode === 'make') {   // (a row `new` added here that the port's list has not yet is KEPT, and said)
    let kept: SwitchRow[] = []; try { const f = JSON.parse(have) as { from?: { added_here?: string[] }; rows: SwitchRow[] }, inPort = new Set(port.rows.map(r => r.row)); kept = f.rows.filter(r => (f.from?.added_here ?? []).includes(r.row) && !inPort.has(r.row)); } catch { /* no file yet */ }
    writeFileSync(SWITCH_LIST, kept.length ? fileText({ made: port.made ?? '', corrections: (port.changed ?? []).length, added_here: kept.map(r => r.row) }, [...port.rows, ...kept]) : want);
    console.log(`tools/qa/switches.json written: ${port.rows.length + kept.length} rows${kept.length ? ` (${kept.length} added here that the port's list does not have yet, kept: ${kept.map(r => r.row).join(', ')})` : ''}`); process.exit(0); }
  if (have === want) { console.log("tools/qa/switches.json is what the port's list makes"); process.exit(0); }
  const rowsOf = (t: string): Map<string, string> => { try { return new Map((JSON.parse(t) as { rows: SwitchRow[] }).rows.map(r => [r.row, `${JSON.stringify(r.page)} ${r.kind}`])); } catch { return new Map(); } };
  const A = rowsOf(have), B = rowsOf(want), diff = [...new Set([...A.keys(), ...B.keys()])].filter(k => A.get(k) !== B.get(k));
  console.log(`tools/qa/switches.json is NOT what the port's list makes: ${diff.length} rows differ${diff.length ? '' : ' (the head or the layout does)'}`);
  for (const k of diff.slice(0, 40)) console.log(`  ${k}: the file ${A.get(k) ?? 'has no such row'} · the port's list ${B.get(k) ?? 'has no such row'}`);
  process.exit(1);
}
