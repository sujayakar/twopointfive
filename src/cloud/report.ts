// ONE WAY TO SAY "THIS WENT WRONG" (the maintainer, 10-27: "let's do an audit of all our errors and have them use some reusable error reporter that sends up
// to convex"). Any file may call it; it imports nothing, never throws, and costs a headless run nothing.
//   reportError(where, err, data?)     an error: printed to the console WORD FOR WORD as the `console.error(err)` it replaces (page tools and checks
//                                      read those lines; `data` is the record's, never printed), and handed on
//   reportWarning(where, err, data?)   the same for something the game went on from (a fallback taken, a thing not built on this machine). It is what a
//                                      catch that said nothing is given, so it PRINTS AT ITS FIRST COMING ONLY: a catch on a timer or in a loop
//                                      does not fill the console
//   reportErrorLine(where, ...said) · reportWarningLine(where, ...said)   a line that was `console.error(a, b)` / `console.warn(a, b)`, moved here with
//                                      all its parts: printed as it was, every time, and reported under its place
//   `where`  a short, stable name of the place, dots between its parts ('gpu.splitrays.buffer', 'replay.keep.write'): what the reports are grouped by
//   `err`    what was caught, or a sentence;  `data`  a few numbers and short words that say what the place knew (sizes, a label, a count)
// WHERE IT GOES: to whoever asked for the reports (`reportSink`: the backend's side, src/cloud/cloud.ts, which writes each as one of the visit's `error`
// records — the first coming of each, a count after that). Until somebody asks, the first `REPORT.held` errors and `REPORT.heldWarnings` warnings wait
// here (each with places of their own: a boot that warns a lot does not push its one error out); a build with no backend holds those few and that is all.
// THE NET UNDER IT (`tapConsole`, the game page calls it once): every `console.error` and `console.warn` the page makes is a report too, under the where
// 'console.error' / 'console.warn', so a place nobody has moved to `reportError` is still seen. What it cannot catch is a catch that says nothing.
// WHAT IT COSTS: a report is MADE once (its stack taken, its words put together). The same line said again, whatever numbers it carries, is handed on as
// the report it already was, and a page makes `REPORT.makes` at most: a line printed every frame pays for its first coming and no other.
// A REPORT IS NOT A CRASH: a page that died says so on its red card (`cloudFatal`); an error nothing caught reaches the records by the window's own
// listeners (cloud.ts). This is for what WAS caught.

export type ReportLevel = 'error' | 'warning';
export interface Report { where: string; message: string; stack: string[]; level: ReportLevel; data?: Record<string, unknown> }
/** `held` / `heldWarnings`: errors / warnings kept while nobody has asked for them · `stack`: lines of a stack kept · `message`: characters of a message
 *  kept · `keys`: of an object printed beside a message, the keys said · `makes`: reports a page makes, at most (the rest of its lines print and that is all) */
export const REPORT = { held: 32, heldWarnings: 16, stack: 6, message: 300, keys: 8, makes: 200 };

let sink: ((r: Report) => void) | null = null, inside = false, heldErrors = 0, heldWarnings = 0;
const held: Report[] = [];
/** the reports this page has made, by kind, place and the line's first words with their numbers folded */
const made = new Map<string, Report>();
const madeKey = (level: ReportLevel, where: string, first: unknown): string => {
  const t = typeof first === 'string' ? first : first instanceof Error ? first.message : '';
  return t ? `${level}|${where}|${t.slice(0, 160).replace(/0x[0-9a-f]+|\d+/gi, '#')}` : '';
};
/** one line's report: the one already made for it, or a new one while the page may still make any (null = neither) */
const once = (where: string, level: ReportLevel, parts: unknown[], data: Record<string, unknown> | undefined, skip: number): Report | null => {
  const k = madeKey(level, where, parts[0]), old = k ? made.get(k) : undefined;
  if (old) return sink ? old : null;   // (while nobody has asked, the first coming is the one that waits)
  if (made.size >= REPORT.makes) return null;
  const r = make(where, level, parts, data, skip + 1); made.set(k || `#${made.size}`, r); return r;
};
/** the console's own two, as they were before `tapConsole` (a report prints through these: it is never its own echo) */
const plain: { error: ((...a: unknown[]) => void) | null; warn: ((...a: unknown[]) => void) | null } = { error: null, warn: null };

const words = (x: unknown): string => {
  try {
    if (x instanceof Error) return x.message || x.name;
    if (typeof x === 'string') return x;
    if (x && typeof x === 'object') { const m = (x as { message?: unknown }).message; if (typeof m === 'string' && m) return m; const b = brief(x, true); return (typeof b === 'string' ? b : JSON.stringify(b)).slice(0, REPORT.message); }
    return String(x);
  } catch { return 'something that cannot be put in words'; }
};
/** an object printed beside a message, in brief: its first few keys, a big array or a buffer by its kind and length. A log line with a million numbers
 *  beside it costs the frame nothing here */
