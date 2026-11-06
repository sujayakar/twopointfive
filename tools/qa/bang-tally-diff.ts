#!/usr/bin/env bun
// WHAT A CHANGE TO A LEVEL DID AT ITS DOORS — bang-tally.ts on one scenario, the tree BEFORE the change against the tree AFTER it, door by door.
//   bun run tools/qa/bang-tally-diff.ts <scenario> --before <checkout | tally.json[,tally.json]> [--after <checkout | tally.json[,…]>] [--seeds 1-24] [--more 25-48 | none] [--jobs 4] [--keep dir]
//   bun run tools/qa/bang-tally-diff.ts --self        the reading's own arithmetic, in a second (no game is stood up)
//     --before / --after   a checkout of the game (a `git worktree` of the commit before the change; --after unsaid = this tree), where the tally is played, or the
//                          JSON a tally already wrote there (`bun run tools/qa/bang-tally.ts --scn <id> --seeds … --arms open,bang --json f`; in a tree
//                          from before one stand, `TPS_PROFILE=shipped` in front)
//     --seeds / --more     the first block of seeds, and the second, which is read only when the first leaves a door suspect or too thin to say
//                          (`--more none`: one block is the whole reading; `--seeds 1-48 --more none` reads 48 at once)
//     --keep dir           keep the tallies' JSON there (to hand back as --before / --after without playing them again)
// IT DECLARES, IT DOES NOT GATE. Nobody's merge is red by it and it is in no gate's list: an author who adds or moves a solid thing that stops feet (over
// 0.19 m) within reach of a door runs it, and says in the step's notes what it printed. A door it NAMES is the author's to explain or to mend.
// WHY IT EXISTS (11-05, the pass): a log laid across two others by a chopping block shut a square metre of the walk grid in front of two doors. No check of
// the dressing saw it (lived-stack-check reads where a stack STANDS, wallfurn-check where a charge goes), and the tally's rows, read by eye on a few seeds,
// blamed one door and a direction that 48 seeds did not bear out. What a thing by a door does to the fight there is only in the fight, and a fight re-rolled
// goes either way: so the tally is read in both trees, on enough seeds, by a test that knows what chance alone does.
// WHAT IT READS: every door of every held room (bang-tally's own list), the plain door and the bang, one entry a seed, ON THE PAGE'S STAND ONLY. An entry is
// COMMON when both trees played it (a room with no defender on a seed is in neither count; an entry with a man lost before the order is counted, which the
// tally's own table leaves out: where a team can gather is part of what a thing by a door changes).
// AN ENTRY WENT WORSE when the room was held before and is not now, or it ended the same way and the team lost more men; BETTER the other way round. A fight
// re-rolled is as likely to go better as worse, so what is asked of a door's arm is how lopsided the split is: `chance` is the exact two-sided sign test
// (how often a fair coin splits the moved entries at least this unevenly).
// TWO BLOCKS OF SEEDS. On the first, an arm whose chance is SUSPECT_P or less is SUSPECT (and one with under FEW common entries is too thin to say); then,
// and only then, the second block is played in both trees and every arm is read over all the seeds. A DOOR IS NAMED when an arm's chance over the whole
// reading is NAMED_P or less, or when an entry that was made before cannot be made now (no floor outside the door for the team, or the game refuses the
// order where it took it: the thing is where the team would stand or stack). A suspect door that could not be read further is said so, and is not cleared.
// THE SAME ENTRY IN BOTH TREES. The tally stands the team on free floor 2.4 to 6.2 m out from the door and Sam beside them, and takes anybody with a line
// to them off the floor: a thing laid there changes the STAGING, and two trees staged differently did not fight the same entry (the pass's bunk room door
// was named at 1 in 230 for a log moved, and all that had changed there was where an unseen Sam was stood, 0.8 m). So the AFTER tree is stood AS the before
// tree stood each entry (`bang-tally.ts --stand-as`, the tally's `at`), wherever those spots are free floor after the change; the table says how many
// entries were `staged alike`, and a verdict on a door that could not be says so. A tally handed over as JSON was staged as it was.
// WHAT A NAMED DOOR SAYS, AND DOES NOT: the entry at that door comes out differently after the change, more often one way than chance allows, on that tree.
// Staged alike, the difference is in the walk and the fight; it still does not say why. Read one moved entry in both trees before writing a cause (the
// verdict's line gives the command).
// HOW OFTEN IT CRIES WOLF, and what it misses, was measured when it was written (docs/agents/qa.md, DOOR TALLIES BEFORE AGAINST AFTER).
// EXIT 0 no door named · 1 a door named · 2 bad usage · 3 no reading (a suspect door not read further, nothing that could be compared, a door in one tree
// only, a tally that did not run whole, a checkout that changed while it was played).
// THE TALLY'S WORDS: an entry's `outcome` is read where the tally writes it; a tally from before it did is read by its notes, and cannot say an order was
// refused (such an entry is a fight to it, as it always was to the tally's own table), so no door is named for a refusal one of the two tallies could not say. TALLYDIFF_BREAK=1 = the tool's own red path: the AFTER tree is played
// with a crate on the way in to the door each entry is played on (bang-tally.ts stage), and a door must be named: exit 1, nothing else.
import { existsSync, mkdirSync, readFileSync, statSync, unlinkSync, writeFileSync } from 'node:fs';
import { basename, relative, resolve } from 'node:path';
import type { Result } from './bang-tally.ts';

