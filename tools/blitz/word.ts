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
import { readFileSync, appendFileSync, existsSync, unlinkSync } from 'node:fs';
import { tmpdir } from 'node:os';

export const DEFAULT_WORD: string = 'none';

/** HOW A CHECK KEPT ON THE TYPESCRIPT TICK IS STOOD (by the table, or by a frozen run): `'ts'` = the word's own stand, the game owning its die and the dress
 *  rows down (the stand the Rust game's tallies are held beside; an exact number recorded with no word moves once); `'none'` = exactly as with no word, to the
 *  byte. `TPS_KEPT=ts|none` says it for one run. */
export const KEPT_STAND: 'ts' | 'none' = 'ts';
const kept = (): 'ts' | 'none' => process.env.TPS_KEPT === 'none' ? 'none' : process.env.TPS_KEPT === 'ts' ? 'ts' : KEPT_STAND;

export type Word = '' | 'rust' | 'audit' | 'ts';
/** the word asked for, before the table and the stand are looked at ('' = none) */
export function asked(): string { const w = process.env.TPS_GAME ?? DEFAULT_WORD; return w === 'none' || w === 'off' ? '' : w; }

interface Kinds { probes: Record<string, string>; tools: Record<string, { word: string; why: string }>; pins?: Record<string, string[]>; waits?: Record<string, string>; families?: string[]; unread?: string[] }
let KINDS: Kinds | null = null;
export function kinds(): Kinds { return KINDS ??= JSON.parse(readFileSync(new URL('./qa-kinds.json', import.meta.url).pathname, 'utf8')); }
// (kinds iii and iv: a check that CALLS the tick's rules between ticks, hangs its own function on a game object, checks a TypeScript function with no
// game, or holds the game to a walk of the JavaScript objects stays on the TypeScript tick until the generated call surface and taps land)
const ON_TS = new Set(['iii', 'iv-a', 'iv-b', 'iv-c']);
const probeNow = (): string | null => (globalThis as any).__probeNow ?? process.env.RH_PROBE ?? null;
const toolNow = (): string => (process.argv[1] ?? '').replace(/\\/g, '/');

export interface Stood { word: Word; who: string; why: string; kind: string; /** kept on the TypeScript tick off the word asked for */ kept?: boolean }
/** which game is stood for the check running now; `frozen`: the run is on the old frozen stand (TPS_PROFILE=frozen). Counts nothing and says nothing. */
export function resolve(frozen = false): Stood { const S = decide(frozen);
  // (a FROZEN run is stood exactly as with no word whatever KEPT_STAND says: the frozen stand is there for what was recorded on it, and that was recorded
  // on a game with no die of its own)
  if (S.word === 'ts' && S.kind !== 'asked') { S.kept = true; if (kept() === 'none' || S.kind === 'frozen') S.word = ''; }
  return S; }
function decide(frozen: boolean): Stood {
  const a = asked(), who = probeNow() ?? toolNow().split('/').pop() ?? '';
  if (a === '') return { word: '', who, why: 'no word', kind: 'none' };
  if (a !== 'rust' && a !== 'audit' && a !== 'ts' && a !== 'table') throw new Error(`TPS_GAME=${a}: rust (the Rust game steps), audit, ts (the same stand, the TypeScript tick), table, or none`);
  // (AN ENGINE THAT IS NOT BUN HAS NO BRIDGE: the preload is bun's. A tool that starts a child under node (a check of two engines, a cost under V8) gets the
  // TypeScript tick there, stood as with no word, said and counted.)
  if (!(globalThis as any).Bun) return { word: '', who, why: 'this engine is not bun and has no bridge preload', kind: 'not bun' };
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
    // (a check SEEN RED ON THE RUST GAME and green on the TypeScript tick, with what it waits on: `waits`, by hand from a run of every Rust-side probe under
    // each word. It stays on the TypeScript tick, said and counted, until its row is taken out.)
    const wait = K.waits?.[probe]; if (wait) return { word: 'ts', who, why: `it waits on ${wait}`, kind: 'waits' };
    return { word: 'rust', who, why: `kind ${k}`, kind: k }; }
  // (a stand-alone tool: its row by its path under the repository, or by its file's name)
  const tool = toolNow(), row = Object.entries(K.tools).find(([n]) => n.includes('/') && tool.endsWith('/' + n))?.[1] ?? K.tools[who];
  if (row?.word === 'none') return { word: '', who, why: row.why, kind: 'a tool\'s row: no word' };
  if (row?.word === 'ts') return { word: 'ts', who, why: row.why, kind: 'a tool\'s row' };
  if (row?.word === 'rust') return { word: 'rust', who, why: row.why, kind: 'a tool\'s row' };
  // (A TOOL WITH NO ROW, like a probe with none: the TypeScript tick, counted and named. Several hundred check files stand a game when run as their own
  // main; only a tool SEEN to play the same on the Rust game has a `rust` row.)
  return { word: 'ts', who, why: 'no row in the table', kind: 'no row' }; }

