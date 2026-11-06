// A QUIET RUN OF STEPS IS THE SAME PLAY (PLAY.md, "A quiet run of steps"): the office stood by the harness, the player walked about by the host's input in
// runs of `W.steps` (no tool code between two steps), and the game's digest printed after each run. Run it under each word and hold the lines to each other:
//   tools/blitz/rust.sh ts   bun run tools/blitz/quiet-check.ts          the TypeScript tick
//   tools/blitz/rust.sh rust bun run tools/blitz/quiet-check.ts          the Rust game, quiet runs
//   RH_OFF=quiet tools/blitz/rust.sh rust bun run tools/blitz/quiet-check.ts   the Rust game, an apply after every step
// With RH_TICKS=<folder> each also writes its line a tick (the die, the host's half, the block's parts) for a tick-by-tick compare.
import '../qa/stand.ts';
const { standUp } = await import('../qa/headless.ts');
const { digestGame, hashDigest } = await import('../../src/game/digest.ts');
const W: any = await standUp({ seed: 5 }), g = W.game, DT = 1 / 60, t0 = performance.now();
const runs: [string[], number, number, number][] = [[['KeyW'], 240, 900, 300], [['KeyW', 'KeyD'], 180, 1200, 420], [['KeyS'], 120, 700, 600], [['KeyA', 'ShiftLeft'], 300, 500, 520], [[], 600, 800, 500], [['KeyW'], 900, 1000, 450]];
let n = 0;
for (const [keys, ticks, mx, my] of runs) { W.input.keys.clear(); for (const k of keys) W.input.keys.add(k); W.input.mouseX = mx; W.input.mouseY = my;
  W.steps(ticks, DT); n += ticks;
  const p = g.player.char.pos; console.log(`after ${n} ticks: digest ${hashDigest(digestGame(g))} · time ${g.time.toFixed(4)} · the player at ${p[0].toFixed(4)}, ${p[2].toFixed(4)} · the die ${g.dice?.n}`); }
const H = g.rustHost; console.log(`${n} ticks in ${(performance.now() - t0).toFixed(0)} ms (${((performance.now() - t0) / n).toFixed(2)} ms a tick)${H ? ` · quiet steps ${H.quietSteps} in ${H.quietRuns} runs` : ''}`);
process.exit(0);
