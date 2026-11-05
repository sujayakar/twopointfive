// WALL SKIN — what is painted on a wall goes with the wall. A level's walls carry thin drawn-only boxes on their faces: a damp foot, a skirt, a dado, a
// painted band, a mend, a stripe, a run of conduit. wallbreak.ts opens a wall cell by cell and its `hangers` take what is MOUNTED on the stretch that opened
// (off the floor, half of what holds it gone, resting on nothing), but a skin that starts at the floor, or has lost less than half its wall, or rests on
// another drawn thing (a conduit run on its own branch) stayed where it was authored: a band of paint in the air across the foot of a hole and a man
// walking through it, a length of conduit across a breach with its clips gone.
//
// THE RULE, one for every skin: wherever it starts, whatever it rests on. A SKIN is an authored static that is drawn and nothing else (NoShadow: no ray,
// eye or body meets it), no deeper than WALLSKIN.thick across one of its level axes, lying square ON a standing face of a wall this engine opens (a
// partition, or a hard wall a charge took on): proud of that face, its near side within WALLSKIN.off of it or of another skin that is on it (a band, a
// mend over the band, a run of conduit and its clips and its branch, a card board and the paper pinned to it). Not by a piece's name, a kit, a group or a level: by what the box is and where it
// lies. A skin is DRAWN ONLY WHERE ITS WALL STILL STANDS BEHIND IT: over every cell that has opened behind it, it is cut square at the cell's edge, as the wall
// is, and what is left of it stays on the standing wall. Every way a cell opens comes through wallbreak.ts openCells (rounds, a blast, a charge on a hard
// wall, a collapse, a checkpoint's load), and that is the one place this is told. A skin that lies on two walls (one band along two runs) is cut by both.
//
// HIDDEN, NOT THROWN. Nothing falls, nothing enters the debris pool, no counter of the lane's is drawn from, and NO FLAG OF ANY BOX IS WRITTEN: the authored
// skin is left out of the picture by BoxWorld.undrawn (scene/boxes.ts: the world's own list of what it draws, where trim follows its box out the same way),
// and what stands of it is handed to the frame's dynamic list as boxes of its own paint (keyed MKEY.skin: bolted on, so the fog of war passes them as it
// passes a wall device, and the look draws them as the level's own and not as things that move: BoxWorld.packBox). So the simulation reads exactly what it read before: what `hangers` throws, it throws as it did (a skin it has taken is hidden by
// its flags, and its pieces go with it); a run is the same run by digest with the switch on or off.
//
// A SKIN THAT CARRIES MARKINGS (a band with stripes, a number on a plate, a sign) is cut where the wall is, with every mark where it was: its markings are
// laid out from the middle of the whole box, so what stands of it is THE WHOLE BOX AGAIN UNDER A CUT RECORD (scene/cut.ts: four planes at most, the
// rectangle that stands), not a smaller box. That needs the cut switch, which draws a record (on in the page): with it off, or where the box's own shape
// leaves no room for four more planes, a marked skin is left as it was (counted: `stats.uncut`) and nothing standing on it is cut. A plain skin that says
// a cut SHAPE is cut into smaller boxes, each under the shape's own planes. PAST A CAP (WALLSKIN.perSkin pieces of one skin, WALLSKIN.maxPieces in all) a skin is first cut by whole columns (a column with
// any cell open behind the skin counts as open: fewer, taller pieces), and if that is still too many it is left as it last stood (counted: `stats.capped`).
// WHAT THE HANGERS THROW AFTER IT WAS CUT comes off as the whole box it was authored as (the pool's chunk is the simulation's and is not this rule's to size).
// NOT A SKIN: anything a ray meets (a solid board, a shelf: the hangers' own), a thing deeper than `thick` (a duct, a case), a sheet standing further off
// the face than `off` with no skin under it (the face of something solid mounted there: it goes or stays with that thing), a sheet lying flat (thin in y: a rug's edge under a wall is the
// floor's), a tilted or a round box, one that glows, a fixture's or a light's box, the lane's own destructibles, and anything on a piece of
// COVER (what stands on or against cover falls by wallbreak's drop rule; a crate against a wall would otherwise claim the wall's band).
// THE CUT IS SQUARE at the column's edge, where the wall's standing piece ends (breakTable.ts `ragged` leaves a wall's piece square under the edge look).
// edgeLook.ts's plates stand up to a hand into the opening past that edge, in the wall's own paint: they are bare of the band. With the edge look off
// and the cut switch on, the piece's end is ragged and the band's is not.
//
// THE SWITCH: WALLSKIN.on — ON in the page (`?wallskin=0` = the bands as they were), OFF in the module and on the frozen stand (FROZEN_HOLDS; TPS_WALLSKIN=1
// raises it there); in the registry (game/knobs.ts) and the journal. CHECK: tools/qa/wallskin-check.ts (WALLSKIN_BREAK=1 must fail), both stands.
// ROLLBACK: everything here hangs off the Game (destr.walls.skins) and is walked with it; reset() drops it and a checkpoint's load re-derives it from the cells.
// Never seen on a GPU.
import { type Box, BoxFlag, MKEY, copyBox, boxDrawn } from '../scene/boxes';
import type { Game } from './game';
import type { BreakLike } from './edgeLook';
import { cutBoxesOn, MAX_CUT_PLANES, type CutPlane } from '../scene/cut';

