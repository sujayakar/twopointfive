// THE RUST SIMULATION'S BUILD — rust/sim (a Rust crate) → WebAssembly → src/sim/built.ts, the module's bytes as text, which is checked in.
//   bun run tools/sim/build.ts            build and write src/sim/built.ts (needs rustup: rust/rust-toolchain.toml names the compiler and the target, and rustup
//                                         fetches both on the first build)
//   bun run tools/sim/build.ts --check    is src/sim/built.ts the build of rust/sim as it stands? exit 0 yes · 1 no (which way is said)
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

const ROOT = resolve(import.meta.dir ?? new URL('.', import.meta.url).pathname, '../..');
const WORK = join(ROOT, 'rust'), CRATE = join(WORK, 'sim'), OUT = join(ROOT, 'src/sim/built.ts'), TARGET = join(WORK, 'target'), WASM = join(TARGET, 'wasm32-unknown-unknown/release/sim.wasm');
/** the crates the module is made from (rust/Cargo.toml's members that the simulation links) */
const CRATES = ['prim', 'sim'];
/** Chrome compiles a module on the page's own thread only up to 8 MB (src/sim/sim.ts boots in one synchronous step): the build is red well short of it */
export const MAX_BYTES = 6 * 1024 * 1024;

/** every file the module is made from, by path */
function sources(): string[] {
  const out: string[] = ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml'];   // (the workspace's three decide the compiler and its settings)
  const walk = (dir: string) => { for (const n of readdirSync(dir).sort()) { const p = join(dir, n); if (statSync(p).isDirectory()) walk(p); else if (n.endsWith('.rs')) out.push(relative(WORK, p).split('\\').join('/')); } };
  for (const c of CRATES) { out.push(`${c}/Cargo.toml`); walk(join(WORK, c, 'src')); }
  return out;
}
export function sourceHash(): string {
  const h = createHash('sha256');
  for (const f of sources()) { h.update(f + '\0'); h.update(readFileSync(join(WORK, f))); h.update('\0'); }
  return h.digest('hex').slice(0, 32);
}
/** who would build it here: the compiler rust/ picks on this machine and the kind of machine, as one word ('rustc 1.97.0 on x86_64-unknown-linux-gnu'); null = no rustc */
function builder(): string | null {
  const r = spawnSync('rustc', ['-vV'], { cwd: WORK, encoding: 'utf8' });
  if (r.error || r.status !== 0) return null;
  const host = /^host: (.+)$/m.exec(r.stdout)?.[1], release = /^release: (.+)$/m.exec(r.stdout)?.[1];
  return host && release ? `rustc ${release} on ${host}` : null;
}
/** the crate compiled: the module's bytes (null with the reason said where there is no compiler) */
export function compile(quiet = false): Uint8Array | null {
  if (builder() === null) { if (!quiet) console.error('sim build — no rustc on this machine (https://rustup.rs; the toolchain file in rust/ names the rest)'); return null; }
  const home = homedir();
  // THE FLAGS ARE THESE AND NO OTHERS (the encoded form: a path with a space in it stays one flag, and it stands over a RUSTFLAGS of the shell's and a cargo
  // config's); the output goes where this file reads it whatever CARGO_TARGET_DIR says.
  const flags = [`${ROOT}=.`, `${join(home, '.cargo')}=/cargo`, `${join(home, '.rustup')}=/rustup`].map(m => `--remap-path-prefix=${m}`).join('\x1f');
  const r = spawnSync('cargo', ['build', '--release', '--locked', '-p', 'sim', '--target', 'wasm32-unknown-unknown', '--target-dir', TARGET], { cwd: WORK, stdio: quiet ? 'pipe' : 'inherit', encoding: 'utf8', env: { ...process.env, CARGO_ENCODED_RUSTFLAGS: flags } });
  if (r.status !== 0) { if (quiet) console.error(r.stderr); return null; }
  if (!existsSync(WASM)) { console.error(`sim build — cargo said it built and ${relative(ROOT, WASM)} is not there`); return null; }
  return new Uint8Array(readFileSync(WASM));
}
function leaks(bytes: Uint8Array): string[] {
  const text = Buffer.from(bytes).toString('latin1');
  return [homedir(), ROOT, '/home/', '/Users/', '/root/', 'C:\\Users'].filter((p, i, a) => p.length > 3 && a.indexOf(p) === i && text.includes(p));
}
interface Built { source: string; builder: string; bytes: Uint8Array }
function readBuilt(): Built | null {
  if (!existsSync(OUT)) return null;
  const t = readFileSync(OUT, 'utf8');
  const source = /SIM_SOURCE = '([0-9a-f]+)'/.exec(t)?.[1], by = /SIM_BUILDER = '([^']*)'/.exec(t)?.[1];
  const b64 = [...t.matchAll(/^  '([A-Za-z0-9+/=]+)',$/gm)].map(m => m[1]).join('');
  if (!source || by === undefined || !b64) return null;
  return { source, builder: by, bytes: new Uint8Array(Buffer.from(b64, 'base64')) };
}
function write(bytes: Uint8Array, by: string): void {
  const b64 = Buffer.from(bytes).toString('base64'), lines: string[] = [];
  for (let i = 0; i < b64.length; i += 160) lines.push(`  '${b64.slice(i, i + 160)}',`);
  writeFileSync(OUT, `// WRITTEN BY tools/sim/build.ts — never by hand. The Rust simulation (rust/sim) built to WebAssembly, its bytes as base64: src/sim/sim.ts loads it.
// \`bun run tools/sim/build.ts\` after any change under rust/; \`bun run tools/sim/build.ts --check\` (and tools/qa/rust-check.ts) is red when this file is not
// the build of rust/sim as it stands.
/** the hash of the crates' sources this was built from */
export const SIM_SOURCE = '${sourceHash()}';
/** the compiler and the kind of machine that built it (the same sources built by another may give other bytes) */
export const SIM_BUILDER = '${by}';
/** the module's size in bytes */
export const SIM_BYTES = ${bytes.length};
export const SIM_WASM: readonly string[] = [
${lines.join('\n')}
];
`);
}
const same = (a: Uint8Array, b: Uint8Array) => a.length === b.length && a.every((x, i) => x === b[i]);

