// THE RUST GAME'S CO-OP CHECK: a game of two players is held to the kinds of fault a game of two had on the TypeScript game (found 2026-11-05, copied into
// rust/game with everything else and mended there on 2026-11-07; the Library's rebuild/rust-sim/port/blitz/coop-needs.md, needs 12 and 13; the account:
// rebuild/multiplayer/seat-faults-2026-11-05.md).
//   bun run tools/blitz/coop-check.ts [--only held,lock,full] [--out DIR] [--no-build] [--keep-blobs]
// NO KEPT CO-OP RUN IS IN THE TREE: the three this was written for parted at tick 1 by design and are deleted (their guard is the native tests), so with none
// here the check says so and exits 0. It stays for new ones, under the names below.
// NO TYPESCRIPT TICK IS RUN. The games are KEPT RUNS (tools/blitz/kept/two-*.tar.zst): the records of every tick and the orders given. hold.sh stands each start again from this tree
// and the native runner steps the Rust game through the records; this file then reads the whole states the runner keeps.
//   held   each of the three plays the series kept with it (the die's count and the block after every tick): the Rust game still plays those two-seat
//          games as it did when they were read. A game that parts is named with its first tick; the lines below are still read off its states
//   lock   two-lock0-9 and two-lock1-9. Need 12: a seat's module keeps to its own seat's men. Each seat's pair is stacked on the locked server door and
//          sent in before tick 961; both men are across the frame line within LOCK_SECS of the order, for BOTH seats. (On the TypeScript game seat 0's pair
//          took about 9.5 s: seat 1's door tools slung seat 0's breaching shotgun every tick.)
//   full   two-full-3, the barracks. Need 13: every budget of men counts the seats. The men the game holds (players, guards and teammates, civilians) are
//          no more than the picture holds (the kept run's coop.json: anim/characters.ts MAX_CHARS as the run was made), at ticks 1, 3600 and 7200. WHAT IT
//          CANNOT SEE: the hand-over to the picture itself (the runner draws nobody)
// NOT HERE ANY MORE: `asks` (need 11: no "the player" outside a seat). Its count was the TypeScript tick's own (seats.ts SEAT_ASKS) and went with it.
// Exit: 0 every line run passed · 1 a line is red · 3 a game could not be run.
import { readFileSync, existsSync, mkdirSync, readdirSync, unlinkSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { decode } from './paths.ts';

const ROOT = new URL('../..', import.meta.url).pathname.replace(/\/$/, '');
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; }, has = (n: string) => argv.includes(n);
const ONLY = arg('--only', 'held,lock,full').split(','), OUT = arg('--out', `${process.env.TMPDIR ?? '/tmp'}/blitz-coop`), BLOBS = has('--keep-blobs');
let built = has('--no-build');
/** both men through the door within this many seconds of the order (a pair with a gun on the lock takes 2 to 3.5 s; the states are read every half second) */
const LOCK_SECS = 5, LOCK_FROM = 960, LOCK_STEP = 30, LOCK_TO = 1800;
mkdirSync(OUT, { recursive: true });
let red = 0, broke = 0;
const say = (s = '') => process.stdout.write(s + '\n');
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) red++; say(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };

interface Run { dir: string; held: string; same: boolean; rust: string; coop: any; ticks: number }
const RUNNER = `${ROOT}/rust/game_run/target/debug/mission`, runs = new Map<string, Run | null>();
/** one kept game: its start made again and its series held (hold.sh), then the Rust game stepped again through its records with the states `keeps` kept */
function run(name: string, keeps: number[]): Run | null {
  if (runs.has(name)) return runs.get(name)!;
  const dir = `${OUT}/${name}`, kept = `${ROOT}/tools/blitz/kept/${name}.tar.zst`;
  const h = spawnSync('bash', ['tools/blitz/hold.sh', kept, ...(built ? ['--no-build'] : [])], { cwd: ROOT, encoding: 'utf8', maxBuffer: 1 << 28, env: { ...process.env, HOLD_DIRS: OUT } }); built = true;
  const held = (h.stdout + h.stderr).trim().split('\n').find(l => l.startsWith(`${name}:`)) ?? `${name}: hold.sh said nothing (exit ${h.status})`;
  if (!existsSync(`${dir}/coop.json`) || !existsSync(`${dir}/ticks.txt`) || !existsSync(RUNNER)) { broke++; red++; say(`- FAIL — ${name}: the kept game could not be run\n    ${held.slice(0, 400)}`); runs.set(name, null); return null; }
  const ticks = +readFileSync(`${dir}/ticks.txt`, 'utf8'), r = spawnSync(RUNNER, [dir, String(ticks), '--keep', keeps.join(',')], { cwd: ROOT, encoding: 'utf8', maxBuffer: 1 << 28 });
  const R: Run = { dir, held, same: h.status === 0, rust: r.stdout + r.stderr, coop: JSON.parse(readFileSync(`${dir}/coop.json`, 'utf8')), ticks };
  runs.set(name, R); return R;
}
/** the whole states a game kept are large (about 8 MB each) and read once: gone unless --keep-blobs */
const sweep = (dir: string) => { for (const f of readdirSync(dir)) if (/^(n|a|s|ns)\d+\.bin$/.test(f)) unlinkSync(`${dir}/${f}`); };
const LOCK_KEEPS: number[] = []; for (let k = LOCK_FROM; k <= LOCK_TO; k += LOCK_STEP) LOCK_KEEPS.push(k);
const GAMES: [string, number[]][] = [['two-lock0-9', LOCK_KEEPS], ['two-lock1-9', LOCK_KEEPS], ['two-full-3', [1, 3600, 7200]]];
if (GAMES.every(([name]) => !existsSync(`${ROOT}/tools/blitz/kept/${name}.tar.zst`))) { say(`coop-check: no kept co-op run is in the tree (tools/blitz/kept/two-*.tar.zst): nothing to check.`); process.exit(0); }

/** a state of either side, the few things this check reads: each man by his callsign (his seat, where he stands), and how many the game holds */
function read(file: string): { men: Map<string, { seat: number | null; pos: number[] }>; holds: number; seats: number } | null {
  if (!existsSync(file)) return null;
  const gd = new Map<number, { callsign?: string; seat?: number | null; ch?: number }>(), ch = new Map<number, number[]>(); let seats = 0, guards = 0, civ = 0;
  for (const r of decode(new Uint8Array(readFileSync(file)))) { const p = r.path; if (p.charCodeAt(0) !== 103 || p.charCodeAt(1) !== 46) continue;
    let m: RegExpExecArray | null;
    if (p.startsWith('g.gd[')) { if ((m = /^g\.gd\[(\d+)\]\.(callsign|ch|unit\.seat)$/.exec(p))) { const o = gd.get(+m[1]) ?? {}; gd.set(+m[1], o);
      if (m[2] === 'callsign') o.callsign = JSON.parse(r.bits); else if (m[2] === 'ch') o.ch = +(/\((\d+)\)/.exec(r.bits)?.[1] ?? -1); else o.seat = r.bits === 'None' ? null : parseFloat(r.bits); } }
    else if (p.startsWith('g.ch[')) { if ((m = /^g\.ch\[(\d+)\]\.pos\[(\d)\]$/.exec(p))) { const o = ch.get(+m[1]) ?? [0, 0, 0]; ch.set(+m[1], o); o[+m[2]] = parseFloat(r.bits); } }
    else if (p === 'g.seats.len') seats = +r.bits; else if (p === 'g.guards.len') guards = +r.bits; else if (p === 'g.civilians.len') civ = +r.bits; }
  const men = new Map<string, { seat: number | null; pos: number[] }>();
  for (const o of gd.values()) if (o.callsign && o.ch !== undefined && ch.has(o.ch)) men.set(o.callsign, { seat: o.seat ?? null, pos: ch.get(o.ch)! });
  return { men, holds: seats + guards + civ, seats };
}

// ---- held
if (ONLY.includes('held')) { const L: string[] = []; let ok = true;
  for (const [name, keeps] of GAMES) { const R = run(name, keeps); if (!R) { ok = false; continue; } if (!R.same) ok = false; L.push(R.held.slice(0, 300)); }
  verdict(ok, 'held: each kept game of two players plays the series kept with it', L.join('\n    ') || 'no kept game ran'); }