export const WALLSKIN = {
  on: false,
  /** m: a skin is no deeper than this across the face it lies on: a coat of paint, a sheet, a run of conduit and its clips, a board of card. Deeper is a
   *  body (a duct, a case, a cabinet's drawn front) and stays the hangers' alone */
  thick: 0.04,
  /** m: … and its near side stands no further than this off that face, or off the outer side of another skin that is on it: ON it, a coat over a coat (a
   *  mend plastered over a dado stands 8 mm off the wall; a paper pinned to a card board stands on the board). Measured over every level (11-05, the
   *  shipped stand): of the thin drawn-only sheets within 12 cm of a wall a charge or a round can open, 975 stand within 1 cm of its face; every one of the
   *  246 further off is the face of something MOUNTED there (a paper on a board, a label on a shelf, a rack's rail). On a board that is itself a skin it is
   *  cut with the board; on anything else it goes or stays with that thing and is not the wall's to cut */
  off: 0.01,
  /** caps: the pieces one skin may stand in, and all skins together (past either the skin is cut by whole columns, or left as it last stood, and counted) */
  perSkin: 12, maxPieces: 256,
  /** the frame's box list keeps this many rows free after ours (BoxWorld throws past WORLD.maxBoxes) */
  boxSlack: 64,
};

type V3 = [number, number, number];
/** a rectangle on a skin's face, in its own frame: along it, then up it */
type Rect = [u0: number, u1: number, v0: number, v1: number];
/** a skin lying on a face of one breakable: the axis of THAT box's frame the skin is thin along, and which of its two faces across it */
interface Seat { W: BreakLike; n: 0 | 2; face: 1 | -1; /** … and the axis of the skin's OWN frame that is */ t: 0 | 2;
  /** how far off that face its near and far sides stand (m; near < 0: sunk in), and its bounds along the wall and up it, in the wall's frame */ near: number; far: number; u0: number; u1: number; y0: number; y1: number }
const EPS = 1e-4;
/** a point in a box's own frame (its centre the origin, its yaw undone), and back: wallbreak.ts toLocal / toWorld */
const into = (b: Box, p: Readonly<V3>): V3 => { const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), x = p[0] - b.c[0], z = p[2] - b.c[2]; return [cs * x - sn * z, p[1] - b.c[1], sn * x + cs * z]; };
const outOf = (b: Box, l: Readonly<V3>): V3 => { const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); return [b.c[0] + cs * l[0] + sn * l[2], b.c[1] + l[1], b.c[2] - sn * l[0] + cs * l[2]]; };

