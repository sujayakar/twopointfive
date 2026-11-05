// THE TWIN OF THE GLASS RULE (rust/sim/src/panes.rs is the rule): the TypeScript that broke glass until it moved to the module, kept for two things and for the length
// of one train: a page with no WebAssembly still plays it, until the switches step, and tools/qa/pane-ident.ts runs every call both ways against it. It hangs nothing
// and is asked only by GlassBreak (glassbreak.ts). Deleted in the next commit of the train.
import { Vec3, v3 } from '../math/vec';
import { Box, BoxFlag, makeBox } from '../scene/boxes';
import { VIEW_BLOWN, VIEW_GIVES, type PaneView } from '../scene/boxMirror';
import type { Game, Player } from './game';
import { men } from './seats';
import { boxSeedOwn as boxSeed, pointSeedOwn as pointSeed, blastSeedOwn as blastSeed, glassImpact, glassEvent, glassPanes, isSheet } from './glass';
import { hash01 } from './debris';
import { gpuFx } from './particleFx';
import { WIND, WIND_FIELD, type WindSource } from './wind';
import { segSegClosest } from './devices';
import { GP_MAX_SITES, GP_FRAGS_PER_SITE, GP_MAX_SECTORS, GP_BANDS, GP_REMAINDER, type GpSite, type GpFrag, type Pt,
  gpClassify, gpNearest, gpFragments, gpChordPt, gpHull, gpClipRect, gpPolyArea, gpUnfid, gpDiceClump, gpDiceKey, gpDicePoly, gpDiceCentre, GP_RHO_LO, GP_RHO_SPAN } from './glassPattern';
import { GLASSBREAK, GLASSREAL, GB_TWIN, GB_HEADER_ROWS, GB_SITE_ROWS, glassBreakLive, type GlassType, type GlassImpact, type GlassFragment, type GlassBlastOpts, type PaneState, type Dice, type Clump, type Pending } from './glassbreak';

type GbTune = { glassBreak?: boolean };
/** how high the open-sky test of exteriorOf looks (m) */
const EXT_SKY_TOP = 12;
/** blast(): a pane the blast reaches with no more strength than this is not touched at all */
const BLAST_MIN_K = 0.05;
/** metres round a pane's nearest point to a blast of strength k within which its pieces are taken (blast(), and wouldGive's reckoning of it) */
const blastReach = (k: number) => 1.4 * k + 0.15;
/** dice clumps whose centres are this close in distance from the blow count as the same distance (they let go together) */
const DICE_TIE = 1e-4;

export class GlassBreakTwin {
  panes = new Map<number, PaneState>();
  log: string[] = [];
  /** THE RELEASE HOOK: set by the shards part; null = the stand-in below spawns the base's Debris shards + chips */
  onRelease: ((f: GlassFragment) => void) | null = null;
  private types = new Map<number, GlassType>();
  private ext = new Map<number, number>();
  private blasted: { t: number; at: Vec3; k: Map<number, number> }[] = [];
  private releasedThisFrame = 0;
  /** BOTH WAYS (tools/qa/pane-ident.ts): nothing leaves this rule. What it lets go is listed here, each with whether it was heard, and no wind source is hung. */
  shadow: (GlassFragment & { key: number; heard: boolean })[] | null = null;
  constructor(private g: Game) {}
  /** the rule's own answer at a point: a piece out, or the view in hand counts the glass there out */
  ruleOpen(bi: number, p: Vec3): boolean { return this.openAt(bi, p) || (this.also !== null && this.also(bi, p)); }
  private note(s: string) { this.log.push(`${this.g.time.toFixed(1)}s ${s}`); if (this.log.length > 60) this.log.shift(); }
  private plog(st: PaneState, s: string) { st.log.push(s); if (st.log.length > 16) st.log.shift(); }

