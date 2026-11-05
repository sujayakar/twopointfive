// common.wgsl's cut test over a box's eight packed rows, statement for statement, for the checks that hold the GPU's rule to the CPU's
// (cutboxes-probes.ts, round-check.ts). `f` rounds as the caller's other statements do (Math.fround, or nothing).
//   cutFour       = common.wgsl cutFour: four rows at once, no loop and no branch
//   cutPlanes     = the two fours isectCut takes: (tN, tF, the code of the face the ray enters by; 7 = none of the planes)
//   cutPlanesLoop = the same planes ONE BY ONE, as the test was written until 2026-11-05 and as cut.ts cutClip still walks them: the reference the fours are set against
type Row = (x: number) => number;
const dot3 = (r: ArrayLike<number>, q: number, v: readonly number[]) => r[q] * v[0] + r[q + 1] * v[1] + r[q + 2] * v[2];
export function cutFour(rows: ArrayLike<number>, k0: number, lo: readonly number[], ld: readonly number[], s: readonly number[], f: Row): number[] {
  const Q = [0, 1, 2, 3].map(i => (k0 + i) * 4), nz = Q.map(q => rows[q] !== 0 || rows[q + 1] !== 0 || rows[q + 2] !== 0);
  const l0 = s[3] > 0.5 && nz[0], l1 = l0 && nz[1], l2 = l1 && nz[2], l3 = l2 && nz[3], live = [l0, l1, l2, l3];
  const den = Q.map(q => f(dot3(rows, q, ld))), num = Q.map(q => f(rows[q + 3] - f(dot3(rows, q, lo))));
  const t = num.map((n, i) => f(n / (den[i] === 0 ? 1 : den[i])));
  const tIn = t.map((v, i) => live[i] && den[i] < 0 ? v : f(-3.0e38)), tOut = t.map((v, i) => live[i] && den[i] > 0 ? v : f(3.0e38));
  const mIn = Math.max(Math.max(tIn[0], tIn[1]), Math.max(tIn[2], tIn[3])), mOut = Math.min(Math.min(tOut[0], tOut[1]), Math.min(tOut[2], tOut[3]));
  const gone = live.some((l, i) => l && den[i] === 0 && num[i] < 0);
  const face = tIn[0] === mIn ? -1 - k0 : tIn[1] === mIn ? -2 - k0 : tIn[2] === mIn ? -3 - k0 : -4 - k0;
  const up = mIn > s[0];
  return [up ? mIn : s[0], gone ? -1 : Math.min(s[1], mOut), up ? face : s[2], l3 ? 1 : 0];
}
export function cutPlanes(rows: ArrayLike<number>, lo: readonly number[], ld: readonly number[], tN0: number, tF0: number, f: Row): [number, number, number] {
  const s = cutFour(rows, 4, lo, ld, cutFour(rows, 0, lo, ld, [tN0, tF0, 7, 1], f), f);
  return [s[0], s[1], s[2]];
}
export function cutPlanesLoop(rows: ArrayLike<number>, lo: readonly number[], ld: readonly number[], tN0: number, tF0: number, f: Row): [number, number, number] {
  let tN = tN0, tF = tF0, code = 7;
  for (let k = 0; k < 8; k++) { const q = k * 4;
    if (rows[q] === 0 && rows[q + 1] === 0 && rows[q + 2] === 0) break;
    const den = f(dot3(rows, q, ld)), num = f(rows[q + 3] - f(dot3(rows, q, lo)));
    if (den < 0) { const t = f(num / den); if (t > tN) { tN = t; code = -1 - k; } }
    else if (den > 0) tF = Math.min(tF, f(num / den));
    else if (num < 0) return [tN, -1, code]; }
  return [tN, tF, code];
}
