// WHAT STANDS SHOWS THE DAMAGE, BY ITS MATERIAL (THINGS.deform; the maintainer, 10-31: "one thing that control did well was having different deformations per
// material, like wood splintering, metal bending"). The break table (breakTable.ts) already shapes every piece a thing is BROKEN into by what it is made of.
// This is the other half: a thing a burst reaches and does NOT break is marked where it stands, in its own stuff —
//     steel            DENTED: creases along the edges of the face that looked at the burst, each running out toward one end (it bends, it does not break)
//     stone, concrete  CHIPPED: the top corner nearest the burst knocked off (and now and then the next one), with its grit and dust
//     wood, board      SPLIT: the near edge torn with the grain, a pale face that tapers out along the piece
//     soft stuff       TORN: one shallow tear along the near top edge
// — as a CUT RECORD on the authored box itself (scene/cut.ts): the picture, every ray, round and sight line meet the marked solid; feet, the walk grid and
// whatever leans on it meet the box as it was (the cut contract), so a mark never bites deeper than its row's `most` of the side it is on, at either end of its slant. No hit
// points: a mark is a picture of one burst and counts toward nothing; a box carries at most `perBox` of them and the next burst leaves it as it is.
// A CUT TAKES AN EDGE OR A CORNER OFF, NEVER A BITE OUT OF THE MIDDLE (the solid stays convex), so a mark lies within `span` of the burst or is not made: a
// short thing is creased along its near face; a long one (a container, a long block) only at the end the burst is by, and not at all when hit mid-length.
//
// Asked of every thing with a body in the burst's SHATTER ring (× `reach`) that the burst has a line to and that still stands when the furniture has taken its
// own: furniture that held (a heavy core, a piece the budget ran out on) and the things no engine breaks (a barrier block, a T-wall, a container, a post).
// Never a wall or a cover panel (the wall engine dresses its own), glass, snow, a light, a drawn-only sheet, a tilted box or one authored with a cut.
//
// STATE: what was marked is three whole numbers a mark (where the burst was, in centimetres, and how near), so a checkpoint files it in the furniture's slice
// (`FurnSave.d`) and a load cuts the same planes again; a level reset takes every mark off. Deterministic: every plane comes off a hash of the box's index and
// the mark's own numbers, never off a stream, so marking a thing moves no other piece's dice. Nothing at all without cut boxes (cutBoxesOn) or the one index.
// Nothing at module level uses another game module (the import cycle).
import { Vec3, v3 } from '../math/vec';
import { Box, BoxFlag } from '../scene/boxes';
import { CutPlane, MAX_CUT_PLANES, cutBoxesOn, cutPlane, cutRecord, cutThrough } from '../scene/cut';
import type { Game } from './game';
import type { Destruction } from './destruction';
import { THINGS, type Thing } from './things';
import { BREAK, faceCore, type Mat } from './breakTable';
import { hash01 } from './debris';
import { gpuFx } from './particleFx';

/** one mark: where the burst was on the plan (cm) and how near it was (0‥100: 100 = lying against the thing) */
export type ScarRow = [i: number, xcm: number, zcm: number, near: number];
export type ScarKind = 'dent' | 'chip' | 'split' | 'tear';
/** the structure a burst marks (the index's own words): what stands for a reason of size or place — never a finish, lying snow, a thing over a man's head */
const MARKED = new Set(['the building', 'full height', 'a post', 'too heavy to move', 'carries what stays']);
const lerp = (r: readonly [number, number], t: number) => r[0] + (r[1] - r[0]) * t;

export class Scars {
  /** every marked static: its marks in the order they came */
  private marks = new Map<number, [number, number, number][]>();
  /** the boxes that carry a cut of OURS (a marked thing the wall engine has since taken on, or a load with cut boxes off, has its rows in `marks` and no cut) */
  private was = new Set<number>();
  constructor(private g: Game, private destr: Destruction) {}