  // ---------------------------------------------------------------- pane geometry
  /** which face of pane bi has the open sky: +1 = the side its normal (normalOf) points to, −1 = the other, 0 = neither or both = not an outside wall. Asked of the
   *  level's own geometry, so every level answers without a table: upward solid rays from the pane's mid height, 0.5‥1.6 m off each face in the pane's own frame (a
   *  canopy or a lamp head over part of one side does not flip the majority; a start inside furniture is passed over by the ray's own rule and still meets the ceiling).
   *  Cached per pane — statics do not move. (Until 10-08 this was floor.ts's office rectangle: right for the office, an accident of coordinates on records / barracks.) */
  private skySide(bi: number): number {
    let e = this.ext.get(bi); if (e !== undefined) return e;
    const b = this.g.level.boxes[bi]; if (!b) return 0;
    const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); const wide = b.h[0] >= b.h[2];
    const n: Vec3 = wide ? [sn, 0, cs] : [cs, 0, -sn], a: Vec3 = wide ? [cs, 0, -sn] : [sn, 0, cs]; const hu = Math.max(b.h[0], b.h[2]); const sky = [0, 0];
    for (let k = 0; k < 2; k++) for (const off of [0.5, 1.0, 1.6]) for (const t of [-0.5, 0, 0.5]) {
      const d = (k ? 1 : -1) * off; const p: Vec3 = [b.c[0] + n[0] * d + a[0] * t * hu, b.c[1], b.c[2] + n[2] * d + a[2] * t * hu];
      if (!this.g.col.solidBlocked(p, [p[0], EXT_SKY_TOP, p[2]], 0.02)) sky[k]++;
    }
    e = (sky[0] >= 5) === (sky[1] >= 5) ? 0 : sky[1] >= 5 ? 1 : -1;
    this.ext.set(bi, e); return e;
  }
  private exteriorOf(bi: number): boolean { return this.skySide(bi) !== 0; }
  /** the type a pane breaks as when nobody has said (setType): outside walls are safety glass — tempered, a low clerestory strip laminated; inside, a narrow sidelight
   *  is laminated and a partition plain float (annealed). The same rule on every level; which pane is which glass wants authoring (fracture.md, a call left open).
   *  Under GLASSREAL.place the pane's own place decides between toughened and float (the table's note); the laminated two are the same either way. */
  typeOf(b: Box, bi = this.g.level.boxes.indexOf(b)): GlassType {
    const outside = bi >= 0 && this.exteriorOf(bi);
    if (outside ? b.h[1] < 0.4 : Math.max(b.h[0], b.h[2]) < 0.35) return 'laminated';   // clerestory strips hold, and a narrow sidelight: under either rule
    if (!GLASSREAL.place) return outside ? 'tempered' : 'annealed';   // the first rule, by the wall: the frontage and the windows are safety glass, an interior partition plain float
    const foot = b.c[1] - b.h[1];   // GLASSREAL.place, by where the pane sits
    return foot < GLASSREAL.lowM || (foot < GLASSREAL.doorFootM && this.doorGap(b) <= GLASSREAL.doorM) ? 'tempered' : 'annealed';
  }
  /** metres on the floor plan from pane b to the nearest door's opening (the closed leaf's line, hinge to latch) — Infinity on a level with no door. Worked out each time
   *  it is asked, kept nowhere: it is asked only of a pane that starts above lowM and under doorFootM, a handful of sill windows against the level's doors, and a
   *  table of answers was game state a restarted level did not share with a fresh one (cleanrestart). */
  private doorGap(b: Box): number {
    const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), wide = b.h[0] >= b.h[2], hu = Math.max(b.h[0], b.h[2]); const ax = wide ? cs : sn, az = wide ? -sn : cs;
    const a: Vec3 = [b.c[0] - ax * hu, 0, b.c[2] - az * hu], c: Vec3 = [b.c[0] + ax * hu, 0, b.c[2] + az * hu];
    let d = Infinity;
    for (const door of this.g.doors?.list ?? []) { const D = door.def; d = Math.min(d, segSegClosest(a, c, [D.hinge[0], 0, D.hinge[1]], [D.hinge[0] + Math.cos(D.closedDir) * D.width, 0, D.hinge[1] + Math.sin(D.closedDir) * D.width]).dist); }
    return d;
  }
  /** the type of pane bi as it will break (an override from setType, else typeOf) — null for anything that is not a sheet of glass */
  typeOfPane(bi: number): GlassType | null { const b = this.g.level.boxes[bi]; if (!b || !isSheet(b)) return null; return this.types.get(bi) ?? this.typeOf(b, bi); }
  /** make pane bi break as `type` from here on (tuning, probes, a level's own table); the pane is made whole first. null = back to typeOf. An override is the level's word on the pane: reset() keeps it */
  setType(bi: number, type: GlassType | null) { if (type) this.types.set(bi, type); else this.types.delete(bi); this.panes.delete(bi); this.g.col.wrote(); this.g.col.glassWrote(); this.blasted.length = 0; const src = this.gusts.get(bi); if (src) { const i = this.g.wind.all.indexOf(src); if (i >= 0) this.g.wind.all.splice(i, 1); this.gusts.delete(bi); } }
  private stateOf(bi: number): PaneState | null {
    let st = this.panes.get(bi); if (st) return st;
    const type = this.typeOfPane(bi); if (!type) return null;   // (a Glass box thicker than a sheet — solid glass — never comes apart)
    const b = this.g.level.boxes[bi]; const uAxis: 0 | 2 = b.h[0] >= b.h[2] ? 0 : 2;
    st = { bi, box: b, key: boxSeed(b), type, uAxis, hu: b.h[uAxis], hv: b.h[1], ht: Math.min(b.h[0], b.h[2]), exterior: this.exteriorOf(bi),
      sites: [], run: false, open: new Uint8Array(GP_MAX_SITES * GP_FRAGS_PER_SITE), nOpen: 0, hits: 0, energy: 0, damage: 0, frags: null, pending: [], nextThink: 0, soundAt: -1e9,
      dice: null, gone: false, lastDir: [0, 0, 1], lastWho: 0, lastUV: [0, 0], walkable: false, log: [] };
    this.panes.set(bi, st); GB_TWIN.panes++;
    return st;
  }
  /** world point → pane-local (u, v) (collision.ts lineBox's frame: local = Rᵀ·(X − c)) */
  toUV(st: PaneState, X: Vec3): Pt {
    const b = st.box; const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); const px = X[0] - b.c[0], pz = X[2] - b.c[2];
    const lx = cs * px - sn * pz, lz = sn * px + cs * pz;
    return [st.uAxis === 0 ? lx : lz, X[1] - b.c[1]];
  }
  /** …and held a hair inside the frame (a blow on the very edge still lands on glass): what impact() and widen() classify by */
  private toUVIn(st: PaneState, X: Vec3): Pt { const [u, v] = this.toUV(st, X); return [Math.max(-st.hu * 0.995, Math.min(st.hu * 0.995, u)), Math.max(-st.hv * 0.995, Math.min(st.hv * 0.995, v))]; }
  /** pane-local (u, v, w across the thickness) → world */
  toWorld(st: PaneState, u: number, v: number, w = 0): Vec3 {
    const b = st.box; const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw);
    const lx = st.uAxis === 0 ? u : w, lz = st.uAxis === 0 ? w : u;
    return [b.c[0] + cs * lx + sn * lz, b.c[1] + v, b.c[2] - sn * lx + cs * lz];
  }
  /** the pane's unit normal = the direction of pane-local +w */
  normalOf(st: PaneState): Vec3 { const b = st.box; const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); return st.uAxis === 0 ? [sn, 0, cs] : [cs, 0, -sn]; }
  private fragsOf(st: PaneState): GpFrag[] { return st.frags ?? (st.frags = st.type === 'annealed' ? gpFragments(st.sites, st.run, st.hu, st.hv) : []); }

  // ---------------------------------------------------------------- sites
  /** a new site at (u, v) for a blow of energy e, its ring zone fitted to the room the others leave; −1 = no room (the list is full, or the point is too close to a zone) */
  private addSite(st: PaneState, u: number, v: number, e: number, seed32: number): number {
    if (st.sites.length >= Math.min(GP_MAX_SITES, GLASSBREAK.maxSites) || st.run) return -1;
    const web = st.type === 'laminated'; const base = GLASSBREAK.rho[web ? 'laminated' : 'annealed'];
    const k = Math.max(0.8, Math.min(1.4, 0.75 + 0.25 * e)); const top = GP_RHO_LO + GP_RHO_SPAN;
    const rho: [number, number, number, number] = [base[0] * k, base[1] * k, base[2] * k, base[3] * k];
    let rz = rho[2] * top * 1.001, room = Infinity;
    for (const q of st.sites) room = Math.min(room, Math.hypot(q.u - u, q.v - v) - q.rz);
    if (room < (web ? GLASSBREAK.rzMin * 0.4 : GLASSBREAK.rzMin)) return -1;
    if (rz > room) { const f = room / rz; rho[0] *= f; rho[1] *= f; rho[2] *= f; rz = room; }
    rho[3] = Math.max(rho[3], rho[2] * 2);
    const seed = seed32 >>> 8; const R = GLASSBREAK.radials[web ? 'laminated' : 'annealed'];
    st.sites.push({ u, v, rot: (seed & 0xfff) / 4096 * Math.PI * 2, n: Math.min(GP_MAX_SECTORS, R[0] + ((seed >>> 12) % Math.max(1, R[1]))), seed, rho, rz, e, merge: web ? 0 : GLASSBREAK.merge, hits: 1 });
    st.frags = null; GB_TWIN.sites++;
    return st.sites.length - 1;
  }
  private nearestSite(st: PaneState, u: number, v: number): number { return gpNearest(st.sites, u, v); }   // (whose territory the point is in: the power diagram the pieces are cut by)

  // ---------------------------------------------------------------- stimuli
  /** THE IMPACT ENTRY POINT (fracture.md): a blow of `energy` (1 = one pistol round square on) met pane `bi` at world X travelling `dir`. Files it and answers what the
   *  glass did; what the round does next is the caller's. `through` = the caller's own verdict on the round, where it has decided one before calling (the penetration
   *  part's toll): true = it went on through, so the glass is opened where it passed (annealed: the crater or the piece out, the cracks run if there was no room for a star; tempered: the
   *  pane dices; laminated: its web, which never opens to a ray) — false = it stopped in the glass: annealed keeps every piece (it still cracks, and its energy still counts
   *  toward the run), tempered still dices at its break energy (a round can spend itself breaking toughened glass) —
   *  undefined = the glass decides by energy (punchE). glass.ts glassImpact wraps this with the star decal, the crack heard and the hearing event. */
  impact(bi: number, X: Vec3, dir: Vec3 | null, energy: number, who: number, through?: boolean): GlassImpact {
    const c = this.g.col; c.glassBegin(); try { return this.impactIn(bi, X, dir, energy, who, through); } finally { c.glassEnd(); }   // (the copy's panes follow every write of this: boxMirror.ts glassBegin)
  }
  private impactIn(bi: number, X: Vec3, dir: Vec3 | null, energy: number, who: number, through?: boolean): GlassImpact {
    const g = this.g; const b = g.level.boxes[bi]; const isGlass = !!b && (b.flags & BoxFlag.Glass) !== 0;
    const res: GlassImpact = { pane: isGlass, live: false, type: 'annealed', thickness: isGlass ? 2 * Math.min(b.h[0], b.h[2]) : 0, wasOpen: false, punched: false, diced: false, released: 0, cracked: false };
    if (!isGlass) return res;
    res.type = this.typeOfPane(bi) ?? 'laminated';   // (solid glass answers as the kind that never opens)
    if (!glassBreakLive(g)) return res;   // the switch off, chaos 0, or the tour = the game byte for byte: glass.ts's star and noise are all that happens
    const st = this.stateOf(bi); if (!st) return res;
    res.live = true;
    const [u, v] = this.toUVIn(st, X);
    if (this.openUV(st, u, v)) { res.wasOpen = true; res.punched = true; return res; }   // (a round through a hole files nothing: raycast should not have found the pane; belt and braces)
    st.hits++; st.energy += energy; st.damage = Math.min(1, st.damage + 0.12 * energy); st.lastWho = who; st.lastUV = [u, v];
    if (dir && v3.len(dir) > 1e-6) st.lastDir = v3.normalize(dir);
    GB_TWIN.impacts++;
    const seed = pointSeed(st.key, X);
    if (st.type === 'tempered') {
      if (st.energy >= GLASSBREAK.breakE.tempered || through === true) { /* (a round that went THROUGH toughened glass has broken it, whatever it had left) */ const started = !st.dice; this.startDice(st, u, v, seed, 'hit'); res.diced = true; res.punched = this.openUV(st, u, v); res.released = started ? 1 : 0; res.cracked = true; }   // (a round into a pane already coming down: the clump under it may still stand — punched says so)
      return res;
    }
    if (st.type === 'laminated') {   // the web: a site while there is room, else the nearest takes it — nothing ever opens to a round
      if (gpClassify(st.sites, false, u, v) < 0 && this.addSite(st, u, v, energy, seed) >= 0) { res.cracked = true; return res; }
      const i = this.nearestSite(st, u, v); if (i >= 0) { st.sites[i].e += energy; st.sites[i].hits++; }
      return res;
    }
    // annealed
    const hard = through ?? energy >= GLASSBREAK.punchE;   // the caller's word on the round where it has one: through = a hole where it passed, stopped = the glass is still there
    let fid = gpClassify(st.sites, st.run, u, v); let noRoom = false;
    if (fid >= 0) {   // on a piece that is still in: the blow counts on its site, and a hard one knocks the piece out
      const S = st.sites[gpUnfid(fid).i]; S.e += energy; S.hits++;
      if (hard) res.released += this.knockOut(st, fid);
    } else {   // on the uncut sheet: a new star if there is room for one
      const i = this.addSite(st, u, v, energy, seed);
      if (i >= 0) { res.cracked = true; if (hard) res.released += this.releaseCrater(st, i, 'hit'); }
      else { const j = this.nearestSite(st, u, v); if (j >= 0) { st.sites[j].e += energy; st.sites[j].hits++; noRoom = true; } }
    }
    // (a round the caller says went THROUGH, on sheet with no room left for a star of its own: the cracks run, so there is a piece under it to go)
    if (!st.run && (st.energy >= GLASSBREAK.breakE.annealed || energy >= GLASSBREAK.runSingleE || (through === true && noRoom))) {
      this.setRun(st, 'the cracks run to the frame');
      if (hard && !this.openUV(st, u, v)) { fid = gpClassify(st.sites, true, u, v); if (fid >= 0) res.released += this.knockOut(st, fid); }   // the dagger the blow landed on
      const j = hard ? this.nearestSite(st, u, v) : -1;   // …and its star's rings go after the crater over the next few frames; what that leaves unheld is the support rule's
      if (j >= 0 && GLASSBREAK.runStar > 0) {
        for (const f of this.fragsOf(st)) if (f.i === j && f.b >= 1 && f.b <= Math.min(2, GLASSBREAK.runStar) && !st.open[f.fid] && !st.pending.some(p => p.fid === f.fid)) st.pending.push({ fid: f.fid, at: g.time + GLASSBREAK.runRingSecs * (f.b + hash01(st.key * 53 + f.fid)), cause: 'hit', from: null, k: 1 });
        st.pending.sort((x, y) => x.at - y.at || x.fid - y.fid);
      }
    }
    res.punched = this.openUV(st, u, v); if (res.released > 0) res.cracked = true;
    if (st.nextThink <= 0) st.nextThink = g.time + GLASSBREAK.think;   // (never pushed BACK: automatic fire must not hold the support pass off for a whole magazine)
    return res;
  }
  /** the base's name for a round's impact at energy 1 (probes, the console) */
  hit(bi: number, X: Vec3, dir: Vec3 | null, who: number, energy = 1) { this.impact(bi, X, dir, energy, who); }
  /** A PELLET THROUGH GLASS ITS OWN SHELL HAS JUST BROKEN (glasspen.ts, the shell rule): no blow is filed — no hit, no energy toward the run, no star — but if a
   *  piece of an annealed pane still stands where it passed, that piece goes with it: the load leaves one ragged hole. Uncut sheet there is left alone (it passed
   *  by the hole, a pellet's width off); toughened glass is already coming down and a laminated web never opens. Returns fragments let go. */
  widen(bi: number, X: Vec3): number {
    if (!glassBreakLive(this.g)) return 0;
    const st = this.panes.get(bi); if (!st || st.type !== 'annealed') return 0;
    const [u, v] = this.toUVIn(st, X); if (this.openUV(st, u, v)) return 0;
    const fid = gpClassify(st.sites, st.run, u, v); if (fid < 0) return 0;
    const c = this.g.col; c.glassBegin();
    try { const n = this.knockOut(st, fid); if (n > 0 && st.nextThink <= 0) st.nextThink = this.g.time + GLASSBREAK.think; return n; } finally { c.glassEnd(); }
  }
  /** a hard blow on piece fid: it goes now (a crater wedge takes the whole crater with it). Returns fragments let go. */
  private knockOut(st: PaneState, fid: number): number {
    if (st.open[fid]) return 0;
    const { i, b } = gpUnfid(fid);
    if (b === 0) return this.releaseCrater(st, i, 'hit');
    const f = this.fragsOf(st).find(x => x.fid === fid); if (!f) return 0;
    this.releaseFrag(st, f, 'hit', null, 1); return 1;
  }
  private setRun(st: PaneState, why: string) {
    if (st.run || st.type !== 'annealed') return;
    if (st.sites.length === 0) return;
    st.run = true; st.frags = null; GB_TWIN.runs++; this.plog(st, `RUN: ${why}`); this.note(`pane ${st.bi} runs (${why})`);
    st.nextThink = this.g.time + GLASSBREAK.think;
  }
  /** A BLAST at `at` (destruction's blast entry point calls this; a stun charge's 'bang' comes through glass.ts at bangPower): every sheet whose nearest point is within
   *  the radius takes it by type and by k = power × (1 − d / radius) — tempered dices (k > blastDiceK), annealed is starred toward the blast, runs (k > blastRunK) and loses every piece
   *  within 1.4 k + 0.15 m of the nearest point but the teeth on its frame, laminated crazes and (k > blastSheetK) leaves its frame as one sheet. Returns the panes it changed. */
  blast(at: Vec3, opts: GlassBlastOpts = {}): number {
    if (!glassBreakLive(this.g)) return 0;   // (as impact(): nothing with the switch off / at chaos 0 / under the tour)
    const c = this.g.col; c.glassBegin(); try { return this.blastIn(at, opts); } finally { c.glassEnd(); }
  }
  private blastIn(at: Vec3, opts: GlassBlastOpts): number {
    const radius = opts.radius ?? GLASSBREAK.blastM, power = opts.power ?? 1, who = opts.who ?? 0;
    if (!(radius > 0) || !(power > 0)) return 0;
    // ONE charge, one blow: the same blast can arrive twice (destruction's feed AND its 'bang' hearing event through glass.ts, which lifts a can on the floor to 0.3 m and
    // has its own radius and power) — a pane takes the second arrival only where it would be the STRONGER blow there
    const now = this.g.time; this.blasted = this.blasted.filter(x => now >= x.t && now - x.t < 0.25);
    let twin = this.blasted.find(x => Math.hypot(x.at[0] - at[0], x.at[2] - at[2]) < 0.05 && Math.abs(x.at[1] - at[1]) < 0.6);
    if (!twin) { twin = { t: now, at: v3.copy(at), k: new Map() }; this.blasted.push(twin); GB_TWIN.blasts++; }
    let changed = 0;
    for (const bi of this.g.engine.world.glassStatics) {
      const b = this.g.level.boxes[bi]; if (!b) continue;
      const q = nearestOn(b, at); const d = v3.dist(q, at); if (d > radius) continue;   // (the range test BEFORE any state is made: a far pane is not 'touched')
      const k = power * (1 - d / radius); if (k <= BLAST_MIN_K) continue;
      const prev = twin.k.get(bi) ?? 0; if (prev >= k) continue;
      const type = this.typeOfPane(bi); if (!type || (type === 'tempered' && k <= GLASSBREAK.blastDiceK && !this.panes.has(bi))) continue;   // (toughened glass too far off to dice: untouched, no record)
      const st = this.stateOf(bi); if (!st || st.gone) continue;
      twin.k.set(bi, k);   // (noted only once the pane HAS taken it: `prev` below is then exactly what this charge already did to this pane, and the stronger arrival adds the difference — the pane ends where the stronger blow alone would have put it, whichever came first)
      const away: Vec3 = d > 1e-6 ? v3.normalize(v3.sub(q, at)) : this.normalOf(st);
      st.lastDir = away; st.lastWho = who; const [qu, qv] = this.toUV(st, q); st.lastUV = [qu, qv]; st.damage = Math.min(1, st.damage + 0.3 * (k - prev));
      if (st.type === 'tempered') { if (k > GLASSBREAK.blastDiceK && !st.dice) { st.energy += 1 + 2 * k; this.startDice(st, qu, qv, blastSeed(st.key, twin.at), 'blast'); changed++; } continue; }
      changed++;
      // stars toward the blast: glass.ts crackNear's construction (a hashed point on the face inside its frame by a tenth, pulled a third of the way to the nearest point), so the first two sit on its decals
      const starsOf = (kk: number) => st.type === 'laminated' ? 3 : 1 + Math.round(2 * Math.min(1, kk));
      let s = blastSeed(st.key, twin.at); const stars = starsOf(k), had = prev > 0 ? starsOf(prev) : 0;   // (had: the stars this charge's weaker arrival already put here)
      for (let n = 0; n < stars; n++) {
        s = (Math.imul(s ^ (s >>> 15), 0x2C1B3C6D) + 0x297A2D39) >>> 0; if (n < had) continue;
        const fu = ((s & 0xffff) / 65535) * 2 - 1, fv = ((s >>> 16) / 65535) * 2 - 1;
        const [pu, pv] = this.toUV(st, v3.lerp(this.toWorld(st, fu * 0.9 * st.hu, fv * 0.9 * st.hv), q, 0.35));
        if (this.openUV(st, pu, pv) || gpClassify(st.sites, st.run, pu, pv) >= 0) continue;
        this.addSite(st, pu, pv, 0.6 + k, s);
      }
      st.energy += prev > 0 ? 2 * (k - prev) : 1 + 2 * k;
      if (st.type === 'laminated') { if (k > GLASSBREAK.blastSheetK) this.releaseSheet(st, away, k); continue; }
      if (k > GLASSBREAK.blastRunK) this.setRun(st, 'blast');
      const reach = blastReach(k); let n = 0;
      for (const f of this.fragsOf(st)) {
        if (st.open[f.fid] || st.pending.some(p => p.fid === f.fid)) continue;
        const dd = Math.hypot(f.cen[0] - qu, f.cen[1] - qv); if (dd > reach) continue;
        if (k < GLASSBREAK.toothK && f.area < GLASSBREAK.toothArea && f.frame.bottom + f.frame.left + f.frame.right + f.frame.top > 1e-4) continue;   // a tooth stays in the frame
        st.pending.push({ fid: f.fid, at: this.g.time + dd * 0.03, cause: 'blast', from: at, k }); n++;
      }
      if (n) st.pending.sort((x, y) => x.at - y.at || x.fid - y.fid);
      this.plog(st, `blast k ${k.toFixed(2)}: ${n} pieces within ${reach.toFixed(2)} m`);
      if (st.nextThink <= 0) st.nextThink = this.g.time + GLASSBREAK.think;
    }
    return changed;
  }

  // ---------------------------------------------------------------- a burst's view of the glass (frag.ts: fragments are stopped by glass that stands, and by none that the blast took)
  private also: ((bi: number, X: Vec3) => boolean) | null = null;
  /** THE PANES THAT ARE NO LONGER WHOLE, for the box world's copy (scene/boxMirror.ts PaneSource): every pane with a piece out, a piece queued to leave,
   *  coming down or gone. A pane that only has cracks answers as a whole one everywhere and has no row. */
  each(to: (p: PaneState) => void): void { for (const st of this.panes.values()) if (st.gone || st.dice !== null || st.nOpen > 0 || st.pending.length > 0) to(st); }
  /** For the length of `fn`, every solid walk (StaticCollision.segmentBlocked and what is built on it: solidBlocked, shotBlocked) and every ray ALSO passes a
   *  pane where a burst at `from` of this size sees no glass — on top of what has already fallen out — by the `view` the burst's judgment is made in
   *  (frag.ts GlassView: 'blown' = goingAt, 'wary' = wouldGive of the whole pane, 'counted' = wouldGive at the point met). Nothing is filed and nothing is
   *  kept: the view is gone when fn returns (or throws). The view is said twice, as a rule for this side's walks and as rows for the module's (`gives`). */
  seeThrough<T>(view: 'blown' | 'wary' | 'counted', from: Vec3, size: GlassBlastOpts, fn: () => T): T {
    for (const bi of this.g.engine.world.glassStatics) this.skySide(bi);   // (which glass each pane is, settled BEFORE the view is on: typeOfPane's first answer for a pane casts solid rays of its own — never from inside a walk, and never through this view)
    const also: (bi: number, X: Vec3) => boolean = view === 'blown' ? (bi, X) => this.goingAt(bi, X) : view === 'wary' ? (bi) => this.wouldGive(bi, from, size) : (bi, X) => this.wouldGive(bi, from, size, X);
    const was = this.also, wasOf = this.viewOf; this.also = also; this.viewOf = { kind: view, from, size, rows: undefined }; this.g.col.glassViewed();
    try { return fn(); } finally { this.also = was; this.viewOf = wasOf; this.g.col.glassViewed(); }
  }
  /** the view in hand as rows (scene/boxMirror.ts PaneSource), worked out when the copy first reads it and kept for the judgment: 'blown' names no pane
   *  (the module answers it from the pieces queued to leave); the other two list the panes the burst would break. null = no view, or the model is not live. */
  get view(): PaneView | null {
    const V = this.viewOf; if (V === null) return null;
    if (V.rows === undefined) V.rows = V.kind === 'blown' ? { kind: VIEW_BLOWN, bi: [], at: [] } : this.gives(V.from, V.size, V.kind === 'counted');
    return V.rows;
  }
  private viewOf: { kind: 'blown' | 'wary' | 'counted'; from: Vec3; size: GlassBlastOpts; rows: PaneView | null | undefined } | null = null;
  /** wouldGive for every sheet of the level at once, as the rows of a view: each pane that gives, its nearest point to the burst and its reach (`give`). A
   *  pane coming down or gone is not listed: its own row says so. null = the model is not live (wouldGive answers false everywhere). */
  private gives(at: Vec3, opts: GlassBlastOpts, sure: boolean): PaneView | null {
    if (!glassBreakLive(this.g)) return null;
    const rows = { kind: VIEW_GIVES, bi: [] as number[], at: [] as number[] }, radius = opts.radius ?? GLASSBREAK.blastM, power = opts.power ?? 1; if (!(radius > 0) || !(power > 0)) return rows;
    const boxes = this.g.level.boxes, q: Vec3 = [0, 0, 0];
    for (const bi of glassPanes(this.g)) {   // (the level's glass, listed once a game: glass.ts)
      const b = boxes[bi]; if (!isSheet(b)) continue;
      const reach = this.give(bi, b, at, radius, power, sure, q); if (reach === reach) { rows.bi.push(bi); rows.at.push(q[0], q[1], q[2], reach); }
    }
    return rows;
  }
  /** HOW A SHEET OF GLASS GIVES to a blast of this size at `at` — wouldGive's rule, stated once: by blast()'s own strengths and nothing of the pane's cracks,
   *  toughened glass dices or it does not, a laminated sheet leaves its frame or it does not; float glass is counted out whole by a wary man wherever the blast
   *  touches it at all, and by one who counts on it (`sure`) only inside the reach blast() takes pieces within, round the pane's nearest point, and only where
   *  the blow is hard enough to take the slivers on the frame with it (toothK). → the reach round the nearest point (written to `q`) within which the glass
   *  goes: below nought = the whole pane, NaN = it does not give. */
  private give(bi: number, b: Box, at: Vec3, radius: number, power: number, sure: boolean, q: Vec3): number {
    const K = GLASSBREAK, p = nearestOn(b, at), k = power * (1 - v3.dist(p, at) / radius); q[0] = p[0]; q[1] = p[1]; q[2] = p[2];
    if (k < Math.min(K.blastDiceK, K.blastSheetK, sure ? K.toothK : BLAST_MIN_K)) return NaN;   // (under every kind's bar: no need to ask which glass it is, which may cast rays)
    const type = this.types.get(bi) ?? this.typeOf(b, bi);
    if (type === 'tempered') return k > K.blastDiceK ? -1 : NaN;
    if (type === 'laminated') return k > K.blastSheetK ? -1 : NaN;
    if (!sure) return k > BLAST_MIN_K ? -1 : NaN;
    const reach = blastReach(k); return k >= K.toothK && reach >= 0 ? reach : NaN;   // (under toothK a sliver on the frame stays in it: not SURE)
  }
  /** Is the glass of pane bi at X ON ITS WAY OUT: the piece there is queued to leave (by a blast() — a crater's wedges go together within a few frames — or by
   *  anything before it: rounds that ran the pane a moment ago, a piece that lost its support), the pane is toughened glass coming down, or it has gone. What
   *  blast() only STARTED on the frame it was called is already no glass to that blast's own fragments. A piece left in (a tooth on the frame, sheet out of the
   *  blast's reach, a laminated web) answers false: it stands, and stops what meets it. */
  goingAt(bi: number, X: Vec3): boolean {
    const st = this.panes.get(bi); if (!st) return false;
    if (st.gone || st.dice) return true;
    if (!st.pending.length) return false;
    const [u, v] = this.toUV(st, X); const fid = gpClassify(st.sites, st.run, u, v); if (fid < 0) return false;
    const at = gpUnfid(fid);
    return st.pending.some(p => p.fid === fid || (at.b === 0 && gpUnfid(p.fid).b === 0 && gpUnfid(p.fid).i === at.i));
  }
  /** WOULD a blast of this size at `at` take pane bi out of the way — a man's reckoning before it goes off, by blast()'s own strengths and nothing of the pane's cracks:
   *  toughened glass dices or it does not, a laminated sheet leaves its frame or it does not; float glass, with no X, is counted out wherever the blast touches it at
   *  all (the cautious answer: a man picking cover from a grenade trusts none of it), and with X only inside the reach blast() takes pieces within, round the pane's
   *  nearest point, and only where the blow is hard enough to take the slivers on the frame with it (toothK — the confident answer: a man counting on his own grenade to open it). False when the model is not live. */
  wouldGive(bi: number, at: Vec3, opts: GlassBlastOpts = {}, X?: Vec3): boolean {
    if (!glassBreakLive(this.g)) return false;
    const st = this.panes.get(bi); if (st && (st.gone || st.dice)) return true;
    const b = this.g.level.boxes[bi]; if (!b || !isSheet(b)) return false;
    const radius = opts.radius ?? GLASSBREAK.blastM, power = opts.power ?? 1; if (!(radius > 0) || !(power > 0)) return false;
    const q: Vec3 = [0, 0, 0], reach = this.give(bi, b, at, radius, power, X !== undefined, q);
    return reach < 0 || (X !== undefined && v3.dist(X, q) <= reach);
  }

  /** a stun charge went off at `at` (glass.ts updateGlass, off the 'bang' event): a blast at bangPower */
  bang(at: Vec3, who: number): number { return this.blast(at, { radius: GLASSBREAK.blastM, power: GLASSBREAK.bangPower, who }); }

  // ---------------------------------------------------------------- tempered: the dice
  private startDice(st: PaneState, u: number, v: number, seed: number, why: string) {
    if (st.dice || st.gone) return;
    const cols = Math.max(1, Math.round(2 * st.hu / GLASSBREAK.clumpM)), rows = Math.max(1, Math.round(2 * st.hv / GLASSBREAK.clumpM));
    const D: Dice = { u, v, cols, rows, seed: seed >>> 8, hu: st.hu, hv: st.hv, t0: this.g.time, clumps: [], next: 0, out: new Uint8Array(cols * rows) };
    for (let r = 0; r < rows; r++) for (let c = 0; c < cols; c++) D.clumps.push({ k: r * cols + c, c, r, key: gpDiceKey(D, c, r) });
    D.clumps.sort((a, b) => a.key - b.key || a.k - b.k);
    st.dice = D;
    GB_TWIN.diced++; this.note(`pane ${st.bi} DICES (${why})`); this.plog(st, `dices (${why}): ${D.clumps.length} clumps`);
    const k0 = gpDiceClump(D, u, v); const key0 = D.clumps.find(c => c.k === k0)!.key;   // the clump under the blow goes with it (and whatever is ahead of it in the order — the renderer's rule is one threshold on the key): there is no glass on the round's line from this call on
    while (D.next < D.clumps.length && D.clumps[D.next].key <= key0 + DICE_TIE) { this.releaseClump(st, D, D.clumps[D.next]); D.next++; }
    if (D.next >= D.clumps.length) this.diceDone(st);
  }
  private diceDone(st: PaneState) { st.gone = true; st.walkable = true; this.g.col.wrote(); if (st.exterior) this.gustDirty = true; }
  private stepDice(st: PaneState) {
    if (GLASSREAL.drop) { if (this.g.time - st.dice!.t0 >= GLASSREAL.hang) this.dropAll(st, st.dice!); else if (st.walkable) { st.walkable = false; this.g.col.wrote(); } return; }
    const D = st.dice!; const R = (this.g.time - D.t0 - GLASSBREAK.diceHang) * GLASSBREAK.diceSpeed; let n = 0;
    // (clumps with the same key go together whatever the budget: the renderer opens by ONE threshold on the key — fill() hands it the midpoint between the last clump out and the next in)
    while (D.next < D.clumps.length && (D.clumps[D.next].key <= D.clumps[D.next - 1].key + DICE_TIE || (D.clumps[D.next].key <= R && n < GLASSBREAK.clumpsPerFrame))) { this.releaseClump(st, D, D.clumps[D.next]); D.next++; n++; }
    if (D.next >= D.clumps.length) this.diceDone(st); else if (st.walkable) { st.walkable = false; this.g.col.wrote(); }
  }
  private releaseClump(st: PaneState, D: Dice, c: Clump) {
    D.out[c.k] = 1; GB_TWIN.clumps++; GB_TWIN.dropped++;
    const pts = gpDicePoly(D, c.c, c.r); if (pts.length < 3) return;
    const { area, cen } = gpPolyArea(pts); if (area < 1e-8) return;
    const n = this.normalOf(st); const outSign = v3.dot(n, st.lastDir) >= 0 ? 1 : -1; const N = v3.scale(n, outSign);
    const h1 = hash01(st.key * 7919 + c.k * 131), h2 = hash01(st.key * 104729 + c.k * 193), h3 = hash01(c.k * 7 + st.key);
    const rad = Math.max(1e-3, Math.hypot(cen[0] - D.u, cen[1] - D.v));
    const spread = this.dirUV(st, (cen[0] - D.u) / rad, (cen[1] - D.v) / rad);   // a little outward in the pane's plane from the blow: the sheet opens like a curtain
    const velocity = v3.add(v3.add(v3.scale(N, GLASSBREAK.out * (0.3 + 0.9 * h1)), v3.scale(spread, 0.5 * h2)), [0, -0.3 - 0.5 * h3, 0]);
    this.emit(st, { pane: st.bi, type: st.type, kind: 'clump', outline: this.outline(st, pts, outSign), centre: this.toWorld(st, cen[0], cen[1]), normal: N, thickness: 2 * st.ht, area,
      velocity, spin: [(h1 - 0.5) * GLASSBREAK.spin, (h2 - 0.5) * 2, (h3 - 0.5) * GLASSBREAK.spin], tint: v3.copy(st.box.albedo), grain: GLASSBREAK.diceGrain, cause: 'dice', who: st.lastWho, seed: (D.seed ^ Math.imul(c.k + 1, 0x9E3779B1)) >>> 0 }, c.k);
  }
  /** GLASSREAL.drop: every clump still standing leaves on this step. The sheet is handed on in square BLOCKS of clumps, at most GLASSREAL.blocks of them (one
   *  fragment a clump would be several hundred emit records in a frame from one facade sheet): a block is the rectangle of its lattice cells, its area theirs, its
   *  centre the mean of its clumps'. They fall — out along the blow a little, most near it, and down; nothing across the pane. */
  private dropAll(st: PaneState, D: Dice) {
    const left = D.clumps.length - D.next; let m = 1; while (Math.ceil(D.cols / m) * Math.ceil(D.rows / m) > Math.max(1, GLASSREAL.blocks)) m++;
    const du = 2 * D.hu / D.cols, dv = 2 * D.hv / D.rows; const bc = Math.ceil(D.cols / m), br = Math.ceil(D.rows / m);
    const n = new Uint16Array(bc * br), su = new Float64Array(bc * br), sv = new Float64Array(bc * br);
    for (; D.next < D.clumps.length; D.next++) {
      const c = D.clumps[D.next]; D.out[c.k] = 1; GB_TWIN.clumps++; GB_TWIN.dropped++;
      const b = Math.floor(c.r / m) * bc + Math.floor(c.c / m); const p = gpDiceCentre(D, c.c, c.r); n[b]++; su[b] += p[0]; sv[b] += p[1];
    }
    const nrm = this.normalOf(st); const outSign = v3.dot(nrm, st.lastDir) >= 0 ? 1 : -1; const N = v3.scale(nrm, outSign);
    for (let b = 0; b < n.length; b++) {
      if (!n[b]) continue; const c0 = (b % bc) * m, r0 = Math.floor(b / bc) * m;
      const u0 = -D.hu + c0 * du, u1 = Math.min(D.hu, u0 + m * du), v0 = -D.hv + r0 * dv, v1 = Math.min(D.hv, v0 + m * dv); const cu = su[b] / n[b], cv = sv[b] / n[b];
      const h1 = hash01(st.key * 7919 + b * 131), h3 = hash01(b * 7 + st.key); const near = Math.max(0, 1 - Math.hypot(cu - D.u, cv - D.v) / 0.6);
      this.emit(st, { pane: st.bi, type: st.type, kind: 'clump', outline: this.outline(st, [[u0, v0], [u1, v0], [u1, v1], [u0, v1]], outSign), centre: this.toWorld(st, cu, cv), normal: N, thickness: 2 * st.ht, area: n[b] * du * dv,
        velocity: v3.add(v3.scale(N, GLASSBREAK.out * (0.1 + 0.25 * h1 + 0.8 * near)), [0, -0.3 - 0.5 * h3, 0]), spin: [(h1 - 0.5) * GLASSBREAK.spin, 0, (h3 - 0.5) * GLASSBREAK.spin], tint: v3.copy(st.box.albedo), grain: GLASSBREAK.diceGrain, cause: 'dice', who: st.lastWho, seed: (D.seed ^ Math.imul(b + 1, 0x9E3779B1)) >>> 0 }, b);
    }
    this.plog(st, `drops: ${left} clumps as blocks of ${m} × ${m}`);
    this.diceDone(st);
  }
  private releaseSheet(st: PaneState, away: Vec3, k: number) {
    if (st.gone) return;
    st.gone = true; st.walkable = true; this.g.col.wrote(); st.pending.length = 0; GB_TWIN.sheets++; GB_TWIN.dropped++; if (st.exterior) this.gustDirty = true;
    const n = this.normalOf(st); const outSign = v3.dot(n, away) >= 0 ? 1 : -1; const N = v3.scale(n, outSign);
    const pts: Pt[] = [[-st.hu, -st.hv], [st.hu, -st.hv], [st.hu, st.hv], [-st.hu, st.hv]];
    this.note(`pane ${st.bi}: the laminated sheet leaves its frame`); this.plog(st, 'sheet blown out');
    this.emit(st, { pane: st.bi, type: st.type, kind: 'sheet', outline: this.outline(st, pts, outSign), centre: v3.copy(st.box.c), normal: N, thickness: 2 * st.ht, area: 4 * st.hu * st.hv,
      velocity: v3.add(v3.scale(N, GLASSBREAK.blastOut * k * 0.6), [0, 0.4, 0]), spin: [1.5 * k, 0, 0.4], tint: v3.copy(st.box.albedo), grain: 0, cause: 'blast', who: st.lastWho, seed: pointSeed(st.key, st.box.c) }, 0);
  }

  // ---------------------------------------------------------------- pieces letting go
  private dirUV(st: PaneState, du: number, dv: number): Vec3 { const a = this.toWorld(st, 0, 0), b = this.toWorld(st, du, dv); return v3.sub(b, a); }
  /** pane-local polygon → the world outline, anticlockwise seen from the side the fragment leaves by */
  private outline(st: PaneState, pts: Pt[], outSign: number): Vec3[] {
    // (u, v, w) is right-handed for uAxis 0 (x, y, z) and left-handed for uAxis 2 (z, y, x): an anticlockwise (u, v) polygon is anticlockwise seen from +w in the first, clockwise in the second
    const ccwFromPlusW = st.uAxis === 0; const keep = (outSign > 0) === ccwFromPlusW;
    const P = keep ? pts : pts.slice().reverse();
    return P.map(p => this.toWorld(st, p[0], p[1]));
  }
  private releaseCrater(st: PaneState, i: number, cause: GlassFragment['cause']): number {
    const S = st.sites[i]; let any = false;
    for (let s = 0; s < S.n; s++) { const fid = i * GP_FRAGS_PER_SITE + s; if (!st.open[fid]) { st.open[fid] = 1; st.nOpen++; any = true; } }
    if (!any) return 0;
    GB_TWIN.dropped++;
    const hull = gpClipRect(gpHull(Array.from({ length: S.n }, (_, s) => gpChordPt(S, s, 0))), st.hu, st.hv); if (hull.length < 3) return 0;
    const { area, cen } = gpPolyArea(hull);
    const n = this.normalOf(st); const outSign = v3.dot(n, st.lastDir) >= 0 ? 1 : -1; const N = v3.scale(n, outSign);
    const h = hash01(S.seed);
    this.plog(st, `crater ${i} out`);
    this.emit(st, { pane: st.bi, type: st.type, kind: 'piece', outline: this.outline(st, hull, outSign), centre: this.toWorld(st, cen[0], cen[1]), normal: N, thickness: 2 * st.ht, area,
      velocity: v3.add(v3.scale(N, 2.5 + 2 * h), [0, -0.2, 0]), spin: [0, 0, 0], tint: v3.copy(st.box.albedo), grain: 0.004, cause, who: st.lastWho, seed: (S.seed ^ 0xC2A7E5) >>> 0 }, i * GP_FRAGS_PER_SITE);
    this.afterRelease(st);
    return 1;
  }
  private releaseFrag(st: PaneState, f: GpFrag, cause: GlassFragment['cause'], from: Vec3 | null, k: number) {
    if (st.open[f.fid]) return;
    st.open[f.fid] = 1; st.nOpen++; GB_TWIN.dropped++;
    const n = this.normalOf(st); const cw = this.toWorld(st, f.cen[0], f.cen[1]);
    let dirOut = st.lastDir; if (from) { const d = v3.sub(cw, from); if (v3.len(d) > 1e-6) dirOut = v3.normalize(d); }
    const outSign = v3.dot(n, dirOut) >= 0 ? 1 : -1; const N = v3.scale(n, outSign);
    const h1 = hash01(st.key * 7919 + f.fid * 131), h2 = hash01(st.key * 104729 + f.fid * 193), h3 = hash01(f.fid * 7 + st.key);
    const speed = cause === 'blast' ? GLASSBREAK.blastOut * k * (0.6 + 0.8 * h3) : cause === 'hit' ? GLASSBREAK.out * (0.8 + 1.2 * h3) : cause === 'rake' ? GLASSBREAK.out * (1.2 + 0.8 * h3) : GLASSBREAK.out * (0.15 + 0.5 * h3);   // what a round knocks out leaves smartly; what merely lost its hold tips out and drops
    const velocity = v3.add((cause === 'blast' || cause === 'rake') && from ? v3.scale(v3.normalize(v3.add(v3.scale(dirOut, 0.7), v3.scale(N, 0.5))), speed) : v3.scale(N, speed), [0, -0.2 - 0.6 * h1, 0]);
    this.plog(st, `piece ${f.i}:${f.b}:${f.s} out (${cause})`);
    this.emit(st, { pane: st.bi, type: st.type, kind: f.b >= 3 ? 'dagger' : 'piece', outline: this.outline(st, f.pts, outSign), centre: cw, normal: N, thickness: 2 * st.ht, area: f.area,
      velocity, spin: [(h1 - 0.5) * GLASSBREAK.spin, (h2 - 0.5) * 2, (h3 - 0.5) * GLASSBREAK.spin], tint: v3.copy(st.box.albedo), grain: 0, cause, who: st.lastWho, seed: (pointSeed(st.key, cw) ^ f.fid) >>> 0 }, f.fid);
    this.afterRelease(st);
  }
  private afterRelease(st: PaneState) { const w = this.walkableNow(st); if (w !== st.walkable) { st.walkable = w; this.g.col.wrote(); } if (st.exterior) this.gustDirty = true; }
  /** a fragment has let go (its hole is already real): the break heard — one crack and one hearing event per pane per frame — then the consumer, or the stand-in */
  private emit(st: PaneState, f: GlassFragment, key: number) {
    const g = this.g;
    if (this.shadow) { let heard = false; if (f.cause !== 'hit' && g.time - st.soundAt >= GLASSBREAK.crackGap) { st.soundAt = g.time; heard = true; } this.shadow.push({ ...f, key, heard }); return; }
    if (f.cause !== 'hit' && g.time - st.soundAt >= GLASSBREAK.crackGap) {   // a round's own piece was heard as the round (glass.ts glassImpact); what falls LATER — by support, a blast, the dice — is heard here, once per pane per crackGap: a diced window is dozens of clumps a frame, and every event costs every man a propagation
      st.soundAt = g.time; const dicing = f.kind === 'clump' || f.kind === 'sheet';
      g.audio.play('glassCrack', f.centre, dicing ? 1.0 : 0.75, { rate: dicing ? 0.7 : 1.25 + 0.3 * hash01(key + st.key) });
      glassEvent(g, st.box, f.centre, 1.0, st.lastWho);
    }
    if (this.onRelease) { GB_TWIN.hooked++; this.onRelease(f); return; }
    this.standIn(st, f, key);
  }
  /** THE STAND-IN (no shards consumer attached): the base's picture — one to three thin Debris boxes of kind 'shard' over the fragment + a burst of pale chips; a clump is
   *  chips with a rigid crumb for every crumbEvery-th */
  private standIn(st: PaneState, f: GlassFragment, key: number) {
    const g = this.g; const uv = f.outline.map(p => this.toUV(st, p)); const { cen } = gpPolyArea(uv);
    const grains = f.grain > 0; const sheet = f.kind === 'sheet';
    const nSh = grains ? (f.kind === 'clump' && key % GLASSBREAK.crumbEvery === 0 ? 1 : 0) : sheet ? 3 : Math.max(1, Math.min(3, Math.round(f.area / 0.07)));
    const outSign = v3.dot(this.normalOf(st), f.normal) >= 0 ? 1 : -1;
    for (let k = 0; k < nSh; k++) {
      const h1 = hash01(st.key * 7919 + key * 131 + k * 17), h2 = hash01(st.key * 104729 + key * 193 + k * 29);
      const vtx = uv[(k * Math.max(1, Math.floor(uv.length / nSh))) % uv.length] ?? cen;
      const pu = cen[0] * (nSh === 1 ? 1 : 0.6) + vtx[0] * (nSh === 1 ? 0 : 0.4), pv = cen[1] * (nSh === 1 ? 1 : 0.6) + vtx[1] * (nSh === 1 ? 0 : 0.4);
      const side = Math.sqrt(f.area / nSh) * (grains ? 0.35 : 0.92); const su = side * (0.8 + 0.4 * h1), sv = side * (0.8 + 0.4 * h2);
      const hh: Vec3 = st.uAxis === 0 ? [su / 2, sv / 2, Math.max(0.006, st.ht)] : [Math.max(0.006, st.ht), sv / 2, su / 2];
      const box = makeBox({ c: this.toWorld(st, pu, pv, outSign * 0.02), h: hh, yaw: st.box.yaw, albedo: v3.copy(GLASSBREAK.shardAlbedo), flags: BoxFlag.Dynamic });
      const ch = g.destr.debris.spawn('shard', box, v3.copy(f.velocity), v3.copy(f.spin), f.who);
      if (ch) { ch.box.rough = 0.05; GB_TWIN.shards++; } else GB_TWIN.refused++;
    }
    const P = gpuFx(g);   // chips: pale specks thrown out and down from across the fragment (GPU particles; nothing with them off / headless)
    if (P) { const nChips = Math.round(Math.max(grains ? 3 : GLASSBREAK.chipsMin, f.area * (grains ? GLASSBREAK.diceChipsPerM2 : GLASSBREAK.chipsPerM2))); P.emit('debris', f.centre, f.normal, nChips, { shape: 'cone', speed: 1.1, speedJit: 0.8, spread: 1.2, life: 30, lifeJit: 0.4, size: grains ? 0.012 : 0.008, sizeJit: 0.6, aspect: 1.4, posJit: Math.sqrt(f.area) * 0.45, color: v3.copy(GLASSBREAK.chipColor) }); GB_TWIN.chips += nChips; }
  }

  // ---------------------------------------------------------------- support
  /** which intact pieces of an annealed pane are HELD (the rule at the head of the file) — grown outward from the frame and the uncut sheet, so every held piece is joined
   *  to them through held pieces; everything else is due to fall */
  heldSet(st: PaneState, without = -1): Set<number> {
    const frags = this.fragsOf(st); const K = GLASSBREAK; const by = new Map<number, GpFrag>(); for (const f of frags) if (!st.open[f.fid] && f.fid !== without) by.set(f.fid, f);
    const held = new Set<number>();
    for (let changed = true; changed;) {
      changed = false;
      for (const f of by.values()) {
        if (held.has(f.fid)) continue;
        let ok = false, u0 = Infinity, u1 = -Infinity, grip = (f.frame.left + f.frame.right + f.frame.bottom) * K.grip.side + f.frame.top * K.grip.top;
        if (f.foot) { u0 = f.foot[0]; u1 = f.foot[1]; }
        for (const n of f.nb) {
          if (n.fid === GP_REMAINDER) { ok = true; break; }
          const o = by.get(n.fid); if (!o || !held.has(o.fid)) continue;
          grip += n.len * K.grip.wedged;
          if (n.ny < -K.standNy) { u0 = Math.min(u0, n.u0); u1 = Math.max(u1, n.u1); }
        }
        if (!ok && u1 > u0) ok = f.cen[0] >= u0 - K.balanceTol && f.cen[0] <= u1 + K.balanceTol && u1 - u0 >= K.standFoot * (f.uMax - f.uMin);   // it stands
        if (!ok) ok = grip > 0 && f.area <= K.hangArea * grip;   // it hangs
        if (ok) { held.add(f.fid); changed = true; }
      }
    }
    return held;
  }
  private supportPass(st: PaneState) {
    if (st.type !== 'annealed' || st.nOpen === 0) return;
    const held = this.heldSet(st); let n = 0;
    for (const f of this.fragsOf(st)) {
      if (st.open[f.fid] || held.has(f.fid) || st.pending.some(p => p.fid === f.fid)) continue;
      const d = Math.hypot(f.cen[0] - st.lastUV[0], f.cen[1] - st.lastUV[1]);
      st.pending.push({ fid: f.fid, at: this.g.time + d * GLASSBREAK.staggerPerM + GLASSBREAK.staggerJit * hash01(st.key * 31 + f.fid), cause: 'support', from: null, k: 1 }); n++;
    }
    if (n) { this.plog(st, `${n} pieces unheld`); st.pending.sort((a, b) => a.at - b.at || a.fid - b.fid); }
  }

  // ---------------------------------------------------------------- per frame
  update(_dt: number) {
    this.releasedThisFrame = 0;
    if (!this.shadow && (this.gusts.size || this.gustDirty)) { this.updateGusts(); this.gustDirty = this.gusts.size > 0; }
    if (!glassBreakLive(this.g)) return;   // (as impact / blast: a pane still shedding when the tour starts, the switch goes off or chaos drops to 0 sheds nothing more and files no noise)
    const now = this.g.time;
    if (this.panes.size === 0) return;
    // (what is stepped below lets pieces go and queues others, and what hears one go asks lines of its own: the glass is worked on from here to there, for
    //  the box world's copy — boxMirror.ts glassBegin)
    const c = this.g.col; c.glassBegin();
    try {
      for (const st of this.panes.values()) {
        if (st.gone) continue;
        if (st.dice) { this.stepDice(st); continue; }
        if (st.pending.length) {   // what is due, within the frame's budget (the rest wait a frame); then the pass over what that left unheld
          let went = false;
          while (st.pending.length && st.pending[0].at <= now && this.releasedThisFrame < GLASSBREAK.releasesPerFrame) {
            const p = st.pending.shift()!; const f = this.fragsOf(st).find(x => x.fid === p.fid);
            if (f && !st.open[p.fid]) { if (f.b === 0) this.releaseCrater(st, f.i, p.cause); else this.releaseFrag(st, f, p.cause, p.from, p.k); this.releasedThisFrame++; went = true; }   // (a crater's wedges are crushed glass: they go together, as one fragment, whatever took the first)
          }
          if (went && st.nextThink <= 0) st.nextThink = now + GLASSBREAK.think;
        }
        if (st.nextThink > 0 && now >= st.nextThink) { st.nextThink = 0; GB_TWIN.thinks++; this.supportPass(st); }
        if (st.run && !st.walkable && GLASSBREAK.rakeOpen > 0 && st.type === 'annealed') this.rake(st);
      }
    } finally { c.glassEnd(); }
  }
  /** RAKING: Sam, on his feet, WALKS INTO what is left of a run pane that is mostly hole — the pieces in his way (not the teeth, nothing wholly above walkClearH) go,
   *  away from him, nearest first; the support pass takes what that leaves hanging. Asked only of panes that have run and are not yet walkable. Sam alone: nobody
   *  else walks at glass (the nav grid has every pane as a wall), and a man filing PAST a partition must not take it down. */
  private rake(st: PaneState) { for (const pl of men(this.g)) this.rakeBy(st, pl); }   // (any player's man: seats.ts)
  private rakeBy(st: PaneState, pl: Player) {
    const g = this.g; const K = GLASSBREAK;
    if (!pl || pl.down || pl.holding || pl.dragging || pl.wall) return;
    // cheap gates first — this is asked every frame of every run pane: within rakeM of the sheet, inside its width, and his legs are taking him AT it, more at it
    // than along it (the commanded velocity — the pane itself has stopped his body; a shallow pass along the glass is not a walk into it)
    const c = pl.char; const n = this.normalOf(st); const w = v3.dot(v3.sub(c.pos, st.box.c), n);
    if (Math.abs(w) > K.rakeM || c.pos[1] >= st.box.c[1] + st.hv) return;
    const [mu] = this.toUV(st, c.pos); if (Math.abs(mu) > st.hu + 0.1) return;
    const vn = -v3.dot(c.vel, n) * Math.sign(w || 1); const sp = Math.hypot(c.vel[0], c.vel[2]);
    if (vn <= 0.3 || vn < 0.6 * sp) return;
    if (st.pending.some(p => p.cause === 'rake') || this.openFraction(st.bi) < K.rakeOpen) return;
    const vClear = K.walkClearH - st.box.c[1]; const from: Vec3 = [c.pos[0], 1.0, c.pos[2]]; let took = 0;
    const waiting = new Set(st.pending.map(p => p.fid));
    for (const f of this.fragsOf(st)) {
      if (st.open[f.fid] || f.vMin >= vClear || waiting.has(f.fid)) continue;
      if (f.area < K.toothArea && f.frame.bottom + f.frame.left + f.frame.right + f.frame.top > 1e-4) continue;   // teeth: shouldered through, as the walk rule has it
      st.pending.push({ fid: f.fid, at: g.time + 0.04 + 0.12 * Math.abs(f.cen[0] - mu) + 0.05 * hash01(st.key * 71 + f.fid), cause: 'rake', from, k: 1 }); took++;
    }
    if (took) { st.lastWho = c.id; st.pending.sort((a, b) => a.at - b.at || a.fid - b.fid); GB_TWIN.raked += took; this.plog(st, `raked: ${took} pieces`); this.note(`pane ${st.bi}: ${took} pieces raked out`); }
  }
  /** the movement rule: every piece reaching below walkClearH above the floor has fallen — but for teeth (pieces under toothArea still in the frame) */
  private walkableNow(st: PaneState): boolean {
    if (st.gone) return true;
    if (st.dice || st.type !== 'annealed' || !st.run) return false;   // (an unrun pane still has its uncut sheet; a dicing one is walkable when it is gone)
    const vClear = GLASSBREAK.walkClearH - st.box.c[1];
    for (const f of this.fragsOf(st)) if (f.vMin < vClear && !st.open[f.fid] && !(f.area < GLASSBREAK.toothArea && f.frame.bottom + f.frame.left + f.frame.right + f.frame.top > 1e-4)) return false;   // (teeth left in the frame do not stop a man: he shoulders through them)
    return true;
  }
  gustDirty = false;
  /** GUSTS through shot-out exterior panes: one wind source per pane more than a quarter open (a jet along the inward normal like an open door's, wind.ts 'glass'), its speed each
   *  frame = the storm at that face × the open share — the particles (papers by the window) and the smoke solver read the one field */
  private gusts = new Map<number, WindSource>();
  /** panes that have already sounded / filed their hearing event this frame (emit) */
  private updateGusts() {
    const g = this.g; const swv = g.wind.stormWind();   // wind.ts's ONE storm test (weather on, night, WIND.storm, the storm or rain actually running): a dead calm or the daylight breach blows nothing through a hole
    const sw = swv ? [swv[0], swv[2]] : null;
    const stormSpeed = sw ? Math.hypot(sw[0], sw[1]) : 0;
    for (const st of this.panes.values()) {
      if (!st.exterior) continue;
      const open = this.openFraction(st.bi);
      let src = this.gusts.get(st.bi);
      if (open < 0.25 || stormSpeed < 0.05) { if (src) src.base = 0; continue; }
      if (!src) {
        const n = this.normalOf(st); const c = st.box.c;
        const inward: Vec3 = this.skySide(st.bi) > 0 ? v3.scale(n, -1) : n;   // away from the face the sky is over (the level's own geometry, as exterior itself)
        const reach = WIND.doorReach, dMax = WIND_FIELD.cutK * reach, tanSpread = Math.tan(WIND.doorSpreadDeg * Math.PI / 180), r0 = st.hu * 0.7;
        const pos: Vec3 = [c[0] - inward[0] * 0.15, c[1], c[2] - inward[2] * 0.15];
        src = { kind: 'jet', why: 'glass', name: `glass:${st.bi}`, pos, dir: inward, speed: 0, base: 0, on: 1, gust: 1, reach, tanSpread, r0, dMax, bound: { c: v3.mad(pos, inward, dMax / 2), r: Math.hypot(dMax / 2, r0 + dMax * tanSpread) + 0.3 }, seed: (0x61A55 ^ Math.imul(st.key, 2654435761)) >>> 0 } as WindSource;
        g.wind.all.push(src); this.gusts.set(st.bi, src); this.note(`gust source through pane ${st.bi}`);
      }
      const facing = Math.max(0, (sw![0] * src.dir[0] + sw![1] * src.dir[2]) / Math.max(1e-3, stormSpeed));
      src.base = stormSpeed * WIND.doorK * open * (0.35 + 0.65 * facing) * GLASSBREAK.gustSpeedK / 0.6;
    }
  }

  // ---------------------------------------------------------------- queries
  private openUV(st: PaneState, u: number, v: number): boolean {
    if (st.gone) return true;
    if (st.dice) return st.dice.out[gpDiceClump(st.dice, u, v)] === 1;
    if (st.nOpen === 0) return false;
    const fid = gpClassify(st.sites, st.run, u, v);
    return fid >= 0 && st.open[fid] === 1;
  }
  /** is world point X (on / near pane bi) where a piece has fallen out? (StaticCollision's raycast / segmentBlocked ask this to let rounds, sound and a step test through the
   *  hole; the same rule common.wgsl glassCellOpen runs on the table fill() packs) */
  openAt(bi: number, X: Vec3): boolean { const st = this.panes.get(bi); if (!st) return false; const [u, v] = this.toUV(st, X); return this.openUV(st, u, v); }
  /** the movement rule's answer for pane bi (false for a pane never hit) */
  walkable(bi: number): boolean { return this.panes.get(bi)?.walkable === true; }
  /** a number that changes whenever what is OPEN in any pane changes — a piece out, a clump down, a sheet gone, every pane whole again (glasspen.ts glassTollSig) */
  openSig(): number { let s = this.panes.size; for (const st of this.panes.values()) s = (Math.imul(s, 31) + st.bi * 7 + st.nOpen + (st.dice ? st.dice.next : 0) + (st.gone ? 4096 : 0)) | 0; return s; }
  /** open fraction of pane bi (0 = whole) — by area */
  openFraction(bi: number): number {
    const st = this.panes.get(bi); if (!st) return 0; if (st.gone) return 1;
    if (st.dice) return st.dice.next / st.dice.clumps.length;
    if (st.nOpen === 0) return 0;
    let a = 0; for (const f of this.fragsOf(st)) if (st.open[f.fid]) a += f.area;
    return Math.min(1, a / (4 * st.hu * st.hv));
  }
  /** what a sibling needs to know about pane bi without touching it (works for a pane never hit); null = not a sheet of glass */
  paneInfo(bi: number): { type: GlassType; thickness: number; exterior: boolean; openFraction: number; walkable: boolean; gone: boolean; run: boolean; sites: number; energy: number } | null {
    const type = this.typeOfPane(bi); if (!type) return null; const b = this.g.level.boxes[bi]; const st = this.panes.get(bi);
    return { type, thickness: 2 * Math.min(b.h[0], b.h[2]), exterior: this.exteriorOf(bi), openFraction: this.openFraction(bi), walkable: this.walkable(bi), gone: !!st?.gone, run: !!st?.run, sites: st?.sites.length ?? 0, energy: st?.energy ?? 0 };
  }
  /** the record for the renderer (boxes.ts packGlass → common.wgsl gb*): GB_RECORD_ROWS vec4 rows —
   *    0: site count · flags (1 something is open, 2 run, 4 dicing, 8 gone, 16 web) · u axis (0 = local x, 1 = local z) · damage 0‥1
   *    1: hu · hv · type (0 annealed, 1 tempered, 2 laminated) · 0          2: the dicing blow's u, v · the open threshold (clumps whose key is under it are out) · grain
   *    3: dice cols · rows · seed · 0
   *    then per site 4 rows: (u, v, rot, rz) · (radials, seed, energy, merge) · the four rho · the open masks (bands 0 | 1 << 12, 2 | 3 << 12, 4 | 5 << 12, 6) */
  fill(bi: number, dst: Float32Array, off: number): boolean {
    const st = this.panes.get(bi); if (!st || (st.sites.length === 0 && !st.dice && !st.gone)) return false;
    const D = st.dice; const anyOpen = st.gone || st.nOpen > 0 || (!!D && D.next > 0);
    dst[off] = st.sites.length; dst[off + 1] = (anyOpen ? 1 : 0) | (st.run ? 2 : 0) | (D && !st.gone ? 4 : 0) | (st.gone ? 8 : 0) | (st.type === 'laminated' ? 16 : 0); dst[off + 2] = st.uAxis === 0 ? 0 : 1; dst[off + 3] = st.damage;
    dst[off + 4] = st.hu; dst[off + 5] = st.hv; dst[off + 6] = st.type === 'laminated' ? 2 : st.type === 'tempered' ? 1 : 0; dst[off + 7] = 0;
    if (D) { dst[off + 8] = D.u; dst[off + 9] = D.v; dst[off + 10] = D.next >= D.clumps.length ? 1e9 : D.next === 0 ? -1 : (D.clumps[D.next - 1].key + D.clumps[D.next].key) / 2; dst[off + 11] = GLASSBREAK.diceGrain; dst[off + 12] = D.cols; dst[off + 13] = D.rows; dst[off + 14] = D.seed; }
    for (let i = 0; i < st.sites.length; i++) {
      const S = st.sites[i]; const o = off + (GB_HEADER_ROWS + i * GB_SITE_ROWS) * 4; const m = [0, 0, 0, 0, 0, 0, 0];
      for (let b = 0; b < GP_BANDS; b++) for (let s = 0; s < S.n; s++) if (st.open[i * GP_FRAGS_PER_SITE + b * GP_MAX_SECTORS + s]) m[b] |= 1 << s;
      dst[o] = S.u; dst[o + 1] = S.v; dst[o + 2] = S.rot; dst[o + 3] = S.rz;
      dst[o + 4] = S.n; dst[o + 5] = S.seed; dst[o + 6] = Math.min(8, S.e); dst[o + 7] = S.merge;
      dst[o + 8] = S.rho[0]; dst[o + 9] = S.rho[1]; dst[o + 10] = S.rho[2]; dst[o + 11] = S.rho[3];
      dst[o + 12] = m[0] | (m[1] << 12); dst[o + 13] = m[2] | (m[3] << 12); dst[o + 14] = m[4] | (m[5] << 12); dst[o + 15] = m[6];
    }
    return true;
  }
  /** the panel's 'break test': three rounds' worth on the pane on Sam's line to the cursor, through glass.ts's path exactly as rounds would arrive — returns the pane or −1 */
  breakTest(): number {
    const g = this.g; const aim = g.aimPoint; const from: Vec3 = v3.add(g.player.char.pos, [0, 1.3, 0]);
    const d = v3.sub(aim, from); const L = v3.len(d); if (L < 1e-3) return -1;
    const rd = v3.scale(d, 1 / L);
    // (the first round opens the crater, so the next two would fly through it: walk them a hand's width along the pane instead — a burst)
    for (let k = 0, bi = -1; k < 3; k++) {
      const o: Vec3 = v3.add(from, [0, 0.07 * k, 0]); const h = g.col.raycast(o, rd, L + 6);
      if (!h || h.index >= 1e6) return bi;
      const b = g.level.boxes[h.index]; if (!b || !(b.flags & BoxFlag.Glass) || Math.min(b.h[0], b.h[2]) > 0.05) return bi;
      bi = h.index; glassImpact(g, bi, v3.mad(o, rd, h.t), rd, 1, 0);
      if (k === 2) return bi;
    }
    return -1;
  }
  summary(): string {
    const T = this.g.tune as GbTune;
    let s = `glass breaking ${T.glassBreak ? (glassBreakLive(this.g) ? 'ON' : 'on — but inert (chaos 0 / the tour)') : 'off'} ▸ ${this.panes.size} panes touched · ${GB_TWIN.impacts} impacts · ${GB_TWIN.sites} stars · ${GB_TWIN.runs} run · ${GB_TWIN.dropped} fragments out (${GB_TWIN.clumps} clumps) · diced ${GB_TWIN.diced}${this.onRelease ? ` · ${GB_TWIN.hooked} to the shards hook` : ` · stand-in: ${GB_TWIN.shards} shards (${GB_TWIN.refused} refused), ${GB_TWIN.chips} chips`}`;
    for (const st of this.panes.values()) if (st.nOpen || st.dice || st.gone) s += ` | pane ${st.bi} ${st.type} ${Math.round(100 * this.openFraction(st.bi))}% open${st.run ? ' run' : ''}${st.walkable ? ' (walkable)' : ''}`;
    return s;
  }
  reset() { this.panes.clear(); this.g.col.wrote(); this.g.col.glassWrote(); this.blasted.length = 0; for (const src of this.gusts.values()) { const i = this.g.wind.all.indexOf(src); if (i >= 0) this.g.wind.all.splice(i, 1); } this.gusts.clear(); this.gustDirty = false; }
}
function nearestOn(b: Box, p: Vec3): Vec3 {
  const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); const px = p[0] - b.c[0], pz = p[2] - b.c[2];
  const lx = Math.max(-b.h[0], Math.min(b.h[0], cs * px - sn * pz)), lz = Math.max(-b.h[2], Math.min(b.h[2], sn * px + cs * pz)), ly = Math.max(-b.h[1], Math.min(b.h[1], p[1] - b.c[1]));
  return [b.c[0] + cs * lx + sn * lz, b.c[1] + ly, b.c[2] - sn * lx + cs * lz];
}