// ---- every game stood under `table`, or forced off the word asked for, is counted, said as its check's first game is stood, and summed in ONE LINE as the
// process ends, so that a green of the old tick is not read as the Rust game's
const SAID = { rust: new Map<string, number>(), ts: new Map<string, number>(), none: new Map<string, number>(), kinds: new Map<string, number>(), norow: new Set<string>(), frozen: 0, hooked: false };
const n = (M: Map<string, number>) => [...M.values()].reduce((a, b) => a + b, 0);
function closing(): string { const T = SAID, parts: string[] = [];
  parts.push(`${T.rust.size} checks stood ${n(T.rust)} games on the RUST game`);
  parts.push(`${T.ts.size} checks stood ${n(T.ts)} games on the TYPESCRIPT TICK${T.ts.size ? ` (${[...T.kinds].map(([k, c]) => `${k} ${c}`).join(', ')}: NOT GREENS OF THE RUST GAME): ${[...T.ts.keys()].slice(0, 40).join(' ')}${T.ts.size > 40 ? ' …' : ''}` : ''}`);
  if (T.none.size) parts.push(`${T.none.size} stood ${n(T.none)} games with no word, by a tool's row or under an engine that has no bridge (NOT GREENS OF THE RUST GAME): ${[...T.none.keys()].join(' ')}`);
  parts.push(`${T.norow.size} checks with no row${T.norow.size ? `: ${[...T.norow].slice(0, 40).join(' ')}${T.norow.size > 40 ? ' …' : ''} (tools/blitz/table-check.ts)` : ''}`);
  if (T.frozen) parts.push(`TPS_PROFILE=frozen: ALL ${T.frozen} games of this run were FORCED TO THE TYPESCRIPT TICK (the Rust game has no frozen stand)`);
  return `=== the game, by ${asked() === 'table' ? 'the table' : `TPS_GAME=${asked()}`}: ${parts.join(' · ')} ===\n`; }
// A TOOL THAT PLAYS ITS GAMES IN CHILDREN (a tally's pool, a check that starts another engine) stands none itself, and a child's output may go nowhere: so the
// line is the TOP process's. The first process of a run under a word (the bridge's preload calls `top()` there, before anything else) names a file in the
// environment; every process below it adds its counts to that file as it ends, and the top one sums them into the one line. A process with no such file
// in its environment (the word asked for with no preload) says its own line.
const TALLY = 'TPS_GAME_TALLY'; let isTop = false;
const mine = () => JSON.stringify({ rust: [...SAID.rust], ts: [...SAID.ts], none: [...SAID.none], kinds: [...SAID.kinds], norow: [...SAID.norow], frozen: SAID.frozen }) + '\n';
function sum(line: string): void { let r: any; try { r = JSON.parse(line); } catch { return; }
  for (const k of ['rust', 'ts', 'none', 'kinds'] as const) for (const [name, c] of r[k] ?? []) SAID[k].set(name, (SAID[k].get(name) ?? 0) + c);
  for (const x of r.norow ?? []) SAID.norow.add(x); SAID.frozen += r.frozen ?? 0; }
function atExit(): void { const file = process.env[TALLY], stood = SAID.rust.size + SAID.ts.size + SAID.none.size > 0;
  if (file && !isTop) { if (stood) try { appendFileSync(file, mine()); } catch { /* the line is not the tool's */ } return; }
  if (file) { try { if (existsSync(file)) { for (const l of readFileSync(file, 'utf8').split('\n')) if (l) sum(l); unlinkSync(file); } } catch { /* as above */ } }
  if (SAID.rust.size + SAID.ts.size + SAID.none.size > 0) process.stdout.write(closing()); }
function hook(): void { if (SAID.hooked) return; SAID.hooked = true; process.on('exit', atExit); }
/** called once by the bridge's preload in every process under a word: the first of a run becomes the one that says the closing line */
export function top(): void { if (!asked()) return; if (!process.env[TALLY]) { isTop = true; process.env[TALLY] = `${tmpdir()}/tps-game-${process.pid}-${Date.now().toString(36)}.jsonl`; } hook(); }
/** the same, for the game being stood now: counted, and said where it is not the Rust game's */
export function stand(frozen = false): Stood {
  const S = resolve(frozen); if (S.kind === 'none' || (S.kind === 'asked' && asked() !== 'table')) return S;
  const T = SAID, M = S.kept || S.word === 'ts' ? T.ts : S.word === '' ? T.none : T.rust, first = !M.has(S.who);
  if (first && S.word !== 'rust' && S.word !== 'audit') { if (M === T.ts) T.kinds.set(S.kind, (T.kinds.get(S.kind) ?? 0) + 1);
    // (not console.log: a check may hold that while it stands its game)
    if (S.kind !== 'frozen' || !T.frozen) process.stdout.write(`  [the game: ${S.kind === 'frozen' ? 'this run' : S.who} is stood ${M === T.ts ? `on the TYPESCRIPT TICK${S.word === '' ? ' (as with no word)' : ''}` : 'WITH NO WORD'} ${S.kind === 'frozen' ? 'whatever the word' : S.kind === 'not bun' ? 'whatever the word' : 'by the table'} (${S.why}); its verdict is not the Rust game's]\n`); }
  if (S.kind === 'no row') T.norow.add(S.who); if (S.kind === 'frozen') T.frozen++;
  M.set(S.who, (M.get(S.who) ?? 0) + 1);
  hook();
  return S; }
