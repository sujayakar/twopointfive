// WHAT IS PAINTED ON A WALL GOES WITH THE WALL (src/game/wallskin.ts, WALLSKIN) — a rule of the picture only.
// A level's walls carry thin drawn-only boxes on their faces (a skirt, a dado, a band, a mend). When the wall opened, a band from the floor stayed where it was
// authored: paint in the air across the foot of the hole. With the rule a sheet is drawn only where something solid still stands behind it.
//
//   bun run tools/qa/wallskin-check.ts          (probes.ts wallskin; WALLSKIN_BREAK=1 must FAIL — the rule is left down where the check raises it)
//   TPS_PROFILE=shipped …                       the same on the stand the page opens on
// Both stands: the check gives the stand its word (TPS_WALLSKIN) itself. THE CHECK BUILDS ITS OWN WALLS' PAINT: on the office it takes the longest partition
// with room to work and a wall a charge can take on, and lays on them, in the level's data before the floor stands, a dado and a skirt from the floor, a
// stripe off it, a mend over the dado, a band that carries markings, a bead thin both ways, trim the dado says, a run of conduit that rests on its own
// branch (with its clips), a card board and the paper pinned to it, a sheet 4 cm off the wall with nothing between, a sheet the hangers' own rule throws,
// and five things that are NOT this rule's (a duct with a body, a sheet 12 cm off the wall, a rug under the wall, a strip lying flat at its foot, a band
// on a wall nothing opens); and, where the level has two partitions in one line, one band along both. Then it opens the walls every way a wall opens (a
// blast, a second beside it, rounds for a port and then the sill under it, a checkpoint's load, a tick put back, a charge on the hard wall) on two
// floors, the rule up and down, and asks after each:
//   THE PICTURE, by an oracle that knows nothing of the rule's rectangles or of what it thinks a sheet lies on: THE LEVEL'S OWN COLLISION. Every sheet's
//   face is sampled every 23.7 mm; a point is DRAWN if the world's own list of what it rasterises after its upload (BoxWorld.visible: the statics by
//   number, the frame's dynamic boxes by where they are) covers it; what it LAY ON is the first solid surface a short ray straight in from in front of
//   the sheet met as the floor stood, no further than WALLSKIN.off behind it, and it has lost its wall when no solid surface stands at that depth now.
//   With the rule up no point that has lost its wall is drawn, and none that has not is lost; with it down the band hangs across the hole (the fault,
//   shown to be there).
//   PLAY: the two floors are one run — the same digest, the same cells, the same pieces of debris in the same places, the lane's hash counter where it
//   was, and NOT ONE FLAG of any authored box differs between them (the rule writes none).
// THEN THREE LEVELS' OWN PAINT, by the same oracle (lines 6): a corner where two partitions overlap, one's end column opened (the band on the one that
// stands is whole, and so is the opened one's band over the other's end); a decal on a piece of cover, the cover razed; and paint on a piece of cover
// opened in part, whose standing piece ends ragged under the cut switch.
// Nothing here is a picture: what a cut band looks like has never been seen on a GPU by this check.
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { WALLSKIN, standing } = await import(`${ROOT}/src/game/wallskin.ts`);
const { WALLS, WALL_STATS } = await import(`${ROOT}/src/game/wallbreak.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { cutBoxesOn } = await import(`${ROOT}/src/scene/cut.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { BoxFlag, MKEY, makeBox } = await import(`${ROOT}/src/scene/boxes.ts`);
const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
const { thingKeyOf } = await import(`${ROOT}/src/game/fogofwar.ts`);
const { takeSnapshot, restoreSnapshot } = await import(`${ROOT}/src/game/rollback.ts`);
const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
const J = await import(`${ROOT}/src/game/journal.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60, BREAK = process.env.WALLSKIN_BREAK === '1', VERBOSE = !!process.env.WALLSKIN_VERBOSE, FILE = 'office.json', STEP = 0.0237;
let grain = STEP;   // (how far apart the oracle's points are: the check's own STEP; a sweep of whole levels asks three times coarser)
function seeded(seed: number) { let a = seed >>> 0; return () => { a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
async function hushed<T>(f: () => T | Promise<T>): Promise<T> { const log = console.log, warn = console.warn; if (!VERBOSE) console.log = console.warn = () => { }; try { return await f(); } finally { console.log = log; console.warn = warn; } }

/** a wall the check works on: its number, box, the way it runs (0 = x, 2 = z) and its columns */
interface Wall { id: number; box: any; u: 0 | 2; N: number; hard: boolean }
/** one thing laid on a wall for the check: its place in the level's list once the floor stands, and what it is */
interface Laid { name: string; walls: Wall[]; box: any; i: number; skin: boolean; /** how far along its ray each sampled point met the surface it lay on as the floor stood (Infinity: on nothing; -1: something solid stands in front of the sheet there, the point is not seen and not asked) */ base?: number[];
  /** a level's own sheet (no wall of the check's): the way it runs and the side its front faces */ u?: 0 | 2; sg?: 1 | -1 }
type Plan = { part: Wall; hard: Wall; far: Wall | null; /** two partitions in one line, a gap between them (a doorway), or none on this level */ pair: [Wall, Wall] | null };
let stood: boolean | null = null;   // (the switch as a stand with no word of the check's left it, read once)
let plan: Plan | null = null;
const laidAt = new Map<string, number>();

const levelOf = (file = FILE) => loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${file}`, 'utf8')));
const colsOf = (b: any, hard: boolean): number => { const L = 2 * Math.max(b.h[0], b.h[2]); return hard ? Math.min(32, Math.max(1, Math.floor(L / WALLS.col + 1e-6))) : Math.min(32, Math.max(1, Math.round(L / WALLS.col))); };
/** a point on a wall: `u` m along it from its middle, `off` m off its + face (negative: off its − face), `y` up */
const onWall = (w: Wall, u: number, off: number, y: number): Vec3 => { const p: Vec3 = [w.box.c[0], y, w.box.c[2]], n = (2 - w.u) as 0 | 2; p[w.u] += u; p[n] += off >= 0 ? w.box.h[n] + off : -w.box.h[n] + off; return p; };
/** a drawn-only box on a wall's face: u0‥u1 along it, y0‥y1 up, `t` thick, its near side `gap` off the face (side ±1) */
function sheet(w: Wall, side: 1 | -1, u0: number, u1: number, y0: number, y1: number, t = 0.008, gap = 0, more: object = {}) {
  const c = onWall(w, (u0 + u1) / 2, side * (gap + t / 2), (y0 + y1) / 2), h: Vec3 = [0, (y1 - y0) / 2, 0]; h[w.u] = (u1 - u0) / 2; h[2 - w.u] = t / 2;
  return makeBox({ c, h, albedo: [0.2, 0.3, 0.5], flags: BoxFlag.NoShadow, ...more });
}
/** the office with the check's paint laid on its walls (the same boxes on every floor the check stands: appended after the level's own) */
function dressed(P: Plan): { lvl: any; laid: Laid[] } {
  const lvl = levelOf(), laid: Laid[] = [], w = P.part, L = Math.max(w.box.h[0], w.box.h[2]);
  const lay = (name: string, wall: Wall | Wall[], b: any, skin = true) => { b.albedo = [0.2, 0.3, 0.4 + 0.03 * laid.length]; laid.push({ name, walls: Array.isArray(wall) ? wall : [wall], box: b, i: -1, skin }); lvl.boxes.push(b); return b; };   // (each its own paint: the oracle knows a thing's boxes by it)
  const dado = lay('a dado from the floor', w, sheet(w, 1, -L, L, 0, 1.1));
  lay('a skirt from the floor on the other face', w, sheet(w, -1, -L, L, 0, 0.38));
  lay('a stripe off the floor', w, sheet(w, 1, -L, L, 1.3, 1.5));
  lay('a mend over the dado', w, sheet(w, 1, 0.4, 1.3, 0.3, 0.8, 0.006, 0.008));
  { const top = w.box.c[1] + w.box.h[1], y = Math.min(top - 0.2, 2.25);   // a run of conduit under the wall's head on the − face, a branch down from its end (the run RESTS on it: the hangers pass it by), a clip every metre and a half
    lay('a run of conduit that rests on its branch', w, sheet(w, -1, -2, 2, y, y + 0.03, 0.03));
    lay("the conduit's branch", w, sheet(w, -1, 1.97, 2, y - 0.4, y, 0.03));
    for (const u of [-1.6, -0.1, 1.4]) lay(`a clip of the conduit at ${u}`, w, sheet(w, -1, u - 0.03, u + 0.03, y - 0.02, y + 0.05, 0.04), false);
    const y1 = Math.min(top - 0.3, 2.15);   // a card board (3 cm, a body the hangers leave up while most of its wall stands) and the paper pinned to it
    lay('a card board', w, sheet(w, 1, -L + 0.02, -L + 1.48, 1.55, y1, 0.03));
    lay('the paper pinned to the board', w, sheet(w, 1, -L + 0.3, -L + 1.3, 1.65, y1 - 0.1, 0.008, 0.03)); }
  lay('a sheet in the middle of the hole (the hangers throw it)', w, sheet(w, -1, -0.2, 0.2, 1.2, 1.7));
  lay('a bead thin both ways, from the floor', w, sheet(w, -1, -0.103, -0.097, 0, 1.0, 0.016));
  lay('a marked band from the floor', w, sheet(w, -1, -2.2, 2.2, 0, 0.3, 0.008, 0.008, { marks: [{ face: '+z', at: [-2, -1], size: [4, 2], pal: [{ color: [0.9, 0.9, 0.9] }], cells: ['aaaa', 'a..a'] }] }));
  lay('trim the dado says', w, sheet(w, 1, -L, L, 1.1, 1.14)).dadoOfCheck = dado;   // (made trim once the floor stands: a level's own list holds none)
  lay('a duct with a body (5 cm)', w, sheet(w, 1, -0.6, 0.6, 0, 0.1, 0.05, 0.02), false);
  lay('a sheet 4 cm off the wall, nothing between', w, sheet(w, -1, -L + 0.05, -L + 1.45, 1.75, 1.95, 0.008, 0.04));
  lay('a sheet 12 cm off the wall', w, sheet(w, -1, -L + 0.05, -L + 1.45, 1.3, 1.5, 0.008, 0.12), false);
  { const c = onWall(w, 0, 0, 0.005), h: Vec3 = [0.7, 0.005, 0.7]; c[2 - w.u] = w.box.c[2 - w.u]; lay('a rug under the wall', w, makeBox({ c, h, albedo: [0.5, 0.2, 0.2], flags: BoxFlag.NoShadow }), false); }
  lay("a strip lying flat at the wall's foot", w, sheet(w, 1, -1.5, 1.5, 0, 0.005, 0.02), false);
  { const h = P.hard, Lh = Math.max(h.box.h[0], h.box.h[2]); for (const side of [1, -1] as const) lay(`a band from the floor on the hard wall's ${side > 0 ? '+' : '−'} face`, h, sheet(h, side, -Lh, Lh, 0, 0.9)); }
  if (P.far) { const f = P.far, Lf = Math.max(f.box.h[0], f.box.h[2]); lay('a dado on a wall nothing opens', f, sheet(f, 1, -Lf, Lf, 0, 1.1), false); }
  if (P.pair) { const [a, b] = P.pair, u = a.u, lo = Math.min(a.box.c[u] - a.box.h[u], b.box.c[u] - b.box.h[u]), hi = Math.max(a.box.c[u] + a.box.h[u], b.box.c[u] + b.box.h[u]);
    lay('one band along two walls', [a, b], sheet(a, 1, lo - a.box.c[u], hi - a.box.c[u], 0, 0.9)); }
  return { lvl, laid };
}
async function stand(on: boolean, P: Plan | null, file = FILE): Promise<{ W: any; laid: Laid[] }> {
  Math.random = seeded(33);
  const word = process.env.TPS_WALLSKIN;
  try {
    if (stood === null) { await hushed(() => standUp({ level: levelOf(), seed: 1 })); stood = !!WALLSKIN.on; }
    process.env.TPS_WALLSKIN = on && !BREAK ? '1' : '0';
    const D = P ? dressed(P) : { lvl: levelOf(file), laid: [] };
    DESTRUCTION.on = true; const W: any = await hushed(() => standUp({ level: D.lvl, seed: 1 })); DESTRUCTION.on = true;
    for (const L of D.laid) { L.i = W.game.col.boxes.indexOf(L.box); if (L.i < 0) throw new Error(`wallskin-check: '${L.name}' is not among the floor's statics`); if (L.box.dadoOfCheck) { L.box.piece = true; L.box.of = W.game.col.boxes.indexOf(L.box.dadoOfCheck); delete L.box.dadoOfCheck; } }
    W.game.tune.chaos = 0.5; W.game.godMode = true; for (let i = 0; i < 20; i++) W.step(DT);
    return { W, laid: D.laid };
  } finally { if (word === undefined) delete process.env.TPS_WALLSKIN; else process.env.TPS_WALLSKIN = word; }
}
/** the walls the check works on, found on a bare floor: the longest partition whose middle a blast reaches with nothing in the way on either face, a wall
 *  a charge can take on (a hard wall), and a partition far from both that nothing here opens */
