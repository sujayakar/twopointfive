// THE RUST GAME PLAYED BY THE SCRIPTED PLAYER, HELD TO THE PLAIN TYPESCRIPT PLAY EVERY TICK (PLAY.md). The host is rust-host.ts: the TypeScript `Game` is a
// PUPPET of a game that steps in the wasm face, the scripted player of tools/qa reads the puppet to decide its keys, and what it presses goes to the Rust
// game as the tick's record. The TypeScript tick (`stepSeats`' body) never runs. This file stands the mission, holds every tick to the reference and reports.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/play.ts <mission>:<seed> --ref DIR [--out DIR] [--ticks N] [--wasm FILE] [--audit] [--whole-every N]
//                                             [--stale N] [--no-warm] [--no-asks] [--handback N] [--baseline puppet|rust] [--apply-check N] [--upload-every N] [--no-dress] [--go-on] [--quiet]
// --ref DIR is the same mission and seed played by the plain TypeScript game (mission.ts wrote it: post.txt, the die's count and a hash of each part of
// rust/sim's block after every tick; h<k>.bin, the host's half before every tick). Held to it, every tick:
//   the host's half   what the scripted player pressed and where its camera stood as the tick opens: the first tick it differs is the first tick the puppet
//                     showed the player something the TypeScript game would not have
//   the die, the block  as the tick returns (12 parts; `poses` parts by design from the first death)
//   the end           how the play ended, beside the reference's last line
// THE FAST WAY is the default (rust-host.ts: the filtered apply, the driver's writes as calls). --audit: the whole state applied every tick and the puppet
// exported whole as every tick opens, to find what TypeScript writes into the game between two ticks. --whole-every N: fast, with a whole apply every N
// ticks. --stale N: fast, and every N ticks the puppet is exported whole and held to the Rust game's whole state: the fields in which the puppet has
// fallen behind, by name (a diagnosis: it costs two walks of the state each time).
// Exit: 0 equal to the end and ended the same way · 1 parted · 2 the Rust tick panicked · 3 a side could not run · 4 equal every tick, the end differs.
import '../qa/stand.ts';
import { readFileSync, writeFileSync, existsSync, mkdirSync } from 'node:fs';
import { RustHost, DEFAULT_WASM } from './rust-host.ts';
const argv = process.argv.slice(2), has = (n: string) => argv.includes(n), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
if (!argv[0] || argv[0].startsWith('--')) { say('play.ts <mission>:<seed> --ref DIR [--out DIR] [--ticks N] [--wasm FILE] [--audit] [--whole-every N] [--stale N] [--no-warm] [--no-asks] [--handback N] [--baseline puppet|rust] [--apply-check N] [--upload-every N] [--no-dress] [--go-on] [--quiet]'); process.exit(64); }
const [scn, seedS] = argv[0].split(':'), seed = +(seedS ?? 1);
const REF = arg('--ref', ''), OUT = arg('--out', REF ? `${REF}/play` : ''), TICKS = +arg('--ticks', '1000000'), DIE = +arg('--die', '20942');
const AUDIT = has('--audit'), STALE = +arg('--stale', '0'), GO_ON = has('--go-on'), QUIET = has('--quiet');
const WASM = arg('--wasm', DEFAULT_WASM);
if (!existsSync(WASM)) { say(`play: no module at ${WASM} (cd rust/game_face && cargo build --target wasm32-unknown-unknown --release)`); process.exit(3); }
if (OUT) mkdirSync(OUT, { recursive: true });
const LESSON = scn.startsWith('tut-');
if (LESSON) process.env.TUT_SQUAD_SKY = '0';   // (as mission.ts stands a tutorial: the estimate as every other tool has it)

// ---- the reference: the plain TypeScript play of the same mission and seed
const refLines = (n: string) => REF && existsSync(`${REF}/${n}`) ? readFileSync(`${REF}/${n}`, 'utf8').split('\n').filter(Boolean) : [];
const post = refLines('post.txt').map(l => l.split(' ')), head = post.shift() ?? [];
const refEnd = refLines('ts.log').find(l => l.startsWith('TypeScript:')) ?? '(no line)';
if (!post.length) say('play: NO REFERENCE (--ref DIR of a mission.ts run): the Rust game is played and NOTHING IS COMPARED');

