// WHAT THE RUST OF THE GAME MAY NOT SAY, where no compiler keeps it out. No cargo, no game: the sources are read.
//   rules     no line of rust/sim, rust/maths, rust/game_types or rust/game_rules (a test's own lines apart) names a rounded function of the standard library, a fused multiply-add, relaxed SIMD, a cell, an atomic or std:
//             one arithmetic on every machine, and nothing kept where a snapshot does not see it
//   statics   every `static mut` of those and of rust/prim is a row here: scratch of one call, a copy proven where it is made, or what is held beside the games
//   sides     every crate under rust/ is gameplay, dressing or both, and neither side links the other; a crate from outside has no side
//   module    the crates the game's module is hashed from (tools/blitz/face-build.ts FACE_CRATES) are the crates its locks link: a crate left out would leave a stale module unseen
//   warnings  no file of a crate turns the compiler's word for unused, dead or misnamed code off (one item may), and every crate's own root makes it an error where the game is played or gated
//   bun run tools/qa/rust-rules-check.ts        exit 0 = it holds · 1 = a line says what does not        RUSTRULES_BREAK=1: one breach of each planted in memory: every line must FAIL
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { FACE_CRATES } from '../blitz/face-build.ts';

const WORK = resolve(import.meta.dir, '../../rust'), BREAK = process.env.RUSTRULES_BREAK === '1';
let red = 0; const line = (bad: string[], what: string, good: string): void => { if (bad.length) red++; console.log(`- ${bad.length ? 'FAIL' : 'PASS'} — ${what}\n    ${bad.length ? bad.join('\n    ') : good}`); };
/** WHICH SIDE OF THE LINE EACH CRATE UNDER rust/ IS ON. 'gameplay': it decides the game, runs on the server, and is bound to one arithmetic. 'dressing': it only makes the picture, and may round as the machine
 *  likes. 'both': exact by the standard and stateless, so either side may link it. A crate with no row here is red: nothing joins without saying which it is. */
