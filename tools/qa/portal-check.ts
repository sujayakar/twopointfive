// DAYLIGHT COUNTED ONCE: the check behind a PORTAL (render/lights.ts LIGHT_PORTAL; common.wgsl evalLightDirect, portalFlux, skyKeep; direct_common.wgsl
// shadowSample; rc.wgsl rcDice; scene/openings.ts ajarOf; engine.ts lookOpeningLights; the look panel's "Daylight counted once" row, lookConfig.portal). The look's page only: no game
// table, nothing for a recording.
//   bun run tools/qa/probes.ts portal      (family `render`)        PORTAL_BREAK=1 = the bounce's rays and the dice keep the sky a portal sends: must FAIL
// The maintainer, 11-04, of the disk light at each opening beside the bounce's own share of the same daylight: "this feels like tweaking the importance
// sampling for restir, right?" and "my preference would be that we use this sampling unconditionally and then have no asymmetry between night and day".
// WHAT IS ASKED. In a room built by hand (a doorway, a wall across the yard outside it), at points of its floor, its walls and a crate's top, with the
// CPU's own ray against the room's boxes:
//   the bounce's share   the sky a cosine-weighted hemisphere of rays meets when it meets nothing (what the gather and the cache took before)
//   the portal's         the shader's arithmetic in this file's words: evalLightDirect's sum over the rectangle's points × the share of shadowSample's
//                        rays that reach sky, each aimed at one of the points in proportion to what it sends
//   what is left         the same hemisphere under skyKeep: none of the sky a ray left the room by
// The portal's must be the bounce's share to within the sum's own error, what is left must be nothing under the roof and everything outside.
// WHAT THIS CANNOT SAY. The arithmetic here is a second writing of the shaders', held to them by their text; whether the picture is right is a GPU's to
// show. Never seen on a GPU.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
type V3 = [number, number, number];
export async function portalCheck(): Promise<boolean> {
  const BRK = process.env.PORTAL_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== portal: daylight counted once${BRK ? ' (the bounce keeps the sky it leaves by: must fail)' : ''} ===`);
  const { rayBox, BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`), { planOpenings, openAreas, openingLooks, LOOK_OPENINGS } = await import(`${ROOT}/src/scene/openings.ts`);
  const { LIGHT_PORTAL, packLight } = await import(`${ROOT}/src/render/lights.ts`);
  const text = (f: string) => readFileSync(resolve(ROOT, f), 'utf8');

  // ---- the room: 6 by 5 m under a roof at 3 m, a doorway 0.9 by 2.05 m in its east wall, a 3 m wall across the yard 3 m outside it
  const T = 0.12, B: any[] = [], box = (c: number[], h: number[], flags = 0) => { B.push({ c, h, yaw: 0, flags }); };
  box([13, -0.1, 12.5], [9, 0.1, 6]);                                                   // the ground
  box([10, 1.5, 12.5], [T, 1.5, 2.5 + T]); box([13, 1.5, 10], [3 + T, 1.5, T]); box([13, 1.5, 15], [3 + T, 1.5, T]);   // west, south and north walls
  box([16, 1.5, 11], [T, 1.5, 1]); box([16, 1.5, 13.95], [T, 1.5, 1.05]); box([16, 2.525, 12.45], [T, 0.475, 0.45]);     // the east wall either side of the doorway, and its lintel
  box([13, 3.1, 12.5], [3 + T, 0.1, 2.5 + T]);                                          // the roof
  box([19, 1.5, 12.5], [T, 1.5, 3.5]);                                                  // the wall across the yard
  const plan = planOpenings(B) as { pos: V3; mid: V3; dir: V3; half: [number, number]; full: number }[], o = plan[0];
  const ZEN = 0.12, HOR = 1.0;   // (a sky much brighter low down than overhead: a light that took one radiance for every direction would be far off)
  // how near the portal's must be to the reference: a fifth at any point, and a third a hand from the sill, where a point of the sum stands for a patch as wide as the distance
  const within = (name: string, k: number) => name.includes('sill') ? k > 0.66 && k < 1.34 : k > 0.8 && k < 1.2;
  const skyL = (y: number) => { const t = Math.min(1, Math.max(0, y * 2.5)), below = Math.min(1, Math.max(0, -y * 8)), up = HOR + (ZEN - HOR) * t; return up + (HOR * 0.15 - up) * below; };   // common.wgsl skyRadiance, by luminance
  // common.wgsl portalOutside, by luminance: the sky above the horizon and THE GROUND (`G`) under it. G is 0 through the lines that ask whether the
  // sky is counted once, and the record's own in the line that asks of the ground
  let G = 0; const outside = (y: number) => y > 0 ? HOR + (ZEN - HOR) * Math.min(1, y * 2.5) : G;
  const hits = (ro: V3, rd: V3, tmax: number) => { for (const b of B) if (rayBox(ro, rd, b, tmax)) return true; return false; };
  const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]], dot = (a: V3, b: V3) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2], len = (a: V3) => Math.hypot(a[0], a[1], a[2]);
  const PORTAL_SKY = 40, PORTAL_OUT = 1, range = 26, win = (d: number) => { const x = d / range, w = Math.max(0, Math.min(1, 1 - x * x * x * x)); return w * w; };   // common.wgsl lightWindow
  // common.wgsl portalPast: how far past its point a portal's shadow ray runs (up: to the sky; down: PORTAL_OUT out from the plane, or to the ground)
  const past = (Po: V3, qd: V3, ql: number) => qd[1] > 0 ? PORTAL_SKY : Math.min(PORTAL_OUT / Math.max(-dot(qd, o.dir), 0.25), Math.max((Po[1] - 0.02) / Math.max(-qd[1], 1e-4) - ql, 0));
  // common.wgsl skyThrough: does a ray from O along d leave by the opening's rectangle
  const through = (O: V3, d: V3) => { const dn = dot(d, o.dir); if (dn > -1e-4) return false; const t = dot(sub(o.mid, O), o.dir) / dn; if (t <= 0) return false;
    const q: V3 = [O[0] + d[0] * t - o.mid[0], O[1] + d[1] * t - o.mid[1], O[2] + d[2] * t - o.mid[2]]; return Math.abs(q[1]) <= o.half[1] && q[0] * q[0] + q[2] * q[2] <= o.half[0] * o.half[0]; };
  // the bounce's share and what is left of it: π × the mean over a cosine-weighted hemisphere (a 64 by 64 lattice) of the sky met by a ray that meets nothing
  const bounce = (P: V3, N: V3) => { const up: V3 = Math.abs(N[1]) > 0.9 ? [1, 0, 0] : [0, 1, 0], Tn: V3 = [up[1] * N[2] - up[2] * N[1], up[2] * N[0] - up[0] * N[2], up[0] * N[1] - up[1] * N[0]], tl = len(Tn); Tn[0] /= tl; Tn[1] /= tl; Tn[2] /= tl;
    const Bn: V3 = [N[1] * Tn[2] - N[2] * Tn[1], N[2] * Tn[0] - N[0] * Tn[2], N[0] * Tn[1] - N[1] * Tn[0]], Po: V3 = [P[0] + N[0] * 0.01, P[1] + N[1] * 0.01, P[2] + N[2] * 0.01]; let whole = 0, left = 0; const n = 64;
    for (let i = 0; i < n; i++) for (let j = 0; j < n; j++) { const r = Math.sqrt((i + 0.5) / n), a = (j + 0.5) / n * 2 * Math.PI, x = r * Math.cos(a), y = r * Math.sin(a), z = Math.sqrt(Math.max(0, 1 - r * r));
      const d: V3 = [Tn[0] * x + Bn[0] * y + N[0] * z, Tn[1] * x + Bn[1] * y + N[1] * z, Tn[2] * x + Bn[2] * y + N[2] * z]; if (hits(Po, d, 200)) continue;
      const L = skyL(d[1]); whole += L; left += L * (BRK || !through(Po, d) ? 1 : 1 - win(len(sub(Po, o.mid)))); }   // (skyThrough: what the portal's own window leaves of it)
    return { whole: Math.PI * whole / (n * n), left: Math.PI * left / (n * n) }; };
  // THE REFERENCE inside the room, where the doorway is the only way to the sky: the same light added up over the doorway itself, 48 by 48 points of it
  // each with its own ray (the hemisphere above has a few dozen rays in a doorway seen from across the room, which is no measure to hold a fifth against)
  const skyBy = (P: V3, N: V3) => { const Po: V3 = [P[0] + N[0] * 0.01, P[1] + N[1] * 0.01, P[2] + N[2] * 0.01], n = 48; let s = 0;
    for (let i = 0; i < n; i++) for (let j = 0; j < n; j++) { const q = sub([o.mid[0] + o.dir[2] * ((i + 0.5) / n * 2 - 1) * o.half[0], o.mid[1] + ((j + 0.5) / n * 2 - 1) * o.half[1], o.mid[2] - o.dir[0] * ((i + 0.5) / n * 2 - 1) * o.half[0]], Po), d2 = dot(q, q), ql = Math.sqrt(d2), qd: V3 = [q[0] / ql, q[1] / ql, q[2] / ql];
      const w = outside(qd[1]) * Math.max(-dot(qd, o.dir), 0) * Math.max(dot(N, qd), 0) / d2; if (w > 0 && !hits(Po, qd, qd[1] > 0 ? 200 : ql - 0.03 + past(Po, qd, ql))) s += w; }
    return s * 4 * o.half[0] * o.half[1] / (n * n); };
  // the portal's, as the shaders have it. common.wgsl portalPoint: what one point of the opening sends, per m² and per unit of the sky's hue (the distance
  // squared held off by portalSoft, a point's own patch over 2π); portalAt: the lattice of PORTAL_POINTS points; portalFlux: their mean
  const SUN = { intensity: 3, dir: [0.3, -0.8, 0.52], color: [1, 1, 1] }, ref = openingLooks([ZEN, ZEN, ZEN], [HOR, HOR, HOR], 1, SUN)[0], groundOf = 2 * ref - (LOOK_OPENINGS.up * ZEN + (1 - LOOK_OPENINGS.up) * HOR), area = 4 * o.half[0] * o.half[1], radius = Math.hypot(o.half[0], o.half[1]), POINTS = 8, soft = o.half[0] * o.half[1] * 2 / (Math.PI * POINTS);
  const fr = (x: number) => x - Math.floor(x), along: V3 = [o.dir[2], 0, -o.dir[0]];
  const latt = (i: number, u: number, v: number): [number, number] => [fr((i + 0.5) * 0.618034 + 0.5 + u) * 2 - 1, fr((i + 0.5) / POINTS + v) * 2 - 1];
  const to = (d: V3, a: [number, number]): V3 => [d[0] + along[0] * a[0] * o.half[0], d[1] + a[1] * o.half[1], d[2] + along[2] * a[0] * o.half[0]];
  const point = (d: V3, N: V3) => { const d2 = dot(d, d), dl = Math.sqrt(d2), L: V3 = [d[0] / dl, d[1] / dl, d[2] / dl]; return outside(L[1]) * Math.max(-dot(L, o.dir), 0) * Math.max(dot(N, L), 0) / (d2 + soft); };
  const flux = (P: V3, N: V3) => { const d = sub(o.mid, P); let s = 0; for (let i = 0; i < POINTS; i++) s += point(to(d, latt(i, 0, 0)), N); return s / POINTS; };
  // evalLightDirect's figure (U: the whole rectangle, nothing in the way) and shadowSample's rays over a lattice of pixels, two rays a pixel as restir
  // has them: each ray aimed at one of the points moved round by its two numbers, picked by what the point sends, run on PORTAL_SKY past it; yes or no.
  //   E = what a pixel of the picture gets · plain = what a pass whose ray stops short gets (evalLightRaw's area light as bright as what is open: the
  //   cache, a bounce's hit off the screen, the meter) · vis = the share of the rays that reach sky
  const portal = (P: V3, N: V3, openShare = 1) => { const d = sub(o.mid, P), dist = len(d); if (dist > range) return { E: 0, U: 0, plain: 0, vis: 0 };
    const Po: V3 = [P[0] + N[0] * 0.01, P[1] + N[1] * 0.01, P[2] + N[2] * 0.01], U = area * win(dist) * flux(P, N), toC = sub(o.mid, Po); let yes = 0, all = 0; const n = 24;
    if (U > 0) for (let i = 0; i < n; i++) for (let j = 0; j < n; j++) for (let k = 0; k < 2; k++) { const ur = fr((i + 0.5) / n + k * 0.618034), ua = fr((j + 0.37) / n + k * 0.7548777), w: number[] = []; let sum = 0;
      for (let q = 0; q < POINTS; q++) { w[q] = point(to(toC, latt(q, ur, ua)), N); sum += w[q]; }
      all++; if (sum <= 0) continue; let x = fr(ur * 61 + ua * 37 + 0.5) * sum, sel = POINTS - 1; for (let q = 0; q < POINTS; q++) { x -= w[q]; if (x < 0) { sel = q; break; } }
      const q = to(toC, latt(sel, ur, ua)), ql = len(q), qd: V3 = [q[0] / ql, q[1] / ql, q[2] / ql]; if (dot(qd, N) > 0 && !hits(Po, qd, ql - 0.03 + past(Po, qd, ql))) yes++; }
    const L: V3 = [d[0] / dist, d[1] / dist, d[2] / dist], ce = -dot(L, o.dir), c0 = Math.max(dot(N, L), 0);
    const plain = ce > 0 && c0 > 0 && !hits(Po, L, dist - 0.02) ? ref * area * openShare * (win(dist) / Math.max(dist * dist, radius * radius)) * ce * c0 : 0;
    return { E: all ? U * yes / all : 0, U, plain, vis: all ? yes / all : 0 }; };

  // 1. the opening the room's walls leave, and the rectangle a portal is given
  verdict(plan.length === 1 && o.dir[0] === -1 && Math.abs(o.mid[2] - 12.45) < 0.06 && Math.abs(o.mid[1] - 1.025) < 0.08 && o.half[0] >= 0.45 && o.half[0] < 0.53 && o.half[1] >= 0.97 && o.half[1] < 1.08,
    'a doorway is one opening, facing into the room, and its rectangle holds all that the walls leave open and little else',
    `${plan.length} opening${plan.length === 1 ? '' : 's'} · at (${o.mid.map(v => v.toFixed(2)).join(', ')}) facing (${o.dir.join(', ')}) · half ${o.half[0].toFixed(3)} along the wall, ${o.half[1].toFixed(3)} up it (the doorway: 0.45, 1.025) · ${o.full.toFixed(2)} m² open`);

  const at4: { name: string; P: V3; N: V3 }[] = [{ name: 'floor a hand from the sill', P: [15.58, 0, 12.45], N: [0, 1, 0] }, { name: 'floor 1 m in', P: [15, 0, 12.45], N: [0, 1, 0] }, { name: 'floor 2.5 m in', P: [13.5, 0, 12.45], N: [0, 1, 0] }, { name: 'floor 4.5 m in', P: [11.5, 0, 12.45], N: [0, 1, 0] },
    { name: 'a crate\'s top at 1.2 m', P: [14.4, 1.2, 12.45], N: [0, 1, 0] }, { name: 'a crate\'s top at 1.6 m', P: [14.4, 1.6, 12.45], N: [0, 1, 0] }, { name: 'the far wall', P: [10 + T, 1.2, 12.45], N: [1, 0, 0] }, { name: 'a side wall', P: [14, 1.2, 15 - T], N: [0, 0, -1] }];
  // 2. the same light, carried once: at points of the room the portal's is the bounce's share, and under skyKeep the bounce has none of it left
  { const at: { name: string; P: V3; N: V3 }[] = [
      { name: 'floor a hand from the sill', P: [15.58, 0, 12.45], N: [0, 1, 0] }, { name: 'floor 1 m in', P: [15, 0, 12.45], N: [0, 1, 0] }, { name: 'floor 2.5 m in', P: [13.5, 0, 12.45], N: [0, 1, 0] }, { name: 'floor 4.5 m in', P: [11.5, 0, 12.45], N: [0, 1, 0] },
      { name: 'floor off to one side', P: [13.5, 0, 14], N: [0, 1, 0] }, { name: 'a crate\'s top at 1.2 m', P: [14.4, 1.2, 12.45], N: [0, 1, 0] }, { name: 'the far wall at 1.2 m', P: [10 + T, 1.2, 12.45], N: [1, 0, 0] }, { name: 'a side wall at 1.2 m', P: [14, 1.2, 15 - T], N: [0, 0, -1] } ];
    const rows = at.map(a => ({ ...a, b: { whole: skyBy(a.P, a.N), left: bounce(a.P, a.N).left }, p: portal(a.P, a.N) }));
    // (a point that sees no sky gets none either way: the far wall looks straight out at the wall across the yard)
    const near = (r: typeof rows[number]) => r.b.whole < 1e-4 ? r.p.E < 1e-4 : within(r.name, r.p.E / r.b.whole), sum = (f: (r: typeof rows[number]) => number) => rows.reduce((s, r) => s + f(r), 0);
    const sumB = sum(r => r.b.whole), sumP = sum(r => r.p.E), sumL = sum(r => r.b.left), sumO = sum(r => r.p.plain), lit = rows.filter(r => r.b.whole >= 1e-4).length;
    verdict(lit >= 7 && rows.every(near) && sumP > 0.85 * sumB && sumP < 1.15 * sumB && sumL < 0.02 * sumB,
      'the sky through a doorway is counted once: the portal sends each point what the bounce\'s rays used to bring it, to a fifth from 1 m inside the sill to the far wall (a third a hand from it), and a ray of the bounce that leaves by the doorway brings none',
      rows.map(r => `${r.name}: bounce ${r.b.whole.toFixed(4)}, portal ${r.p.E.toFixed(4)}${r.b.whole >= 1e-4 ? ` (${(r.p.E / r.b.whole).toFixed(2)} ×)` : ''}, ${(r.p.vis * 100).toFixed(0)} % of its rays reach sky, left ${r.b.left.toFixed(4)}`).join(' · ') + ` · all of them: portal ${(sumP / sumB).toFixed(2)} × the bounce's share, ${(100 * sumL / sumB).toFixed(1)} % of it left to the bounce · a pass whose ray stops short of the opening (the cache, the meter) gets ${(sumO / sumB).toFixed(2)} × over them: the yard's wall is not in its light`); }

  // 2b. the cache's probes give up the same sky (rc.wgsl rcDice): on each of a probe's six faces, the portal's own sum with nothing in the way, and
  // never more than the sky that came to that face. What is left of the sky in the dice is then next to nothing, with the yard open or walled
  { const probes: V3[] = [[14, 1, 12.45], [12, 1, 13.5], [15.2, 0.5, 12.2]], faces: V3[] = [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]];
    const dice = () => { let came = 0, left = 0; for (const P of probes) for (const N of faces) { const Sk = skyBy(P, N), Pk = BRK ? 0 : area * win(len(sub(o.mid, P))) * flux(P, N); came += Sk; left += Sk - Math.min(Sk, Pk); } return { came, left }; };
    const walled = dice(), wall = B.pop(), free = dice(); B.push(wall);
    verdict(walled.came > 0 && free.came > walled.came && walled.left < 0.1 * walled.came && free.left < 0.15 * free.came,
      'the cache gives up the same sky: at three probes, a face at a time, the portal\'s own sum takes out all but a little of the sky that came by the doorway, and never more than came',
      `behind the yard's wall: ${(100 * walled.left / walled.came).toFixed(1)} % of the probes' sky left in the dice · the yard open: ${(100 * free.left / free.came).toFixed(1)} %`); }

  // 3. what a portal does not touch: the yard keeps its whole sky, and a point behind the opening gets nothing from it
  { const yard = bounce([17.5, 0, 12.45], [0, 1, 0]), behind = portal([17.5, 0, 12.45], [0, 1, 0]), wallOut = bounce([16 + T, 1.2, 11], [1, 0, 0]);
    verdict(yard.left === yard.whole && yard.whole > 0.1 && wallOut.left === wallOut.whole && behind.E === 0 && behind.plain === 0,
      'outside nothing changes: ground in the yard and the wall\'s outer face keep every ray\'s sky, and the portal sends nothing behind itself',
      `yard ground ${yard.whole.toFixed(4)} whole, ${yard.left.toFixed(4)} under the rule · the outer face ${wallOut.whole.toFixed(4)} and ${wallOut.left.toFixed(4)} · the portal on the yard ${behind.E}`); }

  // 4. a wall across the yard is in the light: with it taken away the portal's rays reach more sky (the disk it replaces could not know)
  { const P: V3 = [13.5, 0, 12.45], N: V3 = [0, 1, 0], withWall = portal(P, N), wall = B.pop(), free = at4.map(a => ({ ...a, b: { whole: skyBy(a.P, a.N) }, p: portal(a.P, a.N) })), without = portal(P, N); B.push(wall);
    const ok = free.every(r => within(r.name, r.p.E / r.b.whole));
    verdict(without.vis > withWall.vis + 0.1 && without.E > withWall.E * 1.15 && ok,
      'what stands outside is in the light: the wall across the yard takes sky from the portal, as it took it from the bounce, and with the yard open the portal is the bounce\'s share again',
      `2.5 m in: ${(withWall.vis * 100).toFixed(0)} % of the portal's rays reach sky with the yard wall, ${(without.vis * 100).toFixed(0)} % without · portal ${withWall.E.toFixed(4)} and ${without.E.toFixed(4)} · the yard open: ` + free.map(r => `${r.name} ${(r.p.E / r.b.whole).toFixed(2)} ×`).join(', ')); }

  // 4a. THE GROUND OUTSIDE is in the light too (common.wgsl portalGround, portalOutside): under the horizon a point looks out on the ground, as bright as
  // the record says (twice its reference less the low sky). A wall across the room looks out level, on the ground and on next to no sky: with the sky
  // alone it would be all but dark by day. Against the same sum over the doorway with a ray a point, each run as far as the shader's (portalPast)
  { const wallB = B.pop(), at: { name: string; P: V3; N: V3 }[] = [{ name: 'the far wall at 1.5 m', P: [10 + T, 1.5, 12.45], N: [1, 0, 0] }, { name: 'the ceiling 2 m in', P: [14, 3 - 0.001, 12.45], N: [0, -1, 0] }, { name: 'floor 2.5 m in', P: [13.5, 0, 12.45], N: [0, 1, 0] }];
    const sky = at.map(a => portal(a.P, a.N).E); G = groundOf; const both = at.map(a => ({ E: portal(a.P, a.N).E, truth: skyBy(a.P, a.N) })); G = 0; B.push(wallB);
    const ok = both.every((r, i) => r.truth > 0 && r.E > 0.8 * r.truth && r.E < 1.2 * r.truth) && both[0].E > 1.3 * sky[0] && both[1].E > 0 && sky[1] === 0 && both[2].E < 1.15 * sky[2];
    verdict(groundOf > 0 && ok, 'the ground outside is in the portal\'s light: a wall that looks out level gets half as much again as the sky gave it with the yard open, a ceiling gets it where the sky gave nothing, a floor gets none of it, and each is the sum over the doorway to a fifth',
      `the record's ground ${groundOf.toFixed(3)} against a horizon of ${HOR} (a sun of ${SUN.intensity} on ground that sends back ${LOOK_OPENINGS.ground}) · ` + at.map((a, i) => `${a.name}: the sky alone ${sky[i].toFixed(4)}, with the ground ${both[i].E.toFixed(4)} (${(both[i].E / both[i].truth).toFixed(2)} × the sum)`).join(' · ')); }

  // 4b. what is open of a doorway a leaf hangs in (scene/openings.ts ajarOf): shut is shut whatever stands beside the door, and the gap grows with the swing
  { const leafAt = (a: number) => ({ c: [16 - 0.42 * Math.sin(a), 1.0, 12.03 + 0.42 * Math.cos(a)], h: [0.025, 1.0, 0.42], yaw: -a, flags: BoxFlag.Dynamic });   // (0.84 m of leaf hinged at the doorway's low end, swinging into the room)
    const open = (boxes: any[], a: number) => { const pl = planOpenings(boxes) as any[]; if (!pl.length) return -1; openAreas(pl, [leafAt(a)]); return pl[0].ajar as number; };
    const crate = { c: [16.42, 0.6, 11.75], h: [0.3, 0.6, 0.3], yaw: 0, flags: 0 }, cabinet = { c: [15.68, 0.9, 13.1], h: [0.2, 0.9, 0.25], yaw: 0, flags: 0 };   // (one outside against the wall by the jamb, one inside)
    const rooms: [string, any[]][] = [['the bare wall', B], ['a crate outside by the jamb, listed before the walls', [crate, ...B]], ['a cabinet inside by the other, listed first', [cabinet, ...B]]];
    const rows = rooms.map(([name, boxes]) => ({ name, shut: open(boxes, 0), a: [0.3, 0.6, 1.0, 1.5].map(x => open(boxes, x)) })), full = plan[0].full;
    const play = LOOK_OPENINGS.play; LOOK_OPENINGS.play = 0; const noPlay = open(B, 0); LOOK_OPENINGS.play = play;
    verdict(rows.every(r => r.shut === 0 && r.a[0] > 0 && r.a[0] < 0.2 * full && r.a.every((v, i) => i === 0 || v > r.a[i - 1]) && r.a[3] > 0.75 * full) && noPlay > 0 && noPlay < 0.2,
      'a shut leaf is shut and an open one open, by the narrowest way through: whatever stands by the door, the jambs are the wall\'s; the gap grows with the swing; a leaf swung wide leaves the doorway all but its own thickness',
      rows.map(r => `${r.name}: shut ${r.shut.toFixed(3)}, at 0.3, 0.6, 1.0 and 1.5 rad ${r.a.map(v => v.toFixed(2)).join(', ')} of ${full.toFixed(2)} m²`).join(' · ') + ` · with no play allowed the shut leaf reads ${noPlay.toFixed(3)} m² (the two gaps at its jambs)`); }

  // 5. the record and the shaders' text: one bit, one function for the sky, every ray of the bounce through it
  { const f = new Float32Array(16), u = new Uint32Array(f.buffer), l: any = { pos: [1, 2, 3], dir: [1, 0, 0], color: [1, 1, 1], range: 9, innerDeg: 0, outerDeg: 90, radius: 1.2, portal: { halfWide: 0.47, share: 0.25 } };
    packLight(f, u, 0, l, 2, 3 | LIGHT_PORTAL, 0, 0); const plain = new Float32Array(16); packLight(plain, new Uint32Array(plain.buffer), 0, { ...l, portal: undefined }, 2, 3, 0, 0);
    const common = text('src/render/shaders/common.wgsl'), dc = text('src/render/shaders/direct_common.wgsl'), fg = text('src/render/shaders/fgather.wgsl'), gs = text('src/render/shaders/gather_split.wgsl'), cf = text('src/render/shaders/compact/fgather.wgsl'), rc = text('src/render/shaders/rc.wgsl');
    const count = (s: string, w: string) => s.split(w).length - 1, gc = text('src/render/shaders/gather_common.wgsl');
    const pins: [string, boolean][] = [
      ['the record holds the rectangle and the share of it that is open where a cone was', Math.abs(f[11] - 0.47) < 1e-6 && Math.abs(f[12] - 0.25) < 1e-6 && (u[3] >>> 0) === ((3 | LIGHT_PORTAL) >>> 0) && Math.abs(plain[11] - Math.cos(Math.PI / 2)) < 1e-6],
      ['the bit is the shader\'s', LIGHT_PORTAL === 0x10000000 && common.includes('const LIGHT_PORTAL: u32 = 0x10000000u;')],
      ['a portal sends what the receiver looks out on (the sky by the function a ray that misses asks, the ground by its record), added up over its points', common.includes('return vec4f(L, max(dot(-L, li.dir), 0.0) / (d2 + soft));') && common.includes('return portalOutside(r.xyz, ground) * (r.w * select(max(dot(N, r.xyz), 0.0), 1.0, all(N == vec3f(0.0))));')
        && common.includes('fn portalOutside(L: vec3f, ground: vec3f) -> vec3f { if (L.y > 0.0) { return skyRadiance(L); } return ground; }')
        && common.includes('return min(PORTAL_OUT / max(-dot(dir, li.dir), 0.25), max((Po.y - 0.02) / max(-dir.y, 1e-4) - dist, 0.0));')
        && common.includes('fn portalGround(li: Light) -> vec3f { return max(li.color * (2.0 / max(li.cosInner * portalArea(li), 1e-9)) - mix(frame.skyHorizon, frame.skyZenith, PORTAL_UP), vec3f(0.0)); }') && common.includes(`const PORTAL_UP: f32 = ${LOOK_OPENINGS.up};`) && common.includes('const PORTAL_OUT: f32 = 1.0;') && common.includes('const PORTAL_POINTS: i32 = 8;')
        && common.includes('fn portalAt(i: i32, u: vec2f) -> vec2f { let k = f32(i) + 0.5; return fract(vec2f(k * 0.618034 + 0.5, k / f32(PORTAL_POINTS)) + u) * 2.0 - 1.0; }')
        && common.includes('fn portalSoft(li: Light, high: f32) -> f32 { return li.cosOuter * high * (2.0 / (3.14159265 * f32(PORTAL_POINTS))); }')
        && common.includes('r.E = portalFlux(li, P, N) * (portalArea(li) * lightWindow(dist, li.range)); r.ok = any(r.E > vec3f(0.0));') && common.includes('fn portalArea(li: Light) -> f32 { return 4.0 * li.cosOuter * portalHigh(li); }')
        && common.includes('return mix(mix(frame.skyHorizon, frame.skyZenith, t), frame.skyHorizon * 0.15, below);') && !common.includes('fn skyLum(')],
      ['the light evaluation every pass pastes knows nothing of a portal (the final gather has no stack for it), nor do the passes whose ray stops short', !/lightPortal|PORTAL|portal/.test(common.slice(common.indexOf('fn evalLightRaw('), common.indexOf('fn directIrradiance(') + 2600).split('\n// ---- torch bounce cards')[0])
        && [gc, text('src/render/shaders/compact/gather_common.wgsl'), text('src/render/shaders/probe.wgsl')].every(t => !/portal|PORTAL/.test(t))],
      ['its shadow ray is aimed by what each point sends, runs on to the sky or clear of the opening, and answers yes or no', dc.includes('tmax = lightDist - 0.03 + past;') && dc.includes('w[i] = luminance(portalPoint(li, toC + T * (at.x * li.cosOuter) + B * (at.y * hh), N, soft, ground)); sum += w[i]; }') && dc.includes('past = portalPast(li, Po, d / dl, dl);')
        && dc.includes('var x = fract(ur * 61.0 + ua * 37.0 + 0.5) * sum;') && dc.includes('for (var i = 0; i < PORTAL_POINTS; i++) { x -= w[i]; if (x < 0.0) { sel = i; break; } }') && !/skyK\b|PORTAL_HEAD/.test(dc + common)
        && dc.includes('var d = (li.pos + T * (cos(a) * r) + B * (sin(a) * r)) - Po; var past = 0.0;')],   // (… and any other light's ray is written as it was, to the last rounding)
      ['restir weighs a portal by its whole rectangle from frame to frame', common.includes('fn lightPower(li: Light) -> f32 { return luminance(li.color) / select(1.0, li.cosInner, lightPortal(li)); }') && ['restir', 'compact/restir'].every(f => text(`src/render/shaders/${f}.wgsl`).includes('let kNow = lightPower(wgLights[curT[c]]); let kPrev = lightPower(lp);'))],
      ['a blocker past the opening hands on the opening\'s own size', count(dc, 'select(li.radius * dOcc / max(lightDist - dOcc, 0.05), li.radius, dOcc >= lightDist)') === 2],
      ['the direct estimators take the whole rectangle, and no other pass does', ['restir', 'direct', 'sharp_direct', 'compact/restir', 'compact/direct', 'compact/sharp_direct'].every(f => { const t = text(`src/render/shaders/${f}.wgsl`); return count(t, 'evalLightDirect(') >= 2 && !/\bevalLight\(/.test(t); }) && count(fg + gs + cf + rc + gc + text('src/render/shaders/probe.wgsl'), 'evalLightDirect(') === 0],
      ['the bounce keeps what a portal does not send, and walks the list from its first portal', common.includes('for (var i = u32(frame.placeCfg.y) - 1u; i < frame.numLights; i++) {') && common.includes('keep = 1.0 - lightWindow(distance(O, lights[i].pos), lights[i].range); break;') && common.includes('fn skyThrough(O: vec3f, dir: vec3f) -> vec3f { return skyRadiance(dir) * skyKeep(O, dir); }')],
      ['every diffuse ray of the gather asks which sky is its own', count(fg, 'L = skyThrough(Po, dir);') === 3 && count(fg, 'L = skyRadiance(dir);') === 1 && count(gs, 'L = skyThrough(Po, dir);') === 1 && cf.includes('if (gatherRay) { L = skyThrough(ro, dir); } else { L = skyRadiance(dir); }')],
      ['and so does the cache, at the probe the light is for: the cascades carry how much of a direction\'s light is the sky\'s, and the dice bake takes a portal\'s own sum out of it, by the share that is open and never more than the sky that came; the place field is handed back what left the face it reads, through the steady bounce\'s ring too', rc.includes('let sky = skyRadiance(dir); return vec4f(r.xyz + sky, luminance(sky));') && rc.includes('return vec4f(r.xyz + up.xyz, up.w); }')
        && rc.includes('outBuf[probeLin * nOut + o] = packRad(sum * 0.25);') && rc.includes('outBuf[probeLin * DD + d] = packRad(L);') && count(rc, 'skyThrough(') === 0 && count(rc, 'skyKeep(') === 0
        && rc.includes('if (!lightPortal(li) || !lightReaches(i, P)) { continue; }') && rc.includes('let k0 = portalArea(li) * li.cosInner * lightWindow(dist, li.range);') && rc.includes('for (var f = 0; f < 7; f++) { let p = k0 * F[f]; V[f] += hue * p; T[f] += p; }')
        && common.includes('let k = luminance(skyRadiance(r.xyz)) * r.w / f32(PORTAL_POINTS);') && common.includes('s[0] += k * max(r.x, 0.0); s[1] += k * max(-r.x, 0.0); s[2] += k * max(r.y, 0.0); s[3] += k * max(-r.y, 0.0); s[4] += k * max(r.z, 0.0); s[5] += k * max(-r.z, 0.0); s[6] += k; }')
        && rc.includes('gave = luminance(floorLight - E2);') && rc.includes('mix(was.w, gave, k)') && text('src/render/shaders/place_field.wgsl').includes('vec3f(0.2126, 0.7152, 0.0722)) + tap.w;') && text('src/render/shaders/dice_ring.wgsl').includes('textureStore(outDice, gid, sum / max(got, 1.0));')
        && rc.includes('E0 = max(E0 - V[0] * min(1.0, S[0] / max(T[0], 1e-9)), vec3f(0.0));') && rc.includes('mean = max(mean - V[6] * min(1.0, S[6] / max(T[6], 1e-9)), vec3f(0.0));')],
      ['the table still knows where an opening looks', LOOK_OPENINGS.up > 0 && LOOK_OPENINGS.up < 1 && (BoxFlag.Glass | 0) !== 0] ];
    verdict(pins.every(p => p[1]), 'the light\'s record and the shaders say the same rule', pins.map(p => `${p[1] ? 'ok' : 'NOT'}: ${p[0]}`).join(' · ')); }
  console.log(all ? 'portal: all passed' : 'portal: FAILED');
  return all;
}
if (import.meta.main) process.exit(await portalCheck() ? 0 : 1);