// ---- the module, the bridge and the game
const F = await RustHost.ready(WASM), X = F.X, NAMES = F.NAMES;
if (post.length && (head[0] !== 'parts' || head.slice(1).join(' ') !== NAMES.join(' '))) { say(`play: the reference names the block's parts [${head.slice(1)}] and this module [${NAMES}]: one side is of another tree`); process.exit(3); }
const { Game } = await import('../../src/game/game.ts');
const { play, entryOf } = await import('../qa/playthrough.ts');
const { exportGame, exportPart, hostHalf, hostHalfParts, bookOf } = await import('./export.ts');
const { locate, diffOffsets, buf, fieldsDiffering, tallyAdd } = await import('./audit.ts');
type Tally = Map<string, { n: number; eg: string; ticks: number; last: number }>;
if (!LESSON && !entryOf(scn)) { say(`play: the scripted player has no mission '${scn}'`); process.exit(64); }
class Stop extends Error {}
const now = () => performance.now();

// ---- the run's books
let H: RustHost | null = null, k = 0;
const T = { driver: 0, hostTs: 0, hostHold: 0, compare: 0, stale: 0 };
let tLeft = 0, hostParted = 0, hostCompared = 0;
const S = { parted: 0, poolParted: 0, dieParted: 0, partedIn: [] as string[], compared: 0 };
const STALE_F: Tally = new Map(); let staleChecks = 0;

/** WHAT THE SCRIPTED PLAYER PRESSED, held to what it pressed in the plain TypeScript play (the reference's h<k>.bin: the bridge's own hand-over, export.ts
 *  `hostHalf`: a record a seat, every camera row, every Input row; the very bytes the Rust game is handed): the first tick the two differ is the first
 *  tick the puppet showed the player something the TypeScript game did not, whatever the block says by then */
function holdHost(g: any, reg: any, cmds: readonly any[]): void {
  if (!REF || hostParted || !existsSync(`${REF}/h${k}.bin`)) return;
  const mine = hostHalf(g, reg, cmds), ref = new Uint8Array(readFileSync(`${REF}/h${k}.bin`)); hostCompared++;
  if (ref.length === mine.length && buf(ref).equals(buf(mine))) return;
  hostParted = k; const where: string[] = [], bk: any = bookOf(g); let at = 0, rec = 0;
  // (the pieces as the hand-over itself lists them: export.ts hostHalfParts)
  const P: [string, any, string][] = hostHalfParts(g, cmds).map(([ty, v]: [string, any]) => [ty, v, ty === 'u32' ? 'a count' : ty.endsWith('PlayerCmd') ? `record[${rec++}]` : ty.endsWith('FollowCamera') ? `FollowCamera[${bk.listOf('CameraId').indexOf(v)}]` : ty.endsWith('Input') ? `Input[${bk.listOf('InputId').indexOf(v)}]` : ty.split('::').pop()!]);
  if (ref.length !== mine.length) { where.push(`the lengths differ (${mine.length} here, ${ref.length})`);
    // (where the two first differ, by the piece of this side's bytes it falls in: a list of another length, a key more in a set)
    let d = 0; while (d < mine.length && d < ref.length && mine[d] === ref[d]) d++;
    for (const [ty, v, name] of P) { const n = exportPart(g, reg, [[ty, v]]).length; if (d < at + n) { const x = ty === 'u32' ? null : locate(mine.subarray(at, at + n), [d - at], null, true, ty, name)[0]; where.push(`the first byte that differs is in ${x?.path ?? `the count of ${name}`}`); break; } at += n; } }
  // (piece by piece: each record, then each camera row, then each Input row; a piece of one length on both sides is named field by field)
  else for (const [ty, v, name] of P) { const n = exportPart(g, reg, [[ty, v]]).length, a = ref.subarray(at, at + n), b = mine.subarray(at, at + n); at += n; if (buf(a).equals(buf(b))) continue; if (ty === 'u32') { where.push(`the count of ${name}`); continue; }
    for (const x of locate(b, diffOffsets(a, b), null, true, ty, name)) { const dv = (u: Uint8Array) => new DataView(u.buffer, u.byteOffset + x.off, Math.min(8, u.length - x.off)); const f = (u: Uint8Array) => { try { return dv(u).getFloat64(0, true); } catch { return NaN; } }; where.push(`${x.path} (${f(b)} here, ${f(a)})`); } }
  // (kept: this side's half, and the Rust game's whole state as this tick opens, for diff.ts beside the TypeScript's of the same tick: mission.ts --open)
  if (OUT) { writeFileSync(`${OUT}/host${k}.bin`, mine); if (H) { X.face_export(H.h); writeFileSync(`${OUT}/rustopen${k}.bin`, F.outView()); } }
  say(`TICK ${k}: THE HOST'S HALF PARTS from the plain TypeScript play's (what the player pressed, where its camera stood): ${where.slice(0, 40).join(' · ')}`); }
