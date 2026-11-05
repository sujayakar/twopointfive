// THE SMOKE LAB'S FLOOR (levels/smoke-lab.json from tools/levels/smoke-lab.ts; the page's lab is src/host/smokelab.ts, the dev server's /smokelab). Each place on
// the floor is there to show one thing smoke does, so this holds the floor to those things, through the same floor plan the renderer keeps (render/placeField.ts):
//   file      levels/smoke-lab.json is what its generator gives now
//   places    with every door shut the pantry and the shed are shut rooms of their own; the kitchen leaks to the yard by its window hole; the tunnel is open at
//             its mouth; with the door between them open the kitchen and the pantry are one place
//   roofs     the tunnel, the rooms and the shed are under a roof, the yard is under the sky
//   the page  /smokelab opens this floor, a levelFile on the address opens that level instead, and no mission plays the lab
// No GPU: nothing here says what smoke looks like on the floor.
//   bun run tools/qa/smokelab-check.ts                    exit 0 = every line passed, 1 = one failed
//   SMOKELAB_BREAK=1 bun run tools/qa/smokelab-check.ts   the shed's door is left open: must FAIL
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { placeMask, PLACE_W, PLACE_H, PLACE_RES } = await import(`${ROOT}/src/scene/placePlan.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
const BRK = process.env.SMOKELAB_BREAK === '1'; let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== smokelab: the smoke lab's floor${BRK ? ' (the shed\'s door left open: must fail)' : ''} ===`);

{ const r = Bun.spawnSync(['bun', 'run', `${ROOT}/tools/levels/export.ts`, 'smoke-lab', '--check'], { cwd: ROOT, stdout: 'pipe', stderr: 'pipe' });
  check(r.exitCode === 0, 'levels/smoke-lab.json is what tools/levels/smoke-lab.ts gives now', `export --check exit ${r.exitCode}`); }

const l = console.log, w = console.warn; console.log = console.warn = () => { };
const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/smoke-lab.json`, 'utf8'))), seed: 1 }); for (let i = 0; i < 10; i++) W.step(1 / 60);
console.log = l; console.warn = w;
const g = W.game, world = g.engine.world, N = PLACE_W * PLACE_H, ox = WORLD.rcMin[0], oz = WORLD.rcMin[2];
const GAPS = g.doors.list.map((d: any) => { const [hx, hz] = d.def.hinge, a = d.def.closedDir, wd = d.def.width; return [hx, hz, hx + Math.cos(a) * wd, hz + Math.sin(a) * wd]; });
const hang = (name: string, a: number) => { const d = g.doors.byName(name); d.latched = a === 0; d.vel = 0; d.angle = a; };
const cell = (x: number, z: number) => Math.floor((z - oz) * PLACE_RES) * PLACE_W + Math.floor((x - ox) * PLACE_RES);
/** the floor plan as it stands, and which piece of connected floor each free cell is in (a doorway's cells join what is either side of them only while its leaf is off them) */
const plan = () => { for (let i = 0; i < 2; i++) W.step(1 / 60); const m = placeMask(world.statics, world.dynamics, undefined, GAPS) as Uint8Array;
  const comp = new Int32Array(N).fill(-1); let nc = 0;
  for (let k0 = 0; k0 < N; k0++) { if (m[k0] === 255 || comp[k0] >= 0) continue; const st = [k0]; comp[k0] = nc;
    while (st.length) { const c = st.pop()!, i = c % PLACE_W, j = (c - i) / PLACE_W;
      for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) { const x = i + di, z = j + dj; if (x < 0 || z < 0 || x >= PLACE_W || z >= PLACE_H) continue; const t = z * PLACE_W + x; if (m[t] !== 255 && comp[t] < 0) { comp[t] = nc; st.push(t); } } }
    nc++; }
  return { m, comp, nc }; };
const AT: Record<string, [number, number]> = { yard: [18, 14], tunnel: [7.3, 10], kitchen: [14.6, 7.6], pantry: [19.6, 7.6], shed: [30, 13.4] };   // a point well inside each (tools/levels/smoke-lab.ts)
const names = Object.keys(AT);

// every door shut (the break: the shed's stands open)
for (const d of g.doors.list) hang(d.def.name, 0); if (BRK) hang('shed', 1.5);
{ const { m, comp } = plan(), c = Object.fromEntries(names.map(n => [n, comp[cell(...AT[n])]]));
  const said = names.map(n => `${n} ${c[n]}`).join(' · ');
  check(names.every(n => c[n] >= 0), 'the yard, the tunnel, the kitchen, the pantry and the shed each have floor at their middle', said);
  check(c.shed !== c.yard && c.shed !== c.pantry && c.shed !== c.kitchen, 'with its door shut the shed is a shut room: nothing joins its floor to any other', said);
  check(c.pantry !== c.yard && c.pantry !== c.kitchen, 'with its two doors shut the pantry is a shut room', said);
  check(c.tunnel === c.yard, 'the tunnel is open at its mouth: its floor is the yard\'s, door shut or not', said);
  // the window hole is in the wall above a sill, so the floor plan (which asks for a wall from floor to ceiling) leaves that cell open: the kitchen is the yard's
  check(c.kitchen === c.yard, 'the kitchen leaks: its window hole joins it to the yard with both its doors shut', said);
  const roof = (n: string) => (m[cell(...AT[n])] & 4) === 0, under = names.filter(roof);
  check(!roof('yard') && ['tunnel', 'kitchen', 'pantry', 'shed'].every(roof), 'the tunnel, the two rooms and the shed are under a roof and the yard is under the sky', `under a roof: ${under.join(', ')}`); }

// the door between the rooms open
{ hang('between', 1.5); const { comp } = plan(); check(comp[cell(...AT.kitchen)] === comp[cell(...AT.pantry)], 'with the door between them open the kitchen and the pantry are one place'); hang('between', 0); }

// who is on the floor, and how the page gets to it
{ const sp = g.player.char.pos, { m } = plan(), k = cell(sp[0], sp[2]);
  check(m[k] !== 255 && (m[k] & 4) !== 0 && g.guards.length === 0, 'Sam starts on open ground in the yard and nobody else is on the floor', `start (${sp[0].toFixed(1)}, ${sp[2].toFixed(1)}) · ${g.guards.length} guards`);
  const serve = readFileSync(`${ROOT}/tools/serve.ts`, 'utf8'), row = serve.split('\n').find(s => s.includes('"/smokelab"')) ?? '';
  check(/levelFile=smoke-lab\.json&solo&hud&smokelab/.test(row) && /has\("levelFile"\) \? "\/\?look&hud&smokelab"/.test(row) && /&weather=clear/.test(row), 'the dev server\'s /smokelab opens this floor in dry weather, and a levelFile on the address opens that level instead');
  const lists = ['src/scene/scenarioList.ts', 'src/scene/picks.ts'].filter(f => readFileSync(`${ROOT}/${f}`, 'utf8').includes('smoke-lab'));
  check(lists.length === 0, 'no mission list names the lab\'s floor', lists.join(', ')); }

console.log(red ? '=== smokelab: FAILED ===' : '=== smokelab: ALL PASS ===');
process.exit(red ? 1 : 0);
