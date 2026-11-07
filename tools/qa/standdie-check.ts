// NO GAME A TOOL STANDS ROLLS THE CLOCK'S DICE (tools/qa/die.ts).
//   bun run tools/qa/standdie-check.ts                     three lines; exit 0 all pass · 1 a line failed
//   STANDDIE_BREAK=1 bun run tools/qa/standdie-check.ts    the red path: the children run with QA_STAND_DIE=0 (the harness as it was), and line (1) must FAIL
// Each line stands games in children of its own, because the question is what a PROCESS that named no die gets:
//   (1) two processes that stand a bare game (`standUp()`, no seed, no die put in Math.random) stand the same game: one die, one digest after two seconds;
//   (2) a tool that seeds first and loads the harness after keeps its own stream: the game's die is a draw of the tool's die, not the harness's;
//   (3) a tool that names the game's die (`standUp({ seed })`) leaves Math.random the engine's own.
import { resolve } from 'node:path';
const HERE = import.meta.dir, SELF = resolve(HERE, 'standdie-check.ts');

if (process.argv[2] === '--child') {   // one stand-up, said as one line of JSON
  const how = process.argv[3];
  if (how === 'seeds-first') { const { seeded } = await import('./die.ts'); Math.random = seeded(7); }
  const { standUp } = await import('./headless.ts'), { clockDice } = await import('./die.ts');
  const W = await standUp(how === 'named' ? { seed: 5 } : {});
  W.steps(120, 1 / 60);
  const { digestGame, hashDigest } = await import('../../src/game/digest.ts');
  console.log('@@' + JSON.stringify({ die: W.game.dice?.seed ?? null, digest: hashDigest(digestGame(W.game, true)), clock: clockDice(), next: Math.random() }));
  process.exit(0);
}
interface Said { die: number | null; digest: string; clock: boolean; next: number }
async function child(how: string): Promise<Said> {
  const env: Record<string, string | undefined> = { ...process.env, TPS_SEED: '' };   /* (no seed named for the child, whatever this run was given) */ if (process.env.STANDDIE_BREAK === '1') env.QA_STAND_DIE = '0';
  const p = Bun.spawn(['bun', 'run', SELF, '--child', how], { stdout: 'pipe', stderr: 'pipe', env }), out = await new Response(p.stdout).text(), err = await new Response(p.stderr).text(); await p.exited;
  const l = out.split('\n').find(x => x.startsWith('@@')); if (!l) throw new Error(`the child said no line (${how}): ${(err || out).trim().split('\n').slice(-3).join(' | ').slice(0, 300)}`);
  return JSON.parse(l.slice(2)); }
let fails = 0; const line = (ok: boolean, what: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}: ${detail}`); };
{ const [a, b] = await Promise.all([child('bare'), child('bare')]);
  line(a.die !== null && a.die === b.die && a.digest === b.digest && !a.clock && a.next === b.next, '(1) two processes that stand a bare game stand the same game, and Math.random is a die after it', `die ${a.die} and ${b.die} · digest ${a.digest} and ${b.digest} · the clock's dice still in place: ${a.clock}, ${b.clock}`); }
{ const [a, b, bare] = await Promise.all([child('seeds-first'), child('seeds-first'), child('bare')]);
  line(a.die !== null && a.die === b.die && a.die !== bare.die, '(2) a tool that seeds first and loads the harness after keeps its own stream', `die ${a.die} and ${b.die}, a bare game's ${bare.die}`); }
{ const a = await child('named');
  line(a.die === 5 && a.clock, "(3) a tool that names the game's die leaves Math.random the engine's own", `die ${a.die} · the clock's dice still in place: ${a.clock}`); }
console.log(fails ? `=== standdie-check: ${fails} FAIL ===` : '=== standdie-check: ALL PASS ===');
process.exit(fails ? 1 : 0);
