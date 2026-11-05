// THE PIN (tools/qa/headless.ts RETIRE_PIN; docs/agents/qa.md "Retiring a switch", THE SORTING): the rows a list names are held at its values whatever a
// tool writes, a row a level down without its neighbours being touched, a row marked `hold` not at all, and a row no table has is said.
//   bun run tools/retire/pin-check.ts        exit 0 = all hold · 1 = one does not   (TPS_PROFILE=shipped in front = asked on the page's stand)
//   PINCHECK_BREAK=1 in front = the list handed over empty: must FAIL (nothing is held, so the lines that ask for a held row go red).
// It stands no game: it loads the harness in a child, once with no list (to pick its rows from the tables as they are today — any row may retire, so none is
// named here) and once with the list, and in the child writes every picked row to another value and prints what the tables then hold.
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

type Picked = { plain: string; deep: string | null; held: string; other: Record<string, unknown> };
const other = (v: unknown): unknown => typeof v === 'boolean' ? !v : typeof v === 'number' ? v + 1 : `${String(v)}x`;
const get = (R: Record<string, any>, row: string): unknown => row.split('.').reduce((o: any, k) => (o && typeof o === 'object' ? o[k] : undefined), R);

if (process.argv[2] === 'child') {
  // (the child: the harness loaded as any check loads it, the pin in its environment or not)
  await import('../qa/headless.ts');
  const { knobRegistry } = await import('../../src/game/knobs.ts'); const R = knobRegistry() as Record<string, any>;
  if (process.argv[3] === 'pick') {
    const flat: string[] = [], deep: string[] = [];
    for (const [t, o] of Object.entries(R)) for (const [k, v] of Object.entries(o as object)) { if (typeof v === 'boolean') flat.push(`${t}.${k}`);
      else if (v && typeof v === 'object' && !Array.isArray(v)) for (const [k2, v2] of Object.entries(v)) if (typeof v2 === 'boolean' || typeof v2 === 'number') deep.push(`${t}.${k}.${k2}`); }
    const pick: Picked = { plain: flat[0], deep: deep[0] ?? null, held: flat[1], other: {} };
    for (const r of [pick.plain, pick.deep, pick.held]) if (r) pick.other[r] = other(get(R, r));
    console.log(`PICK ${JSON.stringify(pick)}`);
  } else {
    const pick = JSON.parse(readFileSync(process.argv[3], 'utf8')) as Picked, before: Record<string, unknown> = {}, after: Record<string, unknown> = {};
    const parentOf = (row: string): any => get(R, row.split('.').slice(0, -1).join('.')), nb = pick.deep ? Object.keys(parentOf(pick.deep) ?? {}).length : 0;
    for (const r of [pick.plain, pick.deep, pick.held]) { if (!r) continue; before[r] = get(R, r); const o = parentOf(r), k = r.split('.').pop()!; o[k] = other(before[r]); after[r] = get(R, r); }   // (written to another value than it reads: a held row reads the same after)
    console.log(`AFTER ${JSON.stringify({ before, after, deepIsObject: pick.deep ? typeof parentOf(pick.deep) === 'object' && Object.keys(parentOf(pick.deep)).length === nb && nb >= 1 : null })}`);
  }
  process.exit(0);
}