const HERE = resolve(import.meta.dir, '../..');
export const SUSPECT_P = 0.05, NAMED_P = 0.005, FEW = 12, ARMS = ['open', 'bang'];

/** a line of bang-tally's JSON; `at` and `outcome` are unsaid by a tally from before they were written */
export type Entry = Omit<Result, 'at' | 'outcome'> & { at?: string; outcome?: Result['outcome'] };
export interface DoorRow { door: string; room: string; arm: string; inB: number; inA: number; n: number; same: number; alike: number; restaged: number; unsaid: number; gone: number; goneMoved: number; come: number;
  heldB: number; heldA: number; lostB: number; lostA: number; team: number; worse: number[]; better: number[]; chance: number; few: boolean }

/** what became of an entry: the tally's own word, or, of a tally from before it wrote one, its note read as its table reads it (which has no word for a refusal) */
const outcome = (r: Entry): Result['outcome'] => r.outcome ?? (/threw/.test(r.note ?? '') ? 'threw' : /no floor/.test(r.note ?? '') ? 'no-floor' : r.note && !r.of ? 'nobody' : 'played');
const threw = (r: Entry) => outcome(r) === 'threw';                                              // the run itself failed: not an outcome at a door
const fought = (r: Entry) => outcome(r) === 'played' || outcome(r) === 'refused';                // (a refused order leaves the team outside a hot room: it is counted as it falls out)
/** an entry made before that cannot be made now: no floor to stand the team, or an order taken before and refused now (said only where both tallies can say it) */
const lost = (b: Entry, a: Entry) => (fought(b) && outcome(a) === 'no-floor') || (!!b.outcome && !!a.outcome && outcome(b) === 'played' && outcome(a) === 'refused');
const held = (r: Entry) => !!(r.taken || r.held);
const same = (a: Entry, b: Entry) => a.lost === b.lost && a.down === b.down && a.taken === b.taken && held(a) === held(b) && (a.secs ?? -1) === (b.secs ?? -1);
/** +1 the entry went worse for the team after the change, -1 better, 0 neither: the ending first, then the men lost */
const turn = (b: Entry, a: Entry) => held(b) !== held(a) ? (held(b) ? 1 : -1) : Math.sign(a.lost - b.lost);

/** the exact two-sided sign test: how often a fair coin splits `worse + better` tosses at least this unevenly */
export function signChance(worse: number, better: number): number {
  const n = worse + better, k = Math.min(worse, better); if (!n) return 1;
  let c = 1, tail = 0; for (let i = 0; i <= k; i++) { tail += c; c = c * (n - i) / (i + 1); }
  return Math.min(1, 2 * tail / 2 ** n);
}

