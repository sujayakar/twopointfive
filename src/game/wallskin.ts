// WALL SKIN — what is painted on a wall goes with the wall. A level's walls carry thin drawn-only boxes on their faces: a damp foot, a skirt, a dado, a
// painted band, a mend, a stripe, a run of conduit, a card board and its paper. wallbreak.ts opens a wall cell by cell and its `hangers` take what is
// MOUNTED on the stretch that opened (off the floor, half of what holds it gone, resting on nothing), but a sheet that starts at the floor, or has lost
// less than half its wall, or rests on another drawn thing (a conduit run on its own branch) stayed where it was authored: a band of paint in the air
// across the foot of a hole and a man walking through it, a length of conduit across a breach with its clips gone.
//
// THE RULE, one for every sheet and every solid: A SHEET IS DRAWN WHERE SOMETHING SOLID STILL STANDS BEHIND IT. A SHEET is an authored static that is
// drawn and nothing else (NoShadow: no ray, eye or body meets it), no deeper than WALLSKIN.thick across one of its level axes. WHAT IT LIES ON, as the
// level was authored: every solid box (one a ray meets and the picture draws: a partition, a hard wall, a low wall, a pier, the end of the wall round the
// corner) that it lies square against, proud of that box's face, its near side within WALLSKIN.off of the face. Its BACK is the side more of it lies
// against, and each part of it lies on THE NEAREST solid behind that part (a floor slab's edge inside the wall's thickness is behind the wall, not
// under the paint). Not by a piece's name, a kit, a group or a level, and not by the kind of the thing behind: by what the box is and where it lies.
// What stands of a solid is the solid's own to say: a thing wallbreak.ts opens (a partition, a hard wall a charge took on, cover) by its cells, the
// layer at the face the sheet lies on; any other solid whole, until something hides it. A part of the sheet is taken out of the picture when it lay on
// something and NOTHING it lay on stands behind that part any more: over an opened cell it is cut square at the cell's edge, as the wall is, unless
// another solid still stands there (at a corner where two walls overlap the band on the standing one is whole, and so is the opened one's band over the
// standing one's end). A part that lay on nothing as authored (a band run across a gap) stays as authored. Every way a cell opens comes through
// wallbreak.ts openCells (rounds, a blast, a charge on a hard wall, a collapse, a checkpoint's load), and that is the one place this is told.
//
// HIDDEN, NOT THROWN. Nothing falls, nothing enters the debris pool, no counter of the lane's is drawn from, and NO FLAG OF ANY BOX IS WRITTEN: the
// authored sheet is left out of the picture by BoxWorld.undrawn (scene/boxes.ts: the world's own list of what it draws, where trim follows its box out
// the same way), and what stands of it is handed to the frame's dynamic list as boxes of its own paint (keyed MKEY.skin: bolted on, so the fog of war
// passes them as it passes a wall device, and the look draws them as the level's own and not as things that move: BoxWorld.packBox). So the simulation
// reads exactly what it read before: what `hangers` throws, it throws as it did (a sheet it has taken is hidden by its flags, and its pieces go with
// it); a run is the same run by digest with the switch on or off. A frame whose box list has no room for what stands of a sheet draws the authored
// sheet whole for that frame: never neither.
//
// A SHEET THAT CARRIES MARKINGS (a band with stripes, a number on a plate, a sign) is cut where the wall is, with every mark where it was: its markings
// are laid out from the middle of the whole box, so what stands of it is THE WHOLE BOX AGAIN UNDER A CUT RECORD (scene/cut.ts: four planes at most, the
// rectangle that stands), not a smaller box. That needs the cut switch, which draws a record (on in the page): with it off, or where the box's own shape
// leaves no room for four more planes, a marked sheet is left as it was (counted: `stats.uncut`). A plain sheet that says a cut SHAPE is cut into
// smaller boxes, each under the shape's own planes. PAST A CAP (WALLSKIN.perSkin pieces of one sheet, WALLSKIN.maxPieces in all) a sheet is first cut
// by whole columns (a column with any part gone counts as gone: fewer, taller pieces), and if that is still too many it is left as it last stood
// (counted: `stats.capped`).
// WHAT THE HANGERS THROW AFTER IT WAS CUT comes off as the whole box it was authored as (the pool's chunk is the simulation's and is not this rule's to size).
// NOT A SHEET OF THIS RULE: anything a ray meets (a solid board, a shelf: the hangers' own), a thing deeper than `thick` (a duct, a case), a sheet lying
// flat (thin in y: a rug's edge under a wall is the floor's), a tilted or a round box, one that glows, a fixture's or a light's box, the lane's own
// destructibles, and a sheet with nothing solid within `off` behind it.
// THE CUT IS SQUARE at the column's edge, where the wall's standing piece ends (breakTable.ts `ragged` leaves a wall's piece square under the edge look).
// edgeLook.ts's plates stand up to a hand into the opening past that edge, in the wall's own paint: they are bare of the band. With the edge look off
// and the cut switch on, the piece's end is ragged and the band's is not.
//
// THE SWITCH: WALLSKIN.on — ON in the page (`?wallskin=0` = the bands as they were), OFF in the module and on the frozen stand (FROZEN_HOLDS; TPS_WALLSKIN=1
// raises it there); in the registry (game/knobs.ts) and the journal. Raised under a standing game, every wall that has opened is asked again; put down,
// every sheet is whole again. CHECK: tools/qa/wallskin-check.ts (WALLSKIN_BREAK=1 must fail), both stands: its oracle is the level's own collision.
// ROLLBACK: everything here hangs off the Game (destr.walls.skins) and is walked with it; reset() drops it and a checkpoint's load re-derives it from the cells.
// SEEN ON A GPU: an earlier cut of the rule once, in stills of one wall; this one never.
import { type Box, BoxFlag, MKEY, copyBox, boxDrawn } from '../scene/boxes';
import type { Game } from './game';
import { frameRoom, type BreakLike } from './edgeLook';
import { cutBoxesOn, cutRecord, MAX_CUT_PLANES, type CutPlane } from '../scene/cut';

