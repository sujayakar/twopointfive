// THE RUST BUILD — the crates under rust/ → WebAssembly → a built.ts each module, its bytes as text, which is checked in:
//   rust/sim (+ rust/prim) → src/sim/built.ts      THE SIMULATION: what decides the game. The page, the game server and the headless tools run these bytes.
//   rust/dress             → src/dress/built.ts    DRESSING: what only makes the picture. Its own module and memory; the server does not run it.
//   bun run tools/sim/build.ts            build and write both (needs rustup: rust/rust-toolchain.toml names the compiler and the target, and rustup
//                                         fetches both on the first build)
//   bun run tools/sim/build.ts --check    is each built.ts the build of its crates as they stand, and does each module keep its side of the line? exit 0 yes · 1 no
// THE LINE BETWEEN THE TWO IS HELD HERE, ON THE BYTES THAT RUN (docs/agents/rust.md, gameplay and dressing):
//   sides    every crate under rust/ has a side (SIDES), and a simulation crate links no dressing crate, nor a dressing crate the simulation.
//   alone    neither module imports anything: no clock, no random source, no host call, and nobody else's memory. So dressing cannot write the simulation's.
//   strict   the simulation's code holds no relaxed SIMD and no threads instruction (tools/sim/wasm.ts reads every instruction; one it does not know is red).
//   reader   … and that reading is itself tried on modules made by hand that break each rule.
// WHY THE BYTES ARE CHECKED IN: the page, the game server and every headless tool load the module from built.ts as they load any other source file, so nobody
// who only plays or only writes TypeScript needs a Rust compiler, and a hosted build needs none. The price is that built.ts must follow rust/sim: `--check` is the
// guard (tools/qa/rust-check.ts runs it). It holds two things:
//   source   the hash of the crates' sources written into built.ts is their hash now. No compiler needed: this half runs everywhere.
//   rules    no line of the simulation's crate names what its head (rust/sim/src/lib.rs) forbids and the compiler cannot keep out (RULES below).
//   bytes    where cargo is on the machine, the crate is built again and the bytes are the checked-in ones. Another builder than the one that wrote built.ts
//            (the file names its compiler and its kind of machine) may get other bytes from the same sources: that is said and does not fail; the same one fails.
// The module is built with no paths of the building machine in it (--remap-path-prefix; the profile strips names and debug data), and the check looks.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync, readdirSync, statSync, existsSync } from 'node:fs';
import { resolve, relative, join } from 'node:path';
import { spawnSync } from 'node:child_process';
import { homedir } from 'node:os';
import { pow10Text } from './pow10';
import { readWasm, tinyWasm, type WasmRead } from './wasm';

const ROOT = resolve(import.meta.dir ?? new URL('.', import.meta.url).pathname, '../..');
const WORK = join(ROOT, 'rust'), CRATE = join(WORK, 'sim'), TARGET = join(WORK, 'target');
/** WHICH SIDE OF THE LINE EACH CRATE UNDER rust/ IS ON. 'gameplay': it decides the game, runs on the server, and is bound to one arithmetic. 'dressing': it
 *  only makes the picture, and may round as the machine likes. 'both': exact by the standard and stateless, so either side may link it. A crate with no row
 *  here is red (`sides`): nothing joins the workspace without saying which it is. */
type Side = 'gameplay' | 'dressing' | 'both';
export const SIDES: Record<string, Side> = { prim: 'both', sim: 'gameplay', dress: 'dressing' };
/** the modules the build makes: the crate each is the build of, every crate it is made from, the file its bytes go to, and the name its constants carry there */
interface Mod { name: string; side: 'gameplay' | 'dressing'; crates: string[]; out: string; tag: string }
export const MODULES: readonly Mod[] = [
  { name: 'sim', side: 'gameplay', crates: ['prim', 'sim'], out: 'src/sim/built.ts', tag: 'SIM' },
  { name: 'dress', side: 'dressing', crates: ['dress'], out: 'src/dress/built.ts', tag: 'DRESS' },
];
/** the simulation's module (the one the crate rules and the powers' table are about) */
const SIM = MODULES[0];
const wasmOf = (m: Mod) => join(TARGET, `wasm32-unknown-unknown/release/${m.name}.wasm`), outOf = (m: Mod) => join(ROOT, m.out);
/** Chrome compiles a module on the page's own thread only up to 8 MB (src/sim/sim.ts boots in one synchronous step): the build is red well short of it */
export const MAX_BYTES = 6 * 1024 * 1024;

