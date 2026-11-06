// THE ONE SWITCH: WHICH GAME A HEADLESS TOOL'S `standUp` STANDS (tools/blitz/PLAY.md, "The harness's word"). Everything that asks goes through here: the
// harness (tools/qa/headless.ts), the bridge's preload (tools/blitz/preload.ts: whether a process needs the bridge at all) and a tool that must know on which
// game it is (tools/qa/coop-check.ts). It imports nothing of src and nothing of bun's: it is read before any source file loads, and under node.
//
//   TPS_GAME=rust | audit   the game is a puppet of the Rust game
//   TPS_GAME=ts             the same stand (the game's own die, the dress rows down), the TypeScript tick
//   TPS_GAME=table          the Rust game, but for the checks tools/blitz/qa-kinds.json keeps on the TypeScript tick
//   TPS_GAME=none           nothing of this is loaded
//   unset                   DEFAULT_WORD below
//
// THE FLIP is the one line below: `'none'` to `'table'`.
import { readFileSync } from 'node:fs';

export const DEFAULT_WORD: string = 'none';

export type Word = '' | 'rust' | 'audit' | 'ts';
/** the word asked for, before the table and the stand are looked at ('' = none) */
export function asked(): string { const w = process.env.TPS_GAME ?? DEFAULT_WORD; return w === 'none' || w === 'off' ? '' : w; }

interface Kinds { probes: Record<string, string>; tools: Record<string, { word: string; why: string }>; pins?: Record<string, string[]>; families?: string[]; unread?: string[] }
let KINDS: Kinds | null = null;
export function kinds(): Kinds { return KINDS ??= JSON.parse(readFileSync(new URL('./qa-kinds.json', import.meta.url).pathname, 'utf8')); }
// (kinds iii and iv: a check that CALLS the tick's rules between ticks, hangs its own function on a game object, checks a TypeScript function with no
// game, or holds the game to a walk of the JavaScript objects stays on the TypeScript tick until the generated call surface and taps land)
const ON_TS = new Set(['iii', 'iv-a', 'iv-b', 'iv-c']);
const probeNow = (): string | null => (globalThis as any).__probeNow ?? process.env.RH_PROBE ?? null;
const toolNow = (): string => (process.argv[1] ?? '').replace(/\\/g, '/');

export interface Stood { word: Word; who: string; why: string; kind: string }
/** which game is stood for the check running now; `frozen`: the run is on the old frozen stand (TPS_PROFILE=frozen). Counts nothing and says nothing. */
export function resolve(frozen = false): Stood {
  const a = asked(), who = probeNow() ?? toolNow().split('/').pop() ?? '';
  if (a === '') return { word: '', who, why: 'no word', kind: 'none' };
  if (a !== 'rust' && a !== 'audit' && a !== 'ts' && a !== 'table') throw new Error(`TPS_GAME=${a}: rust (the Rust game steps), audit, ts (the same stand, the TypeScript tick), table, or none`);
  // THE FROZEN STAND IS THE TYPESCRIPT TICK'S. Its rows are the settled switches held off their page values, and a settled switch is a constant in the Rust
  // game (the page's branch was translated, the other dropped): the Rust game cannot be stood there. Said on the closing line.
  if (frozen && a !== 'ts') return { word: 'ts', who, why: 'TPS_PROFILE=frozen: the frozen stand holds settled switches off their page values, which the Rust game has as constants', kind: 'frozen' };
  if (a !== 'table') return { word: a, who, why: `TPS_GAME=${a}`, kind: 'asked' };
  const K = kinds(), probe = probeNow();
  if (probe) {
    const k = K.probes[probe];
    // (A NAME THE TABLE HAS NO ROW FOR: a probe added since the census was run, or one the census found no line for. Counted and printed.)
    if (k === undefined) return { word: 'ts', who, why: 'no row in the table', kind: 'no row' };
    if (ON_TS.has(k)) return { word: 'ts', who, why: `kind ${k}`, kind: k };
    // (a check that runs with a SETTLED switch off its page value asks for a game the Rust game does not have: `pins`, from a run with RH_FOLDED)
    const pin = K.pins?.[probe]; if (pin?.length) return { word: 'ts', who, why: `it runs with a settled switch off its page value: ${pin.slice(0, 3).join(', ')}`, kind: 'pins a settled switch' };
    return { word: 'rust', who, why: `kind ${k}`, kind: k }; }
  // (a stand-alone tool: its row by its path under the repository, or by its file's name)
  const tool = toolNow(), row = Object.entries(K.tools).find(([n]) => n.includes('/') && tool.endsWith('/' + n))?.[1] ?? K.tools[who];
  if (row?.word === 'none') return { word: '', who, why: row.why, kind: 'a tool\'s row: no word' };
  if (row?.word === 'ts') return { word: 'ts', who, why: row.why, kind: 'a tool\'s row' };
  return { word: 'rust', who, why: row?.why ?? 'no row: the default', kind: 'asked' }; }