function choose(g: any): Plan | null {
  const walls = g.destr.walls, boxes = g.col.boxes, nav = g.col.nav, free = (p: Vec3) => !nav.blocked[nav.idx(Math.floor(p[0] / nav.cell), Math.floor(p[2] / nav.cell))];
  const mk = (id: number, hard: boolean): Wall => { const b = boxes[id]; return { id, box: b, u: b.h[0] >= b.h[2] ? 0 : 2, N: colsOf(b, hard), hard }; };
  const every: Wall[] = walls.all().filter((x: any) => x.kind === 'wall').map((x: any) => mk(x.id, false));
  const parts: Wall[] = every.filter((w: Wall) => w.N >= 9)
    .filter((w: Wall) => [0.9, -0.9].every(o => free(onWall(w, 0, o, 0)) && free(onWall(w, 2, o, 0)) && free(onWall(w, -2, o, 0)))).sort((a: Wall, b: Wall) => b.N - a.N || a.id - b.id);
  const part = parts[0]; if (!part) return null;
  const away = (w: Wall, o: Wall, d = 9) => Math.hypot(w.box.c[0] - o.box.c[0], w.box.c[2] - o.box.c[2]) > d, near = (w: Wall, o: Wall) => !away(w, o, 6);
  let hard: Wall | null = null;
  for (let i = 0; i < walls.authored && !hard; i++) if (walls.adoptable(i)) { const w = mk(i, true); if (w.N >= 6 && away(w, part) && [0.5, -0.5].some(o => free(onWall(w, 0, o, 0)))) hard = w; }
  if (!hard) return null;
  let pair: [Wall, Wall] | null = null;
  for (const a of every) for (const b of every) { if (pair || a.id >= b.id || a.u !== b.u || a.id === part.id || b.id === part.id || a.N < 3 || b.N < 3) continue; const n = 2 - a.u, u = a.u;
    if (Math.abs(a.box.c[n] - b.box.c[n]) > 1e-3 || Math.abs(a.box.h[n] - b.box.h[n]) > 1e-3) continue;
    const gap = Math.max(a.box.c[u] - a.box.h[u] - (b.box.c[u] + b.box.h[u]), b.box.c[u] - b.box.h[u] - (a.box.c[u] + a.box.h[u])); if (gap < 0.3 || gap > 2.5) continue;
    if ([a, b].every(w => !near(w, part) && [0.9, -0.9].some(o => free(onWall(w, 0, o, 0))))) pair = [a, b]; }
  return { part, hard, far: every.filter(w => w.id !== part.id && w.N >= 3 && away(w, part, 7) && away(w, hard!, 7) && !(pair && (w.id === pair[0].id || w.id === pair[1].id))).sort((a, b) => b.N - a.N || a.id - b.id)[0] ?? null, pair };
}

