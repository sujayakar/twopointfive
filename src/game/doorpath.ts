// EACH MAN WALKS A REAL PATH THROUGH THE DOOR (`DOORPATH.on`; ON in the page — `?nodoorpath` puts it off, off in the
// harness unless TPS_DOORPATH=1[,knob=value…]; off: the entry exactly as it was).
//
// What a room entry did (doortools.ts, the 'enter' stage): every man was let go a quarter second after the one before, each handed his point of domination and
// the nav grid's path to it. The grid knows walls; it does not know a LEAF standing open, nor the man in front. So the second man in hooked tight round the
// hinge jamb for his corner, walked into the leaf's open face and pushed it to its stop — where the door's own rule (doors.ts: a keyed man pressed on a leaf at
// its stop pulls it to) swung it shut THROUGH him and through the next man in the opening; and four men let go on a clock met in the frame shoulder to
// shoulder. With the switch on:
//   THE LANE    each man's walk is laid by hand from where he stands: to a point `outM` short of the frame line, across it at `at` of the opening from the
//               hinge (the half the leaf is not in), on to a point `inM` inside — one file through the middle of the gap, square to the frame
//   THE LEAF    from there to his point a leg may not come within `leafClear` of any leaf AS IT STANDS (his own door's, the next room's): where the straight
//               leg would, he goes ROUND THE FREE TIP on an arc that keeps that much air (re-laid as the leaf moves). A point with NO way round — the pocket between an
//               open leaf and the wall behind it, its mouth narrower than a man and that clearance — is not walked into through the leaf: his point is
//               moved to the nearest floor of the same room he can reach and stand on clear of every leaf (`moved`; the order's own record of his point
//               moves with it). While the leaf is still swinging nothing is decided: he walks his lane and waits at its end for it to stand. Only a point
//               with no such floor near it is walked straight, as before, and counted (`noRound`)
//   THE FILE    the men go in the order the door order dealt, one at a time: a man is let go when the man before him will be `gapM` ahead of him as he
//               reaches the frame line (0: the moment that man will be first at the line — the second gun is through half a second behind the first); and a man who comes within `shortM` of one sent before him, with that man in front of him, eases off until there is
//               room. Nobody is ever kept longer than `cap` seconds either way — a file that cannot form goes as the old one did
// The walk itself is the runner's own (squad.ts → guards.ts followPath): this module only keeps the man's path two points long — a point `look` metres ahead
// of him on his line, then his goal — so his legs, his pace, his lean and his collisions are everybody's. followPath's stuck watchdog still has him: when it
// side-steps or re-plans, the walk stands back for a moment and is re-laid from where that left him; twice, and the nav has him for good (`gaveUp`).
// But for a point moved out from behind a leaf it changes no point of domination, and it changes no order and no gun: where the men end up and what they
// do there is the door order's — but for the gun lowered at a leaf.
import { SideMap } from '../math/side';
import type { Game, Guard } from './game';
import type { Door } from './doors';
import type { DoorOp } from './doortools';
import type { Vec3 } from '../math/vec';
import { RUST_SURFACE, goneWithTick } from './crossing';

export const DOORPATH = {
  on: false,
  /** where the lane cuts the frame line: this share of the opening from the hinge (the leaf's root is at 0). 0.52 since the doors are real size
   *  (scene/doorSize.ts): between a leaf standing square on its stop and the latch stop a man's body has 0.325 to 0.556 m of a 0.86 m leaf's frame line to
   *  put his middle on, and the lane runs down the middle of that (0.6, as it was for the 1.16 m leaves, ran him a hand off the latch stop) */
  at: 0.52,
  /** the lane's ends: metres short of the frame line, metres past it */
  outM: 0.7, inM: 0.7,
  /** how far ahead of him on his line the point he walks at is kept */
  look: 0.5,
  /** no leg nearer than this to a leaf's centre line as it stands (the door's own contact starts at 0.365) */
  leafClear: 0.45,
  /** the arc round a leaf's free tip is laid this far out from it (a man walking at a point ahead of him cuts a corner by a hand or two) */
  tipM: 0.75,
  /** let go when the man before him will be this far ahead at the frame line · eased off inside this of a man sent before him (stopped 0.35 nearer).
   *  0 and 0.6 = the next man goes the moment the man before him will reach the line first, and gives way only to a body's width: the four cross inside
   *  about 1.1 s, half a second apart. (0.5 and 1.05, as first built, spaced them 0.6 s apart and brought the second man through the depot's first door
   *  0.2 s after the old entry did — his partner fought the room alone for that long, and the depot lost 24 teammates in 576 over 144 seeds) */
  gapM: 0, shortM: 0.6,
  /** the most a man is held back, seconds: at the stack, and again on the way */
  cap: 2.5,
  /** a point behind a leaf is moved at most this far · and stays this far from another man's point */
  moveM: 0, apartM: 0.7,   // (moveM 0 since the doors rules share this tree: where a body rests behind a leaf is theirs; 1.8 = the walk moving a point by itself)
  /** a man down within this of a point still being walked to: the point is moved off him (0: never — a man walks onto the fallen as before) · the new
   *  point stands this far from where any man down lies · and at most this far from the point as dealt */
  bodyM: 0, bodyOff: 0.6, bodyMove: 1.2,
};

