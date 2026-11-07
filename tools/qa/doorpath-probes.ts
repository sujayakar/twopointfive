// The 'doorpath' family: EACH MAN OF A ROOM ENTRY WALKS A REAL PATH THROUGH THE DOOR (src/game/doorpath.ts DOORPATH: a settled rule; its gun rule, DOORPATH.gun, is still a switch and ships off).
// The checks are made on whole seeded playthroughs (tools/qa/playthrough.ts — the scripted commander, the real door orders, the real fight), each watched by
// tools/qa/paths-watch.ts on every frame of every room entry:
//   doorpath-paths   over the seeded runs below: no man of an entry in a door leaf (and no leaf swept through one), none through a
//                    friend, no two on one spot (centres under 0.45 m for over 0.3 s), none in a jamb or any other static — and the walk was really used (men sent on it, most of them walked it out)
//                    of `GUN_SECS` or longer fails; a shorter brush (a muzzle swung past a leaf's edge) is listed, not failed
//   doorpath-file    the men cross the frame line one at a time in the order dealt (nobody crosses while a man dealt ahead of him is
//                    still outside; nobody is dealt ahead of a man who stood well nearer the frame AT HIS OWN JAMB), no two nearer than a body apart
//                    as they cross, and nobody is kept waiting past the cap. The order dealt is the entry's own queue, kept by the watcher from the
//                    frame it is dealt; until 10-13 it was guessed from who stood nearer the frame at the go, whichever jamb he stood at — which a
//                    stack split unevenly round a door breaks by the deal alone (the rebuilt barracks' s1: the two sides go in turn about, the far
//                    side's man stands 0.86 m further off and is dealt second). That crossing is listed, not failed. ITS RED PATH:
//                    `DOORPATH_FILE_BREAK=1` preloads tools/qa/doorpath-backward.ts (the walk lets each entry go back to front) and must FAIL on
//                    the order line, with a "red path" line that must PASS
// THE ENTRY AS IT WAS IS GONE (2026-11-07): `DOORPATH_OLD=1` ran paths and gun with the walk switched off and had to fail. The walk is settled, the word is
// out, and neither of the two has a red path of its own now; doorpath-file keeps its own (DOORPATH_FILE_BREAK: above).
// `PT_NOCLOSER=1` in front of any of them: no door shuts itself — the leaves an entry meets once doors stay open.
import { existsSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';

const ROOT = new URL('../..', import.meta.url).pathname.replace(/\/$/, '');
/** the seeded runs: pairs and fours, a kicked door and a bang, the office and the built levels — each one reaches a room entry on both stands (the depot's
 *  seed 2 was here until 10-13: on the depot as rebuilt its scripted four is wiped in the yard before any door, as that floor means it to be) */
export const DOORPATH_RUNS = ['room-clear-four:1', 'room-clear-pair:1', 'hostage-rescue:1', 'depot-four:1', 'records-pair:1', 'barracks-four:1'];
/** a rifle or a light through a leaf this long is a failure (seconds; the runs step at 60 Hz) */
export const GUN_SECS = 0.2;

function check(title: string) {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const memo = new Map<string, Promise<any>>();
/** one seeded playthrough in a process of its own (the tables it sets are that process's), its JSON back */
function runOne(spec: string, env: Record<string, string>, pre = ''): Promise<any> {
  const key = spec + '|' + JSON.stringify(env) + (pre && '|' + pre); let p = memo.get(key);
  if (!p) { p = (async () => { const f = `${tmpdir()}/doorpath-${process.pid}-${memo.size}.json`, E: Record<string, string> = { ...(process.env as any), PT_DIGEST: '1', ...env };
      for (const k of ['TPS_DOORPATH', 'TPS_CAST', 'TPS_MOVES', 'TPS_TURN', 'PT_PATHS']) if (!(k in env)) delete E[k];   // (word left out on purpose: a word a job does not name is not the shell's either — the run plays that rule as the stand has it, a shell's TPS_MOVES=0 included; a job that wants one OFF says '0')
      if (!('PT_PATHS' in env)) delete E.PT_NOCLOSER;   // (the digests are the runs as they were: doors shut themselves)
      const pr = Bun.spawn(['bun', 'run', ...(pre ? ['--preload', pre] : []), `${ROOT}/tools/qa/playthrough.ts`, '--one', spec, '--out', f], { env: E, stdout: 'ignore', stderr: 'pipe' });
      if (await pr.exited !== 0 || !existsSync(f)) throw new Error(`${spec}: the run did not finish — ${await new Response(pr.stderr).text()}`);
      const out = JSON.parse(readFileSync(f, 'utf8')); rmSync(f, { force: true }); return out; })(); memo.set(key, p); }
  return p;
}
type Job = [spec: string, env: Record<string, string>, pre?: string];
/** a list of runs, four at a time (`soft`: a run that did not finish comes back as { error } in its place) */
async function runJobs(jobs: Job[], soft = false): Promise<any[]> {
  const out: any[] = new Array(jobs.length); let next = 0;
  await Promise.all([0, 1, 2, 3].map(async () => { while (next < jobs.length) { const i = next++; out[i] = await runOne(...jobs[i]).catch(e => { if (!soft) throw e; return { error: String(e?.message ?? e) }; }); } }));
  return out;
}
// (a child stands the game as the page opens it, the walk on. `knobs`: the stand's own word for the walk's table, for the one row of it that is still a switch)
const watched = (knobs = '', pre = '') => runJobs(DOORPATH_RUNS.map((s): Job => [s, { PT_PATHS: '1', ...(knobs ? { TPS_DOORPATH: `1${knobs}` } : {}) }, pre]));

export async function doorpathPaths(): Promise<boolean> {
  const C = check(`doorpath-paths: nobody of a room entry in a leaf, through a friend or in a jamb`);
  const R = await watched();
  for (const kind of ['leaf', 'friend', 'near', 'jamb'] as const) {
    const bad = R.flatMap(r => (r.paths.spells as any[]).filter(s => s.kind === kind).map(s => `${r.scn}:${r.seed} ${s.who} ${kind === 'leaf' ? `${s.swept ? 'swept by' : 'in'} the leaf of ${s.what}` : kind === 'friend' ? `through ${s.what}` : kind === 'near' ? `on one spot with ${s.what}` : `in a static at ${s.what}`} at ${s.t0} s (${s.frames} frames)`));
    C.verdict(bad.length === 0, kind === 'leaf' ? 'no man of an entry stands in a door leaf, and no leaf goes through one' : kind === 'friend' ? 'no man of an entry stands through a friend' : kind === 'near' ? 'no two living men stand on one spot during an entry (centres under 0.45 m for over 0.3 s; a teammate and a teammate, or a defender)' : 'no man of an entry stands in a jamb, a wall or furniture',
      bad.length ? `${bad.length}: ${bad.slice(0, 6).join(' · ')}${bad.length > 6 ? ' · …' : ''}` : `${R.reduce((a, r) => a + r.paths.entries, 0)} entries, ${R.reduce((a, r) => a + r.paths.frames, 0)} man-frames watched over ${R.length} runs`);
  }
  { const B = R.flatMap(r => (r.paths.spells as any[]).filter(s => s.kind === 'body').map(s => `${r.scn}:${r.seed} ${s.who} on ${s.what} at ${s.t0} s`));
    console.log(`- (listed, not judged) a man of an entry standing on a fallen defender for over 0.3 s: ${B.length}${B.length ? ` — ${B.slice(0, 4).join(' · ')}${B.length > 4 ? ' · …' : ''}` : ''}`); }
  { const sent = R.reduce((a, r) => a + (r.doorpath?.sent ?? 0), 0), walked = R.reduce((a, r) => a + (r.doorpath?.walked ?? 0), 0), gave = R.reduce((a, r) => a + (r.doorpath?.gaveUp ?? 0), 0);
    C.verdict(sent > 0 && walked >= sent * 0.6, 'the walk is what carried them: men were sent on it and most walked it out', `sent ${sent}, walked to the point ${walked}, handed back to the nav ${gave}, taken by the fight ${R.reduce((a, r) => a + (r.doorpath?.taken ?? 0), 0)}, no way round a leaf ${R.reduce((a, r) => a + (r.doorpath?.noRound ?? 0), 0)}`); }
  return C.done();
}

export async function doorpathFile(): Promise<boolean> {
  const word = process.env.DOORPATH_FILE_BREAK ?? '', brk = word === '1';
  if (!['', '0', '1'].includes(word)) throw new Error(`DOORPATH_FILE_BREAK='${word}' not understood: 1`);
  const C = check('doorpath-file: the men go in one at a time, in the order dealt, a body apart');
  if (brk) console.log('  DOORPATH_FILE_BREAK=1: the walk lets every entry go back to front — the "dealt ahead of him" line below must FAIL and the "red path" line must PASS');
  // the red path's preload (doorpath-backward.ts) evaluates doorpath.ts ahead of every other game module: sound only while that file brings in no other
  // game module's values. (This line was the off side's probe's until that went, 2026-11-06.)
  { const src = readFileSync(`${ROOT}/src/game/doorpath.ts`, 'utf8'), value: string[] = []; let types = 0, outside = 0;
    for (const m of src.matchAll(/^(?:import\s*(?:(type\s)?([^'";]*?)\sfrom\s*)?|export\s+(type\s)?(?:\*|\{[^}]*\})[^'";]*?\sfrom\s*)['"]([^'"]+)['"]/gm)) {
      const names = /^\{([^}]*)\}$/.exec((m[2] ?? '').trim())?.[1].split(',').map(n => n.trim()).filter(Boolean);
      if (m[1] || m[3] || names?.length && names.every(n => /^type\s/.test(n))) types++;
      else if (new URL(m[4], 'file:///src/game/doorpath.ts').pathname.startsWith('/src/game/')) value.push(m[0].replace(/\s+/g, ' '));
      else outside++;
    }
    if (/\bimport\s*\(/.test(src)) value.push('an import() call');
    C.verdict(value.length === 0, 'src/game/doorpath.ts brings in no other game module but for its types (the red path\'s preload may evaluate it first)',
      value.length ? value.join(' · ') : `${types} imports of types${outside ? `, ${outside} of values from outside src/game (not followed)` : ''}`); }
  const R = await watched('', brk ? `${ROOT}/tools/qa/doorpath-backward.ts` : '');
  const cross = R.flatMap(r => (r.paths.cross as any[]).map(c => ({ ...c, run: `${r.scn}:${r.seed}` }))), said = (c: any) => `${c.run} ${c.who} at ${c.door}, ${c.t} s`;
  const unread = cross.filter(c => !(c.turn >= 0)), jumped = cross.filter(c => c.jumped), own = cross.filter(c => c.far === 'own'), other = cross.filter(c => c.far === 'other' && !c.jumped);
  const close = cross.filter(c => c.gapM !== null && c.gapM < 0.6), gaps = cross.filter(c => c.gapS !== null && c.gapS < 6).map(c => c.gapS as number).sort((a, b) => a - b);
  C.verdict(cross.length > 0 && unread.length === 0, 'the order dealt was read for every man who crossed', unread.length ? unread.map(said).join(' · ') : `${cross.length} crossings`);
  C.verdict(jumped.length === 0, 'nobody crosses the frame line while a man dealt ahead of him is still outside', jumped.length ? jumped.map(said).join(' · ') : 'none');
  C.verdict(own.length === 0, 'nobody is dealt ahead of a man who stood well nearer the frame at his own jamb', own.length ? own.map(said).join(' · ')
    : other.length ? `listed, not failed — dealt ahead of a man who stood over half a metre nearer at the OTHER jamb (the two sides go in turn about): ${other.map(said).join(' · ')}` : 'none, at either jamb');
  if (brk) C.verdict(jumped.length > 0 && unread.length === 0, 'the red path: let go back to front, men read as crossing out of turn', `${jumped.length} of ${cross.length} crossings`);
  C.verdict(close.length === 0, 'no two men nearer than 0.6 m as the second crosses the frame line', close.length ? close.map(c => `${c.run} ${c.who} ${c.gapM} m behind at ${c.t} s`).join(' · ') : `gaps between men of one entry: median ${gaps[gaps.length >> 1]} s, least ${gaps[0]} s, most ${gaps[gaps.length - 1]} s`);
  C.verdict(R.every(r => (r.doorpath?.capped ?? 0) === 0), 'nobody was held short for the whole cap', R.map(r => `${r.scn}:${r.seed} held ${(r.doorpath?.heldS ?? 0).toFixed(1)} s, let go late ${r.doorpath?.late ?? 0}`).join(' · '));
  return C.done();
}

if (import.meta.main) {
  let ok = true; for (const f of [doorpathPaths, doorpathFile]) ok = await f() && ok; process.exit(ok ? 0 : 1);
}