export const WALLSKIN = {
  on: false,
  /** m: a sheet is no deeper than this across the face it lies on: a coat of paint, a run of conduit and its clips, a board of card. Deeper is a body
   *  (a duct, a case, a cabinet's drawn front) and stays the hangers' alone */
  thick: 0.04,
  /** m: … and its near side stands no further than this off the face of the solid behind it. Bands are authored on the face or a few millimetres proud,
   *  but not all: of the levels' own, a dado stands 6 cm off its partition and a cork board 1.5 cm off its, with nothing between. The nearest solid this
   *  near behind a part of a sheet is what that part is on: a paper on the front of a solid board is on the board, not on the wall behind the board */
  off: 0.08,
  /** caps: the pieces one sheet may stand in (16 at most: a piece's key is its sheet and one of 16 places), and all sheets together (past either the
   *  sheet is cut by whole columns, or left as it last stood, and counted) */
  perSkin: 12, maxPieces: 256,
  /** the frame's box list keeps this many rows free after ours (BoxWorld throws past WORLD.maxBoxes) */
  boxSlack: 64,
};

type V3 = [number, number, number];
/** what this module reads of a thing wallbreak.ts opens: its cells, and the statics that stand in for what is left of it (the first `live` of `slots`) */
type Brk = BreakLike & { slots: number[]; live: number };
/** a rectangle on a sheet's face, in its own frame: along it, then up it */
type Rect = [u0: number, u1: number, v0: number, v1: number];
/** a sheet lying on a face of solid `k` (an authored static): the axis of THAT box's frame the sheet is thin along and which of its two faces across
 *  it; the axis of the sheet's OWN frame that is, and which side of the sheet the solid is on; how far off the face the sheet's near side stands (m;
 *  under 0: sunk in); and the part of the sheet that lies on it, in the sheet's own frame */
