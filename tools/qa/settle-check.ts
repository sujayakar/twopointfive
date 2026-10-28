// THE START WAITS FOR THE PICTURE (src/ui/settle.ts SETTLE): a mission's brief holds its START until every shader is in, the frames behind it have been drawn
// with them and auto quality has made its pick. What is checked here is the rule itself on a clock the check moves by hand, and that main.ts still has its
// places; the brief's own held state is in tools/qa/loadbrief-check.ts and the queue's rush in tools/qa/pipequeue-check.ts. What a real page looks like at its
// start is not something a check sees: never seen on a GPU from here.
//   bun run tools/qa/settle-check.ts                  exit 0 = every line passed, 1 = one failed
//   SETTLE_BREAK=1 bun run tools/qa/settle-check.ts   the start is live at once (the build as it was): must FAIL
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { SETTLE, Settle } from '../../src/ui/settle';

const ROOT = resolve(import.meta.dir, '../..');
let red = 0; const line = (ok: boolean, what: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
const F = 1 / 60, IN = { out: 0, measuring: false }, OUT = { out: 3, measuring: true }, MEASURING = { out: 0, measuring: true };
/** `n` display frames, each drawn, under `f`: how many steps it took to answer true (-1 = it never did) */
const run = (S: Settle, n: number, f: { out: number; measuring: boolean }, dt = F, drawn = 1): number => { for (let i = 1; i <= n; i++) if (S.step(dt, drawn, f)) return i; return -1; };

line(SETTLE.on === false, 'the switch is off in the module: the page raises it (main.ts), a tool that stands a brief on a fake DOM gets the brief as it was');
{ const S = new Settle();
  line(run(S, 600, OUT) === -1 && !S.picture && S.frames === 0, `programs still compiling: ten seconds of drawn frames and the start is not live, nor has the picture's turn begun (${S.waited.toFixed(1)} s waited)`);
  const a = run(S, SETTLE.frames + 30, MEASURING);
  line(a === -1 && S.picture && S.frames === 0, `the programs in, auto quality still measuring: ${SETTLE.frames + 30} frames drawn and none of them counts, the pick will change the picture`);
  const b = run(S, 200, IN);
  line(b === SETTLE.frames && S.done && S.step(F, 1, OUT), `the pick made: live ${SETTLE.frames} drawn frames after it, and it stays so (${b})`); }
{ const S = new Settle(), n = run(S, 200, IN);
  line(n === SETTLE.frames, `a pinned preset with every program in from the start: live after ${SETTLE.frames} drawn frames, no sooner (${n})`); }
{ const S = new Settle(); run(S, 30, IN); run(S, 1, { out: 1, measuring: false }); const zeroed = S.frames === 0 && !S.picture, n = run(S, 200, IN);
  line(zeroed && n === SETTLE.frames, `a program that lands late starts the picture's turn again: the 30 frames before it were a stand-in's (${n} more)`); }
{ const S = new Settle(), n = run(S, 2000, MEASURING), secs = n * F;
  line(n > 0 && Math.abs(secs - SETTLE.maxSecs) < 0.05, `a page that never makes its pick is live after the picture's ${SETTLE.maxSecs} s (${secs.toFixed(2)} s)`); }
{ const S = new Settle(), n = run(S, 2000, IN, F, 0), secs = n * F;
  line(n > 0 && Math.abs(secs - SETTLE.maxSecs) < 0.05 && S.frames === 0, `frames that are not drawn count for nothing: only the ${SETTLE.maxSecs} s end that wait (${secs.toFixed(2)} s)`); }
{ const S = new Settle(), n = run(S, 100, IN, 2), secs = n * 2;
  line(n === Math.ceil(SETTLE.maxSecs / 2) && secs < SETTLE.maxSecs + 2, `a page at a frame every two seconds is not held for ${SETTLE.frames} of them: the ${SETTLE.maxSecs} s are the wall clock's (${secs} s)`); }
{ const S = new Settle(), a = run(S, SETTLE.programSecs * 4, { out: 1, measuring: false }, 0.25), b = run(S, 200, { out: 1, measuring: false });
  line(a === -1 && b === SETTLE.frames, `a compile that never comes back does not keep the mission shut: after ${SETTLE.programSecs} s the picture has its turn and the start goes live (${b} frames on)`); }

// ---- wiring (the page's own entry, read as text: the harness never runs it)
{ const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8'), brief = readFileSync(`${ROOT}/src/ui/loadbrief.ts`, 'utf8');
  const flag = /SETTLE\.on = v === null \? navigator\.webdriver !== true : v !== '0';/.test(main);
  const road = /else if \(brief && briefGo\) briefSettle\(brief\);/.test(main) && !/brief\.live\(/.test(main);
  const waits = /b\.hold\(\); pipeQueue\.rush\(true\);/.test(main) && /out = P\.total - P\.done \+ laterProgress\.pending/.test(main) && /cutSwap\.state === 'building'\) void \(cutSwap\.ready \?\? Promise\.resolve\(false\)\)\.finally\(laterHold\(\)\)/.test(main) && /measuring: quality\.calibrating/.test(main);
  const ends = /const end = \(\): void => \{ settling = false; pipeQueue\.rush\(false\);/.test(main) && /if \(done\) \{ end\(\); boot\.mark\('settled'\); b\.live\(run\); return; \}/.test(main) && /if \(!b\.held\) \{ end\(\); return; \}/.test(main);
  const drawn = /quality\.measuring \|\| settling \|\|/.test(main) && /settling = S\.picture;/.test(main);
  const away = /const back = \(\): void => \{ settleLast = performance\.now\(\); \};/.test(main) && /addEventListener\('visibilitychange', back\)/.test(main);
  const held = /get held\(\) \{ return up && \(live \|\| holding\); \}/.test(brief) && /hold\(\) \{ if \(up && !live\) holding = true; \}/.test(brief);
  const records = (main.match(/missionLive\(game\) && !menu\.isOpen && !brief\?\.held && !document\.hidden/g) ?? []).length === 2;
  line(flag && road && waits && ends && drawn && away && held && records, `wiring: main.ts raises the switch where a person sits, a mission's brief goes live by this road alone, it waits on every pipeline asked for, the later set and the second set and on auto quality's pick, the picture's frames are drawn, a tab's time away is not counted, the queue's rush ends with it, the brief holds the world meanwhile, and a visit's records do not count the frames behind it as play (${[flag, road, waits, ends, drawn, away, held, records].map(x => x ? 'y' : 'N').join('')})`); }

console.log(`=== settle: ${red ? `${red} FAILED` : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