/** the two tallies of one scenario, door by door and arm by arm, over the entries both played */
export function compare(before: Entry[], after: Entry[], scn: string): DoorRow[] {
  const key = (r: Entry) => `${r.door}|${r.room}|${r.arm}|${r.seed}`;
  const B = new Map(before.filter(r => r.scn === scn).map(r => [key(r), r])), A = new Map(after.filter(r => r.scn === scn).map(r => [key(r), r]));
  const rows = new Map<string, DoorRow>();
  // every door and arm either tree asked about has a row, so a door one tree lacks is said and not silently gone
  const row = (r: Entry) => { const k = `${r.door}|${r.room}|${r.arm}`; let R = rows.get(k);
    if (!R) rows.set(k, R = { door: r.door, room: r.room, arm: r.arm, inB: 0, inA: 0, n: 0, same: 0, alike: 0, restaged: 0, unsaid: 0, gone: 0, goneMoved: 0, come: 0, heldB: 0, heldA: 0, lostB: 0, lostA: 0, team: 0, worse: [], better: [], chance: 1, few: false });
    return R; };
  for (const b of B.values()) row(b).inB++; for (const a of A.values()) row(a).inA++;
  for (const [k, b] of B) { const a = A.get(k); if (!a) continue; const R = row(b);
    if (lost(b, a)) { R.gone++; if (b.at !== a.at) R.goneMoved++; continue; } if (lost(a, b)) { R.come++; continue; }
    if (!fought(b) || !fought(a)) continue;
    const t = turn(b, a); R.n++; if (same(a, b)) R.same++; if (!b.at || !a.at) R.unsaid++; else if (b.at === a.at) R.alike++; else R.restaged++;
    R.heldB += +held(b); R.heldA += +held(a); R.lostB += b.lost; R.lostA += a.lost; R.team += b.team; if (t > 0) R.worse.push(b.seed); if (t < 0) R.better.push(b.seed); }
  for (const R of rows.values()) { R.few = R.n < FEW; R.chance = R.few ? 1 : signChance(R.worse.length, R.better.length); }
  return [...rows.values()].sort((x, y) => x.door.localeCompare(y.door) || x.room.localeCompare(y.room) || y.arm.localeCompare(x.arm));
}