  /** how this stuff is marked; null = it is not (snow, glass) */
  kindOf(t: Thing): ScarKind | null { return t.mat === 'metal' ? 'dent' : t.mat === 'stone' || t.mat === 'hard' ? 'chip' : t.mat === 'wood' || t.mat === 'board' ? 'split' : t.mat === 'soft' ? 'tear' : null; }
  /** the statics that carry a mark's cut (ascending) with their kinds, for the lab and the checks */
  list(): { i: number; kind: ScarKind; marks: number; planes: number }[] {
    const T = this.destr.things.all(); return [...this.was].sort((p, q) => p - q).map(i => ({ i, kind: this.kindOf(T[i])!, marks: this.marks.get(i)!.length, planes: this.g.col.boxes[i].cut?.length ?? 0 }));
  }
  /** may static `i` take a mark at all (by what it was authored as: asked of a burst and of a checkpoint alike) */
  private markable(t: Thing | undefined): boolean {
    if (!t || t.ghost || !this.kindOf(t) || !(t.via === 'furniture' || (t.via === 'structure' && MARKED.has(t.why)))) return false;
    const b = this.g.col.boxes[t.i]; if (!b || b.rot || (b.cut && !this.was.has(t.i)) || this.destr.walls.breakableAt(t.i)) return false;
    const hs = [b.h[0], b.h[1], b.h[2]].sort((p, q) => q - p), K = THINGS.deform;   // (a face big enough to show it: a 4 cm pole takes no mark)
    if (b.h[1] * 2 < K.min || t.a.y1 < K.stands) return false;   // (it STANDS: a floor slab, a carpet, a kerb is 'the building' too and is not marked)
    return hs[0] * 2 >= K.min && hs[1] * 2 >= K.minFace && b.emissive[0] + b.emissive[1] + b.emissive[2] <= 0;
  }

  /** A burst at `at` whose shatter ring is `shatterM`: mark what it reached and left standing. `origins` = where it blows from (the furniture engine's: the charge
   *  and the far mouth of a hole it opened). Returns how many things took a mark. */
  blast(at: Vec3, shatterM: number, who: number, origins: Vec3[]): number {
    const K = THINGS.deform, g = this.g; if (!THINGS.on || !K.on || !cutBoxesOn()) return 0;
    const R = shatterM * K.reach; if (!(R > 0)) return 0;
    const near: { t: Thing; d: number }[] = [];
    for (const t of this.destr.things.all()) {
      const a = t.a, d = Math.hypot(Math.max(a.x0 - at[0], 0, at[0] - a.x1), Math.max(a.z0 - at[2], 0, at[2] - a.z1)); if (d > R || a.y0 > at[1] + 1.5 || !this.markable(t)) continue;
      const b = g.col.boxes[t.i]; if (b.flags & BoxFlag.NoPrimary || (this.marks.get(t.i)?.length ?? 0) >= K.perBox) continue;
      near.push({ t, d });
    }
    near.sort((p, q) => p.d - q.d || p.t.i - q.t.i); let n = 0;
    for (const { t, d } of near) {
      if (n >= K.most) break; if (!this.exposed(origins, t)) continue;
      const row: [number, number, number] = [Math.round(at[0] * 100), Math.round(at[2] * 100), Math.max(1, Math.round(100 * (1 - d / R)))];
      const list = this.marks.get(t.i) ?? []; list.push(row);
      if (!this.apply(t, list)) { list.pop(); continue; }
      this.marks.set(t.i, list); this.was.add(t.i); n++;
      this.puff(t, row, who);
    }
    if (n) this.bump();
    return n;
  }
  /** a line from one of the burst's origins to the nearest point of the thing, met by nothing but the thing itself */
  private exposed(origins: Vec3[], t: Thing): boolean {
    const g = this.g, a = t.a;
    for (const o of origins) {
      const p: Vec3 = [Math.max(a.x0, Math.min(o[0], a.x1)), Math.max(a.y0 + 0.05, Math.min(o[1], a.y1 - 0.02)), Math.max(a.z0, Math.min(o[2], a.z1))];
      const d = v3.sub(p, o), L = v3.len(d); if (L < 0.05) return true;
      const hit = g.col.raycast(o, v3.scale(d, 1 / L), L); if (!hit || hit.index === t.i || hit.t >= L - 0.03) return true;
    }
    return false;
  }

