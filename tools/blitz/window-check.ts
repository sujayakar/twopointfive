// DOES ANYTHING WRITE A BOX INSIDE A FRAME'S WINDOW AND NOT SAY SO? (src/scene/collision.ts `window`, `writes`; src/host/loop.ts: a frame is one window.)
//   bun tools/blitz/window-check.ts [level.json?row …]
// Plays games with the dressing rows up (the out events and the dress step run, inside one window a step), blows a wall where the row carries charges, and
// wherever a copy shuts compares every row of it with its box: a static, a leaf of the game's own list, a part of the furniture. One that differs was
// written unsaid, and a question behind the write had a stale answer. WINDOW_BREAK=1 = a step is put on the objects without `writes`: must be red.
import { readFileSync } from 'node:fs';
process.env.TPS_GAME_LIST ??= 'page';
const R = `${import.meta.dir}/../..`;
const { standUp } = await import(`${R}/tools/qa/headless.ts`), { startFromQuery } = await import(`${R}/src/game/scenarioRun.ts`);
const { loadLevel } = await import(`${R}/src/scene/levelLoad.ts`), { parseLevelData } = await import(`${R}/src/scene/levelData.ts`);
const { LEAVES, THINGS } = await import(`${R}/src/scene/boxMirror.ts`), { DOORTOOLS } = await import(`${R}/src/game/doortools.ts`), { WALL_ORDER, wallTarget } = await import(`${R}/src/game/wallcharge.ts`);
const CR = await import(`${R}/src/game/crossing.ts`);
const ROWS = process.argv.length > 2 ? process.argv.slice(2) : ['pass.json?roomclear&squad4', 'compound.json?roomclear&squad4'];
let bad = 0;
for (const row of ROWS) {
  const [file, q] = row.split('?');
  const W: any = await standUp({ dress: true, level: loadLevel(parseLevelData(readFileSync(`${R}/levels/${file}`, 'utf8'))), seed: 3 } as any), r = startFromQuery(W.game, `?levelFile=${file}&${q}`); if (r.why) throw new Error(r.why);
  const g = W.game, M = g.col.mirror, DT = 1 / 60; g.god = true;
  const at = { shuts: 0, statics: 0, leaves: 0, things: 0, first: '' };
  const shut = M.shut; M.shut = function (this: any) { if (this.live && !this.owes) { at.shuts++;
    const s = this.stale(g.col.boxes, [], -1); at.statics += s;
    if (this.own === g.col.dynamicBoxes && this.ownAt === LEAVES.n) { const l = this.stale(g.col.boxes, g.col.dynamicBoxes, 0) - s; at.leaves += l; if (l && !at.first) at.first = `leaves at tick ${g.ticks ?? ''}`; }
    if (this.thingsAt === THINGS.n && this.thingsOk && this.thingsOf) at.things += this.staleThings(this.thingsOf);
    if (s && !at.first) at.first = 'a static'; }
    return shut.call(this); };
  if (process.env.WINDOW_BREAK) g.col.writes = (fn: () => unknown) => fn();
  const S0 = CR.RUST_SURFACE, step = (n: number) => { for (let i = 0; i < n; i++) g.col.window(() => { g.col.raycast(g.player.char.pos, [0, 1, 0], 0.5); CR.setSurface(null); try { W.step(DT); } finally { CR.setSurface(S0); } }); };
  step(240);
  const C = g.wallCharge, O = g.orders, here = [...g.player.char.pos]; let armed = 0;
  if (C?.carried != null) { C.setCarried(3);
    for (let d = 2; d < 40 && armed < 1; d += 1) for (let k = 0; k < 16 && armed < 1; k++) { const a = k * Math.PI / 8, p = [here[0] + Math.cos(a) * d, 0, here[2] + Math.sin(a) * d], t = wallTarget(g, p, { kind: 'floor', at: p }); if (t.kind !== 'wall' && !t.wall) continue;
      const n0 = C.ops.length; if (O.give(WALL_ORDER, t, 'all') || C.ops.length === n0) continue; const op = C.ops[C.ops.length - 1]; for (let i = 0; i < 60 && op.phase !== 'armed' && C.ops.includes(op); i++) step(60); if (op.phase === 'armed') armed++; }
    if (armed) { W.input.pressed.add(DOORTOOLS.blowKey); step(600); } }
  const boxesMoved = g.destr.debris.list.length, n = at.statics + at.leaves + at.things; bad += n;
  console.log(`${n ? 'FAIL' : 'ok  '} ${row}: ${at.shuts} copies shut, ${at.statics} statics, ${at.leaves} leaves, ${at.things} parts written unsaid${at.first ? ` (first: ${at.first})` : ''} · charges blown ${armed}, debris ${boxesMoved}`);
  if (!at.shuts) { bad++; console.log('FAIL no copy stood: the check looked at nothing'); }
}
console.log(bad ? 'window-check FAIL' : 'window-check PASS'); process.exit(bad ? 1 : 0);