/** THE DRIVER'S CALLS BETWEEN TWO TICKS (an order, a checkpoint, a flooded room, a pick of a verb), in the replay loop's words, held to the plain play's
 *  own files as each tick opens: an order the scripted player gave there and not here (or the other way) is the puppet having shown it another game, and
 *  the keys may stay the same for many ticks after */
const refCalls = new Map<number, string[]>(); for (const f of ['gives.txt', 'hostwrites.txt']) for (const l of refLines(f)) { if (/ chron_load$/.test(l)) continue; const t = +l.split(' ')[0]; refCalls.set(t, [...(refCalls.get(t) ?? []), l]); }
let callsParted = 0; const myCalls = new Map<number, string[]>(), seenN = [0, 0];
function holdCalls(Hh: RustHost): void {
  if (!post.length || callsParted || k < 2) return;
  [Hh.gave, Hh.wrote].forEach((L, i) => { for (; seenN[i] < L.length; seenN[i]++) { const l = L[seenN[i]], t = +l.split(' ')[0]; myCalls.set(t, [...(myCalls.get(t) ?? []), l]); } });
  // (as tick k opens, every line of tick k - 1 is written on both sides: a checkpoint's is written as its tick ends)
  const mine = myCalls.get(k - 1) ?? [], theirs = refCalls.get(k - 1) ?? [];
  if ([...mine].sort().join('\n') !== [...theirs].sort().join('\n')) { callsParted = k - 1; say(`TICK ${k - 1}: THE DRIVER'S CALLS PART from the plain TypeScript play's: here [${mine.join(' | ') || 'none'}], there [${theirs.join(' | ') || 'none'}]`); } }
/** held to the plain TypeScript play, where its hashes were taken: as the tick returns, before the host's steps */
function holdBlock(Hh: RustHost): void {
  const p = post[k - 1];
  if (p) { const got = Hh.hashes(), die = Hh.dieN(); S.compared++;
    let off = NAMES.filter((_, i) => p[4 + i] !== String(got[i])); const poolOff = off.includes('poses'); off = off.filter(n => n !== 'poses');
    if (!off.length && poolOff && !S.poolParted) { S.poolParted = k; if (!QUIET) say(`TICK ${k}: the part \`poses\` parts and every other part is the same (a dead man's pose is in the pool on the Rust side: README.md)`); }
    if (p[1] !== String(die) && !S.dieParted) { S.dieParted = k; say(`TICK ${k}: THE DIE'S COUNT PARTS: ${die} here, ${p[1]} in the TypeScript play`); }
    if (off.length && !S.parted) { S.parted = k; S.partedIn = off; say(`TICK ${k}: THE BLOCK PARTS in [${off}]; time ${Hh.time().toFixed(4)}; die n=${die} (TypeScript ${p[1]})`); }
  } else if (post.length && !S.parted && !tLeft) { tLeft = k; say(`TICK ${k}: the TypeScript play ended at tick ${post.length}: this play goes on past it`); } }
/** --stale: the puppet exported whole and held to the Rust game's whole state, field by field (what the fast way has let fall behind) */
function holdStale(Hh: RustHost, g: any): void {
  const t = now(); staleChecks++; X.face_export(Hh.h); const rust = F.outView().slice(), mine = exportGame(g, Hh.B.reg), cut: string[] = [];
  for (const x of fieldsDiffering(rust, mine, cut, 20000)) if (!Hh.B.SCHEMA.serial?.[x.q] && x.q !== 'tail') tallyAdd(STALE_F, x.q, x.path, k);
  T.stale += now() - t; }