  /** the planes one mark cuts in box `b`, in its own frame: by its kind, off a hash of (`i`, the mark) */
  private planes(i: number, b: Box, kind: ScarKind, m: readonly [number, number, number], k: number): CutPlane[] {
    const K = THINGS.deform, h = b.h, cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), dx = m[0] / 100 - b.c[0], dz = m[1] / 100 - b.c[2];
    const l: Vec3 = [cs * dx - sn * dz, 0, sn * dx + cs * dz];   // the burst in the box's frame (boxes.ts: world = c + [cs·x + sn·z, y, −sn·x + cs·z])
    const u: 0 | 2 = Math.abs(l[0]) - h[0] >= Math.abs(l[2]) - h[2] ? 0 : 2, v: 0 | 2 = u === 0 ? 2 : 0;   // the face that looked at it, and the way along that face
    const r = (j: number) => hash01(Math.imul(i + 1, 7919) + Math.imul(m[0], 31) + Math.imul(m[1], 17) + k * 131 + j * 61 + 5);
    const su = l[u] >= 0 ? 1 : -1, sv = Math.abs(l[v]) > 0.05 ? (l[v] >= 0 ? 1 : -1) : r(1) < 0.5 ? -1 : 1, pv = Math.max(-h[v], Math.min(h[v], l[v]));
    // (how deep: by how near the burst was, and by how big the thing is — a container's dent is not a locker's; `most` of the side it is on still bounds it)
    const nearK = (0.5 + 0.5 * m[2] / 100) * Math.max(1, Math.min(K.big, 2 * Math.min(h[0], h[1], h[2]) / K.bigAt)), out: CutPlane[] = [];
    // WHERE: `whole` = the far end of the near face is within `span` of the burst (a crease may run the face); else only the end the burst is by, if that is
    // within `span`; else nothing (a long thing hit mid-length: no edge or corner of it is near the burst)
    const whole = h[v] + Math.abs(pv) <= K.span, end = h[v] - Math.abs(pv) <= K.span; if (!whole && !end) return out;
    // an EDGE of the near face taken off on a slant: the edge along `e` between the near face and the face `w` (sign sw), half of `deep` at the point `at` along
    // it, growing toward one end and running out toward the other; at its deep end it takes no more than `share` of the side it bites (u) or the side it runs
    // down (w). A crease too fine to see is not cut (a cut box costs every ray a test).
    const edge = (e: number, w: number, sw: number, deep: number, lean: number, run: number, share: number, at = 0) => {
      const f = 0.5 + Math.abs(run) * (1 + Math.abs(at) / Math.max(h[e], 0.05)); deep = Math.min(deep, share * 2 * Math.min(h[u], h[w] * lean) / f); if (!(deep >= K.fine)) return;
      const n: Vec3 = [0, 0, 0], p: Vec3 = [0, 0, 0]; n[u] = su; n[w] = sw * lean; n[e] = run * deep / Math.max(h[e], 0.05);
      p[u] = su * (h[u] - deep * 0.5); p[w] = sw * h[w]; p[e] = at; out.push(cutThrough(p, n));
    };
    // a top CORNER of the near face knocked off or pushed in: the plane through three points, each its own bite in from the corner along one edge
    const corner = (s: number, range: readonly [number, number], share: number, scale: number, j: number) => {
      const bite = (a: number, k: number, jj: number) => Math.min(lerp(range, r(jj)) * nearK * scale * k, share * 2 * h[a]);
      const bu = bite(u, 1, j), by = bite(1, 1, j + 1), bv = bite(v, 1.4, j + 2); if (!(Math.min(bu, by, bv) >= K.fine)) return;
      const n: Vec3 = [0, 0, 0]; n[u] = su / bu; n[1] = 1 / by; n[v] = s / bv; const c: Vec3 = [0, h[1], 0]; c[u] = su * h[u]; c[v] = s * h[v];
      out.push(cutPlane(n, n[0] * c[0] + n[1] * c[1] + n[2] * c[2] - 1));
    };
    const flip = (j: number) => (r(j) < 0.5 ? -1 : 1);
    if (kind === 'dent') {   // creases: the upright edge at the near end, and on a short thing the top edge and the other upright; a long one has its near top corner pushed in
      const deep = (j: number) => lerp(K.dent, r(j)) * nearK;
      edge(1, v, sv, deep(20), 0.6 + r(21), flip(22) * (0.7 + 0.5 * r(23)), K.dentMost);
      if (whole) { edge(v, 1, 1, deep(10), 0.6 + r(11), flip(12) * (0.7 + 0.5 * r(13)), K.dentMost, pv); if (m[2] > 55) edge(1, v, -sv, deep(30) * 0.7, 0.6 + r(31), flip(32) * (0.7 + 0.5 * r(33)), K.dentMost); }
      else corner(sv, K.dent, K.dentMost, 1.6, 40);
    } else if (kind === 'chip') {   // the top corner by the burst knocked off, and on a short thing now and then the next one
      corner(sv, K.chip, K.chipMost, 1, 10); if (whole && r(40) < K.chipTwo * (m[2] / 100 + 0.3)) corner(-sv, K.chip, K.chipMost, 0.6, 20);
    } else if (kind === 'split') {   // torn with the grain: the near upright edge opening toward the top, and on a short thing the near top edge off on a long slant
      const bite = (j: number) => lerp(K.split, r(j)) * nearK;
      edge(1, v, sv, bite(20), 0.5 + 0.9 * r(21), 0.9 + 0.9 * r(22), K.splitMost, h[1] * 0.4);
      if (whole) edge(v, 1, 1, bite(10), 0.5 + 0.9 * r(11), flip(12) * (1.2 + 1.2 * r(13)), K.splitMost, pv);
    } else {   // a tear: one shallow slant along the near top edge of a short thing; the near top corner of a long one
      if (whole) edge(v, 1, 1, lerp(K.tear, r(10)) * nearK, 0.7 + 0.6 * r(11), flip(12) * (0.5 + 0.8 * r(13)), K.tearMost, pv);
      else corner(sv, K.tear, K.tearMost, 1.5, 40);
    }
    return out;
  }
  /** cut `t`'s box by every mark in `list`. false = nothing was cut (a mark that would leave nothing, a box too small for it): the box is as it was */
  private apply(t: Thing, list: readonly (readonly [number, number, number])[]): boolean {
    const b = this.g.col.boxes[t.i], kind = this.kindOf(t); if (!kind) return false;
    const all: CutPlane[] = []; list.forEach((m, k) => all.push(...this.planes(t.i, b, kind, m, k)));
    if (!all.length) return false; const rec = cutRecord(b.h, all.slice(0, MAX_CUT_PLANES)); if (rec.empty || !rec.cut || !rec.cut.length) return false;
    b.cut = rec.cut; b.cutCore = this.core(t, b);
    return true;
  }
  /** the colour a marked face shows: the stuff inside (pale wood, grey stone, a soft thing's stuffing); steel shows its own paint */
  private core(t: Thing, b: Box): Vec3 | undefined {
    const m: Mat | null = t.mat === 'wood' ? 'wood' : t.mat === 'board' ? 'cardboard' : t.mat === 'stone' || t.mat === 'hard' ? 'blockwork' : t.mat === 'soft' ? 'fabric' : null;
    return m ? faceCore(BREAK[m], b.albedo) : undefined;
  }
  /** the mark's own debris, where the burst met the thing: splinters, grit and its dust, a few flecks off steel */
  private puff(t: Thing, m: readonly [number, number, number], who: number) {
    const a = t.a, b = this.g.col.boxes[t.i], kind = this.kindOf(t), K = THINGS.deform; const x = m[0] / 100, z = m[1] / 100;
    const p: Vec3 = [Math.max(a.x0, Math.min(x, a.x1)), a.y1 - 0.03, Math.max(a.z0, Math.min(z, a.z1))], dx = x - p[0], dz = z - p[2], L = Math.hypot(dx, dz) || 1, dir: Vec3 = v3.normalize([dx / L, 0.9, dz / L]);
    const P = gpuFx(this.g), n = Math.round((kind === 'dent' ? K.chips.dent : kind === 'chip' ? K.chips.chip : kind === 'split' ? K.chips.split : K.chips.tear) * (0.4 + 0.6 * m[2] / 100));
    if (P && n > 0) P.emit('debris', p, dir, n, { speed: 2.2, speedJit: 0.7, spread: 1.0, life: 18, lifeJit: 0.3, size: kind === 'split' ? 0.006 : 0.011, sizeJit: 0.6, aspect: kind === 'split' ? 4 : 1, color: this.core(t, b) ?? v3.copy(b.albedo), owner: who });
    if (kind === 'chip') this.destr.cellDust(p, dir, K.dust * (0.4 + 0.6 * m[2] / 100), 'concrete');
  }
  private bump() { const w = this.g.engine.world as { geomSerial?: number }; w.geomSerial = (w.geomSerial ?? 0) + 1; }

  /** every mark off (a level reset; the debugger's switch). true = something was marked */
  reset(): boolean {
    if (!this.marks.size) return false; const any = this.was.size > 0;
    for (const i of this.was) { const b = this.g.col.boxes[i]; if (b) { b.cut = undefined; b.cutCore = undefined; } }
    this.was.clear(); this.marks.clear(); if (any) this.bump(); return true;
  }
  /** the marks, for a checkpoint (the furniture's slice files them as `d`): undefined = none */
  snapshot(): ScarRow[] | undefined {
    const out: ScarRow[] = []; for (const i of [...this.marks.keys()].sort((p, q) => p - q)) for (const m of this.marks.get(i)!) out.push([i, m[0], m[1], m[2]]);
    return out.length ? out : undefined;
  }
  /** can these rows be applied to this level? null = yes */
  check(d: unknown): string | null {
    if (d === undefined) return null; if (!Array.isArray(d)) return 'marks: unknown slice';
    // (the rows' shape and that each names a box of this level, no more: whether a box takes its mark TODAY is restore's to ask — a marked wall the wall engine
    // has since taken on, a knob moved since the save, must not refuse the whole checkpoint)
    const N = this.destr.things.all().length;
    for (const r of d) if (!Array.isArray(r) || r.length !== 4 || !r.every(Number.isInteger) || r[0] < 0 || r[0] >= N || r[3] < 1 || r[3] > 100) return 'marks: unknown row';
    return null;
  }
  /** a checkpoint's marks cut again on the level put back, silently (after the furniture's own slice: a thing that has since gone keeps its mark unseen) */
  restore(d: ScarRow[] | undefined) {
    if (!d) return; const T = this.destr.things.all(), on = cutBoxesOn(); let any = false;
    for (const r of d) {   // (every row is KEPT, cut or not, so the next save files what this one did: a load with cut boxes off, a thing that takes no mark today)
      const t = T[r[0]]; if (!t) continue; const list = this.marks.get(r[0]) ?? []; if (list.length >= THINGS.deform.perBox) continue;
      list.push([r[1], r[2], r[3]]); this.marks.set(r[0], list);
      if (on && this.markable(t) && this.apply(t, list)) { this.was.add(r[0]); any = true; }
    }
    if (any) this.bump();
  }
}
