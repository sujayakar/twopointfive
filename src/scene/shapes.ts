// A BOX SAYS ITS SHAPE. Asked 2026-11-04 from the library's showroom: "could we make it so the cars have rounded surfaces? look really blocky right now"; and
// before it, of the pass's lorry, "some curved surfaces on the truck would go a long way". A level is boxes, and until now a box of a level file could only be
// a box: what was curved on the look page was cut by code that went over one level after it loaded. Here the level file says it. A box may carry a SHAPE:
// how it stands when shapes are on.
//
// THE DATA NAMES THE SHAPE, never the engine's record (scene/levelData.ts BoxData.shape, scene/levelLoad.ts reads and writes it). Everything in it is said in
// the box's OWN frame (metres from its centre along its own axes), so a box that is moved, turned or copied takes its shape along:
//   round   1 | 3            the box is a twelve-sided prism about its own x or z (a wheel on its axle, a tank): the box is square across that axis, or
//                            within a tenth of it (the round's radius is then the smaller half, and the longer way keeps the box's own two faces)
//   arc     { axis, rise }   the box's top is a round about its own x or z that rises `rise` from its sides to its middle (a bowed roof, a bonnet, an arch)
//   cut     [nx, ny, nz, d]… planes, each keeping n · p ≤ d (a corner rounded in plan, a pane that leans): 8 at most, 1 beside an arc, none beside a round
//   old     true             not a fresh break: the cut faces take the box's own paint and lying snow (a pressed panel, not a snapped end). The PICTURE's
//                            word, and nothing reads it until the look's cut pass is in the build
//   with    [ piece… ]       pieces of TRIM that are there only when shapes are on (a hub, an arch over a wheel, a post between two panes): each an `off`
//                            from the box's centre, a half size, a yaw from the box's own, a paint and a form of the kinds above
// A SHAPE IS CUT FROM THE PLAIN BOX: the box keeps its centre and its half size. So the shaped level, to the simulation, is the plain level with some of
// its boxes cut — never a thing standing where none stood.
// A PIECE IS TRIM: drawn, and nothing else. It stands NoShadow, the flag of every drawn-only static, so no ray, eye or body meets it and it casts no
// shadow; what must stop a round or a man is a box of the level, which may say a shape of its own. (A piece says no flags.) It knows the box that says
// it (Box.of) and follows it out of the picture: withPieces, asked where a level box is hidden by its number (a thing a round takes: game/destruction.ts;
// a kit view's hidden boxes: host/kitView.ts). It is never a thing of its own to the destruction lane's items.
// A whole cylinder standing on its end is not said here: that is a ROUND THING (scene/round.ts), the one way to say it.
//
// ONE MODEL: the record is built by scene/cut.ts cutRecord, so a shaped box is a cut box to everybody — the picture draws the solid, every GPU ray and every
// CPU ray and segment meet it (sight, a round, a lamp's shadow, sound, a throw's arc; the game's own module too, whose copy of the boxes carries each record:
// scene/boxMirror.ts).
// WHAT WALKS MEETS THE PLAIN LEVEL, by construction and not by a rule its author must keep: a shaped box is pushed against as its UNCUT box (cut.ts's own
// rule), which is the plain box, and a piece is met by nobody. The rules that mark a level's boxes by their size as it is stood up (thin things, a window's
// bars: host/standup.ts) read the plain boxes too: shapes are given after them.
// WHAT AN ENGINE CUTS UP ITSELF SAYS NO FORM: a window's bar (game/winbars.ts breaks it into lengths) is left plain by shapeLevel, a slab of one of the kit's
// octagons is refused by the lint (scene/round.ts makes the four one round box), and tools/qa/shapes-check.ts is red on a level file whose shape does not
// stand.
// A BLAST: a box whose shape has a form is one cell to the wall engine and never a wall cut into cells (its cells would be cut from the whole box), and a
// piece is one of the things that hang on what breaks (wallbreak.ts ghosts, furnblast.ts riders: any authored NoShadow static) and goes with it.
// THE SWITCH IS THE SIMULATION'S, and a picture setting may not move it (as round.ts): with the cut-pieces switch down (scene/cut.ts cutConfig.on: a headless
// tool, `?nocutboxes`) the game's rays still meet a shape's record (boxes.ts cutMet), and only the picture and the GPU's rays fall back to its uncut box.
// THE PICTURE TODAY draws a round and an arc as the flat facets they are cut to and a cut face bare. That a round or an arc is shaded as ONE turning face and
// that an `old` cut takes the box's paint are the look's cut pass's words (not in this build yet): formBox below is where a shaped box will be told them.
//
// HOW: as a level is stood up (host/standup.ts populateHost, after its round things, before a box goes to the world and before the Game is built), with the
// switch up each box that says a shape is given its record (Box.shaped) and its pieces are added AFTER every box the level has (Box.piece), so no box changes
// its number; plainLevel takes the records and the pieces away. A level built by a generator in the page (the office) is shaped the same way as
// one read from a file: the shape rides on the Box.
// A LEVEL STOOD ON ITS SHAPES IS ANOTHER LEVEL: the level's fingerprint (game/savestate.ts) takes in a number of the shapes that STAND (shapeStamp: their
// geometry to the millimetre, 0 = none), so a checkpoint or a recording made on them is refused by the plain level, and the other way about, and by the level
// once its shapes have changed — by the one refusal a checkpoint and every tape player already have. With no shape standing the fingerprint is the one the
// level always had, so nothing recorded on the frozen stand moves.
//
// THE SWITCH: SHAPES.on — ON in the page (`?shapes=0` puts it off), OFF in the module and on the frozen stand (FROZEN_HOLDS; TPS_SHAPES=1 raises it there).
// Read ONCE, as a level is stood up, and the debugger's row reloads the page. Off = no box touched, none added: the build as it was by run digest.
// A recording carries the stand's word (host/standEnv.ts) and the table; one from before the table has the switch put down before its level stands
// (shapesTape: replay/recipestand.ts and tools/qa/rerun.ts `tables`; journal.ts applyKnobs says the same afterwards).
// CHECK: tools/qa/shapes-check.ts (SHAPES_BREAK=1 must fail), both stands. Nothing here has been seen on a GPU.
import { makeBox, BoxFlag, type Box } from './boxes';
import { cutPlane, cutRecord, cutToWorld, MAX_CUT_PLANES, type CutPlane } from './cut';
import type { Vec3 } from '../math/vec';

