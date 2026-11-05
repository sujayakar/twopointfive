// DAYLIGHT COUNTED ONCE: the check behind a PORTAL (render/lights.ts LIGHT_PORTAL; common.wgsl evalLightDirect, portalFlux, skyKeep; direct_common.wgsl
// shadowSample; rc.wgsl rcDice; scene/openings.ts ajarOf; engine.ts lookOpeningLights; the only way an opening is lit since 11-04, when the "Daylight counted once" row was folded in). The look's page only: no game
// table, nothing for a recording.
//   bun run tools/qa/probes.ts portal      (family `render`)        PORTAL_BREAK=1 = the bounce's rays and the dice keep the sky a portal sends: must FAIL
// The maintainer, 11-04, of the disk light at each opening beside the bounce's own share of the same daylight: "this feels like tweaking the importance
// sampling for restir, right?" and "my preference would be that we use this sampling unconditionally and then have no asymmetry between night and day".
// WHAT IS ASKED. In a room built by hand (a doorway, a wall across the yard outside it), at points of its floor, its walls and a crate's top, with the
// CPU's own ray against the room's boxes:
//   the bounce's share   the sky a cosine-weighted hemisphere of rays meets when it meets nothing (what the gather and the cache took before)
//   the portal's         the shader's arithmetic in this file's words: evalLightDirect's sum over the rectangle's pieces × the share of shadowSample's
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
  const BRK = process.env.PORTAL_BREAK === '1'; let all = true;   // (PORTALDEEP_BREAK=1, PORTALRIMS_BREAK=1 and PORTALBANDS_BREAK=1, read where the light's sum is mirrored below: the light is its whole rectangle, as it was, or has a rim at both ends whatever stands there, or is the mean of eight points, and the line on the hole's sides or on its rims must fail)
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== portal: daylight counted once${BRK ? ' (the bounce keeps the sky it leaves by: must fail)' : ''} ===`);
  const { rayBox, BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`), { planOpenings, openAreas, openingLooks, LOOK_OPENINGS } = await import(`${ROOT}/src/scene/openings.ts`);
  const { LIGHT_PORTAL, packLight, portalHoleBits } = await import(`${ROOT}/src/render/lights.ts`);
  const { WAY } = await import(`${ROOT}/src/scene/wayThrough.ts`);
  const text = (f: string) => readFileSync(resolve(ROOT, f), 'utf8');

  // ---- the room: 6 by 5 m under a roof at 3 m, a doorway 0.9 by 2.05 m in its east wall, a 3 m wall across the yard 3 m outside it
  const T = 0.12, B: any[] = [], box = (c: number[], h: number[], flags = 0) => { B.push({ c, h, yaw: 0, flags }); };
  box([13, -0.1, 12.5], [9, 0.1, 6]);                                                   // the ground
  box([10, 1.5, 12.5], [T, 1.5, 2.5 + T]); box([13, 1.5, 10], [3 + T, 1.5, T]); box([13, 1.5, 15], [3 + T, 1.5, T]);   // west, south and north walls
  box([16, 1.5, 11], [T, 1.5, 1]); box([16, 1.5, 13.95], [T, 1.5, 1.05]); box([16, 2.525, 12.45], [T, 0.475, 0.45]);     // the east wall either side of the doorway, and its lintel
  box([13, 3.1, 12.5], [3 + T, 0.1, 2.5 + T]);                                          // the roof
  box([19, 1.5, 12.5], [T, 1.5, 3.5]);                                                  // the wall across the yard
  type Hole = { pos: V3; mid: V3; dir: V3; half: [number, number]; full: number; deep: number; rims: number };
  const plan = planOpenings(B) as Hole[]; let o = plan[0];   // (`o`: the opening every helper below asks of; the line on a long strip of glass puts its own in its place for a while)
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
  const skyBy = (P: V3, N: V3, n = 48) => { const Po: V3 = [P[0] + N[0] * 0.01, P[1] + N[1] * 0.01, P[2] + N[2] * 0.01]; let s = 0;   // (n: points a side; 48 is plenty from a metre off, and a face a hand from the opening asks for more)
    for (let i = 0; i < n; i++) for (let j = 0; j < n; j++) { const q = sub([o.mid[0] + o.dir[2] * ((i + 0.5) / n * 2 - 1) * o.half[0], o.mid[1] + ((j + 0.5) / n * 2 - 1) * o.half[1], o.mid[2] - o.dir[0] * ((i + 0.5) / n * 2 - 1) * o.half[0]], Po), d2 = dot(q, q), ql = Math.sqrt(d2), qd: V3 = [q[0] / ql, q[1] / ql, q[2] / ql];
      const sill = qd[1] < 0 && ((Po[1] - 0.02) / -qd[1] - ql) * -dot(qd, o.dir) < o.deep;   // (a way down that meets the floor in the wall's own depth: the threshold, which is floor the bounce lights and not the ground a portal stands for)
      const w = outside(qd[1]) * Math.max(-dot(qd, o.dir), 0) * Math.max(dot(N, qd), 0) / d2; if (w > 0 && !sill && !hits(Po, qd, qd[1] > 0 ? 200 : ql - 0.03 + past(Po, qd, ql))) s += w; }
    return s * 4 * o.half[0] * o.half[1] / (n * n); };
  // the portal's, as the shaders have it. common.wgsl portalPoint: what one point of the opening sends, per m² and per unit of the sky's hue (the distance
  // squared held off by portalSoft, a point's own patch over 2π); portalAt: the lattice of PORTAL_POINTS points; portalFlux: their mean
  const SUN = { intensity: 3, dir: [0.3, -0.8, 0.52], color: [1, 1, 1] }, ref = openingLooks([ZEN, ZEN, ZEN], [HOR, HOR, HOR], 1, SUN)[0], groundOf = 2 * ref - (LOOK_OPENINGS.up * ZEN + (1 - LOOK_OPENINGS.up) * HOR), POINTS = 8;
  let area = 0, radius = 0, soft = 0, along: V3 = [0, 0, 0];   // (the opening's own figures, taken again whenever another opening is asked of)
  const use = (h: Hole) => { o = h; area = 4 * o.half[0] * o.half[1]; radius = Math.hypot(o.half[0], o.half[1]); soft = o.half[0] * o.half[1] * 2 / (Math.PI * POINTS); along = [o.dir[2], 0, -o.dir[0]]; }; use(o);
  const fr = (x: number) => x - Math.floor(x);
  const latt = (i: number, u: number, v: number): [number, number] => [fr((i + 0.5) * 0.618034 + 0.5 + u) * 2 - 1, fr((i + 0.5) / POINTS + v) * 2 - 1];
  const to = (d: V3, a: [number, number]): V3 => [d[0] + along[0] * a[0] * o.half[0], d[1] + a[1] * o.half[1], d[2] + along[2] * a[0] * o.half[0]];
  const point = (d: V3, N: V3, sf = soft) => { const d2 = dot(d, d), dl = Math.sqrt(d2), L: V3 = [d[0] / dl, d[1] / dl, d[2] / dl]; return outside(L[1]) * Math.max(-dot(L, o.dir), 0) * Math.max(dot(N, L), 0) / (d2 + sf); };
  // common.wgsl portalSeen: the part of the rectangle a point sees the outside through, past the hole's far rim `deep` further out (the middle and the half
  // sizes, in half widths and half heights). `whole` = the light as it was until 11-04, all of its rectangle (PORTALDEEP_BREAK=1 asks every line of it)
  let whole = process.env.PORTALDEEP_BREAK === '1', walled = process.env.PORTALRIMS_BREAK === '1';   // (walled: a far rim at both ends, whatever the opening says: PORTALRIMS_BREAK=1, and the line on the rims must fail)
  const RIM = [0.025, 0.05];   // common.wgsl PORTAL_RIM: the far rim is taken half a sample of the walls wider than the rectangle
  const seen = (d: V3): [number, number, number, number] => { const pz = -dot(d, o.dir); if (whole || !(o.deep > 0) || !(pz > 1e-4)) return [0, 0, 1, 1];
    const px = d[2] * o.dir[0] - d[0] * o.dir[2], py = -d[1], k = pz / (pz + o.deep), hw = o.half[0], hh = o.half[1];
    const rim = walled ? 3 : o.rims, fx = (hw + RIM[0]) * k, fy = (hh + RIM[1]) * k, lx = rim & 1 ? Math.max(px * (1 - k) - fx, -hw) : -hw, hx = rim & 2 ? Math.min(px * (1 - k) + fx, hw) : hw, ly = Math.max(py * (1 - k) - fy, -hh), hy = Math.min(py * (1 - k) + fy, hh);
    return hx <= lx || hy <= ly ? [0, 0, 0, 0] : [(lx + hx) / 2 / hw, (ly + hy) / 2 / hh, (hx - lx) / 2 / hw, (hy - ly) / 2 / hh]; };
  const into = (see: [number, number, number, number], a: [number, number]): [number, number] => [see[0] + a[0] * see[2], see[1] + a[1] * see[3]];
  // common.wgsl portalEdge, portalQuad, portalPiece, portalCorners, portalEnds, portalMean, portalFlux: THE OPENING'S LIGHT ADDED UP PIECE BY PIECE. The
  // part a point sees is cut into PIECES that are each nearly even: the ground's, below the point's own height, and the sky above it in two halves
  // along the wall (by angle from the point), each cut where the sky's ramp ends over its middle. A piece, cut at the face's own horizon, is the sum
  // over its edges of half the angle the edge subtends about its plane's normal (a polygon's vector irradiance; the angle over its sine by the
  // shader's fit). A sky piece counts as bright as the mean of its foot's and its head's brightness weighed by how the face takes each, read where
  // the part the face sees lies. `points` = the mean of the eight points it was until 11-04 (PORTALBANDS_BREAK=1 asks every line of it)
  const PIECES = 5, RAMP = 0.4364, points = process.env.PORTALBANDS_BREAK === '1', sky = (y: number) => { const up = HOR + (ZEN - HOR) * Math.min(1, Math.max(y, 0) * 2.5); return up + (HOR * 0.15 - up) * Math.min(1, Math.max(-y * 8, 0)); };   // (sky: common.wgsl skyRadiance, by luminance)
  const crs = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]], unit = (a: V3): V3 => { const l = len(a); return [a[0] / l, a[1] / l, a[2] / l]; }, add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
  const edge = (a: V3, b: V3): V3 => { const x = dot(a, b), y = Math.abs(x), v = (0.8543985 + (0.4965155 + 0.0145206 * y) * y) / (3.4175940 + (4.1616724 + y) * y), c = crs(a, b), k = Math.PI * (x > 0 ? v : 0.5 / Math.max(len(c), 1e-7) - v); return [c[0] * k, c[1] * k, c[2] * k]; };
  const quad = (c: V3[], N: V3): V3 => { let F: V3 = [0, 0, 0], under: V3 = [0, 0, 0], back: V3 = [0, 0, 0], cuts = 0;
    for (let i = 0; i < 4; i++) { const a = c[i], b = c[(i + 1) & 3], da = dot(a, N), db = dot(b, N), cut = (): V3 => { const k = da / (da - db); return unit([a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k, a[2] + (b[2] - a[2]) * k]); };
      if (da >= 0 && db >= 0) F = add(F, edge(a, b)); else if (da >= 0) { under = cut(); F = add(F, edge(a, under)); cuts++; } else if (db >= 0) { back = cut(); F = add(F, edge(back, b)); cuts++; } }
    return cuts === 2 ? add(F, edge(under, back)) : F; };
  // (a piece's sides: its two ends along the wall from the point's foot on the plane, its foot and head from the point's own height)
  const piece = (d: V3, see: [number, number, number, number], i: number): [number, number, number, number] => { const pz = -dot(d, o.dir), px = dot(d, along);
    const X0 = (see[0] - see[2]) * o.half[0] + px, X1 = (see[0] + see[2]) * o.half[0] + px, y0 = (see[1] - see[3]) * o.half[1] + d[1], y1 = (see[1] + see[3]) * o.half[1] + d[1];
    if (i === 0) return [X0, X1, y0, Math.min(y1, 0)];
    const r0 = Math.hypot(X0, pz), r1 = Math.hypot(X1, pz), Xs = (X0 * r1 + X1 * r0) / (r0 + r1), rs = Math.hypot(Xs, pz), [xa, xb, ra, rb] = i > 2 ? [Xs, X1, rs, r1] : [X0, Xs, r0, rs];
    const lo = Math.max(y0, 0), cut = Math.min(Math.max(RAMP * Math.hypot((xa * rb + xb * ra) / (ra + rb), pz), lo), y1);
    return (i & 1) === 1 ? [xa, xb, lo, cut] : [xa, xb, cut, y1]; };
  const from = (pz: number, x: number, y: number): V3 => [-o.dir[0] * pz + along[0] * x, y, -o.dir[2] * pz + along[2] * x];
  const corners = (q: number[], pz: number): V3[] => [unit(from(pz, q[0], q[2])), unit(from(pz, q[0], q[3])), unit(from(pz, q[1], q[3])), unit(from(pz, q[1], q[2]))];
  const ends = (q: number[], pz: number, F: V3): [V3, V3] => { const fz = -dot(F, o.dir), x = Math.min(Math.max(fz > 1e-9 ? pz * dot(F, along) / fz : 0.5 * (q[0] + q[1]), q[0]), q[1]); return [unit(from(pz, x, q[2])), unit(from(pz, x, q[3]))]; };
  const meanL = (L0: number, L1: number, d0: number, d1: number) => { if (d0 <= 0 && d1 <= 0) return d1 >= d0 ? L1 : L0; let a = L0, b = L1, wa = d0, wb = d1;
    if (d0 < 0) { a = L0 + (L1 - L0) * (d0 / (d0 - d1)); wa = 0; } if (d1 < 0) { b = L1 + (L0 - L1) * (d1 / (d1 - d0)); wb = 0; } return (a * (2 * wa + wb) + b * (wa + 2 * wb)) / (3 * (wa + wb)); };
  const flux = (P: V3, N: V3) => { const d = sub(o.mid, P), see = seen(d), part = see[2] * see[3], pz = -dot(d, o.dir); if (!(part > 0) || !(pz > 1e-4)) return 0;
    if (points) { let s = 0; for (let i = 0; i < POINTS; i++) s += point(to(d, into(see, latt(i, 0, 0))), N, soft * part); return s * part / POINTS; }
    let s = 0;
    for (let i = 0; i < PIECES; i++) { const q = piece(d, see, i); if (!(q[3] > q[2]) || !(q[1] > q[0])) continue; const F = quad(corners(q, pz), N), w = dot(F, N); if (!(w > 0)) continue;
      if (i === 0) { s += G * w; continue; }
      const e = ends(q, pz, F); s += meanL(sky(e[0][1]), sky(e[1][1]), dot(e[0], N), dot(e[1], N)) * w; }
    return s / area; };
  // common.wgsl portalAbove: the part of the seen rectangle above a face's horizon, as the smallest rectangle round it (what a shadow ray's points are
  // laid in). `flat` = the whole seen part, as the rays had it until the second day (PORTALBANDS_BREAK=1 too)
  const above = (d: V3, see: [number, number, number, number], N: V3): [number, number, number, number] => { if (points) return see; const hw = o.half[0], hh = o.half[1], lo = [(see[0] - see[2]) * hw, (see[1] - see[3]) * hh], hi = [(see[0] + see[2]) * hw, (see[1] + see[3]) * hh];
    let mn = [1e9, 1e9], mx = [-1e9, -1e9]; const cor = (i: number) => [i >= 2 ? hi[0] : lo[0], i === 1 || i === 2 ? hi[1] : lo[1]], dn = (c: number[]) => dot([d[0] + along[0] * c[0], d[1] + c[1], d[2] + along[2] * c[0]], N), take = (c: number[]) => { mn = [Math.min(mn[0], c[0]), Math.min(mn[1], c[1])]; mx = [Math.max(mx[0], c[0]), Math.max(mx[1], c[1])]; };
    for (let i = 0; i < 4; i++) { const a = cor(i), b = cor((i + 1) & 3), da = dn(a), db = dn(b); if (da >= 0) take(a); if ((da >= 0) !== (db >= 0)) { const k = da / (da - db); take([a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k]); } }
    return mx[0] <= mn[0] || mx[1] <= mn[1] ? [0, 0, 0, 0] : [(mn[0] + mx[0]) / 2 / hw, (mn[1] + mx[1]) / 2 / hh, (mx[0] - mn[0]) / 2 / hw, (mx[1] - mn[1]) / 2 / hh]; };
  // common.wgsl portalSolid and portalFaces: the sky's part of the sum on a cube's six faces and with no face (the dice), per m² of the opening
  const solid = (q: number[], pz: number) => { const A = (X: number, Y: number) => Math.atan2(X * Y, pz * Math.sqrt(X * X + Y * Y + pz * pz)); return Math.max(A(q[1], q[3]) - A(q[0], q[3]) - A(q[1], q[2]) + A(q[0], q[2]), 0); };
  const faces = (P: V3): number[] => { const s = [0, 0, 0, 0, 0, 0, 0], d = sub(o.mid, P), see = seen(d), pz = -dot(d, o.dir); if (!(see[2] * see[3] > 0) || !(pz > 1e-4)) return s;
    for (let i = 0; i < PIECES; i++) { const q = piece(d, see, i); if (!(q[3] > q[2]) || !(q[1] > q[0])) continue; const c = corners(q, pz);
      for (let f = 0; f < 6; f++) { const n: V3 = [0, 0, 0]; n[f >> 1] = 1 - 2 * (f & 1); const F = quad(c, n), w = dot(F, n); if (!(w > 0)) continue; const e = ends(q, pz, F), L0 = sky(e[0][1]), L1 = i === 0 ? L0 : sky(e[1][1]); s[f] += meanL(L0, L1, dot(e[0], n), dot(e[1], n)) * w / area; }
      const m = ends(q, pz, [0, 0, 0]), M0 = sky(m[0][1]); s[6] += 0.5 * (M0 + (i === 0 ? M0 : sky(m[1][1]))) * solid(q, pz) / area; }
    return s; };
  // evalLightDirect's figure (U: what of the rectangle shows the outside, nothing in the way) and shadowSample's rays over a lattice of pixels, two rays a pixel as restir
  // has them: each ray aimed at one of the points moved round by its two numbers, picked by what the point sends, run on PORTAL_SKY past it; yes or no.
  //   E = what a pixel of the picture gets · plain = what a pass whose ray stops short gets (evalLightRaw's area light as bright as what is open: the
  //   cache, a bounce's hit off the screen, the meter) · vis = the share of the rays that reach sky
  const portal = (P: V3, N: V3, openShare = 1) => { const d = sub(o.mid, P), dist = len(d); if (dist > range) return { E: 0, U: 0, plain: 0, vis: 0 };
    const Po: V3 = [P[0] + N[0] * 0.01, P[1] + N[1] * 0.01, P[2] + N[2] * 0.01], U = area * win(dist) * flux(P, N), toC = sub(o.mid, Po), see = above(toC, seen(toC), N), part = see[2] * see[3]; let yes = 0, all = 0; const n = 48;   // (48 by 48 pixels: at 24 a point that 4 % of the rays reach sky from read 0.78 × one way and 0.97 × the other by the count alone)
    if (U > 0 && part > 0) for (let i = 0; i < n; i++) for (let j = 0; j < n; j++) for (let k = 0; k < 2; k++) { const ur = fr((i + 0.5) / n + k * 0.618034), ua = fr((j + 0.37) / n + k * 0.7548777), w: number[] = []; let sum = 0;
      for (let q = 0; q < POINTS; q++) { w[q] = point(to(toC, into(see, latt(q, ur, ua))), N, soft * part); sum += w[q]; }
      all++; if (sum <= 0) continue; let x = fr(ur * 61 + ua * 37 + 0.5) * sum, sel = POINTS - 1; for (let q = 0; q < POINTS; q++) { x -= w[q]; if (x < 0) { sel = q; break; } }
      const q = to(toC, into(see, latt(sel, ur, ua))), ql = len(q), qd: V3 = [q[0] / ql, q[1] / ql, q[2] / ql]; if (dot(qd, N) > 0 && !hits(Po, qd, ql - 0.03 + past(Po, qd, ql))) yes++; }
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

  // 4z. THE HOLE'S OWN SIDES ARE IN THE LIGHT, NOT IN A RAY'S LUCK (common.wgsl portalSeen, 11-04; the maintainer on his GPU: "boiling in the sun on the
  // ground through the door, especially along left edge"). A doorway's daylight fans over the floor, and the fan's sides are the wall the doorway is
  // cut in: from a point off to one side most of the rectangle looks onto the hole's far side. Until then the light was the whole rectangle and each ray
  // found that out by chance. With the yard open, at points across the fan: the share of a pixel's rays that reach the sky, as it is and as it was, and
  // the light itself both ways (the same light: what the rays lost is now out of the sum they are weighed against).
  { const wallB = B.pop(), at: { name: string; P: V3; N: V3 }[] = [{ name: 'floor 1 m in and 1.5 m to the side', P: [15, 0, 13.95], N: [0, 1, 0] }, { name: 'floor 1 m in and 2 m to the side', P: [15, 0, 14.45], N: [0, 1, 0] }, { name: 'floor 2.5 m in and 1.5 m to the side', P: [13.5, 0, 14], N: [0, 1, 0] }, { name: 'floor 2.5 m in and 2.4 m to the side', P: [13.5, 0, 14.85], N: [0, 1, 0] }, { name: 'a side wall at 1.2 m', P: [14, 1.2, 15 - T], N: [0, 0, -1] }, { name: 'floor 2.5 m in, square on', P: [13.5, 0, 12.45], N: [0, 1, 0] }];
    const now = at.map(a => ({ ...portal(a.P, a.N), truth: skyBy(a.P, a.N) })); const keep = whole; whole = true; const was = at.map(a => portal(a.P, a.N)); whole = keep; B.push(wallB);
    const lit = now.map((r, i) => r.truth > 2e-4 || was[i].E > 2e-4), ok = now.every((r, i) => !lit[i] || (r.vis > 0.93 && r.E > 0.8 * r.truth && r.E < 1.2 * r.truth)), lost = was.filter((r, i) => lit[i] && r.vis < 0.75).length, same = now.every((r, i) => !lit[i] || Math.abs(r.E - was[i].E) < 0.2 * Math.max(r.E, was[i].E));
    verdict(o.deep > 0.2 && o.deep < 0.4 && lit.filter(Boolean).length >= 5 && ok && lost >= 3 && same,
      'the hole\'s own sides are in the light and not in a ray\'s luck: across the fan of a doorway\'s daylight nearly every ray of a pixel now reaches the sky, where a third to most of them died on the wall the doorway is cut in; the light is the same light, the sum over the doorway to a fifth',
      `the hole is ${o.deep.toFixed(2)} m deep · ` + at.map((a, i) => `${a.name}: ${(100 * now[i].vis).toFixed(0)} % of its rays reach sky (it was ${(100 * was[i].vis).toFixed(0)} %), the light ${now[i].E.toFixed(4)} (it was ${was[i].E.toFixed(4)}; the sum over the doorway ${now[i].truth.toFixed(4)})`).join(' · ')); }

  // 4y. A SIDE WITH NO WALL BEHIND IT HAS NO RIM (scene/openings.ts Opening.rims, common.wgsl portalRims). The hand on which a long opening goes on: a
  // strip of glass 4 m long is two lights side by side, each with the wall at one end and its neighbour at the other, and from the floor under it each
  // is seen whole on its neighbour's side. In each of the room's four walls the rim must be at the end the shader calls it (toward −(dir.z, 0, −dir.x)
  // for bit 1); in two of them each light against the sum over its own rectangle with a ray a point, and against the same light with a rim at both
  // ends.
  { const door = o, keep = walled, room = B.splice(0), rows: { name: string; now: number; truth: number; vis: number; walled: number }[] = [], ends: string[] = []; let endsOk = true;
    for (const wall of ['east', 'west', 'north', 'south'] as const) {
      B.length = 0; box([13, -0.1, 12.5], [9, 0.1, 6]); box([13, 3.1, 12.5], [3 + T, 0.1, 2.5 + T]);   // the same room, the strip of glass from 1 to 2 m up in one wall, 4 m long
      if (wall !== 'west') box([10, 1.5, 12.5], [T, 1.5, 2.5 + T]); if (wall !== 'east') box([16, 1.5, 12.5], [T, 1.5, 2.5 + T]); if (wall !== 'south') box([13, 1.5, 10], [3 + T, 1.5, T]); if (wall !== 'north') box([13, 1.5, 15], [3 + T, 1.5, T]);
      if (wall === 'east' || wall === 'west') { const x = wall === 'east' ? 16 : 10; box([x, 1.5, 10.25], [T, 1.5, 0.25]); box([x, 1.5, 14.75], [T, 1.5, 0.25]); box([x, 0.5, 12.5], [T, 0.5, 2]); box([x, 2.5, 12.5], [T, 0.5, 2]); }
      else { const z = wall === 'north' ? 15 : 10; box([10.5, 1.5, z], [0.5, 1.5, T]); box([15.5, 1.5, z], [0.5, 1.5, T]); box([13, 0.5, z], [2, 0.5, T]); box([13, 2.5, z], [2, 0.5, T]); }
      const tOf = (h: Hole) => h.mid[0] * h.dir[2] - h.mid[2] * h.dir[0], parts = (planOpenings(B) as Hole[]).sort((a, b) => tOf(a) - tOf(b));   // (in order along the shader's first axis)
      const good = parts.length === 2 && parts[0].rims === 1 && parts[1].rims === 2 && parts.every(h => h.deep > 0.2 && h.deep < 0.4); endsOk &&= good; ends.push(`${wall} ${parts.map(h => h.rims).join(' and ')}`);
      if (good && (wall === 'east' || wall === 'north')) for (const [i, h] of parts.entries()) { use(h); for (const t of [-0.75, 0, 0.75]) { const P: V3 = [h.mid[0] + h.dir[0] + along[0] * t, 0, h.mid[2] + h.dir[2] + along[2] * t], N: V3 = [0, 1, 0], now = portal(P, N), truth = skyBy(P, N); walled = true; const w = portal(P, N); walled = keep;
        rows.push({ name: `${wall} part ${i + 1}, ${t} m along`, now: now.E, truth, vis: now.vis, walled: w.E }); } } }
    B.length = 0; B.push(...room); use(door);
    const sumNow = rows.reduce((s, r) => s + r.now, 0), sumWalled = rows.reduce((s, r) => s + r.walled, 0);
    const ok = door.rims === 3 && endsOk && rows.length === 12 && rows.every(r => r.truth > 0 && r.now > 0.8 * r.truth && r.now < 1.2 * r.truth && r.vis > 0.9) && sumWalled < 0.95 * sumNow;
    verdict(ok, 'a side with no wall behind it has no rim: each half of a long strip of glass is seen whole on its neighbour\'s side, in whichever wall it is; each is the sum over its own rectangle to a fifth, and a rim at both ends would cost it light',
      `the doorway's rims ${door.rims} (3: a wall at both ends) · a strip's two lights in order along the wall (1 and 2: each its own end): ${ends.join(', ')} · ` + rows.map(r => `${r.name}: ${r.now.toFixed(4)} (${(r.now / r.truth).toFixed(2)} × the sum, ${(100 * r.vis).toFixed(0)} % of its rays reach sky; with a rim on its open end ${r.walled.toFixed(4)})`).join(' · ') + ` · all of them: ${(sumWalled / sumNow).toFixed(2)} × with a rim at both ends`); }

  // 4x. NO BANDS ON A FACE A HAND FROM THE OPENING (common.wgsl portalFlux's bands, 11-04; the maintainer on his GPU at dusk: "orange door has horizontal
  // banding when open, not when closed"). A leaf swung into the room stands across the doorway's end, its face 5 cm inside the jamb's line: up that
  // face every 5 cm, the light against the sum over the doorway with a ray a point, the ground in it as at dusk. The eight points it was added up by
  // until then are one to each eighth of the height, and each was its own blob on a face this near: the light swung by half between one height and
  // the next (0.71 to 1.47 × the sum, steps of 53 %). Added up exactly it follows the sum to a few per cent, a tenth over it by the floor (where the
  // light's far rim at the foot and the sum's own rule for the threshold part company a little)
  { const wallB = B.pop(), N: V3 = [0, 0, 1], rows: { y: number; E: number; truth: number }[] = []; G = groundOf;
    for (let y = 0.1; y < 1.91; y += 0.05) { const P: V3 = [15.6, y, 12.05]; rows.push({ y, E: area * win(len(sub(o.mid, P))) * flux(P, N), truth: skyBy(P, N, 400) }); }   // (400 a side: at 48 the sum itself steps by a tenth every quarter metre, its rows against the heights)
    G = 0; B.push(wallB);
    const r = rows.map(a => a.E / a.truth), lo = Math.min(...r), hi = Math.max(...r); let jump = 0; for (let i = 1; i < r.length; i++) jump = Math.max(jump, Math.abs(r[i] - r[i - 1]));
    verdict(rows.every(a => a.truth > 0) && lo > 0.8 && hi < 1.2 && jump < 0.05,
      'a face a hand from the opening wears no bands: up a leaf swung across the doorway\'s end the light is the sum over the doorway to a fifth at every height, and never steps by a twentieth of it from one height to the next',
      `${rows.length} heights 5 cm apart on a face 5 cm inside the jamb\'s line, 0.23 m from the opening\'s plane: the light is ${lo.toFixed(2)} to ${hi.toFixed(2)} × the sum over the doorway; the largest step between two heights ${(100 * jump).toFixed(1)} % of the sum`); }

  // 4w. THE SUM HOLDS WHERE A FEW POINTS NEVER COULD: a long opening seen from close and off its middle, a piece of a strip of glass, a face that leans
  // (a man's body, a relief's tilted normal). Openings with nothing round them and no depth, so the sum itself is what is asked: each against 300 by
  // 300 rays over the rectangle, under this file's sky (eight times brighter low down than overhead) with the ground in it. THE RAYS FOLLOW THE LIGHT:
  // with nothing in the way nine in ten of a pixel's rays must get through on every ask (the eight points they are aimed by are laid in the part
  // above the face's horizon; laid over the whole seen part, a face that takes a sliver had a quarter of its light). AND NO STEP: along 10 cm of three
  // leaning faces, 0.5 mm at a time, and round two faces turned through a right angle so that the ends of every piece go under their horizon one after
  // another, the light never moves by a hundredth of its most between two neighbours (a rule that changed its mind there stepped by a fifth)
  { const room = B.splice(0), door = o, hole = (half: [number, number], y: number): Hole => ({ pos: [16, y, 12.5], mid: [16, y, 12.5], dir: [-1, 0, 0], half, full: 4 * half[0] * half[1], deep: 0, rims: 0 }), nrm = (v: V3) => unit(v);
    const rnd = (() => { let s = 4711; return () => (s = (s * 1103515245 + 12345) & 0x7fffffff) / 0x7fffffff; })();
    const asks: { name: string; h: Hole; P: V3; N: V3 }[] = [], wide = hole([2, 1.25], 1.25), strip = hole([1, 0.5], 1.5), plain = hole([0.45, 1.0], 1.0);
    for (const [inn, off] of [[0.1, 0], [0.1, 1], [0.1, 1.8], [0.5, 1.8], [1, 1.8], [0.25, 0.9]]) asks.push({ name: `a 4 m opening, floor ${inn} m in, ${off} m off its middle`, h: wide, P: [16 - inn, 0, 12.5 + off], N: [0, 1, 0] });
    for (const off of [0, 0.9]) asks.push({ name: `a 2 m piece of a strip of glass, floor 0.1 m in, ${off} m off its middle`, h: strip, P: [15.9, 0, 12.5 + off], N: [0, 1, 0] });
    asks.push({ name: 'the doorway, floor 5 cm in by the jamb', h: plain, P: [15.95, 0, 12.91], N: [0, 1, 0] }, { name: 'a top face that leans away from the doorway', h: plain, P: [14.67, 1.11, 12.7], N: nrm([-0.32, 0.94, 0.09]) }, { name: 'a face that leans up at the doorway', h: plain, P: [15.18, 1.55, 12.6], N: nrm([0.94, 0.33, 0.11]) });
    for (let k = 0; k < 48; k++) { const h = [wide, strip, plain][k % 3], inn = [0.05, 0.25, 0.5, 1, 2][k % 5]; asks.push({ name: `a leaning face ${k}`, h, P: [16 - inn, h.mid[1] - h.half[1] + rnd() * 1.4, 12.5 + (rnd() * 2 - 1) * h.half[0] * 1.4], N: nrm([rnd() * 1.2 - 0.2, rnd() * 2 - 1, rnd() * 2 - 1]) }); }
    G = groundOf; const got: { name: string; r: number; vis: number }[] = [];
    for (const a of asks) { use(a.h); const truth = skyBy(a.P, a.N, 300), E = area * flux([a.P[0] + a.N[0] * 0.01, a.P[1] + a.N[1] * 0.01, a.P[2] + a.N[2] * 0.01], a.N); if (truth > 2e-3) got.push({ name: a.name, r: E / truth, vis: portal(a.P, a.N).vis }); }   // (the light at the point the rays leave from, a centimetre off the face: 5 cm from an opening that is a fifth of the way)
    const walk = (h: Hole, P0: V3, N: V3) => { use(h); let step = 0, was = -1; for (let i = 0; i <= 200; i++) { const E = area * flux([P0[0], P0[1] + i * 0.00025, P0[2] + i * 0.0005], N); if (was > 0) step = Math.max(step, Math.abs(E - was) / was); was = E; } return step; };
    const turn = (h: Hole, P: V3, tilt: number) => { use(h); const E: number[] = []; for (let i = 0; i <= 3000; i++) { const a = -0.2 + (Math.PI / 2 + 0.4) * i / 3000; E.push(area * flux(P, nrm([Math.cos(a), Math.sin(a) * Math.cos(tilt), Math.sin(a) * Math.sin(tilt)]))); }
      const top = Math.max(...E); let step = 0; for (let i = 1; i < E.length; i++) step = Math.max(step, Math.abs(E[i] - E[i - 1]) / top); return step; };   // (the face turns from looking at the doorway to looking up, and past both)
    const steps = [walk(plain, [15.18, 1.5, 12.55], nrm([0.94, 0.33, 0.11])), walk(plain, [14.67, 1.06, 12.65], nrm([-0.32, 0.94, 0.09])), walk(wide, [15.81, 0.3, 12.3], nrm([0.49, 0.81, 0.32])), turn(plain, [15.18, 1.55, 12.6], 0.3), turn(wide, [15.7, 0.6, 13.1], -0.5)];
    G = 0; B.push(...room); use(door);
    const rs = got.map(g => g.r).sort((a, b) => a - b), q = (f: number) => rs[Math.min(rs.length - 1, Math.floor(f * rs.length))], thin = got.reduce((w, g) => g.vis < w.vis ? g : w, got[0]), worst = got.reduce((w, g) => Math.abs(Math.log(g.r)) > Math.abs(Math.log(w.r)) ? g : w, got[0]);
    verdict(got.length >= 40 && rs[0] > 0.7 && rs[rs.length - 1] < 1.35 && q(0.1) > 0.85 && q(0.9) < 1.15 && thin.vis > 0.9 && Math.max(...steps) < 0.01,
      'the sum holds where a few points never could: a long opening from close and off its middle, a piece of a strip of glass, the doorway at its jamb and faces that lean are each the sum of 300 by 300 rays to a third, eight in ten of them to 15 %; nine in ten of a pixel\'s rays get through on every one; and the light of a leaning or a turning face never steps',
      `${got.length} asks that get light: ${rs[0].toFixed(2)} to ${rs[rs.length - 1].toFixed(2)} × the rays' sum, a tenth of them under ${q(0.1).toFixed(2)} and a tenth over ${q(0.9).toFixed(2)}; the worst: ${worst.name}, ${worst.r.toFixed(2)} × · ` + got.slice(0, 11).map(g => `${g.name} ${g.r.toFixed(2)} ×`).join(' · ') + ` · the fewest rays through: ${(100 * thin.vis).toFixed(0)} % (${thin.name}) · the largest move between two neighbours on three walks and two turns: ${steps.map(s => (100 * s).toFixed(2) + ' %').join(', ')}`); }

  // 4v. THE DICE'S OWN SUM (common.wgsl portalFaces, portalSolid): what the cache gives up of the sky by an opening, a face of a probe at a time and with
  // no face, is the sky a probe's rays would bring by it. An opening with nothing round it and nothing outside, so a ray that leaves by it meets the
  // sky above the horizon or the sky's own under-horizon below it: each face and the mean against 300 by 300 rays over the rectangle
  { const room = B.splice(0), door = o, h: Hole = { pos: [16, 1, 12.45], mid: [16, 1, 12.45], dir: [-1, 0, 0], half: [0.45, 1], full: 1.8, deep: 0, rims: 0 }; use(h);
    const rays = (P: V3, N: V3 | null, n = 300) => { let s = 0; for (let i = 0; i < n; i++) for (let j = 0; j < n; j++) { const q = sub([o.mid[0], o.mid[1] + ((j + 0.5) / n * 2 - 1) * o.half[1], o.mid[2] + ((i + 0.5) / n * 2 - 1) * o.half[0]], P), d2 = dot(q, q), ql = Math.sqrt(d2), qd: V3 = [q[0] / ql, q[1] / ql, q[2] / ql];
      s += skyL(qd[1]) * Math.max(-dot(qd, o.dir), 0) * (N ? Math.max(dot(N, qd), 0) : 1) / d2; } return s * 4 * o.half[0] * o.half[1] / (n * n); };
    const rows: { name: string; r: number }[] = [];
    for (const P of [[14, 1, 12.45], [14.2, 1, 13.4], [15.33, 1.5, 12.45], [15.2, 0.5, 12.2]] as V3[]) { const F = faces(P); for (let f = 0; f < 7; f++) { const n: V3 | null = f < 6 ? [0, 0, 0] : null; if (n) n[f >> 1] = 1 - 2 * (f & 1); const truth = rays(P, n); if (truth > 1e-3) rows.push({ name: `(${P.join(', ')}) ${f < 6 ? ['+x', '-x', '+y', '-y', '+z', '-z'][f] : 'no face'}`, r: area * F[f] / truth }); } }
    B.push(...room); use(door);
    const lo = Math.min(...rows.map(r => r.r)), hi = Math.max(...rows.map(r => r.r));
    verdict(rows.length >= 16 && lo > 0.75 && hi < 1.3, 'the dice give up what a probe\'s rays would bring: at four probes the sky by the opening on each face that gets any, and with no face, is the sum of 300 by 300 rays to a quarter, the under-horizon sky of a ray that leaves downward and meets nothing in it',
      `${rows.length} faces that get sky: ${lo.toFixed(2)} to ${hi.toFixed(2)} × the rays' sum · ` + rows.map(r => `${r.name} ${r.r.toFixed(2)}`).join(' · ')); }

  // 4a. THE GROUND OUTSIDE is in the light too (common.wgsl portalGround, portalOutside): under the horizon a point looks out on the ground, as bright as
  // the record says (twice its reference less the low sky). A wall across the room looks out level, on the ground and on next to no sky: with the sky
  // alone it would be all but dark by day. Against the same sum over the doorway with a ray a point, each run as far as the shader's (portalPast)
  { const wallB = B.pop(), at: { name: string; P: V3; N: V3 }[] = [{ name: 'the far wall at 1.5 m', P: [10 + T, 1.5, 12.45], N: [1, 0, 0] }, { name: 'the ceiling 2 m in', P: [14, 3 - 0.001, 12.45], N: [0, -1, 0] }, { name: 'floor 2.5 m in', P: [13.5, 0, 12.45], N: [0, 1, 0] },
      { name: 'the ceiling 1 m in', P: [14.83, 3 - 0.001, 12.45], N: [0, -1, 0] }, { name: 'the ceiling 0.5 m in', P: [15.33, 3 - 0.001, 12.45], N: [0, -1, 0] }, { name: 'a shelf\'s underside at 1.2 m, 0.6 m in', P: [15.23, 1.2, 12.45], N: [0, -1, 0] }];
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
    const play = WAY.play; WAY.play = 0; const noPlay = open(B, 0); WAY.play = play;
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
      ['a portal sends what the receiver looks out on (the sky by the function a ray that misses asks, the ground by its record), added up piece by piece', common.includes('return vec4f(L, max(dot(-L, li.dir), 0.0) / (d2 + soft));') && common.includes('return portalOutside(r.xyz, ground) * (r.w * select(max(dot(N, r.xyz), 0.0), 1.0, all(N == vec3f(0.0))));')
        && common.includes('fn portalOutside(L: vec3f, ground: vec3f) -> vec3f { if (L.y > 0.0) { return skyRadiance(L); } return ground; }')
        && common.includes('return min(PORTAL_OUT / max(-dot(dir, li.dir), 0.25), max((Po.y - 0.02) / max(-dir.y, 1e-4) - dist, 0.0));')
        && common.includes('fn portalGround(li: Light) -> vec3f { return max(li.color * (2.0 / max(li.cosInner * portalArea(li), 1e-9)) - mix(frame.skyHorizon, frame.skyZenith, PORTAL_UP), vec3f(0.0)); }') && common.includes(`const PORTAL_UP: f32 = ${LOOK_OPENINGS.up};`) && common.includes('const PORTAL_OUT: f32 = 1.0;') && common.includes('const PORTAL_POINTS: i32 = 8;')
        && common.includes('fn portalAt(i: i32, u: vec2f) -> vec2f { let k = f32(i) + 0.5; return fract(vec2f(k * 0.618034 + 0.5, k / f32(PORTAL_POINTS)) + u) * 2.0 - 1.0; }')
        && common.includes('fn portalSoft(li: Light, high: f32) -> f32 { return li.cosOuter * high * (2.0 / (3.14159265 * f32(PORTAL_POINTS))); }')
        && common.includes('  let x = dot(a, b); let y = abs(x); let v = (0.8543985 + (0.4965155 + 0.0145206 * y) * y) / (3.4175940 + (4.1616724 + y) * y); let c = cross(a, b);\n  return c * (3.14159265 * select(0.5 / max(length(c), 1e-7) - v, v, x > 0.0));')
        && common.includes('    if (da >= 0.0 && db >= 0.0) { F += portalEdge(a, b); }\n    else if (da >= 0.0) { under = normalize(a + (b - a) * (da / (da - db))); F += portalEdge(a, under); cuts++; }\n    else if (db >= 0.0) { back = normalize(a + (b - a) * (da / (da - db))); F += portalEdge(back, b); cuts++; }\n  }\n  if (cuts == 2) { F += portalEdge(under, back); }')
        && common.includes(`const PORTAL_PIECES: i32 = ${PIECES};`) && common.includes(`const PORTAL_RAMP: f32 = ${RAMP};`) && Math.abs(RAMP - 0.4 / Math.sqrt(1 - 0.16)) < 1e-4 && common.includes('let t = clamp(rd.y * 2.5, 0.0, 1.0);')
        && common.includes('  let r0 = length(vec2f(X0, pz)); let r1 = length(vec2f(X1, pz)); let Xs = (X0 * r1 + X1 * r0) / (r0 + r1); let rs = length(vec2f(Xs, pz));') && common.includes('  let lo = max(y0, 0.0); let cut = min(max(PORTAL_RAMP * length(vec2f((xa * rb + xb * ra) / (ra + rb), pz)), lo), y1);')
        && common.includes('return array<vec3f, 4>(normalize(o + T * q.x + vec3f(0.0, q.z, 0.0)), normalize(o + T * q.x + vec3f(0.0, q.w, 0.0)), normalize(o + T * q.y + vec3f(0.0, q.w, 0.0)), normalize(o + T * q.y + vec3f(0.0, q.z, 0.0)));')
        && common.includes('  let x = clamp(select(0.5 * (q.x + q.y), pz * dot(F, T) / max(fz, 1e-9), fz > 1e-9), q.x, q.y); let o = -li.dir * pz + T * x;')
        && common.includes('  if (d0 <= 0.0 && d1 <= 0.0) { return select(L0, L1, d1 >= d0); }\n  var a = L0; var b = L1; var wa = d0; var wb = d1;\n  if (d0 < 0.0) { a = mix(L0, L1, d0 / (d0 - d1)); wa = 0.0; }\n  if (d1 < 0.0) { b = mix(L1, L0, d1 / (d1 - d0)); wb = 0.0; }\n  return (a * (2.0 * wa + wb) + b * (wa + 2.0 * wb)) / (3.0 * (wa + wb));')
        && common.includes('    if (i > 0) { let e = portalEnds(li, q, pz, F); L = portalMean(skyRadiance(e[0]), skyRadiance(e[1]), dot(e[0], N), dot(e[1], N)); }\n    s += L * w;')
        && common.includes('r.E = portalFlux(li, P, N) * (portalArea(li) * lightWindow(dist, li.range)); r.ok = any(r.E > vec3f(0.0));') && common.includes('fn portalArea(li: Light) -> f32 { return 4.0 * li.cosOuter * portalHigh(li); }')
        && common.includes('return mix(mix(frame.skyHorizon, frame.skyZenith, t), frame.skyHorizon * 0.15, below);') && !common.includes('fn skyLum(')],
      ['the light evaluation every pass pastes knows nothing of a portal (the final gather has no stack for it), nor do the passes whose ray stops short', !/lightPortal|PORTAL|portal/.test(common.slice(common.indexOf('fn evalLightRaw('), common.indexOf('fn directIrradiance(') + 2600).split('\n// ---- torch bounce cards')[0])
        && [gc, text('src/render/shaders/compact/gather_common.wgsl'), text('src/render/shaders/probe.wgsl')].every(t => !/portal|PORTAL/.test(t))],
      ['its shadow ray is aimed by what each point sends, runs on to the sky or clear of the opening, and answers yes or no', dc.includes('tmax = lightDist - 0.03 + past;') && dc.includes('w[i] = luminance(portalPoint(li, toC + T * (at.x * li.cosOuter) + B * (at.y * hh), N, soft, ground)); sum += w[i]; }') && dc.includes('past = portalPast(li, Po, d / dl, dl);')
        && dc.includes('var x = fract(ur * 61.0 + ua * 37.0 + 0.5) * sum;') && dc.includes('for (var i = 0; i < PORTAL_POINTS; i++) { x -= w[i]; if (x < 0.0) { sel = i; break; } }') && !/skyK\b|PORTAL_HEAD/.test(dc + common)
        && dc.includes('var d = (li.pos + T * (cos(a) * r) + B * (sin(a) * r)) - Po; var past = 0.0;')],   // (… and any other light's ray is written as it was, to the last rounding)
      ['restir weighs a portal by its whole rectangle from frame to frame', common.includes('fn lightPower(li: Light) -> f32 { return luminance(li.color) / select(1.0, li.cosInner, lightPortal(li)); }') && ['restir', 'compact/restir'].every(f => text(`src/render/shaders/${f}.wgsl`).includes('let kNow = lightPower(wgLights[curT[c]]); let kPrev = lightPower(lp);'))],
      ['a blocker past the opening hands on the opening\'s own size', count(dc, 'select(li.radius * dOcc / max(lightDist - dOcc, 0.05), li.radius, dOcc >= lightDist)') === 2],
      ['the direct estimators take the rectangle itself and trace through it, and no other pass does', ['restir', 'direct', 'sharp_direct', 'compact/restir', 'compact/direct', 'compact/sharp_direct'].every(f => { const t = text(`src/render/shaders/${f}.wgsl`); return count(t, 'evalLightDirect(') >= 2 && !/\bevalLight\(/.test(t); }) && count(fg + gs + cf + rc + gc + text('src/render/shaders/probe.wgsl'), 'evalLightDirect(') === 0],
      ['the bounce keeps what a portal does not send, and walks the list from its first portal', common.includes('for (var i = u32(frame.placeCfg.y) - 1u; i < frame.numLights; i++) {') && common.includes('keep = 1.0 - lightWindow(distance(O, lights[i].pos), lights[i].range); break;') && common.includes('fn skyThrough(O: vec3f, dir: vec3f) -> vec3f { return skyRadiance(dir) * skyKeep(O, dir); }')],
      ['every diffuse ray of the gather asks which sky is its own', count(fg, 'L = skyThrough(Po, dir);') === 3 && count(fg, 'L = skyRadiance(dir);') === 1 && count(gs, 'L = skyThrough(Po, dir);') === 1 && cf.includes('if (gatherRay) { L = skyThrough(ro, dir); } else { L = skyRadiance(dir); }')],
      ['and so does the cache, at the probe the light is for: the cascades carry how much of a direction\'s light is the sky\'s, and the dice bake takes a portal\'s own sum out of it, by the share that is open and never more than the sky that came; the place field is handed back what left the face it reads, through the steady bounce\'s ring too', rc.includes('let sky = skyRadiance(dir); return vec4f(r.xyz + sky, luminance(sky));') && rc.includes('return vec4f(r.xyz + up.xyz, up.w); }')
        && rc.includes('outBuf[probeLin * nOut + o] = packRad(sum * 0.25);') && rc.includes('outBuf[probeLin * DD + d] = packRad(L);') && count(rc, 'skyThrough(') === 0 && count(rc, 'skyKeep(') === 0
        && rc.includes('if (!lightPortal(li) || !lightReaches(i, P)) { continue; }') && rc.includes('let k0 = portalArea(li) * li.cosInner * lightWindow(dist, li.range);') && rc.includes('for (var f = 0; f < 7; f++) { let p = k0 * F[f]; V[f] += hue * p; T[f] += p; }')
        && common.includes('      let e = portalEnds(li, q, pz, F); let L0 = luminance(skyRadiance(e[0])); let L1 = select(luminance(skyRadiance(e[1])), L0, i == 0);\n      s[f] += portalMean(vec3f(L0), vec3f(L1), dot(e[0], n), dot(e[1], n)).x * w * per;') && common.includes('    s[6] += 0.5 * (M0 + select(luminance(skyRadiance(m[1])), M0, i == 0)) * portalSolid(q, pz) * per;')
        && common.includes('  let s = atan2(q.y * q.w, pz * sqrt(q.y * q.y + q.w * q.w + pz * pz)) - atan2(q.x * q.w, pz * sqrt(q.x * q.x + q.w * q.w + pz * pz)) - atan2(q.y * q.z, pz * sqrt(q.y * q.y + q.z * q.z + pz * pz)) + atan2(q.x * q.z, pz * sqrt(q.x * q.x + q.z * q.z + pz * pz));')
        && rc.includes('gave = luminance(floorLight - E2);') && rc.includes('mix(was.w, gave, k)') && text('src/render/shaders/place_field.wgsl').includes('vec3f(0.2126, 0.7152, 0.0722)) + tap.w;') && text('src/render/shaders/dice_ring.wgsl').includes('textureStore(outDice, gid, sum / max(got, 1.0));')
        && rc.includes('E0 = max(E0 - V[0] * min(1.0, S[0] / max(T[0], 1e-9)), vec3f(0.0));') && rc.includes('mean = max(mean - V[6] * min(1.0, S[6] / max(T[6], 1e-9)), vec3f(0.0));')],
      ['the hole\'s depth rides the kind word where a lamp\'s marking does, and the light is the part of its rectangle a point sees past the hole\'s far rim: one function, asked by the light\'s sum, the cache\'s and the ray\'s aim',
        portalHoleBits(0.29) === 290 << 11 && portalHoleBits(0.29, 3) === ((290 << 11) | (3 << 21)) && portalHoleBits(undefined, 1) === 1 << 21 && portalHoleBits(99, 0) === 1023 << 11 && (portalHoleBits(99, 3) & (255 | 256 | 512 | 1024 | LIGHT_PORTAL | (1 << 25) | (1 << 26) | (1 << 27))) === 0
        && common.includes('fn portalDeep(li: Light) -> f32 { return f32((li.kind >> 11u) & 0x3ffu) * 0.001; }') && common.includes('fn portalRims(li: Light) -> vec2<bool> { return (vec2u(li.kind >> 21u) & vec2u(1u, 2u)) != vec2u(0u); }') && common.includes('fn lightMarkRef(li: Light) -> i32 { return select(-1, i32((li.kind >> 11u) & 0x3fffu), (li.kind & 1024u) != 0u); }')
        && common.includes('  let half = vec2f(li.cosOuter, hh); let p = vec2f(d.z * li.dir.x - d.x * li.dir.z, -d.y);') && common.includes('  let k = pz / (pz + deep);\n  let rim = portalRims(li);\n  let far = (half + PORTAL_RIM) * k;\n  let lo = select(-half, max(p * (1.0 - k) - far, -half), vec2<bool>(rim.x, true)); let hi = select(half, min(p * (1.0 - k) + far, half), vec2<bool>(rim.y, true));\n  if (hi.x <= lo.x || hi.y <= lo.y) { return vec4f(0.0); }\n  return vec4f((lo + hi) * 0.5, (hi - lo) * 0.5) / vec4f(half, half);')
        && common.includes(`const PORTAL_RIM = vec2f(${LOOK_OPENINGS.ds * 0.5}, ${LOOK_OPENINGS.dy * 0.5});`) && RIM[0] === LOOK_OPENINGS.ds * 0.5 && RIM[1] === LOOK_OPENINGS.dy * 0.5
        && count(common, 'let see = portalSeen(li, d, hh);') === 2 && count(dc, 'let see = portalAbove(li, toC, portalSeen(li, toC, hh), hh, N); let part = see.z * see.w;') === 1 && count(common + dc + rc, 'portalSeen(') === 4
        && common.includes('    if (da >= 0.0) { mn = min(mn, a); mx = max(mx, a); }\n    if ((da >= 0.0) != (db >= 0.0)) { let p = mix(a, b, da / (da - db)); mn = min(mn, p); mx = max(mx, p); }') && common.includes('    let a = vec2f(select(lo.x, hi.x, i >= 2), select(lo.y, hi.y, i == 1 || i == 2)); let b = vec2f(select(lo.x, hi.x, j >= 2), select(lo.y, hi.y, j == 1 || j == 2));') && count(common, 'portalPiece(li, d, see, hh, i)') === 2 && count(dc, 'see.xy + portalAt(') === 2
        && count(common + dc, 'portalSoft(li, hh) * part') === 1
        && text('src/render/lights.ts').includes('(l.portal ? LIGHT_PORTAL | portalHoleBits(l.portal.deep, l.portal.rims) : 0) | (!l.portal && l.markRef !== undefined') && text('src/engine.ts').includes('l.portal!.deep = this.settings.portalHole ? o.deep : 0; l.portal!.rims = o.rims;') && text('src/main.ts').includes("if (q.get('holedepth') === '0') engine.settings.portalHole = false;") && /VISIT_FLAGS = \[[^\]]*'holedepth'/.test(text('src/ui/replaykey.ts')) && /LOOK_PAGE_WORDS[^\n]*'holedepth'/.test(text('src/scene/look.ts')) && text('src/scene/openings.ts').includes('half: [hw, hh], deep: holeDeep, rims: ((lowS0 ? first : last) ? 1 : 0) | ((lowS0 ? last : first) ? 2 : 0),') && text('src/scene/openings.ts').includes('const lowS0 = r.axis === 1 ? r.sign > 0 : r.sign < 0;')],
      ['the table still knows where an opening looks', LOOK_OPENINGS.up > 0 && LOOK_OPENINGS.up < 1 && (BoxFlag.Glass | 0) !== 0] ];
    verdict(pins.every(p => p[1]), 'the light\'s record and the shaders say the same rule', pins.map(p => `${p[1] ? 'ok' : 'NOT'}: ${p[0]}`).join(' · ')); }
  // HOW FAR AN OPENING'S OWN LIGHT REACHES (scene/openings.ts openingReach, LOOK_OPENINGS.worth): one rule at every hour, where what it alone would show on
  // a floor facing it straight out falls to `worth`; past it the bounce has the sky (the window and skyKeep above). PORTALREACH_BREAK=1 = the worth it had
  // until 11-05 (0.003): a night's opening reaches the far side of a building again
  { const { openingReach } = await import(`${ROOT}/src/scene/openings.ts`); const K = LOOK_OPENINGS, common = text('src/render/shaders/common.wgsl'), count = (t: string, w: string) => t.split(w).length - 1, old = process.env.PORTALREACH_BREAK === '1' ? 0.003 : undefined;
    // (the consulate's widest opening on its night as a cloud card listed it: it looks out on 0.0853, exposure 2, 5.65 m², half diagonal 2.04 m. A door at
    // midday: 0.97 by the hour's sky and sun, exposure 1.7. A slit at dusk. A wide opening on a dim night. A glass wall by day. A strip of glass in five pieces)
    const night = openingReach(0.0853 * 2, 5.65, 2.04, old), noon = openingReach(0.97 * 1.7, 1.85, 1.12, old), slit = openingReach(0.02 * 2, 0.3, 0.6, old), wide = openingReach(0.01 * 2, 12.25, 2.75, old), wall = openingReach(0.6 * 2, 30, 4, old);
    const was = openingReach(0.0853 * 2, 5.65, 2.04, 0.003), piece = openingReach(0.0853 * 2, 1.2, 1.04, old), strip = openingReach(0.0853 * 2, 5 * 1.2, 1.04, old);
    let mono = true; for (let m = 0.01; m < 2; m *= 1.3) for (let w = 0.2; w < 30; w *= 1.4) { const r = openingReach(m, w, 1, old); if (openingReach(m * 1.3, w, 1, old) < r || openingReach(m, w * 1.4, 1, old) < r || openingReach(m, w, 1.5, old) < r) mono = false; }
    const eng = text('src/engine.ts'), op = text('src/scene/openings.ts');
    verdict(night > 4 && night < 6 && noon > 8 && noon < 9.5 && slit === K.near && Math.abs(wide - (2.75 + K.past)) < 1e-9 && wall === K.far && mono && Math.abs(was - 17.9) < 0.1 && piece < 3.1 && strip > 4.5
      && eng.includes('l.range = openingReach(m * this.settings.exposure, Math.max(whole, o.of), l.radius, this.settings.portalWorth > 0 ? this.settings.portalWorth : undefined);') && count(eng, 'l.range = ') === 1
      && op.includes('for (let i = first0; i < out.length; i++) out[i].of = sum;')
      && text('src/main.ts').includes("{ const pw = Number(q.get('portalworth')); if (pw > 0) engine.settings.portalWorth = pw; }")
      && /VISIT_FLAGS = \[[^\]]*'portalworth'/.test(text('src/ui/replaykey.ts')) && /LOOK_PAGE_WORDS[^\n]*'portalworth'/.test(text('src/scene/look.ts'))
      && common.includes('keep = 1.0 - lightWindow(distance(O, lights[i].pos), lights[i].range); break;'),
      'an opening\'s own light reaches as far as it is worth a ray, by one rule at every hour, and the bounce has the sky past it',
      `the consulate's widest opening on its night ${night.toFixed(1)} m (it was ${was.toFixed(1)}), a door at midday ${noon.toFixed(1)} m, a slit at dusk ${slit} m, a wide opening on a dim night ${wide.toFixed(2)} m (never short of its own corners), a glass wall ${wall} m; one piece of a strip of glass alone ${piece} m, as a piece of its strip ${strip.toFixed(1)} m; never shorter for a brighter sky, a larger opening or a longer rectangle`); }
  console.log(all ? 'portal: all passed' : 'portal: FAILED');
  return all;
}
if (import.meta.main) process.exit(await portalCheck() ? 0 : 1);
