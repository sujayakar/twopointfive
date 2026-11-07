// THE MODULE'S OWN START HELD TO THE NATIVE START (rust/game_start/README.md, "The module's start"). The page's module (rust/game_face) can start a
// game itself, by the game's own start (`face_start`: rust/game_start `stand` on the level file's text, the address's words and the character's
// files, handed in as bytes). This holds the game the MODULE starts to the game the same start makes natively: rust/sim's block BYTE FOR BYTE,
// and the whole state as the bridge writes it PATH FOR PATH, with one thing forgiven by name (tools/blitz/known-module.txt): where the box
// mirror's tables stand in memory, which a module and a native process cannot share. (That the native start is the TypeScript start, path for path, is the levels table's to hold:
// tools/blitz/start-levels.ts, rust/game_start/LEVELS.md.)
//   bun run tools/blitz/start-module.ts [--only id,id] [--seats 2] [--times N] [--wasm FILE] [--keep DIR] [--no-build]
// For each mission of the list (scene/scenarioList.ts ALL_MISSIONS, read through `start.ts --missions`: its file in the look that is on, its link):
//   1. the native start: rust/game_start's bin stands the file from the link and writes n1.bin (the state) and ns0.bin (the block);
//   2. the module is handed the same bytes and starts the same mission, `--times` times over (default 3): the first in a module that has stood
//      nothing of this mission, the second in the slot the first gave up, the third beside the second in another slot;
//   3. each of the module's games is read back (`face_export`, the slot's block) and compared with the native start's two files.
// A mission the game refuses (for two players: "it takes one player today") is refused by the native start first and is a row that says so.
// Exit 0 every started mission equal · 1 one is not · 3 a side could not run · 64 a wrong word.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { packStart } from './start-pack.ts';
const ROOT = resolve(import.meta.dir, '../..');
const say = (s = '') => process.stdout.write(s + '\n');
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const SEATS = +arg('--seats', '1'), TIMES = Math.max(1, +arg('--times', '3')), ONLY = arg('--only', '').split(',').filter(Boolean), SEED = 20942;
const WASM = resolve(arg('--wasm', resolve(ROOT, 'rust/game_face/target/wasm32-unknown-unknown/play/game_face.wasm')));
const KEEP = resolve(arg('--keep', resolve(ROOT, 'rust/game_start/target/start-module')), `seats${SEATS}`);
if (SEATS !== 1 && SEATS !== 2) { say('start-module: --seats 1 or 2'); process.exit(64); }
const ENV = { ...process.env, BUN_OPTIONS: '' };
const run = (cmd: string, args: string[], cwd = ROOT) => { const r = spawnSync(cmd, args, { cwd, encoding: 'utf8', maxBuffer: 1 << 26, env: ENV }); return { out: (r.stdout ?? '') + (r.stderr ?? ''), code: r.status ?? -1 }; };

if (!argv.includes('--no-build')) { const b = run('cargo', ['build', '--bin', 'start'], resolve(ROOT, 'rust/game_start')); if (b.code !== 0) { say(b.out.split('\n').filter(l => /^error/.test(l)).slice(0, 20).join('\n')); say('start-module: the native start does not build'); process.exit(3); } }
if (!existsSync(WASM)) { say(`start-module: no module at ${WASM} (bun run tools/blitz/face-build.ts --profile play)`); process.exit(3); }
const listed = run('bun', ['run', '--preload', './tools/blitz/expose.ts', 'tools/blitz/start.ts', '--missions']);
const missions = listed.out.split('\n').map(l => l.split('\t')).filter(c => c.length === 4).map(([id, file, link]) => ({ id, file, link }));
if (!missions.length) { say(listed.out.slice(-2000)); say('start-module: start.ts --missions listed nothing'); process.exit(3); }
{ const stray = ONLY.filter(id => !missions.some(m => m.id === id)); if (stray.length) { say(`start-module: no mission ${stray.join(', ')}`); process.exit(64); } }

// the module, once for the whole run: a game started in it stands in a module that has started every mission before it
const X: any = new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(WASM))), { env: { face_roll: () => { throw new Error('start-module: the start asked the host for a roll'); } } }).exports;
const u8 = () => new Uint8Array(X.memory.buffer);
const out = () => { const p = X.face_out_ptr() >>> 0; return u8().slice(p, p + (X.face_out_len() >>> 0)); };
const panicText = () => { try { const p = X.face_panic_ptr() >>> 0; return new TextDecoder().decode(u8().subarray(p, p + X.face_panic_len())); } catch { return ''; } };
if (typeof X.face_start !== 'function') { say(`start-module: the module at ${WASM} has no face_start: it is of older sources`); process.exit(3); }
/** a game started in the module: its handle, or the reason it was not */
function started(handed: Uint8Array, slot: number): { h: number; why: string } {
  const p = X.face_alloc(handed.length) >>> 0; u8().set(handed, p);
  try { X.face_panic_clear?.(); const h = X.face_start(p, handed.length, slot, SEED, SEATS, 0) >>> 0; return { h, why: h ? '' : 'no slot was free' }; }
  catch (e) { return { h: 0, why: panicText() || String((e as Error)?.message ?? e) }; }
  finally { try { X.face_free(p, handed.length); } catch { /* (a module stopped by a panic frees nothing) */ } }
}
const blockOf = (slot: number) => { const at = X.state_addr(slot) >>> 0; return u8().slice(at, at + (X.state_len() >>> 0)); };
const firstOff = (a: Uint8Array, b: Uint8Array) => { const n = Math.min(a.length, b.length); for (let i = 0; i < n; i++) if (a[i] !== b[i]) return i; return a.length === b.length ? -1 : n; };

