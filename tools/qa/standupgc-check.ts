// A GAME STOOD UP HAS DONE WITH THE LAST ONE (tools/qa/headless.ts standUp, STANDUP_COLLECTS). A tool that stands games up back to back used to grow without
// end: a dropped game stayed as garbage (these tools never go back to the event loop, where bun's collector does its full collections from), and a tally's
// worker went from 0.4 GB to 6.4 GB in two and a half minutes. The stand-up now asks for one full collection before it builds.
// This script IS the process (probes.ts runs it as one of its own: a probe run's heap is whatever the probes before it left). It stands up twenty-five games
// back to back, a few steps each, with nothing collected by the script, and reads the heap as the runtime reports it: from the second game to the last it
// may grow by MOST. It runs under the word UNSET, which is what every tool runs under.
//   bun run tools/qa/standupgc-check.ts       exit 0 = pass, 1 = a FAIL line        bun run tools/qa/probes.ts standupgc   (family `perf`)
//   STANDUPGC_BREAK=1 …   = QA_STANDUP_GC=0, the stand-up as it was: the growth line must FAIL
// What it does not say: anything of the page (in a browser a mission is a page load and its world goes with the page), or of garbage that is not a dropped
// game. It holds the heap, not the process's size: that is printed, and follows the heap, but an allocator's habits are not this check's to bar.
import { heapStats } from 'bun:jsc';
const BREAK = process.env.STANDUPGC_BREAK === '1';
if (BREAK) process.env.QA_STANDUP_GC = '0'; else delete process.env.QA_STANDUP_GC;   // (before the harness is loaded: it reads the word once)
const { standUp, STANDUP_COLLECTS } = await import('./headless.ts');
const GAMES = 25, STEPS = 20, MOST = 60;   // (MB: measured 28 and 32 with the collection on the two stands and 141 and 184 without, see docs/agents/qa.md)
const say = console.log, mb = (x: number) => Math.round(x / 1048576); let red = 0;
const check = (ok: boolean, w: string) => { say(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
console.log = console.info = console.warn = () => { };
let second = 0, secondRss = 0, stood = 0;
for (let k = 0; k < GAMES; k++) { const W: any = await standUp({ seed: k + 1, uploadEvery: 4 }); for (let i = 0; i < STEPS; i++) W.step(1 / 60); stood++; if (k === 1) { second = mb(heapStats().heapSize); secondRss = mb(process.memoryUsage().rss); } }
const last = mb(heapStats().heapSize), rss = mb(process.memoryUsage().rss);
check(stood === GAMES && STANDUP_COLLECTS === !BREAK && last - second <= MOST, `${stood} games stood up back to back in one process, ${STEPS} steps each, nothing collected by this script: the heap grew ${last - second} MB from the second game to the last (${second} to ${last}), ${MOST} at the most; the process went from ${secondRss} MB to ${rss}${BREAK ? ' — the stand-up as it was (QA_STANDUP_GC=0): this line must fail' : ' (the word unset, as every tool runs)'}`);
// the word is read strictly: a spelling it does not know stops the tool, it does not quietly leave the collection on
{ const p = Bun.spawn(['bun', '-e', `await import(${JSON.stringify(import.meta.dir + '/headless.ts')})`], { stdout: 'ignore', stderr: 'pipe', env: { ...process.env, QA_STANDUP_GC: 'off' } as Record<string, string> });
  const err = await new Response(p.stderr).text(), code = await p.exited;
  check(code !== 0 && /QA_STANDUP_GC=off:/.test(err), `QA_STANDUP_GC=off (a spelling the harness does not know) stops the tool by name: exit ${code}${p.signalCode ? `, signal ${p.signalCode}` : ''}, "${(err.split('\n').find(l => /QA_STANDUP_GC=off:/.test(l)) ?? err.trim().split('\n').pop() ?? '').trim().slice(0, 120)}"`); }
say(`\nstandupgc-check: ${red ? `${red} FAILED` : 'all passed'}${BREAK ? ' (STANDUPGC_BREAK=1: the growth line must fail)' : ''}`);
process.exit(red ? 1 : 0);