export const SHAPES = { on: false };
export const SHAPES_STATS = { shaped: 0, pieces: 0, levels: 0 };

/** one plane of a shape in the box's own frame: [nx, ny, nz, d] keeps n · p ≤ d (n need not be unit: d is measured along n as given) */
export type ShapePlane = [number, number, number, number];
/** the kinds of shape (see the head) */
export interface ShapeForm { round?: 1 | 3; arc?: { axis: 1 | 3; rise: number }; cut?: ShapePlane[]; old?: true }
/** a piece of trim that is there only when shapes are on: `off` = its centre from the box's, in the box's frame; `yaw` = from the box's own */
export interface ShapePiece extends ShapeForm { off: Vec3; h: Vec3; yaw?: number; albedo: Vec3; rough?: number; metal?: boolean }
/** how a box stands when shapes are on */
export interface BoxShape extends ShapeForm { with?: ShapePiece[] }
/** the most planes a shape may say: the record's rows, one fewer beside an arc (the picture's record keeps its last row for the arc) */
export const SHAPE_PLANES = MAX_CUT_PLANES, ARC_FACETS = 6;
/** the keys of a form, and of a shape */
export const FORM_KEYS = ['round', 'arc', 'cut', 'old'] as const, SHAPE_KEYS = [...FORM_KEYS, 'with'] as const;
/** a form's own copy (a level and its data never share an array) */
export const formOf = (f: ShapeForm): ShapeForm => ({ ...(f.round ? { round: f.round } : {}), ...(f.arc ? { arc: { axis: f.arc.axis, rise: f.arc.rise } } : {}), ...(f.cut ? { cut: f.cut.map((q): ShapePlane => [q[0], q[1], q[2], q[3]]) } : {}), ...(f.old ? { old: true as const } : {}) });