/** WHAT STANDS of a face 2·hu × 2·hv once the `cuts` are taken out of it: rectangles, each as long along the face as it can be, rows of one outline joined */
export function standing(hu: number, hv: number, cuts: readonly Rect[]): Rect[] {
  const edges = (h: number, k: 0 | 2): number[] => { const e = [-h, h]; for (const c of cuts) for (const x of [c[k], c[k + 1]]) if (x > -h + EPS && x < h - EPS && !e.some(y => Math.abs(y - x) <= EPS)) e.push(x); return e.sort((p, q) => p - q); };
  const us = edges(hu, 0), vs = edges(hv, 2), out: Rect[] = []; let last: Rect[] = [], lastKey = '';
  for (let j = 0; j + 1 < vs.length; j++) {
    const vm = (vs[j] + vs[j + 1]) / 2, row: Rect[] = []; let from = -1;
    for (let i = 0; i + 1 < us.length; i++) {
      const um = (us[i] + us[i + 1]) / 2, gone = cuts.some(c => um > c[0] && um < c[1] && vm > c[2] && vm < c[3]);
      if (!gone && from < 0) from = i; else if (gone && from >= 0) { row.push([us[from], us[i], vs[j], vs[j + 1]]); from = -1; }
    }
    if (from >= 0) row.push([us[from], us[us.length - 1], vs[j], vs[j + 1]]);
    const key = row.map(r => `${r[0]}:${r[1]}`).join('|');
    if (j > 0 && key === lastKey) for (const r of last) r[3] = vs[j + 1]; else { for (const r of row) out.push(r); last = row; lastKey = key; }
  }
  return out;
}

export class WallSkin {
  /** THE SKINS THAT ARE CUT: authored static → the boxes that stand of it now (none: every part of it lies over open cells, or it could not be cut square).
   *  The world leaves the authored box out of the picture while its number is here (BoxWorld.undrawn is this very map) */
  readonly cut = new Map<number, Box[]>();
  /** each such skin's seats on the things that have opened behind it */
  private seats = new Map<number, Seat[]>();
  /** a breakable's skins, found the first time a cell of it opens (the level as authored: the same answer whenever it is asked) */
  private on = new Map<number, number[]>();
  /** the authored statics that could be a skin of anything: drawn and nothing else, level, lightless */
  private cand: number[] = [];
  /** pieces standing · handed to the last frame · what the frame's list can take of them · an error has put the rule down for this level (the picture never costs play a frame) */
  private n = 0; private last = 0; private room = 1e9; private failed = false;
  /** counts, this level since it was last put back: skins cut now, of them with nothing left standing · pieces standing · times a cap left a skin as it stood ·
   *  times one was cut by whole columns for a cap · marked skins with open wall behind them that no cut record could draw, left as they were */
  readonly stats = { cut: 0, whole: 0, pieces: 0, capped: 0, coarse: 0, uncut: 0 };

  /** `own(j)`: static j has a road of its own (a fixture's box, a light's, one of the lane's destructibles) and is nobody's skin */
  constructor(private g: Game, authored: number, private own: (j: number) => boolean) {
    const boxes = g.col.boxes, lone = BoxFlag.NoShadow | BoxFlag.NoPrimary | BoxFlag.Glass | BoxFlag.Dynamic;
    for (let i = 0; i < authored; i++) {
      const b = boxes[i];
      if ((b.flags & lone) === BoxFlag.NoShadow && !b.rot && b.round !== true && b.emissive[0] + b.emissive[1] + b.emissive[2] <= 0) this.cand.push(i);   // (trim a box says is asked like any other: what it is and where it lies)
    }
  }

