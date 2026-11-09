// THE CHECKS THAT STAND NO GAME OF THEIR OWN MAKING: the picture's code, the shaders' texts, the page and its cards, the sounds, the level files, the player's words.
// Where a mission's row is asked (bench, pricerun, mystery, writing), the game's module is: one built of these sources, or the checked-in one where it is of them (tools/desk.ts).
// Each is a file run in a process of its own. The game's rules are asked in Rust (`gate tests`), whole plays by the native `gate`, missions and `hold`, the page by pages.ts.
//   bun run tools/qa/checks.ts [group | name …] [--list]     no word: every group    groups: picture · shaders · page · menus · sound · levels · words · build
//   env: CHECK_JOBS (four) at a time · CHECK_CAP (ten) minutes each, its process group stopped there
// Exit 0 every one green · 1 one red (its exit, a FAIL line, or the cap), or a *-check.ts here or in tools/draw that no row names · 2 an unknown name
import { existsSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';

type Group = 'picture' | 'shaders' | 'page' | 'menus' | 'sound' | 'levels' | 'words' | 'build';
type Row = readonly [file: string, group: Group, ...args: string[]];
const ROWS: readonly Row[] = [
  ['bouncewalls-check.ts', 'picture'], ['cast-probes.ts', 'picture'], ['charmats-probes.ts', 'picture'], ['floor-order-check.ts', 'picture'], ['frame-layout.ts', 'picture'],
  ['highlights-check.ts', 'picture'], ['huesplit-check.ts', 'picture'], ['kept-check.ts', 'picture'], ['lookcells-check.ts', 'picture'],
  ['lookexpo-check.ts', 'picture'], ['lookgrade-check.ts', 'picture'], ['lookmaps-check.ts', 'picture'], ['lookunits-check.ts', 'picture'], ['lookveil-check.ts', 'picture'], ['lookworkers-check.ts', 'picture'],
  ['materials-probes.ts', 'picture'], ['passtimer-check.ts', 'picture'], ['perf-probes.ts', 'picture'], ['portal-check.ts', 'picture'], ['quality-check.ts', 'picture'], ['rainscale-check.ts', 'picture'],
  ['rainwalls-check.ts', 'picture'], ['reflup-check.ts', 'picture'], ['rolled-check.ts', 'picture'], ['sdfrig-probes.ts', 'picture'], ['smokeline-check.ts', 'picture'],
  ['smokelook-check.ts', 'picture'], ['splitrays-check.ts', 'picture'], ['steadybounce-check.ts', 'picture'], ['sunglint-check.ts', 'picture'],
  ['sweep-ir-check.ts', 'picture'],
  ['shaders-check.ts', 'shaders', 'parse'], ['shaders-check.ts', 'shaders', 'trim'], ['shaders-check.ts', 'shaders', 'fold'], ['shaders-check.ts', 'shaders', 'inline'], ['shaders-check.ts', 'shaders', 'levelset'], ['shaders-check.ts', 'shaders', 'rows'],
  ['bench-check.ts', 'page'], ['framecap-check.ts', 'page'], ['gpulost-check.ts', 'page'], ['keeppage-check.ts', 'page'], ['nogpu-check.ts', 'page'], ['pace-check.ts', 'page'], ['perfrec-check.ts', 'page'], ['pipequeue-check.ts', 'page'],
  ['pricerun-check.ts', 'page'], ['savefile-check.ts', 'page'], ['settle-check.ts', 'page'], ['view-check.ts', 'page'], ['perf-sweep-logic.ts', 'page'], ['perf-sweep-selftest.ts', 'page'],
  ['camspace-check.ts', 'menus'], ['hud-look-check.ts', 'menus'], ['menustills-check.ts', 'menus'], ['squadkeys-check.ts', 'menus'], ['textsize-check.ts', 'menus'],
  ['gunfire-table-check.ts', 'sound'], ['sfxwarm-check.ts', 'sound'], ['sound-takes-check.ts', 'sound'],
  ['ground-pieces-check.ts', 'levels'], ['kit-stores-check.ts', 'levels'], ['palette-check.ts', 'levels'], ['stuff-check.ts', 'levels'],
  ['mystery-check.ts', 'words'], ['writing-check.ts', 'words'],
  ['docs-check.ts', 'build'], ['rust-rules-check.ts', 'build'], ['../draw/gen.ts', 'build', '--check'],
];
/** check files no row runs, each with who runs it */
const BY_HAND: Readonly<Record<string, string>> = {
  'cloud-check.ts': 'the backend\'s packages', 'lobby-check.ts': 'the backend\'s packages', 'mp-control-check.ts': 'the backend\'s packages', 'mpserver-check.ts': 'three processes, minutes',
  'zfight-check.ts': 'the level skill',
  '../draw/hands-check.ts': 'the module', '../draw/keep-check.ts': 'the module', '../draw/net-check.ts': 'the module', '../draw/play-check.ts': 'the module, minutes', '../draw/tape-check.ts': 'the module, minutes', '../draw/walk-check.ts': 'the module',
};

const HERE = import.meta.dir, argv = process.argv.slice(2), words = argv.filter(a => !a.startsWith('--')), name = (r: Row): string => r[0].replace(/(-check|-probes)?\.ts$/, '') + (r[2] && !r[2].startsWith('--') ? `-${r[2]}` : '');   // (a file of sections has a row for each: shaders-fold)
const picked = words.length ? ROWS.filter(r => words.includes(r[1]) || words.includes(name(r)) || words.includes(r[0])) : ROWS;
{ const strange = words.filter(w => !ROWS.some(r => r[1] === w || name(r) === w || r[0] === w)); if (strange.length) { console.error(`checks: no check or group ${strange.join(', ')}`); process.exit(2); } }
if (argv.includes('--list')) { for (const r of picked) console.log(`${r[1].padEnd(8)} ${name(r)}`); process.exit(0); }

const here = ['', '../draw/'].flatMap(d => readdirSync(resolve(HERE, d)).filter(f => f.endsWith('-check.ts')).map(f => d + f)), lost = here.filter(f => !ROWS.some(r => r[0] === f) && !(f in BY_HAND)), dead = [...ROWS.map(r => r[0]), ...Object.keys(BY_HAND)].filter(f => !existsSync(resolve(HERE, f)));
const red: string[] = []; if (lost.length || dead.length) { red.push('the table'); console.log(`- FAIL — the table: run by nothing ${lost.join(' ') || 'none'} · named and gone ${dead.join(' ') || 'none'}`); }
const JOBS = Number(process.env.CHECK_JOBS ?? 4), CAP = Number(process.env.CHECK_CAP ?? 10) * 60000, queue = [...picked], t0 = performance.now();
async function worker(): Promise<void> {
  for (let r = queue.shift(); r; r = queue.shift()) { const t = performance.now(), c = Bun.spawn(['bun', 'run', resolve(HERE, r[0]), ...r.slice(2)], { stdout: 'pipe', stderr: 'pipe', cwd: resolve(HERE, '../..'), detached: true });
    let capped = false; const stop = setTimeout(() => { capped = true; try { process.kill(-c.pid, 'SIGKILL'); } catch { c.kill(9); } }, CAP);
    const [out, err, code] = await Promise.all([new Response(c.stdout).text(), new Response(c.stderr).text(), c.exited]); clearTimeout(stop);
    const fails = out.split('\n').filter(l => /^\s*(- )?FAIL\b/.test(l)), bad = capped || code !== 0 || fails.length > 0; if (bad) red.push(name(r));
    console.log(`${bad ? 'RED  ' : 'green'} ${name(r).padEnd(18)} ${((performance.now() - t) / 1000).toFixed(1).padStart(6)} s${capped ? ' · stopped at the cap' : ''}`);
    if (bad) for (const l of fails.length ? fails : [...out.trim().split('\n').slice(-6), ...err.trim().split('\n').slice(-6)]) console.log(`        ${l.trim().slice(0, 240)}`); } }
await Promise.all(Array.from({ length: Math.max(1, JOBS) }, worker));
console.log(`=== checks: ${picked.length} run in ${((performance.now() - t0) / 1000).toFixed(0)} s, ${red.length ? `${red.length} RED: ${red.join(' ')}` : 'ALL PASS'} ===`);
process.exit(red.length ? 1 : 0);