/** every file a module is made from, by path */
function sources(m: Mod): string[] {
  const out: string[] = ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml'];   // (the workspace's three decide the compiler and its settings)
  const walk = (dir: string) => { for (const n of readdirSync(dir).sort()) { const p = join(dir, n); if (statSync(p).isDirectory()) walk(p); else if (n.endsWith('.rs')) out.push(relative(WORK, p).split('\\').join('/')); } };
  for (const c of m.crates) { out.push(`${c}/Cargo.toml`); walk(join(WORK, c, 'src')); }
  return out;
}
export function sourceHash(m: Mod): string {
  const h = createHash('sha256');
  for (const f of sources(m)) { h.update(f + '\0'); h.update(readFileSync(join(WORK, f))); h.update('\0'); }
  return h.digest('hex').slice(0, 32);
}
/** who would build it here: the compiler rust/ picks on this machine and the kind of machine, as one word ('rustc 1.97.0 on x86_64-unknown-linux-gnu'); null = no rustc */
function builder(): string | null {
  const r = spawnSync('rustc', ['-vV'], { cwd: WORK, encoding: 'utf8' });
  if (r.error || r.status !== 0) return null;
  const host = /^host: (.+)$/m.exec(r.stdout)?.[1], release = /^release: (.+)$/m.exec(r.stdout)?.[1];
  return host && release ? `rustc ${release} on ${host}` : null;
}
/** a module's crate compiled: its bytes (null with the reason said where there is no compiler) */
export function compile(quiet: boolean, m: Mod, by: string | null = builder()): Uint8Array | null {
  if (by === null) { if (!quiet) console.error('rust build — no rustc on this machine (https://rustup.rs; the toolchain file in rust/ names the rest)'); return null; }
  const home = homedir();
  // THE FLAGS ARE THESE AND NO OTHERS (the encoded form: a path with a space in it stays one flag, and it stands over a RUSTFLAGS of the shell's and a cargo
  // config's); the output goes where this file reads it whatever CARGO_TARGET_DIR says.
  const flags = [`${ROOT}=.`, `${join(home, '.cargo')}=/cargo`, `${join(home, '.rustup')}=/rustup`].map(m => `--remap-path-prefix=${m}`).join('\x1f');
  const r = spawnSync('cargo', ['build', '--release', '--locked', '-p', m.name, '--target', 'wasm32-unknown-unknown', '--target-dir', TARGET], { cwd: WORK, stdio: quiet ? 'pipe' : 'inherit', encoding: 'utf8', env: { ...process.env, CARGO_ENCODED_RUSTFLAGS: flags } });
  if (r.status !== 0) { if (quiet) console.error(r.stderr); return null; }
  if (!existsSync(wasmOf(m))) { console.error(`rust build — cargo said it built and ${relative(ROOT, wasmOf(m))} is not there`); return null; }
  return new Uint8Array(readFileSync(wasmOf(m)));
}
function leaks(bytes: Uint8Array): string[] {
  const text = Buffer.from(bytes).toString('latin1');
  const homes = ['home', 'Users', 'root'].map(d => `/${d}/`).concat(['C:', 'Users'].join('\\'));   // (where a builder's own folders live, on any of the three systems)
  return [homedir(), ROOT, ...homes].filter((p, i, a) => p.length > 3 && a.indexOf(p) === i && text.includes(p));
}
interface Built { source: string; builder: string; bytes: Uint8Array }
function readBuilt(m: Mod): Built | null {
  if (!existsSync(outOf(m))) return null;
  const t = readFileSync(outOf(m), 'utf8');
  const source = new RegExp(`${m.tag}_SOURCE = '([0-9a-f]+)'`).exec(t)?.[1], by = new RegExp(`${m.tag}_BUILDER = '([^']*)'`).exec(t)?.[1];
  const b64 = [...t.matchAll(/^  '([A-Za-z0-9+/=]+)',$/gm)].map(m => m[1]).join('');
  if (!source || by === undefined || !b64) return null;
  return { source, builder: by, bytes: new Uint8Array(Buffer.from(b64, 'base64')) };
}
function write(m: Mod, bytes: Uint8Array, by: string): void {
  const b64 = Buffer.from(bytes).toString('base64'), lines: string[] = [], T = m.tag;
  for (let i = 0; i < b64.length; i += 160) lines.push(`  '${b64.slice(i, i + 160)}',`);
  const what = m.side === 'gameplay' ? `The Rust simulation (rust/${m.name})` : `The Rust dressing (rust/${m.name})`, face = `src/${m.name}/${m.name}.ts`;
  writeFileSync(outOf(m), `// WRITTEN BY tools/sim/build.ts — never by hand. ${what} built to WebAssembly, its bytes as base64: ${face} loads it.
// \`bun run tools/sim/build.ts\` after any change under rust/; \`bun run tools/sim/build.ts --check\` (and tools/qa/rust-check.ts) is red when this file is not
// the build of rust/${m.name} as it stands.
/** the hash of the crates' sources this was built from */
export const ${T}_SOURCE = '${sourceHash(m)}';
/** the compiler and the kind of machine that built it (the same sources built by another may give other bytes) */
export const ${T}_BUILDER = '${by}';
/** the module's size in bytes */
export const ${T}_BYTES = ${bytes.length};
export const ${T}_WASM: readonly string[] = [
${lines.join('\n')}
];
`);
}
const same = (a: Uint8Array, b: Uint8Array) => a.length === b.length && a.every((x, i) => x === b[i]);