// ---- lock
if (ONLY.includes('lock')) { const out: string[] = []; let ok = true, ran = 0;
  for (const seat of [0, 1]) { const R = run(`two-lock${seat}-9`, LOCK_KEEPS); if (!R) { ok = false; continue; } ran++;
    const D = R.coop.door, side = (p: number[]) => Math.sign((p[0] - D.hinge[0]) * Math.sin(D.closedDir) - (p[2] - D.hinge[1]) * Math.cos(D.closedDir));
    const refused = (R.coop.gave as any[]).filter(x => x.why).map(x => `${x.line.split(' ')[1]} REFUSED: ${x.why}`), rustGave = (R.rust.match(/the driver's order before tick \d+: .*/g) ?? []).filter(l => !/-> None$/.test(l));
    // off the states the runner kept: the first kept tick each man of the seat's pair stands across the frame line from where he stood at the order
    const at0 = read(`${R.dir}/n${LOCK_FROM}.bin`), crossed = new Map<string, number>();
    if (at0) { const pair = [...at0.men].filter(([, m]) => m.seat === seat), from = new Map(pair.map(([n, m]) => [n, side(m.pos)]));
      for (const k of LOCK_KEEPS.slice(1)) { if (crossed.size === pair.length) break; const S = read(`${R.dir}/n${k}.bin`); if (!S) continue;
        for (const [n] of pair) { const m = S.men.get(n); if (m && !crossed.has(n) && side(m.pos) !== 0 && side(m.pos) !== from.get(n)) crossed.set(n, k); } }
      const secs = (k: number | undefined) => k === undefined ? 'never' : `${((k - LOCK_FROM) / 60).toFixed(1)} s`, ts = Object.entries(R.coop.crossed as Record<string, number>).map(([n, k]) => `${n} ${((k - LOCK_FROM) / 60).toFixed(2)} s`).join(', ') || 'nobody crossed';
      const through = pair.length === 2 && pair.every(([n]) => crossed.has(n) && (crossed.get(n)! - LOCK_FROM) / 60 <= LOCK_SECS);
      if (!through || refused.length || rustGave.length) ok = false;
      out.push(`seat ${seat}'s pair: ${pair.map(([n]) => `${n} ${secs(crossed.get(n))}`).join(', ') || 'no pair found in its state'}${through ? '' : ' (OVER)'} · the TypeScript game, as the run was kept: ${ts}${refused.length ? ` · ${refused.join(' · ')}` : ''}${rustGave.length ? ` · THE ORDER ANSWERED: ${rustGave.join(' | ').slice(0, 200)}` : ''}`); }
    else { ok = false; out.push(`seat ${seat}'s pair: the runner kept no state at tick ${LOCK_FROM}: ${R.rust.trim().split('\n').slice(-2).join(' | ').slice(0, 300)}`); } }
  if (ran) verdict(ok, `lock: each seat's pair, stacked on a locked door and sent in, is through within ${LOCK_SECS} s of the order (need 12)`, out.join('\n    ')); }

// ---- full
if (ONLY.includes('full')) { const R = run('two-full-3', [1, 3600, 7200]);
  if (R) { const at = [1, 3600, 7200].map(k => ({ k, S: read(`${R.dir}/n${k}.bin`) })).filter(x => x.S), holds = R.coop.picture.holds as number;
    verdict(at.length === 3 && at.every(x => x.S!.holds <= holds), `full: a game of two holds no more men than the picture holds (need 13)`,
      `${at.length ? at.map(x => `tick ${x.k}: ${x.S!.holds} men in a game of ${x.S!.seats} seats`).join(' · ') : 'the runner kept no state'} on the barracks · the TypeScript game stood ${R.coop.men} · the picture holds ${holds} · the hand-over itself is not seen here (the header says why)`); } }

if (!BLOBS) for (const R of runs.values()) if (R) sweep(R.dir);
say(red ? `=== coop-check: ${red} RED${broke ? ` (${broke} could not be run)` : ''} ===` : '=== coop-check: ALL PASS ===');
process.exit(broke ? 3 : red ? 1 : 0);