const D = Math.PI / 180;
/** the eight planes that, with the box's own faces, make a box a twelve-sided prism of radius r about its own x (1) or z (3) */
const ringPlanes = (r: number, axis: 1 | 3): CutPlane[] => [30, 60, 120, 150, 210, 240, 300, 330].map(d => cutPlane(axis === 1 ? [0, Math.sin(d * D), Math.cos(d * D)] : [Math.cos(d * D), Math.sin(d * D), 0], r));
/** an arc's six facets over a box of half size h (three a side of the top): the round rises `rise` over the chord of the box's half width w, so its radius
 *  is (w² + rise²) ÷ 2 rise and its axis lies that far under the box's top */
function arcPlanes(h: Vec3, axis: 1 | 3, rise: number): CutPlane[] {
  const w = axis === 1 ? h[2] : h[0], R = (w * w + rise * rise) / (2 * rise), k = R - h[1], step = Math.asin(Math.min(1, w / R)) / 3.5;
  return [-3, -2, -1, 1, 2, 3].map(i => { const a = i * step; return cutPlane(axis === 1 ? [0, Math.cos(a), Math.sin(a)] : [Math.sin(a), Math.cos(a), 0], R - k * Math.cos(a)); });
}
/** a form's planes over a box of half size h */
function planesOf(h: Vec3, f: ShapeForm): CutPlane[] {
  const more = (f.cut ?? []).map(p => cutPlane([p[0], p[1], p[2]], p[3]));
  if (f.round) return ringPlanes(Math.min(h[1], f.round === 1 ? h[2] : h[0]), f.round);
  if (f.arc) return [...arcPlanes(h, f.arc.axis, f.arc.rise), ...more.slice(0, SHAPE_PLANES - 1 - ARC_FACETS)];
  return more;
}

/** what is wrong with a shape said of a box of half size h (the level lint, scene/levelData.ts): nothing = []. `pieceKeys` = the keys a piece may carry
 *  beside its form (its place, size, turn, paint and finish as a box says them: the lint's own list, so a piece is painted by the one reader a box is).
 *  `piece` = this IS a piece of that box. */
