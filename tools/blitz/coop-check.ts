// THE RUST GAME'S CO-OP CHECK: three lines, one for each kind of two-seat fault found on the TypeScript game on 2026-11-05 and copied into rust/game with
// everything else (the Library's rebuild/rust-sim/port/blitz/coop-needs.md, needs 11 to 13; the account: rebuild/multiplayer/seat-faults-2026-11-05.md).
//   bun run tools/blitz/coop-check.ts [--only asks,lock,full] [--out DIR] [--no-build] [--keep-blobs]
// THE LINES ARE EXPECTED TO BE RED until the seats' parts of rust/game are mended: they are the target of that work, not a gate on anything today. Each line
// runs a game of two seats through run.sh (the TypeScript once, the Rust game stepped from the same start on the same two records) and then reads the RUST
// side: its log and the states it kept. The TypeScript's own answer is printed beside it. While the Rust game is in step with the TypeScript it has the
// TypeScript's faults; when a need is mended it parts from the TypeScript ON PURPOSE at that game's first difference, the loop says PARTED, and the line is
// still read off the Rust states (`--after`: the Rust game plays on from the same records).
//   asks   two-roomclear:9. Need 11: no "the player" outside a seat. The TypeScript counts every ask of the bare Game for a seat's field inside a tick
//          (seats.ts SEAT_ASKS); the Rust game has no such counter, so this line cannot see the Rust read directly. RED while the TypeScript asked and the
//          Rust game stayed in step (it then reads the first seat's at the same places); once it parts, the line says so and somebody reads the paths
//   lock   two-lock0:9 and two-lock1:9. Need 12: a seat's module keeps to its own seat's men. Each seat's pair is stacked on the locked server door and sent
//          in before tick 961; both men are across the frame line within LOCK_SECS of the order, for BOTH seats. Today seat 0's pair takes about 9.5 s:
//          seat 1's door tools sling seat 0's breaching shotgun every tick (doortools.ts slingTick; doortools/p_04.rs)
//   full   two-full:3, the barracks. Need 13: every budget of men counts the seats. The men the game holds (players, guards and teammates, civilians) are
//          no more than the picture holds (anim/characters.ts MAX_CHARS). Today 17 against 16. WHAT IT CANNOT SEE: the hand-over itself. The Rust tick hands
//          the picture nothing (its renderer is an empty boundary), and the loop stands the TypeScript with the men's row of the dress down, so nobody is
//          drawn on either side. If need 13 is met by capping what the HOST is handed and not by the count, this line belongs to the host's check
//          (play.ts with two seats and the dress up), which does not exist yet
// Exit: 0 every line run passed · 1 a line is red · 3 a game could not be run.
import { readFileSync, existsSync, mkdirSync, readdirSync, unlinkSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { decode } from './paths.ts';

const ROOT = new URL('../..', import.meta.url).pathname.replace(/\/$/, '');
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; }, has = (n: string) => argv.includes(n);
const ONLY = arg('--only', 'asks,lock,full').split(','), OUT = arg('--out', `${process.env.TMPDIR ?? '/tmp'}/blitz-coop`), BLOBS = has('--keep-blobs');
let built = has('--no-build');
/** both men through the door within this many seconds of the order (a pair with a gun on the lock takes 2 to 3.5 s; the states are read every half second) */
const LOCK_SECS = 5, LOCK_FROM = 960, LOCK_STEP = 30, LOCK_TO = 1800;
mkdirSync(OUT, { recursive: true });
let red = 0, broke = 0;
const say = (s = '') => process.stdout.write(s + '\n');
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) red++; say(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };

interface Run { dir: string; code: number; rust: string; coop: any; inStep: boolean; parted: number; panic: string; ticks: number }
/** one game through the loop. `--after`: a Rust game that parts from the TypeScript plays on to the end of the records */
function run(name: string, extra: string[]): Run | null {
  const dir = `${OUT}/${name.replace(':', '-')}`, a = ['tools/blitz/run.sh', name, '--out', dir, '--after', '100000', '--show', '0', ...(built ? ['--no-build'] : []), ...extra];
  const r = spawnSync('bash', a, { cwd: ROOT, encoding: 'utf8', maxBuffer: 1 << 28 }); built = true;
  if (!existsSync(`${dir}/rust.log`) || !existsSync(`${dir}/coop.json`)) { broke++; say(`- FAIL — ${name}: the game could not be run (run.sh exit ${r.status})\n    ${(r.stdout + r.stderr).trim().split('\n').slice(-3).join(' | ').slice(0, 400)}`); red++; return null; }
  const rust = readFileSync(`${dir}/rust.log`, 'utf8'), coop = JSON.parse(readFileSync(`${dir}/coop.json`, 'utf8'));
  const parts = [...rust.matchAll(/FIRST PARTED AT TICK (\d+)/g), ...rust.matchAll(/^TICK (\d+);[^\n]*block PARTS in/gm)].map(m => +m[1]), parted = parts.length ? Math.min(...parts) : 0, panic = rust.match(/PANIC in tick \d+.*/)?.[0] ?? '';
  return { dir, code: r.status ?? 3, rust, coop, inStep: !parted && !panic && /^DONE: /m.test(rust), parted, panic, ticks: coop.ticks };
}
/** the whole states a game kept are large (about 8 MB each) and read once: gone unless --keep-blobs */
const sweep = (dir: string) => { for (const f of readdirSync(dir)) if (/^(n|a|s|ns)\d+\.bin$/.test(f)) unlinkSync(`${dir}/${f}`); };
const step = (R: Run) => R.panic ? `THE RUST TICK PANICKED: ${R.panic.slice(0, 160)}` : R.parted ? `the Rust game's block or die PARTED from the TypeScript's at tick ${R.parted} of ${R.ticks} and it played on` : `the Rust game in step with the TypeScript for ${R.ticks} ticks`;

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

