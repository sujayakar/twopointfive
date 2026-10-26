// TUT-KIT-CHECK: the tutorial's kit mission (levels/tut-kit.json, its fights in levels/tut-kit.script.ts) played by its scripted player
// (tools/qa/lessons/tut-kit.ts, through the input only) on the tutorial's frame (game/rails.ts: raised here on the frozen stand), six ways a seed:
//   · as the lines say                      → every step goes by and the mission ends, the four at their truck until he calls them
//   · the sandbags walked STANDING          → the player goes down at that line (the gun's traverse is over a crouched head and through a standing chest)
//   · the gap crossed with NO SMOKE up      → the player goes down at that line (the gap is hosed until a smoke can has gone off between it and the gun)
//   · the command key BEFORE ITS LINE       → no ring comes up, nobody leaves the truck, and every step still goes by
//   · a TAP where the line says hold        → the tap does nothing (the squad is not sent to stack on the bunk room's door) and every step still goes by
//   · the yard's two SHOT BY HIM, no frag   → every step still goes by (the frag's line never hangs on the frag)
// The two lessons that are about not getting shot are only lessons if they can be failed: a mission where the wrong way also works teaches nothing.
//   bun run tools/qa/tut-kit-check.ts [--seeds 1,2]        and again with TPS_PROFILE=shipped in front (the stand the page opens on)
//   TUT_KIT_BREAK=1 …   the standing arm is played crouched, as the line says: nobody goes down there, and the check must be RED
// EXIT: 0 = all six as said on every seed, and the lines below · 1 = a fault, each a line.
// Then HIS KIT ON THE MISSION'S PAGE: one smoke, one stun grenade, one frag and one grenade on the launcher (never the sponge round, which no line here
// teaches; that count is the page's pouches rule, so the frozen stand is asked only for each can and the grenade), and the squad's bag holds what his lines and theirs spend: a door charge, a stun grenade, a frag and a wall charge.
// Then THE SHOP'S LAMP: the ground beside the shop's door is lit by that lamp and dark without it (the line that has him shoot it out says the dark hides him).
// Last, THE WALL: the wall the squad's charge is asked for is one a charge cuts (built at the shell's thickness it was refused as concrete).
// Headless: no GPU, no smoke sight lines, nothing here saw a picture or heard a round.
import { runLesson } from './lesson-run';

process.env.TPS_RAILS ??= '1';   // (the level has a lesson only on the frame: off on the frozen stand unless raised. The scripted player's own `raises` bring the rest)

const argv = process.argv.slice(2), flag = (f: string) => { const i = argv.indexOf(f); return i >= 0 ? argv[i + 1] : undefined; };
const seeds = (flag('--seeds') ?? '1,2').split(',').map(Number).filter(n => Number.isFinite(n));
const BROKEN = process.env.TUT_KIT_BREAK === '1';
/** `downAt`: the step the player must go down in (the last step that went by is the one before it) */
const ARMS: { word: string; say: string; downAt?: string; after?: string }[] = [
  { word: '', say: 'as the lines say' },
  { word: BROKEN ? '' : 'stand', say: 'the sandbags walked standing', downAt: 'low', after: 'arch' },
  { word: 'nosmoke', say: 'the gap crossed with no smoke up', downAt: 'smoke', after: 'low' },
  { word: 'early', say: 'the command key before its line: no ring, and nobody leaves the truck' },
  { word: 'tap', say: 'a tap where the line says hold: nobody is sent to the door' },
  { word: 'shoot', say: 'the yard\'s two shot by him and no frag ordered' },
];
const faults: string[] = [];
for (const seed of seeds) for (const A of ARMS) {
  process.env.TUT_KIT_ARM = A.word;
  const r = await runLesson('tut-kit', { seed });
  const last = r.steps[r.steps.length - 1]?.id ?? '(none)', went = /the player went down/.test(r.why) && last === A.after;
  const ok = A.downAt ? r.code === 1 && went : r.code === 0;
  console.log(`  ${ok ? 'PASS' : 'FAIL'}  seed ${seed} · ${A.say}: ${A.downAt ? (went ? `he went down at '${A.downAt}'` : `he did NOT go down at '${A.downAt}' (after '${last}': ${r.why || 'the run went through'})`) : r.code === 0 ? `all ${r.steps.length} steps, ${r.secs.toFixed(0)} s` : r.why}`);
  if (!ok) faults.push(`seed ${seed}, ${A.say}: ${A.downAt ? `the player was not put down at '${A.downAt}'` : r.why}`);
}
delete process.env.TUT_KIT_ARM;