/** WHAT NO LINE OF THE SIMULATION'S CRATE MAY NAME (rust/sim/src/lib.rs: what it owes the game) — the part the compiler does not hold. The crate is `no_std`,
 *  which keeps out every `std::` path; but once any crate of the module links the standard library its float methods resolve everywhere, so those are held here. */
const RULES: readonly { re: RegExp; why: string; but?: string }[] = [
  { re: /\.(sin|cos|tan|asin|acos|atan|atan2|sin_cos|sinh|cosh|tanh|asinh|acosh|atanh|exp|exp2|exp_m1|ln|ln_1p|log|log2|log10|powf|powi|cbrt|hypot)\s*\(/, why: 'a rounded function of the standard library: the game has its own (maths.rs), the same bits as the TypeScript and on every machine' },
  { re: /\.mul_add\s*\(/, why: 'a fused multiply-add rounds once where the TypeScript rounds twice' },
  { re: /\.(sqrt|floor|trunc|ceil|round|round_ties_even|fract|rem_euclid|div_euclid)\s*\(/, why: 'the standard library\'s method: the exact operations come from rust/prim by name (add one there if it is exact by the standard)' },
  { re: /\bf32\b/, why: 'a single where the TypeScript holds a double', but: 'sim/src/single.rs' },   // (the one file that names it: a pose is STORED as the TypeScript's Float32Array stores it)
  { re: /relaxed/i, why: 'relaxed SIMD rounds as the machine likes: it belongs to dressing, never to the simulation' },
  { re: /\b(UnsafeCell|RefCell|Cell|OnceCell|Atomic\w+)\b/, why: 'a static written through a cell: what the crate keeps between calls is a `static mut` with a row in STATICS (the games\' state, scratch or a copy), so the build can say which' },
  { re: /\bextern\s+crate\s+std\b|\bstd::/, why: 'the standard library: the crate is no_std so that no hash map, thread, clock or file can be named' },
];
/** EVERY `static mut` OF THE SIMULATION'S CRATE, AND WHAT IT IS (rust/sim/src/state.rs): STATE = the games' blocks, the one place a game keeps anything from one
 *  call to the next (the host carries a block through every snapshot: src/sim/state.ts) · SCRATCH = written and read inside one call · COPY = the host's own
 *  truth copied in and proven there (the box world). A `static mut` with no row here is red: state kept anywhere but the blocks is in no snapshot. */
const STATICS: Readonly<Record<string, string>> = {
  'sim/src/state.rs ALL': 'STATE · a block a game: the head and a part a family',
  'sim/src/lib.rs SCRATCH': 'SCRATCH · arguments and answers too many for a call\'s own parameters',
  'sim/src/world.rs ALL': 'COPY · the box world, written by src/scene/boxMirror.ts and compared with the boxes as every tick opens; its stamps and lists are scratch of one question',
  'sim/src/fog.rs EYES': 'SCRATCH · the eyes of one sight pass, handed over as the pass opens',
  'sim/src/clips.rs ALL': 'COPY · a character file\'s clips, authored data nothing writes after the file is loaded (src/anim/gltf.ts): copied in once by src/anim/skeleton.ts as a clip is first sampled, the same for every game and in no snapshot; tools/qa/pose-ident.ts reads every single back and samples each clip both ways',
  'sim/src/doors.rs IO': 'SCRATCH · one door step\'s hand-over: the level\'s doors, the contacts and the rules\' numbers written by the host before it asks, the answers read back after (the leaves\' own numbers are in the block)',
  'sim/src/skeleton.rs BENCH': 'SCRATCH · the pose a question is asked of, written, asked and read back',
  'sim/src/light.rs LAMPS': 'SCRATCH · the lamps of one question about the light on a point, written by the host before it asks',
  'sim/src/smoke.rs BLOBS': 'SCRATCH · the smoke blobs of one question about a line, written by the host before it asks',
  'sim/src/nav.rs SHUT': 'SCRATCH · the nav grid\'s shut cells as they stand, written by the host for each search (the grid is the host\'s)',
  'sim/src/nav.rs G': 'SCRATCH · one search\'s scores, left blank after it', 'sim/src/nav.rs F': 'SCRATCH · one search\'s scores, left blank after it',
  'sim/src/nav.rs CAME': 'SCRATCH · one search\'s way back, left blank after it', 'sim/src/nav.rs CLOSED': 'SCRATCH · one search\'s closed cells, left blank after it',
  'sim/src/nav.rs OPEN': 'SCRATCH · one search\'s open list', 'sim/src/nav.rs SEEN': 'SCRATCH · the cells one search wrote, so it can blank them',
  'sim/src/nav.rs CELLS': 'SCRATCH · one way\'s cells before the smoothing', 'sim/src/nav.rs WAY': 'SCRATCH · one answer\'s waypoints, read back by the host',
  'sim/src/nav.rs BLANK': 'SCRATCH · whether the scores have been made infinite once since the module loaded (they load as zeros): a fact of the tables, the same for every game',
  'sim/src/bricks.rs ST': 'COPY · a game\'s smoke on the coarse grid. The truth is the game\'s own array (game/sense.ts SmokeGrid.st, in every snapshot): src/scene/boxMirror.ts gridSync writes it in whenever the module\'s is not that game\'s latest and reads it back whole after every step. It was written before the blocks were and is to become a part of State',
  'sim/src/bricks.rs INDEX': 'COPY · which brick stands where on the plan, made again from ST whenever ST is written in',
  'sim/src/bricks.rs FACES': 'COPY · the walls between a brick\'s cells, asked of the box world\'s copy and asked again wherever a static row or a leaf moved (tools/qa/smokegrid-check.ts `faces`: always what a fresh ask gives)',
  'sim/src/bricks.rs LEAF_AT': 'COPY · the leaves FACES was last asked against', 'sim/src/bricks.rs LEAF_N': 'COPY · how many leaves that was',
  'sim/src/glow.rs NEXT': 'SCRATCH · one pass of the bounce, written before any of it is read back',
  'sim/src/bricks.rs DELTA': 'SCRATCH · one step\'s flows', 'sim/src/bricks.rs THIN': 'SCRATCH · one step\'s thin bricks', 'sim/src/bricks.rs HELD': 'SCRATCH · one step\'s sum a brick',
};
/** the `static mut`s of the simulation's crate that have no row above, and the rows that name none */
export function staticBreaks(): string[] {
  const out: string[] = [], met = new Set<string>();
  for (const f of sources(SIM).filter(f => f.endsWith('.rs'))) {   // (every crate the simulation's module is made of: a static of rust/prim is in its memory too)
    readFileSync(join(WORK, f), 'utf8').split('\n').forEach((raw, i) => { const code = raw.replace(/\/\/.*$/, '');
      if (/^\s*(?:pub(?:\([a-z]+\))?\s+)?static\s+(?!mut\b)\w+\s*:.*\b(\w*Cell|Atomic\w+|Mutex|RwLock|Once\w*|Lazy\w*)\b/.test(code)) out.push(`rust/${f}:${i + 1} — a static written through a cell or a lock: what the module keeps between calls is a \`static mut\` with a row in STATICS`);
      const m = /^\s*(?:pub(?:\([a-z]+\))?\s+)?static\s+mut\s+(\w+)/.exec(raw.replace(/\/\/.*$/, '')); if (!m) return;
      const key = `${f} ${m[1]}`; met.add(key); if (!(key in STATICS)) out.push(`rust/${f}:${i + 1} — \`static mut ${m[1]}\` has no row in tools/sim/build.ts STATICS: a game's state belongs in its block (state.rs), and scratch or a copy says which it is`); });
  }
  for (const k of Object.keys(STATICS)) if (!met.has(k)) out.push(`STATICS names ${k}, which is not there`);
  return out;
}
/** every line of the simulation's crate that breaks a rule, as 'file:line — why' (comments are not code) */
export function ruleBreaks(): string[] {
  const out: string[] = [];
  for (const f of sources(SIM).filter(f => f.startsWith('sim/') && f.endsWith('.rs'))) {
    const lines = readFileSync(join(WORK, f), 'utf8').split('\n');
    lines.forEach((raw, i) => { const code = raw.replace(/\/\/.*$/, ''); for (const R of RULES) if (R.but !== f && R.re.test(code)) out.push(`rust/${f}:${i + 1} — ${R.why}`); });
  }
  return out;
}

/** the crates each crate links, by name: the workspace's folders (a crate is a folder of rust/ with a Cargo.toml) and, for what each depends on, CARGO'S OWN
 *  LIST in rust/Cargo.lock — every build here is `--locked`, so the lock is what is linked, and nobody has to read the many ways a Cargo.toml can spell a
 *  dependency. A package the lock names that is no folder here (a crate from outside) comes back too, and has no side. */
function links(): Record<string, string[]> {
  const out: Record<string, string[]> = {};
  for (const n of readdirSync(WORK).sort()) if (existsSync(join(WORK, n, 'Cargo.toml'))) out[n] = [];
  const lock = existsSync(join(WORK, 'Cargo.lock')) ? readFileSync(join(WORK, 'Cargo.lock'), 'utf8') : '';
  for (const block of lock.split(/^\[\[package\]\]\s*$/m).slice(1)) {
    const name = /^name\s*=\s*"([^"]+)"/m.exec(block)?.[1]; if (!name) continue;
    const list = /^dependencies\s*=\s*\[([^\]]*)\]/m.exec(block)?.[1] ?? '';
    out[name] = [...list.matchAll(/"([^"\s]+)[^"]*"/g)].map(m => m[1]);   // ("name" or "name version (source)": the name)
  }
  return out;
}
/** what is wrong with the sides, a line each ('' = nothing) */
export function sideBreaks(L: Record<string, string[]> = links(), S: Record<string, Side> = SIDES, mods: readonly Mod[] = MODULES): string[] {
  const out: string[] = [];
  for (const [c, deps] of Object.entries(L)) {
    const side = S[c]; if (!side) { out.push(`the crate ${c} has no side: say in tools/sim/build.ts SIDES whether it is gameplay, dressing or both (a crate from outside the workspace has none)`); continue; }
    for (const d of deps) {
      const ds = S[d];
      if (!ds || !(d in L)) out.push(`rust/${c} links ${d}, which is not a crate of this workspace with a side`);
      else if (side === 'both' && ds !== 'both') out.push(`rust/${c} is for both sides and links ${d}, which is ${ds}`);
      else if (side !== 'both' && ds !== 'both' && ds !== side) out.push(`rust/${c} is ${side} and links ${d}, which is ${ds}`);
    }
  }
  for (const m of mods) for (const c of m.crates) if (!(c in L)) out.push(`the ${m.name} module names rust/${c}, and there is no such crate`);
  for (const m of mods) { const want = new Set<string>(), walk = (c: string) => { if (want.has(c)) return; want.add(c); for (const d of L[c] ?? []) walk(d); }; walk(m.name);
    const have = [...m.crates].sort().join(' '), need = [...want].sort().join(' '); if (have !== need) out.push(`the ${m.name} module is made from ${need}, and MODULES says ${have}: its source hash would miss a crate`); }
  return out;
}
/** what a module's bytes break of its side's rules, a line each ('' = nothing) — the simulation: no import, no relaxed SIMD, no threads, every instruction read;
 *  dressing: no import, every instruction read */
