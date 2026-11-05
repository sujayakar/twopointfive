// A DOOR OPENS THE WAY ITS LEAF LEAVES ROOM (src/game/doors.ts DOORWAY.leafWay, src/game/doorswing.ts 'leaf room'; the maintainer, of his four on the compound:
// 'feels like squad gets stuck when stacking up or going through doors more now'). The compound's spare room door was dealt into the room, whose cot stands a
// metre inside it: the open leaf hung across the one strip between cot and wall and shut the room off from its own doorway.
//   bun run tools/qa/leafway-check.ts        (or: bun run tools/qa/probes.ts leafway)        exit 0 = every line PASS, 1 = a FAIL line
//   the deal      off: the spare room door opens into the room and its open leaf shuts floor off there · on: it opens to the corridor, nothing else of the level turns
//   why           no furniture, no turn · an authored side is never turned · the switch is read live: a leaf standing open is stood as wide the other way
//   four men      stood in the spare room with the door standing open, sent down the corridor: all through in seconds, not queued behind a leaf
//   kept          a checkpoint keeps the leaf where it stands · a recording from before the key replays with the rule off
//   every level   what the rule turns, and that no door it may turn is left shutting floor off
// LEAFWAY_BREAK=1: the rule is left off where the lines want it on — red.      --levels: the last line's table alone, every door whose open leaf shuts a square metre or more off named
type Vec3 = [number, number, number];
const BREAK = process.env.LEAFWAY_BREAK === '1';