// ---- every game stood under `table`, or forced off the word asked for, is counted, said as its check's first game is stood, and summed in ONE LINE as the
// process ends, so that a green of the old tick is not read as the Rust game's
const SAID = { rust: new Map<string, number>(), ts: new Map<string, number>(), none: new Map<string, number>(), kinds: new Map<string, number>(), norow: new Set<string>(), frozen: 0, hooked: false };
const n = (M: Map<string, number>) => [...M.values()].reduce((a, b) => a + b, 0);
function closing(): string { const T = SAID, parts: string[] = [];
  parts.push(`${T.rust.size} checks stood ${n(T.rust)} games on the RUST game`);
  parts.push(`${T.ts.size} checks stood ${n(T.ts)} games on the TYPESCRIPT TICK${T.ts.size ? ` (${[...T.kinds].map(([k, c]) => `${k} ${c}`).join(', ')}: NOT GREENS OF THE RUST GAME): ${[...T.ts.keys()].slice(0, 40).join(' ')}${T.ts.size > 40 ? ' …' : ''}` : ''}`);
  if (T.none.size) parts.push(`${T.none.size} stood ${n(T.none)} games with no word by a tool's row (NOT GREENS OF THE RUST GAME): ${[...T.none.keys()].join(' ')}`);
  parts.push(`${T.norow.size} checks with no row${T.norow.size ? `: ${[...T.norow].slice(0, 40).join(' ')}${T.norow.size > 40 ? ' …' : ''} (tools/blitz/table-check.ts)` : ''}`);
  if (T.frozen) parts.push(`TPS_PROFILE=frozen: ALL ${T.frozen} games of this run were FORCED TO THE TYPESCRIPT TICK (the Rust game has no frozen stand)`);
  return `=== the game, by ${asked() === 'table' ? 'the table' : `TPS_GAME=${asked()}`}: ${parts.join(' · ')} ===\n`; }
/** the same, for the game being stood now: counted, and said where it is not the Rust game's */
export function stand(frozen = false): Stood {
  const S = resolve(frozen); if (S.kind === 'none' || (S.kind === 'asked' && asked() !== 'table')) return S;
  const T = SAID, M = S.word === '' ? T.none : S.word === 'ts' ? T.ts : T.rust, first = !M.has(S.who);
  if (first && S.word !== 'rust' && S.word !== 'audit') { if (S.word === 'ts') T.kinds.set(S.kind, (T.kinds.get(S.kind) ?? 0) + 1);
    // (not console.log: a check may hold that while it stands its game)
    if (S.kind !== 'frozen' || !T.frozen) process.stdout.write(`  [the game: ${S.kind === 'frozen' ? 'this run' : S.who} is stood ${S.word === '' ? 'WITH NO WORD' : 'on the TYPESCRIPT TICK'} ${S.kind === 'frozen' ? 'whatever the word' : 'by the table'} (${S.why}); its verdict is not the Rust game's]\n`); }
  if (S.kind === 'no row') T.norow.add(S.who); if (S.kind === 'frozen') T.frozen++;
  M.set(S.who, (M.get(S.who) ?? 0) + 1);
  if (!T.hooked) { T.hooked = true; process.on('exit', () => { process.stdout.write(closing()); }); }
  return S; }