interface Seat { k: number; n: 0 | 2; face: 1 | -1; t: 0 | 2; side: 1 | -1; near: number; r: Rect }
const EPS = 1e-4, ALL: Rect = [-1e9, 1e9, -1e9, 1e9], SLOTS = 16;
/** m: two solids whose faces stand this near one plane behind a sheet are both what it lies on there */
const FLUSH = 0.002;
/** a point in a box's own frame (its centre the origin, its yaw undone), and back */
const into = (b: Box, p: Readonly<V3>): V3 => { const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), x = p[0] - b.c[0], z = p[2] - b.c[2]; return [cs * x - sn * z, p[1] - b.c[1], sn * x + cs * z]; };
const outOf = (b: Box, l: Readonly<V3>): V3 => { const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); return [b.c[0] + cs * l[0] + sn * l[2], b.c[1] + l[1], b.c[2] - sn * l[0] + cs * l[2]]; };
const inside = (r: Readonly<Rect>, u: number, v: number): boolean => u > r[0] && u < r[1] && v > r[2] && v < r[3];

/** WHAT STANDS of a face 2·hu × 2·hv: `on[i]` is a part of it that has something behind it, `near[i]` how far behind, and `open[i]` the rectangles of
 *  that thing that have gone. A POINT LIES ON THE NEAREST THING BEHIND IT (and on any other within FLUSH of that: two walls' faces in one plane at a
 *  corner): what stands further back, behind that thing, is not what it is on. A point is gone when it lay on something and nothing it lay on stands
 *  there; the rest stands, in rectangles each as long along the face as it can be, rows of one outline joined. `cols`: a column with any point gone is
 *  gone over its whole height (the coarse cut a cap falls back to). `parts`: rectangles no standing rectangle may straddle (each is wholly in one of
 *  them or in none) */
export function standing(hu: number, hv: number, on: readonly Readonly<Rect>[], near: readonly number[], open: readonly (readonly Readonly<Rect>[])[], cols = false, parts: readonly Readonly<Rect>[] = []): Rect[] {
  const edges = (h: number, k: 0 | 2): number[] => { const e = [-h, h], add = (x: number) => { if (x > -h + EPS && x < h - EPS && !e.some(y => Math.abs(y - x) <= EPS)) e.push(x); };
    for (const r of on) { add(r[k]); add(r[k + 1]); } for (const l of open) for (const r of l) { add(r[k]); add(r[k + 1]); } for (const r of parts) { add(r[k]); add(r[k + 1]); } return e.sort((p, q) => p - q); };
  const gone = (u: number, v: number): boolean => { let m = 1e9; for (let i = 0; i < on.length; i++) if (near[i] < m && inside(on[i], u, v)) m = near[i];
    for (let i = 0; i < on.length; i++) if (near[i] <= m + FLUSH && inside(on[i], u, v) && !open[i].some(r => inside(r, u, v))) return false;
    return m < 1e9; };
  const part = (u: number, v: number): number => parts.findIndex(r => inside(r, u, v));
  const us = edges(hu, 0), vs = edges(hv, 2), out: Rect[] = []; let last: Rect[] = [], lastKey = '';
  const col = cols ? us.slice(0, -1).map((_, i) => vs.slice(0, -1).some((_v, j) => gone((us[i] + us[i + 1]) / 2, (vs[j] + vs[j + 1]) / 2))) : null;
  for (let j = 0; j + 1 < vs.length; j++) {
    const vm = (vs[j] + vs[j + 1]) / 2, row: Rect[] = [], tags: number[] = []; let from = -1, tag = -1;
    for (let i = 0; i + 1 < us.length; i++) {
      const um = (us[i] + us[i + 1]) / 2, g = col ? col[i] : gone(um, vm), k = g || !parts.length ? -1 : part(um, vm);
      if (from >= 0 && (g || k !== tag)) { row.push([us[from], us[i], vs[j], vs[j + 1]]); tags.push(tag); from = -1; }
      if (!g && from < 0) { from = i; tag = k; }
    }
    if (from >= 0) { row.push([us[from], us[us.length - 1], vs[j], vs[j + 1]]); tags.push(tag); }
    const key = row.map((r, i) => `${r[0]}:${r[1]}:${tags[i]}`).join('|');
    if (j > 0 && key === lastKey) for (const r of last) r[3] = vs[j + 1]; else { for (const r of row) out.push(r); last = row; lastKey = key; }
  }
  return out;
}