const SELF = resolve(import.meta.dir, 'pin-check.ts'), BREAK = process.env.PINCHECK_BREAK === '1';
const child = (args: string[], env: Record<string, string>): { out: string; err: string; code: number } => { const p = Bun.spawnSync(['bun', 'run', SELF, 'child', ...args], { env: { PATH: process.env.PATH ?? '', HOME: process.env.HOME ?? '', ...(process.env.TPS_PROFILE ? { TPS_PROFILE: process.env.TPS_PROFILE } : {}), ...env }, stdout: 'pipe', stderr: 'pipe' }); return { out: p.stdout.toString(), err: p.stderr.toString(), code: p.exitCode ?? -1 }; };
let fails = 0; const line = (ok: boolean, label: string, detail = ''): void => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${label}${detail ? ` — ${detail}` : ''}`); if (!ok) fails++; };
const dir = mkdtempSync(join(tmpdir(), 'pin-check-')), picked = child(['pick'], {}), pm = /^PICK (.*)$/m.exec(picked.out);
if (!pm) { console.log(`  FAIL  the harness loaded in a child and named its rows — exit ${picked.code}: ${(picked.err || picked.out).trim().split('\n').slice(-2).join(' / ').slice(0, 200)}`); console.log('=== pin-check: 1 FAIL ==='); process.exit(1); }
const pick = JSON.parse(pm[1]) as Picked, pickFile = join(dir, 'pick.json'), pinFile = join(dir, 'pin.json'); writeFileSync(pickFile, JSON.stringify(pick));
// the list: each picked row at ANOTHER value than the table has (so a held row shows), the third marked `hold`, and a row no table has
const rows = [{ row: pick.plain, value: pick.other[pick.plain] }, ...(pick.deep ? [{ row: pick.deep, value: pick.other[pick.deep] }] : []), { row: pick.held, value: pick.other[pick.held], hold: true }, { row: 'NO_SUCH_TABLE.row', value: true }];
writeFileSync(pinFile, JSON.stringify(BREAK ? [] : rows));
const traceFile = join(dir, 'trace.jsonl'); writeFileSync(traceFile, '');
const run = child([pickFile], { RETIRE_PIN: pinFile, LINES_TRACE_OUT: traceFile }), am = /^AFTER (.*)$/m.exec(run.out), A = am ? JSON.parse(am[1]) as { before: Record<string, unknown>; after: Record<string, unknown>; deepIsObject: boolean | null } : null;
line(!!A && A.before[pick.plain] === pick.other[pick.plain] && A.after[pick.plain] === pick.other[pick.plain], 'a listed row reads the list\'s value as the harness loads, and after a tool has written it', A ? `${pick.plain}: listed ${JSON.stringify(pick.other[pick.plain])} · read ${JSON.stringify(A.before[pick.plain])} · after the write ${JSON.stringify(A.after[pick.plain])}` : (run.err || run.out).trim().slice(-200));
if (pick.deep) line(!!A && A.before[pick.deep] === pick.other[pick.deep] && A.after[pick.deep] === pick.other[pick.deep] && A.deepIsObject === true, 'a row a level down is held by its whole name, and the rows beside it are still there', A ? `${pick.deep}: listed ${JSON.stringify(pick.other[pick.deep])} · read ${JSON.stringify(A.before[pick.deep])} · after the write ${JSON.stringify(A.after[pick.deep])} · its part still a table of its rows ${A.deepIsObject}` : '');
else console.log('  (no table has a row a level down today: that line is not asked)');
line(!!A && A.before[pick.held] !== pick.other[pick.held] && A.after[pick.held] !== A.before[pick.held], 'a row marked `hold` in the list is not held: it reads as the table has it and a tool\'s write takes', A ? `${pick.held}: read ${JSON.stringify(A.before[pick.held])} · after the write ${JSON.stringify(A.after[pick.held])}` : '');
line(/RETIRE_PIN: 1 rows are in no registered table and are not held: NO_SUCH_TABLE\.row/.test(run.err), 'a row no table has is said, and the rest are held all the same', run.err.trim().split('\n').filter(l => /RETIRE_PIN/.test(l)).join(' ').slice(0, 140) || 'nothing said');
{ const want = rows.filter(r => !(r as { hold?: boolean }).hold).length, mark = existsSync(traceFile) ? readFileSync(traceFile, 'utf8').trim() : '';
  line(mark === JSON.stringify({ v: 'PIN', text: `${want - 1} of ${want} rows held`, frames: [] }), 'a run that keeps a trace (tools/retire/lines.ts) is marked with how many of the list\'s rows were held', mark.slice(0, 120) || 'no mark'); }
rmSync(dir, { recursive: true, force: true });
console.log(fails ? `=== pin-check: ${fails} FAIL ===` : '=== pin-check: ALL PASS ===');
process.exit(fails ? 1 : 0);
