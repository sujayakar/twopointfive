// THE RUST GAME'S MODULE FOR THE PAGE (rust/game_face → game_face.wasm, FACE.md): where the file is, whether it is older than its sources, and the build.
// The file is 12 to 19 MB and is never checked in (rust/.gitignore: target/). The dev server (tools/serve.ts) answers /game_face.wasm with it and the static
// build (tools/build.ts) copies it into dist/; both build it first when a source under rust/ is newer than the file. Where it cannot be built (no cargo, no
// wasm target) the answer is the reason in plain words, which the page shows under `?rust=1` (src/host/rustgame.ts); without the word the page never asks.
//   bun run tools/blitz/face-build.ts [--check]     build if stale and say where the file is · --check: only say whether it is stale
import { existsSync, readdirSync, statSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { homedir } from 'node:os';

const ROOT = resolve(import.meta.dir, '../..'), RUST = join(ROOT, 'rust'), CRATE = join(RUST, 'game_face');
export const FACE_WASM = join(CRATE, 'target/wasm32-unknown-unknown/release/game_face.wasm');
/** the crates the module is made from (tools/sim/build.ts FILE_MODULES holds this list to the crate's own lock) */
export const FACE_CRATES = ['prim', 'sim', 'game', 'game_run', 'game_face'] as const;

/** the newest source the module is made from: its time and its path */
function newest(): { t: number; file: string } {
  let best = { t: 0, file: '' };
  const see = (p: string) => { const t = statSync(p).mtimeMs; if (t > best.t) best = { t, file: p }; };
  const walk = (dir: string) => { for (const n of readdirSync(dir)) { const p = join(dir, n); if (statSync(p).isDirectory()) walk(p); else if (n.endsWith('.rs')) see(p); } };
  for (const c of FACE_CRATES) { const d = join(RUST, c); if (!existsSync(d)) continue; for (const f of ['Cargo.toml', 'build.rs']) if (existsSync(join(d, f))) see(join(d, f)); if (existsSync(join(d, 'src'))) walk(join(d, 'src')); }
  if (existsSync(join(RUST, 'rust-toolchain.toml'))) see(join(RUST, 'rust-toolchain.toml'));
  return best;
}
/** '' = the file is there and no source is newer; else why it is stale */
export function faceStale(): string {
  if (!existsSync(FACE_WASM)) return 'not built yet';
  const n = newest(), t = statSync(FACE_WASM).mtimeMs;
  return n.t > t ? `${n.file.slice(ROOT.length + 1)} is newer than the module` : '';
}
/** the compiler rust/rust-toolchain.toml picks: rustup's cargo (the system's own cannot link wasm where it has no lld) */
function cargo(): string | null {
  const own = join(homedir(), '.cargo/bin/cargo'); if (existsSync(own)) return own;
  return Bun.which('cargo');
}
let building: Promise<{ path: string } | { why: string }> | null = null;
/** THE MODULE'S FILE, built first where it is stale: `{ path }`, or `{ why }` in words a player can be shown. One build at a time; a second ask waits on the first. */
export function faceWasm(say: (line: string) => void = () => {}): Promise<{ path: string } | { why: string }> {
  const stale = faceStale(); if (!stale) return Promise.resolve({ path: FACE_WASM });
  return building ??= (async () => {
    const c = cargo();
    if (!c) return existsSync(FACE_WASM) ? (say(`game_face.wasm: ${stale}, and there is no cargo on this machine to build it: the file as it stands is served`), { path: FACE_WASM }) : { why: 'The Rust game is not built, and this machine has no cargo to build it (rustup, then the wasm32-unknown-unknown target).' };
    say(`game_face.wasm: ${stale}; building (cargo build --release in rust/game_face: minutes) …`);
    const t0 = performance.now();
    const p = Bun.spawn([c, 'build', '--target', 'wasm32-unknown-unknown', '--release'], { cwd: CRATE, stdout: 'pipe', stderr: 'pipe', env: { ...process.env, PATH: `${join(homedir(), '.cargo/bin')}:${process.env.PATH ?? ''}` } });
    const [err, code] = await Promise.all([new Response(p.stderr).text(), p.exited]);
    if (code !== 0) {
      const first = err.split('\n').filter(l => /^error/.test(l)).slice(0, 3).join(' · ') || err.trim().split('\n').slice(-2).join(' · ');
      // (the commonest way a machine that HAS a cargo cannot build this: a compiler with no wasm target or no linker for it, where rustup's own would have both)
      const hint = /linker `lld` not found|target may not be installed|can't find crate for `(core|std)`/.test(err) ? ` (this cargo, ${c}, cannot build for wasm32-unknown-unknown: rustup's toolchain, pinned in rust/rust-toolchain.toml, brings the target and its linker)` : '';
      say(`game_face.wasm: the build failed: ${first}${hint}`);
      return { why: `The Rust game did not build: ${first.slice(0, 300)}${hint}` };
    }
    say(`game_face.wasm: built in ${((performance.now() - t0) / 1000).toFixed(0)} s, ${statSync(FACE_WASM).size} bytes`);
    return { path: FACE_WASM };
  })().finally(() => { building = null; });
}

if (import.meta.main) {
  if (process.argv.includes('--check')) { const s = faceStale(); console.log(s ? `game_face.wasm: ${s}` : `game_face.wasm: up to date (${statSync(FACE_WASM).size} bytes)`); process.exit(s ? 1 : 0); }
  const r = await faceWasm(l => console.log(l));
  if ('why' in r) { console.error(r.why); process.exit(1); }
  console.log(r.path.slice(ROOT.length + 1));
}
