// WHAT THE RUST OF THE GAME MAY NOT SAY, where no compiler keeps it out. No cargo, no game: the sources are read.
//   rules     no line of rust/sim or rust/maths (the world's numbers: lines, bodies, doors, light, the maths) names a rounded function of the standard library, a fused multiply-add, relaxed SIMD, a cell, an atomic or std:
//             one arithmetic on every machine, and nothing kept where a snapshot does not see it
//   statics   every `static mut` of rust/sim and rust/prim is a row here: the games' state, scratch of one call, or a copy proven where it is made
//   sides     every crate under rust/ is gameplay, dressing or both, and neither side links the other; a crate from outside has no side
//   module    the crates the game's module is hashed from (tools/blitz/face-build.ts FACE_CRATES) are the crates its locks link: a crate left out would leave a stale module unseen
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
/** what a line of rust/sim may not say */
const RULES: readonly { re: RegExp; why: string; but?: string }[] = [
  { re: /\.(sin|cos|tan|asin|acos|atan|atan2|sin_cos|sinh|cosh|tanh|asinh|acosh|atanh|exp|exp2|exp_m1|ln|ln_1p|log|log2|log10|powf|powi|cbrt|hypot)\s*\(/, why: 'a rounded function of the standard library: the game has its own (maths.rs), the same bits as the TypeScript and on every machine' },
  { re: /\.mul_add\s*\(/, why: 'a fused multiply-add rounds once where the TypeScript rounds twice' },
  { re: /\.(sqrt|floor|trunc|ceil|round|round_ties_even|fract|rem_euclid|div_euclid)\s*\(/, why: 'the standard library\'s method: the exact operations come from rust/prim by name (add one there if it is exact by the standard)' },
  { re: /relaxed[-_]|_relaxed/i, why: 'relaxed SIMD rounds as the machine likes: it belongs to dressing, never to the simulation' },
  { re: /\b(UnsafeCell|RefCell|Cell|OnceCell|Atomic\w+)\b/, why: 'a static written through a cell: what the crate keeps between calls is a `static mut` with a row in STATICS (the games\' state, scratch or a copy), so the build can say which' },
  { re: /\bextern\s+crate\s+std\b|\bstd::/, why: 'the standard library: the crate is no_std so that no hash map, thread, clock or file can be named' },
];
/** EVERY `static mut` OF THE SIMULATION'S CRATE, AND WHAT IT IS (rust/sim/src/state.rs): STATE = the games' blocks, the one place a game keeps anything from one
 *  call to the next (the host carries a block through every snapshot: rust/game_rules/src/tick/sim_state.rs) · SCRATCH = written and read inside one call · COPY = the host's own
 *  truth copied in and proven there (the box world). A `static mut` with no row here is red: state kept anywhere but the blocks is in no snapshot. */
const STATICS: Readonly<Record<string, string>> = {
  'sim/src/state.rs ALL': 'STATE · a block a game: the head and a part a family',
  'sim/src/lib.rs SCRATCH': 'SCRATCH · arguments and answers too many for a call\'s own parameters',
  'sim/src/world.rs ASKING': 'SCRATCH · one question of the box world: what it has met, the cells it walked, the panes and statics it lists',
  'sim/src/world.rs ALL': 'COPY · the box world, written by src/scene/boxMirror.ts and compared with the boxes as every tick opens',
  'sim/src/fog.rs EYES': 'SCRATCH · the eyes of one sight pass, handed over as the pass opens',
  'sim/src/clips.rs ALL': 'COPY · a character file\'s clips, authored data nothing writes after the file is loaded (src/anim/gltf.ts): copied in once by src/anim/skeleton.ts as a clip is first sampled, the same for every game and in no snapshot; the same for every game',
  'sim/src/doors.rs IO': 'SCRATCH · one door step\'s hand-over: the level\'s doors, the contacts and the rules\' numbers written by the host before it asks, the answers read back after (the leaves\' own numbers are in the block)',
  'sim/src/props.rs IO': 'SCRATCH · one step of the furniture\'s hand-over: the level\'s facts of each thing by row, the men, the leaves and door frames and the rules\' numbers written by the host before it asks, the noises read back after (the things\' own numbers are in the block)',
  'sim/src/slabs.rs PAR': 'SCRATCH · one slab\'s step\'s hand-over: its door\'s facts and the rules\' numbers written by the host before it asks, what the step met read back after (the slab\'s own numbers are in the block)',
  'sim/src/items.rs PAR': 'SCRATCH · one walk of the thrown things\' hand-over: the tick, the row to start at and what the host says of panes and of lying snow written before it asks, the row handed back and the counts read after (each thing\'s own numbers are in the block)',
  'sim/src/items.rs EVS': 'SCRATCH · what one walk of the thrown things listed for the host to do after it (a bounce to say, a share of momentum for a thing of furniture, a thing whose time is up), read back before the next walk',
  'sim/src/skeleton.rs BENCH': 'SCRATCH · the pose a question is asked of, written, asked and read back',
  'sim/src/roles.rs ROLE': 'COPY · the nodes of the rig on the bench by the animator\'s names for them, written by the host with the rig',
  'sim/src/roles.rs LIST': 'COPY · the animator\'s lists of nodes to run forward, for the rig on the bench, written by the host with the rig',
  'sim/src/roles.rs LIST_LEN': 'COPY · how many nodes each of those lists holds, written with them',
  'sim/src/roles.rs NUM': 'COPY · the tables of numbers the animator worked out once from the character file, for the rig on the bench, written by the host with the rig',
  'sim/src/roles.rs NUM_LEN': 'COPY · how many numbers each of those tables holds, written with them',
  'sim/src/panes.rs HELD_BITS': 'SCRATCH · one support pass\'s held pieces', 
  'sim/src/pattern.rs CACHE': 'SCRATCH · a pane\'s pieces, derived from its row and held against the numbers they were made from: compared on every ask, so no write has to say, and no state', 'sim/src/pattern.rs CLOCK': 'SCRATCH · which of those were asked for last',
  'sim/src/pattern.rs NBS': 'SCRATCH · one support pass\'s neighbours across the cracks', 'sim/src/pattern.rs NB_HEAD': 'SCRATCH · where each piece\'s list of them starts', 'sim/src/pattern.rs NB_TAIL': 'SCRATCH · and ends',
  'sim/src/pattern.rs ORDER': 'SCRATCH · the order a toughened pane\'s clumps let go in, derived from its dice and held against the numbers they were made from',
  'sim/src/light.rs SUM': 'SCRATCH · the six faces of one question about the light while its lamps come in parts: begun by the first part, summed on by each later one, read by the host after the last, before any other question about the light is begun',
  'sim/src/light.rs AT': 'SCRATCH · how many of the level\'s rows one question about the light has read so far, while its own lamps come in parts: begun by the first part, gone on from by each later one, like SUM',
  'sim/src/light.rs LAMPS': 'SCRATCH · the lamps of its own of one question about the light on a point (the level\'s are read from their rows), written by the host before it asks',
  'sim/src/nav.rs SHUT': 'SCRATCH · the nav grid\'s shut cells as they stand, written by the host for each search (the grid is the host\'s)',
  'sim/src/nav.rs G': 'SCRATCH · one search\'s scores, left blank after it', 'sim/src/nav.rs F': 'SCRATCH · one search\'s scores, left blank after it',
  'sim/src/nav.rs CAME': 'SCRATCH · one search\'s way back, left blank after it', 'sim/src/nav.rs CLOSED': 'SCRATCH · one search\'s closed cells, left blank after it',
  'sim/src/nav.rs OPEN': 'SCRATCH · one search\'s open list', 'sim/src/nav.rs SEEN': 'SCRATCH · the cells one search wrote, so it can blank them',
  'sim/src/nav.rs CELLS': 'SCRATCH · one way\'s cells before the smoothing', 'sim/src/nav.rs WAY': 'SCRATCH · one answer\'s waypoints, read back by the host',
  'sim/src/nav.rs PRICE': 'SCRATCH · one priced search\'s prices, written by the host where the search asked', 'sim/src/nav.rs KNOWN': 'SCRATCH · which cells one priced search has a price for, blank after it',
  'sim/src/nav.rs PRICED': 'SCRATCH · the cells one priced search has a price for, so it can blank them', 'sim/src/nav.rs WANT': 'SCRATCH · the cells a priced search asks the price of, read by the host', 'sim/src/nav.rs N_WANT': 'SCRATCH · how many cells that list names now',
  'sim/src/nav.rs RUN': 'SCRATCH · a priced search between its calls: one question of the host\'s, resumed to its end or dropped before any other search is asked',
  'sim/src/nav.rs BLANK': 'SCRATCH · whether the scores have been made infinite once since the module loaded (they load as zeros): a fact of the tables, the same for every game',
  'sim/src/bricks.rs INDEX': 'COPY · which brick stands where on the plan, a table a slot of the blocks, made again from the block\'s smoke part whenever the block that stands there is another or was written wholesale (MADE)',
  'sim/src/bricks.rs FACES': 'COPY · the walls between a brick\'s cells, asked of the box world\'s copy and asked again wherever a static row or a leaf moved (always what a fresh ask gives)',
  'sim/src/bricks.rs LEAF_AT': 'COPY · the leaves FACES was last asked against', 'sim/src/bricks.rs LEAF_N': 'COPY · how many leaves that was',
  'sim/src/bricks.rs MADE': 'COPY · whose a slot\'s tables are: the stamp of the block INDEX was made from, and the box world and the copy of it FACES was asked of (the host\'s numbers; where one differs the tables are made again)',
  'sim/src/air.rs PLANS': 'COPY · which cells are roofed, the lists over them and which edges a leaf stands across, a set a slot of the blocks, made again from the block\'s roofs and the leaves whenever the block that stands there is another or was written wholesale (the stamp)',
  'sim/src/air.rs NEXT': 'SCRATCH · one step of the dust\'s field, written before any of it is read back',
  'sim/src/state.rs PUTS': 'SCRATCH · how many times a slot was written whole: whose the tables beside a block are',
  'sim/src/glow.rs SWEPT': 'SCRATCH · how many metres the last step of the light field swept; read by a test, by nothing of a game',
  'sim/src/glow.rs CASTS': 'COPY · what the light field\'s lines meet of a game\'s statics, made of the box world and held to it row by row before it is read (the world\'s count of writes); made again wherever a row is another',
  'sim/src/glow.rs WAYS': 'COPY · the lines\' directions and what each counts for on a face: a table of numbers, worked out once and the same for every game',
  'sim/src/bricks.rs DELTA': 'SCRATCH · one step\'s flows', 'sim/src/bricks.rs WIND': 'SCRATCH · the wind\'s share of each square metre of the plan, written by the host before a step that has wind', 'sim/src/bricks.rs CAME': 'SCRATCH · one step\'s inflow of each brick', 'sim/src/bricks.rs THIN': 'SCRATCH · one step\'s thin bricks', 'sim/src/bricks.rs HELD': 'SCRATCH · one step\'s sum a brick',
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
/** every source line of a crate, its comment off */
function code(crate: string): { at: string; file: string; text: string }[] {
  const out: { at: string; file: string; text: string }[] = [], walk = (d: string): void => { for (const n of readdirSync(d).sort()) { const p = join(d, n); if (statSync(p).isDirectory()) walk(p); else if (n.endsWith('.rs')) { const file = relative(WORK, p); readFileSync(p, 'utf8').split('\n').forEach((raw, i) => out.push({ at: `rust/${file}:${i + 1}`, file, text: raw.replace(/\/\/.*$/, '') })); } } };
  walk(join(WORK, crate, 'src')); return out; }
const sim = [...code('sim'), ...code('maths')], prim = code('prim');
if (BREAK) sim.push({ at: 'rust/sim/src/planted.rs:1', file: 'sim/src/planted.rs', text: 'static mut PLANTED: f64 = x.sin();' });

{ const bad = sim.flatMap(l => RULES.filter(R => R.but !== l.file && R.re.test(l.text)).map(R => `${l.at} — ${R.why}`));
  line(bad, 'rules: no line of rust/sim or rust/maths rounds as the machine likes or keeps what no block holds', `${sim.length} lines read for ${RULES.length} rules`); }
{ const bad: string[] = [], met = new Set<string>();
  for (const l of [...sim, ...prim]) {
    if (/^\s*(?:pub(?:\([a-z]+\))?\s+)?static\s+(?!mut\b)\w+\s*:.*\b(\w*Cell|Atomic\w+|Mutex|RwLock|Once\w*|Lazy\w*)\b/.test(l.text)) bad.push(`${l.at} — a static written through a cell or a lock: what is kept between calls is a \`static mut\` with a row in STATICS`);
    const m = /^\s*(?:pub(?:\([a-z]+\))?\s+)?static\s+mut\s+(\w+)/.exec(l.text); if (!m) continue;
    const key = `${l.file} ${m[1]}`; met.add(key); if (!(key in STATICS)) bad.push(`${l.at} — \`static mut ${m[1]}\` has no row in STATICS (here): a game's state belongs in its block (state.rs), and scratch or a copy says which it is`); }
  for (const k of Object.keys(STATICS)) if (!met.has(k)) bad.push(`STATICS names ${k}, which is not there`);
  line(bad, 'statics: every `static mut` is the games\' state, scratch of one call or a proven copy', `${met.size} of them, a row each`); }
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
console.log(`=== rust-rules-check: ${red ? `${red} FAIL` : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
