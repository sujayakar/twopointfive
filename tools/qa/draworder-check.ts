// THE STATICS ARE DRAWN FROM THE TOP DOWN, AND NO FACE CHANGES HANDS: the check behind scene/boxes.ts BoxWorld's drawRank and rankStatics
// (docs/agents/render.md has the rule and what was timed).
//   bun run tools/qa/probes.ts draworder        (family `render`)
//   DRAWORDER_BREAK=1   on each level two boxes that share a face's plane have their places swapped behind the world's back: must FAIL
//   DRAWORDER_ALL=1     every level file, not the six below (two minutes a stand)
// Why: the g-buffer drew the static boxes in the list's order, so a ground slab laid first under a whole level was shaded over the whole screen and then
// drawn over by the floor. Under the look that slab is a built surface, and the pass cost a third more wherever the camera stood. The camera looks down,
// so what stands higher is nearly always nearer: drawn tops falling, what lies under a thing fails the depth test before it is shaded.
// But where two faces lie in one plane the depth test keeps the FIRST drawn, and that was always the earlier box in the list: a plinth written before
// the piers it runs across keeps its colour over their feet, and a look's ground sheets, laid a fiftieth of a millimetre apart, show in the order they
// were written. Tops falling alone would hand thousands of such patches to the taller box. So a box waits for every earlier box it shares a face with.
// WHAT THIS CAN AND CANNOT SAY. No GPU runs here. It reads the list the g-buffer is handed (BoxWorld.visible) on levels stood up headless and finds the
// shared faces a second way: every face of every drawn box against every other (no sorting), two faces sharing a plane when they look the same way and
// lie within a quarter of a millimetre along it, and the patch they share cut out exactly (one rectangle clipped by the other), where the world asks
// their bounds. Any turn, any tilt. Whether a GPU gives two faces in one plane the same depth at every pixel is not asked: where it does not, the pair
// fights whatever the order, as it did. What the order saves is a GPU's to time.
import { readFileSync, readdirSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
// the default stand-up (it has tilted things) · plinths across piers' feet · posts in walls' ends · boxes turned 45 degrees · a look's stepped sheets · the most statics
const LEVELS = ['office', 'white-roof', 'police-post', 'block-gwot', 'records-gwot', 'barracks-gwot'];
type V = [number, number, number];
export async function drawOrderCheck(): Promise<boolean> {
  const BRK = process.env.DRAWORDER_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== draworder: the statics are drawn from the top down, and no face changes hands${BRK ? ' (two tied boxes swapped: must fail)' : ''} ===`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`); const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`); const { quat } = await import(`${ROOT}/src/math/vec.ts`);
  const levels = process.env.DRAWORDER_ALL === '1' ? ['office', ...readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.endsWith('.lint.json') && f !== 'office.json').map(f => f.slice(0, -5))] : LEVELS;
  const DT = 1 / 60, f = Math.fround, PLANE = 0.00025, PATCH = 0.0005;
  const dot = (a: V, b: V) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  const axesOf = (b: any): [V, V, V] => b.rot ? [quat.rotate(b.rot, [1, 0, 0]), quat.rotate(b.rot, [0, 1, 0]), quat.rotate(b.rot, [0, 0, 1])] : [[Math.cos(b.yaw), 0, -Math.sin(b.yaw)], [0, 1, 0], [Math.sin(b.yaw), 0, Math.cos(b.yaw)]];
  type Face = { i: number; n: V; d: number; m: V; e1: V; e2: V; h1: number; h2: number; lo: V; hi: V };
  const facesOf = (i: number, b: any): Face[] => { const A = axesOf(b), out: Face[] = [];
    for (let u = 0; u < 3; u++) for (const s of [1, -1]) { const n: V = [A[u][0] * s, A[u][1] * s, A[u][2] * s]; if (n[1] < -0.5) continue;
      const m: V = [b.c[0] + n[0] * b.h[u], b.c[1] + n[1] * b.h[u], b.c[2] + n[2] * b.h[u]]; const e1 = A[(u + 1) % 3], e2 = A[(u + 2) % 3], h1 = b.h[(u + 1) % 3], h2 = b.h[(u + 2) % 3], ex = [0, 1, 2].map(k => Math.abs(e1[k]) * h1 + Math.abs(e2[k]) * h2);
      out.push({ i, n, d: dot(n, m), m, e1, e2, h1, h2, lo: [m[0] - ex[0], m[1] - ex[1], m[2] - ex[2]], hi: [m[0] + ex[0], m[1] + ex[1], m[2] + ex[2]] }); }
    return out; };
  // the patch two faces in one plane share, cut out exactly: B's rectangle clipped to A's, in A's own two directions; its width each way
  const shared = (A: Face, B: Face): [number, number] => { let P: [number, number][] = [];
    for (const [u, v] of [[1, 1], [-1, 1], [-1, -1], [1, -1]]) { const p: V = [B.m[0] + B.e1[0] * B.h1 * u + B.e2[0] * B.h2 * v - A.m[0], B.m[1] + B.e1[1] * B.h1 * u + B.e2[1] * B.h2 * v - A.m[1], B.m[2] + B.e1[2] * B.h1 * u + B.e2[2] * B.h2 * v - A.m[2]]; P.push([dot(p, A.e1), dot(p, A.e2)]); }
    for (const [ax, lim, sg] of [[0, A.h1, 1], [0, A.h1, -1], [1, A.h2, 1], [1, A.h2, -1]] as const) { const Q: [number, number][] = [];
      for (let k = 0; k < P.length; k++) { const p = P[k], q = P[(k + 1) % P.length], ip = p[ax] * sg <= lim, iq = q[ax] * sg <= lim; if (ip) Q.push(p); if (ip !== iq) { const t = (lim - p[ax] * sg) / ((q[ax] - p[ax]) * sg); Q.push([p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t]); } }
      P = Q; if (P.length < 3) return [0, 0]; }
    return [Math.max(...P.map(p => p[0])) - Math.min(...P.map(p => p[0])), Math.max(...P.map(p => p[1])) - Math.min(...P.map(p => p[1]))]; };
  const rows: string[] = [], shape: string[] = [], bad: string[] = [], free: string[] = [], late: string[] = []; let W0: any = null, pair0: [number, number] | null = null, tall0 = -1;
  for (const name of levels) {
    const W: any = name === 'office' ? await standUp() : await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8'))) });
    for (let i = 0; i < 3; i++) W.step(DT);
    const w = W.game.engine.world, n = w.statics.length as number, B = w.all as any[]; if (name === 'office') W0 = W;
    const drawn0 = Array.from((w.visible as Uint32Array).subarray(0, w.visibleCount)).filter(v => v < n).sort((a, b) => a - b);
    const isDrawn = new Set(drawn0), F: Face[] = []; for (let i = 0; i < n; i++) if (!(B[i].flags & (1 | (1 << 17)))) F.push(...facesOf(i, B[i]));   // (every static in the picture, the cut pass's too: the world asks them all; 1 = NoPrimary, 1 << 17 = Glass)
    const pairs: [number, number][] = [], waitsWide = new Set<number>();
    for (let a = 0; a < F.length; a++) for (let c = a + 1; c < F.length; c++) { const A = F[a], C = F[c]; if (A.i === C.i) continue; const dd = Math.abs(A.d - C.d); if (dd > 0.001 || dot(A.n, C.n) < 1 - 1e-6) continue;
      if ([0, 1, 2].every(k => Math.min(A.hi[k], C.hi[k]) - Math.max(A.lo[k], C.lo[k]) > -0.001)) waitsWide.add(C.i);   // (the generous finding, for the third line: within a millimetre of one plane, their bounds within a millimetre of touching)
      const [s1, s2] = shared(A, C);
      if (dd <= PLANE && dot(A.n, C.n) >= 1 - 1e-9 && s1 > PATCH && s2 > PATCH && isDrawn.has(A.i) && isDrawn.has(C.i)) pairs.push([A.i, C.i]); }
    if (name === 'office') { pair0 = pairs.find(([a, b]) => B[b].c[1] + B[b].h[1] > B[a].c[1] + B[a].h[1] + 0.5 && !B[a].rot) ?? null; tall0 = drawn0.find(i => !waitsWide.has(i) && !B[i].rot && B[i].c[1] + B[i].h[1] > 1.5 && B[i].c[1] - B[i].h[1] < 0.5) ?? -1; }
    if (BRK && pairs.length) { const r = w.drawRank as Uint32Array, [a, b] = pairs[pairs.length >> 1]; const t = r[a]; r[a] = r[b]; r[b] = t; W.step(DT); }
    const vis = Array.from((w.visible as Uint32Array).subarray(0, w.visibleCount)), st = vis.filter(v => v < n), place = new Map(st.map((v, k) => [v, k] as const));
    if (new Set(vis).size !== vis.length) shape.push(`${name}: a word is drawn twice`);
    if (vis.slice(0, st.length).some((v, k) => v !== st[k])) shape.push(`${name}: the drawn statics are not the head of the list`);
    const dyn = vis.slice(st.length).map(v => v & 0xffff); if (dyn.some((v, k) => k > 0 && v <= dyn[k - 1])) shape.push(`${name}: the dynamics are not in the list's own order behind them`);
    // 2 · no face changes hands
    const wrong = pairs.filter(([a, b]) => place.get(a)! > place.get(b)!);
    if (wrong.length) bad.push(`${name}: ${wrong.length} of ${pairs.length} shared faces would be won by the later box, the first boxes ${wrong[0][0]} and ${wrong[0][1]}`);
    // 3 · a box that shares no plane with an earlier one, by the generous finding, is in its top's order among its like
    const top = (b: any) => f(f(b.c[1]) + f(b.rot ? (w.constructor as any).heightExtent(b) : b.h[1]));
    const loose = st.filter(i => !waitsWide.has(i)); let out = 0;
    for (let k = 1; k < loose.length; k++) { const ta = top(B[loose[k - 1]]), tb = top(B[loose[k]]); if (tb > ta || (tb === ta && loose[k] < loose[k - 1])) out++; }
    if (out) free.push(`${name}: ${out} of ${loose.length} boxes that wait for nothing out of their tops' order`);
    // 4 · what it is for: the widest static (the ground under the level) comes after what stands above it
    let wide = st[0]; for (const i of st) if (B[i].h[0] * B[i].h[2] > B[wide].h[0] * B[wide].h[2]) wide = i;
    const above = st.filter(i => top(B[i]) > top(B[wide])), after = above.filter(i => place.get(i)! > place.get(wide)!);
    if (after.length > 0.05 * above.length) late.push(`${name}: ${after.length} of the ${above.length} statics above the widest one (box ${wide}) are drawn after it`);
    rows.push(`${name}: ${st.length} drawn, ${pairs.length} shared faces, ${loose.length} wait for nothing, ground after ${above.length - after.length} of ${above.length} (the world made ${w.drawTies} waits)`);
  }
  verdict(shape.length === 0, 'the drawn statics lead the list, each once, and the dynamics follow in the list\'s own order as ever', shape.join('; ') || `${levels.length} levels`);
  verdict(bad.length === 0, 'no face changes hands: of every two drawn boxes with faces that look the same way within a quarter of a millimetre of one plane, sharing a patch wider than half a millimetre each way, the earlier in the list is drawn first', bad.join('; ') || rows.join(' · '));
  verdict(free.length === 0, 'the boxes that share no plane with an earlier box (found generously: within a millimetre of one plane, their bounds within a millimetre of touching) are drawn tops falling, the list\'s own order between equal tops', free.join('; ') || 'on every level');
  verdict(late.length === 0, 'the widest static, the ground under a level, is drawn after at least nineteen in twenty of the statics that stand above it', late.join('; ') || 'on every level');
  { // THE ORDER IS MADE AGAIN WHEN IT SHOULD BE, AND ONLY THEN. A tall box that waits for a low earlier one (office): each change to the low box that frees the tall
    // one must show in the list the same frame, as an order made fresh would have it; paint and the flags the order does not read must make nothing
    const W = W0, w = W.game.engine.world, B = w.all as any[], ff: string[] = [], did: string[] = []; const read = () => { const n = w.statics.length as number; return Array.from((w.visible as Uint32Array).subarray(0, w.visibleCount)).filter(v => v < n); };
    if (BRK) { w.drawRankN = -1; W.step(DT); }   // (the swap above is forgotten: these lines ask about making the order again, not about the swap)
    if (!pair0) ff.push('the office has no tall box waiting for a low earlier one to ask with');
    else { const [lo, hi] = pair0, b = B[lo], keep = { c: b.c, h: b.h, yaw: b.yaw, rot: b.rot, flags: b.flags, cutHeight: b.cutHeight }; const base = read(), at0 = base.indexOf(hi);
      const same = (x: number[], y: number[]) => x.length === y.length && x.every((v, k) => v === y[k]);
      const ask = (what: string, change: () => void, undo: () => void = () => {}) => { change(); W.step(DT); const got = read(); w.drawRankN = -1; W.step(DT); const fresh = read();
        if (!same(got, fresh)) ff.push(`${what}: the list is not the one an order made fresh gives`); else if (same(fresh, base)) ff.push(`${what}: changed nothing, so it asked nothing`); else did.push(what);
        b.c = keep.c; b.h = keep.h; b.yaw = keep.yaw; b.rot = keep.rot; b.flags = keep.flags; b.cutHeight = keep.cutHeight; undo(); W.step(DT); const back = read(); if (!same(back, base)) ff.push(`${what}: put back, the order is not the one it was`); };
      ask('moved aside', () => { b.c = [b.c[0] + 300, b.c[1], b.c[2] + 300]; });
      ask('raised', () => { b.c = [b.c[0], b.c[1] + 50, b.c[2]]; });
      ask('made taller', () => { b.h = [b.h[0], b.h[1] + 40, b.h[2]]; });
      ask('turned', () => { b.yaw = keep.yaw + 0.3; });
      ask('tilted', () => { b.rot = quat.axisAngle([1, 0, 0], 0.3); });
      ask('taken out of the picture', () => { b.flags = keep.flags | 1; });   // (1 = BoxFlag.NoPrimary)
      if (tall0 < 0) ff.push('the office has no tall box that waits for nothing to cut down');
      else { const t = B[tall0]; ask('cut down to a lip', () => { t.flags |= 4; t.cutHeight = t.c[1] - t.h[1] + 0.01; }, () => { t.flags &= ~4; t.cutHeight = 1e6; }); }   // (4 = BoxFlag.Cutaway, on a tall box that waits for nothing: drawn as a lip, its top is among the lowest)
      const n0 = w.statics.length as number, extra = { ...B[hi], c: [B[hi].c[0], B[hi].c[1] + 60, B[hi].c[2]], marks: undefined, name: undefined }; w.addStatic(extra); W.step(DT); const grown = read(); w.drawRankN = -1; W.step(DT);
      if (!same(grown, read()) || grown[0] !== n0) ff.push(`a static added high above is at place ${grown.indexOf(n0) + 1}, or the list is not the fresh one`); else did.push('one added');
      w.statics.pop(); w.staticsDirty = true; W.step(DT); if (!same(read(), base)) ff.push('the added static taken away: the order is not the one it was');
      const r0 = w.drawRanked as number, a0 = b.albedo; b.albedo = [a0[0] * 0.5, a0[1], a0[2]]; W.step(DT); b.albedo = a0; b.flags = keep.flags ^ 128; W.step(DT); b.flags = keep.flags ^ 2; W.step(DT); b.flags = keep.flags; W.step(DT);   // (128 = Gloss, as a wet floor is given; 2 = NoShadow)
      for (let i = 0; i < 300; i++) W.step(DT);
      if ((w.drawRanked as number) !== r0) ff.push(`paint, a floor's gloss, the shadow flag and five seconds of the shift made the order ${(w.drawRanked as number) - r0} times`);
      did.push(`box ${hi} waits for box ${lo}, place ${at0 + 1} of ${base.length}`); }
    verdict(ff.length === 0, 'the order is made again the same frame when a static is moved aside, raised, made taller, turned, tilted, taken out of the picture, cut down or added, each as an order made fresh would have it and back as it was after; paint, a floor\'s gloss, the shadow flag and five seconds of the shift make nothing',
      ff.join('; ') || did.join(' · ')); }
  console.log(`=== draworder: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await drawOrderCheck()) ? 0 : 1);