// ---- `Game.update` on the puppet: its own lines round the step run; the step is the host's
const was = Game.prototype.update; let tOut = 0, wall0 = 0, G0: any = null, stopped = false;
Game.prototype.update = function (this: any, ...a: any[]) {
  const g = this; G0 = g; k++; const t0 = now(); if (k > 1) T.driver += t0 - tOut; else wall0 = t0;
  if (!H) H = RustHost.attach(g, { wasm: WASM, audit: AUDIT, wholeEvery: +arg('--whole-every', '0'), uploadEvery: +arg('--upload-every', '4'), dress: !has('--no-dress'), die: DIE, warm: has('--no-warm') ? 0 : undefined,
    asks: !has('--no-asks'), handback: +arg('--handback', '0'), baseline: arg('--baseline', 'rust') as 'rust' | 'puppet', applyCheck: +arg('--apply-check', '50'), out: OUT || undefined, quiet: QUIET, say,
    beforeHost: (Hh, cmds) => { const t = now(); holdCalls(Hh); holdHost(g, Hh.B.reg, cmds); T.hostHold += now() - t; },
    afterStep: Hh => { const t = now(); holdBlock(Hh); T.compare += now() - t; } });
  const Hh = H, s0 = Hh.spent(), c0 = T.hostHold + T.compare;
  let err: unknown = null;
  try { (was as any).apply(this, a); } catch (e) { err = e; }
  const inHost = Hh.spent() - s0 + (T.hostHold + T.compare - c0);
  T.hostTs += now() - t0 - inHost;
  if (err) { const words = F.panicText(); if (words) { say(`PANIC in tick ${k}: ${words}`); finish(g, 2); } say(`TICK ${k} THREW on the puppet: ${(err as any)?.stack ?? err}`); finish(g, 3); }
  if (STALE > 0 && !AUDIT && k % STALE === 0 && k >= +arg('--stale-from', '0')) holdStale(Hh, g);
  if (S.parted === k && OUT) { if (Hh.openState) writeFileSync(`${OUT}/open${k}.bin`, Hh.openState); X.face_export(Hh.h); writeFileSync(`${OUT}/rust${k}.bin`, F.outView()); writeFileSync(`${OUT}/block${k}.bin`, Hh.rustBlock()); }
  if ((!QUIET || has('--progress')) && (k % 500 === 0)) say(`tick ${k}; time ${Hh.time().toFixed(3)}; die n=${Hh.dieN()}; ${S.parted || S.dieParted || hostParted ? `PARTED (block ${S.parted}, die ${S.dieParted}, host ${hostParted}, calls ${callsParted})` : 'equal so far'}; ${((now() - wall0) / k).toFixed(2)} ms a tick; memory ${(F.mem.buffer.byteLength / 1048576).toFixed(0)} MB, ${X.face_rows(Hh.h)} box rows`);
  tOut = now();
  if (((S.parted || S.dieParted || hostParted || callsParted) && !GO_ON) || k >= TICKS) { stopped = true; throw new Stop(); }
} as any;