export function shapeErrors(s: unknown, h: Vec3, pieceKeys: readonly string[], piece = false): string[] {
  const out: string[] = [], isV = (v: unknown): v is Vec3 => Array.isArray(v) && v.length === 3 && v.every(x => typeof x === 'number' && Number.isFinite(x));
  if (s === null || typeof s !== 'object' || Array.isArray(s)) return ['must be an object'];
  const o = s as Record<string, unknown>, keys: readonly string[] = piece ? [...pieceKeys, ...FORM_KEYS] : SHAPE_KEYS;
  for (const k of Object.keys(o)) if (!keys.includes(k)) out.push(`unknown key '${k}'${piece && k === 'flags' ? ' (a piece is trim: it always stands NoShadow)' : ''}`);
  if (piece && !(isV(o.off) && isV(o.h) && o.h.every(x => x > 0))) { out.push('a piece needs off and h > 0'); return out; }
  if (piece && o.yaw !== undefined && !(typeof o.yaw === 'number' && Number.isFinite(o.yaw))) { out.push('yaw must be a number'); return out; }
  const hh = piece ? o.h as Vec3 : h;   // (the box a form is cut from: the piece's own, or the plain box — a shaped box keeps its centre and half size)
  if (o.old !== undefined && o.old !== true) out.push('old is true or absent');
  if (o.round !== undefined && o.arc !== undefined) out.push('round and arc: one or the other');
  const planes = o.cut; let form = true;
  if (planes !== undefined && !(Array.isArray(planes) && planes.length > 0 && planes.every(p => Array.isArray(p) && p.length === 4 && p.every(x => typeof x === 'number' && Number.isFinite(x)) && Math.hypot(p[0], p[1], p[2]) > 1e-9))) { out.push('cut must be planes [nx, ny, nz, d] with a normal'); form = false; }
  const nCut = Array.isArray(planes) ? planes.length : 0;
  if (o.round !== undefined) {
    if (o.round !== 1 && o.round !== 3) { out.push('round is the axis 1 (x) or 3 (z); a cylinder on its end is a round thing of the kit'); form = false; }
    else { const a = o.round === 1 ? hh[2] : hh[0]; if (Math.abs(hh[1] - a) > 0.1 * Math.min(hh[1], a)) out.push(`round ${o.round}: the box must be square across that axis, or within a tenth of it (${hh[1]} by ${a})`); }
    if (nCut) out.push('round takes every plane of the record: no cut beside it');
  } else if (o.arc !== undefined) {
    const a = o.arc as { axis?: unknown; rise?: unknown };
    if (a === null || typeof a !== 'object' || (a.axis !== 1 && a.axis !== 3) || typeof a.rise !== 'number' || !(a.rise > 0)) { out.push('arc must be { axis: 1 | 3, rise > 0 }'); form = false; }
    else { const w = a.axis === 1 ? hh[2] : hh[0]; if (a.rise > 2 * hh[1] + 1e-6) out.push(`arc: a rise of ${a.rise} in a box ${2 * hh[1]} high`); if (a.rise > w + 1e-6) out.push(`arc: a rise of ${a.rise} over a half width of ${w} (a half round is the most an arc can be)`); }
    if (nCut > SHAPE_PLANES - 1 - ARC_FACETS) out.push(`an arc leaves room for ${SHAPE_PLANES - 1 - ARC_FACETS} plane (${nCut})`);
  } else if (nCut > SHAPE_PLANES) out.push(`at most ${SHAPE_PLANES} planes (${nCut})`);
  // the record itself (cut.ts cutRecord): a plane of the author's that takes the whole box or none of it (a slip of a sign says nothing and stands the plain
  // box), and a form that leaves nothing. (Each plane is judged against the box alone: whether it still bites beside an arc's own facets is not asked.)
  if (form && !out.length) {
    (planes as ShapePlane[] | undefined)?.forEach((q, i) => { const r = cutRecord(hh, [cutPlane([q[0], q[1], q[2]], q[3])]);
      if (r.empty) out.push(`cut[${i}] leaves nothing of the box`); else if (!r.cut?.length) out.push(`cut[${i}] cuts nothing of the box`); });
    if (!out.length && (nCut || o.arc !== undefined || o.round !== undefined) && cutRecord(hh, planesOf(hh, o as ShapeForm)).empty) out.push('the form leaves nothing of the box');
  }
  if (!piece && o.with !== undefined) {
    if (!Array.isArray(o.with) || !o.with.length) out.push('with must be a list of pieces');
    else o.with.forEach((w, i) => { for (const e of shapeErrors(w, h, pieceKeys, true)) out.push(`with[${i}]: ${e}`); });
  }
  if (!piece && o.round === undefined && o.arc === undefined && !nCut && o.with === undefined) out.push('says nothing');
  return out;
}

/** give `b` a shape's form: its record. (THE PICTURE'S WORDS go here too once the look's cut pass is in the build — a round or an arc as one turning face
 *  about its axis, an old cut in the box's paint: the head.) */
function formBox(b: Box, f: ShapeForm): void {
  const planes = planesOf(b.h, f);
  if (planes.length) b.cut = cutRecord(b.h, planes).cut;
}
/** A NUMBER OF THE SHAPES A LEVEL STANDS ON (the level's fingerprint takes it in: game/savestate.ts), over its first `n` boxes: FNV-1a over each shaped
 *  box's place in the list and the GEOMETRY of its shape to the millimetre — its form, and each piece's place, size, turn and form (a piece has a number in
 *  the list and a blast reads where it hangs). Never a paint, a finish or the picture's `old`: a repaint is not another level. 0 = no box stands shaped. */