  // ---------------------------------------------------------------- the hook (wallbreak.ts openCells)
  /** cells of B have just opened (a checkpoint's load too): every skin lying on it is cut again from the cells as they stand */
  opened(B: BreakLike) {
    if (!WALLSKIN.on || this.failed || B.kind !== 'wall') return;
    try {
      for (const j of this.skinsOf(B)) this.recut(j);
      this.count();
    } catch (e) { this.clear(); this.failed = true; console.error('wall skin switched off:', e); }
  }
  /** the level put back (WallBreak.reset): every skin whole again */
  clear() { this.cut.clear(); this.seats.clear(); this.on.clear(); this.marked.clear(); this.n = this.last = 0; this.room = 1e9; this.failed = false; const S = this.stats; S.capped = S.coarse = S.uncut = 0; this.count(); }
  private marked = new Set<number>();
  private count() { let whole = 0; for (const l of this.cut.values()) if (!l.length) whole++; const S = this.stats; S.cut = this.cut.size; S.whole = whole; S.pieces = this.n; }

  /** once a frame, before the frame's box list is made again (Destruction.update, beside edgeLook's): what the list can still take of ours — last frame's
   *  boxes (the PICTURE's under the fog of war, which can be the longer) less ours */
  update() {
    if (!this.cut.size) return;
    const g = this.g, world = g.engine?.world as { maxBoxes?: number; statics?: unknown[] } | undefined;
    if (world && typeof world.maxBoxes === 'number' && Array.isArray(world.statics)) this.room = world.maxBoxes - world.statics.length - Math.max(0, Math.max(g.dynRaw, g.dynBoxes.length) - this.last) - WALLSKIN.boxSlack;
  }
  /** what stands of the cut skins, into the frame's dynamic list, and the world told which authored boxes to leave out (every frame: the world is the
   *  host's, and holds no word of ours a tick put back could leave stale). A skin something else has since hidden (the hangers threw it, the box that says
   *  it as trim went) is gone whole: its pieces are not handed over */
  boxes(out: Box[]) {
    const g = this.g, world = g.engine?.world as { undrawn?: unknown } | undefined;
    if (this.cut.size && !WALLSKIN.on) this.clear();   // (put down under a standing game: the bands as authored)
    if (world && 'undrawn' in world) world.undrawn = this.cut.size ? this.cut : null;
    if (!this.cut.size) return;
    const boxes = g.col.boxes; let n = 0;
    for (const [j, l] of this.cut) { if (!boxDrawn(boxes, boxes[j])) continue; for (const p of l) { if (n >= this.room) break; out.push(p); n++; } }
    this.last = n;
  }