// ---- asks
if (ONLY.includes('asks')) { const R = run('two-roomclear:9', ['--asks']);
  if (R) verdict(R.coop.asks === 0 && R.inStep, 'asks: nothing in a tick of a game of two reads "the player" off the bare Game (need 11)',
    R.coop.asks === 0 ? `the TypeScript asked the bare Game for no seat's field · ${step(R)}`
      : R.inStep ? `the TypeScript asked the bare Game for a seat's field ${R.coop.asks} times, first in tick ${R.coop.firstAsk}, and ${step(R)}: it reads the first seat's at the same places (the places: run.sh two-roomclear:9 --asks; every one known, tools/qa/seats-check.ts KNOWN_ASKS)`
      : `the TypeScript asked ${R.coop.asks} times, first in tick ${R.coop.firstAsk} · ${step(R)}: NOT DECIDED HERE. If it parted where the TypeScript first asks, read the paths at that tick: that is the mend showing`); }

// ---- lock
if (ONLY.includes('lock')) { const keeps: number[] = []; for (let k = LOCK_FROM; k <= LOCK_TO; k += LOCK_STEP) keeps.push(k);
  const out: string[] = []; let ok = true, ran = 0;
  for (const seat of [0, 1]) { const R = run(`two-lock${seat}:9`, ['--keep', keeps.join(','), '--blobs']); if (!R) { ok = false; continue; } ran++;
    const D = R.coop.door, side = (p: number[]) => Math.sign((p[0] - D.hinge[0]) * Math.sin(D.closedDir) - (p[2] - D.hinge[1]) * Math.cos(D.closedDir));
    const refused = (R.coop.gave as any[]).filter(x => x.why).map(x => `${x.line.split(' ')[1]} REFUSED: ${x.why}`), rustGave = (R.rust.match(/the driver's order before tick \d+: .*/g) ?? []).filter(l => !/-> None$/.test(l));
    // the Rust side, off the states it kept: the first kept tick each man of the seat's pair stands across the frame line from where he stood at the order
    const at0 = read(`${R.dir}/n${LOCK_FROM}.bin`), crossed = new Map<string, number>(); let read1 = 0;
    if (at0) { const pair = [...at0.men].filter(([, m]) => m.seat === seat), from = new Map(pair.map(([n, m]) => [n, side(m.pos)]));
      for (const k of keeps.slice(1)) { if (crossed.size === pair.length) break; const S = read(`${R.dir}/n${k}.bin`); if (!S) continue; read1++;
        for (const [n] of pair) { const m = S.men.get(n); if (m && !crossed.has(n) && side(m.pos) !== 0 && side(m.pos) !== from.get(n)) crossed.set(n, k); } }
      const secs = (k: number | undefined) => k === undefined ? 'never' : `${((k - LOCK_FROM) / 60).toFixed(1)} s`, ts = Object.entries(R.coop.crossed as Record<string, number>).map(([n, k]) => `${n} ${((k - LOCK_FROM) / 60).toFixed(2)} s`).join(', ') || 'nobody crossed';
      const through = pair.length === 2 && pair.every(([n]) => crossed.has(n) && (crossed.get(n)! - LOCK_FROM) / 60 <= LOCK_SECS);
      if (!through || refused.length || rustGave.length) ok = false;
      out.push(`seat ${seat}'s pair, RUST: ${pair.map(([n]) => `${n} ${secs(crossed.get(n))}`).join(', ') || 'no pair found in its state'}${through ? '' : ' (OVER)'} · TypeScript: ${ts} · ${step(R)}${refused.length ? ` · ${refused.join(' · ')}` : ''}${rustGave.length ? ` · THE RUST ORDER ANSWERED: ${rustGave.join(' | ').slice(0, 200)}` : ''}`); }
    else { ok = false; out.push(`seat ${seat}'s pair: the Rust side kept no state at tick ${LOCK_FROM} · ${step(R)}`); }
    if (!BLOBS) sweep(R.dir); }
  if (ran) verdict(ok, `lock: each seat's pair, stacked on a locked door and sent in, is through within ${LOCK_SECS} s of the order (need 12)`, out.join('\n    ')); }

// ---- full
if (ONLY.includes('full')) { const R = run('two-full:3', ['--every', '3600', '--blobs']);
  if (R) { const last = [7200, 3600].map(k => ({ k, S: read(`${R.dir}/n${k}.bin`) })).find(x => x.S), holds = R.coop.picture.holds as number;
    verdict(!!last && last.S!.holds <= holds, `full: a game of two holds no more men than the picture holds (need 13)`,
      `${last ? `RUST, at tick ${last.k}: ${last.S!.holds} men in a game of ${last.S!.seats} seats on the barracks` : 'the Rust side kept no state'} · TypeScript: ${R.coop.men} as stood up · the picture holds ${holds} · ${step(R)} · the hand-over itself is not seen here (the header says why)`);
    if (!BLOBS) sweep(R.dir); } }

say(red ? `=== coop-check: ${red} RED${broke ? ` (${broke} could not be run)` : ''} · expected until needs 11 to 13 are mended in rust/game ===` : '=== coop-check: ALL PASS ===');
process.exit(broke ? 3 : red ? 1 : 0);