async function kit() {
  const { standUp, ROOT } = await import('./headless.ts'); const { readFileSync, readdirSync } = await import('node:fs');
  const { DOORWAY, Doors } = await import(`${ROOT}/src/game/doors.ts`); const { dealSwings, leafCut, leafSwings, swingsNow } = await import(`${ROOT}/src/game/doorswing.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`); const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { StaticCollision } = await import(`${ROOT}/src/scene/collision.ts`); const { LIVED } = await import(`${ROOT}/src/scene/lived.ts`);
  const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  const data = (name: string) => parseLevelData(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8'));
  /** the compound stood up with its rooms furnished (whatever the stand says of lived-in rooms) and doors swinging one way; the rule as asked */
  const world = async (on: boolean, o: { lived?: boolean; edit?: (d: any) => void } = {}) => { const was = LIVED.on; LIVED.on = o.lived ?? true;
    try { const d = data('compound'); o.edit?.(d); const W: any = await quiet(() => standUp({ level: loadLevel(d, { lived: o.lived ?? true }) })); DOORWAY.oneWay = true; DOORWAY.leafWay = on && !BREAK; return W; } finally { LIVED.on = was; } };   // (AFTER standUp: the stand-up lines write the stand's own words)
  /** a level's deal with no game stood up (the editor's road: src/editor/swing.ts) */
  const deal = (name: string) => { const level = loadLevel(data(name), { validate: false, lived: true }), col = new StaticCollision(level.boxes), doors = new Doors(level.doors, () => {}); return { col, doors, deals: dealSwings(level, doors, col.nav, col) }; };
  const levels = () => readdirSync(`${ROOT}/levels`).filter((f: string) => f.endsWith('.json')).map((f: string) => f.slice(0, -5)).sort();
  return { ROOT, DOORWAY, leafCut, leafSwings, swingsNow, quiet, world, deal, levels };
}

/** every level's doors: what the rule turns, what it may not turn, what is left */
async function table(K: Awaited<ReturnType<typeof kit>>) {
  const turned: string[] = [], kept: string[] = [], left: string[] = [], bad: string[] = []; let n = 0, doors = 0, live = 0;   // (live: the doors the deal's own read hangs the other way, as the switch stands)
  for (const name of K.levels()) { let D: ReturnType<typeof K.deal>; try { D = K.deal(name); } catch { continue; }   // (a file in levels/ that is not a level)
    n++; live += K.swingsNow(D.deals).filter((s: any) => s.why === 'leaf room').length; for (const d of D.doors.list) { doors++; const row = D.deals.find((s: any) => s.door === d.def.name)!, cut = K.leafCut(D.col, d, d.dealt);
      if (row.clear) { const now = K.leafCut(D.col, d, row.clear.swing); turned.push(`${name} ${d.def.name} (${cut.toFixed(1)} m² shut off as dealt by '${row.why}', ${now.toFixed(1)} the other way)`); if (now < 0 || now >= 1 || now * 2 > cut || !K.leafSwings(D.col, d, row.clear.swing)) bad.push(`${name} ${d.def.name}: turned to a side that shuts ${now.toFixed(1)} m² off, or where its leaf meets something`); }
      else if (cut >= 1) { const other = K.leafCut(D.col, d, -d.dealt as 1 | -1); if (['authored', 'folds', 'plan', 'stands open'].includes(row.why)) kept.push(`${name} ${d.def.name} (${cut.toFixed(1)} m², '${row.why}')`); else if (other >= 0 && other < 1 && other * 2 <= cut && !d.def.angle && (d.dealt > 0 ? -d.minAuth : d.maxAuth) >= 0.9 && K.leafSwings(D.col, d, -d.dealt as 1 | -1)) bad.push(`${name} ${d.def.name}: shuts ${cut.toFixed(1)} m² off as dealt by '${row.why}', ${other.toFixed(1)} the other way, and was not turned`); else left.push(`${name} ${d.def.name} (${cut.toFixed(1)} m², ${other < 0 ? 'nowhere to stand' : other.toFixed(1)} the other way)`); } } }
  return { n, doors, live, turned, kept, left, bad };
}

export async function leafwayCheck(): Promise<boolean> {
  const K = await kit(), { DOORWAY } = K; let failed = false;
  const line = (ok: boolean, words: string) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${words}`); };
  const f2 = (x: number) => x.toFixed(2), was = DOORWAY.leafWay, wasOneWay = DOORWAY.oneWay;
  console.log(`=== leafway-check: a door opens the way its leaf leaves room${BREAK ? ' (LEAFWAY_BREAK=1: the rule left off)' : ''} ===`);
  try {
    const { spawnUnit } = await import(`${K.ROOT}/src/game/guards.ts`); const { saveState, loadState } = await import(`${K.ROOT}/src/game/savestate.ts`); const { knobsSnapshot, applyKnobs } = await import(`${K.ROOT}/src/game/journal.ts`);
    // ---- the deal
    const W0 = await K.world(false), g0 = W0.game, d0 = g0.doors.byName('c_n'), row0 = g0.doorDeals.find((s: any) => s.door === 'c_n'), into = K.leafCut(g0.col, d0, d0.dealt), out = K.leafCut(g0.col, d0, -d0.dealt as 1 | -1);
    DOORWAY.leafWay = false;
    line(d0.swing === d0.dealt && into >= 1 && out < 0.3 && !!row0?.clear, `off: the compound's spare room door opens the side the room rules deal it (${d0.swing > 0 ? 'into the room' : 'to the corridor'}, by '${row0?.why}') and its open leaf shuts ${into.toFixed(1)} m² of floor off from its doorway there, ${out.toFixed(1)} the other way`);
    const W1 = await K.world(true), g1 = W1.game, d1 = g1.doors.byName('c_n'), now = g1.doorSwings.find((s: any) => s.door === 'c_n'), others = g1.doors.list.filter((d: any) => d.swing !== d.dealt && d !== d1).map((d: any) => d.def.name);
    line(d1.swing === -d0.dealt && now?.why === 'leaf room' && others.length === 0, `on: it opens the other way (${d1.swing === -d0.dealt ? 'to the corridor' : 'INTO THE ROOM STILL'}: '${now?.why}', ${now?.into ?? '—'}) and no other door of the level is turned${others.length ? ` — TURNED: ${others.join(', ')}` : ''}`);
    // ---- why
    { const W = await K.world(true, { lived: false }), d = W.game.doors.byName('c_n'); line(d.swing === d.dealt && !d.clearway, `the same level with no furniture in its rooms: the spare room door is not turned (the cot is what shuts the floor off: ${K.leafCut(W.game.col, d, d.dealt).toFixed(1)} m² without it)`); }
    { const W = await K.world(true, { edit: L => { L.doors.find((x: any) => (x.id ?? x.name) === 'c_n').swing = d0.dealt; } }), d = W.game.doors.byName('c_n'), row = W.game.doorSwings.find((s: any) => s.door === 'c_n');
      line(d.swing === d0.dealt && row?.why === 'authored', `a side the level file authors is never turned: 'swing' written on that door keeps it (${row?.why})`); }
    { const g = g1, d = d1; DOORWAY.leafWay = false; d.angle = 1.84 * d.swing; d.vel = 0; d.latched = false; d.place(); const a = d.angle; DOORWAY.leafWay = !BREAK; for (let i = 0; i < 3; i++) W1.step(1 / 60); const b = d.angle;
      line(Math.abs(a) > 1.8 && Math.abs(b + a) < 0.05, `read live: a leaf standing ${f2(a)} rad open into the room when the rule comes on stands ${f2(b)} — as wide to the corridor`); void g; }
    // ---- four men out of the spare room past the standing leaf
    { const W = await K.world(true), g = W.game, d = g.doors.byName('c_n'); g.aiEnabled = true; g.godMode = true; g.playerInvisible = true; g.teleportPlayer([24.5, 0, 13.0]);
      const men = ([[29.2, 0, 17.2], [29.2, 0, 16.4], [29.25, 0, 15.6], [29.9, 0, 15.3]] as Vec3[]).map((p, i) => spawnUnit(g, { team: 'operative', pos: [...p], yaw: 0, name: ['Blue One', 'Blue Two', 'Red One', 'Red Two'][i], light: false }));
      d.angle = 1.84 * d.swing; d.vel = 0; d.latched = false; d.place(); for (let i = 0; i < 20; i++) W.step(1 / 60);
      const a0 = d.angle, spot = g.orders.freeSpot([27.6, 0, 13.0]), why = g.orders.give('move', { kind: 'floor', at: spot }, 'all'); let turned = 0, t = 0; const through = men.map(() => -1);
      for (; t < 30 && through.some((x: number) => x < 0); t += 1 / 60) { W.step(1 / 60); turned = Math.max(turned, Math.abs(d.angle - a0)); men.forEach((m: any, i: number) => { if (through[i] < 0 && m.char.pos[2] < d.def.hinge[1] - 0.5) through[i] = t; }); }
      const last = Math.max(...through), all = through.every((x: number) => x >= 0);
      line(!why && all && last <= 8, `four men stood in the spare room with its door standing open, sent down the corridor past its leaf${why ? ` (ORDER REFUSED: ${why})` : ''}: ${all ? `all through in ${last.toFixed(1)} s` : `${through.filter((x: number) => x >= 0).length} of 4 through in 30 s`} (${through.map((x: number) => x < 0 ? '—' : x.toFixed(1)).join(' / ')}), the leaf turned ${f2(turned)} rad from where it stood, last touched by ${d.lastWho} (bar: all in 8 s; the rule off, 13 s and more: the first man turns the leaf shut under him and the next pulls it open onto the rest)`); }
    // ---- kept
    { const W = await K.world(true), g = W.game, d = g.doors.byName('c_n'); d.angle = 1.7 * d.swing; d.vel = 0; d.latched = false; d.place(); const a = d.angle, s = JSON.parse(JSON.stringify(saveState(g)));
      const W2 = await K.world(true), g2 = W2.game; await K.quiet(() => loadState(g2, s)); DOORWAY.leafWay = !BREAK; const b = g2.doors.byName('c_n').angle;
      line(Math.abs(a - b) < 1e-6 && a * d0.dealt < 0, `a checkpoint keeps the leaf where it stands, open to the corridor: ${f2(a)} saved, ${f2(b)} loaded`); }
    { const g = g1, snap = () => JSON.parse(JSON.stringify(knobsSnapshot(g)));
      DOORWAY.leafWay = true; const old = snap(); delete old.tables.DOORWAY.leafWay; applyKnobs(g, old); const before = DOORWAY.leafWay;
      DOORWAY.leafWay = false; const own = snap(); own.tables.DOORWAY.leafWay = true; applyKnobs(g, own); const carried = DOORWAY.leafWay;
      line(before === false && carried === true, `a recording from before the key replays with the rule off (journal.ts applyKnobs): ${before} · one that carries it keeps its own word: ${carried}`); }
    // ---- every level
    { DOORWAY.leafWay = !BREAK; const T = await table(K);
      line(T.bad.length === 0 && T.turned.some(s => s.startsWith('compound c_n')) && T.live === T.turned.length, `${T.n} levels, ${T.doors} doors, rooms furnished: ${T.turned.length} turned for their leaf — ${T.turned.join(' · ') || 'none'}${T.kept.length ? ` · not the rule's to turn: ${T.kept.join(', ')}` : ''}${T.left.length ? ` · no better the other way: ${T.left.join(', ')}` : ''}${T.bad.length ? ` · WRONG: ${T.bad.join(' | ')}` : ''}${T.live === T.turned.length ? '' : ` · THE DEAL AS READ HANGS ${T.live} OF THEM THE OTHER WAY`}`); }
  } catch (err) { line(false, `threw: ${(err as Error)?.stack ?? err}`); }
  finally { DOORWAY.leafWay = was; DOORWAY.oneWay = wasOneWay; }
  console.log(failed ? '=== leafway-check: FAIL ===' : '=== leafway-check: ALL PASS ===');
  return !failed;
}

if (import.meta.main) {
  if (process.argv.includes('--levels')) { const K = await kit(), T = await table(K); console.log(`${T.n} levels, ${T.doors} doors\nturned: ${T.turned.join('\n        ') || 'none'}\nnot the rule's to turn: ${T.kept.join('\n        ') || 'none'}\nno better the other way: ${T.left.join('\n        ') || 'none'}\nwrong: ${T.bad.join('\n        ') || 'none'}`); process.exit(0); }
  process.exit(await leafwayCheck() ? 0 : 1);
}
