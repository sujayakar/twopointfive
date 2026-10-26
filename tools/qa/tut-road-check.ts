// TUT-ROAD-CHECK: the tutorial's first mission (levels/tut-road.json, its fights in levels/tut-road.script.ts) played by its scripted player
// (tools/qa/lessons/tut-road.ts, through the input only) on the tutorial's frame (game/rails.ts: raised here on the frozen stand), eight ways a seed:
//   · as the lines say                      → every step goes by and the mission ends
//   · the sandbags walked STANDING          → the player goes down at that line (the gun's traverse is over a crouched head and through a standing chest)
//   · out onto THE OPEN SNOW in front of the gun, past the end of the roadblock's wall → the player goes down at that line (the gunner is the game's own
//     from the moment a man walks out in front of him)
//   · six seconds on the road BESIDE THE TRUCK, in the sentry's line, before anything else → every step still goes by (he holds his fire for a man there)
//   · a run at the hut's door, then a long burst into the wall BESIDE IT, not one round in the leaf → the door holds the run, the line goes by on the
//     burst, the script opens the door as the reload's line comes up, and every step still goes by (the way on never hangs on the door splintering)
//   · up the road INTO THE GATEWAY'S MOUTH on the first line, eight seconds between the sentry and the truck he shoots; then at a sprint to the wall and on
//     round its end into the gateway → every step still goes by (he holds his fire for a man anywhere between him and the truck)
//   · a long burst at the hut's door FROM THE SIDE, down the line that runs through the leaf and the boards to where the back room's man will stand, and on
//     past the leaf → he is still up when his own line comes (he waits behind a wall until it), and every step still goes by
//   · the two CHECKPOINTS                   → down at the sandbags, Enter puts him back at the hut (its door out, the rifle full, two men down, as the
//     checkpoint was kept); the other is put back by the harness a step after it is kept; each beat played again from its first line → every step goes by
// The two lessons that are about not getting shot are only lessons if they can be failed: a mission where the wrong way also works teaches nothing.
//   bun run tools/qa/tut-road-check.ts [--seeds 1,2]        and again with TPS_PROFILE=shipped in front (the stand the page opens on)
//   TUT_ROAD_BREAK=1 …   the standing way is played crouched, as the line says: nobody goes down there, and the check must be RED
// EXIT: 0 = all eight as said on every seed · 1 = a fault, each a line.
// Then THE BOARDS: the hut's plank wall is a thing a rifle round goes through (game/wallpen.ts thinThing) and the roadblock's wall is not, where that rule
// is on; and THE PAGE'S KIT: on its page he carries the rifle and nothing to throw, set or fire from under it.
// Headless: no GPU, nothing here saw a picture or heard a round.
import { runLesson } from './lesson-run';

process.env.TPS_RAILS ??= '1';       // (the level has a lesson only on the frame: off on the frozen stand unless raised)
process.env.TPS_DOORSHOT ??= '1';    // (the hut's door is a door shot out: held off on the frozen stand unless raised)
process.env.TPS_WALLPEN ??= '1';     // (and the plank wall is a round through wood: held off there too)

const argv = process.argv.slice(2), flag = (f: string) => { const i = argv.indexOf(f); return i >= 0 ? argv[i + 1] : undefined; };
const seeds = (flag('--seeds') ?? '1,2').split(',').map(Number).filter(n => Number.isFinite(n));
const BROKEN = process.env.TUT_ROAD_BREAK === '1';
/** `downAt`: the step the player must go down in (the last step that went by is the one before it) */
const ARMS: { word: string; say: string; downAt?: string; after?: string }[] = [
  { word: '', say: 'as the lines say' },
  { word: BROKEN ? '' : 'stand', say: 'the sandbags walked standing', downAt: 'low', after: 'crouch' },
  { word: 'field', say: 'out onto the open snow in front of the gun', downAt: 'cover', after: 'low' },
  { word: 'east', say: 'six seconds on the road beside the truck, in the sentry\'s line' },
  { word: 'jamb', say: 'a run at the hut\'s door, then a long burst into the wall beside it: the line goes by and the door is opened for him' },
  { word: 'mouth', say: 'up the road into the gateway\'s mouth on the first line, and again at a sprint round the wall\'s end' },
  { word: 'slant', say: 'a long burst at the hut\'s door from the side, down the line to the back room: its man is still up for his own line' },
  { word: 'again', say: 'down at the sandbags and back by Enter, the other checkpoint put back by hand, each beat played again' },
];
const faults: string[] = [];
for (const seed of seeds) for (const A of ARMS) {
  process.env.TUT_ROAD_ARM = A.word;
  const r = await runLesson('tut-road', { seed });
  const last = r.steps[r.steps.length - 1]?.id ?? '(none)', went = /the player went down/.test(r.why) && last === A.after;
  const ok = A.downAt ? r.code === 1 && went : r.code === 0;
  console.log(`  ${ok ? 'PASS' : 'FAIL'}  seed ${seed} · ${A.say}: ${A.downAt ? (went ? `he went down at '${A.downAt}'` : `he did NOT go down at '${A.downAt}' (after '${last}': ${r.why || 'the run went through'})`) : r.code === 0 ? `all ${r.steps.length} steps, ${r.secs.toFixed(0)} s` : r.why}`);
  if (!ok) faults.push(`seed ${seed}, ${A.say}: ${A.downAt ? `the player was not put down at '${A.downAt}'` : r.why}`);
}
delete process.env.TUT_ROAD_ARM;

