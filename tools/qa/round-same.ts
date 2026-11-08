// EVERY ROUND THING IS THE SAME CYLINDER AFTER THE FILES SAID IT THEMSELVES (tools/levels/round-convert.ts). Each level is stood on the shipped stand by the tree before
// the conversion and by this one. The round boxes must agree one to one, to the bit: centre, half size, flags, paint, stuff, does, marks. Every other box, light,
// door and prop must agree in order, once the specks the old stand-up left are out of the base's list and every box index the level holds follows.
//   bun run tools/qa/round-same.ts --base <commit>     stands the base in a worktree of its own, this tree here, compares; exit 1 on any difference
//   ROUNDSAME_BREAK=1 ...                              one round thing's radius is moved by the last bit: must fail
// Both sides read the base's list of level files, so a level the tree has cut since is said and left out, and a level only this tree has is said too.
import { execFileSync } from 'node:child_process';
import { copyFileSync, existsSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

const SELF = resolve(import.meta.path), TREE = resolve(import.meta.dir, '../..');
const argAfter = (word: string) => { const at = process.argv.indexOf(word); return at >= 0 ? process.argv[at + 1] : undefined; };
type Dump = Record<string, { boxes: Record<string, unknown>[]; rest: Record<string, unknown>; rounds: number; specks: number }>;
const SPECK = 1e-2;

/** this tree's stand of every level file, written to `out` */
async function dump(out: string) {
  const { standUp, ROOT } = await import(join(TREE, 'tools/qa/headless.ts'));
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
  const { kitViewSetLevel } = await import(`${ROOT}/src/host/kitViewSet.ts`);
  const hush = console.log; console.log = () => { };
  const all: Dump = {};
  for (const file of readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).sort()) {
    const data0 = parseLevelData(readFileSync(`${ROOT}/levels/${file}`, 'utf8'));
    // (a kit view's set stands up as a level of its own: asked as the page stands it)
    const sets = (data0.scenarios?.kitView?.ways ?? []).flatMap((w: { set?: string }, i: number) => w.set ? [i] : []) as number[];
    for (const [name, data] of [[file, data0], ...sets.map(i => [`${file} set ${i}`, kitViewSetLevel(data0, i)])] as [string, any][]) {
    const W = await standUp({ level: loadLevel(data), seed: 1 }), L = W.game.level;
    const n = L.boxes.length, isSpeck = (b: any) => !!b.was && !b.round && !!(b.flags & BoxFlag.NoPrimary) && b.h[0] === SPECK;
    // a box's place in the list with the specks out of it, and where a speck's index goes: its round's first box
    const place: number[] = []; let at = 0, lastRound = -1;
    for (let i = 0; i < n; i++) { if (isSpeck(L.boxes[i])) { place[i] = lastRound; continue; } if (L.boxes[i].round) lastRound = at; place[i] = at++; }
    const fileIndex = new Map<string, number>(data.boxes.map((b: { id: string }, i: number) => [b.id, i] as [string, number]));
    const boxes = L.boxes.filter((b: any) => !isSpeck(b)).map((b: any) => { const { was: _was, mkey: _mkey, ...rest } = b; return { ...rest, of: b.of === undefined ? undefined : place[b.of] }; });
    const movedOnes = (v: unknown): unknown => {   // (a kit view's hide list names boxes by id: as places in the list, each once)
      if (Array.isArray(v)) { const here = (x: unknown) => typeof x === 'string' && /^box_\d+$/.test(x); return v.some(here) ? [...new Set(v.map(x => here(x) ? place[fileIndex.get(x as string)!] : x))] : v.map(movedOnes); }
      if (v && typeof v === 'object') return Object.fromEntries(Object.entries(v).map(([k, x]) => [k, movedOnes(x)]));
      return v;
    };
    const rest: Record<string, unknown> = { ...L, boxes: undefined,
      switchables: L.switchables.map((s: any) => ({ ...s, boxes: s.boxes.map((b: number) => place[b]) })),
      lights: L.lights.map((l: any) => ({ ...l, fixtureBox: l.fixtureBox === undefined ? undefined : place[l.fixtureBox] })),
      breaker: L.breaker ? { ...L.breaker, boxes: L.breaker.boxes.map((b: number) => place[b]) } : undefined,
      beacons: L.beacons?.map((b: any) => ({ ...b, box: place[b.box] })),
      scenarios: movedOnes(L.scenarios ? { ...L.scenarios, kitView: L.scenarios.kitView ? { ...L.scenarios.kitView, sets: undefined } : undefined } : undefined) };   // (a set's boxes are asked as that set stood up)
    all[name] = JSON.parse(JSON.stringify({ boxes, rest, rounds: boxes.filter((b: any) => b.round).length, specks: n - boxes.length }));
    }
  }
  console.log = hush;
  writeFileSync(out, JSON.stringify(all));
}