/** what the world rasterises after its last upload: the statics by number, and the frame's dynamic boxes */
function picture(g: any): { st: Set<number>; dyn: any[]; cut: Set<any> } {
  const w = g.engine.world, nS = w.statics.length, st = new Set<number>(), dyn: any[] = [], cut = new Set<any>();
  for (let k = 0; k < w.visibleCount + w.visibleCutCount; k++) { const i = w.visible[k] & 0xFFFF; if (i < nS) st.add(i); else { dyn.push(w.all[i]); if (k >= w.visibleCount) cut.add(w.all[i]); } }   // (past visibleCount: the boxes the cut pipeline draws, each under its record)
  return { st, dyn, cut };
}
const cellsOf = (g: any, id: number): string => (g.destr.walls.snapshot()?.b ?? []).find((r: any) => r[0] === id)?.[1] ?? '';
/** a square-standing box's half extents along the world's axes (a quarter-turned one's x and z change places) */
const ext = (b: any): Vec3 => Math.abs(Math.sin(b.yaw)) > 0.5 ? [b.h[2], b.h[1], b.h[0]] : b.h;
/** THE ORACLE: THE LEVEL'S OWN COLLISION. Sample the face of authored static `i` (as the level laid it) every STEP, each point by a short ray from
 *  2 cm in front of the sheet, straight in, no further than WALLSKIN.off behind the sheet's back (no ray meets a drawn-only box, and the frame's own
 *  boxes are not in the collision's list; a bare stud left in an opened cell is the wall's skeleton and holds no paint: BoxFlag.Thin). As the floor
 *  stood, the first solid surface the ray met is WHAT THAT POINT LAY ON. A point has LOST ITS WALL when it lay on something and no solid surface
 *  stands there now, at that depth: paint is on a face, and a broken core that runs on further back is not that face. A point with something solid
 *  or something that moves in front of it is not seen and not asked. Counted: points
 *  drawn that have lost their wall, points not drawn that have not, and how many have lost theirs at all. The first sample of a thing is its baseline */
function sample(g: any, L: Laid, pic = picture(g), hits?: number[], why?: string[]): { over: number; lost: number; open: number; n: number } {
  const s = g.col.boxes[L.i], e = ext(s), u = L.u ?? L.walls[0].u, n = (2 - u) as 0 | 2, sg = L.sg ?? (s.c[n] >= L.walls[0].box.c[n] ? 1 : -1), first = !L.base, base = L.base ?? (L.base = []);
  const whole = pic.st.has(L.i), mine = pic.dyn.filter((b: any) => b.mkey !== undefined && b.mkey >>> 24 === MKEY.skin >>> 24 && ((b.mkey - MKEY.skin) >>> 4) === L.i);   // (what stands of THIS sheet in the frame's list, by its key: a level's sheets share their paints, and a band round the corner crosses this one's plane)
  /** is the point at `x` along the sheet, `y` up, on its middle plane, inside piece b: its box, and its planes where the cut pipeline draws it (a plane is in the box's own frame, before its yaw) */
  const inPiece = (b: any, x: number, y: number): boolean => { const eb = ext(b), d: Vec3 = [0, y - b.c[1], 0]; d[u] = x - b.c[u]; if (Math.abs(d[u]) > eb[u] + 1e-9 || Math.abs(d[1]) > b.h[1] + 1e-9) return false; if (!pic.cut.has(b)) return true;
    const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), lx = cs * d[0] - sn * d[2], lz = sn * d[0] + cs * d[2]; return b.cut.every((c: any) => c.n[0] * lx + c.n[1] * d[1] + c.n[2] * lz <= c.d + 1e-9); };
  let over = 0, lost = 0, nOpen = 0, cnt = 0; const du = Math.min(grain, e[u] / 1.01), dy = Math.min(grain, s.h[1] / 1.01), ro: Vec3 = [0, 0, 0], rd: Vec3 = [0, 0, 0], reach = 0.02 + 2 * e[n] + WALLSKIN.off; rd[n] = -sg; ro[n] = s.c[n] + sg * (e[n] + 0.02);
  for (let x = s.c[u] - e[u] + du / 2; x < s.c[u] + e[u]; x += du) for (let y = s.c[1] - s.h[1] + dy / 2; y < s.c[1] + s.h[1]; y += dy) {
    ro[u] = x; ro[1] = y; const hit = g.col.raycast(ro, rd, reach), met = hit ? g.col.boxes[hit.index] : undefined, moves = !!hit && !met, t = met && !(met.flags & BoxFlag.Thin) ? hit.t : Infinity;   // (a door leaf or a body answers the ray under a number past the statics: no sheet lies on a thing that moves, and what is behind one is not seen by this ray)
    if (first) base.push(moves || t < 0.02 - 1e-6 ? -1 : t); if (hits) hits.push(t < Infinity ? hit.index : -1);
    const b0 = base[cnt++]; if (b0 < 0 || moves || t < 0.02 - 1e-6) continue;
    const gone = b0 < Infinity && !(t <= b0 + 1e-4);
    const drawn = whole || mine.some((b: any) => inPiece(b, x, y));
    if (gone) nOpen++; if (drawn && gone) over++; if (!drawn && !gone) lost++;
    if (why && drawn === gone && why.length < 3) why.push(`${drawn ? 'drawn' : 'lost'} at ${u ? 'z' : 'x'} ${x.toFixed(3)} y ${y.toFixed(3)}: lay ${b0 === Infinity ? 'on nothing' : `at ${b0.toFixed(3)}`}, now ${hit ? `#${hit.index}${met ? `${met.name ? ' ' + met.name : ''}${met.cut ? ' (cut)' : ''}` : ' (moves)'} at ${hit.t.toFixed(3)}` : 'nothing'}`);
  }
  return { over, lost, open: nOpen, n: cnt };
}
/** a level's OWN sheets within `r` of `at`, by what a box is and nothing of the rule's: drawn and nothing else, standing square (a quarter turn or
 *  none), upright, no deeper than WALLSKIN.thick, lightless, not a fixture's and not a destructible's. Each one's front is the side fewer of a few rays
 *  straight in find anything behind */
function sheetsNear(g: any, at: Vec3, r: number): Laid[] {
  const walls = g.destr.walls, boxes = g.col.boxes, lone = BoxFlag.NoShadow | BoxFlag.NoPrimary | BoxFlag.Glass | BoxFlag.Dynamic, out: Laid[] = [];
  for (let i = 0; i < walls.authored; i++) { const b = boxes[i];
    if ((b.flags & lone) !== BoxFlag.NoShadow || Math.abs(Math.sin(2 * b.yaw)) > 1e-6 || b.rot || b.round === true || b.emissive[0] + b.emissive[1] + b.emissive[2] > 0 || walls['fixtureBoxes'].has(i) || g.destr.itemOf(i)) continue;
    const e = ext(b), t = e[0] <= e[2] ? 0 : 2, u = (2 - t) as 0 | 2; if (2 * e[t] > WALLSKIN.thick + 1e-6 || e[t] > b.h[1] || Math.max(Math.abs(b.c[0] - at[0]) - e[0], Math.abs(b.c[2] - at[2]) - e[2]) > r) continue;
    const met = (sg: 1 | -1) => { let k = 0; const ro: Vec3 = [b.c[0], b.c[1], b.c[2]], rd: Vec3 = [0, 0, 0]; rd[t] = -sg; ro[t] += sg * (e[t] + 0.02); for (let q = 0; q < 9; q++) { ro[u] = b.c[u] + e[u] * (q - 4) / 4.5; if (g.col.raycast(ro, rd, 0.02 + 2 * e[t] + WALLSKIN.off)) k++; } return k; };
    const a = met(1), c = met(-1); if (a + c === 0) continue;
    out.push({ name: `#${i}`, walls: [], box: b, i, skin: true, u, sg: a >= c ? 1 : -1 }); }
  return out;
}
/** what stands of every cut skin, as text (two floors, or one floor twice, are compared by it) */
const piecesOf = (g: any): string => [...g.destr.walls.skins.cut].sort((a: any, b: any) => a[0] - b[0]).map(([j, l]: [number, any[]]) => `${j}:${l.map(p => [...p.c, ...p.h].map((x: number) => x.toFixed(4)).join(',')).join(';')}`).join('|');
/** what play is made of, as text: the digest, the cells, the feed's serial, the lane's hash counter, the debris, every authored box's flags */
function play(g: any) {
  const walls = g.destr.walls;
  return { digest: hashDigest(digestGame(g)), cells: JSON.stringify(walls.snapshot() ?? null), serial: walls.serial, counter: walls['counter'], statics: g.col.boxes.length,
    debris: g.destr.debris.list.map((c: any) => c.box.c.map((x: number) => x.toFixed(3)).join(',')).join(' '), flags: g.col.boxes.slice(0, walls.authored).map((b: any) => b.flags).join(',') };
}
/** rounds into the wall at `u` along it, `y` up, from its + side, until the cell there has gone (at most `max`) */
function shoot(g: any, w: Wall, u: number, y: number, max = 40): number {
  const walls = g.destr.walls, from = onWall(w, u, 1.2, y), dir: Vec3 = [0, 0, 0]; dir[2 - w.u] = -1; const had = cellsOf(g, w.id); let k = 0;
  for (; k < max && cellsOf(g, w.id) === had; k++) { const hit = g.col.raycast(from, dir, 3); if (!hit || walls.breakableAt(hit.index)?.id !== w.id) break; walls.round(g.player.id ?? 0, [from[0] + dir[0] * hit.t, y, from[2] + dir[2] * hit.t], dir, hit.index, WEAPONS.fn2000, 0.5); }
  return k;
}