/** --self: the arithmetic above on entries written by hand */
function self(): boolean {
  const E = (seed: number, lost: number, o: Partial<Entry> = {}): Entry => ({ scn: 's', door: 'd', room: 'r', arm: 'open', seed, lost, team: 4, down: 2, of: 2, taken: true, held: true, still: 0, after: 0, secs: 5, bangs: 0, pre: 0, outside: 0, note: '', at: 'x', outcome: 'played', ...o });
  const near = (a: number, b: number) => Math.abs(a - b) < 1e-12, seeds = Array.from({ length: 24 }, (_, i) => i + 1);
  const one = (after: (s: number) => Entry) => compare(seeds.map(s => E(s, 1)), seeds.map(after), 's')[0];
  const worse = one(s => E(s, s <= 9 ? 2 : s === 10 ? 0 : 1)), lostRoom = one(s => E(s, 0, s <= 3 ? { taken: false, held: false } : {})), gone = one(s => E(s, 0, s === 1 ? { outcome: 'no-floor', at: '' } : s === 2 ? { outcome: 'refused', lost: 4 } : {}));
  const old = (e: Entry): Entry => { const { outcome: _o, at: _a, ...rest } = e; return rest; }, blind = compare(seeds.map(s => old(E(s, 1))), seeds.map(s => E(s, s === 2 ? 4 : 1, s === 2 ? { outcome: 'refused' } : {})), 's')[0];
  const restaged = one(s => E(s, 1, { at: s <= 5 ? 'y' : 'x' })), thin = compare(seeds.slice(0, 11).map(s => E(s, 1)), seeds.slice(0, 11).map(s => E(s, 2)), 's')[0];
  const lines: [string, boolean][] = [
    ['the sign test: 9 against 1 is 22 in 1024, 16 against 1 is 36 in 131072, nothing moved is 1, and it reads the same either way round', near(signChance(9, 1), 22 / 1024) && near(signChance(16, 1), 36 / 131072) && signChance(0, 0) === 1 && signChance(3, 11) === signChance(11, 3)],
    ['more men lost with the same ending is worse, fewer is better, and the seeds are kept', worse.worse.length === 9 && worse.better.length === 1 && worse.better[0] === 10 && worse.same === 14 && near(worse.chance, 22 / 1024)],
    ['a room no longer held is worse though fewer men were lost', lostRoom.worse.length === 3 && lostRoom.better.length === 21],
    ['an entry that can no longer be made (no floor, or an order taken before and refused now) is counted as gone and not as a fight, and whether it was staged on other floor is kept', gone.gone === 2 && gone.goneMoved === 1 && gone.n === 22],
    ['against a tally from before the outcome was written, a refused order is a fight like any other and names nothing', blind.gone === 0 && blind.n === 24 && blind.worse.length === 1 && blind.unsaid === 24],
    ['entries staged on other floor are counted', restaged.restaged === 5 && restaged.alike === 19 && restaged.unsaid === 0],
    [`under ${FEW} common entries an arm is too few and has no chance`, thin.few && thin.chance === 1 && thin.worse.length === 11],
  ];
  for (const [what, ok] of lines) console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`);
  console.log(`=== bang-tally-diff --self: ${lines.every(l => l[1]) ? 'ALL PASS' : 'FAILURES above'} ===`);
  return lines.every(l => l[1]);
}

const table = (head: string[], body: string[][]) => { const w = head.map((h, i) => Math.max(h.length, ...body.map(r => r[i].length))); return [head, ...body].map(r => r.map((c, i) => c.padEnd(w[i])).join('  ').trimEnd()); };
const stop = (code: number, why: string): never => { console.error(`bang-tally-diff: ${why}`); process.exit(code); };
const seedsOf = (s: string): number[] => s.split(',').flatMap(p => { const m = p.match(/^(\d+)(?:-(\d+))?$/); if (!m || +(m[2] ?? m[1]) < +m[1]) return stop(2, `bad seeds '${s}'`); return Array.from({ length: +(m[2] ?? m[1]) - +m[1] + 1 }, (_, i) => +m[1] + i); });
const armOf = (r: DoorRow) => r.arm === 'open' ? 'the plain door' : `the ${r.arm}`, doorOf = (r: DoorRow) => `${r.door} (${r.room})`;
const oneIn = (p: number) => `1 in ${p > 0.1 ? (1 / p).toFixed(1) : Math.round(1 / p)}`;
const split = (r: DoorRow) => `worse in ${r.worse.length} entries and better in ${r.better.length} of ${r.n} (rooms held ${r.heldB} → ${r.heldA}, team lost ${r.lostB} → ${r.lostA}; chance ${oneIn(r.chance)})`;
/** what a row's staging lets a reader conclude, said beside its verdict */
const staging = (r: DoorRow) => r.restaged ? ` · STAGED DIFFERENTLY in ${r.restaged} of ${r.n}: the tally stood the team or Sam on other floor after the change, or took other men off it first, so this may be the tally's staging and not the walk to the door`
  : r.unsaid ? ` · staging not written in ${r.unsaid} of ${r.n} (a tally from before it was): this may be the tally's staging and not the walk to the door` : ` · staged alike in all ${r.n}`;

if (import.meta.main) {
  const argv = process.argv.slice(2); if (argv[0] === '--self') process.exit(self() ? 0 : 1);
  const flag = (k: string) => { const i = argv.indexOf(`--${k}`); if (i < 0) return undefined; const v = argv[i + 1]; argv.splice(i, 2); return v; };
  const before = flag('before'), after = flag('after') ?? HERE, seeds = flag('seeds') ?? '1-24', more = flag('more') ?? '25-48', jobs = flag('jobs') ?? '4', keep = flag('keep'), scn = argv.shift();
  if (!scn || !before || argv.length) stop(2, 'usage: bun run tools/qa/bang-tally-diff.ts <scenario> --before <checkout | tally.json[,…]> [--after <checkout | tally.json[,…]>] [--seeds 1-24] [--more 25-48 | none] [--jobs 4] [--keep dir]   ·   --self');
  const SCN = scn!, BREAK = process.env.TALLYDIFF_BREAK === '1', S1 = seedsOf(seeds), S2 = more === 'none' ? [] : seedsOf(more);
  if (S2.some(s => S1.includes(s))) stop(2, `--seeds ${seeds} and --more ${more} share seeds: the second block is other seeds`);
  if (keep) mkdirSync(keep, { recursive: true });
  const tmp = keep ?? process.env.TMPDIR ?? '/tmp', mine: string[] = [];   // (files of this run's own, gone when it ends unless kept)
  let child: ReturnType<typeof Bun.spawn> | null = null; const stoodAs: string[] = [];   // (the stagings the after tree was stood as, for the trace's line)
  const tidy = () => { if (child) { Bun.spawnSync(['pkill', '-TERM', '-P', String(child.pid)]); child.kill(); } for (const f of mine) if (existsSync(f)) unlinkSync(f); };
  process.on('exit', tidy); for (const s of ['SIGINT', 'SIGTERM'] as const) process.on(s, () => process.exit(130));

  /** one tally's JSON, held to what it says of itself: the page's stand, no knob set, crated only if asked, every run it was asked given back, none thrown */
  const whole = (file: string, crated: boolean): { seeds: number[]; results: Entry[]; stand: boolean } => {
    const name = basename(file); let J: { seeds: number[]; set?: object; stand?: string; crated?: boolean; asked?: number; results: Entry[] };
    try { J = JSON.parse(readFileSync(file, 'utf8')); } catch { return stop(3, `${name} is not a tally's JSON (cut short?)`); }
    if (!Array.isArray(J.seeds) || !Array.isArray(J.results)) stop(3, `${name} is not a tally's JSON`);
    if (J.stand !== undefined && J.stand !== 'shipped') stop(2, `${name} was played on the ${J.stand} stand: this reads the page's (TPS_PROFILE=shipped)`);
    if (J.set && Object.keys(J.set).length) stop(2, `${name} was played with knobs set (--set): this reads the game as it is`);
    if (!!J.crated !== crated) stop(2, `${name} was played ${J.crated ? 'with' : 'without'} TALLYDIFF_BREAK=1's crates and is asked for ${crated ? 'with' : 'without'}`);
    const bad = J.results.find(threw); if (bad) stop(3, `${name}: ${J.results.filter(threw).length} runs threw and were not played (${bad.door}, seed ${bad.seed}: ${bad.note.slice(0, 160)})`);
    if (J.asked !== undefined && J.results.length !== J.asked) stop(3, `${name} is not whole: ${J.results.length} of the ${J.asked} runs asked (a worker died?)`);
    const per = new Map<string, number>(); for (const r of J.results) { const k = `${r.scn} · ${r.door} (${r.room}), ${r.arm}`; per.set(k, (per.get(k) ?? 0) + 1); }
    const short = [...per].filter(([, n]) => n !== J.seeds.length);
    if (short.length) stop(3, `${name} is not whole (a worker died?): ${short.slice(0, 3).map(([k, n]) => `${k} has ${n} of ${J.seeds.length} seeds`).join(' · ')}${short.length > 3 ? ` · and ${short.length - 3} more` : ''}`);
    return { seeds: J.seeds, results: J.results.filter(r => r.scn === SCN && ARMS.includes(r.arm)), stand: J.stand !== undefined };
  };
  interface Side { label: string; tally: string | null; holds: (block: number[]) => boolean; read: (block: string, as: Entry[]) => Promise<Entry[]> }
  /** one side: a tally already played (its JSON), or a checkout where a block is played now on the page's stand, stood as `as` stood it */
  const side = (what: string, name: 'before' | 'after'): Side => {
    const parts = what.split(','), crated = BREAK && name === 'after';
    if (parts.every(p => existsSync(p) && statSync(p).isFile())) {
      if (crated) stop(2, 'TALLYDIFF_BREAK=1 plays the AFTER tree with crates laid: --after must be a checkout, not a tally already played');
      const files = parts.map(p => ({ name: basename(p), ...whole(p, false) })), have = new Map<number, string>();
      for (const f of files) for (const s of f.seeds) { if (have.has(s)) stop(2, `seed ${s} is in ${have.get(s)} and in ${f.name}: give each seed once`); have.set(s, f.name); }
      const all = files.flatMap(f => f.results), trust = files.filter(f => !f.stand).map(f => f.name);
      return { label: `the tall${files.length > 1 ? 'ies' : 'y'} ${files.map(f => f.name).join(' + ')}${trust.length ? ` (stand not written in ${trust.length === files.length ? 'them' : trust.join(', ')}: taken as the page's on your word)` : ''}`, tally: null,
        holds: block => { const n = block.filter(s => have.has(s)).length; if (n && n < block.length) stop(2, `the tallies given for --${name} hold ${n} of the ${block.length} seeds of a block: give all of a block or none of it`); return n > 0; },
        read: async block => { const S = new Set(seedsOf(block)); return all.filter(r => S.has(r.seed)); } }; }
    if (parts.length !== 1 || !existsSync(resolve(what, 'tools/qa/bang-tally.ts'))) stop(2, `--${name}: '${what}' is neither a checkout with tools/qa/bang-tally.ts nor tally JSON files`);
    const dir = resolve(what), git = (...a: string[]) => new TextDecoder().decode(Bun.spawnSync(['git', '-C', dir, ...a]).stdout).trim();
    const tally = readFileSync(resolve(dir, 'tools/qa/bang-tally.ts'), 'utf8'), head = git('rev-parse', '--short', 'HEAD'), own = relative(dir, tmp);
    // the tree as it stands: its commit, its tracked changes, and every untracked file's size and time (this run's own kept files apart)
    const loose = () => git('ls-files', '-o', '--exclude-standard').split('\n').filter(f => f && !(own && !own.startsWith('..') && f.startsWith(own))).map(f => { const q = resolve(dir, f), s = existsSync(q) ? statSync(q) : null; return `${f}:${s?.size}:${s?.mtimeMs}`; });
    const stamp = () => `${git('rev-parse', 'HEAD')} ${Bun.hash(git('diff', 'HEAD') + loose().join('|'))}`, was = stamp(), dirty = !!git('diff', 'HEAD') || loose().length > 0;
    if (crated && !tally.includes('TALLYDIFF_BREAK')) stop(2, `TALLYDIFF_BREAK=1: the tally of the --after checkout is from before the crate and would lay none`);
    return { label: `${head ? `commit ${head}${dirty ? ' and its uncommitted changes' : ''}` : 'a folder that is no git checkout (nothing can tell if it changes while it is played)'}${crated ? ', crated' : ''}`, tally, holds: () => true, read: async (block, as) => {
      const file = resolve(tmp, `bang-tally-diff-${SCN}-${name}${crated ? '-crated' : ''}-${block}${keep ? '' : `-${process.pid}`}.json`), asFile = resolve(tmp, `bang-tally-diff-${SCN}-${name}-stood-as-${block}${keep ? '' : `-${process.pid}`}.json`);
      if (!keep) mine.push(file);
      const standAs = as.some(r => r.at) && tally.includes('--stand-as'); if (standAs) { writeFileSync(asFile, JSON.stringify({ results: as })); if (!keep) mine.push(asFile); stoodAs.push(asFile); }
      console.error(`bang-tally-diff: playing ${SCN}, seeds ${block}, in the --${name} checkout${standAs ? ', stood as the other tree stood it' : ''}${crated ? ' (TALLYDIFF_BREAK=1: a crate on the way in to each door played)' : ''} …`);
      child = Bun.spawn(['bun', 'run', 'tools/qa/bang-tally.ts', '--scn', SCN, '--seeds', block, '--arms', ARMS.join(','), '--jobs', jobs, '--json', file, ...(standAs ? ['--stand-as', asFile] : [])],
        { cwd: dir, env: { ...process.env, TPS_PROFILE: 'shipped', TALLYDIFF_BREAK: crated ? '1' : undefined }, stdout: 'ignore', stderr: 'pipe' });
      const err = await new Response(child.stderr as ReadableStream).text(), code = await child.exited; child = null;
      if (code !== 0 || !existsSync(file)) stop(3, `the tally in the --${name} checkout ended with ${code}: ${err.trim().split('\n').slice(-2).join(' | ').slice(0, 300)}`);
      if (stamp() !== was) stop(3, `the --${name} checkout changed while it was being played: what was read is of no one tree (play a worktree nobody edits)`);
      const W = whole(file, crated); if (W.seeds.length !== seedsOf(block).length) stop(3, `the tally in the --${name} checkout played ${W.seeds.length} seeds of ${seedsOf(block).length}`);
      return W.results; } };
  };
  const Bf = side(before!, 'before'), Af = side(after, 'after');
  if (!Bf.holds(S1) || !Af.holds(S1)) stop(2, `the tally given for ${Bf.holds(S1) ? '--after' : '--before'} holds no seed of ${seeds}`);
  const show = (rows: DoorRow[], over: string, final: boolean) => { console.log(`  seeds ${over}:`);
    for (const l of table(['door (room)', 'arm', 'common', 'the same', 'staged alike', 'rooms held', 'team lost', 'worse, better', 'chance', ''],
      rows.map(r => [doorOf(r), r.arm === 'open' ? 'plain' : r.arm, String(r.n), String(r.same), r.unsaid ? (r.unsaid === r.n ? 'not written' : `${r.alike} (${r.unsaid} not written)`) : String(r.alike),
        `${r.heldB} → ${r.heldA}`, `${r.lostB} → ${r.lostA} of ${r.team}`, `${r.worse.length}, ${r.better.length}`, r.few || !(r.worse.length + r.better.length) ? '' : oneIn(r.chance),
        r.gone ? `NAMED (${r.gone} cannot be made now)` : !r.inB || !r.inA ? `in the ${r.inB ? 'before' : 'after'} tree only` : r.few ? `too few (under ${FEW} common entries)` : final && r.chance <= NAMED_P ? 'NAMED' : r.chance <= SUSPECT_P ? 'suspect' : '']))) console.log(`    ${l}`); };
  const B = await Bf.read(seeds, []), A = await Af.read(seeds, B);
  let rows = compare(B, A, SCN), over = seeds, final = !S2.length;
  if (!rows.length) stop(2, `no entry of '${SCN}' in either tally`);
  console.log(`bang-tally-diff · ${SCN} · the page's stand · it declares, it gates nothing${BREAK ? ' · TALLYDIFF_BREAK=1' : ''}`);
  console.log(`  before: ${Bf.label}`); console.log(`  after:  ${Af.label}`);
  if (Bf.tally !== null && Af.tally !== null && Bf.tally !== Af.tally) console.log('  (the two trees play different copies of bang-tally.ts: a difference may be the tally\'s own)');
  show(rows, over, final);
  const first = rows.filter(r => r.chance <= SUSPECT_P);
  if (!final && (first.length || rows.some(r => r.few && r.n))) {
    for (const r of first) console.log(`- suspect on seeds ${seeds} — ${doorOf(r)}, ${armOf(r)}: ${split(r)}`);
    if (!first.length) console.log(`  (an arm is too thin to say on seeds ${seeds}: reading seeds ${more} too)`);
    if (Bf.holds(S2) && Af.holds(S2)) { const B2 = await Bf.read(more, []); B.push(...B2); A.push(...await Af.read(more, B2)); rows = compare(B, A, SCN); over = `${seeds} and ${more}`; final = true; show(rows, over, final); }
    else console.log(`  (the tally given for ${Bf.holds(S2) ? '--after' : '--before'} holds no seed of ${more}: not read further)`);
  } else final = true;
  const named = rows.filter(r => r.gone || (final && r.chance <= NAMED_P)), open = final ? [] : rows.filter(r => r.chance <= SUSPECT_P && !r.gone);
  const lone = rows.filter(r => !r.inB !== !r.inA), few = rows.filter(r => r.few && !lone.includes(r)), read = rows.filter(r => !r.few);
  const doors = (rs: DoorRow[]) => [...new Set(rs.map(doorOf))];
  /** one entry to read in both trees: the first seed that went the way the door did */
  /** one entry to read in both trees: the first seed that went the way the door did, replayed as it was counted */
  const trace = (r: DoorRow) => { const s = (r.worse.length >= r.better.length ? r.worse : r.better)[0]; if (s === undefined) return '';
    const cmd = `bun run tools/qa/bang-tally.ts trace ${SCN}:${r.door}:${r.arm}:${s}:${r.room}`, as = stoodAs.length ? (keep ? ` --stand-as ${stoodAs.map(f => relative(process.cwd(), f)).join(',')}` : ' --stand-as <the before tally\'s JSON: --keep keeps it>') : '';
    return `\n    read one entry in both trees (TPS_PROFILE=shipped in front in a tree from before one stand) — before: ${cmd} · after: ${BREAK ? 'TALLYDIFF_BREAK=1 ' : ''}${cmd}${as}`; };
  for (const r of named) console.log(`- NAMED — ${doorOf(r)}, ${armOf(r)}, over seeds ${over}: ${r.gone ? `${r.gone} entries that were made before cannot be now (no floor outside the door for the team, or the order refused)${r.goneMoved ? `, ${r.goneMoved} of them staged on other floor than before: this may be where the tally stands the team or Sam` : ''}${r.few ? '' : '; the rest '}` : ''}${r.few ? '' : split(r) + staging(r) + trace(r)}`);
  for (const r of open) console.log(`- SUSPECT, NOT READ FURTHER — ${doorOf(r)}, ${armOf(r)}, over seeds ${over}: ${split(r)}${staging(r)}`);
  if (over !== seeds) for (const r of first) { const now = rows.find(x => x.door === r.door && x.room === r.room && x.arm === r.arm)!; if (!named.includes(now)) console.log(`  not borne out over seeds ${over} — ${doorOf(now)}, ${armOf(now)}: ${split(now)}`); }
  for (const r of rows) if (r.come) console.log(`  ${doorOf(r)}, ${armOf(r)}: ${r.come} entries that could not be made before can be now`);
  for (const r of lone) console.log(`- IN ONE TREE ONLY — ${doorOf(r)}, ${armOf(r)}: ${r.inB} results before, ${r.inA} after (a door or a room renamed, added or gone: nothing to compare)`);
  if (few.length) console.log(`  too few to say: ${doors(few).join(' · ')}`);
  const nd = doors(named).length, no = doors(open).length;
  console.log(nd ? `=== bang-tally-diff: ${nd} door${nd === 1 ? '' : 's'} named — say why in the step's notes, or mend it ===`
    : no ? `=== bang-tally-diff: NO READING — ${no} door${no === 1 ? '' : 's'} suspect and not read further, neither named nor cleared: play a second block of seeds in both trees ===`
    : lone.length ? `=== bang-tally-diff: NO READING — ${doors(lone).length} door${doors(lone).length === 1 ? '' : 's'} in one tree only ===`
    : !read.length ? `=== bang-tally-diff: NO READING — no door arm has ${FEW} entries both trees played ===`
    : `=== bang-tally-diff: no door named (${read.length} door arms read over seeds ${over}${few.length ? `, ${few.length} too few` : ''}) ===`);
  process.exit(nd ? 1 : no || lone.length || !read.length ? 3 : 0);
}