export function moduleBreaks(side: 'gameplay' | 'dressing', W: WasmRead): string[] {
  const out: string[] = [];
  if (W.unknown) out.push(`not read whole (${W.unknown}): tools/sim/wasm.ts must be taught it before the module can be vouched for`);
  if (W.imports.length) out.push(`imports ${W.imports.join(', ')}: a module of the game imports nothing`);
  if (side === 'gameplay' && W.relaxed) out.push(`${W.relaxed} relaxed SIMD instruction${W.relaxed > 1 ? 's' : ''}: they round as the machine likes`);
  if (side === 'gameplay' && W.atomics) out.push(`${W.atomics} threads instruction${W.atomics > 1 ? 's' : ''}: an order nobody wrote down`);
  return out;
}

type Verdict = { ok: boolean; label: string; detail: string };
/** the check's verdicts, for tools/qa/rust-check.ts */
export function checkBuilt(): Verdict[] {
  const out: Verdict[] = [];
  const host = builder();
  { const L = links(), br = sideBreaks(L);
    out.push({ ok: br.length === 0, label: 'sides: every crate under rust/ has a side, and neither side links the other', detail: br.length ? br.join('\n    ') : Object.keys(L).map(c => `${c} ${SIDES[c]}${L[c].length ? ` (links ${L[c].join(' ')})` : ''}`).join(' · ') }); }
  { // the reading the next lines stand on, tried where the answer is known: hand-made modules that break each rule, and one that breaks none
    const c = [0xfd, 0x0c, ...new Array<number>(16).fill(0)];
    const relaxed = tinyWasm([...c, ...c, ...c, 0xfd, 0x85, 0x02, 0x1a]), fixed = tinyWasm([...c, ...c, 0xfd, 0xe4, 0x01, 0x1a]), imp = tinyWasm([0x10, 0x00], ['now']), odd = tinyWasm([0xfb, 0x00]), wait = tinyWasm([0xfe, 0x03, 0x00]);
    // (the READER's answers: what the engine here thinks is asked only of the two samples every engine takes — one with no relaxed SIMD refuses that sample)
    const g = (b: Uint8Array) => moduleBreaks('gameplay', readWasm(b)).length, d = (b: Uint8Array) => moduleBreaks('dressing', readWasm(b)).length;
    const hole = tinyWasm([0xfd, 0x9a, 0x01]);
    const real = [fixed, imp].every(b => WebAssembly.validate(b as BufferSource));
    const ok = real && g(hole) === 1 && g(relaxed) === 1 && d(relaxed) === 0 && g(fixed) === 0 && g(imp) === 1 && d(imp) === 1 && g(odd) === 1 && d(odd) === 1 && g(wait) === 1 &&
      sideBreaks({ a: ['b'], b: [] }, { a: 'gameplay', b: 'dressing' }, []).length === 1 && sideBreaks({ a: ['b'], b: [] }, { a: 'dressing', b: 'gameplay' }, []).length === 1 &&
      sideBreaks({ a: [], c: [] }, { a: 'gameplay' }, []).length === 1 && sideBreaks({ a: ['b'], b: [] }, { a: 'gameplay', b: 'both' }, []).length === 0;
    out.push({ ok, label: 'reader: the check sees each breach in a module made by hand', detail: ok ? 'a relaxed SIMD instruction (red for the simulation, allowed for dressing), fixed SIMD (allowed), an import, a threads instruction, an instruction nobody named, a number of the SIMD space that is no instruction, a simulation crate linking a dressing one, a crate with no side' : `it does not: the engine takes the plain hand-made modules ${real} · relaxed ${g(relaxed)}/${d(relaxed)} · fixed ${g(fixed)} · import ${g(imp)}/${d(imp)} · unnamed ${g(odd)}/${d(odd)} · no instruction ${g(hole)} · threads ${g(wait)}` }); }
  for (const m of MODULES) {
    const built = readBuilt(m), now = sourceHash(m), of = `${m.name}: `, src = `source: ${m.out} is the build of rust/${m.name} as it stands`;
    if (!built) { out.push({ ok: false, label: of + src, detail: `${m.out} is missing or not in the shape the build writes: bun run tools/sim/build.ts` }); continue; }
    out.push({ ok: built.source === now, label: of + src, detail: built.source === now ? `sources ${now} (${sources(m).length} files) · ${built.bytes.length} bytes` : `built from ${built.source}, rust/${m.name} is now ${now}: bun run tools/sim/build.ts` });
    out.push({ ok: built.bytes.length <= MAX_BYTES, label: of + 'size: the module is small enough for a page to compile in one step', detail: `${built.bytes.length} bytes of ${MAX_BYTES} (Chrome refuses a synchronous compile on the page's thread past 8 MB)` });
    const lk = leaks(built.bytes);
    out.push({ ok: lk.length === 0, label: of + 'paths: nothing of the building machine is in the module', detail: lk.length ? `found: ${lk.join(' · ')}` : 'no home or checkout path in the bytes' });
    { const W = readWasm(built.bytes), br = moduleBreaks(m.side, W), strict = m.side === 'gameplay';
      if (strict && !WebAssembly.validate(built.bytes as BufferSource)) br.push('the engine here does not take the module');
      out.push({ ok: br.length === 0, label: of + (strict ? 'alone and strict: the module imports nothing, and its code holds no relaxed SIMD and no threads' : 'alone: the module imports nothing (so it cannot reach the simulation\'s memory)'),
        detail: br.length ? br.join('\n    ') : `${W.funcs} functions read whole · imports 0 · its own memory ${W.memories} · fixed SIMD ${W.simd} · relaxed SIMD ${W.relaxed}${strict ? '' : ' (allowed here)'} · threads ${W.atomics}` }); }
    if (m.name === 'sim') {
      { const want = pow10Text(), file = join(CRATE, 'src/pow10.rs'), ok = existsSync(file) && readFileSync(file, 'utf8') === want;
        out.push({ ok, label: of + 'table: the decimal powers the Rust maths read are the language\'s own', detail: ok ? 'rust/sim/src/pow10.rs is what tools/sim/pow10.ts writes' : 'rust/sim/src/pow10.rs is not what tools/sim/pow10.ts writes: bun run tools/sim/pow10.ts, then bun run tools/sim/build.ts' }); }
      { const br = staticBreaks();
        out.push({ ok: br.length === 0, label: of + 'statics: every `static mut` of the simulation\'s crate is the games\' state, scratch of one call or a proven copy', detail: br.length ? br.join('\n    ') : Object.entries(STATICS).map(([k, v]) => `${k.replace('sim/src/', '')}: ${v.split(' · ')[0]}`).join(' · ') }); }
      { const br = ruleBreaks();
        out.push({ ok: br.length === 0, label: of + 'rules: no line of the simulation\'s crate names what it may not', detail: br.length ? br.slice(0, 6).join('\n    ') + (br.length > 6 ? `\n    … and ${br.length - 6} more` : '') : `${RULES.length} rules over rust/sim/src: nothing found` }); }
    }
    const bl = of + 'bytes: the crate built again gives the checked-in bytes';
    if (host === null) out.push({ ok: true, label: bl, detail: 'NOT RUN — no rustc on this machine; the source hash above is what holds here' });
    else {
      const again = compile(true, m, host);
      if (!again) out.push({ ok: false, label: bl, detail: `rust/${m.name} does not build (above)` });
      else if (same(again, built.bytes)) out.push({ ok: true, label: bl, detail: `${again.length} bytes, the same, by ${host}` });
      else if (host !== built.builder) out.push({ ok: true, label: bl, detail: `NOT THE SAME, and not held: here it is ${host}, the file was built by ${built.builder} (${again.length} bytes here, ${built.bytes.length} there)` });
      else out.push({ ok: false, label: bl, detail: `${again.length} bytes here, ${built.bytes.length} in the file, and they differ under the same builder (${host}): the file was edited by hand, or built with other flags` });
    }
  }
  return out;
}

