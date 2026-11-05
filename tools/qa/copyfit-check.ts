// DOES EVERY LEVEL FIT THE RUST MODULE'S COPY OF THE BOX WORLD AND THE RULES' POOLS (src/scene/boxMirror.ts and rust/sim/src/world.rs: the copy's tables
// are of a fixed size; sim/doorshome.ts, sim/propshome.ts: so are the pools)? A level that does not fit the copy is a level whose every question the
// TypeScript answers, and the day the TypeScript is gone it is a level that cannot be played; a level over a pool is not loaded at all. So each level
// of levels/ is stood up as a game stands it, a tick's open is asked for, and what it holds is read against what holds it — BY THE GAME'S OWN COUNT
// (src/game/levelfit.ts fitRows: one source for the game, the validator and this check): statics, grid cells, the cells' entries, tilted statics, the cut
// rows the copy spent, leaves, the loose furniture and its part boxes, the glass a line can note, sheets of glass, doors. A TABLE OF THE COPY THAT NO
// ROW OF THAT COUNT IS HELD BY IS A FAIL: the list cannot fall behind the copy.
//   bun run tools/qa/copyfit-check.ts [--levels a.json,b.json]   exit 0 = every level fits with the room below · 1 = one does not (named)
//   COPYFIT_BREAK=1 …                                            the red path: every cap is read as a twentieth of itself, and the check must FAIL
// THE ROOM: one thing grows in play. A wall holed stands back up as pieces (game/wallbreak.ts rebuild, the one caller of StaticCollision.addStatic),
// each a static, at most one cut row, and an entry in every cell of the box it was cut from. The wall breaker holds itself to the room the level left
// for them as it loaded (WallBreak.room: the smaller of its knob and that room), so a level with little room is no error; but a SHIPPED level is to
// have the room for every piece the knob allows, whichever things are broken, and the row `pieces` says so (WallBreak.piecesSure). The other tables
// do not grow in play and need only fit. That every breakable there is, and every wall a charge may take on, is counted in that row is said per level
// too: the pieces of anything else would be filed in cells nobody counted.
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
  catch (e) { hush(false); if (String((e as Error).message).startsWith('this level is too big to load')) { bad.push(`${file}: ${(e as Error).message}`); continue; }   // (…and its word for a game that stood over a table: stoodOver)
    log(`${file.padEnd(26)} not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}) — skipped`); skipped++; continue; }
  hush(false); stood++;
  const g = W.game, col = g.col as any, M = col.mirror as any;
  col.tickOpen(); const fits = M.live === true, tables = M.tables as Record<string, number>, rows: any[] = LF.fitRows(g, tables); col.tickShut();
  const over: string[] = [];
  for (const c of Object.keys(tables)) if (!rows.some(r => r.table === c)) over.push(`the table '${c}' of the copy is held by no row of game/levelfit.ts fitRows`);   // (a table added to the copy is a row added there)
  for (const r of rows) { const cap = Math.floor(r.cap * K);
    if (!most[r.row] || r.has / cap > most[r.row].n / most[r.row].cap) most[r.row] = { n: r.has, cap, file };
    if (r.has > cap) over.push(`${r.row} ${r.has} of ${cap}${r.play ? ' (the room this level leaves for its pieces is less than the knob allows)' : ''}`); }
  if (!fits && !BREAK) over.push('the copy did not stand as a tick opened');
  // the room's count rests on every piece being cut from a box that count read: each breakable's own, and (were a charge to take a wall on) an adoptable one
  { const walls = g.destr.walls, foot = (b: any) => col.footEntries(b); let most = 1; for (const B of walls.all()) most = Math.max(most, foot(B.box));
    for (let i = 0; i < walls.authored; i++) if (walls.adoptable(i)) most = Math.max(most, foot(col.boxes[i]));
    const R = walls.room, sure = Math.max(0, Math.min(R.boxes, R.cuts, Math.floor(R.items / most)));
    if (walls.piecesSure() !== sure) over.push(`the wall breaker says its room surely holds ${walls.piecesSure()} pieces, and by its breakables and the walls a charge may take on it is ${sure}`);
    if (R.boxes !== tables.boxes - walls.authored) over.push(`the room was not taken as the level loaded: ${R.boxes} statics, and the copy holds ${tables.boxes - walls.authored} past the level's own`); }
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
// THE REFUSALS BEHIND THE COUNT: (1) a stood game over a table is refused by name as it is made (game/levelfit.ts stoodOver: the last statement of Game's
// constructor), and one inside every table is not; (2) a box world too big for the copy that nobody counted first (a game's own first questions come before
// its count; a box world with no game has no count) is "too big to load" from the copy itself, at its first question; (3) one that has outgrown the copy IN
// PLAY (statics added past the wall breaker, which nothing in the game does) is a throw by name as the next tick opens, not a level whose questions go elsewhere.
{ hush(true); const W: any = await standUp({ uploadEvery: 1000 }); hush(false);
  const g = W.game, col = g.col as any, tables = col.mirror.tables as Record<string, number>, { makeBox } = await import(`${ROOT}/src/scene/boxes.ts`);
  const none = LF.stoodOver(g, tables), small = LF.stoodOver(g, { ...tables, boxes: col.boxes.length - 1 }), noCopy = LF.stoodOver(g, Object.fromEntries(Object.keys(tables).map(k => [k, 0])));
  if (none !== null) bad.push(`refusals: the default level, which fits, is refused: ${none}`);
  if (BREAK ? false : small === null || !/statics/.test(small)) bad.push(`refusals: a game one static over the copy's table is not refused (${small})`);
  if (noCopy !== null) bad.push(`refusals: with no copy at all (no module) the level is refused by the copy's tables: ${noCopy}`);
  const { StaticCollision } = await import(`${ROOT}/src/scene/collision.ts`);
  let load = ''; try { const big = new StaticCollision(Array.from({ length: tables.boxes + 1 }, () => makeBox({ c: [1, 1, 1], h: [0.1, 0.1, 0.1] }))) as any; big.tickOpen(); big.tickShut(); } catch (e) { load = String((e as Error).message); }   // (its paths' bake asks the copy first, as a game's does: the box world is refused as it is made)
  if (!/^this level is too big to load: \d+ statics/.test(load)) bad.push(`refusals: a box world of ${tables.boxes + 1} statics was stood up over a copy of ${tables.boxes} without being called too big to load (${load || 'no error'})`);
  col.tickOpen(); col.tickShut();   // (a tick: the game's box world is a level in play from here)
  let said = ''; for (let i = col.boxes.length; i <= tables.boxes; i++) col.addStatic(makeBox({ c: [1, 1, 1], h: [0.1, 0.1, 0.1] }));
  try { col.tickOpen(); col.tickShut(); } catch (e) { said = String((e as Error).message); }
  if (!/has outgrown the Rust module's copy of it in play/.test(said)) bad.push(`refusals: a box world of ${col.boxes.length} statics opened a tick over a copy of ${tables.boxes} without saying so (${said || 'no error'})`);
  log(`refusals: a game over a table is refused as it is made (${small ? 'said' : 'NOT said'}) · a box world too big for the copy is too big to load from its first question (${load ? 'said' : 'NOT said'}) · one grown past the copy in play is a throw by name (${said ? 'said' : 'NOT said'})`); }
log(`${stood} levels stood up${skipped ? `, ${skipped} skipped` : ''}${BREAK ? ' · RED PATH: every cap read as a twentieth of itself' : ''}`);
for (const [k, m] of Object.entries(most)) log(`  ${k.padEnd(7)} fullest ${String(m.n).padStart(6)} of ${String(m.cap).padStart(6)} (${(100 * m.n / m.cap).toFixed(1)} %) · ${m.file}`);
for (const b of bad) log(`- FAIL — ${b}`);
const ok = bad.length === 0 && stood > 0 && skipped === 0;
log(ok ? '=== copyfit-check: PASS — every level fits the module\'s copy of the box world, with the room for every piece its breaking may stand up ===' : `=== copyfit-check: ${bad.length} level(s) do not fit${skipped ? `, ${skipped} did not stand` : ''} — FAIL ===`);
process.exit(ok ? 0 : 1);