function table(T0: Tally, n = 30): string { return [...T0].sort((a, b) => b[1].ticks - a[1].ticks).slice(0, n).map(([q, e]) => `    ${q}: ${e.n} places over ${e.ticks} ticks, e.g. ${e.eg.slice(0, 160)}`).join('\n') || '    (none)'; }
let res: any = null, les: any = null, ended = false, callsOff = false;
function finish(g: any, code: number | null = null): never {
  if (ended) process.exit(code ?? 3); ended = true; console.log = LOG; console.error = ERR; console.warn = WARN;
  const Hh = H; if (!Hh) { say('play: the game never stepped'); process.exit(code ?? 3); }
  // (where the play stopped short of its end, the last state of both sides is kept: what the player read against what the Rust game held)
  if (OUT && g && (les?.code || res?.end === 'error')) { try { X.face_export(Hh.h); writeFileSync(`${OUT}/last-rust.bin`, F.outView()); writeFileSync(`${OUT}/last-puppet.bin`, exportGame(g, Hh.B.reg)); } catch { /* the puppet may not export where it threw */ } }
  const wall = (now() - wall0) / 1000, per = (x: number) => (x / Math.max(1, k)).toFixed(3), list = (M: Map<string, number | string>) => [...M].map(([n, c]) => `${n} ${typeof c === 'number' ? `x${c}` : `(${c})`}`).join(' · ') || '(none)';
  const mineEnd = res ? `${res.end} (${String(res.why).split('\n')[0]}); t ${res.t}; team ${res.team} of ${res.of}; hostiles ${res.hostiles} of ${res.hostilesOf}` : les ? `${les.code === 0 ? 'every step went by and the run ended' : `STOPPED (${les.why})`}; ${les.steps.length} steps gone by: ${les.steps.map((s: any) => s.id).join(' ')}` : '(the play was stopped before its end)';
  say(''); say(`THE RUST GAME PLAYED BY THE SCRIPTED PLAYER THROUGH THE PUPPET, ${AUDIT ? 'AUDITED (the whole state applied and the puppet exported whole every tick)' : 'THE FAST WAY'}: ${scn}:${seed}, ${k} ticks`);
  say(`  held to the plain TypeScript play after every tick: ${S.compared} ticks x (the die's count + ${NAMES.length - 1} parts' hashes; \`poses\` apart)${post.length ? '' : '  NOTHING COMPARED: no reference'}`);
  say(`  the host's half (the record, the cameras, the Input rows) held to the plain play's: ${hostCompared ? (hostParted ? `PARTED AT TICK ${hostParted}` : `equal on every one of ${hostCompared} ticks`) : 'not compared (the reference kept no h<k>.bin)'}`);
  say(`  the block: ${S.parted ? `PARTED AT TICK ${S.parted} in [${S.partedIn}]` : `equal after every one of ${S.compared} ticks`}; the die's count: ${S.dieParted ? `PARTED AT TICK ${S.dieParted}` : 'equal after every tick'}; \`poses\` ${S.poolParted ? `parts from tick ${S.poolParted}` : 'equal'}`);
  say(`  the end here:           ${mineEnd} at tick ${k}`);
  say(`  the end in TypeScript:  ${refEnd.replace(/^TypeScript: /, '')}`);
  say(`  ms a tick (mean of ${k}): the scripted player and its world's own lines ${per(T.driver)} · Game.update's own lines ${per(T.hostTs)} · ${Hh.times()} · THE CHECK, not the host's: the host's half held to the reference's ${per(T.hostHold)}, the die and the block's hashes ${per(T.compare)}${STALE ? `, the stale check ${per(T.stale)}` : ''}`);
  say(`  wall: ${wall.toFixed(1)} s for ${k} ticks (${(wall * 1000 / Math.max(1, k)).toFixed(2)} ms a tick, the check included; ${((wall * 1000 - T.hostHold - T.compare - T.stale) / Math.max(1, k)).toFixed(2)} without it); the module's memory ${(F.mem.buffer.byteLength / 1048576).toFixed(0)} MB`);
  say(`  whole applies, by why: ${list(Hh.wholes)}; starts ${Hh.starts}`);
  say(`  the driver's calls that went to the Rust game: ${list(Hh.calls)}; the light meter handed ${Hh.wb.meter} times`);
  { // (the driver's calls, in the replay loop's words, beside the plain play's own files)
    const ref = (n: string) => refLines(n).filter(l => !/ chron_load$/.test(l)), cmp = (name: string, mine: string[], theirs: string[]) => { const a = mine.join('\n'), b = theirs.join('\n'); if (a === b) return `${name}: the same ${mine.length} lines`; callsOff = true; const i = mine.findIndex((l, j) => l !== theirs[j]); return `${name}: DIFFER (${mine.length} here, ${theirs.length}); the first that differs: ${JSON.stringify(mine[i < 0 ? theirs.length : i] ?? '(none)')} here, ${JSON.stringify(theirs[i < 0 ? mine.length : i] ?? '(none)')}`; };
    if (post.length && k >= post.length) say(`  the driver's calls held to the plain play's: ${cmp('orders (gives.txt)', Hh.gave, ref('gives.txt'))} · ${cmp('checkpoints, floods and picks (hostwrites.txt)', Hh.wrote, ref('hostwrites.txt'))}`); }
  say(`  out events dispatched on the puppet's host objects, over ${Hh.outTicks} ticks: ${list(Hh.outN)}`); if (Hh.outThrew.size) say(`    DISPATCH THREW (the first of each arm): ${list(Hh.outThrew)}`);
  if (Hh.outNoHand.size) say(`    not called (the harness's stand-in has no such method): ${list(Hh.outNoHand)}`);
  if (AUDIT) {
    say(`  THE WRITE-BACK (held to ${(Hh.o.baseline ?? 'rust') === 'rust' ? 'the Rust bytes last applied' : 'the puppet exported as the apply left it'}): ${Hh.wb.hostOnly} compares found nothing but the host's half, ${Hh.wb.meter} only the player's light meter (handed by face_meter), ${Hh.wb.fired} something else (the whole puppet handed by face_import_into); the block differed ${Hh.wb.block} times (${[...Hh.BLOCKW].map(([n, c]) => `${n} ${c}`).join(', ') || '-'})`);
    say(`  what TypeScript wrote into the game between ticks, by field (not the host's half):`); say(table(Hh.WB));
    say(`  the host's half as the compare saw it:`); say(table(Hh.HOSTW, 8));
    say(`  seen by the compare and not a write (identity serials, a clip's place in the Rust game's own table):`); say(table(Hh.OWNW, 8));
    say(`  THE APPLY, held to the Rust bytes it was given (the puppet exported again right after it): ${Hh.applyChecks} ticks checked, ${Hh.applyUnequal} with a field outside the cut ones not as Rust has it`); if (Hh.applyUnequal) say(table(Hh.APPLY_MISS));
    say(`    of the fields cut out of every compare, the puppet's differed from the Rust game's on: ${[...Hh.CUTW].map(([q, n]) => `${q} ${n} of ${Hh.applyChecks}`).join(' · ') || 'none'}`);
    say(`  the box world listed on the Rust side ${Hh.uploads.rust} times (${Hh.uploads.late} asked for on a tick not expected, ${Hh.uploads.missed} expected and not asked for)`); }
  if (STALE) { say(`  THE PUPPET HELD TO THE RUST GAME'S WHOLE STATE every ${STALE} ticks (${staleChecks} times): the fields in which it had fallen behind (the filtered list does not carry them, or the TypeScript round a step wrote them):`); say(table(STALE_F, +arg('--stale-show', '80'))); }
  say(`  the apply made new rows: ${JSON.stringify(Hh.made)}; no way back: ${[...Hh.noWay].map(([q, n]) => `${q} x${n}`).join(', ') || '-'}; no setter: ${[...Hh.noSetter].map(([q, n]) => `${q} x${n}`).join(', ') || '-'}; no window: ${[...Hh.noWindow].join(', ') || '-'}`);
  say(`  STAND-INS used: a box's null markings made undefined ${Hh.standIns.marks} times; ${Hh.standIns.lists} missing empty lists made after whole applies; the menu's questions asked of the Rust game ${Hh.asked} times${Hh.o.asks === false ? ' (not installed: --no-asks)' : ''}`);
  if (Hh.handbacks) say(`  HANDED BACK WHOLE: ${Hh.handbacks} times, ${Hh.handbacksInBeat} with a beat of the level's script running`);
  const sameEnd = !post.length ? false : (res ? refEnd.includes(`: ${res.end} (`) && refEnd.includes(`t ${res.t};`) && refEnd.includes(`team ${res.team} of ${res.of}`) && refEnd.includes(`PLAYED OUT at tick ${k} `) : les ? refEnd.includes(` at tick ${k}:`) && refEnd.includes(les.steps.map((s: any) => s.id).join(' ')) : false);
  // (THE CHECK CHECKED: a host's half compared on fewer ticks than were played and held to the reference is a check that died, not a play that passed)
  const owed = Math.min(k, post.length); if (code === null && post.length && !hostParted && hostCompared < owed) { say(`  THE HOST'S HALF WAS COMPARED ON ${hostCompared} OF ${owed} TICKS: the reference lacks h<k>.bin for the rest (a reference made with --no-host, or its files deleted). NOT A PASS.`); code = 3; }
  if (callsParted) say(`  THE DRIVER'S CALLS PARTED AT TICK ${callsParted} (an order, a checkpoint, a flood or a pick made on one side only)`);
  if (code === null) code = S.parted || S.dieParted || hostParted || callsParted ? 1 : !post.length ? 3 : sameEnd ? 0 : (res || les) ? 4 : 0;
  say(`  ${code === 0 ? (res || les ? `EQUAL TO THE END, and it ended the same way${callsOff ? ' (BUT THE DRIVER\'S CALLS DIFFER: above)' : ''}` : `equal for the ${k} ticks played (stopped by --ticks)`) : code === 1 ? 'PARTED' : code === 4 ? 'equal every tick, BUT THE END IS NOT WORDED THE SAME' : `exit ${code}`}`);
  process.exit(code); }
const LOG = console.log, ERR = console.error, WARN = console.warn;
try { if (LESSON) les = await (await import('../qa/lesson-run.ts')).runLesson(scn, { seed, sky: false, glow: false, floor: 0 }); else res = await play(scn, seed, 300, false, {}); }
catch (e) { if (!(e instanceof Stop)) { console.log = LOG; say(`play: ${(e as any)?.stack ?? e}`); finish(G0, 3); } }
// (playthrough.ts turns whatever is thrown under it into a run that ended 'error': this file's own stop is not the player's)
if (stopped) { res = null; les = null; } else if (les && les.code !== 0) say(`play: THE LESSON'S PLAYER STOPPED ON THE PUPPET at tick ${k}: ${String(les.why).slice(0, 600)}`); else if (res?.end === 'error') say(`play: THE SCRIPTED PLAYER THREW ON THE PUPPET at tick ${k}: ${res.why}`);
finish(G0);
