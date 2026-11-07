// THE RUST GAME'S MODULE FOR THE PAGE (rust/game_face → game_face.wasm, FACE.md): where the file is, whether it is older than its sources, and the build.
// A build's file (20 to 27 MB, under target/) is never checked in. The dev server (tools/serve.ts) answers /game_face.wasm with it and the static
// build (tools/build.ts) copies it into dist/; both build it first when a source under rust/ is newer than the file. Where it cannot be built (no cargo, no
// wasm target) the answer is the reason in plain words WITH THE ONE COMMAND THAT MENDS IT, which the page shows under `?rust=1` (src/host/rustgame.ts);
// without the word the page never asks.
// A CHECKED-IN MODULE FOR A MACHINE THAT CANNOT BUILD (rust/game_face/prebuilt/game_face.wasm, with SOURCES beside it: a hash of every source file the
// module is made from). Where no build of this machine is up to date and the checked-in module's hash IS this checkout's sources' hash, it is served and
// cargo is never started: a pull is enough to play. Where the hashes differ it is not served (the page's half of the bridge is generated with the Rust
// and a module of other sources would read garbage): the machine builds, or says why it cannot. `--prebuilt` makes it: the `play` profile built with the
// checkout's and the home folder's paths mapped away (the file must not carry a machine's paths), copied there with its hash.
// A HOSTED BUILD MAKES ITS OWN (tools/host/build.sh sets FACE_BUILD=1): with FACE_BUILD=1 the checked-in module is never taken, the module is built from
// the sources on that machine (paths mapped away), and the static build (tools/build.ts) FAILS if it cannot be.
// TWO PROFILES (rust/game_face/Cargo.toml). `play`: optimised, no link-time optimisation: what the DEV SERVER builds, because the release profile's fat
// link needs over 10.5 GB of memory and many minutes, and a first `bun run dev` should not. `release`: the fat link, what `bun run build` ships
// (FACE_PROFILE=play in front of it builds the other). A module of EITHER profile that is newer than the sources is served as it is: nobody is made to
// wait for a second build of the same tree.
// THE DEV SERVER DOES NOT HOLD THE PAGE'S REQUEST WHILE IT BUILDS: `faceAsk` starts the build and answers at once with what is going on (`faceNow`),
// and the page asks again every two seconds and shows the line.
//   bun run tools/blitz/face-build.ts [--check] [--profile play|release]     build if stale and say where the file is · --check: only say whether it is stale
//   bun run tools/blitz/face-build.ts --prebuilt | --where                    make the checked-in module · say which module this machine would serve
import { existsSync, readdirSync, statSync, readFileSync, writeFileSync, mkdirSync, copyFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { join, resolve } from 'node:path';
import { homedir } from 'node:os';

const ROOT = resolve(import.meta.dir, '../..'), RUST = join(ROOT, 'rust'), CRATE = join(RUST, 'game_face');
export type FaceProfile = 'play' | 'release';
export const faceFile = (profile: FaceProfile): string => join(CRATE, `target/wasm32-unknown-unknown/${profile}/game_face.wasm`);
/** the release profile's file (the mission loop through wasm and the simulation's build name it) */
export const FACE_WASM = faceFile('release');
/** the crates the module is made from (tools/sim/build.ts FILE_MODULES holds this list to the crate's own lock) */
export const FACE_CRATES = ['prim', 'sim', 'game', 'game_run', 'game_face'] as const;

/** the checked-in module and the hash of the sources it was built from */
export const PREBUILT = join(CRATE, 'prebuilt/game_face.wasm'), PREBUILT_SOURCES = join(CRATE, 'prebuilt/SOURCES');
/** the newest source the module is made from: its time and its path (and, in `files`, every source file) */
function newest(): { t: number; file: string; files: string[] } {
  let best = { t: 0, file: '', files: [] as string[] };
  const see = (p: string) => { best.files.push(p); const t = statSync(p).mtimeMs; if (t > best.t) { best.t = t; best.file = p; } };
  const walk = (dir: string) => { for (const n of readdirSync(dir)) { const p = join(dir, n); if (statSync(p).isDirectory()) walk(p); else if (n.endsWith('.rs')) see(p); } };
  for (const c of FACE_CRATES) { const d = join(RUST, c); if (!existsSync(d)) continue; for (const f of ['Cargo.toml', 'build.rs']) if (existsSync(join(d, f))) see(join(d, f)); if (existsSync(join(d, 'src'))) walk(join(d, 'src')); }
  if (existsSync(join(RUST, 'rust-toolchain.toml'))) see(join(RUST, 'rust-toolchain.toml'));
  return best;
}
/** a hash of every source file the module is made from (its path under rust/ and its bytes), the same on any machine with the same checkout */
let hashed: { t: number; n: number; hash: string } | null = null;
export function sourcesHash(): string {
  const n = newest(); if (hashed && hashed.t === n.t && hashed.n === n.files.length) return hashed.hash;
  const h = createHash('sha256'); for (const f of n.files.map(f => f.slice(RUST.length + 1).split('\\').join('/')).sort()) { h.update(f); h.update('\0'); h.update(readFileSync(join(RUST, f))); h.update('\0'); }
  hashed = { t: n.t, n: n.files.length, hash: h.digest('hex') }; return hashed.hash;
}
/** the checked-in module where it was built from exactly this checkout's sources, else null */
export function prebuilt(): string | null {
  if (!existsSync(PREBUILT) || !existsSync(PREBUILT_SOURCES)) return null;
  return readFileSync(PREBUILT_SOURCES, 'utf8').trim().split(/\s+/)[0] === sourcesHash() ? PREBUILT : null;
}
/** '' = the profile's file is there and no source is newer; else why it is stale */
export function faceStale(profile: FaceProfile = 'release'): string {
  const f = faceFile(profile); if (!existsSync(f)) return 'not built yet';
  const n = newest(), t = statSync(f).mtimeMs;
  return n.t > t ? `${n.file.slice(ROOT.length + 1)} is newer than the module` : '';
}
/** a module that is up to date, the asked profile's first: its path, or null */
function fresh(profile: FaceProfile): string | null { for (const p of [profile, profile === 'play' ? 'release' : 'play'] as FaceProfile[]) if (!faceStale(p)) return faceFile(p); return process.env.FACE_BUILD === '1' ? null : prebuilt(); }
/** the module a headless tool plays: a build of this machine that is up to date (release first, then play), else the checked-in one where it is of these sources; null = none */
export const faceHere = (): string | null => fresh('release');
/** the compiler rust/rust-toolchain.toml picks: rustup's cargo (the system's own cannot link wasm where it has no lld) */
function cargo(): string | null {
  const own = join(homedir(), '.cargo/bin/cargo'); if (existsSync(own)) return own;
  return Bun.which('cargo');
}
/** THE ONE COMMAND for a machine with no cargo, and for one whose cargo cannot build for wasm */
const RUSTUP = `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`;
/** what a machine that could not build is told when a checked-in module is there but is of other sources */
const otherSources = (): string => existsSync(PREBUILT) && !prebuilt() ? ' The module checked in (rust/game_face/prebuilt) was built from other sources than this checkout has under rust/, so it is not served: pull again, or have it made again where cargo runs (bun run tools/blitz/face-build.ts --prebuilt).' : '';
const NO_CARGO = `The Rust game is not built, and this machine has no cargo to build it. Install Rust with rustup (one line in a terminal: ${RUSTUP} ), then load this page again: the dev server builds the module itself (rust/rust-toolchain.toml names the compiler and the wasm32-unknown-unknown target, which rustup fetches on the first build).`;
type Answer = { path: string } | { why: string };
/** what the build is doing now, for whoever shows it: `building` with its seconds so far and cargo's last line, or the last build's failure */
const NOW = { building: false, since: 0, line: '', profile: 'play' as FaceProfile, failed: '', failedAt: 0 };
export function faceNow(): { building: boolean; secs: number; line: string; profile: FaceProfile; failed: string } { return { building: NOW.building, secs: NOW.building ? Math.round((performance.now() - NOW.since) / 1000) : 0, line: NOW.line, profile: NOW.profile, failed: NOW.failed }; }
let building: Promise<Answer> | null = null;
let saidNoCargo = false;
/** `--prebuilt`: the build maps the checkout's and the home folder's paths away, so the file it makes carries no machine's paths */
let MAPPED = false;
/** THE MODULE'S FILE, built first where it is stale: `{ path }`, or `{ why }` in words a player can be shown. One build at a time; a second ask waits on the first. */
export function faceWasm(say: (line: string) => void = () => {}, profile: FaceProfile = (process.env.FACE_PROFILE === 'play' ? 'play' : 'release')): Promise<Answer> {
  const have = fresh(profile); if (have) return Promise.resolve({ path: have });
  return building ??= (async (): Promise<Answer> => {
    const stale = faceStale(profile), file = faceFile(profile), c = cargo();
    // (no compiler: an older module of either profile is better than none, and the server says that it is old)
    if (!c) { const old = [file, faceFile(profile === 'play' ? 'release' : 'play')].find(f => existsSync(f)); if (old) { say(`game_face.wasm: ${stale}, and there is no cargo on this machine to build it: the file as it stands is served (${old.slice(ROOT.length + 1)})`); return { path: old }; } const why = NO_CARGO + otherSources(); if (!saidNoCargo) { saidNoCargo = true; say(`game_face.wasm: ${why}`); } return { why }; }
    say(`game_face.wasm: ${stale}; building it (cargo build --profile ${profile} in rust/game_face: ${profile === 'play' ? 'a few minutes the first time, and about 7 GB of memory' : 'the fat link: many minutes and over 10 GB of memory'}) …`);
    const t0 = performance.now(); NOW.building = true; NOW.since = t0; NOW.line = 'starting cargo'; NOW.profile = profile; NOW.failed = '';
    const p = Bun.spawn([c, 'build', '--target', 'wasm32-unknown-unknown', '--profile', profile], { cwd: CRATE, stdout: 'ignore', stderr: 'pipe', env: { ...process.env, PATH: `${join(homedir(), '.cargo/bin')}:${process.env.PATH ?? ''}`, ...(MAPPED || process.env.FACE_BUILD === '1' ? { RUSTFLAGS: `${process.env.RUSTFLAGS ?? ''} --remap-path-prefix=${ROOT}=. --remap-path-prefix=${homedir()}=~`.trim() } : {}) } });
    // cargo's own words as they come: the crates it compiles and what rustup fetches are said, the compiler's warnings are not
    let err = '', part = '', lastSaid = 0;
    const dec = new TextDecoder();
    for await (const chunk of p.stderr as unknown as AsyncIterable<Uint8Array>) { const text = dec.decode(chunk, { stream: true }); err += text; if (err.length > 4_000_000) err = err.slice(-2_000_000);
      const lines = (part + text).split('\n'); part = lines.pop() ?? '';
      for (const l of lines) { const m = l.match(/^\s*(Compiling|Finished|Downloading|Downloaded|Updating|Blocking|info: (?:syncing|downloading|installing).*|error(?:\[E\d+\])?:.*)\b(.*)$/); if (!m) continue; NOW.line = l.trim().replace(/\s*\(.*\)$/, '').slice(0, 120); const t = performance.now(); if (/^\s*(error|info:)/.test(l) || t - lastSaid > 1500) { say(`game_face.wasm: ${NOW.line} (${((t - t0) / 1000).toFixed(0)} s)`); lastSaid = t; } } }
    const code = await p.exited; NOW.building = false;
    if (code !== 0) {
      const first = err.split('\n').filter(l => /^error/.test(l)).slice(0, 3).join(' · ') || err.trim().split('\n').slice(-2).join(' · ');
      // (the commonest ways a machine that HAS a cargo cannot build this: a compiler with no wasm target or no linker for it, where rustup's own would have
      //  both; and a compiler killed for memory)
      const noTarget = /linker `lld` not found|target may not be installed|can't find crate for `(core|std)`/.test(err), killed = /signal: 9|SIGKILL/.test(err);
      const hint = noTarget ? ` This cargo (${c}) cannot build for wasm32-unknown-unknown. If it is rustup's: rustup target add wasm32-unknown-unknown . If it is not (a system package): install rustup (${RUSTUP} ), whose toolchain, pinned in rust/rust-toolchain.toml, brings the target and its linker.`
        : killed ? ' The compiler was killed, most likely for memory (this build needs about 7 GB free): close what else is running and load the page again.' : '';
      NOW.failed = `The Rust game did not build: ${first.slice(0, 400)}${hint}${otherSources()}`; NOW.failedAt = performance.now();
      say(`game_face.wasm: the build failed after ${((performance.now() - t0) / 1000).toFixed(0)} s: ${first}${hint}`);
      return { why: NOW.failed };
    }
    say(`game_face.wasm: built in ${((performance.now() - t0) / 1000).toFixed(0)} s, ${statSync(file).size} bytes (${file.slice(ROOT.length + 1)})`);
    return { path: file };
  })().finally(() => { building = null; });
}
/** FOR A SERVER THAT MUST NOT HOLD A REQUEST FOR MINUTES: the file if one is up to date; else the build is started (if none runs) and the answer is what
 *  is going on now: `{ wait }` = a line for the page to show while it asks again, `{ why }` = it cannot be built and why. A build that failed is not
 *  started again by every ask: its reason stands for ten seconds, and a reload after that tries once more. */
export function faceAsk(say: (line: string) => void = () => {}, profile: FaceProfile = 'play'): Answer | { wait: string } {
  const have = building ? null : fresh(profile); if (have) return { path: have };   // (while a build runs its file may be half written: nothing is served until it has ended)
  if (!building && NOW.failed && performance.now() - NOW.failedAt < 10_000) return { why: NOW.failed };
  let settled: Answer | null = null;
  void faceWasm(say, profile).then(r => { settled = r; });
  if (settled) return settled;   // (never: the build's answer comes a turn later; kept for a cargo-less machine's quick answer below)
  if (!cargo()) { const old = [faceFile(profile), faceFile(profile === 'play' ? 'release' : 'play')].find(f => existsSync(f)); return old ? { path: old } : { why: NO_CARGO + otherSources() }; }
  const n = faceNow();
  return { wait: `Building the Rust game: ${n.secs} s so far${n.line ? ` (${n.line})` : ''}. The first build takes a few minutes; the terminal that runs the dev server shows it. This page starts by itself when it is done.` };
}

if (import.meta.main) {
  const at = process.argv.indexOf('--profile'), profile: FaceProfile = (at > 0 ? process.argv[at + 1] : process.env.FACE_PROFILE) === 'play' ? 'play' : 'release';
  if (process.argv.includes('--check')) { const s = faceStale(profile), f = faceFile(profile); console.log(s ? `game_face.wasm (${profile}): ${s}` : `game_face.wasm (${profile}): up to date (${statSync(f).size} bytes)`); process.exit(s ? 1 : 0); }
  if (process.argv.includes('--prebuilt')) {
    // (always a build of its own flags: cargo sees the flags changed and compiles the crates again; the file is then read for any path of this machine)
    MAPPED = true; process.env.FACE_BUILD = '1';
    const f = faceFile('play'); const was = existsSync(f) ? statSync(f).mtimeMs : 0;
    const r = await faceWasm(l => console.log(l), 'play'); if ('why' in r) { console.error(r.why); process.exit(1); }
    if (r.path !== f || statSync(f).mtimeMs === was) { console.error(`--prebuilt: no new build was made (${r.path.slice(ROOT.length + 1)} was already up to date): touch a source or remove the file, and run again`); process.exit(1); }
    const bytes = readFileSync(f), text = bytes.toString('latin1'); for (const bad of [ROOT, homedir()]) if (text.includes(bad)) { console.error(`--prebuilt: the module still carries a path of this machine (${bad}): not copied`); process.exit(1); }
    mkdirSync(join(CRATE, 'prebuilt'), { recursive: true }); copyFileSync(f, PREBUILT); writeFileSync(PREBUILT_SOURCES, `${sourcesHash()}\n`);
    console.log(`${PREBUILT.slice(ROOT.length + 1)}: ${bytes.length} bytes, sources ${sourcesHash().slice(0, 12)}`); process.exit(0);
  }
  if (process.argv.includes('--where')) { const p = prebuilt(), s = faceStale(profile); console.log(`a build of this machine (${profile}): ${s || 'up to date'} · the checked-in module: ${p ? 'of these sources' : existsSync(PREBUILT) ? 'of OTHER sources' : 'none'} · sources ${sourcesHash().slice(0, 12)}`); process.exit(0); }
  const r = await faceWasm(l => console.log(l), profile);
  if ('why' in r) { console.error(r.why); process.exit(1); }
  console.log(r.path.slice(ROOT.length + 1));
}
