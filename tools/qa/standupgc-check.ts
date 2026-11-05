// A GAME STOOD UP HAS DONE WITH THE LAST ONE (tools/qa/headless.ts standUp, STANDUP_COLLECTS). A tool that stands games up back to back used to grow without
// end: bun leaves a dropped game (some 40 MB) uncollected, and a tally's worker went from 0.4 GB to 6.4 GB in two and a half minutes. The stand-up now asks for
// one full collection before it builds. This check plays twenty-four of that tally's own missions back to back IN A PROCESS OF ITS OWN (a probe run's own heap is
// whatever the probes before it left; a game only stood up and stepped is too little to show it) and reads the heap as the runtime itself reports it, with
// nothing collected by the check: it must not have grown, from the second mission to the last, by more than a few games' worth.
//   bun run tools/qa/standupgc-check.ts       exit 0 = pass, 1 = a FAIL line        bun run tools/qa/probes.ts standupgc   (family `harness`)
//   STANDUPGC_BREAK=1 …   = the child is run with QA_STANDUP_GC=0 (the stand-up as it was): must FAIL
// What it does not say: anything of the page. In a browser a mission is a page load and its world goes with the page.
import { heapStats } from 'bun:jsc';
const GAMES = 24, SCN = 'room-clear-pair', MOST = 110;   // (MB the heap may have GROWN between the second mission and the last: measured with the collection and without it, see the notes)
if (process.argv[2] === 'child') {
  const quiet = console.log; console.log = console.info = console.warn = () => { };
  const T = await import('./bang-tally.ts'), doors = await T.discover(SCN), mb = (x: number) => Math.round(x / 1048576); let second = 0, played = 0;
  for (let k = 0; k < GAMES; k++) { const d = doors[(k >> 1) % doors.length];
    try { await T.run({ scn: SCN, door: d.door, room: d.room, arm: k % 2 ? 'bang' : 'open', seed: 1 + (k >> 2) } as any); played++; } catch { /* a mission that will not start still stood a game up */ }
    if (k === 1) second = mb(heapStats().heapSize); }
  quiet(`@@${JSON.stringify({ second, last: mb(heapStats().heapSize), rss: mb(process.memoryUsage().rss), played })}`); process.exit(0);
}
export async function standupGcCheck(): Promise<boolean> {
  const BREAK = process.env.STANDUPGC_BREAK === '1', env = { ...process.env, QA_STANDUP_GC: BREAK ? '0' : '1' } as Record<string, string>; delete env.STANDUPGC_BREAK;
  const p = Bun.spawn(['bun', 'run', import.meta.path, 'child'], { stdout: 'pipe', stderr: 'pipe', env }), out = await new Response(p.stdout).text(), err = await new Response(p.stderr).text(); await p.exited;
  const line = out.split('\n').find(l => l.startsWith('@@')), r = line ? JSON.parse(line.slice(2)) as { second: number; last: number; rss: number; played: number } : null;
  const ok = !!r && r.played >= GAMES - 2 && r.last - r.second <= MOST;
  console.log(`${ok ? '- PASS' : '- FAIL'} — ${GAMES} of the tally's missions (${SCN}) back to back in one process, nothing collected by the check: ${r ? `${r.played} played, the heap grew ${r.last - r.second} MB from the second to the last (${r.second} to ${r.last}; the process ${r.rss} MB), ${MOST} at the most` : `the child said nothing (${err.trim().split('\n').pop()?.slice(0, 160) ?? 'no error line'})`}${BREAK ? ' — the stand-up as it was (QA_STANDUP_GC=0): this run must fail' : ''}`);
  console.log(`\nstandupgc-check: ${ok ? 'all passed' : '1 FAILED'}`);
  return ok;
}
if (import.meta.main && process.argv[2] !== 'child') process.exit(await standupGcCheck() ? 0 : 1);
