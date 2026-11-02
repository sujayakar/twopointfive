// WHAT A TICK COSTS — the simulation's own CPU time, tick by tick, on the stand a game server plays: the shipped stand with the senses on the CPU
// (TPS_CPUSENSE=1) and the game's own maths. A scripted play-through (tools/qa/playthrough.ts) is the load; the clock is read round Game.update alone and round
// the box list's pack and trace grid (BoxWorld.upload's CPU half, which rounds and the cursor read; the play-through packs it every 4th tick, the page every frame). After every update the raw bits
// of what moves are hashed and chained (game/digest.ts rawBits, as coop-check does), OUTSIDE the clock: two builds whose `bits` agree played the same run to
// the last place, so their times are for the same work. Not game code: it reads.
//   bun run tools/qa/tickcost.ts                              the runs below, one at a time, each in its own process → the table
//   bun run tools/qa/tickcost.ts --runs compound-four:1       other runs (playthrough.ts's scenarios)
//   bun run tools/qa/tickcost.ts --rounds 3                   each run played n times; the times kept are the round with the least mean (the quietest)
//   bun run tools/qa/tickcost.ts --node                       the same under node (V8, the page's engine) through tools/qa/framecost-v8.mjs
//   bun run tools/qa/tickcost.ts --json out.json              the numbers to a file;  --compare a.json b.json  the before / after table (exit 1 if a run's bits differ)
//   bun run tools/qa/tickcost.ts --prof <dir>                 bun's sampling profiler on every run (one .md a run)
//   bun run tools/qa/tickcost.ts --each                       the box list made again after EVERY update, as a networked game's tick does (the play-through's own cadence is every 4th:
//                                                             another run, so its bits are compared with another --each run's only)
//   bun run tools/qa/tickcost.ts --list                       … and made by BoxWorld.relist() (the half the game reads) instead of upload(): the same run to the bit as without it
//   TICKCOST_BREAK=1                                          the red path: a run is played with the last bit of one float flipped once, after the warm-up; --compare must then FAIL
// EXIT 0 = every run left its ticks (and, under --compare, every run's bits agree) · 1 = a run left none / bits differ / a run is in one file only · 2 = unknown flag
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { resolve } from 'node:path';

const RUNS = ['compound-four:1', 'warehouse-four:1', 'pass-four:1', 'hostage-rescue:1', 'block-squads:1'];
const WARM = 120;   // the first two seconds of a run are left out of the times (the JIT warming up, the first paths)
interface Row { scn: string; seed: number; ticks: number; end: string; digest: string; bits: string; engine: string
  update: Stat; step: Stat; over4: number; over8: number; over16: number }
interface Stat { mean: number; p50: number; p95: number; p99: number; max: number; sum: number }
const stat = (a: number[]): Stat => { const s = [...a].sort((x, y) => x - y), q = (p: number) => s.length ? s[Math.min(s.length - 1, Math.floor(p * s.length))] : 0;
  const sum = a.reduce((x, y) => x + y, 0); return { mean: a.length ? sum / a.length : 0, p50: q(0.5), p95: q(0.95), p99: q(0.99), max: s.length ? s[s.length - 1] : 0, sum }; };
const engineName = (): string => { const B = (globalThis as { Bun?: { version: string } }).Bun; return B ? `bun ${B.version}` : `node ${process.version}`; };

/** one run in THIS process */
async function one(scn: string, seed: number): Promise<Row> {
  process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.PT_DIGEST ??= '1';
  const { Game } = await import('../../src/game/game.ts');
  const PT = await import('./playthrough.ts');
  const { rawBits, RAW_BITS_SEED } = await import('../../src/game/digest.ts');
  const BREAK = process.env.TICKCOST_BREAK === '1';
  const LIST = process.env.TICKCOST_LIST === '1', EACH = process.env.TICKCOST_EACH === '1'; let packNow: () => void = () => { };
  let h: number = RAW_BITS_SEED, ticks = 0;
  const upd: number[] = [], stp: number[] = [];
  const update = Game.prototype.update; let wrapped: unknown = null, upMs = 0;
  Game.prototype.update = function (this: any, ...a: unknown[]) {
    const world = this.engine?.world;
    if (world && wrapped !== world) { wrapped = world; const up = world.upload;
      const timed = function (this: unknown, ...x: unknown[]) { const t0 = performance.now(); const r = LIST ? up.call(this, true, true) /* = relist(), spelt out: world.upload is this wrapper */ : up.apply(this, x); upMs += performance.now() - t0; return r; };
      world.upload = EACH ? function () { /* the harness's own pack, every 4th tick: this run packs after every update instead (below) */ } : timed; packNow = () => timed.call(world); }
    if (ticks > WARM) { stp.push(upMs); } upMs = 0;   // (the upload that followed the LAST update: the harness packs the box list after the game's step)
    const t0 = performance.now(); const r = (update as (...x: unknown[]) => unknown).apply(this, a); const ms = performance.now() - t0;
    if (BREAK && ticks === WARM) { const f = new Float64Array([this.player.char.pos[0]]); new BigUint64Array(f.buffer)[0] ^= 1n; this.player.char.pos[0] = f[0]; }   // (the red path: one float, its last bit)
    if (EACH) packNow();   // (a networked game's tick: the list is made again after every update)
    h = rawBits(this as never, h); if (++ticks > WARM) upd.push(ms); return r; } as typeof update;
  const r = await (PT as any).play(scn, seed, 300, false, {}) as any;
  const u = stat(upd), s = stat(stp);
  return { scn, seed, ticks, end: String(r.end), digest: String(r.digest), bits: h.toString(36), engine: engineName(), update: u, step: s,
    over4: upd.filter(x => x > 4).length, over8: upd.filter(x => x > 8).length, over16: upd.filter(x => x > 16.6).length };
}

