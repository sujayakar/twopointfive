// THE NARROWEST WAY THROUGH A GAP THAT THINGS HANG IN, in plan. A gap has two jambs; what hangs in it (a door's leaf, a pair of leaves, a shutter) and the
// jambs are a chain across it, and what can pass is what the gaps of the SHORTEST chain from jamb to jamb leave: the cut that lets least through. So a
// leaf a hand's width off its frame leaves a hand's width, whichever way it swings and however thick the wall, a pair of leaves leaves the gap between
// them, and nothing here knows what a door is.
// WHO ASKS: scene/openings.ts (what is open of an opening to the sky: a portal's light), scene/placePlan.ts (how open a doorway of the floor plan is).
// Pure arithmetic on outlines: no box, no level, no state.

/** `play` (m): the slack a shut leaf has in its frame, and a finger over (a 0.84 m leaf in a 0.9 m doorway has 6 cm, at one jamb or shared). A gap of a way
 *  through is open by what it has over this. ONE number for whoever asks (scene/openings.ts ajarOf, scene/placePlan.ts doorwayOpen and doorJambs) */
export const WAY = { play: 0.07 };

/** a point of the plan (any two axes, the same for every outline of one question) */
export type Pt = readonly [number, number];

/** how far apart two outlines are (each a segment or a convex outline, as corners in order): 0 where they touch, cross or one holds the other */
export function apart(A: readonly Pt[], B: readonly Pt[]): number {
  const edges = (P: readonly Pt[]) => P.length === 2 ? 1 : P.length, side = (a: Pt, b: Pt, p: Pt) => (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
  const toSeg = (p: Pt, a: Pt, b: Pt) => { const ex = b[0] - a[0], ey = b[1] - a[1], l2 = ex * ex + ey * ey, t = l2 > 0 ? Math.max(0, Math.min(1, ((p[0] - a[0]) * ex + (p[1] - a[1]) * ey) / l2)) : 0; return Math.hypot(p[0] - a[0] - ex * t, p[1] - a[1] - ey * t); };
  const holds = (P: readonly Pt[], p: Pt) => { if (P.length < 3) return false; let pos = 0, neg = 0; for (let i = 0; i < P.length; i++) { const k = side(P[i], P[(i + 1) % P.length], p); if (k > 0) pos++; else if (k < 0) neg++; } return (pos === 0) !== (neg === 0); };   // (all on one side; an outline with no area holds nothing)
  if (holds(B, A[0]) || holds(A, B[0])) return 0;
  let d = Infinity;
  for (let i = 0; i < edges(A); i++) { const a0 = A[i], a1 = A[(i + 1) % A.length];
    for (let j = 0; j < edges(B); j++) { const b0 = B[j], b1 = B[(j + 1) % B.length];
      if (side(a0, a1, b0) * side(a0, a1, b1) < 0 && side(b0, b1, a0) * side(b0, b1, a1) < 0) return 0;   // (they cross)
      d = Math.min(d, toSeg(a0, b0, b1), toSeg(a1, b0, b1), toSeg(b0, a0, a1), toSeg(b1, a0, a1)); } }
  return d;
}

/** the shortest chain from one jamb to the other over the outlines that hang between them, each gap of it less `play` (the slack a shut leaf has in its
 *  frame: a gap narrower than that is no gap). Infinity with nothing between: jamb to jamb is the gap itself and no chain, which is the asker's to know. */
export function narrowest(jambA: readonly Pt[], jambB: readonly Pt[], between: readonly (readonly Pt[])[], play: number): number {
  if (!between.length) return Infinity;
  const nodes = [jambA, jambB, ...between], N = nodes.length, dist = new Array<number>(N).fill(Infinity), done = new Array<boolean>(N).fill(false); dist[0] = 0;
  for (let k = 0; k < N; k++) { let u = -1; for (let i = 0; i < N; i++) if (!done[i] && (u < 0 || dist[i] < dist[u])) u = i; done[u] = true;
    for (let w = 0; w < N; w++) if (!done[w] && u + w !== 1) dist[w] = Math.min(dist[w], dist[u] + Math.max(0, apart(nodes[u], nodes[w]) - play)); }
  return dist[1];
}

/** a box's footprint as corners in order (x, z): centre, half extents along its own x and z, and its yaw (scene/placePlan.ts stampCells' turn) */
export function footprint(cx: number, cz: number, hx: number, hz: number, yaw: number): Pt[] {
  const c = Math.cos(yaw), s = Math.sin(yaw);
  return [[-1, -1], [1, -1], [1, 1], [-1, 1]].map(([i, j]): Pt => { const lx = i * hx, lz = j * hz; return [cx + c * lx + s * lz, cz - s * lx + c * lz]; });
}
