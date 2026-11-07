// THE SETTLED ROWS, FOLDED. A switch row that is settled is a constant of the game: the Rust game has no such row (the branch the page takes was translated,
// the other dropped). Here each settled row of the knob tables is HELD at its value on the page: a read answers that value, a write of any other is REFUSED
// and counted with who wrote it. So whatever a check pins of the SETTLED rows, it runs with them as the page has them, and what it tried is on a list.
// (A row that is not settled, an open decision or a player's own, is still a switch: a pin or a hold over one takes, as it should, and no line lists it.)
//   the list: tools/qa/switches.json (every switch row: its value on the page and its kind; written by `bun run tools/qa/switch-list.ts make <the port's list>`)
//   laid:     by tools/qa/headless.ts as it loads, on the page's stand, when QA_FOLD=1 is in front or the game asked for is the Rust one (TPS_GAME = rust,
//             audit or table). Not on the old stand: that stand IS the other side of these rows. THE WORDS RIDE THE ENVIRONMENT: a child given an
//             environment by name (holds-check's, stand-bisect's) is folded only if it is handed the word.
//   said:     ONE LINE A PROBE on stdout, two spaces then `{"fold":{"probe":…,"red":…,"refused":[{"row","value","writer","n"}],"lost":[…]}}`, by whoever
//             runs the probes: the runner (probes.ts run), or a check file's own main run alone (holds.ts asProbe); see `foldSpeaker`. `writer` is
//             `<file>:<line>`: `tools/…` for a check, `src/…` for the game's or the page's own code, the check that called it after it (see `writer`).
//             What was refused outside any probe (a word given by hand that the harness reads as it loads) has a line of its own, `"outside"` in place
//             of `"probe"`. A child process hands its count to the speaker it is a child of; with none it says its own on STDERR as it ends (a parent
//             that reads the last line of a child's stdout must not find this one).
//             A row that stood OFF its page value as the fold was laid reads its page value from then on: a word given by hand that a module read as
//             it loaded (GLASSFX=off), or a row the page raises only as a game is stood (the cut-pieces switch). The first process of a tree names them
//             in its notice on stderr; a child that has one its parent had not (a word an arm gave it) has it counted, writer `(a word read before the
//             fold)`, and says so on stderr.
//   A WORD GIVEN BY HAND over a settled row (TPS_GEAR=0 in front of a folded run) is refused wherever it is read: as the harness loads (the `outside`
//             line), as each game is stood and in each child (the PROBE's line, writer `src/host/standup.ts:… (from <the check>)`).
//   FOR ONE CHECK AT A TIME: the laid fold can be OFF for a check and ON for the next (`foldFor`). Off, every held row is plain in effect: a read answers
//             what was last written while off, a write takes, nothing is counted or named lost, and the check runs as with no fold laid; its line says
//             `"folded":false`. On, a row answers its page value and a write is refused and leaves NO trace in what the row answers while off. So
//             nothing passes between a folded check and an unfolded one in either direction. For a host that stands some checks on the Rust game
//             (folded: that game has the rows as constants) and keeps the rest on the TypeScript tick as they are until they are restaged.
//             · THE FOLD STARTS OFF where it is asked for check by check: under TPS_GAME=table with no QA_FOLD=1, and under QA_FOLD_ONLY (so what the
//               harness writes as it loads, a word given by hand, is there for the unfolded checks as with no word; a folded check does not have it,
//               and that is not said). Under QA_FOLD=1 alone, rust and audit it starts on. The notice on stderr says which.
//             · CALL `foldFor` BETWEEN CHECKS, before the next check's fence opens (probes.ts run() → holds.ts asProbe). The fence takes the mode as it
//               opens and puts it back FIRST as it closes, so its own put-back of the tables runs in the mode it copied them in. A check that
//               switches the fold itself inside its fence is not held: what it wrote in the other mode stays.
//             · It rides to a child started after the call (QA_FOLD_FOR = 1 or 0; another spelling stops the tool). Where no fold is laid it is nothing.
//             · BY HAND, THE RUNNER'S WORD: `QA_FOLD_ONLY=<probe,probe…>` beside QA_FOLD=1 makes probes.ts fold the named probes alone (a probe's own
//               name; one that no probe of the run had is an error, and so is a list that names nothing). A check file run alone does not read it:
//               there the fold stays off. A run so mixed reads its unfolded reds against the list of expected reds as ever, and names its folded
//               reds apart; a red that is no probe's name (a family's banner) is read as before.
//             · NOT HELD while off: an old branch object put back by reference after a look laid its replacement (the two share the row's cell).
//   A FOLDED PROBE IS NOT THE STAND AS IT OPENS while a check still pins a settled row: the runner reads no folded probe's red against the list of
//             expected reds (a run folded whole says so in one line).
//   beside:   QA_VERDICTS=1 on a run WITHOUT the fold makes the runner (the outermost one) print `{"plain":{"probe":…,"red":…}}` after each probe, to lay
//             the two runs side by side.
// What it cannot hold, and says instead:
//   · a row that is already a property with a getter or a setter of its own: left alone (`FOLD.skipped`; fold-check (1) is red on one).
//   · a table, or a branch of one, replaced whole (`T.a = {…}`): the rows under it are plain again and a write to them TAKES, uncounted, until `foldLost()`
//     finds them, lays them again and names them (`lost`). The fence round a probe (holds.ts asProbe) looks before it puts the tables back, so a probe's line
//     names the row with the value it was left at.
//   · a row read into a local before the fold was laid (a module that copies a row as it loads): the copy is the module's own.
//   · a switch kept as 0 / 1 is a number to the list's walk: only the ones the two stands differ in are on the list (MOVES.on is, MOVES.recoil is not).
//   · a child killed before it ends hands over nothing; neither does one whose speaker could make no folder for the counts (it says its own, as a top would).
// WHAT A REFUSED WRITE MEANS FOR A CHECK: its staging may not have formed. A check that pinned a neighbour off to get a fight, and counts what happened in
// that fight, can pass with nothing to count. Green under the fold with a refused write is NOT a re-aimed check until its lines are read against the same
// run without the fold (the same words and numbers = the pin made no difference; anything else is read).
import { readFileSync, writeFileSync, readdirSync, mkdtempSync, renameSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { resolve, basename } from 'node:path';
import { DEFAULT_WORD } from '../blitz/word.ts';

export const SWITCH_KINDS = ['settled', 'open decision', "player's or run-time"] as const;
export type SwitchKind = typeof SWITCH_KINDS[number];
export interface SwitchRow { row: string; page: unknown; kind: SwitchKind }
export interface Refused { row: string; value: unknown; writer: string; n: number }

export const SWITCH_LIST = `${import.meta.dir}/switches.json`;
export const switchRows = (path = SWITCH_LIST): SwitchRow[] => (JSON.parse(readFileSync(path, 'utf8')) as { rows: SwitchRow[] }).rows;
export const settledOf = (rows: readonly SwitchRow[]): Record<string, unknown> => Object.fromEntries(rows.filter(r => r.kind === 'settled').map(r => [r.row, r.page]));
/** a settled yes/no row `depth` names deep (2 = TABLE.key), for a check that needs one to write to: the first whose branch holds no other settled row */
export function settledYesNo(rows: readonly SwitchRow[], depth: number): SwitchRow | undefined {
  const st = rows.filter(r => r.kind === 'settled'), branch = (r: SwitchRow): string => r.row.slice(0, r.row.lastIndexOf('.'));
  return st.find(r => typeof r.page === 'boolean' && r.row.split('.').length === depth && (depth === 2 || st.filter(o => branch(o) === branch(r)).length === 1));
}

/** the fold is asked for: by its own word (QA_FOLD=1; 0 or nothing = not asked by this word; any other spelling stops the tool, as the harness's other
 *  words do), or by the word that stands the Rust game (TPS_GAME = rust, audit or table: a constant there is a constant here, whatever QA_FOLD says. That
 *  word is the Rust host's own: nothing else in this tree reads it yet, and what else it may be spelt is that host's to say and to refuse) */
export function foldWanted(env: Record<string, string | undefined> = process.env): boolean {
  if (!['1', '0', '', undefined].includes(env.QA_FOLD)) throw new Error(`QA_FOLD=${env.QA_FOLD}: 1 lays the fold of the settled switch rows (tools/qa/fold.ts), 0 or nothing does not`);
  // (the word with nothing in front is the Rust host's to say too: tools/blitz/word.ts DEFAULT_WORD, the one line that makes `table` the default)
  return env.QA_FOLD === '1' || ['rust', 'audit', 'table'].includes(env.TPS_GAME ?? DEFAULT_WORD);
}

export const FOLD = { on: false, rows: 0, skipped: [] as string[], missing: [] as string[], offAtLay: [] as { row: string; value: unknown }[] };
interface Laid { row: string; root: Record<string, unknown>; path: string[]; key: string; held: unknown; now: unknown; gone?: boolean }   // (`now`: what the row answers while the fold is off)
const REFUSED = new Map<string, Refused>(), TAKEN = new Map<string, number>(), LAID: Laid[] = [], LOST: string[] = [];
/** a yes/no row is 0 / 1 in some tables and false / true in others: the same answer either way */
export const sameValue = (a: unknown, b: unknown): boolean => a === b || (typeof a === 'boolean' && typeof b === 'number' && (b === 0 || b === 1) && a === !!b) || (typeof b === 'boolean' && typeof a === 'number' && (a === 0 || a === 1) && b === !!a);
const text = (v: unknown): string => { try { return typeof v === 'object' ? JSON.stringify(v) : String(v); } catch { return Object.prototype.toString.call(v); } };   // (never a throw out of a row's setter)
/** a written value as a line can carry it: itself where it is a plain yes/no, number, string or null, else its text */
const kept = (v: unknown): unknown => v === null || typeof v === 'boolean' || typeof v === 'string' || (typeof v === 'number' && Number.isFinite(v)) ? v : text(v);
const ROOT = resolve(import.meta.dir, '../..'), FRAME = new RegExp(`${ROOT.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}/([^\\s:)]+):(\\d+)`);
/** who wrote: the first frame outside this file, `<file>:<line>` from the checkout's root (`src/…` = the game's or the page's own code, `tools/…` = a check
 *  or a tool). Where that is the game's code, the first frame of a check after it follows, the harness's own files passed over (`src/game/journal.ts:380
 *  (from tools/qa/x-probes.ts:88)`: a recording's rows read back; `src/host/standup.ts:273 (from tools/qa/x-check.ts:40)`: a word of the stand read as that
 *  check stood a game. So `src/…` alone, with no `from`, is the page's own code with no check in sight). A script outside the checkout is
 *  `(outside) <file>:<line>`. */
const HARNESS = /^tools\/qa\/(headless|holds|stand|fold)\.ts:/;
function writer(): string {
  const E = Error as unknown as { stackTraceLimit: number }, limit = E.stackTraceLimit; E.stackTraceLimit = 40; const stack = new Error().stack ?? ''; E.stackTraceLimit = limit;
  let first = '';
  for (const l of stack.split('\n').slice(1)) {
    const m = FRAME.exec(l), at = m ? `${m[1]}:${m[2]}` : (/([^\s(]+\.[cm]?[jt]s):(\d+)/.exec(l) ?? []).slice(1, 3).join(':');
    if (!at || (m && m[1] === 'tools/qa/fold.ts')) continue;
    const here = m ? at : `(outside) ${basename(at)}`;
    if (!first) { first = here; if (!here.startsWith('src/')) return first; }
    else if (here.startsWith('tools/') && !HARNESS.test(here)) return `${first} (from ${here})`;
  }
  return first || '?';
}
const count = (row: string, value: unknown, w = writer()): void => { const k = `${row}\u0000${text(value)}\u0000${w}`, r = REFUSED.get(k); if (r) r.n++; else REFUSED.set(k, { row, value: kept(value), writer: w, n: 1 }); };
/** the object a row's key lives on, under the table `root` */
export const hostOf = (root: unknown, path: readonly string[]): Record<string, unknown> | null => { let h = root; for (const k of path) h = (h as Record<string, unknown> | null)?.[k]; return h && typeof h === 'object' ? h as Record<string, unknown> : null; };
let through = false;   // (the check's red path, FOLD_BREAK=1: a write is counted and TAKES)
/** the laid fold is on (see the head, FOR ONE CHECK AT A TIME). As the process starts: QA_FOLD_FOR where a parent left it; else on under QA_FOLD=1 with no
 *  QA_FOLD_ONLY; else off where the fold is asked for check by check (QA_FOLD_ONLY, TPS_GAME=table); else on. */
export function foldStartsOn(env: Record<string, string | undefined> = process.env): boolean {
  if (!['1', '0', '', undefined].includes(env.QA_FOLD_FOR)) throw new Error(`QA_FOLD_FOR=${env.QA_FOLD_FOR}: 1 = the laid fold is on, 0 = off (tools/qa/fold.ts foldFor writes it for a child)`);
  if (env.QA_FOLD_FOR) return env.QA_FOLD_FOR === '1';
  if (env.QA_FOLD_ONLY) return false;
  return env.QA_FOLD === '1' || env.TPS_GAME !== 'table';
}
let active = foldStartsOn();
function lay(L: Laid, host: Record<string, unknown>): void {
  const d = Object.getOwnPropertyDescriptor(host, L.key)!;
  Object.defineProperty(host, L.key, { enumerable: d.enumerable, configurable: true, get: () => !active || through ? L.now : L.held,
    set: (v: unknown) => { if (!active) { L.now = v; return; } if (!sameValue(v, L.held)) count(L.row, v); if (through) L.now = v; } });
}
/** the laid fold on or off from here on. Nothing where no fold is laid, and nothing for `null` (the mode of a process with none laid). */
export function foldFor(on: boolean | null): void { if (!FOLD.on || on === null) return; active = on; process.env.QA_FOLD_FOR = on ? '1' : '0'; }
/** the mode now: on, off, or null where no fold is laid (what a fence takes as it opens and hands back to foldFor as it closes) */
export const foldMode = (): boolean | null => FOLD.on ? active : null;
/** a fold is laid AND on: the settled rows answer their page value now */
export const foldActive = (): boolean => FOLD.on && active;

/** lay the fold over the registry's tables: `settled` = row → its value on the page */
export function foldSettled(reg: Record<string, object>, settled: Record<string, unknown>, letThrough = false): void {
  through = letThrough;
  for (const [row, page] of Object.entries(settled)) {
    const [t, ...rest] = row.split('.'), key = rest[rest.length - 1], path = rest.slice(0, -1), root = reg[t] as Record<string, unknown> | undefined, host = root ? hostOf(root, path) : null;
    if (!root || !host || key === undefined || !(key in host)) { FOLD.missing.push(row); continue; }
    const d = Object.getOwnPropertyDescriptor(host, key);
    if (!d || d.get || d.set || !d.configurable) { FOLD.skipped.push(row); continue; }
    // (the table's own type is kept: a list that says true of a row the table keeps as 1 holds 1)
    const held = typeof d.value === typeof page ? page : typeof d.value === 'number' && typeof page === 'boolean' ? +page : typeof d.value === 'boolean' && typeof page === 'number' ? !!page : page;
    if (!sameValue(d.value, held)) FOLD.offAtLay.push({ row, value: d.value });   // (it stood off its page value already: a word given by hand that its module read as it loaded, or a row the page raises only as a game is stood. Said once, by headless.ts)
    const L: Laid = { row, root, path, key, held, now: d.value }; lay(L, host); LAID.push(L); FOLD.rows++;
  }
  if (FOLD.on) return; FOLD.on = true;
  // what nobody took by the end: to the speaker this process is a child of, else said here, on stderr
  process.on('exit', () => {
    const mine = process.env.QA_FOLD_LOG_OWNER === String(process.pid), dir = process.env.QA_FOLD_LOG;
    foldLost(); const late = mine ? foldLogTake() : { refused: [], lost: [] }, refused = [...foldTake(), ...late.refused], lost = [...LOST.splice(0), ...late.lost];   // (the speaker: what a child handed in after its last line)
    if (refused.length || lost.length) { let handed = false;
      // (a file of its own, whole before it has its name: the speaker never reads half of one)
      if (dir && !mine) { try { const f = `${dir}/${process.pid}`; writeFileSync(`${f}.tmp`, [...refused.map(r => JSON.stringify(r)), ...lost.map(l => JSON.stringify({ lost: l }))].join('\n') + '\n'); renameSync(`${f}.tmp`, `${f}.jsonl`); handed = true; } catch { /* said below */ } }
      if (!handed) console.error(`  ${JSON.stringify({ fold: { process: (process.argv[1] ?? '').replace(`${ROOT}/`, ''), refused, ...(lost.length ? { lost } : {}) } })}`); }
    if (dir && mine) { try { rmSync(dir, { recursive: true, force: true }); } catch { /* gone already */ } } });
}
/** the rows of `FOLD.offAtLay` as one text, and (a child of a folded process) the ones its parent did not have counted as refused: a word given to THIS
 *  process that a module read as it loaded. The first process of a tree has nothing to compare with: its notice names them all (headless.ts). */
export const offAtLayText = (): string => FOLD.offAtLay.map(o => `${o.row}=${text(o.value)}`).join(', ');
export function foldBefore(parents: string): string[] {
  if (!active) return [];   // (the fold is off for this child: the row is plain there, and the word took)
  const had = new Set(parents.split(', ')), more = FOLD.offAtLay.filter(o => !had.has(`${o.row}=${text(o.value)}`));
  for (const o of more) count(o.row, o.value, '(a word read before the fold)');
  return more.map(o => `${o.row}=${text(o.value)}`);
}
/** the rows whose hold is gone (their table, or a branch of it, was replaced whole since): laid again on what stands there now, named here and in the next
 *  line. A row whose branch is not there at all is named once, `(gone)`, and looked for again each time. */
export function foldLost(): string[] {
  const lost: string[] = [];
  for (const L of LAID) { const host = hostOf(L.root, L.path); if (!host || !(L.key in host)) { if (active && !L.gone) { L.gone = true; lost.push(`${L.row} (gone)`); } continue; }
    L.gone = false; const d = Object.getOwnPropertyDescriptor(host, L.key); if (!d || d.get) continue;
    // (while the fold is off a branch replaced whole is the check's own business: the row takes what stands there and nothing is named. While it is on,
    // what was written into the plain row is a pin like any other: named, and it leaves no trace in what the row answers while off.)
    if (active) lost.push(sameValue(d.value, L.held) ? L.row : `${L.row} (was at ${text(d.value)})`); else L.now = d.value;
    if (d.configurable) lay(L, host); }
  LOST.push(...lost); return lost;
}
/** everything refused that no earlier call returned */
export function foldTake(): Refused[] {
  const out: Refused[] = [];
  for (const [k, r] of REFUSED) { const d = r.n - (TAKEN.get(k) ?? 0); if (d > 0) { out.push({ row: r.row, value: r.value, writer: r.writer, n: d }); TAKEN.set(k, r.n); } }
  return out;
}
/** ONE LINE A PROBE COMES FROM ONE SPEAKER: the outermost that runs probes. Down a tree of processes it is the first to ask (it leaves a folder under the
 *  system's temp for its descendants to hand their counts to, a file each, taken after each probe and removed as the speaker ends; a speaker killed by a
 *  signal leaves it behind). Inside a process it is the first asker: the runner's run() ('runner') before the fence a probe is run in (holds.ts asProbe,
 *  'probe': a check file's own main, run alone). Everyone else says nothing and hands on as it ends. */
let speaker: string | null = null;
export function foldSpeaker(who: 'runner' | 'probe'): boolean {
  if (!FOLD.on) return false;
  if (speaker === null) {
    if (process.env.QA_FOLD_LOG) speaker = '(above)';
    else { speaker = who; try { process.env.QA_FOLD_LOG = mkdtempSync(`${tmpdir()}/qa-fold-`); process.env.QA_FOLD_LOG_OWNER = String(process.pid); } catch { /* no folder: a child says its own */ } } }
  return speaker === who;
}
function foldLogTake(): { refused: Refused[]; lost: string[] } {
  const dir = process.env.QA_FOLD_LOG, out = { refused: [] as Refused[], lost: [] as string[] }; if (!dir || process.env.QA_FOLD_LOG_OWNER !== String(process.pid)) return out;
  try { for (const f of readdirSync(dir)) { if (!f.endsWith('.jsonl')) continue; const all = readFileSync(`${dir}/${f}`, 'utf8'); rmSync(`${dir}/${f}`, { force: true });
    for (const l of all.split('\n')) { if (!l) continue; try { const r = JSON.parse(l) as Refused & { lost?: string }; if (typeof r.lost === 'string') out.lost.push(r.lost); else out.refused.push(r); } catch { /* not a line of ours */ } } } } catch { /* no folder */ }
  return out;
}
/** the true/false rows of the knob tables that are on no list (TABLE.a.b), with their value now: a new switch. `notKnobs` = the registry's own table of
 *  keys that are data and no knob (src/game/knobs.ts NOT_KNOBS). Plain objects only are walked, as the registry's own copy walks. One walk for the check
 *  that says so (fold-check (2)) and the command that mends it (switch-list.ts new). A switch kept as 0 / 1 is a number here and is not found. */
export function unlistedYesNo(reg: Record<string, unknown>, rows: readonly SwitchRow[], notKnobs: Record<string, readonly string[] | undefined>): { row: string; value: boolean }[] {
  const on = new Set(rows.map(r => r.row)), out: { row: string; value: boolean }[] = [], plain = (o: unknown): o is Record<string, unknown> => !!o && typeof o === 'object' && [Object.prototype, null].includes(Object.getPrototypeOf(o));
  const walk = (name: string, T: unknown, skip: readonly string[]): void => { if (!plain(T)) return;
    for (const [k, v] of Object.entries(T)) { if (skip.includes(k)) continue; const row = `${name}.${k}`; if (typeof v === 'boolean') { if (!on.has(row)) out.push({ row, value: v }); } else walk(row, v, []); } };
  for (const [t, T] of Object.entries(reg)) walk(t, T, notKnobs[t] ?? []);
  return out;
}
/** everything refused or lost since the last line, here and in the children: one entry a row, value and writer, in one order */
function gather(): { refused: Refused[]; lost: string[] } {
  foldLost(); const kids = foldLogTake(), by = new Map<string, Refused>();
  for (const r of [...foldTake(), ...kids.refused]) { const k = `${r.row}\u0000${text(r.value)}\u0000${r.writer}`, o = by.get(k); if (o) o.n += r.n; else by.set(k, { ...r }); }
  return { refused: [...by.entries()].sort((a, b) => a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0).map(e => e[1]), lost: [...new Set([...LOST.splice(0), ...kids.lost])].sort() };
}
/** one probe's line: what it, and its children, tried to write over a settled row */
export function foldLine(probe: string, red: boolean): string { const G = gather(); return `  ${JSON.stringify({ fold: { probe, red, ...(active ? {} : { folded: false }), refused: G.refused, ...(G.lost.length ? { lost: G.lost } : {}) } })}`; }
/** what was refused or lost outside any probe since the last line (as the harness loaded, between two probes): a line of its own, '' when there is nothing */
export function foldOutside(where: string): string { const G = gather(); return G.refused.length || G.lost.length ? `  ${JSON.stringify({ fold: { outside: where, refused: G.refused, ...(G.lost.length ? { lost: G.lost } : {}) } })}` : ''; }