// THE BOARDS AND THE KIT, on the level as its page stands it
{ const { readFileSync } = await import('node:fs'), { resolve } = await import('node:path'), ROOT = resolve(import.meta.dir, '../..');
  const { standUp } = await import('./headless');
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`), K = await import(`${ROOT}/src/game/kit.ts`), { OWNCHARGE } = await import(`${ROOT}/src/game/owncharge.ts`), { WALLPEN, thinThing } = await import(`${ROOT}/src/game/wallpen.ts`);
  const T = await import(`${ROOT}/tools/levels/tut-road.ts`);
  const log = console.log, warn = console.warn, err = console.error; console.log = console.warn = console.error = () => { };
  let held = '', charges = -1, why = '', boards = 'not found', wall = 'not found';
  try { K.kitIssuedPage(true); const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels/tut-road.json'), 'utf8'))), seed: 1 }), g = W.game;
    const r = startFromQuery(g, '?levelFile=tut-road.json&quiet'); why = r.why ?? ''; K.kitSoloIf(g, false);   // (main.ts, straight after the page's start: a quiet mission alone wears a card)
    for (let i = 0; i < 5; i++) W.step(1 / 60);
    const pl = g.player; held = `${pl.canisters}/${pl.flashbangs}/${pl.frags}/${pl.rifle.shells}/${pl.rifle.rings}`; charges = g.doorTools.charges;
    // a ray across each wall at chest height: what it meets first, and what that is to a round
    const across = (a: number[], d: number[], len: number) => { const h = g.col.raycast(a, d, len); if (!h) return 'nothing in the way'; const t = thinThing(g, h.index); return t ? `${t.stuff} ${(t.thick * 100).toFixed(0)} cm` : 'stops a round'; };
    boards = across([T.PLANKS.x - 1, 1.2, (T.PLANKS.z0 + T.PLANKS.z1) / 2], [1, 0, 0], 2);
    wall = across([T.COVER[0], 1.2, T.COVER_Z + 1], [0, 0, -1], 2);
  } catch (x) { why = String((x as Error).message); } finally { K.kitIssuedPage(false); console.log = log; console.warn = warn; console.error = err; }
  // (where his card is not the whole kit — the frozen stand — the launcher's lent sponge rounds are over it, and the door charges are the squad's count:
  //  the page deals neither)
  const lend = K.ISSUE.whole ? 0 : (await import(`${ROOT}/src/game/kittree.ts`)).KITTREE.lend;
  const kitOk = !why && held === `0/0/0/0/${lend}` && (!OWNCHARGE.on || charges === 0);
  console.log(`  ${kitOk ? 'PASS' : 'FAIL'}  on its page he carries the rifle and nothing to throw or set${lend ? ' (the lent sponge rounds are over his card on this stand)' : ''}: smoke/stun/frags/40 mm/sponge ${held}${OWNCHARGE.on ? `, door charges ${charges}` : ' (his own charges are off on this stand)'}${why ? ` (${why})` : ''}`);
  if (!kitOk) faults.push(`the page's kit is not the rifle alone (${held}${why ? `, ${why}` : ''})`);
  const wallOk = /^wood/.test(boards) && wall === 'stops a round';
  console.log(`  ${wallOk ? 'PASS' : 'FAIL'}  ${WALLPEN.on ? 'a rifle round goes through the hut\'s boards and stops in the roadblock\'s wall' : 'the hut\'s boards are wood thin enough for a round and the roadblock\'s wall is not (the rule itself is off here)'}: the boards ${boards}, the wall ${wall}`);
  if (!wallOk) faults.push(`the boards are '${boards}' and the roadblock's wall '${wall}' to a round`);
}
console.log(faults.length ? `=== tut-road-check: FAIL (${faults.length}) ===\n${faults.map(f => '  ' + f).join('\n')}` : `=== tut-road-check: ALL PASS (${seeds.length} seeds × ${ARMS.length}, the boards, and the page's kit) ===`);
process.exit(faults.length ? 1 : 0);
