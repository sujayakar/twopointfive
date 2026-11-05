// DOES EVERY LEVEL FIT THE RUST MODULE'S COPY OF THE BOX WORLD (src/scene/boxMirror.ts; rust/sim/src/world.rs: the copy's tables are of a fixed size)?
// A level that does not fit is a level whose every question the TypeScript answers, and the day the TypeScript is gone it is a level that cannot be
// played. So each level of levels/ is stood up as a game stands it, a tick's open is asked for, and what it holds is read against the module's own caps:
// statics, grid cells, the cells' entries, tilted statics, the cut rows the copy spent (a cut or a shaped static takes one), door leaves, loose furniture and
// its part boxes (a list of the leaves' table), the glass a line can note. A table of the copy that no row here counts is a FAIL: the list cannot fall behind.
//   bun run tools/qa/copyfit-check.ts [--levels a.json,b.json]   exit 0 = every level fits with the room below · 1 = one does not (named)
//   COPYFIT_BREAK=1 …                                            the red path: every cap is read as a twentieth of itself, and the check must FAIL
// THE ROOM: a level's statics grow in play (a wall holed stands back up as pieces, StaticCollision.addStatic), so a level must leave half the statics'
// table and half the cells' entries free as it loads; the other tables do not grow in play and need only fit.
// Not game code; nothing here needs a GPU.
import { readdirSync } from 'fs';
import { resolve } from 'path';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const BREAK = process.env.COPYFIT_BREAK === '1', K = BREAK ? 0.05 : 1;
const files = (flag('--levels') ?? readdirSync(resolve(ROOT, 'levels')).filter(f => f.endsWith('.json') && !f.endsWith('.lint.json')).join(',')).split(',');
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
/** the tables that grow in play, and the share of each a level may fill as it loads */
const GROWS: Record<string, number> = { boxes: 0.5, items: 0.5, cuts: 0.5 };   // (a blast's pieces are cut boxes: a row each)
/** a row of this check that is counted against another row's table */
const CAP_OF: Record<string, string> = { parts: 'leaves' };
const most: Record<string, { n: number; cap: number; file: string }> = {}; const bad: string[] = []; let stood = 0, skipped = 0;
for (const file of files) {
  let W: any; hush(true);
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); W = await standUp({ uploadEvery: 1000, ...(file === 'office.json' ? {} : { level: loadLevel(data) }) }); }
  catch (e) { hush(false); log(`${file.padEnd(26)} not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}) — skipped`); skipped++; continue; }
  hush(false); stood++;
  const g = W.game, col = g.col as any, M = col.mirror as any;
  col.tickOpen(); const fits = M.live === true; const caps = M.caps as Record<string, number>, cutRows = (M.cutNext as number) - (M.cutFree as number[]).length; col.tickShut();   // (the cut rows the copy itself spent on the statics: a cut box and a shaped one each take one, boxMirror.ts kindOf)
  const boxes = col.boxes as any[];
  const has: Record<string, number> = {
    boxes: boxes.length, cells: col.W * col.H, items: (col.cells as number[][]).reduce((s, c) => s + c.length, 0), tilted: (col.tiltedAt as number[]).length / 5,
    cuts: cutRows, leaves: (col.dynamicBoxes as any[]).length, things: (g.props?.all?.length ?? 0),
    parts: ((g.props?.props ?? []) as any[]).reduce((n, q) => n + q.boxes.length, 0),   // (every part box of the furniture is a list of the leaves' table: boxMirror.ts things)
    panes: boxes.reduce((n, q) => n + ((q.flags & BoxFlag.Glass) ? 1 : 0), 0) };   // (the panes one line can note: no more than the level's glass)
  const over: string[] = [];
  for (const c of Object.keys(caps)) if (!Object.keys(has).some(k => (CAP_OF[k] ?? k) === c)) over.push(`the table '${c}' is counted by no row of this check`);   // (a table added to the copy is a row added here)
  for (const k of Object.keys(has)) { const cap = Math.floor((caps[CAP_OF[k] ?? k] ?? 0) * K), room = GROWS[k] ?? 1;
    if (!most[k] || has[k] / cap > most[k].n / most[k].cap) most[k] = { n: has[k], cap, file };
    if (has[k] > cap * room) over.push(`${k} ${has[k]} of ${cap}${room < 1 ? ` (a table that grows in play: ${Math.round(room * 100)} % at most as it loads)` : ''}`); }
  if (!fits && !BREAK) over.push('the copy did not stand as a tick opened');
  if (over.length) bad.push(`${file}: ${over.join(' · ')}`);
}
log(`${stood} levels stood up${skipped ? `, ${skipped} skipped` : ''}${BREAK ? ' · RED PATH: every cap read as a twentieth of itself' : ''}`);
for (const [k, m] of Object.entries(most)) log(`  ${k.padEnd(7)} fullest ${String(m.n).padStart(6)} of ${String(m.cap).padStart(6)} (${(100 * m.n / m.cap).toFixed(1)} %) · ${m.file}`);
for (const b of bad) log(`- FAIL — ${b}`);
const ok = bad.length === 0 && stood > 0 && skipped === 0;
log(ok ? '=== copyfit-check: PASS — every level fits the module\'s copy of the box world, with room to grow ===' : `=== copyfit-check: ${bad.length} level(s) do not fit${skipped ? `, ${skipped} did not stand` : ''} — FAIL ===`);
process.exit(ok ? 0 : 1);