const todo = missions.filter(m => !ONLY.length || ONLY.includes(m.id));
let equal = 0, refused = 0, red = 0;
for (const m of todo) {
  const dir = resolve(KEEP, m.id); mkdirSync(dir, { recursive: true });
  const file = resolve(ROOT, 'levels', m.file);
  const n = run(resolve(ROOT, 'rust/game_start/target/debug/start'), [file, '--out', dir, '--seed', String(SEED), '--link', m.link, ...(SEATS > 1 ? ['--seats', String(SEATS)] : [])]);
  if (n.code !== 0) {
    const why = /the game refused the start[^\n]*/.exec(n.out)?.[0];
    if (why) { refused++; say(`${m.id}\t${m.file}\trefused by the game\t${why.replace(/^the game refused the start from '[^']*': /, '').slice(0, 120)}`); continue; }
    red++; say(`${m.id}\t${m.file}\tRED\tthe native start stopped: ${n.out.trim().split('\n').slice(-3).join(' · ').slice(0, 300)}`); continue;
  }
  const state = new Uint8Array(readFileSync(resolve(dir, 'n1.bin'))), block = new Uint8Array(readFileSync(resolve(dir, 'ns0.bin')));
  const handed = packStart(ROOT, readFileSync(file), m.link);
  const said: string[] = [], live: number[] = []; let bad = false;
  for (let t = 0; t < TIMES && !bad; t++) {
    const g = started(handed, -1);
    if (!g.h) { bad = true; said.push(`start ${t + 1} stopped: ${g.why.slice(0, 300)}`); break; }
    const slot = X.face_slot(g.h) >>> 0; X.face_export(g.h); const mine = out(), mineBlock = blockOf(slot);
    // THE BLOCK byte for byte; THE STATE path for path (diff.ts), because the state holds where the box mirror's tables stand in memory, and a
    // module's memory is not a native process's: known-module.txt forgives those addresses and nothing else
    const b = firstOff(block, mineBlock); let a = 'equal';
    if (firstOff(state, mine) >= 0) {
      writeFileSync(resolve(dir, `m${t + 1}.bin`), mine); writeFileSync(resolve(dir, `ms${t + 1}.bin`), mineBlock);
      const d = run('bun', ['run', 'tools/blitz/diff.ts', resolve(dir, 'n1.bin'), resolve(dir, `m${t + 1}.bin`), '--out', resolve(dir, `diff${t + 1}.tsv`), '--canon-file', 'tools/blitz/canon.txt', '--known', 'tools/blitz/known-module.txt', '--show', '6']);
      const m2 = /(\d+) paths differ, (\d+) known, (\d+) NOT KNOWN/.exec(d.out);
      a = !m2 ? `the path diff did not run: ${d.out.trim().split('\n').slice(-2).join(' · ').slice(0, 200)}` : m2[3] === '0' ? `equal but for ${m2[1]} addresses of the box mirror` : `${m2[3]} paths differ: ${d.out.split('\n').slice(1, 5).map(l => l.trim().split('\t').slice(0, 2).join(' ')).join(' · ').slice(0, 300)}`;
    }
    if (!a.startsWith('equal') || b >= 0) { bad = true; said.push(`start ${t + 1} (slot ${slot}) is another game: the state: ${a}; the block: ${b >= 0 ? `differs from byte ${b}` : 'equal'}. Kept in ${dir}`); }
    else said.push(`slot ${slot}: ${a === 'equal' ? 'the state byte for byte' : a}`);
    // (the first gives its slot up, so the second stands where a game stood; the second keeps its, so the third stands beside it in another)
    if (t === 1) live.push(g.h); else X.face_drop(g.h);
  }
  for (const h of live) X.face_drop(h);
  if (bad) { red++; say(`${m.id}\t${m.file}\tRED\t${said.join(' · ')}`); if (/stopped/.test(said.join(' '))) { say('start-module: a start that stopped by a panic leaves the module unfit for another: stopping here'); break; } }
  else { equal++; say(`${m.id}\t${m.file}\tEQUAL\t${TIMES} starts in the module, each the native start's game (${state.length} bytes of state, ${block.length} of block; ${said.join(', ')})`); }
}
say(); say(`${equal} of ${todo.length - refused} missions the module starts are the game the native start makes (the block byte for byte, the state path for path) (${TIMES} starts each, die seed ${SEED}${SEATS > 1 ? `, ${SEATS} seats` : ''})${refused ? `; ${refused} the game refuses` : ''}${red ? `; ${red} RED` : ''}.`);
process.exit(red ? 1 : 0);