interface Stage { name: string; hidden: Record<string, boolean>; play: ReturnType<typeof play>; skins: Record<string, ReturnType<typeof sample>>; pieces: string; stats: any; drawn: Record<string, boolean>; cut: Record<string, number | null>; extra: any }
/** one floor, the rule as asked, opened every way: what the picture and play were after each */
async function floor(rule: boolean, P: Plan): Promise<Stage[] & { sameAs: string }> {
  const { W, laid } = await stand(rule, P), g = W.game, walls = g.destr.walls, S = walls.skins, out = Object.assign([] as Stage[], { sameAs: laid.map(L => L.i).join(',') }), w = P.part, falls0 = WALL_STATS.falls;
  const step = (n: number) => { for (let i = 0; i < n; i++) W.step(DT); };
  const mark = (name: string, extra: any = {}) => { const pic = picture(g);
    out.push({ name, hidden: Object.fromEntries(laid.map(L => [L.name, (g.col.boxes[L.i].flags & BoxFlag.NoPrimary) !== 0])), play: play(g), skins: g.col.window(() => Object.fromEntries(laid.map(L => [L.name, sample(g, L, pic)]))), pieces: piecesOf(g), stats: { ...S.stats, falls: WALL_STATS.falls - falls0 },
      drawn: Object.fromEntries(laid.map(L => [L.name, pic.st.has(L.i)])), cut: Object.fromEntries(laid.map(L => [L.name, S.cut.has(L.i) ? S.cut.get(L.i).length : null])), extra }); };
  for (const L of laid) laidAt.set(L.name, L.i);
  mark('as the floor stands');
  g.destr.blast(onWall(w, 0, 0.3, 1.0), { who: g.player.id ?? 0, radius: 1.2 }); step(40); mark('a blast against the wall');
  { const trim = laid.find(L => L.name === 'trim the dado says')!, dado = laid.find(L => L.name === 'a dado from the floor')!, b = g.col.boxes[dado.i], f = b.flags, mineOf = () => g.engine.world.dynamics.filter((x: any) => x.name === 'skin' && Math.abs(x.albedo[2] - trim.box.albedo[2]) < 1e-9).length;
    const before = mineOf(); b.flags |= BoxFlag.NoPrimary | BoxFlag.NoShadow; step(1); const hid = mineOf(); b.flags = f; step(1); out[out.length - 1].extra.trim = { before, hid, back: mineOf(), thrown: (g.col.boxes[trim.i].flags & 1) !== 0 }; }
  g.destr.blast(onWall(w, 2.0, -0.3, 1.0), { who: g.player.id ?? 0, radius: 2.2 }); step(40);
  { const m = laid.find(L => L.name === 'a marked band from the floor')!, b = g.col.boxes[m.i]; mark('a second beside it, from the other face', { marked: (S.cut.get(m.i) ?? []).map((p: any) => ({ whole: p.c.every((v: number, k: number) => v === b.c[k]) && p.h.every((v: number, k: number) => v === b.h[k]), marks: p.marks === b.marks, planes: p.cut?.length ?? 0 })) }); }
  { const u = -Math.max(w.box.h[0], w.box.h[2]) + 0.75, a = shoot(g, w, u, 1.4); step(10); mark('rounds through the upper of a column (a port)', { rounds: a });
    const b = shoot(g, w, u, 0.3); step(10); mark('rounds through the sill under it', { rounds: b }); }
  { const save = W.snapshot('wallskin', true), json = JSON.stringify(save), before = piecesOf(g);
    g.destr.blast(onWall(w, -2.2, 0.3, 1.0), { who: g.player.id ?? 0, radius: 2.2 }); step(10); Math.random = seeded(55); W.restore(JSON.parse(json)); step(2);
    mark("a checkpoint's load (taken, the wall opened further, loaded)", { same: piecesOf(g) === before, pieces: S.stats.pieces }); }
  { const snap = takeSnapshot(g), before = piecesOf(g), cut0 = S.cut.size;
    g.destr.blast(onWall(w, -2.2, 0.3, 1.0), { who: g.player.id ?? 0, radius: 2.2 }); step(6); const grew = piecesOf(g) !== before;
    restoreSnapshot(snap); const back = piecesOf(g) === before && S.cut.size === cut0 && (S.out.size ? g.engine.world.undrawn === S.out : g.engine.world.undrawn === null); step(2);
    mark('a tick put back (snapshot, the wall opened further, restored)', { grew, back }); }
  { const h = P.hard, side = [0.3, -0.3].find(o => !g.col.nav.blocked[g.col.nav.idx(Math.floor(onWall(h, 0, o * 1.6, 0)[0] / g.col.nav.cell), Math.floor(onWall(h, 0, o * 1.6, 0)[2] / g.col.nav.cell))]) ?? 0.3;
    const took = walls.adopt(h.id); g.destr.blast(onWall(h, 0, side, 0.9), { who: g.player.id ?? 0, radius: 2.2, breach: h.id }); step(40); mark('a charge on a hard wall', { took, cells: cellsOf(g, h.id) }); }
  if (P.pair) { for (const q of P.pair) { const side = [0.3, -0.3].find(o => !g.col.nav.blocked[g.col.nav.idx(Math.floor(onWall(q, 0, o * 3, 0)[0] / g.col.nav.cell), Math.floor(onWall(q, 0, o * 3, 0)[2] / g.col.nav.cell))]) ?? 0.3; g.destr.blast(onWall(q, 0, side, 1.0), { who: g.player.id ?? 0, radius: 1.2 }); } step(40); }
  mark('a blast on each of two walls in one line', { cells: P.pair ? P.pair.map(q => cellsOf(g, q.id)) : null });
  { const piece = [...S.cut.values()].find((l: any[]) => l.length)?.[0]; out[out.length - 1].extra.fog = piece ? thingKeyOf(piece) : null; out[out.length - 1].extra.key = piece ? piece.mkey >>> 24 === MKEY.skin >>> 24 : null;
    out[out.length - 1].extra.inList = g.engine.world.dynamics.filter((b: any) => b.name === 'skin').length; out[out.length - 1].extra.keys = new Set(g.engine.world.dynamics.filter((b: any) => b.name === 'skin').map((b: any) => b.mkey)).size;
    out[out.length - 1].extra.standing = [...S.cut].reduce((a: number, [j, l]: [number, any[]]) => a + (g.engine.world.staticDrawn(j) || !(g.col.boxes[j].flags & BoxFlag.NoPrimary) ? l.length : 0), 0); }   // (a skin the hangers threw keeps no piece in the list)
  g.destr.reset(); step(2); mark('the level put back');
  return out;
}