const MARK = 'TICKCOST-RUN ';
const argv = process.argv.slice(2);
const flag = (n: string): string | undefined => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const f2 = (x: number) => x.toFixed(2);
function table(rows: Row[]): string {
  const head = ['run', 'ticks', 'end', 'update mean', 'p50', 'p95', 'p99', 'max', '>4', '>8', '>16.6', 'pack mean', 'pack max', 'bits'];
  const body = rows.map(r => [`${r.scn}:${r.seed}`, String(r.ticks), r.end, f2(r.update.mean), f2(r.update.p50), f2(r.update.p95), f2(r.update.p99), f2(r.update.max), String(r.over4), String(r.over8), String(r.over16), f2(r.step.mean), f2(r.step.max), r.bits]);
  const n = rows.reduce((a, r) => a + (r.ticks - WARM), 0) || 1, w = (k: (r: Row) => number) => rows.reduce((a, r) => a + k(r) * (r.ticks - WARM), 0) / n;
  body.push(['ALL (by ticks)', String(rows.reduce((a, r) => a + r.ticks, 0)), '', f2(w(r => r.update.mean)), '', '', '', f2(Math.max(0, ...rows.map(r => r.update.max))), String(rows.reduce((a, r) => a + r.over4, 0)), String(rows.reduce((a, r) => a + r.over8, 0)), String(rows.reduce((a, r) => a + r.over16, 0)), f2(w(r => r.step.mean)), '', '']);
  const wd = head.map((h, i) => Math.max(h.length, ...body.map(b => b[i].length)));
  return [head, ...body].map(l => l.map((c, i) => i ? c.padStart(wd[i]) : c.padEnd(wd[i])).join('  ')).join('\n');
}