if (argAfter('--dump')) { await dump(argAfter('--dump')!); process.exit(0); }

const base = argAfter('--base');
if (!base) { console.error('usage: bun run tools/qa/round-same.ts --base <commit>'); process.exit(2); }
const at = mkdtempSync(join(tmpdir(), 'round-same-')), baseTree = join(at, 'base'), mine = join(at, 'mine.json'), theirs = join(at, 'base.json');
const env = { ...process.env, TPS_PROFILE: 'shipped' };
try {
  execFileSync('git', ['-C', TREE, 'worktree', 'add', '--detach', baseTree, base], { stdio: 'ignore' });
  copyFileSync(SELF, join(baseTree, 'tools/qa/round-same.ts'));
  execFileSync('bun', ['run', 'tools/qa/round-same.ts', '--dump', theirs], { cwd: baseTree, env, stdio: ['ignore', 'ignore', 'inherit'] });
  execFileSync('bun', ['run', SELF, '--dump', mine], { cwd: TREE, env, stdio: ['ignore', 'ignore', 'inherit'] });
  const A: Dump = JSON.parse(readFileSync(theirs, 'utf8')), B: Dump = JSON.parse(readFileSync(mine, 'utf8'));
  if (process.env.ROUNDSAME_BREAK === '1') { const b = Object.values(B).flatMap(l => l.boxes).find(x => x.round) as { h: number[] }; const f = new Float64Array([b.h[0]]), u = new BigUint64Array(f.buffer); u[0] += 1n; b.h[0] = f[0]; }
  let rounds = 0, boxes = 0, specks = 0; const off: string[] = [];
  for (const file of Object.keys(A)) {
    const a = A[file], b = B[file]; if (!b) { console.log(`  ${file}: this tree has cut it`); continue; }
    rounds += b.rounds; boxes += b.boxes.length; specks += a.specks;
    if (b.specks) off.push(`${file}: this tree still has ${b.specks} specks`);
    if (a.specks !== a.rounds * 3) off.push(`${file}: the base has ${a.rounds} rounds and ${a.specks} specks`);
    if (a.boxes.length !== b.boxes.length) off.push(`${file}: ${a.boxes.length} boxes in the base with the specks out, ${b.boxes.length} here`);
    for (let i = 0; i < Math.min(a.boxes.length, b.boxes.length); i++) if (JSON.stringify(a.boxes[i]) !== JSON.stringify(b.boxes[i])) { off.push(`${file} box ${i}${b.boxes[i].round ? ' (round)' : ''}:\n    base ${JSON.stringify(a.boxes[i]).slice(0, 300)}\n    here ${JSON.stringify(b.boxes[i]).slice(0, 300)}`); break; }
    for (const k of new Set([...Object.keys(a.rest), ...Object.keys(b.rest)])) if (JSON.stringify(a.rest[k]) !== JSON.stringify(b.rest[k])) off.push(`${file} ${k}: differs`);
  }
  for (const file of Object.keys(B)) if (!A[file]) console.log(`  ${file}: only this tree has it`);
  for (const o of off.slice(0, 30)) console.log(`  ${o}`);
  console.log(`${off.length ? 'FAIL' : 'PASS'} round-same: ${Object.keys(A).length} level files, ${rounds} round things, ${boxes} boxes, ${specks} specks gone from the base's list, ${off.length} differences`);
  process.exitCode = off.length ? 1 : 0;
} finally {
  if (existsSync(baseTree)) execFileSync('git', ['-C', TREE, 'worktree', 'remove', '--force', baseTree], { stdio: 'ignore' });
  rmSync(at, { recursive: true, force: true });
}