  // ---------------------------------------------------------------- which boxes are a thing's skins
  /** where sheet `s` lies against breakable W, or null: square to W, no deeper than WALLSKIN.thick across one of W's level axes (and no thinner up than
   *  across: a strip lying flat at a wall's foot is the floor's), overlapping W along the other and in height, and PROUD of the face across the thin one (a
   *  sheet sunk inside the box, the band of another wall running through where this one butts in, is not on its face). How far off the face it may stand
   *  is `skinsOf`'s to say */
  private seat(W: BreakLike, s: Box): Seat | null {
    const w = W.box, r = s.yaw - w.yaw; if (Math.abs(Math.sin(2 * r)) > 1e-4) return null;
    const turned = Math.abs(Math.sin(r)) > 0.5, hx = turned ? s.h[2] : s.h[0], hz = turned ? s.h[0] : s.h[2], l = into(w, s.c);
    if (Math.min(l[1] + s.h[1], w.h[1]) - Math.max(l[1] - s.h[1], -w.h[1]) <= EPS) return null;
    for (const n of (hx <= hz ? [0, 2] : [2, 0]) as (0 | 2)[]) {
      const u = (2 - n) as 0 | 2, hn = n === 0 ? hx : hz, hu = n === 0 ? hz : hx, H = w.h[n];
      if (2 * hn > WALLSKIN.thick + EPS || hn > s.h[1] + EPS || Math.min(l[u] + hu, w.h[u]) - Math.max(l[u] - hu, -w.h[u]) <= EPS) continue;
      const t = (turned ? 2 - n : n) as 0 | 2, at = { W, n, t, u0: l[u] - hu, u1: l[u] + hu, y0: l[1] - s.h[1], y1: l[1] + s.h[1] };
      if (l[n] + hn - H > EPS) return { ...at, face: 1, near: l[n] - hn - H, far: l[n] + hn - H };
      if (-H - (l[n] - hn) > EPS) return { ...at, face: -1, near: -H - (l[n] + hn), far: -H - (l[n] - hn) };
    }
    return null;
  }
  /** a wall's skins: of the sheets that lie against it, those ON a face — the near side within WALLSKIN.off of the face itself, or of the outer side of a
   *  skin already on it that it overlaps (a coat over a coat: nearest the wall first). A sheet that carries markings holds nothing up while the cut switch is off:
   *  it is not cut then, so nothing on it is */
  private skinsOf(B: BreakLike): number[] {
    let L = this.on.get(B.id); if (L) return L;
    const boxes = this.g.col.boxes, w = B.box, reach = Math.max(w.h[0], w.h[2]) + WALLSKIN.off + WALLSKIN.thick, lie: { j: number; seat: Seat }[] = [];
    for (const j of this.cand) {
      const s = boxes[j], far = reach + Math.max(s.h[0], s.h[2]); if (Math.abs(s.c[0] - w.c[0]) > far || Math.abs(s.c[2] - w.c[2]) > far || this.own(j)) continue;
      const seat = this.seat(B, s); if (seat) lie.push({ j, seat });
    }
    lie.sort((p, q) => p.seat.near - q.seat.near || p.j - q.j);
    L = []; const got: Seat[] = [];
    for (const { j, seat: a } of lie) {
      const on = a.near <= WALLSKIN.off || got.some(b => b.n === a.n && b.face === a.face && a.near - b.far <= WALLSKIN.off && a.far > b.far && Math.min(a.u1, b.u1) - Math.max(a.u0, b.u0) > EPS && Math.min(a.y1, b.y1) - Math.max(a.y0, b.y0) > EPS);
      if (!on) continue;
      L.push(j); const S = this.seats.get(j); if (S) S.push(a); else this.seats.set(j, [a]);
      if (!(boxes[j].marks && boxes[j].marks!.length) || cutBoxesOn()) got.push(a);
    }
    L.sort((p, q) => p - q);
    this.on.set(B.id, L);
    return L;
  }

