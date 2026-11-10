// THE BOX WORLD OF A PAGE THAT HOLDS NO GAME: the renderer's tables, filled with the rows Rust left (rust/game_draw boxes). Nothing is packed here and no box
// is an object, but for what the picture's own plans ask of a static (where it stands and what kind of thing it is: scene/boxes.ts PlanBox), which are told
// where one changed, and of a door's leaf.
import { BoxFlag, BoxWorld, GB_ROWS, MAT_STRIDE, boxOf, type Box, type DrawWait } from '../scene/boxes';
import { MARK_TABLE_ROWS } from '../scene/markings';
import { AIR_ROWS } from '../scene/airGrid';
import { lookNeedBits } from '../render/gbuffer';
import type { Vec3 } from '../math/vec';
import type { CutPlane } from '../scene/cut';
import type { Drawn } from './drawn';
import { Beam, Cut, Geo, Head, Leaf, Mat, Row4, Section, Word } from './sections';

/** the GPU's flag word (rust/game_draw boxes/pack.rs): what of it a plan asks */
const KEPT = 0xe7 | BoxFlag.Glass, ROUND = 8, ROT = 64, CUT = 1 << 16;
/** m round a box that changed */
const STALE = 0.6;
const A_CUT: CutPlane[] = [{ n: [0, 1, 0], d: 0 }];

export class DrawWorld extends BoxWorld {
  /** where the statics that a wall could be have changed since this was last read: rects on the plan */
  readonly walls: [number, number, number, number][] = [];
  /** …and where any static came, went or was cut, whatever its height: what the smoke's solver has baked round there is stale */
  readonly stale: { c: Vec3; r: number }[] = [];
  private boxes = 0; private stuffs = 0;
  private leaf: Box[] = [];
  /** the lines of light nearest the view, and the dot one ends on (engine.ts `beams`) */
  readonly beams: { list: Box[]; is(b: Box): boolean; airOf(b: Box): Box | null | undefined } = { list: [], is: () => true, airOf: b => { const a = b.owner; return a === 0 ? undefined : a === 1 ? null : this.dot; } };
  private dot = boxOf([0, 0, 0], [0, 0, 0], 0, [0, 0, 0], [0, 0, 0], 0, 0); private lines: Box[] = [];
  constructor(device: GPUDevice, private D: Drawn) { super(device); }
  get count(): number { return this.boxes; }

  upload(_between = false, _wait: DrawWait | null = null): void {
    const D = this.D, u = D.u, h = D.at[Section.head], q = this.device.queue, n = u[h + Head.count], st = u[h + Head.statics], from = u[h + Head.from], to = u[h + Head.to];
    this.boxes = n;
    // the statics that changed, and the frame's own
    D.send(q, this.geoBuf, Section.geo, Geo.row, from, to - from); D.send(q, this.matBuf, Section.mat, Mat.row, from, to - from);
    D.send(q, this.geoBuf, Section.geo, Geo.row, st, n - st); D.send(q, this.matBuf, Section.mat, Mat.row, st, n - st);
    // the tables behind the materials: marks, the panes' cells, the cut planes
    let base = this.maxBoxes * MAT_STRIDE * 4;
    if (u[h + Head.marksMoved]) D.send(q, this.matBuf, Section.marks, Row4.row, 0, D.n[Section.marks], base);
    base += MARK_TABLE_ROWS * 16;
    if (u[h + Head.panesMoved]) D.send(q, this.matBuf, Section.panes, Row4.row, 0, D.n[Section.panes], base);
    base += GB_ROWS * 16 + AIR_ROWS * 16;
    if (this.cutOn) for (let k = 0; k < 2; k++) { const a = u[h + (k ? Head.cutFromOwn : Head.cutFrom)], b = u[h + (k ? Head.cutToOwn : Head.cutTo)]; if (b > a) D.send(q, this.matBuf, Section.cut, Cut.row, a, b - a, base + a * Cut.row * 4); }
    D.send(q, this.visibleBuf, Section.visible, Word.row);
    this.visibleCount = u[h + Head.visible]; this.visibleCutCount = u[h + Head.visibleCut]; this.visibleCutTraced = u[h + Head.visibleCutTraced]; this.cutTraced = u[h + Head.cutTraced];
    if (u[h + Head.gridMoved]) {
      const bytes = D.n[Section.items] * 4;
      if (bytes > this.itemBuf.size) { this.itemBuf.destroy(); this.itemBuf = this.device.createBuffer({ label: 'gridItems', size: bytes * 2, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST }); }
      D.send(q, this.cellBuf, Section.cells, Word.row); D.send(q, this.itemBuf, Section.items, Word.row);
    }
    // the info block is the module's but for the men's proxies, which are fitted here
    { const words = this.infoData, at = D.at[Section.info], keep = BoxWorld.INFO_BOUNDS; for (let i = 0, m = D.n[Section.info]; i < m; i++) if (i < keep || i >= keep + BoxWorld.MAX_CAST * 4) words[i] = u[at + i];
      words[20] = this.capsuleCount; words[21] = this.charCount;
      if (this.rig) this.applyRig(this.rig);
      q.writeBuffer(this.capsuleBuf, 0, this.capsuleData.buffer, 0, Math.max(32, this.capsuleCount * 32)); q.writeBuffer(this.infoBuf, 0, words); }
    this.gridStats.slack = D.f[h + Head.slack];
    if (u[h + Head.stuffs] !== this.stuffs) { this.stuffs = u[h + Head.stuffs]; lookNeedBits(this.stuffs); }
    if (to > from || this.statics.length !== st) this.mirror(from, to, st);
    this.geomSerial = u[h + Head.serial];
    { const L = this.glassList, n = this.infoData[BoxWorld.GLASS_WORD]; L.length = n; for (let k = 0; k < n; k++) L[k] = this.infoF32[BoxWorld.GLASS_WORD + 4 + k * 12 + 11]; }
  }