interface Walk {
  gd: Guard; op: DoorOp; door: Door; pod: Vec3; n: number; t0: number;
  /** his line (pts[0] where he stood when it was laid), its running length, how far along he is · the leaf angles it was laid against */
  pts: Vec3[]; len: number[]; s: number; laidT: number; near: { d: Door; angle: number }[]; lane: number;
  /** what this module last put under him (the watchdog's hand shows as a change) · standing back until · how often · seconds held short · off his order since */
  via: Vec3[]; arr: Vec3[] | null; arrN: number; yieldT: number; yields: number; held: number; offT: number; round: boolean;
  /** a leaf by his line is still swinging: his line ends at the lane's end until it stands · seconds waited so */
  wait: boolean; waited: number;
  /** the lane's inner end as it was laid (metres past the frame line) · his point as the door order dealt it (a moved point is looked for round this one) */
  inM: number; pod0: Vec3;
  /** when his point was last looked at for a man down on it */
  bodyT: number;
}

const flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);
/** a door blown off its hinges (doors.ts `Door.off`, where a build has that rule): there is no leaf in the frame to walk round, or to keep a gun out of —
 *  the slab on the floor is nobody's to route by. A build without the rule has no such door, and this is never true */
const gone = (d: Door) => (d as Door & { off?: boolean }).off === true;
/** the least distance between segments a→b and c→d on the floor */
function segSeg(a: Vec3, b: Vec3, c: Vec3, d: Vec3): number {
  const cross = (ox: number, oz: number, px: number, pz: number, qx: number, qz: number) => (px - ox) * (qz - oz) - (pz - oz) * (qx - ox);
  const d1 = cross(c[0], c[2], d[0], d[2], a[0], a[2]), d2 = cross(c[0], c[2], d[0], d[2], b[0], b[2]), d3 = cross(a[0], a[2], b[0], b[2], c[0], c[2]), d4 = cross(a[0], a[2], b[0], b[2], d[0], d[2]);
  if (d1 * d2 < 0 && d3 * d4 < 0) return 0;
  return Math.min(ptSeg(a, c, d), ptSeg(b, c, d), ptSeg(c, a, b), ptSeg(d, a, b));
}
function ptSeg(p: Vec3, a: Vec3, b: Vec3): number {
  const dx = b[0] - a[0], dz = b[2] - a[2], l2 = dx * dx + dz * dz, t = l2 > 1e-9 ? Math.max(0, Math.min(1, ((p[0] - a[0]) * dx + (p[2] - a[2]) * dz) / l2)) : 0;
  return Math.hypot(p[0] - a[0] - dx * t, p[2] - a[2] - dz * t);
}

export class DoorPath {
  walks = new Map<Guard, Walk>();
  /** per door order: the walk last let go, how many so far */
  private lastOf = new SideMap<DoorOp, { W: Walk | null; n: number; t: number }>();
  stats = { sent: 0, walked: 0, round: 0, noRound: 0, noLane: 0, late: 0, heldS: 0, capped: 0, yields: 0, gaveUp: 0, taken: 0, relaid: 0, turned: 0, moved: 0, waitS: 0, nav: 0, stepped: 0 };
  constructor(private g: Game) {}
  reset() { this.walks.clear(); }