{ const { readFileSync } = await import('node:fs'), { resolve } = await import('node:path'), ROOT = resolve(import.meta.dir, '../..');
  const { standUp } = await import('./headless');
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`), K = await import(`${ROOT}/src/game/kit.ts`), { RAILS } = await import(`${ROOT}/src/game/rails.ts`);
  const { wallAt } = await import(`${ROOT}/src/game/wallcharge.ts`), { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  const { BACK, WALL_MARK } = await import(`${ROOT}/tools/levels/tut-kit.ts`);
  // (the frozen stand holds off the card that is his whole kit: the scripted player's own words are raised for this floor as the runner raises them for a run)
  const up: string[] = []; if (process.env.TPS_PROFILE !== 'shipped') for (const [k, v] of Object.entries((await import('./lessons/tut-kit')).raises as Record<string, string>)) if (process.env[k] === undefined) { process.env[k] = v; up.push(k); }
  const log = console.log, warn = console.warn, err = console.error; console.log = console.warn = console.error = () => { };
  let held = '', bag = '', why = '', lampLit = -1, lampOut = -1, wall = ''; const rails = RAILS.on, destr = DESTRUCTION.on;
  try { K.kitIssuedPage(true); RAILS.on = true; DESTRUCTION.on = true;
    const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels/tut-kit.json'), 'utf8'))), seed: 1 }), g = W.game;
    const r = startFromQuery(g, '?levelFile=tut-kit.json&roomclear=1&squad4'); why = r.why ?? '';
    for (let i = 0; i < 5; i++) W.step(1 / 60);
    const P = g.player, T = g.doorTools;
    held = `${P.canisters}/${P.flashbangs}/${P.frags}/${P.rifle.shells} ${P.rifle.attachment}`;
    bag = `${T.charges}/${T.bangs}/${T.frags}/${g.wallCharge?.carried ?? '?'}`;
    // the shop's lamp, by the CPU's estimate (no moon, no bounce: the page's meter reads higher both times)
    const { lightEstimateAt } = await import(`${ROOT}/src/game/guards.ts`);
    const meter = () => Math.round(g.visibilityOf(Math.max(lightEstimateAt(g, [BACK[0], 1.2, BACK[2]]), lightEstimateAt(g, [BACK[0], 1.6, BACK[2]]))) * 100);
    const lamp = g.engine.lights.lights.find((l: any) => l.name === 'lamp_shop'); lampLit = meter(); if (lamp) { const was = lamp.intensity; lamp.intensity = 0; lampOut = meter(); lamp.intensity = was; }
    // the wall between the bunk room and the fuel room takes a charge
    const spot = wallAt(g, WALL_MARK, [28.4, 0, 6.6]); wall = spot ? (spot.ok ? 'takes a charge' : spot.why) : 'no wall found there';
  } catch (x) { why = String((x as Error).message); } finally { for (const k of up) delete process.env[k]; K.kitIssuedPage(false); RAILS.on = rails; DESTRUCTION.on = destr; console.log = log; console.warn = warn; console.error = err; }
  // (one of each is the page's pouches rule; the frozen stand has no such rule and he is as a Player is built, the launcher still on its grenade)
  const kitOk = !why && (process.env.TPS_PROFILE === 'shipped' ? held === '1/1/1/1 he40' : / he40$/.test(held) && held.split(' ')[0].split('/').every(n => +n >= 1));
  console.log(`  ${kitOk ? 'PASS' : 'FAIL'}  on its page he carries each can (one of each on the stand the page opens on) and the launcher is on its grenade: smoke/stun/frags/40 mm ${held}${why ? ` (${why})` : ''}`);
  if (!kitOk) faults.push(`the page's kit is not one of each with the launcher on its grenade (${held}${why ? `, ${why}` : ''})`);
  const [c, b, f] = bag.split('/').map(Number), bagOk = c >= 1 && b >= 1 && f >= 1;
  console.log(`  ${bagOk ? 'PASS' : 'FAIL'}  the squad's bag holds what the lines spend: door charges/stun/frags/wall charges ${bag}`);
  if (!bagOk) faults.push(`the squad's bag is short of a door charge, a stun grenade or a frag (${bag})`);
  const lampOk = lampLit >= 50 && lampOut >= 0 && lampOut <= 20;
  console.log(`  ${lampOk ? 'PASS' : 'FAIL'}  the ground beside the shop's door is lit by the shop's lamp and dark without it: ${lampLit} with it, ${lampOut} with it out`);
  if (!lampOk) faults.push(`the shop's lamp does not decide the light beside its door (${lampLit} lit, ${lampOut} out)`);
  const wallOk = wall === 'takes a charge';
  console.log(`  ${wallOk ? 'PASS' : 'FAIL'}  the wall the squad's charge is asked for is one a charge cuts: the marked wall ${wall}`);
  if (!wallOk) faults.push(`the marked wall: ${wall}`);
}
console.log(faults.length ? `=== tut-kit-check: ${faults.length} FAULT${faults.length === 1 ? '' : 'S'} ===\n${faults.map(f => `  - ${f}`).join('\n')}` : '=== tut-kit-check: ALL PASS ===');
process.exit(faults.length ? 1 : 0);