export async function wallskinCheck(): Promise<boolean> {
  let all = true;
  const line = (ok: boolean, what: string, detail = '') => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what}${detail ? ` — ${detail}` : ''}`); if (!ok) all = false; };
  console.log(`\n=== wallskin: what is painted on a wall goes with the wall${BREAK ? ' (WALLSKIN_BREAK: the rule left down)' : ''} ===`);
  const was = JSON.parse(JSON.stringify(WALLSKIN)), rnd = Math.random, destr = DESTRUCTION.on;
  try {
    if (!plan) { const { W } = await stand(false, null); plan = choose(W.game); }
    { const shipped = (process.env.TPS_PROFILE ?? '') === 'shipped', said = process.env.TPS_WALLSKIN;
      line(stood === (said !== undefined ? said === '1' : shipped) && FROZEN_HOLDS['WALLSKIN.on'] === 'TPS_WALLSKIN=1', '0 the stand: down on the frozen stand, up on the shipped one, either as its word says where the run gives one; the frozen stand holds it by name', `stood up ${stood} · ${shipped ? 'shipped' : 'frozen'}${said !== undefined ? ` with TPS_WALLSKIN=${said}` : ''}`); }
    if (!plan) { line(false, `1 a partition and a hard wall to work on in ${FILE}`, 'none found'); return false; }
    const P = plan, on = await floor(true, P), off = await floor(false, P), names = Object.keys(on[0].skins);
    line(on.sameAs === off.sameAs, '1.0 the two floors lay the same boxes at the same numbers', on.sameAs);
    const DADO = 'a dado from the floor', SKIRT = 'a skirt from the floor on the other face', STRIPE = 'a stripe off the floor', BEAD = 'a bead thin both ways, from the floor', TRIM = 'trim the dado says', PAIR = 'one band along two walls';
    const RUN = 'a run of conduit that rests on its branch', BOARD = 'a card board', PAPER = 'the paper pinned to the board';
    const OFF4 = 'a sheet 4 cm off the wall, nothing between', SKINS = [DADO, SKIRT, STRIPE, 'a mend over the dado', BEAD, TRIM, RUN, "the conduit's branch", BOARD, PAPER, OFF4], HARD = names.filter(n => n.startsWith('a band from the floor on the hard wall'));
    const tot = (st: Stage, list: string[], k: 'over' | 'lost' | 'open') => list.reduce((a, n) => a + (k === 'lost' && st.hidden[n] ? 0 : st.skins[n][k]), 0);   // (a skin the hangers have thrown is gone whole by their rule, on both floors: none of it is 'lost')
    const HOLE = 1, SECOND = 2, PORT = 3, SILL = 4, LOAD = 5, BACK = 6, CHARGE = 7, TWO = 8, RESET = 9;
    line(P.part.N >= 9 && on.length === 10 && on[0].stats.cut === 0 && names.every(n => on[0].drawn[n]) && on[0].pieces === '', `1 the walls: partition #${P.part.id} (${P.part.N} columns), hard wall #${P.hard.id} (${P.hard.N})${P.pair ? `, two in one line #${P.pair[0].id} and #${P.pair[1].id}` : ', no two partitions in one line on this level'}, ${names.length} things laid on them; nothing is cut on a floor nobody has opened`, `${on[0].stats.cut} cut · every laid box drawn ${names.every(n => on[0].drawn[n])}`);

    // ---- the picture, the rule up: after every way the wall opens
    for (let k = HOLE; k <= BACK; k++) { const st = on[k], opened = tot(st, SKINS, 'open'), over = tot(st, SKINS, 'over'), lost = tot(st, SKINS, 'lost');
      line(opened > 0 && over === 0 && lost === 0, `2.${k} ${st.name}: no point of the ${SKINS.length} sheets that has lost its wall is drawn, and none that has not is lost`, `${opened} points have lost their wall · drawn there ${over} · lost with a wall behind ${lost} · ${st.stats.cut} skins cut into ${st.stats.pieces} pieces${Object.keys(st.extra).length ? ' · ' + JSON.stringify(st.extra) : ''}`); }
    { const st = on[HOLE], r = st.skins[STRIPE], b = st.skins[BEAD]; line(!st.hidden[STRIPE] && r.open > 0 && r.over === 0 && r.lost === 0 && (st.cut[STRIPE] ?? 0) >= 2 && b.open > 0 && b.over === 0 && st.cut[BEAD] !== null, '2.1b a stripe off the floor that has lost less than half its wall is cut, not thrown; a bead thin both ways is cut along the wall it stands on', `stripe thrown ${st.hidden[STRIPE]} · ${r.open} points over open cells · drawn there ${r.over} · lost ${r.lost} · pieces ${st.cut[STRIPE]} · bead: ${b.open} points over open cells, drawn there ${b.over}`); }
    { const st = on[SECOND], F = off[SECOND], r = st.skins[RUN], f = F.skins[RUN], half = r.open * 2 >= r.n;
      line(!st.hidden[RUN] && !F.hidden[RUN] && half && r.over === 0 && r.lost === 0 && f.over === f.open && f.open > 0, '2.2b a run of conduit that rests on its own branch: the hangers pass it by with half its wall gone (without the rule it hangs across the breach), and it is cut like the band', `thrown ${st.hidden[RUN]} · ${r.open} of its ${r.n} points over open cells · drawn there ${r.over} with the rule, ${f.over} without · lost ${r.lost}`);
      const p = on[PORT], b = p.skins[BOARD], q = p.skins[PAPER];
      line(!p.hidden[BOARD] && b.open > 0 && q.open > 0 && b.over + q.over === 0 && b.lost + q.lost === 0 && (p.cut[BOARD] ?? 0) >= 1 && (p.cut[PAPER] ?? 0) >= 1 && off[PORT].skins[PAPER].over > 0, '2.3b a card board the hangers leave up is cut with its wall, and the paper pinned to it (3 cm off the wall, on the board) with it', `board: ${b.open} points over open cells, drawn there ${b.over}, pieces ${p.cut[BOARD]} · paper: ${q.open}, drawn there ${q.over}, pieces ${p.cut[PAPER]} · without the rule the paper has ${off[PORT].skins[PAPER].over} drawn there`); }
    { const a = on[PORT], b = on[SILL];
      line(a.extra.rounds > 0 && b.extra.rounds > 0 && a.skins[DADO].open > on[SECOND].skins[DADO].open && a.skins[SKIRT].open === on[SECOND].skins[SKIRT].open && b.skins[SKIRT].open > a.skins[SKIRT].open, '2.7 a port cuts the dado over the sill and leaves the skirt under it; the sill going takes the skirt there too', `dado points over open cells ${on[SECOND].skins[DADO].open} → ${a.skins[DADO].open} → ${b.skins[DADO].open} · skirt ${on[SECOND].skins[SKIRT].open} → ${a.skins[SKIRT].open} → ${b.skins[SKIRT].open}`); }
    line(on[LOAD].extra.same === true && on[BACK].extra.grew === true && on[BACK].extra.back === true, "2.8 a checkpoint's load stands the same pieces again, and a tick put back puts the pieces back", JSON.stringify({ load: on[LOAD].extra, back: on[BACK].extra }));
    { const st = on[CHARGE], opened = tot(st, HARD, 'open'), over = tot(st, HARD, 'over'), lost = tot(st, HARD, 'lost');
      line(st.extra.took === true && opened > 0 && over === 0 && lost === 0, '2.9 a charge on a hard wall: the bands on both its faces are cut with it', `${opened} points over open cells · drawn there ${over} · lost ${lost} · ${JSON.stringify(st.extra)}`); }
    { const st = on[TWO], r = P.pair ? st.skins[PAIR] : null, both = !!st.extra.cells && st.extra.cells.every((c: string) => c.includes('1'));
      line(P.pair ? both && r!.open > 0 && r!.over === 0 && r!.lost === 0 : true, '2.10 one band along two walls in one line is cut by both', P.pair ? `cells ${JSON.stringify(st.extra.cells)} · ${r!.open} points over open cells · drawn there ${r!.over} · lost ${r!.lost} · pieces ${st.cut[PAIR]}` : `NOT ASKED: ${FILE} has no two partitions in one line`);
      const T = on[HOLE].extra.trim;
      line(st.extra.fog === -1 && st.extra.key === true && st.extra.inList === st.extra.standing && st.extra.inList > 0 && st.extra.keys === st.extra.inList && T.hid === 0 && !T.thrown && T.before > 0 && T.back === T.before, "2.11 the pieces are in the frame's list under the skin key, each under a key of its own, and the fog of war passes them; trim cut as a skin goes out with the box that says it, and comes back with it", JSON.stringify({ fog: st.extra.fog, key: st.extra.key, inList: st.extra.inList, keys: st.extra.keys, standing: st.extra.standing, trim: T })); }

    // ---- what the rule leaves alone
    { const idx = laidAt, flag = (S: Stage, n: string) => Number(S.play.flags.split(',')[idx.get(n)!]);
      const st = on[SECOND], thrown = 'a sheet in the middle of the hole (the hangers throw it)', marked = 'a marked band from the floor';
      line((flag(st, thrown) & 1) === 1 && (flag(off[SECOND], thrown) & 1) === 1 && st.stats.falls === off[SECOND].stats.falls && st.stats.falls > 0 && st.skins[thrown].over === 0, '3.1 what the hangers throw, they throw as they did: the sheet in the hole is hidden by its flags on both floors, the same falls', `falls ${st.stats.falls} with the rule, ${off[SECOND].stats.falls} without`);
      { const r = st.skins[marked], recs = st.extra.marked;
        if (cutBoxesOn()) line(r.open > 0 && r.over === 0 && r.lost === 0 && (st.cut[marked] ?? 0) >= 2 && recs.every((q: any) => q.whole && q.marks && q.planes >= 1 && q.planes <= 4) && st.stats.uncut === 0 && (flag(st, marked) & 1) === 0 && off[SECOND].skins[marked].over > 0,
          '3.2 a band that carries markings is cut where the wall is with every mark where it was: each piece is the whole box and its own markings under a cut record of the rectangle that stands (the cut switch is up)', `${r.open} points over open cells · drawn there ${r.over} (without the rule ${off[SECOND].skins[marked].over}) · lost ${r.lost} · pieces ${JSON.stringify(recs)}`);
        else line(st.cut[marked] === null && st.drawn[marked] && r.open > 0 && st.stats.uncut >= 1 && (flag(st, marked) & 1) === 0, '3.2 a band that carries markings, with the cut switch DOWN (no record is drawn): left as it was, not cut and not hidden, and counted', `cut ${st.cut[marked]} · drawn ${st.drawn[marked]} · ${r.open} points over open cells · uncut ${st.stats.uncut}`); }
      const NOT = ['a duct with a body (5 cm)', 'a sheet 12 cm off the wall', 'a rug under the wall', "a strip lying flat at the wall's foot", 'a dado on a wall nothing opens'].filter(n => names.includes(n));
      { const p = on[PORT].skins[OFF4]; line(p.open > 0 && p.over === 0 && p.lost === 0 && (on[PORT].cut[OFF4] ?? 0) >= 1 && off[PORT].skins[OFF4].over > 0, '3.3 a sheet standing 4 cm off the wall with nothing between is the wall\'s: cut over the port', `${p.open} points have lost their wall · drawn there ${p.over} (without the rule ${off[PORT].skins[OFF4].over}) · lost ${p.lost} · pieces ${on[PORT].cut[OFF4]}`); }
      const last = on[TWO];
      line(NOT.length === 5 && NOT.every(n => last.cut[n] === null && last.drawn[n] === off[TWO].drawn[n]) && last.skins['a duct with a body (5 cm)'].over > 0, '3.4 not this rule\'s, not cut: a duct with a body (it hangs over the hole as it did), a sheet 12 cm off the wall, a rug under the wall, a strip lying flat at its foot, a dado on a wall nothing opened', NOT.map(n => `${n}: ${last.cut[n] === null ? 'whole' : 'CUT'}`).join(' · ')); }
    { const st = on[RESET]; line(st.stats.cut === 0 && st.stats.capped === 0 && st.stats.uncut === 0 && st.stats.coarse === 0 && st.stats.pieces === 0 && st.pieces === '' && names.every(n => st.drawn[n]) && st.play.statics === on[0].play.statics, '3.5 the level put back: every laid box drawn whole, nothing cut, no piece left, every count at nought', `${JSON.stringify(st.stats)} · pieces '${st.pieces}'`); }

    // ---- without the rule the fault is there; with it, play is the same run
    { const st = off[HOLE], over = tot(st, SKINS, 'over'); line(over > 0 && st.stats.cut === 0, '4.1 without the rule the band hangs across the hole (the fault this mends, shown to be there)', `${over} points drawn over open cells · ${st.stats.cut} cut`); }
    { const bad: string[] = []; for (let k = 0; k < on.length; k++) for (const f of ['digest', 'cells', 'serial', 'counter', 'statics', 'debris', 'flags'] as const) if (String(on[k].play[f]) !== String(off[k].play[f])) bad.push(`${on[k].name}: ${f}`);
      line(bad.length === 0 && on[TWO].play.serial > 4 && on[SECOND].play.debris.length > 0, `4.2 play is one run on the two floors at every one of the ${on.length} stages: the digest, the cells, the feed's serial, the lane's hash counter, the debris and its places, the count of statics, and every authored box's flags`, bad.length ? bad.slice(0, 4).join(' · ') : `serial ${on[TWO].play.serial} · hash counter ${on[TWO].play.counter} · digest ${on[TWO].play.digest}`); }

    // ---- the caps, the cutter by itself, the journal
    { const keep = { ...WALLSKIN };
      try { const tryCap = async (set: Partial<typeof keep>) => { Object.assign(WALLSKIN, keep, set); const { W, laid } = await stand(true, P), g = W.game, S = g.destr.walls.skins, dado = laid.find(L => L.name === DADO)!; g.col.window(() => sample(g, dado));
          g.destr.blast(onWall(P.part, 0, 0.3, 1.0), { who: 0, radius: 2.2 }); for (let i = 0; i < 10; i++) W.step(DT);
          return { r: g.col.window(() => sample(g, dado)), n: S.cut.has(dado.i) ? S.cut.get(dado.i).length : null, stats: { ...S.stats }, drawn: picture(g).st.has(dado.i), out: S.out.has(dado.i), handed: g.engine.world.dynamics.filter((b: any) => b.name === 'skin').length, W, g, dado }; };
        const two = await tryCap({ perSkin: 2 }), one = await tryCap({ perSkin: 1 }), all3 = await tryCap({ maxPieces: 3 });
        line(two.n === 2 && two.stats.coarse >= 1 && two.r.over === 0 && two.r.lost > 0 && one.n === null && one.drawn && one.stats.capped >= 1 && one.r.over > 0 && all3.stats.pieces <= 3 && all3.stats.capped >= 1,
          '5.1 past a cap a skin is cut by whole columns (nothing drawn over an open cell, the sills under the ports given up), and past that left as it last stood, counted', `two a skin: ${two.n} pieces, by columns ${two.stats.coarse}, drawn over open cells ${two.r.over}, lost ${two.r.lost} · one a skin: cut ${one.n}, drawn whole ${one.drawn}, capped ${one.stats.capped} · three in all: ${all3.stats.pieces} standing, capped ${all3.stats.capped}`);
        const full = await tryCap({ boxSlack: 1e6 });
        line(BREAK ? true : (full.n ?? 0) >= 1 && full.handed === 0 && !full.out && full.drawn && full.r.lost === 0 && full.r.over > 0, "5.1b a frame whose box list has no room for what stands of a sheet draws the authored sheet whole: never neither", `the dado cut into ${full.n} · pieces handed over ${full.handed} · the world leaves the dado out ${full.out} · drawn whole ${full.drawn} · lost with a wall behind ${full.r.lost}`);
        { Object.assign(WALLSKIN, keep); const { W, laid } = await stand(false, P), g = W.game, S = g.destr.walls.skins, dado = laid.find(L => L.name === DADO)!, step = (n: number) => { for (let i = 0; i < n; i++) W.step(DT); }; g.col.window(() => sample(g, dado));
          g.destr.blast(onWall(P.part, 0, 0.3, 1.0), { who: 0, radius: 2.2 }); step(10); const down = { cut: S.stats.cut, r: g.col.window(() => sample(g, dado)) };
          WALLSKIN.on = !BREAK; step(2); const up = { cut: S.stats.cut, r: g.col.window(() => sample(g, dado)) };
          WALLSKIN.on = false; step(2); const again = { cut: S.stats.cut, drawn: picture(g).st.has(dado.i), undrawn: g.engine.world.undrawn };
          line(down.cut === 0 && down.r.over > 0 && up.cut > 0 && up.r.over === 0 && up.r.lost === 0 && again.cut === 0 && again.drawn && again.undrawn === null, '5.1c the switch turned under a standing floor: raised, every wall that has opened is asked again; put down, every sheet is whole again', `down: ${down.cut} cut, ${down.r.over} points drawn with no wall · raised: ${up.cut} cut, ${up.r.over} drawn with no wall, ${up.r.lost} lost · down again: ${again.cut} cut, the dado drawn whole ${again.drawn}`); }
      } finally { Object.assign(WALLSKIN, keep); } }
    { const F = [-2, 2, -0.5, 0.5], area = (r: number[][]) => r.reduce((a, q) => a + (q[1] - q[0]) * (q[3] - q[2]), 0);
      const r = standing(2, 0.5, [F], [0], [[[-0.5, 0.5, -0.5, 0.5], [1, 1.5, 0.2, 0.5]]]), none = standing(1, 1, [[-1, 1, -1, 1]], [0], [[[-1, 1, -1, 1]]]), whole = standing(1, 1, [[-1, 1, -1, 1]], [0], [[]]);
      const two = standing(2, 0.5, [F, [-0.5, 0, -0.5, 0.5]], [0, 0], [[[-0.5, 0.5, -0.5, 0.5]], []]), part = standing(2, 0.5, [[-2, 0, -0.5, 0.5]], [0], [[[-2, 2, -0.5, 0.5]]]), deep = standing(2, 0.5, [F, [-0.5, 0, -0.5, 0.5]], [0, 0.05], [[[-0.5, 0.5, -0.5, 0.5]], []]);
      line(Math.abs(area(r) - (4 - 1 - 0.15)) < 1e-9 && r.length <= 5 && none.length === 0 && whole.length === 1 && Math.abs(area(two) - 3.5) < 1e-9 && Math.abs(area(part) - 2) < 1e-9 && Math.abs(area(deep) - 3) < 1e-9,
        '5.2 the cutter by itself: what stands of a face with two bites taken has the right area in few rectangles; all gone = none; untouched = one; a part another thing in the same plane still stands behind is kept; a part that lay on nothing is kept; a thing further back than what a part lies on does not keep it', `${r.length} rectangles, area ${area(r).toFixed(3)} of ${(4 - 1.15).toFixed(3)} · behind another ${area(two).toFixed(2)} of 3.50 · on nothing ${area(part).toFixed(2)} of 2.00 · further back ${area(deep).toFixed(2)} of 3.00`); }
    { const { W } = await stand(true, null); const snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(W.game)));
      WALLSKIN.on = true; const old = snap(); delete old.tables.WALLSKIN; J.applyKnobs(W.game, old); const before = WALLSKIN.on;
      WALLSKIN.on = true; const rec = snap(); WALLSKIN.on = false; J.applyKnobs(W.game, rec); const kept = WALLSKIN.on;
      line(before === false && kept === true, '5.3 the journal: a recording from before the table replays with the rule down, one made with it up replays with it up', `without the table ${before} · recorded up ${kept}`); }
    // ---- two levels' own paint, by the same oracle
    { // a corner where two partitions overlap: one's end column opened by itself. The band on the wall that stands is whole, and so is the opened wall's
      // band over the standing one's end (the cells of the wall that opened are not the whole of what stands behind a sheet)
      const { W } = await stand(true, null, 'block-gwot.json'), g = W.game, walls = g.destr.walls, list: any[] = walls['list'], step = (n: number) => { for (let i = 0; i < n; i++) W.step(DT); };
      let found: { A: any; B: any; cells: number[]; at: Vec3 } | null = null;
      for (const A of list) { if (found || A.kind !== 'wall' || A.hard || A.box.yaw) continue; const a = A.box, u = a.h[0] >= a.h[2] ? 0 : 2, n = 2 - u, N = u === 0 ? A.nx : A.nz; if (N < 4) continue;
        for (const B of list) { const b = B.box; if (found || B === A || B.kind !== 'wall' || B.hard || b.yaw || (b.h[0] >= b.h[2] ? 0 : 2) !== n) continue;
          if (Math.min(a.c[0] + a.h[0], b.c[0] + b.h[0]) - Math.max(a.c[0] - a.h[0], b.c[0] - b.h[0]) < 0.02 || Math.min(a.c[2] + a.h[2], b.c[2] + b.h[2]) - Math.max(a.c[2] - a.h[2], b.c[2] - b.h[2]) < 0.02) continue;   // (they overlap: the corner is both walls')
          const end = b.c[u] >= a.c[u] ? 1 : -1, k = end > 0 ? N - 1 : 0, at: Vec3 = [a.c[0], 0, a.c[2]]; at[u] = a.c[u] + end * a.h[u];
          if (sheetsNear(g, at, 0.6).length < 2) continue;
          found = { A, B, at, cells: Array.from({ length: A.ny }, (_, iy) => (iy * A.nz + (u === 0 ? 0 : k)) * A.nx + (u === 0 ? k : 0)) }; } }
      if (!found) line(false, '6.1 a corner where two partitions overlap, with paint on it, in block-gwot.json', 'none found');
      else { const F = found, near = sheetsNear(g, F.at, 0.6), h0: number[][] = near.map(() => []), h1: number[][] = near.map(() => []);
        g.col.window(() => near.forEach((L, k) => sample(g, L, picture(g), h0[k])));
        const row = walls['openCells'](F.A, F.cells, 'fire', 0, null, true); step(3);
        const pic = picture(g), r = g.col.window(() => near.map((L, k) => sample(g, L, pic, h1[k])));
        const over = r.reduce((a: number, x: any) => a + x.over, 0), lost = r.reduce((a: number, x: any) => a + x.lost, 0), n = r.reduce((a: number, x: any) => a + x.n, 0);
        let kept = 0; h0.forEach((l, k) => l.forEach((i, q) => { if (i >= 0 && walls.breakableAt(i)?.id === F.A.id && h1[k][q] >= 0 && walls.breakableAt(h1[k][q])?.id !== F.A.id) kept++; }));   // (points that lay on the opened wall's own box and still have the other wall behind them)
        line(!!row && over === 0 && lost === 0 && kept > 0, '6.1 a corner where two partitions overlap, one\'s end column opened: no sheet at the corner loses paint where a wall still stands behind it, and none hangs', `block-gwot: wall #${F.A.id}'s end column at #${F.B.id} · ${near.length} sheets, ${n} points · ${kept} lay on the opened wall and have the other behind them still · drawn with nothing behind ${over} · lost with a wall behind ${lost}`); } }
    { // a decal on a piece of COVER (a low wall the lane breaks whole): the cover razed, the decal goes with it
      const { W } = await stand(true, null, 'mousehole-gwot.json'), g = W.game, walls = g.destr.walls, list: any[] = walls['list'], step = (n: number) => { for (let i = 0; i < n; i++) W.step(DT); };
      let C: any = null, on: Laid[] = [];
      g.col.window(() => { for (const B of list) { if (C || B.kind !== 'cover' || B.box.yaw) continue; const near = sheetsNear(g, B.box.c, Math.max(B.box.h[0], B.box.h[2]) + 0.1).filter(L => { const hits: number[] = []; sample(g, { ...L, base: undefined }, picture(g), hits); return hits.some(i => i >= 0 && walls.breakableAt(i)?.id === B.id); }); if (near.length) { C = B; on = near; } } });
      if (!C) line(false, '6.2 a piece of cover with a decal on it, in mousehole-gwot.json', 'none found');
      else { g.col.window(() => on.forEach(L => sample(g, L)));
        const row = walls.raze(C.id, 0, [1, 0, 0]); step(30);
        const pic = picture(g), r = g.col.window(() => on.map(L => sample(g, L, pic))), open = r.reduce((a: number, x: any) => a + x.open, 0), over = r.reduce((a: number, x: any) => a + x.over, 0), lost = r.reduce((a: number, x: any) => a + x.lost, 0);
        const slack = WALLSKIN.boxSlack; WALLSKIN.boxSlack = 1e6; step(2); const stays = on.filter(L => (walls.skins.cut.get(L.i) ?? [0]).length === 0).every(L => !picture(g).st.has(L.i)); WALLSKIN.boxSlack = slack;
        line(!!row && open > 0 && over === 0 && lost === 0 && (BREAK || stays), '6.2 a decal on a piece of cover goes with the cover: razed, no point of it is drawn with nothing behind; and a sheet with nothing left standing stays out of a frame whose list has no room', `mousehole-gwot: cover #${C.id}, ${on.length} sheet${on.length === 1 ? '' : 's'} on it (${on.map(L => L.name).join(' ')}) · ${open} points have lost what backed them · drawn there ${over} · lost with something behind ${lost} · with no room it stays out ${stays}`); } }
    { // a piece of cover opened IN PART: under the cut switch what stands of it ends ragged (a cut record), and the paint on it takes that outline
      const { W } = await stand(true, null, 'consulate-gwot.json'), g = W.game, walls = g.destr.walls, list: any[] = walls['list'], S = walls.skins, step = (n: number) => { for (let i = 0; i < n; i++) W.step(DT); };
      let C: any = null, on: Laid[] = [];
      g.col.window(() => { for (const B of list) { if (C || B.kind !== 'cover' || B.box.yaw || B.open.length < 3) continue; const near = sheetsNear(g, B.box.c, Math.max(B.box.h[0], B.box.h[2]) + 0.1).filter(L => { const hits: number[] = []; sample(g, { ...L, base: undefined }, picture(g), hits); return hits.some(i => i >= 0 && walls.breakableAt(i)?.id === B.id); }); if (near.length) { C = B; on = near; } } });
      if (!C) line(false, '6.3 a piece of cover of three cells or more with paint on it, in consulate-gwot.json', 'none found');
      else { g.col.window(() => on.forEach(L => sample(g, L)));
        const row = walls['openCells'](C, [0], 'fire', 0, null, true); step(3);
        const pic = picture(g), r = g.col.window(() => on.map(L => sample(g, L, pic))), open = r.reduce((a: number, x: any) => a + x.open, 0), over = r.reduce((a: number, x: any) => a + x.over, 0), lost = r.reduce((a: number, x: any) => a + x.lost, 0);
        const ragged = C.slots.slice(0, C.live).some((i: number) => g.col.boxes[i].cut?.length), took = on.some(L => (S.cut.get(L.i) ?? []).some((p: any) => p.cut?.length));
        line(!!row && open > 0 && over === 0 && lost === 0 && (BREAK || ragged === took), '6.3 a piece of cover opened in part: the paint on it goes where the cover went, and where what stands ends ragged the paint takes that outline', `consulate-gwot: cover #${C.id} (${C.open.length} cells, one opened), ${on.length} sheets on it · ${open} points have lost what they lay on · drawn there ${over} · lost with it still there ${lost} · what stands ends ragged ${ragged}, the paint's piece carries its planes ${took}`); } }
  } finally { Object.assign(WALLSKIN, was); Math.random = rnd; DESTRUCTION.on = destr; }
  console.log(all ? '  wallskin: all lines pass' : '  wallskin: FAILED');
  return all;
}
/** A LEVEL'S OWN PAINT, EVERY WALL OF IT (a tool, not a line of the check: `bun run tools/qa/wallskin-check.ts --sweep a.json b.json`, or `--sweep all`).
 *  Each partition and each piece of cover is blasted at three places, and each hard wall a charge can take on that carries a sheet is taken on and
 *  breached once, one at a time, the level put back between; after each, every thin drawn-only sheet near the blast is asked by the same oracle. Sheets that lay on something wallbreak.ts opens are the rule's; a sheet that lay only on a thing
 *  another lane destroys (a label on a crate) is counted apart: the rule is told of cells opening, not of furniture going. */