export class WallSkin {
  /** THE SHEETS THAT ARE CUT: authored static → the boxes that stand of it now (none: nothing it lay on stands behind any part of it) */
  readonly cut = new Map<number, Box[]>();
  /** … and of them, those whose standing boxes this frame's list took: the world leaves these authored boxes out of the picture (BoxWorld.undrawn is this very set) */
  readonly out = new Set<number>();
  /** each piece's rectangle on its sheet and its place among the sheet's 16 keys, beside `cut` */
  private laid = new Map<number, { r: Rect; slot: number }[]>();
  /** what each sheet lies on, found the first time something near it opens (the level as authored: the same answer whenever it is asked; none: not a sheet of this rule) */
  private seats = new Map<number, Seat[]>();
  /** a breakable's sheets, found the first time a cell of it opens */
  private on = new Map<number, number[]>();
  /** sheets whose solids have changed since they were last cut: cut again as the frame's boxes are handed over, once however many cells opened */
  private dirty = new Set<number>();
  /** the authored statics that could be a sheet: drawn and nothing else, level, lightless; and those that are solid: met by rays and drawn, level, square */
  private cand: number[] = []; private solid: Uint8Array;
  /** pieces handed to the last frame · what the frame's list can take of them · the switch as the last frame found it · an error has put the rule down for this level (the picture never costs play a frame) */
  private last = 0; private room = 1e9; private was = false; private failed = false;
  private marked = new Set<number>();
  /** counts, this level since it was last put back: sheets cut now, of them with nothing left standing · pieces in the frame's list (as the last frame was made) · times a cap left a sheet as it stood ·
   *  times one was cut by whole columns for a cap · marked sheets with nothing behind a part of them that no cut record could draw, left as they were */
  readonly stats = { cut: 0, whole: 0, pieces: 0, capped: 0, coarse: 0, uncut: 0 };

  /** `own(j)`: static j has a road of its own (a fixture's box, a light's, one of the lane's destructibles) and is no sheet of this rule. `brk(k)`: the
   *  breakable whose own box static k is, if wallbreak.ts opens it. `all()`: every breakable there is now */
  constructor(private g: Game, authored: number, private own: (j: number) => boolean, private brk: (k: number) => Brk | undefined, private all: () => readonly Brk[]) {
    const boxes = g.col.boxes, lone = BoxFlag.NoShadow | BoxFlag.NoPrimary | BoxFlag.Glass | BoxFlag.Dynamic; this.solid = new Uint8Array(authored);
    for (let i = 0; i < authored; i++) {
      const b = boxes[i]; if (b.rot || b.round === true) continue;
      if ((b.flags & lone) === 0) this.solid[i] = 1;
      else if ((b.flags & lone) === BoxFlag.NoShadow && b.emissive[0] + b.emissive[1] + b.emissive[2] <= 0) this.cand.push(i);   // (trim a box says is asked like any other: what it is and where it lies)
    }
  }

  // ---------------------------------------------------------------- the hook (wallbreak.ts openCells)
  /** cells of B have just opened (a checkpoint's load too): every sheet lying on it is cut again from what stands behind it, as this frame's boxes are handed over */
  opened(B: BreakLike) {
    if (!WALLSKIN.on || this.failed) return;
    try { for (const j of this.skinsOf(B)) this.dirty.add(j); } catch (e) { this.fail(e); }
  }
  /** the level put back (WallBreak.reset): every sheet whole again */
  clear() { this.cut.clear(); this.out.clear(); this.laid.clear(); this.seats.clear(); this.on.clear(); this.dirty.clear(); this.marked.clear(); this.last = 0; this.room = 1e9; this.was = this.failed = false; const S = this.stats; S.capped = S.coarse = S.uncut = 0; this.count(); }
  private fail(e: unknown) { this.clear(); this.failed = true; console.error('wall skin switched off:', e); }
  private count() { let whole = 0; for (const l of this.cut.values()) if (!l.length) whole++; const S = this.stats; S.cut = this.cut.size; S.whole = whole; S.pieces = this.last; }

