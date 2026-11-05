// DOES EVERY LEVEL FIT THE RUST MODULE'S COPY OF THE BOX WORLD AND THE RULES' POOLS (src/scene/boxMirror.ts and rust/sim/src/world.rs: the copy's tables
// are of a fixed size; sim/doorshome.ts, sim/propshome.ts: so are the pools)? A level that does not fit the copy is a level whose every question the
// TypeScript answers, and the day the TypeScript is gone it is a level that cannot be played; a level over a pool is not loaded at all. So each level
// of levels/ is stood up as a game stands it, a tick's open is asked for, and what it holds is read against what holds it — BY THE GAME'S OWN COUNT
// (src/game/levelfit.ts fitRows: one source for the game, the validator and this check): statics, grid cells, the cells' entries, tilted statics, the cut
// rows the copy spent, leaves, the loose furniture and its part boxes, the glass a line can note, sheets of glass, doors. A TABLE OF THE COPY THAT NO
// ROW OF THAT COUNT IS HELD BY IS A FAIL: the list cannot fall behind the copy.
//   bun run tools/qa/copyfit-check.ts [--levels a.json,b.json]   exit 0 = every level fits with the room below · 1 = one does not (named)
//   COPYFIT_BREAK=1 …                                            the red path: every cap is read as a twentieth of itself, and the check must FAIL
// THE ROOM: a level's statics grow in play (a wall holed stands back up as pieces, StaticCollision.addStatic), so a level must leave half the statics'
// table, half the cells' entries and half the cut rows free as it loads (levelfit.ts says which rows); the other tables do not grow in play and need
// only fit.
// A STAND LOADS ITS OWN LEVEL: the page's stand loads the lived-in rooms' furniture and the frozen one does not, so run it on both (TPS_PROFILE=shipped in
// front); the document as a whole, whatever a stand loads of it, is held to the pools' numbers by the level validator (scene/levelData.ts).
// Not game code; nothing here needs a GPU.
import { readdirSync } from 'fs';
import { resolve } from 'path';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const LF: any = await import(`${ROOT}/src/game/levelfit.ts`);
const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const BREAK = process.env.COPYFIT_BREAK === '1', K = BREAK ? 0.05 : 1;
const files = (flag('--levels') ?? readdirSync(resolve(ROOT, 'levels')).filter(f => f.endsWith('.json') && !f.endsWith('.lint.json')).join(',')).split(',');
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
const most: Record<string, { n: number; cap: number; file: string }> = {}; const bad: string[] = []; let stood = 0, skipped = 0;
for (const file of files) {
  let W: any; hush(true);
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()), level = file === 'office.json' ? null : loadLevel(data);
    if (level) { const over = LF.levelOver(level); if (over) { hush(false); bad.push(`${file}: ${over}`); continue; } }   // (the game's own word for a level it will not load: game/levelfit.ts)
    W = await standUp({ uploadEvery: 1000, ...(level ? { level } : {}) }); }
  catch (e) { hush(false); log(`${file.padEnd(26)} not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}) — skipped`); skipped++; continue; }
  hush(false); stood++;
  const g = W.game, col = g.col as any, M = col.mirror as any;
  col.tickOpen(); const fits = M.live === true, tables = M.tables as Record<string, number>, rows: any[] = LF.fitRows(g, tables); col.tickShut();
  const over: string[] = [];
  for (const c of Object.keys(tables)) if (!rows.some(r => r.table === c)) over.push(`the table '${c}' of the copy is held by no row of game/levelfit.ts fitRows`);   // (a table added to the copy is a row added there)
  for (const r of rows) { const cap = Math.floor(r.cap * K);
    if (!most[r.row] || r.has / cap > most[r.row].n / most[r.row].cap) most[r.row] = { n: r.has, cap, file };
    if (r.has > cap * r.room) over.push(`${r.row} ${r.has} of ${cap}${r.room < 1 ? ` (a table that grows in play: ${Math.round(r.room * 100)} % at most as it loads)` : ''}`); }
  if (!fits && !BREAK) over.push('the copy did not stand as a tick opened');
  if (over.length) bad.push(`${file}: ${over.join(' · ')}`);
}
// THE CIVILIANS' ONE WAY IN (civilians.ts join): a game takes civilians up to levelfit.ts civMost and refuses the next by name, whoever adds him
{ hush(true); const W: any = await standUp({ uploadEvery: 1000 }), g = W.game, CV: any = await import(`${ROOT}/src/game/civilians.ts`); hush(false);
  const most: number = LF.civMost(), def = (i: number) => ({ name: `Extra ${i}`, kind: 'worker', tint: [0.4, 0.4, 0.4], pos: [...g.player.char.pos], face: 0 }); let said = '', at = -1;
  try { for (let i = g.civilians.length; i <= most; i++) { at = i; CV.spawnHostage(g, def(i)); } } catch (e) { said = String((e as Error).message); }
  const rows: any[] = LF.fitRows(g, g.col.mirror.tables), men = rows.find(r => r.row === 'doormen'), civ = rows.find(r => r.row === 'civilians');
  const okJoin = Number.isFinite(most) && most > 0 && g.civilians.length === most && at === most && /too big to load: more than \d+ civilians/.test(said) && !!men && men.has <= men.cap && !!civ && civ.has === most && civ.cap === most;
  log(`  civilians: a game took ${g.civilians.length} of ${most} and refused the next ${said ? 'by name' : '— NOT AT ALL'}; full of them, one step of its doors is handed ${men?.has} contacts of ${men?.cap} at most`);
  if (!okJoin) bad.push(`civilians: a game of ${g.civilians.length} (the most is ${most}) ${said ? `said "${said.slice(0, 80)}"` : 'refused nobody'}; doormen ${men?.has} of ${men?.cap}`); }
log(`${stood} levels stood up${skipped ? `, ${skipped} skipped` : ''}${BREAK ? ' · RED PATH: every cap read as a twentieth of itself' : ''}`);
for (const [k, m] of Object.entries(most)) log(`  ${k.padEnd(7)} fullest ${String(m.n).padStart(6)} of ${String(m.cap).padStart(6)} (${(100 * m.n / m.cap).toFixed(1)} %) · ${m.file}`);
for (const b of bad) log(`- FAIL — ${b}`);
const ok = bad.length === 0 && stood > 0 && skipped === 0;
log(ok ? '=== copyfit-check: PASS — every level fits the module\'s copy of the box world, with room to grow ===' : `=== copyfit-check: ${bad.length} level(s) do not fit${skipped ? `, ${skipped} did not stand` : ''} — FAIL ===`);
process.exit(ok ? 0 : 1);