export async function wallskinSweep(files: string[]): Promise<boolean> {
  const { readdirSync } = await import('node:fs'); let bad = 0; const ONLY = process.env.WALLSKIN_ONLY ? process.env.WALLSKIN_ONLY.split(',').map(Number) : null;
  if (files[0] === 'all') files = readdirSync(`${ROOT}/levels`).filter((f: string) => f.endsWith('.json') && !f.includes('.lint.')).sort();
  const rnd = Math.random, destr = DESTRUCTION.on, was = JSON.parse(JSON.stringify(WALLSKIN)), fine = grain; grain = 3 * STEP; const T = { blasts: 0, hard: 0, sheets: 0, points: 0, open: 0, over: 0, lost: 0, otherOver: 0, idle: 0 };
  try { for (const file of files) {
    let W: any; try { ({ W } = await stand(true, null, file)); } catch { console.log(`  ${file}: does not stand`); continue; }
    const g = W.game, walls = g.destr.walls, list: any[] = walls['list'].slice(), t = { blasts: 0, hard: 0, sheets: 0, points: 0, open: 0, over: 0, lost: 0, otherOver: 0, idle: 0 }, ex: string[] = [], other = new Map<string, string>();
    const ask = (id: number, kind: string, b: any, uk: number, blow: (at: Vec3) => void) => { const u = b.h[0] >= b.h[2] ? 0 : 2, n = 2 - u, at: Vec3 = [b.c[0], Math.min(1.0, b.c[1] + b.h[1]), b.c[2]]; at[u] += uk * b.h[u]; at[n] += b.h[n] + 0.3;
      const near = g.col.window(() => { const l = sheetsNear(g, at, 3.5), mine = l.map(L => { const hits: number[] = []; sample(g, L, picture(g), hits); return hits.some(i => i >= 0 && (walls.breakableAt(i) || walls.adoptable(i))); }); return l.map((L, k) => ({ L, mine: mine[k] })); });
      blow(at); for (let i = 0; i < 12; i++) W.step(DT); t.blasts++;
      const pic = picture(g); g.col.window(() => { for (const { L, mine } of near) { if (g.col.boxes[L.i].flags & BoxFlag.NoPrimary) continue;   // (the hangers threw it: gone whole by their rule)
        const why: string[] = [], r = sample(g, L, pic, undefined, why); t.sheets++; t.points += r.n; t.open += r.open; if (!r.open && walls.skins.cut.has(L.i)) t.idle++;
        if (mine) { t.over += r.over; t.lost += r.lost; if ((r.over || r.lost) && ex.length < 8) ex.push(`\n      ${L.name} after #${id} (${kind}) @${uk}: over ${r.over} lost ${r.lost} of ${r.n} · pieces ${(walls.skins.cut.get(L.i) ?? { length: 'uncut' }).length} · ${why.join(' | ')}`); } else { t.otherOver += r.over; if (r.over && !other.has(L.name)) other.set(L.name, `${L.name} after #${id}: ${why[0] ?? ''}`); } } });
      g.destr.reset(); W.step(DT); };
    for (const B of list) { if (B.hard || B.box.yaw || (ONLY && !ONLY.includes(B.id))) continue; for (const uk of [0, 0.85, -0.85]) ask(B.id, B.kind, B.box, uk, at => g.destr.blast(at, { who: 0, radius: 2.2 })); }
    // … and every hard wall a charge can take on that has a sheet on it: taken on and breached once, at its middle
    for (let i = 0; i < walls.authored; i++) { const b = g.col.boxes[i]; if (!walls.adoptable(i) || b.yaw || (ONLY && !ONLY.includes(i))) continue;
      const has = g.col.window(() => sheetsNear(g, b.c, Math.max(b.h[0], b.h[2]) + 0.1).some(L => { const hits: number[] = []; sample(g, L, picture(g), hits); return hits.includes(i); })); if (!has) continue;
      ask(i, 'hard wall', b, 0, at => { walls.adopt(i); g.destr.blast(at, { who: 0, radius: 2.2, breach: i }); }); t.hard++; }
    for (const k of Object.keys(T) as (keyof typeof T)[]) T[k] += t[k]; bad += t.over + t.lost;
    console.log(`  ${t.over + t.lost ? 'FAIL' : 'PASS'}  ${file}: ${t.blasts} blasts (${t.hard} of them hard walls breached) · ${t.sheets} sheets asked, ${t.points} points · ${t.open} have lost what they lay on · drawn there ${t.over} · lost with it still there ${t.lost} · cut with nothing gone at this grain ${t.idle} · on a thing another lane took, still drawn ${t.otherOver}${other.size ? ` (${other.size} sheets: ${[...other.values()].slice(0, 3).join(' ; ')})` : ''}${ex.length ? ' · ' + ex.join(' · ') : ''}`); }
  } finally { Object.assign(WALLSKIN, was); Math.random = rnd; DESTRUCTION.on = destr; grain = fine; }
  console.log(`  ${files.length} levels: ${T.blasts} blasts (${T.hard} hard walls breached) · ${T.points} points of ${T.sheets} sheets · ${T.open} lost what they lay on · drawn there ${T.over} · lost with it still there ${T.lost} · on a thing another lane took, still drawn ${T.otherOver}`);
  return bad === 0;
}
if (import.meta.main) { const k = process.argv.indexOf('--sweep'); process.exit((k >= 0 ? await wallskinSweep(process.argv.slice(k + 1)) : await wallskinCheck()) ? 0 : 1); }