  // ---------------------------------------------------------------- cutting one
  /** the open cells behind a skin, as rectangles on its face in its own frame (run axis `a`): of each wall it lies on, the layer of cells at the face it
   *  lies on. `cols`: a column with any such cell counts as open over the skin's whole height (the coarse cut a cap falls back to) */
  private cuts(s: Box, seats: Seat[], a: 0 | 2, cols: boolean): Rect[] {
    const out: Rect[] = [];
    for (const { W, n, face, t } of seats) {
      if (2 - t !== a) continue;   // (a seat across the skin's other axis: a thin upright on two walls at a corner is cut along one)
      const w = W.box, u = 2 - n, N = n === 0 ? W.nz : W.nx, at = face > 0 ? (n === 0 ? W.nx : W.nz) - 1 : 0;
      for (let k = 0; k < N; k++) for (let iy = 0; iy < W.ny; iy++) {
        const ix = n === 0 ? at : k, iz = n === 0 ? k : at; if (!W.open[(iy * W.nz + iz) * W.nx + ix]) continue;
        const lo: V3 = [0, W.yb[iy], 0], hi: V3 = [0, W.yb[iy + 1], 0];
        lo[n] = hi[n] = face * w.h[n]; lo[u] = -w.h[u] + 2 * w.h[u] * k / N; hi[u] = -w.h[u] + 2 * w.h[u] * (k + 1) / N;
        const p = into(s, outOf(w, lo)), q = into(s, outOf(w, hi)), r: Rect = [Math.min(p[a], q[a]), Math.max(p[a], q[a]), Math.min(p[1], q[1]), Math.max(p[1], q[1])];
        if (Math.min(r[1], s.h[a]) - Math.max(r[0], -s.h[a]) <= EPS || Math.min(r[3], s.h[1]) - Math.max(r[2], -s.h[1]) <= EPS) continue;
        if (cols) { r[2] = -s.h[1] - 1; r[3] = s.h[1] + 1; }
        out.push(r);
      }
    }
    return out;
  }
  private recut(j: number) {
    const s = this.g.col.boxes[j], seats = this.seats.get(j) ?? [], K = WALLSKIN; if (!seats.length) return;
    const t = seats[0].t, a = (2 - t) as 0 | 2, cuts = this.cuts(s, seats, a, false), had = this.cut.get(j)?.length ?? 0;
    if (!cuts.length) { this.n -= had; this.cut.delete(j); return; }   // (nothing behind it has opened: the authored box, whole)
    const marked = !!(s.marks && s.marks.length), base = s.cut ?? [];
    if (marked && (!cutBoxesOn() || base.length + 4 > MAX_CUT_PLANES)) { if (!this.marked.has(j)) { this.marked.add(j); this.stats.uncut++; } return; }   // (its marks are laid out on the whole box and no record can be drawn, or has room: left as it was)
    const fits = (r: Rect[]) => r.length <= K.perSkin && this.n - had + r.length <= K.maxPieces;
    let rects = standing(s.h[a], s.h[1], cuts);
    if (!fits(rects)) { rects = standing(s.h[a], s.h[1], this.cuts(s, seats, a, true)); if (!fits(rects)) { this.stats.capped++; return; } this.stats.coarse++; }   // (past a cap: by whole columns, else as it last stood)
    this.n += rects.length - had;
    this.cut.set(j, rects.map(r => {
      const lc: V3 = [0, (r[2] + r[3]) / 2, 0], h: V3 = [0, (r[3] - r[2]) / 2, 0]; lc[a] = (r[0] + r[1]) / 2; h[a] = (r[1] - r[0]) / 2; h[t] = s.h[t];
      const p = copyBox(s); p.flags = (s.flags & ~BoxFlag.NoPrimary) | BoxFlag.Dynamic; p.name = 'skin';
      // (its key is its place on the skin, to the millimetre: a piece a later cut leaves alone keeps its key, and one that changes is a new thing to the
      //  picture's memory of where each box was — never an old key on another stretch of the band)
      let k = Math.imul(j + 1, 0x9E3779B1) ^ Math.imul(Math.round(lc[a] * 1000), 0x85EBCA77) ^ Math.imul(Math.round(lc[1] * 1000), 0xC2B2AE3D); k ^= k >>> 15; k = Math.imul(k, 0x2C1B3C6D);
      p.mkey = MKEY.skin + (k >>> 8);
      if (marked) {   // the whole box again, under the planes of the rectangle that stands (a side that is the box's own takes none): every mark where it was
        const planes: CutPlane[] = base.slice(), side = (ax: 0 | 1 | 2, sign: 1 | -1, at: number) => { const n: V3 = [0, 0, 0]; n[ax] = sign; planes.push({ n, d: sign * at }); };
        if (r[0] > -s.h[a] + EPS) side(a, -1, r[0]); if (r[1] < s.h[a] - EPS) side(a, 1, r[1]); if (r[2] > -s.h[1] + EPS) side(1, -1, r[2]); if (r[3] < s.h[1] - EPS) side(1, 1, r[3]);
        p.c = [s.c[0], s.c[1], s.c[2]]; p.h = [s.h[0], s.h[1], s.h[2]]; p.cut = planes;
      } else {
        p.c = outOf(s, lc); p.h = h;
        if (base.length) p.cut = base.map(c => ({ n: c.n, d: c.d - (c.n[0] * lc[0] + c.n[1] * lc[1] + c.n[2] * lc[2]) }));   // (a shape's planes are in the box's own frame: the same planes about the piece's middle)
      }
      p.was = undefined; p.shape = undefined; p.shaped = undefined; p.piece = undefined; p.of = undefined;   // (a plain box of the skin's paint: what the level's stand-up keeps on the authored one is not its)
      return p;
    }));
  }
}