  /** the leaf as it stands: hinge → free tip */
  private leaf(d: Door): [Vec3, Vec3] { const u = d.dir(), h = d.def.hinge; return [[h[0], 0, h[1]], [h[0] + u[0] * d.def.width, 0, h[1] + u[1] * d.def.width]]; }
  private leaves(near: Vec3, far: Vec3): [Vec3, Vec3][] { const out: [Vec3, Vec3][] = []; for (const d of this.g.doors.list) { if (gone(d)) continue; const L = this.leaf(d); if (segSeg(near, far, L[0], L[1]) < 2.5) out.push(L); } return out; }
  /** a man can stand here: clear of every static at his own width */
  private free(p: Vec3): boolean { const q: Vec3 = [p[0], 0, p[2]]; this.g.col.collideCircle(q, 0.3, 0.2, 1.5, 3); return flat(q, p) < 0.03; }
  private open(a: Vec3, b: Vec3): boolean {
    if (this.g.col.solidBlocked([a[0], 0.45, a[2]], [b[0], 0.45, b[2]])) return false;
    const d = flat(a, b), n = Math.floor(d / 0.3); for (let i = 1; i < n; i++) { const k = i / n; if (!this.free([a[0] + (b[0] - a[0]) * k, 0, a[2] + (b[2] - a[2]) * k])) return false; }
    return true;
  }

  /** from a to b keeping `leafClear` off every leaf nearby: the corners to walk (b last), or null when no way round was found. A point that itself stands nearer
   *  a leaf than the clearance is asked only for what it has. */
  private roundLeaves(a: Vec3, b: Vec3): { pts: Vec3[]; round: boolean } | null {
    const K = DOORPATH, c = K.leafClear, L = this.leaves(a, b);
    const off = (p: Vec3) => { let m = Infinity; for (const l of L) m = Math.min(m, ptSeg(p, l[0], l[1])); return m; };
    const ok = (p: Vec3, q: Vec3, dp: number, dq: number) => { const need = Math.max(0.05, Math.min(c, dp, dq) - 0.02); for (const l of L) if (segSeg(p, q, l[0], l[1]) < need) return false; return true; };
    const da = off(a), db = off(b);
    if (ok(a, b, da, db)) return { pts: [b], round: false };
    // the corners: an arc round each leaf's free tip, far enough out (tipM) that the chords between them — and the corner a man cuts walking at a point
    // ahead of him — keep the clearance
    const rr = K.tipM, nodes: Vec3[] = [a, b], dist: number[] = [da, db];
    for (const l of L) { const th = Math.atan2(l[1][2] - l[0][2], l[1][0] - l[0][0]);
      for (let k = -3; k <= 3; k++) { const p: Vec3 = [l[1][0] + Math.cos(th + k * Math.PI / 4) * rr, 0, l[1][2] + Math.sin(th + k * Math.PI / 4) * rr]; if (this.free(p)) { nodes.push(p); dist.push(off(p)); } } }
    // the shortest way over them (a dozen points: the plain search)
    const N = nodes.length, best = new Array<number>(N).fill(Infinity), from = new Array<number>(N).fill(-1), done = new Array<boolean>(N).fill(false); best[0] = 0;
    for (;;) { let i = -1; for (let k = 0; k < N; k++) if (!done[k] && best[k] < Infinity && (i < 0 || best[k] < best[i])) i = k; if (i < 0 || i === 1) break; done[i] = true;
      for (let j = 1; j < N; j++) { if (done[j]) continue; const w = best[i] + flat(nodes[i], nodes[j]); if (w >= best[j]) continue; if (!ok(nodes[i], nodes[j], dist[i], dist[j]) || !this.open(nodes[i], nodes[j])) continue; best[j] = w; from[j] = i; } }
    if (best[1] === Infinity) return null;
    const pts: Vec3[] = []; for (let i = 1; i > 0; i = from[i]) pts.unshift(nodes[i]);
    return { pts, round: true };
  }