if (argv[0] === '--one') {
  const [scn, seed] = argv[1].split(':'); const row = await one(scn, Number(seed));
  console.log(MARK + JSON.stringify(row)); process.exit(0);
} else if (argv[0] === '--compare') {
  const A = JSON.parse(readFileSync(argv[1], 'utf8')), B = JSON.parse(readFileSync(argv[2], 'utf8')); let bad = false;
  console.log(`tick cost · before ${A.commit} (${A.engine}) → after ${B.commit} (${B.engine}) · ms a tick, Game.update alone, then the box list's pack (a tick's share: ${A.pack ?? 'every 4th tick, whole'} → ${B.pack ?? 'every 4th tick, whole'})`);
  const pct = (a: number, b: number) => `${b <= a ? '−' : '+'}${Math.abs(100 * (b - a) / (a || 1)).toFixed(0)} %`;
  const lines: string[][] = [['run', 'ticks', 'mean', '', 'p95', '', 'p99', '', 'pack mean', '', 'same run']];
  let ta = 0, tb = 0, sa = 0, sb = 0, n = 0;
  const gone = (r: Row, where: string) => { bad = true; lines.push([`${r.scn}:${r.seed}`, `${r.ticks}`, '', '', '', '', '', '', '', '', `NOT IN THE ${where} FILE`]); };
  for (const b of B.rows as Row[]) if (!(A.rows as Row[]).some(x => x.scn === b.scn && x.seed === b.seed)) gone(b, 'FIRST');
  for (const a of A.rows as Row[]) { const b = (B.rows as Row[]).find(x => x.scn === a.scn && x.seed === a.seed); if (!b) { gone(a, 'SECOND'); continue; }
    const same = a.bits === b.bits && a.ticks === b.ticks && a.digest === b.digest; if (!same) bad = true;
    const k = a.ticks - WARM; ta += a.update.mean * k; tb += b.update.mean * k; sa += a.step.mean * k; sb += b.step.mean * k; n += k;
    lines.push([`${a.scn}:${a.seed}`, `${a.ticks}`, `${f2(a.update.mean)} → ${f2(b.update.mean)}`, pct(a.update.mean, b.update.mean), `${f2(a.update.p95)} → ${f2(b.update.p95)}`, pct(a.update.p95, b.update.p95), `${f2(a.update.p99)} → ${f2(b.update.p99)}`, pct(a.update.p99, b.update.p99), `${f2(a.step.mean)} → ${f2(b.step.mean)}`, pct(a.step.mean, b.step.mean), same ? 'to the bit' : `DIFFERS (${a.ticks}/${b.ticks} ticks · ${a.bits}/${b.bits})`]); }
  if (!n) bad = true;   // (no run in both files: nothing was compared)
  if (n) lines.push(['ALL (by ticks)', '', `${f2(ta / n)} → ${f2(tb / n)}`, pct(ta, tb), '', '', '', '', `${f2(sa / n)} → ${f2(sb / n)}`, pct(sa, sb), bad ? 'A RUN DIFFERS' : 'every run to the bit']);
  const wd = lines[0].map((_, i) => Math.max(...lines.map(l => l[i].length)));
  console.log(lines.map(l => l.map((c, i) => i ? c.padStart(wd[i]) : c.padEnd(wd[i])).join('  ')).join('\n'));
  process.exit(bad ? 1 : 0);
} else {
  const known = ['--runs', '--rounds', '--node', '--json', '--prof', '--each', '--list'];
  for (const a of argv) if (a.startsWith('--') && !known.includes(a)) { console.error(`unknown flag ${a} — ${known.join(' ')} --compare --one`); process.exit(2); }
  const runs = (flag('--runs') ?? RUNS.join(',')).split(','), rounds = Math.max(1, Number(flag('--rounds') ?? 1)), node = argv.includes('--node'), prof = flag('--prof');
  if (prof) mkdirSync(prof, { recursive: true });
  const self = resolve(import.meta.dir, 'tickcost.ts'), loader = resolve(import.meta.dir, 'framecost-v8.mjs');
  const rows: Row[] = []; let failed = false;
  for (const run of runs) { let best: Row | null = null;
    for (let k = 0; k < rounds; k++) {
      const cmd = node ? ['node', '--no-warnings', '--import', loader, self, '--one', run]
        : ['bun', 'run', ...(prof ? ['--cpu-prof-md', `--cpu-prof-dir=${resolve(prof)}`, `--cpu-prof-name=${run.replace(':', '-')}.md`] : []), self, '--one', run];
      const pr = Bun.spawn(cmd, { cwd: resolve(import.meta.dir, '../..'), stdout: 'pipe', stderr: 'pipe', env: { ...process.env, TPS_PROFILE: process.env.TPS_PROFILE ?? 'shipped', TPS_CPUSENSE: process.env.TPS_CPUSENSE ?? '1', PT_DIGEST: '1', ...(argv.includes('--each') ? { TICKCOST_EACH: '1' } : {}), ...(argv.includes('--list') ? { TICKCOST_LIST: '1' } : {}), ...(node ? { V8_ENTRY: self, FC_NOSTANDIN: '1' } : {}) } });
      const [out, err] = await Promise.all([new Response(pr.stdout).text(), new Response(pr.stderr).text()]); await pr.exited;
      const line = out.split('\n').find(l => l.startsWith(MARK));
      if (!line) { console.error(`  ${run} left nothing: ${err.trim().split('\n').slice(-3).join(' / ').slice(0, 400)}`); failed = true; break; }
      const row = JSON.parse(line.slice(MARK.length)) as Row;
      if (best && (best.bits !== row.bits || best.ticks !== row.ticks)) { console.error(`  ${run}: two rounds of one build played DIFFERENT runs (${best.bits} / ${row.bits})`); failed = true; }
      if (!best || row.update.mean < best.update.mean) best = row; }
    if (best) { rows.push(best); console.error(`  ${run} · ${best.ticks} ticks · update ${f2(best.update.mean)} ms · pack ${f2(best.step.mean)} ms · ${best.bits}`); } }
  const git = (a: string[]) => { try { return new TextDecoder().decode(Bun.spawnSync(['git', ...a], { cwd: resolve(import.meta.dir, '../..') }).stdout).trim(); } catch { return ''; } };
  const out = { tool: 'tickcost', commit: git(['rev-parse', '--short=8', 'HEAD']), dirty: git(['status', '--porcelain', '--', 'src']) !== '', when: new Date().toISOString(), engine: rows[0]?.engine ?? '', rounds, warm: WARM, pack: `${argv.includes('--each') ? 'after every tick' : 'every 4th tick'}, ${argv.includes('--list') ? 'the list alone' : 'whole'}`, rows };
  console.log(`tick cost · ${out.commit}${out.dirty ? ' + uncommitted src' : ''} · ${out.engine} · rounds ${rounds} (the quietest kept) · first ${WARM} ticks of a run left out · ms`);
  console.log(table(rows));
  const j = flag('--json'); if (j) writeFileSync(j, JSON.stringify(out, null, 1));
  process.exit(failed || !rows.length ? 1 : 0);
}