export function shapeStamp(boxes: readonly Box[], n = boxes.length): number {
  let h = 0x811c9dc5, any = false;
  const num = (v: number) => { h ^= Math.round(v * 1000) & 0xffffffff; h = Math.imul(h, 0x01000193) >>> 0; };
  const form = (f: ShapeForm) => { num(f.round ?? 0); num(f.arc ? f.arc.axis : 0); num(f.arc ? f.arc.rise : 0); num(f.cut ? f.cut.length : 0); for (const q of f.cut ?? []) for (const v of q) num(v); };
  for (let i = 0; i < n && i < boxes.length; i++) {
    const b = boxes[i]; if (b.shaped !== true) continue; const s = b.shape!; any = true;
    num(i / 1000); form(s); num(s.with ? s.with.length : 0);
    for (const w of s.with ?? []) { for (const v of w.off) num(v); for (const v of w.h) num(v); num(w.yaw ?? 0); form(w); }
  }
  return any ? (h || 1) : 0;
}

/** A LEVEL AS ITS FILE HAS IT: every record a stand-up gave is taken back and the pieces added then are taken away. Idempotent. */
export function plainLevel(boxes: Box[]): void {
  { let w = 0; for (let i = 0; i < boxes.length; i++) if (boxes[i].piece !== true) boxes[w++] = boxes[i]; boxes.length = w; }   // (the pieces are the tail of a level as stood; wherever one is, it goes)
  for (const b of boxes) if (b.shaped === true) { b.cut = undefined; b.shaped = undefined; }   // (set, never deleted: a box keeps its one layout — boxes.ts boxOf)
}
/** A LEVEL'S SHAPES, as it is stood up: the level is put back as its file has it (plainLevel) and, with `on`, each box that says a shape is given its record
 *  and its pieces follow the level's last box. Never a tilted box, a box cut already, a round thing's slab (scene/round.ts keeps those) or a window's bar
 *  (the head). Returns how many boxes are shaped now. */
export function shapeLevel(boxes: Box[], on: boolean): number {
  plainLevel(boxes);
  let made = 0; const tail: Box[] = [];
  if (on) boxes.forEach((b, i) => {
    const s = b.shape; if (!s || b.rot || b.was || b.cut !== undefined || (b.flags & BoxFlag.Bar)) return;
    formBox(b, s); b.shaped = true;
    for (const w of s.with ?? []) {
      const o = cutToWorld(w.off, b.yaw);   // (the box's own frame: cut.ts)
      const p = makeBox({ c: [b.c[0] + o[0], b.c[1] + o[1], b.c[2] + o[2]], h: [w.h[0], w.h[1], w.h[2]], yaw: b.yaw + (w.yaw ?? 0), albedo: [w.albedo[0], w.albedo[1], w.albedo[2]], flags: BoxFlag.NoShadow | (b.flags & BoxFlag.Cutaway), ...(w.rough !== undefined ? { rough: w.rough } : {}), ...(w.metal ? { metal: true } : {}), cutHeight: b.cutHeight, piece: true, of: i });   // (cut away with its box, where the box is)
      formBox(p, w); tail.push(p);
    }
    made++;
  });
  for (const p of tail) boxes.push(p);
  SHAPES_STATS.shaped = made; SHAPES_STATS.pieces = tail.length; SHAPES_STATS.levels++;
  return made;
}
/** the numbers `idx` of a stood level's boxes, and after them the numbers of the pieces those boxes say: what goes out of the picture with them */
export function withPieces(boxes: readonly Box[], idx: readonly number[]): number[] {
  const out = [...idx]; let first = boxes.length; for (let i = 0; i < boxes.length; i++) if (boxes[i].piece === true) { first = i; break; }
  if (first < boxes.length) { const own = new Set(idx); for (let i = first; i < boxes.length; i++) { const b = boxes[i]; if (b.piece === true && b.of !== undefined && own.has(b.of)) out.push(i); } }
  return out;
}

/** A RECORDING'S TABLES, before its level is stood up (replay/recipestand.ts and tools/qa/rerun.ts `tables`): one from before this table is played on a floor
 *  of plain boxes, as it was recorded — journal.ts applyKnobs says the same afterwards, too late for a level already standing */
export function shapesTape(tables: Record<string, unknown>) {
  const t = tables.SHAPES; if (t === null || typeof t !== 'object') SHAPES.on = false;
}