  /** once a frame, before the frame's box list is made again (Destruction.update, beside edgeLook's): what the list can still take of ours */
  update() { if (this.cut.size || this.dirty.size) this.room = frameRoom(this.g, this.last, WALLSKIN.boxSlack) ?? this.room; }
  /** the rule is in force on this level: the switch is up and no error has put it down */
  get live(): boolean { return WALLSKIN.on && !this.failed; }
  /** what stands of the cut sheets, into the frame's dynamic list, and the world told which authored boxes to leave out (every frame: the world is the
   *  host's, and holds no word of ours a tick put back could leave stale). A sheet something else has since hidden (the hangers threw it, the box that
   *  says it as trim went) is gone whole: its pieces are not handed over. A sheet whose pieces the list has no room for is drawn as authored this frame */
  boxes(out: Box[]) {
    const g = this.g, world = g.engine?.world, on = WALLSKIN.on && !this.failed;
    if (!on && (this.cut.size || this.dirty.size)) { const f = this.failed; this.clear(); this.failed = f; }   // (put down under a standing game: the bands as authored)
    if (on && !this.was) for (const B of this.all()) if (B.open.some(x => x !== 0)) this.opened(B);   // (raised under a standing game: every wall that has opened is asked again)
    this.was = on;
    if (this.dirty.size) try { for (const j of [...this.dirty].sort((p, q) => p - q)) this.recut(j); this.dirty.clear(); this.count(); } catch (e) { this.fail(e); }
    this.out.clear();
    if (this.cut.size) {
      const boxes = g.col.boxes, room = Math.min(this.room, frameRoom(g, 0, WALLSKIN.boxSlack, out.length) ?? this.room); let n = 0;   // (… and no more than the list has left as it stands now)
      for (const [j, l] of this.cut) { if (!boxDrawn(boxes, boxes[j]) || (l.length && n + l.length > room)) continue; for (const p of l) out.push(p); n += l.length; this.out.add(j); }   // (a sheet with nothing left standing needs no room to stay out)
      this.last = n;
    } else this.last = 0;
    this.stats.pieces = this.last;
    if (world) world.undrawn = this.out.size ? this.out : null;
  }