type Side = 'gameplay' | 'dressing' | 'both';
const SIDES: Record<string, Side> = { prim: 'both', maths: 'both', sim: 'gameplay', game_derive: 'both', game_types: 'gameplay', game_rules: 'gameplay', game_draw: 'gameplay', game_start: 'gameplay', game_face: 'gameplay' };   // (game_derive: what a type says of itself, made as it compiles · game_rules: the tick · game_start: the game's own start, a level file's text in and a Game out, and what a host does round a step · game_face: the module a host loads)
/** what a line of the game may not say */
const RULES: readonly { re: RegExp; why: string; but?: string }[] = [
  { re: /\.(sin|cos|tan|asin|acos|atan|atan2|sin_cos|sinh|cosh|tanh|asinh|acosh|atanh|exp|exp2|exp_m1|ln|ln_1p|log|log2|log10|powf|powi|cbrt|hypot)\s*\(/, why: 'a rounded function of the standard library: the game has its own (rust/maths), the same bits on every machine' },
  { re: /\.mul_add\s*\(/, why: 'a fused multiply-add rounds once where the TypeScript rounds twice' },
  { re: /\.(sqrt|floor|trunc|ceil|round|round_ties_even|fract|rem_euclid|div_euclid)\s*\(/, why: 'the standard library\'s method: the exact operations come from rust/prim by name (add one there if it is exact by the standard)' },
  { re: /relaxed-simd|x\d+_relaxed_/i, why: 'relaxed SIMD rounds as the machine likes: it belongs to dressing, never to the simulation' },
  { re: /\b(UnsafeCell|RefCell|OnceCell|Atomic\w+|cell::Cell)\b/, why: 'a static written through a cell: what the crate keeps between calls is a `static mut` with a row in STATICS (scratch or a copy), so the build can say which', but: 'game_types/src/made.rs' },   // (made.rs: tables made again from the game, in no bytes and no copy, and the number of a writing, which no rule reads)
  { re: /\bextern\s+crate\s+std\b|\bstd::/, why: 'the standard library: the crate is no_std so that no hash map, thread, clock or file can be named' },
];
/** EVERY `static mut` OF THE GAME'S CRATES, AND WHAT IT IS: SCRATCH = written and read inside one call · COPY = a table of numbers, or the host's own truth copied in and proven
 *  there · HELD = what is kept beside the games. No game keeps anything here (a moment of it is the game as a value), and a `static mut` with no row is red: state kept here is in no snapshot. */
const STATICS: Readonly<Record<string, string>> = {
  'sim/src/world.rs ASKING': 'SCRATCH · one question of the box world: what it has met, the cells it walked, the panes and statics it lists; and what stopped the last line like it, asked first of the next, which changes no answer',
  'sim/src/fog.rs EYES': 'SCRATCH · the eyes of one sight pass, handed over as the pass opens',
  'sim/src/panes.rs HELD_BITS': 'SCRATCH · one support pass\'s held pieces', 
  'sim/src/pattern.rs CACHE': 'SCRATCH · a pane\'s pieces, derived from its row and held against the numbers they were made from: compared on every ask, so no write has to say, and no state', 'sim/src/pattern.rs CLOCK': 'SCRATCH · which of those were asked for last',
  'sim/src/pattern.rs NBS': 'SCRATCH · one support pass\'s neighbours across the cracks', 'sim/src/pattern.rs NB_HEAD': 'SCRATCH · where each piece\'s list of them starts', 'sim/src/pattern.rs NB_TAIL': 'SCRATCH · and ends',
  'sim/src/pattern.rs ORDER': 'SCRATCH · the order a toughened pane\'s clumps let go in, derived from its dice and held against the numbers they were made from',
  'sim/src/nav.rs SHUT': 'COPY · the nav grid\'s shut cells, held to the grid cell by cell at every search', 'sim/src/nav.rs ENTERS': 'COPY · which of its eight a step from a cell enters, made with SHUT', 'sim/src/nav.rs WIDE': 'COPY · how wide and how large the grid SHUT is of was',
  'sim/src/nav.rs G': 'SCRATCH · one search\'s scores, left blank after it',
  'sim/src/nav.rs CAME': 'SCRATCH · one search\'s way back, left blank after it', 'sim/src/nav.rs CLOSED': 'SCRATCH · one search\'s closed cells, left blank after it',
  'sim/src/nav.rs OPEN': 'SCRATCH · one search\'s open list', 'sim/src/nav.rs SEEN': 'SCRATCH · the cells one search wrote, so it can blank them',
  'sim/src/nav.rs CELLS': 'SCRATCH · one way\'s cells before the smoothing', 'sim/src/nav.rs WAY': 'SCRATCH · one answer\'s waypoints, read back by the host',
  'sim/src/nav.rs PRICE': 'SCRATCH · one priced search\'s prices, written by the host where the search asked', 'sim/src/nav.rs KNOWN': 'SCRATCH · which cells one priced search has a price for, blank after it',
  'sim/src/nav.rs PRICED': 'SCRATCH · the cells one priced search has a price for, so it can blank them',
  'sim/src/nav.rs RUN': 'SCRATCH · a search set to its start, before it is run', 'sim/src/nav.rs LIVE': 'SCRATCH · whether a search has the tables: one asked from inside a price has none',
  'sim/src/nav.rs AT': 'SCRATCH · where a cell last stands in one search\'s open list, left blank after it', 'sim/src/nav.rs BEFORE': 'SCRATCH · where it stands beside',
  'sim/src/nav.rs SCORE': 'SCRATCH · one search\'s open cells\' scores', 'sim/src/nav.rs LEAST': 'SCRATCH · the least of each block of them',
  'sim/src/nav.rs BLANK': 'SCRATCH · whether the scores have been made infinite once since the module loaded (they load as zeros): a fact of the tables, the same for every game',
  'sim/src/dome.rs FAN': 'COPY · the ways up to the sky and down to the ground and what each counts for on a face: a table of numbers, worked out once and the same for every game',
  'sim/src/light.rs SET': 'SCRATCH · one sweep\'s lamps, gathered before its metres ask them',
  'game_types/src/made.rs N': 'HELD · the number of the last writing of a thing, which no rule of the game reads',
  'game_rules/src/tick/savestate/control.rs IMAGES': 'HELD · what is kept of each stand, to go back to: a game lost to a panic is gone while this is not', 'game_rules/src/tick/savestate/control.rs STANDS': 'HELD · how many games have stood', 'game_rules/src/tick/savestate/control.rs USES': 'HELD · which stand was asked for last',
  'sim/src/glow.rs WAYS': 'COPY · the lines\' directions and what each counts for on a face: a table of numbers, worked out once and the same for every game',
};
/** the crates each crate links, by name: the workspace's folders (a crate is a folder of rust/ with a Cargo.toml) and, for what each depends on, CARGO'S OWN
 *  LIST in its Cargo.lock: the lock is what is linked, and nobody has to read the many ways a Cargo.toml can spell a
 *  dependency. A package the lock names that is no folder here (a crate from outside) comes back too, and has no side. */
function links(): Record<string, string[]> {
  const out: Record<string, string[]> = {};
  for (const n of readdirSync(WORK).sort()) if (existsSync(join(WORK, n, 'Cargo.toml'))) out[n] = [];
  for (const n of Object.keys(out)) { const own = join(WORK, n, 'Cargo.lock'); if (!existsSync(own)) continue;
    for (const block of readFileSync(own, 'utf8').split(/^\[\[package\]\]\s*$/m).slice(1)) {
      const name = /^name\s*=\s*"([^"]+)"/m.exec(block)?.[1]; if (!name || (out[name]?.length ?? 0) > 0) continue;
      const list = /^dependencies\s*=\s*\[([^\]]*)\]/m.exec(block)?.[1] ?? '';
      out[name] = [...list.matchAll(/"([^"\s]+)[^"]*"/g)].map(m => m[1]);
    } }
  return out;
}
/** every source line of a crate, its comment off (or whole) */
function code(crate: string, whole = false): { at: string; file: string; text: string }[] {
  const out: { at: string; file: string; text: string }[] = [], walk = (d: string): void => { for (const n of readdirSync(d).sort()) { const p = join(d, n); if (statSync(p).isDirectory()) walk(p); else if (n.endsWith('.rs')) { const file = relative(WORK, p); readFileSync(p, 'utf8').split('\n').forEach((raw, i) => out.push({ at: `rust/${file}:${i + 1}`, file, text: whole ? raw : raw.replace(/\/\/.*$/, '') })); } } };
  walk(join(WORK, crate, 'src')); return out; }