const brief = (x: unknown, top = false): unknown => {
  if (x === null || typeof x !== 'object') return typeof x === 'string' ? x.slice(0, 80) : typeof x === 'function' ? 'fn' : typeof x === 'bigint' ? String(x) : x;
  if (ArrayBuffer.isView(x) || x instanceof ArrayBuffer) return `${(x as object).constructor?.name ?? 'buffer'}(${(x as { length?: number }).length ?? (x as ArrayBuffer).byteLength})`;
  if (Array.isArray(x)) return top && x.length <= REPORT.keys ? x.map(v => brief(v)) : `Array(${x.length})`;
  if (!top) return '{…}';
  const o: Record<string, unknown> = {}; let n = 0;
  for (const k in x as Record<string, unknown>) { if (n++ >= REPORT.keys) { o['…'] = 1; break; } o[k] = brief((x as Record<string, unknown>)[k]); }
  return o;
};
/** a stack's own lines (the frames, not its first line where that only repeats the message, and not a line of a message that only has an @ in it) */
const frames = (stack: unknown, drop = 0): string[] => {
  const lines = String(stack ?? '').split('\n').map(l => l.trim()).filter(l => /^at |@.*:\d+/.test(l));
  return lines.slice(drop, drop + REPORT.stack).map(l => l.slice(0, 200));
};
const hand = (r: Report): void => {
  if (inside) return;   // (a sink that itself reports is not asked again for it)
  if (!sink) { if (room(r.level)) { held.push(r); if (r.level === 'warning') heldWarnings++; else heldErrors++; } return; }
  inside = true; try { sink(r); } catch { /* the reports' own trouble is nobody's error */ } finally { inside = false; }
};
/** would a report of this kind go anywhere? (asked BEFORE one is made: a console line nobody will take costs nothing) */
const room = (level: ReportLevel): boolean => !inside && (!!sink || (level === 'warning' ? heldWarnings < REPORT.heldWarnings : heldErrors < REPORT.held));
const make = (where: string, level: ReportLevel, parts: unknown[], data: Record<string, unknown> | undefined, skip: number): Report => {
  const err = parts.find(p => p instanceof Error) as Error | undefined;
  let stack: string[] = []; try { stack = err?.stack ? frames(err.stack) : frames(new Error().stack, skip); } catch { /* */ }
  return { where: where.slice(0, 80), message: parts.map(words).join(' ').replace(/\s+/g, ' ').trim().slice(0, REPORT.message), stack, level, ...(data ? { data: brief(data, true) as Record<string, unknown> } : {}) };
};
/** the warnings already printed (a catch's warning prints once a page) */
const printed = new Set<string>();
const say = (where: string, level: ReportLevel, parts: unknown[], data: Record<string, unknown> | undefined, everyTime: boolean): void => {
  try {
    let quiet = false;
    if (!everyTime) { const k = madeKey(level, where, parts[0]); if (k) { quiet = printed.has(k); if (!quiet && printed.size < REPORT.makes) printed.add(k); } }
    if (!quiet) { const c = typeof console !== 'undefined' ? console : null, print = level === 'error' ? plain.error ?? c?.error?.bind(c) : plain.warn ?? c?.warn?.bind(c); try { print?.(...parts); } catch { /* no console */ } }
    if (room(level)) { const r = once(where, level, parts, data, 3); if (r) hand(r); }   // (3: make, say, the exported call — the stack starts at the caller)
  } catch { /* a report never throws */ }
};

/** AN ERROR THAT WAS CAUGHT: printed, and one of the visit's records where this browser sends. Never throws */
export function reportError(where: string, err: unknown, data?: Record<string, unknown>): void { say(where, 'error', [err], data, true); }
/** something the game went on from: a fallback taken, a thing not built on this machine. Printed as a warning, recorded as one */
export function reportWarning(where: string, err: unknown, data?: Record<string, unknown>): void { say(where, 'warning', [err], data, false); }
/** A PRINTED LINE moved here whole: what `console.error(...said)` printed, as it printed it, and one report under `where` */
export function reportErrorLine(where: string, ...said: unknown[]): void { say(where, 'error', said, undefined, true); }
/** the same for a line that was `console.warn(...said)` */
export function reportWarningLine(where: string, ...said: unknown[]): void { say(where, 'warning', said, undefined, true); }

/** who the reports go to (null = nobody: they wait again). What waited is handed over at once, oldest first */
export function reportSink(s: ((r: Report) => void) | null): void {
  sink = s; if (!s) return;
  heldErrors = heldWarnings = 0; for (const r of held.splice(0)) hand(r);
}
/** THE NET: this console's `error` and `warn` report what they are told, under 'console.error' / 'console.warn', and print as before. Once a page; a
 *  second call does nothing. Returns the way back (a check's) */
export function tapConsole(c: { error: (...a: unknown[]) => void; warn: (...a: unknown[]) => void } = console): () => void {
  if (plain.error || plain.warn) return () => { /* already tapped */ };
  const error = c.error, warn = c.warn; plain.error = error.bind(c); plain.warn = warn.bind(c);
  const tap = (level: ReportLevel, print: (...a: unknown[]) => void) => (...a: unknown[]): void => {
    try { print(...a); } catch { /* */ }
    try { if (room(level)) { const r = once(level === 'error' ? 'console.error' : 'console.warn', level, a, undefined, 2); if (r) hand(r); } } catch { /* */ }   // (2: make, this — the stack starts at who printed)
  };
  c.error = tap('error', plain.error); c.warn = tap('warning', plain.warn);
  return () => { c.error = error; c.warn = warn; plain.error = plain.warn = null; };
}
/** (a check's: what waits, and a clean slate) */
export function reportHeld(): readonly Report[] { return held; }
export function reportForget(): void { held.length = 0; made.clear(); printed.clear(); heldErrors = heldWarnings = 0; sink = null; inside = false; }