  /** from a to b for a man's feet: clear of the leaves (roundLeaves) AND of the level — where the straight leg is not a man's width of open floor (a table
   *  between the door and his point, a wall between a late man and the lane), the nav grid's own path, each leg of it taken round the leaves */
  private way(a: Vec3, b: Vec3): { pts: Vec3[]; round: boolean } | null {
    const R = this.roundLeaves(a, b); if (!R) return null;
    let prev = a, clear = true; for (const q of R.pts) { if (!this.open(prev, q)) { clear = false; break; } prev = q; }
    if (clear) return R;
    const N = this.g.col.nav.findPath(a, b, 3000); if (!N || !N.length) return R;   // (no plan either: the straight leg, as the runner would have walked it)
    if (flat(N[N.length - 1], b) > 0.05) N.push(b);
    const pts: Vec3[] = []; let round = false; prev = a;
    for (const w of N) { if (flat(prev, w) < 0.05) continue; const r = this.roundLeaves(prev, w); if (!r) return null; pts.push(...r.pts); round ||= r.round; prev = w; }
    this.stats.nav++; return pts.length ? { pts, round } : R;
  }

  /** his line from where he stands: the lane through the opening while he is outside it, then round the leaves to his point */
  private lay(W: Walk) {
    const K = DOORPATH, d = W.door, p = W.gd.char.pos, h = d.def.hinge, ux = Math.cos(d.def.closedDir), uz = Math.sin(d.def.closedDir), wide = d.def.width;
    const vx = uz * W.op.side, vz = -ux * W.op.side;   // into the room: off the side the men stacked on
    const at = (s: number): Vec3 => [h[0] + ux * wide * K.at + vx * s, 0, h[1] + uz * wide * K.at + vz * s];
    const s0 = (p[0] - h[0]) * vx + (p[2] - h[1]) * vz, a0 = (p[0] - h[0]) * ux + (p[2] - h[1]) * uz;
    const pts: Vec3[] = [[p[0], 0, p[2]]]; W.lane = 0;
    if (s0 < K.inM - 0.25 && flat(p, W.pod) > 0.6) {
      let inM = K.inM; const far = at(inM); if (!this.free(far)) inM = 0.45; W.inM = inM;
      const Pa = at(-K.outM), Pi = at(inM);
      if (this.free(Pi) && (s0 > -0.15 || this.free(Pa))) { if (s0 < -K.outM - 0.2 || (s0 < -0.15 && Math.abs(a0 - wide * K.at) > 0.15)) { const R0 = this.way(pts[0], Pa); if (R0) pts.push(...R0.pts); else pts.push(Pa); }   // (a leaf standing open on HIS side of the wall is gone round as well)
        pts.push(Pi); W.lane = pts.length - 1; }
      else this.stats.noLane++;
    }
    const from = pts[pts.length - 1]; let R = this.way(from, W.pod); W.wait = false;
    if (!R && W.waited < K.cap && this.g.doors.list.some(x => !gone(x) && Math.abs(x.vel) > 0.15 && segSeg(from, W.pod, ...this.leaf(x)) < 2.5)) W.wait = true;   // (a leaf still swinging: no verdict on it yet)
    else if (!R) { const q = this.stand(W, from); if (q) { this.move(W, q); R = this.way(from, q); } }
    if (R) { pts.push(...R.pts); if (R.round && !W.round) { W.round = true; this.stats.round++; } } else if (!W.wait) { pts.push(W.pod); this.stats.noRound++; }
    W.pts = pts; W.len = [0]; for (let i = 1; i < pts.length; i++) W.len.push(W.len[i - 1] + flat(pts[i - 1], pts[i]));
    W.s = 0; W.laidT = this.g.time; W.arr = null;
    W.near = []; for (const x of this.g.doors.list) { if (gone(x)) continue; const L = this.leaf(x); let m = Infinity; for (let i = 1; i < pts.length; i++) m = Math.min(m, segSeg(pts[i - 1], pts[i], L[0], L[1])); if (m < 2.5) W.near.push({ d: x, angle: x.angle }); }
  }
  /** his point stands where no walk reaches it clear of a leaf: the nearest floor of the same room that one does, a man can stand on, `leafClear` off every
   *  leaf as it stands and `apartM` off every other point of his order — or null */
  private stand(W: Walk, from: Vec3, reach = DOORPATH.moveM): Vec3 | null {
    const K = DOORPATH, pod = W.pod0, L = this.leaves(pod, pod), others: Vec3[] = [], room = this.g.fight?.roomAt(pod) ?? null, down = K.bodyM > 0 ? this.fallen(pod, reach + K.bodyOff + 0.5) : [];
    for (const [m, q] of W.op.pod) if (m !== W.gd) others.push(q.pos); for (const q of W.op.queue) if (q.gd !== W.gd) others.push(q.pod.pos);
    for (let r = 0.3; r <= reach + 1e-6; r += 0.3) { let best: Vec3 | null = null, bd = Infinity;
      for (let k = 0; k < 16; k++) { const c: Vec3 = [pod[0] + Math.cos(k * Math.PI / 8) * r, 0, pod[2] + Math.sin(k * Math.PI / 8) * r];
        if (!this.free(c) || L.some(l => ptSeg(c, l[0], l[1]) < K.leafClear + 0.05) || others.some(o => flat(o, c) < K.apartM) || down.some(o => flat(o, c) < K.bodyOff) || (this.g.fight?.roomAt(c) ?? null) !== room) continue;
        const dd = flat(c, from); if (dd >= bd || !this.way(from, c)) continue; best = c; bd = dd; }
      if (best) return best; }
    return null;
  }
  /** where the men down lie within `r` of a point (hips, chest and head of each: a body is longer than it is wide) */
  private fallen(at: Vec3, r: number): Vec3[] {
    const out: Vec3[] = [];
    for (const x of [...this.g.guards, ...this.g.civilians]) { if (x.state !== 'dead' && !x.char.ragdoll) continue; const b = x.char.bones;
      for (const q of [b.hips, b.chest, b.head]) if (q && flat(q, at) < r) out.push([q[0], 0, q[2]]); }
    return out;
  }
  /** his point is `q` now: this walk's, the order's (orders.ts Assignment post / goal — the same object, so the door order still knows it for its own),
   *  the runner's, and the door order's record of where he was sent */
  private move(W: Walk, q: Vec3) {
    const gd = W.gd, old = W.pod, A = this.g.orders.of(gd), pod = W.op.pod.get(gd); W.pod = q; this.stats.moved++;
    if (A && A.post && flat(A.post, old) < 0.1) { A.post = [q[0], A.post[1], q[2]]; A.goal = [q[0], A.post[1], q[2]]; if (gd.script && gd.script.goal && flat(gd.script.goal, old) < 0.1) gd.script.goal = A.goal; }
    if (pod && flat(pod.pos, old) < 0.1) W.op.pod.set(gd, { ...pod, pos: [q[0], pod.pos[1], q[2]] });
  }