  // ---------------------------------------------------------------- what a sheet lies on
  /** where sheet `s`, thin along its own axis `t`, lies against authored solid `k`, or null: square to it, overlapping it along the face and in height,
   *  PROUD of the face across the thin axis (a sheet sunk inside the box, the band of another wall running through where this one butts in, is not on
   *  its face) and its near side within WALLSKIN.off of it */
  private seat(k: number, s: Box, t: 0 | 2): Seat | null {
    const w = this.g.col.boxes[k], r = s.yaw - w.yaw; if (Math.abs(Math.sin(2 * r)) > 1e-4) return null;
    const n = (Math.abs(Math.sin(r)) > 0.5 ? 2 - t : t) as 0 | 2, u = (2 - n) as 0 | 2, a = (2 - t) as 0 | 2, hn = s.h[t], hu = s.h[a], H = w.h[n], l = into(w, s.c);
    const u0 = Math.max(l[u] - hu, -w.h[u]), u1 = Math.min(l[u] + hu, w.h[u]), y0 = Math.max(l[1] - s.h[1], -w.h[1]), y1 = Math.min(l[1] + s.h[1], w.h[1]);
    if (u1 - u0 <= EPS || y1 - y0 <= EPS) return null;
    let face: 1 | -1, near: number;
    if (l[n] + hn - H > EPS) { face = 1; near = l[n] - hn - H; } else if (-H - (l[n] - hn) > EPS) { face = -1; near = -H - (l[n] + hn); } else return null;
    if (near > WALLSKIN.off + EPS) return null;
    const lo: V3 = [0, y0, 0], hi: V3 = [0, y1, 0], o: V3 = [0, 0, 0]; lo[n] = hi[n] = face * H; lo[u] = u0; hi[u] = u1; o[n] = face;
    const p = into(s, outOf(w, lo)), q = into(s, outOf(w, hi)), out = into(s, outOf(w, o))[t] - into(s, w.c as V3)[t];   // (the face's outward normal along the sheet's thin axis: the solid is on the other side)
    return { k, n, face, t, side: out > 0 ? -1 : 1, near, r: [Math.min(p[a], q[a]), Math.max(p[a], q[a]), Math.min(p[1], q[1]), Math.max(p[1], q[1])] };
  }
  /** what sheet j lies on: every authored solid it has a seat on, on its BACK — the side more of it lies against (then the nearer, then the earlier box).
   *  A sheet thin both ways (an upright run of conduit) is asked across its thinner axis first */
  private backs(j: number): Seat[] {
    let L = this.seats.get(j); if (L) return L;
    const boxes = this.g.col.boxes, s = boxes[j], K = WALLSKIN, R = s.h[0] + s.h[2] + K.off; L = [];
    if (!this.own(j)) for (const t of (s.h[0] <= s.h[2] ? [0, 2] : [2, 0]) as (0 | 2)[]) {
      if (2 * s.h[t] > K.thick + EPS || s.h[t] > s.h[1] + EPS) continue;   // (deeper than a sheet; or thinner up than across: a strip lying flat at a wall's foot is the floor's)
      const got: Seat[] = [], sum = [0, 0], near = [1e9, 1e9], first = [1e9, 1e9];
      for (let k = 0; k < this.solid.length; k++) {
        if (!this.solid[k]) continue; const w = boxes[k], far = R + w.h[0] + w.h[2];
        if (Math.abs(s.c[0] - w.c[0]) > far || Math.abs(s.c[2] - w.c[2]) > far || Math.abs(s.c[1] - w.c[1]) > s.h[1] + w.h[1]) continue;
        const q = this.seat(k, s, t); if (!q) continue;
        const i = q.side > 0 ? 1 : 0; got.push(q); sum[i] += (q.r[1] - q.r[0]) * (q.r[3] - q.r[2]); near[i] = Math.min(near[i], q.near); first[i] = Math.min(first[i], k);
      }
      if (!got.length) continue;
      const side = Math.abs(sum[1] - sum[0]) > EPS ? (sum[1] > sum[0] ? 1 : -1) : Math.abs(near[1] - near[0]) > EPS ? (near[1] < near[0] ? 1 : -1) : first[1] < first[0] ? 1 : -1;
      L = got.filter(q => q.side === side); break;
    }
    this.seats.set(j, L);
    return L;
  }
  /** a breakable's sheets: of the sheets near it, those that lie on its own box */
  private skinsOf(B: BreakLike): number[] {
    let L = this.on.get(B.id); if (L) return L;
    const boxes = this.g.col.boxes, w = B.box, reach = w.h[0] + w.h[2] + WALLSKIN.off; L = [];
    for (const j of this.cand) {
      const s = boxes[j], far = reach + s.h[0] + s.h[2]; if (Math.abs(s.c[0] - w.c[0]) > far || Math.abs(s.c[2] - w.c[2]) > far) continue;
      if (this.backs(j).some(q => q.k === B.id)) L.push(j);
    }
    this.on.set(B.id, L);
    return L;
  }

