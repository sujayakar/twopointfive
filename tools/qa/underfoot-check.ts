// WHAT THE WALK GRID CALLS FLOOR, FEET CROSS — the check behind UNDERFOOT.on (src/scene/collision.ts; a settled rule since 2026-11-07: every tool
// stands the game with it on, and with the hole's last pass that takes a pile's bottom layer, which this check used to hold off to have his breach as his tape
// had it. Gone with the off sides: the slab's '(off: he is held at it)', 'with the rule off the same man is held at the wall', the recording and the page's flag).
//   bun run tools/qa/probes.ts underfoot        (family `furnblast`)        UNDERFOOT_BREAK=1 = every stand at chaos 0, where a charge opens nothing: must FAIL
// the maintainer, 10-17, of two wall charges on the pass (levels/pass.json): "one set of guys couldn't pathfind through the wall breach". His tape: the pair at
// the store's wall were sent through their hole and walked on the spot at the wall's line until the tape ended. The store keeps sacks to the beams against that
// wall; the blast took the pile's upper layers and left the bottom one (two sacks, top 0.2 m) lying in the hole. The walk grid's bake asks from 0.25 m up, so
// the route went straight across them; a walking body was asked from 0.2 m up, and a top AT 0.2 is in that band.
// THE SITE: his own (the store's outer wall by the floor point 31.3, 5.0, set from the yard side), on a fresh stand of the level with the page's destruction
// switches: a charge set as the order sets it and blown as F blows it. Then: the grid's own way through the hole, and a body as wide as a man walked along it
// with the walkers' own collision call (guards.ts: radius 0.3, 0.2 to 1.5 m). The level at rest: floor cells of the grid where that call is pushed at all
// (the low steps of snow banks, before the rule). And a place of the check's own, whatever a blast leaves: a slab 0.2 m
// high laid across open floor in the yard and the grid re-baked over it.
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
import { samBreach, type Breach } from './charge-rig.ts';
type Vec3 = [number, number, number];
export async function underfootCheck(): Promise<boolean> {
  const BRK = process.env.UNDERFOOT_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== underfoot: what the walk grid calls floor, feet cross${BRK ? ' (UNDERFOOT_BREAK: chaos 0, a charge opens nothing: must fail)' : ''} ===`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`); const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { BAKE_Y } = await import(`${ROOT}/src/scene/collision.ts`);
  const { FURN } = await import(`${ROOT}/src/game/furnblast.ts`); const { DESTRUCTION, FURNBLAST } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  const { WALLCHARGE, wallAt } = await import(`${ROOT}/src/game/wallcharge.ts`);
  const was = { d: DESTRUCTION.on, f: FURNBLAST.on, m: FURN.mouth.on, w: WALLCHARGE.on, o: WALLCHARGE.outer }, log = console.log;
  const data = readFileSync(`${ROOT}/levels/pass.json`, 'utf8'), R = 0.3, Y0 = 0.2, Y1 = 1.5;
  const raise = () => { DESTRUCTION.on = true; FURNBLAST.on = true; FURN.mouth.on = true; WALLCHARGE.on = true; WALLCHARGE.outer = true; };
  const stand = async () => { raise(); console.log = () => { }; try { const W: any = await standUp({ level: loadLevel(parseLevelData(data)), seed: 1 }); raise(); const g = W.game; g.godMode = true; g.aiEnabled = false; g.tune.chaos = BRK ? 0 : 0.5; for (let i = 0; i < 4; i++) W.step(1 / 60); return W; } finally { console.log = log; } };
  const top = (b: any) => b.c[1] + b.h[1], out = new Int32Array(64);
  /** floor cells of the grid where a walker's own call is pushed at all */
  const pushed = (g: any): number => { const col = g.col, nav = col.nav, cs = nav.cell; let n = 0;
    { for (let z = 0; z < nav.H; z++) for (let x = 0; x < nav.W; x++) { if (nav.blocked[nav.idx(x, z)]) continue; const px = (x + 0.5) * cs, pz = (z + 0.5) * cs, q: Vec3 = [px, 0, pz];
      if (col.collideCircle(q, R, Y0, Y1, 1) && Math.hypot(q[0] - px, q[2] - pz) > 0.005) n++; } }
    return n; };
  /** a body walked along the grid's way from A to B with the walkers' own call: how near B it ends, and how long the way was */
  const walk = (g: any, A: Vec3, B: Vec3) => { const col = g.col, path: Vec3[] | null = col.nav.findPath(A, B); if (!path) return { len: -1, end: Infinity, at: A };
    let len = Math.hypot(path[0][0] - A[0], path[0][2] - A[2]); for (let i = 1; i < path.length; i++) len += Math.hypot(path[i][0] - path[i - 1][0], path[i][2] - path[i - 1][2]);
    const p: Vec3 = [A[0], 0, A[2]]; let i = 0;
    for (let s = 0; s < 3000 && i < path.length; s++) { const w = path[i], dx = w[0] - p[0], dz = w[2] - p[2], d = Math.hypot(dx, dz); if (d < 0.15) { i++; continue; } p[0] += dx / d * 0.02; p[2] += dz / d * 0.02; col.collideCircle(p, R, Y0, Y1); }
    return { len, end: Math.hypot(p[0] - B[0], p[2] - B[2]), at: p }; };
  try {
    // (1) his breach: the store's outer wall, set from the yard
    const at: Vec3 = [31.3, 0, 5.0], from: Vec3 = [31.3, 0, 3.6], A: Vec3 = [31.25, 0, 3.75], B: Vec3 = [31.25, 0, 6.75];
    const blown = async () => { const W = await stand(), g = W.game, col = g.col, w = wallAt(g, at, from); if (!w || !w.ok) return null;
      console.log = () => { }; let res: Breach; try { res = samBreach(W, w); for (let i = 0; i < 60; i++) W.step(1 / 60); } finally { console.log = log; }   // (set by Sam's own hands from the yard, set off by his own action: charge-rig.ts)
      const k = col.circleTouches(at[0], at[2] + 0.5, 0.6, Y0, Y1, out);
      const left = [...out.slice(0, Math.max(0, k))].filter(i => top(col.boxes[i]) < BAKE_Y).map(i => top(col.boxes[i]).toFixed(2));
      return { W, g, hole: !!res.hole, breach: res.why, left, ...walk(g, A, B) }; };
    const on = await blown();
    if (!on) verdict(false, "the store's outer wall takes a charge where his was set", 'no wall place there: the level changed, and this check needs its site again');
    else {
      verdict(on.hole && on.len > 0 && on.len < 5, "his breach is a hole, and the walk grid goes straight through it", `the charge ${on.hole ? 'opened a hole' : 'did NOT open a hole'}; the grid's way from the yard to the store's floor is ${on.len < 0 ? 'NONE' : on.len.toFixed(1) + ' m'} (3 m apart in a line)`);
      verdict(on.end < 0.35, 'a man walked along that way gets through the hole', `ends ${on.end.toFixed(2)} m from the far point, at (${on.at[0].toFixed(2)}, ${on.at[2].toFixed(2)}) — the wall's line is z 5.0, the far point z 6.75. Lying in the hole under the bake's band: ${on.left.length ? on.left.length + ' thing(s), top ' + on.left.join(', ') + ' m' : 'nothing (the blast took it all: this line asks less than it did)'}`);
      // (2) the level at rest, and a slab of the check's own across open floor
      const rest = await stand(), rg = rest.game, n = pushed(rg);
      verdict(n === 0, 'on the pass at rest no floor cell of the walk grid pushes a walker', `${n} cell(s) of the grid's floor where a man's own collision call is pushed`);
      { const col = rg.col, nav = col.nav, like = col.boxes.find((b: any) => b.flags === 0 && !b.rot), C: Vec3 = [31.25, 0, 3.75], A2: Vec3 = [31.25, 0, 3.0], B2: Vec3 = [31.25, 0, 4.4];
        const open0 = [A2, C, B2].every(p => !nav.isBlocked(p[0], p[2])); col.addStatic({ ...like, c: [C[0], 0.1, C[2]], h: [2.5, 0.1, 0.15], yaw: 0 }); nav.rebake(C[0] - 2.5, C[2] - 0.15, C[0] + 2.5, C[2] + 0.15);
        const floor = !nav.isBlocked(C[0], C[2]), w = walk(rg, A2, B2);
        verdict(open0 && floor && w.end < 0.35, 'a slab 0.2 m high laid across open floor: the grid still calls it floor, and a man walked along the grid\'s way crosses it', `open before ${open0}; floor to the grid after the re-bake ${floor}; ends ${w.end.toFixed(2)} m from the far point`); }
    }
  } finally { console.log = log; DESTRUCTION.on = was.d; FURNBLAST.on = was.f; FURN.mouth.on = was.m; WALLCHARGE.on = was.w; WALLCHARGE.outer = was.o; }
  return all;
}