  /** doortools.ts, the entry: may the queue's i-th man be let go now? One at a time, in the order dealt, each once the man before him has his start. */
  mayGo(op: DoorOp, q: { gd: Guard; pod: { pos: Vec3 } }, i: number): boolean { return goneWithTick('DoorPath.mayGo') as any; }
  /** doortools.ts, the entry: this man has just been handed his point */
  sent(op: DoorOp, gd: Guard, pod: { pos: Vec3 }) {
    const L = this.lastOf.get(op) ?? { W: null, n: 0, t: 0 }, t = this.g.time;
    const W: Walk = { gd, op, door: op.door, pod: [pod.pos[0], 0, pod.pos[2]], n: L.n, t0: t, pts: [], len: [0], s: 0, laidT: t, near: [], lane: 0, via: [], arr: null, arrN: 0, yieldT: -1, yields: 0, held: 0, offT: NaN, round: false, wait: false, waited: 0, inM: DOORPATH.inM, pod0: [pod.pos[0], 0, pod.pos[2]], bodyT: -1 };
    this.own(W);
    this.lay(W); this.walks.set(gd, W); this.lastOf.set(op, { W, n: L.n + 1, t }); this.stats.sent++;
  }

  /** the order's own way round (orders.ts route: waypoints to its door) is this walk's while it lasts: taken off the order, kept, and handed back if the
   *  walk is dropped before he is through */
  private own(W: Walk) { const A = this.g.orders.of(W.gd); if (!A || !A.post || flat(A.post, W.pod) > 0.1 || !A.via.length) return false; W.via = A.via.slice(); A.via.length = 0; A.goal = [A.post[0], A.post[1], A.post[2]]; return true; }
  /** once a frame, after the orders have posed their men and before anybody moves */
  tick(dt: number): void { if (RUST_SURFACE?.tapFiring('DoorPath.tick')) return RUST_SURFACE.tapAnswer() as any; return goneWithTick('DoorPath.tick') as any; }
}