  /** the statics a plan reads: those from `from` to `to` as they stand now */
  private mirror(from: number, to: number, count: number): void {
    const D = this.D, f = D.f, u = D.u, g0 = D.at[Section.geo], m0 = D.at[Section.mat];
    for (let i = count; i < this.statics.length; i++) this.wall(this.statics[i]);
    this.statics.length = Math.min(this.statics.length, count); this.glassStatics.length = 0;
    for (let i = from; i < to; i++) {
      const g = g0 + i * Geo.row, m = m0 + i * Mat.row, w = u[g + 7], was = this.statics[i], b = this.statics[i] ?? (this.statics[i] = boxOf([0, 0, 0], [0, 0, 0], 0, [0, 0, 0], [0, 0, 0], 0, 0));
      const stood = was !== undefined && b.c[0] === f[g] && b.c[1] === f[g + 1] && b.c[2] === f[g + 2] && b.h[0] === f[g + 4] && b.h[1] === f[g + 5] && b.h[2] === f[g + 6] && b.flags === (w & KEPT) && (b.cut !== undefined) === ((w & CUT) !== 0);
      if (was && !stood) this.wall(b);
      for (let k = 0; k < 3; k++) { b.c[k] = f[g + k]; b.h[k] = f[g + 4 + k]; b.albedo[k] = f[m + k]; b.emissive[k] = f[m + 4 + k]; }
      b.flags = w & KEPT; b.round = (w & ROUND) !== 0 ? true : undefined; b.cut = (w & CUT) !== 0 ? A_CUT : undefined; b.cutHeight = f[m + 3];
      if (w & ROT) { b.rot ??= [0, 0, 0, 1]; for (let k = 0; k < 4; k++) b.rot[k] = f[m + 8 + k]; } else b.rot = undefined;
      b.yaw = 2 * Math.atan2(f[m + 9], f[m + 11]);
      if (!stood && this.told) this.wall(b);
    }
    for (let i = 0; i < this.statics.length; i++) if (this.statics[i].flags & BoxFlag.Glass) this.glassStatics.push(i);
    this.told = true;
  }
  /** before a frame is drawn, the rows standing where it has them: what of them the renderer asks for by object */
  from(): void {
    const D = this.D, f = D.f, L = this.beams.list, n = D.n[Section.beams]; L.length = n;
    for (let i = 0; i < n; i++) { const o = D.at[Section.beams] + i * Beam.row, b = this.lines[i] ?? (this.lines[i] = boxOf([0, 0, 0], [0, 0, 0], 0, [0, 0, 0], [0, 0, 0], 0, 0, [0, 0, 0, 1])); L[i] = b;
      for (let k = 0; k < 3; k++) { b.c[k] = f[o + Beam.c + k]; b.h[k] = f[o + Beam.h + k]; b.emissive[k] = f[o + Beam.emits + k]; }
      for (let k = 0; k < 4; k++) b.rot![k] = f[o + Beam.rot + k];
      b.owner = D.u[o + Beam.air]; }
    this.leaves();
  }
  private told = false;
  private wall(b: Box): void { this.stale.push({ c: [b.c[0], b.c[1], b.c[2]], r: Math.hypot(b.h[0], b.h[1], b.h[2]) + STALE }); const [ex, ez] = BoxWorld.footprint(b); if (b.h[1] > 0.25) this.walls.push([b.c[0] - ex, b.c[2] - ez, b.c[0] + ex, b.c[2] + ez]); }
  private leaves(): void {
    const D = this.D, f = D.f, u = D.u, n = D.n[Section.leaves], list = this.dynamics; list.length = n;
    for (let i = 0; i < n; i++) { const o = D.at[Section.leaves] + i * Leaf.row, b = this.leaf[i] ?? (this.leaf[i] = boxOf([0, 0, 0], [0, 0, 0], 0, [0.5, 0.5, 0.5], [0, 0, 0], 0, 0)); list[i] = b;
      for (let k = 0; k < 3; k++) { b.c[k] = f[o + Leaf.c + k]; b.h[k] = f[o + Leaf.h + k]; }
      b.yaw = f[o + Leaf.yaw]; b.flags = u[o + Leaf.flags]; b.mkey = u[o + Leaf.key]; }
  }
  relist(): void { /* nothing is listed here: the module lists its own */ }
  ensurePacked(): void { /* …and what is on the device is the last frame's */ }
}