/** a crate's lines without its tests' */
function played(crate: string): ReturnType<typeof code> { const out: ReturnType<typeof code> = []; let skip = '', next = false, file = '';
  for (const l of code(crate)) { if (l.file !== file) { file = l.file; skip = ''; next = false; }
    if (skip) { if (l.text.startsWith(skip)) skip = ''; continue; }
    if (/^\s*#\[cfg\(test\)\]/.test(l.text)) { next = true; continue; }
    if (next && /^\s*#\[/.test(l.text)) continue;
    if (next) { next = false; if (/\{\s*$/.test(l.text)) skip = `${/^\s*/.exec(l.text)![0]}}`; continue; }
    out.push(l); }
  return out; }
const sim = [...code('sim'), ...code('maths'), ...played('game_types'), ...played('game_rules')], prim = code('prim');
if (BREAK) sim.push({ at: 'rust/sim/src/planted.rs:1', file: 'sim/src/planted.rs', text: 'static mut PLANTED: f64 = x.sin();' });

{ const bad = sim.flatMap(l => RULES.filter(R => R.but !== l.file && R.re.test(l.text)).map(R => `${l.at} — ${R.why}`));
  line(bad, 'rules: no line of rust/sim, rust/maths, rust/game_types or rust/game_rules rounds as the machine likes or keeps what no block holds', `${sim.length} lines read for ${RULES.length} rules`); }
{ const bad: string[] = [], met = new Set<string>();
  for (const l of [...sim, ...prim]) {
    const c = /^\s*(?:pub(?:\([a-z]+\))?\s+)?static\s+(?!mut\b)(\w+)\s*:.*\b(\w*Cell|Atomic\w+|Mutex|RwLock|Once\w*|Lazy\w*)\b/.exec(l.text);
    if (c) { const key = `${l.file} ${c[1]}`; met.add(key); if (!(key in STATICS)) bad.push(`${l.at} — a static written through a cell or a lock: what is kept between calls is a \`static mut\` with a row in STATICS`); continue; }
    const m = /^\s*(?:pub(?:\([a-z]+\))?\s+)?static\s+mut\s+(\w+)/.exec(l.text); if (!m) continue;
    const key = `${l.file} ${m[1]}`; met.add(key); if (!(key in STATICS)) bad.push(`${l.at} — \`static mut ${m[1]}\` has no row in STATICS (here): a game's state belongs on the game, and scratch or a copy says which it is`); }
  for (const k of Object.keys(STATICS)) if (!met.has(k)) bad.push(`STATICS names ${k}, which is not there`);
  line(bad, 'statics: every `static mut` is scratch of one call or a proven copy', `${met.size} of them, a row each`); }
const L = links(); if (BREAK) L.planted = ['outside', 'sim'];
{ const bad: string[] = [];
  for (const [c, deps] of Object.entries(L)) {
    const side = SIDES[c]; if (!side) { bad.push(`the crate ${c} has no side: say in SIDES whether it is gameplay, dressing or both (a crate from outside has none)`); continue; }
    for (const d of deps) { const ds = SIDES[d];
      if (!ds || !(d in L)) bad.push(`rust/${c} links ${d}, which is not a crate of rust/ with a side`);
      else if (side === 'both' && ds !== 'both') bad.push(`rust/${c} is for both sides and links ${d}, which is ${ds}`);
      else if (side !== 'both' && ds !== 'both' && ds !== side) bad.push(`rust/${c} is ${side} and links ${d}, which is ${ds}`); } }
  for (const c of Object.keys(SIDES)) if (!(c in L)) bad.push(`SIDES names ${c}, and there is no such crate`);
  line(bad, 'sides: every crate has a side, and neither side links the other', Object.keys(L).map(c => `${c} ${SIDES[c]}`).join(' · ')); }
{ const want = new Set<string>(), walk = (c: string): void => { if (want.has(c)) return; want.add(c); for (const d of L[c] ?? []) walk(d); }; walk('game_face');
  const have = BREAK ? FACE_CRATES.slice(1) : [...FACE_CRATES], bad = [...want].filter(c => !have.includes(c as never)).map(c => `the module links ${c}, and FACE_CRATES has no such row`); for (const c of have) if (!want.has(c)) bad.push(`FACE_CRATES names ${c}, which the module does not link`);
  line(bad, 'module: what it is hashed from is what it links', [...want].sort().join(' ')); }
{ const bad: string[] = [], crates = Object.keys(SIDES); let read = 0;
  const off = (at: string, raw: string): void => { if (/^\s*#!\[\s*(cfg_attr\([^\]]*)?allow\([^\]]*\b(unused\w*|dead_code|warnings|nonstandard_style|non_\w+)\b/.test(raw)) bad.push(`${at} — the compiler's word is off for a whole file`); };
  for (const c of crates) { for (const l of code(c, true)) { read++; off(l.at, l.text); }
    if (!code(c, true).some(l => l.file === `${c}/src/lib.rs` && l.text.startsWith('#![cfg_attr(not(debug_assertions), deny(unused, nonstandard_style))]'))) bad.push(`rust/${c}/src/lib.rs does not make a warning an error where the game is played or gated`); }
  if (BREAK) off('a planted line', '#![allow(unused_imports)]');
  line(bad, 'warnings: no file turns the compiler\'s word for unused code off', `${crates.length} crates, ${read} lines`); }
console.log(`=== rust-rules-check: ${red ? `${red} FAIL` : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