if (import.meta.main) {
  if (process.argv.includes('--check')) {
    let bad = 0;
    for (const v of checkBuilt()) { if (!v.ok) bad++; console.log(`- ${v.ok ? 'PASS' : 'FAIL'} — ${v.label}\n    ${v.detail}`); }
    process.exit(bad ? 1 : 0);
  }
  { const br = sideBreaks(); if (br.length) { console.error(`rust build — ${br.join('\n             ')}`); process.exit(1); } }
  const by = builder(), made: { m: Mod; bytes: Uint8Array }[] = [];
  for (const m of MODULES) {   // every module compiled and vetted BEFORE the first is written: the two built.ts files move together or not at all
    const bytes = compile(false, m, by), say = `${m.name} build —`;
    if (!bytes) process.exit(1);
    if (bytes.length > MAX_BYTES) { console.error(`${say} the module is ${bytes.length} bytes, over the ${MAX_BYTES} a page may compile in one step: nothing written`); process.exit(1); }
    const lk = leaks(bytes); if (lk.length) { console.error(`${say} the module carries a path of this machine (${lk.join(' · ')}): nothing written`); process.exit(1); }
    if (!WebAssembly.validate(bytes as BufferSource)) { console.error(`${say} the module does not validate: nothing written`); process.exit(1); }
    const mb = moduleBreaks(m.side, readWasm(bytes)); if (mb.length) { console.error(`${say} the module does not keep its side of the line: nothing written\n    ${mb.join('\n    ')}`); process.exit(1); }
    made.push({ m, bytes });
  }
  for (const { m, bytes } of made) {
    const before = readBuilt(m);
    write(m, bytes, by ?? '');
    console.log(`${m.name} build — ${m.out}: ${bytes.length} bytes, sources ${sourceHash(m)}${before && same(before.bytes, bytes) ? ' (the same bytes as before)' : ''}`);
  }
}