  // ---------------------------------------------------------------- cutting one
  /** the open cells of breakable W behind a sheet lying on it at seat `q`, as rectangles on the sheet's face in its own frame (run axis `a`): the layer
   *  of cells at the face it lies on */
  private cells(s: Box, q: Seat, W: BreakLike, a: 0 | 2): Rect[] {
    const out: Rect[] = [], { n, face } = q, w = W.box, u = 2 - n, N = n === 0 ? W.nz : W.nx, at = face > 0 ? (n === 0 ? W.nx : W.nz) - 1 : 0;
    for (let k = 0; k < N; k++) for (let iy = 0; iy < W.ny; iy++) {
      const ix = n === 0 ? at : k, iz = n === 0 ? k : at; if (!W.open[(iy * W.nz + iz) * W.nx + ix]) continue;
      const lo: V3 = [0, W.yb[iy], 0], hi: V3 = [0, W.yb[iy + 1], 0];
      lo[n] = hi[n] = face * w.h[n]; lo[u] = -w.h[u] + 2 * w.h[u] * k / N; hi[u] = -w.h[u] + 2 * w.h[u] * (k + 1) / N;
      const p = into(s, outOf(w, lo)), e = into(s, outOf(w, hi)), r: Rect = [Math.min(p[a], e[a]), Math.max(p[a], e[a]), Math.min(p[1], e[1]), Math.max(p[1], e[1])];
      if (Math.min(r[1], s.h[a]) - Math.max(r[0], -s.h[a]) > EPS && Math.min(r[3], s.h[1]) - Math.max(r[2], -s.h[1]) > EPS) out.push(r);
    }
    return out;
  }
  private recut(j: number) {
    const boxes = this.g.col.boxes, s = boxes[j], seats = this.backs(j), K = WALLSKIN; if (!seats.length) return;
    const t = seats[0].t, a = (2 - t) as 0 | 2, hu = s.h[a], hv = s.h[1];
    // what has gone of each thing it lay on: a breakable's open cells at that face; any other solid whole, once something has hidden it
    const on = seats.map(q => q.r), near = seats.map(q => q.near), open = seats.map(q => { const W = this.brk(q.k); return W ? this.cells(s, q, W, a) : boxes[q.k].flags & BoxFlag.NoPrimary ? [ALL] : []; });
    const whole = (r: Rect[]) => r.length === 1 && r[0][0] <= -hu + EPS && r[0][1] >= hu - EPS && r[0][2] <= -hv + EPS && r[0][3] >= hv - EPS;
    // what stands behind may END RAGGED (a breakable's standing piece under a cut record, with the cut switch up): the part of the sheet on such a piece
    // takes the outline of that piece's face
    const parts: Rect[] = [], rag: { P: Box; q: Seat }[] = [];
    if (cutBoxesOn()) for (const q of seats) { const W = this.brk(q.k); if (!W) continue;
      for (let i = 0; i < W.live; i++) { const P = boxes[W.slots[i]]; if (!P || !P.cut || !P.cut.length) continue;
        const lo = into(s, outOf(P, [-P.h[0], -P.h[1], -P.h[2]])), hi = into(s, outOf(P, [P.h[0], P.h[1], P.h[2]])), r: Rect = [Math.min(lo[a], hi[a]), Math.max(lo[a], hi[a]), Math.min(lo[1], hi[1]), Math.max(lo[1], hi[1])];
        if (Math.min(r[1], hu) - Math.max(r[0], -hu) > EPS && Math.min(r[3], hv) - Math.max(r[2], -hv) > EPS) { parts.push(r); rag.push({ P, q }); } } }
    let rects = open.some(l => l.length) || parts.length ? standing(hu, hv, on, near, open, false, parts) : [[-hu, hu, -hv, hv] as Rect];
    const uncut = () => { this.cut.delete(j); this.laid.delete(j); };
    if (whole(rects) && !parts.length) { uncut(); return; }   // (something solid stands behind every part of it that lay on anything: the authored box, whole)
    const marked = !!(s.marks && s.marks.length), base = s.cut ?? [];
    if (marked && (!cutBoxesOn() || base.length + 4 > MAX_CUT_PLANES)) { if (!this.marked.has(j)) { this.marked.add(j); this.stats.uncut++; } return; }   // (its marks are laid out on the whole box and no record can be drawn, or has room: left as it was)
    let others = 0; for (const [i, l] of this.cut) if (i !== j && boxDrawn(boxes, boxes[i])) others += l.length;   // (what the other sheets hand over: one the hangers have since thrown spends none of the cap)
    const fits = (r: Rect[]) => r.length <= Math.min(K.perSkin, SLOTS) && others + r.length <= K.maxPieces;
    if (!fits(rects) && parts.length) { parts.length = 0; rag.length = 0; rects = standing(hu, hv, on, near, open); if (whole(rects)) { uncut(); return; } }   // (past a cap: square ends first)
    if (!fits(rects)) { rects = standing(hu, hv, on, near, open, true); if (!fits(rects)) { this.stats.capped++; return; } this.stats.coarse++; }   // (… then by whole columns, else as it last stood)
    /** the planes of the piece behind rectangle `r`, about a box of the sheet's frame centred at `pc`: each taken where it meets the face the sheet lies on,
     *  and stood square to that face (paint is on the face: where the broken core runs on behind it there is none) */
    const outline = (r: Rect, pc: V3): CutPlane[] => { const k = parts.findIndex(x => inside(x, (r[0] + r[1]) / 2, (r[2] + r[3]) / 2)); if (k < 0) return [];
      const { P, q } = rag[k], O = into(P, pc), tf = q.side * (s.h[t] + Math.max(q.near, 0)), zero = into(s, P.c as V3), out: CutPlane[] = [];
      for (const c of P.cut!) { const n = into(s, outOf(P, c.n as V3)); n[0] -= zero[0]; n[1] -= zero[1]; n[2] -= zero[2];
        const d = c.d - (c.n[0] * O[0] + c.n[1] * O[1] + c.n[2] * O[2]) - n[t] * tf; n[t] = 0; const L = Math.hypot(n[0], n[1], n[2]); if (L > 1e-6) out.push({ n: [n[0] / L, n[1] / L, n[2] / L], d: d / L }); }
      return out; };
    // a piece's key is its sheet and one of 16 places: a piece a later cut leaves alone keeps its place, and a new one takes a place no piece of the
    // last cut had while there is one — the picture's memory of where each box was is never handed an old key on another stretch of the band
    const prev = this.laid.get(j) ?? [], old = new Set(prev.map(p => p.slot)), now = new Set<number>();
    const lay = rects.map(r => { const same = prev.find(p => p.r.every((x, i) => Math.abs(x - r[i]) <= EPS)); if (same) now.add(same.slot); return { r, slot: same ? same.slot : -1 }; });
    for (const x of lay) { if (x.slot >= 0) continue; let k = 0; while (k < SLOTS && (now.has(k) || old.has(k))) k++; if (k === SLOTS) { k = 0; while (now.has(k)) k++; } x.slot = k; now.add(k); }
    const pieces: Box[] = [], kept: { r: Rect; slot: number }[] = []; let bit = false, area = 0; for (const r of rects) area += (r[1] - r[0]) * (r[3] - r[2]);
    for (const x of lay) { const { r, slot } = x;
      const lc: V3 = [0, (r[2] + r[3]) / 2, 0], h: V3 = [0, (r[3] - r[2]) / 2, 0]; lc[a] = (r[0] + r[1]) / 2; h[a] = (r[1] - r[0]) / 2; h[t] = s.h[t];
      const p = copyBox(s); p.flags = (s.flags & ~BoxFlag.NoPrimary) | BoxFlag.Dynamic; p.name = 'skin'; p.mkey = MKEY.skin + j * SLOTS + slot;
      if (marked) {   // the whole box again, under the planes of the rectangle that stands (a side that is the box's own takes none): every mark where it was
        const planes: CutPlane[] = base.slice(), side = (ax: 0 | 1 | 2, sign: 1 | -1, at: number) => { const n: V3 = [0, 0, 0]; n[ax] = sign; planes.push({ n, d: sign * at }); };
        if (r[0] > -hu + EPS) side(a, -1, r[0]); if (r[1] < hu - EPS) side(a, 1, r[1]); if (r[2] > -hv + EPS) side(1, -1, r[2]); if (r[3] < hv - EPS) side(1, 1, r[3]);
        p.c = [s.c[0], s.c[1], s.c[2]]; p.h = [s.h[0], s.h[1], s.h[2]]; p.cut = planes;
      } else {
        p.c = outOf(s, lc); p.h = h;
        p.cut = base.length ? base.map(c => ({ n: c.n, d: c.d - (c.n[0] * lc[0] + c.n[1] * lc[1] + c.n[2] * lc[2]) })) : undefined;   // (a shape's planes are in the box's own frame: the same planes about the piece's middle)
      }
      if (parts.length) { const more = outline(r, p.c as V3); if (more.length) { const own = cutRecord(p.h as V3, p.cut ?? []).cut?.length ?? 0, R = cutRecord(p.h as V3, [...(p.cut ?? []), ...more]); if (R.empty || (R.cut?.length ?? 0) > own) bit = true; if (R.empty) continue; p.cut = R.cut; } }
      p.was = undefined; p.shape = undefined; p.shaped = undefined; p.piece = undefined; p.of = undefined;   // (a plain box of the sheet's paint: what the level's stand-up keeps on the authored one is not its)
      pieces.push(p); kept.push(x);
    }
    if (!bit && area >= 4 * hu * hv - EPS) { uncut(); return; }   // (split at a ragged piece's bounds, and no plane of it bites the sheet: nothing has gone, the authored box, whole)
    this.laid.set(j, kept); this.cut.set(j, pieces);
  }
}