/** WHAT NO LINE OF THE SIMULATION'S CRATE MAY NAME (rust/sim/src/lib.rs: what it owes the game) — the part the compiler does not hold. The crate is `no_std`,
 *  which keeps out every `std::` path; but once any crate of the module links the standard library its float methods resolve everywhere, so those are held here. */
const RULES: readonly { re: RegExp; why: string }[] = [
  { re: /\.(sin|cos|tan|asin|acos|atan|atan2|sin_cos|sinh|cosh|tanh|asinh|acosh|atanh|exp|exp2|exp_m1|ln|ln_1p|log|log2|log10|powf|powi|cbrt|hypot)\s*\(/, why: 'a rounded function of the standard library: the game has its own (maths.rs), the same bits as the TypeScript and on every machine' },
  { re: /\.mul_add\s*\(/, why: 'a fused multiply-add rounds once where the TypeScript rounds twice' },
  { re: /\.(sqrt|floor|trunc|ceil|round|round_ties_even|fract|rem_euclid|div_euclid)\s*\(/, why: 'the standard library\'s method: the exact operations come from rust/prim by name (add one there if it is exact by the standard)' },
  { re: /\bf32\b/, why: 'a single where the TypeScript holds a double' },
  { re: /relaxed/i, why: 'relaxed SIMD rounds as the machine likes: it belongs to dressing, never to the simulation' },
  { re: /\bextern\s+crate\s+std\b|\bstd::/, why: 'the standard library: the crate is no_std so that no hash map, thread, clock or file can be named' },
];
/** every line of the simulation's crate that breaks a rule, as 'file:line — why' (comments are not code) */
export function ruleBreaks(): string[] {
  const out: string[] = [];
  for (const f of sources().filter(f => f.startsWith('sim/') && f.endsWith('.rs'))) {
    const lines = readFileSync(join(WORK, f), 'utf8').split('\n');
    lines.forEach((raw, i) => { const code = raw.replace(/\/\/.*$/, ''); for (const R of RULES) if (R.re.test(code)) out.push(`rust/${f}:${i + 1} — ${R.why}`); });
  }
  return out;
}

/** the check's verdicts, for tools/qa/rust-check.ts */
export function checkBuilt(): { ok: boolean; label: string; detail: string }[] {
  const out: { ok: boolean; label: string; detail: string }[] = [];
  const built = readBuilt(), now = sourceHash();
  if (!built) return [{ ok: false, label: 'source: src/sim/built.ts is the build of rust/sim as it stands', detail: 'src/sim/built.ts is missing or not in the shape the build writes: bun run tools/sim/build.ts' }];
  out.push({ ok: built.source === now, label: 'source: src/sim/built.ts is the build of rust/sim as it stands', detail: built.source === now ? `sources ${now} (${sources().length} files) · ${built.bytes.length} bytes` : `built from ${built.source}, rust/sim is now ${now}: bun run tools/sim/build.ts` });
  out.push({ ok: built.bytes.length <= MAX_BYTES, label: 'size: the module is small enough for a page to compile in one step', detail: `${built.bytes.length} bytes of ${MAX_BYTES} (Chrome refuses a synchronous compile on the page's thread past 8 MB)` });
  const lk = leaks(built.bytes);
  out.push({ ok: lk.length === 0, label: 'paths: nothing of the building machine is in the module', detail: lk.length ? `found: ${lk.join(' · ')}` : 'no home or checkout path in the bytes' });
  { const want = pow10Text(), file = join(CRATE, 'src/pow10.rs'), ok = existsSync(file) && readFileSync(file, 'utf8') === want;
    out.push({ ok, label: 'table: the decimal powers the Rust maths read are the language\'s own', detail: ok ? 'rust/sim/src/pow10.rs is what tools/sim/pow10.ts writes' : 'rust/sim/src/pow10.rs is not what tools/sim/pow10.ts writes: bun run tools/sim/pow10.ts, then bun run tools/sim/build.ts' }); }
  { const br = ruleBreaks();
    out.push({ ok: br.length === 0, label: 'rules: no line of the simulation\'s crate names what it may not', detail: br.length ? br.slice(0, 6).join('\n    ') + (br.length > 6 ? `\n    … and ${br.length - 6} more` : '') : `${RULES.length} rules over rust/sim/src: nothing found` }); }
  const host = builder();
  if (host === null) out.push({ ok: true, label: 'bytes: the crate built again gives the checked-in bytes', detail: 'NOT RUN — no rustc on this machine; the source hash above is what holds here' });
  else {
    const again = compile(true);
    if (!again) out.push({ ok: false, label: 'bytes: the crate built again gives the checked-in bytes', detail: 'rust/sim does not build (above)' });
    else if (same(again, built.bytes)) out.push({ ok: true, label: 'bytes: the crate built again gives the checked-in bytes', detail: `${again.length} bytes, the same, by ${host}` });
    else if (host !== built.builder) out.push({ ok: true, label: 'bytes: the crate built again gives the checked-in bytes', detail: `NOT THE SAME, and not held: here it is ${host}, the file was built by ${built.builder} (${again.length} bytes here, ${built.bytes.length} there)` });
    else out.push({ ok: false, label: 'bytes: the crate built again gives the checked-in bytes', detail: `${again.length} bytes here, ${built.bytes.length} in the file, and they differ under the same builder (${host}): the file was edited by hand, or built with other flags` });
  }
  return out;
}

if (import.meta.main) {
  if (process.argv.includes('--check')) {
    let bad = 0;
    for (const v of checkBuilt()) { if (!v.ok) bad++; console.log(`- ${v.ok ? 'PASS' : 'FAIL'} — ${v.label}\n    ${v.detail}`); }
    process.exit(bad ? 1 : 0);
  }
  const bytes = compile();
  if (!bytes) process.exit(1);
  if (bytes.length > MAX_BYTES) { console.error(`sim build — the module is ${bytes.length} bytes, over the ${MAX_BYTES} a page may compile in one step`); process.exit(1); }
  const lk = leaks(bytes); if (lk.length) { console.error(`sim build — the module carries a path of this machine (${lk.join(' · ')}): not written`); process.exit(1); }
  if (!WebAssembly.validate(bytes as BufferSource)) { console.error('sim build — the module does not validate'); process.exit(1); }
  const before = readBuilt();
  write(bytes, builder() ?? '');
  console.log(`sim build — src/sim/built.ts: ${bytes.length} bytes, sources ${sourceHash()}${before && same(before.bytes, bytes) ? ' (the same bytes as before)' : ''}`);
}
