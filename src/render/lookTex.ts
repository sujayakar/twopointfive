// THE MATERIAL MAPS (STYLE.md, Surfaces; the page /look, render/frame.ts lookConfig; shaders/look.wgsl reads them): the tiling materials (LOOK_NAMES) and two of masonry made HERE, by the page,
// at boot — no image file, nothing fetched, nothing licensed. Each layer is LOOK_N × LOOK_N texels over LOOK_TILE metres, two maps a layer:
//   A (rgba8): rgb = what the box's paint is multiplied by, ÷ 2 (128 = the paint as it is; so a map can also turn a hue), a = the height, 0‥1 of the layer's range
//   B (rgba8): rg = the tangent-space normal's xy (½ + ½·n), b = roughness, a = the hollows: ½ of how open the texel is (1 = not in a hollow) + what puts the tile's paint-weighed mean at ½ (lookLayer)
// Everything is periodic over the tile, so a layer repeats without a seam; the shader breaks the repeat up. Pure arithmetic on typed arrays: runs the same in a browser and under bun.
export const LOOK_N = 512, LOOK_TILE = 2.0, LOOK_LAYERS = 30;
/** the layers. A STUFF is a layer a box's paint can be drawn as, and its number is its place here and one (0 = plain). After the stuffs, the two layers of
 *  what a mud wall is made of under its coat, which no box is: NEW STUFFS GO IN FRONT OF THOSE TWO and nothing else moves. A box's word has room for 31
 *  stuffs (gbuffer.ts lookWordOf). Ask lookStuffOf for a name's number; never count places. EVERYTHING THE SHADER DOES BY STUFF IS A WORD IN THAT STUFF'S ROW
 *  below (STUFFS): how it is laid, what the weather and use do to it, which way its grain runs, what it is made of under its coat. look.wgsl names no
 *  stuff: it reads the rows (through the prelude, gbuffer.ts lookWgsl), so a new stuff is a row and its map and no line of shader. */
export const LOOK_NAMES = ['mud wall', 'plaster', 'block', 'earth', 'gravel', 'slab', 'wood', 'sheet', 'cloth', 'metal', 'paint', 'veneer', 'plastic', 'fabric', 'steel', 'carpaint', 'stucco', 'skim', 'gloss block', 'board concrete', 'cast panel', 'paver', 'brushed', 'louvre', 'enamel', 'carpet', 'tarmac', 'grass', 'field stone', 'mud brick'] as const;
export type LookName = typeof LOOK_NAMES[number];
/** WHAT A STUFF IS, one row a name (a name with no row does not compile). The maps below and the shader (look.wgsl lookSurface, through the prelude:
 *  gbuffer.ts lookWgsl) both read these, and the shader names no stuff for any of them.
 *  rough   the roughness the layer is written around (the shader adds a texel's difference from it to the box's own finish)
 *  relief  how much the relief is stood up when the normal is taken off the height (1 = true to the metres drawn; a map read from twenty metres wants more)
 *  built   laid or cast IN PLACE (a rendered wall, laid blocks, a poured floor, pavers): its map lies by the world, so courses and seams run on from one box
 *          of a wall to the next. Absent = the thing's own (boards, cloth, what a factory made): its map lies in the box's frame and goes where the box goes
 *  fades   how far sun and years blotch it (1; 0.3 = made in a factory and kept indoors, or washed). ABSENT = a place somebody keeps: no blotch, no drift
 *          of hue, none of the traffic's dirt on it
 *  soaks   standing on the ground it takes a damp foot, and its head bleaches and sheds dark runs
 *          (HOW FAR `fades` and `soaks` SHOW IN A PLACE is the place's word, not the stuff's: a level says how kept it is, look.wgsl lookKept; these two
 *          say what the stuff does where nobody keeps it)
 *  unit    a stuff MADE OF PIECES: one piece's size, metres along and up (each divides LOOK_TILE), whether every other course is laid half a piece along,
 *          and how far one piece's shade may sit from the next's (± × the paint). The map draws the pieces' joints at these sizes (`piece`) and gives them no
 *          shade of their own; the shader gives each piece of the WORLD its shade by where it is, so a wall of eighty panels is not a tile's eight repeated
 *  untiles it has no courses and no pieces, so its map is read twice, the second read turned and shifted, and the two cross-faded by a slow noise: a
 *          two-metre map does not print its repeat on a forty-metre yard (the four the shader used to name, and the kept ones that are a plain coat)
 *  bay     a stuff made of pieces is LAID IN BAYS: so many pieces along and up to a movement joint, wider and darker than a piece's own, and each bay a
 *          shade its own (pavers laid a day's work at a time, a wall cast in lifts). 0 = none that way. A bonded stuff has none along: its courses
 *          are half a piece out of step and the joint would zigzag. The shader draws both by where the piece is in the WORLD
 *  worn    how hard USE shows on it in a place somebody keeps (0 or absent = not at all: steel, enamel, a carpet). A stuff that also `fades` shows
 *          it by how far the place it stands in is kept (a level's word, look.wgsl lookKept): plaster in a kept office has a knocked foot as skim
 *          has, and none in a ruin, where the weather has it. Not weather: what boots, a trolley and a mason called back leave. The shader gives it two things by where it is in the WORLD, never in
 *          the map: a piece in four is stained, chipped at a corner or was changed for a new one (look.wgsl lookPiece: a stuff made of pieces), and a
 *          wall's foot has its coat knocked off in stretches to a hand's height
 *  runs    which way its grain or its ribs run on a thing of its own (one not built in place, or one that moves): 'long' = along the long side of every
 *          face (boards, a veneered top); 'flat' = along the long side of a face that LIES and up a face that stands (a sheet's ribs run up a wall of
 *          it; and a built stuff, when a thing that moves is made of it). Absent = as the box's own frame has it (cloth, bare metal, old paint)
 *  least   a face narrower than this, metres from its middle to its edge, is too small to show the stuff's map (a sheet's ribs are a roof's, not a bucket's)
 *  under   what it is MADE of under its coat, on a wall of it at least `long` by `tall` metres: a `plinth` course from the ground up to a wandering line,
 *          and a `core` where the coat has let go in patches (a mud wall: field stone, mud brick). The two are layers no box is
 *  foot    what works its way up through it at a wall's foot on ground nobody keeps (stones through earth): another stuff's layer
 *  ground  bare ground: within a stride of lying snow it freezes (ice in the low places, rime over the rest)
 *  sheds   snow does not lie on it or cling to it (bare iron on a snow level is a stove, a flue, a brazier) */
interface Stuff { rough: number; relief: number; built?: true; fades?: number; soaks?: true; worn?: number; untiles?: true; unit?: readonly [u: number, v: number, bond: boolean, shade: number]; bay?: readonly [along: number, up: number];
  runs?: 'long' | 'flat'; least?: number; under?: { readonly plinth: LookName; readonly core: LookName; readonly long: number; readonly tall: number }; foot?: LookName; ground?: true; sheds?: true }
const STUFFS: Readonly<Record<LookName, Stuff>> = {
  'mud wall': { rough: 0.95, relief: 1.6, built: true, fades: 1, soaks: true, untiles: true, runs: 'flat', under: { plinth: 'field stone', core: 'mud brick', long: 0.6, tall: 0.24 } }, plaster: { rough: 0.86, relief: 1.8, built: true, fades: 1, soaks: true, worn: 1, untiles: true, runs: 'flat' },
  block: { rough: 0.93, relief: 1.5, built: true, fades: 1, soaks: true, worn: 0.8, runs: 'flat' }, earth: { rough: 0.94, relief: 1.6, built: true, fades: 1, soaks: true, untiles: true, runs: 'flat', foot: 'gravel', ground: true },
  gravel: { rough: 0.88, relief: 1.3, built: true, fades: 1, soaks: true, untiles: true, runs: 'flat', ground: true }, slab: { rough: 0.82, relief: 1.8, built: true, fades: 1, soaks: true, worn: 1, runs: 'flat' },   // (plaster, block and slab say `worn` as their kept cousins do — skim, cast panel, paver: it shows only in a place a level calls kept)
  wood: { rough: 0.84, relief: 1.6, fades: 1, runs: 'long' }, sheet: { rough: 0.5, relief: 1.0, fades: 1, soaks: true, runs: 'flat', least: 0.2 }, cloth: { rough: 0.96, relief: 1.6, fades: 1, soaks: true },
  metal: { rough: 0.75, relief: 1.6, fades: 1, soaks: true, sheds: true }, paint: { rough: 0.5, relief: 1.6, fades: 1, soaks: true },
  veneer: { rough: 0.4, relief: 1.4, fades: 0.3, runs: 'long' }, plastic: { rough: 0.5, relief: 1.4, fades: 0.3 }, fabric: { rough: 0.95, relief: 1.6, fades: 0.3 },
  steel: { rough: 0.42, relief: 1.4, fades: 0.3 }, carpaint: { rough: 0.26, relief: 1.2, fades: 0.3 },
  stucco: { rough: 0.42, relief: 1.5, built: true, worn: 1, untiles: true }, skim: { rough: 0.55, relief: 1.6, built: true, worn: 1, untiles: true },
  'gloss block': { rough: 0.4, relief: 1.4, built: true, worn: 0.6, unit: [0.4, 0.2, true, 0.02] }, 'board concrete': { rough: 0.42, relief: 1.6, built: true, worn: 0.8 },
  'cast panel': { rough: 0.68, relief: 1.4, built: true, worn: 0.8, unit: [1.0, 0.5, true, 0.07], bay: [0, 3] }, paver: { rough: 0.5, relief: 1.4, built: true, worn: 1, unit: [0.5, 0.5, false, 0.018], bay: [6, 6] },
  brushed: { rough: 0.32, relief: 1.2 }, louvre: { rough: 0.45, relief: 1.0 }, enamel: { rough: 0.3, relief: 1.2 }, carpet: { rough: 0.95, relief: 1.4, untiles: true },
  tarmac: { rough: 0.86, relief: 1.3, built: true, fades: 1, untiles: true, runs: 'flat', ground: true }, grass: { rough: 0.95, relief: 1.2, built: true, fades: 1, soaks: true, untiles: true, runs: 'flat', ground: true },
  'field stone': { rough: 0.8, relief: 1.0, fades: 1, soaks: true }, 'mud brick': { rough: 0.95, relief: 1.2, fades: 1, soaks: true },
};
/** the rows as the prelude and the checks read them, in LOOK_NAMES' order */
export const LOOK_ROUGH: readonly number[] = LOOK_NAMES.map(n => STUFFS[n].rough);
export const LOOK_BUILT: readonly number[] = LOOK_NAMES.map(n => STUFFS[n].built ? 1 : 0);
export const LOOK_FADES: readonly number[] = LOOK_NAMES.map(n => STUFFS[n].fades ?? 0);
export const LOOK_SOAKS: readonly number[] = LOOK_NAMES.map(n => STUFFS[n].soaks ? 1 : 0);
export const LOOK_WORN: readonly number[] = LOOK_NAMES.map(n => STUFFS[n].worn ?? 0);
/** how a stuff is laid over more than a piece: (it untiles, pieces a bay along, pieces a bay up, which way it runs: 0 its box's own, 1 flat, 2 long) */
export const LOOK_LAY: readonly (readonly [number, number, number, number])[] = LOOK_NAMES.map(n => { const st = STUFFS[n]; return [st.untiles ? 1 : 0, st.bay?.[0] ?? 0, st.bay?.[1] ?? 0, st.runs === 'long' ? 2 : st.runs === 'flat' ? 1 : 0]; });
const layerOf = (n: LookName | undefined): number => n === undefined ? -1 : LOOK_NAMES.indexOf(n);
/** what a thing of a stuff is, beyond its map: (least: half a face's width under which the map is not shown, 0 = any; the layer that comes up at a wall's
 *  foot, −1 = none; bare ground that freezes by snow; snow does not lie on it) */
export const LOOK_THING: readonly (readonly [number, number, number, number])[] = LOOK_NAMES.map(n => { const st = STUFFS[n]; return [st.least ?? 0, layerOf(st.foot), st.ground ? 1 : 0, st.sheds ? 1 : 0]; });
/** what a stuff is made of under its coat: (the plinth's layer, the core's layer, −1 = none; and the least wall that shows them, as half its length and
 *  half its height in metres, which is what a box's own record holds) */
export const LOOK_UNDER: readonly (readonly [number, number, number, number])[] = LOOK_NAMES.map(n => { const u = STUFFS[n].under; return u ? [layerOf(u.plinth), layerOf(u.core), u.long / 2, u.tall / 2] : [-1, -1, 0, 0]; });
/** WHAT A HAND MOVES has its arrises rounded and rubbed (a crate, a table, a sill), and a wall, whose boxes meet their neighbours edge to edge, has not: a
 *  thing under `long` on its longer side and under `tall`, and no thinner than `thin` (metres). Not a stuff's: a plain box is rubbed too. */
export const LOOK_HANDLED = { long: 1.3, tall: 1.6, thin: 0.04 } as const;
/** WHERE THE GROUND IS, for what the weather does (metres above the level's floor, which every level lays at 0): a top lower than `top` is ground and not
 *  a table; a face lower than `snow` is the snow field's own; a box whose foot is lower than `foot` stands on the ground and takes its damp. (Use and
 *  dirt, the detail layer's and the look's one answer a mark, keep their own heights in look.wgsl.) */
export const LOOK_GROUND = { top: 0.25, snow: 0.06, foot: 0.12 } as const;
export const LOOK_UNIT: readonly (readonly [number, number, number, number])[] = LOOK_NAMES.map(n => { const u = STUFFS[n].unit; return u ? [u[0], u[1], u[2] ? 1 : 0, u[3]] : [0, 0, 0, 0]; });
/** the layers a layer's own drawing reads beside its own, from its row (`under`, `foot`): a mud wall shows its stone and its brick, and stones come up
 *  through earth at a wall's foot. Whoever makes a layer makes these with it (gbuffer.ts needLook). */
export const LOOK_WITH: Readonly<Partial<Record<LookName, readonly LookName[]>>> = Object.fromEntries(LOOK_NAMES.map(n => { const st = STUFFS[n]; return [n, [...(st.under ? [st.under.plinth, st.under.core] : []), ...(st.foot ? [st.foot] : [])]] as const; }).filter(([, w]) => w.length > 0));

const TAU = Math.PI * 2;
function hash(ix: number, iy: number, seed: number): number {
  let h = (Math.imul(ix | 0, 73856093) ^ Math.imul(iy | 0, 19349663) ^ Math.imul(seed | 0, 83492791)) >>> 0;
  h = (Math.imul(h, 747796405) + 2891336453) >>> 0; h = Math.imul((h >>> ((h >>> 28) + 4)) ^ h, 277803737) >>> 0;
  return (((h >>> 22) ^ h) >>> 0) / 4294967296;
}
const wrap = (i: number, p: number) => ((i % p) + p) % p;
const smooth = (a: number, b: number, x: number) => { const t = Math.min(1, Math.max(0, (x - a) / (b - a))); return t * t * (3 - 2 * t); };
const mix = (a: number, b: number, t: number) => a + (b - a) * t;
const clamp01 = (x: number) => Math.min(1, Math.max(0, x));
/** value noise, 0‥1, with `px` × `py` cells across the tile (u, v in tiles) */
function vn(u: number, v: number, px: number, py: number, seed: number): number {
  const x = u * px, y = v * py, ix = Math.floor(x), iy = Math.floor(y), fx = x - ix, fy = y - iy;
  const wx = fx * fx * (3 - 2 * fx), wy = fy * fy * (3 - 2 * fy);
  const a = hash(wrap(ix, px), wrap(iy, py), seed), b = hash(wrap(ix + 1, px), wrap(iy, py), seed), c = hash(wrap(ix, px), wrap(iy + 1, py), seed), d = hash(wrap(ix + 1, px), wrap(iy + 1, py), seed);
  return mix(mix(a, b, wx), mix(c, d, wx), wy);
}
/** `oct` octaves of it from `p` cells up, 0‥1 */
function fbm(u: number, v: number, p: number, oct: number, seed: number, gain = 0.5): number {
  let s = 0, a = 1, n = 0, q = p;
  for (let o = 0; o < oct; o++) { s += a * vn(u, v, q, q, seed + o * 17); n += a; a *= gain; q *= 2; }
  return s / n;
}
interface Cell { f1: number; f2: number; id: number; dx: number; dy: number }
const cellOut: Cell = { f1: 0, f2: 0, id: 0, dx: 0, dy: 0 };
/** the nearest two feature points of a jittered `p` × `p` lattice: distances in CELLS, the nearest one's hash and the offset to it (cells) */
function cells(u: number, v: number, p: number, seed: number, jitter = 0.85): Cell { return cells2(u, v, p, p, seed, jitter); }
/** the same on a `p` × `q` lattice (a cell wider than tall when p < q) */
function cells2(u: number, v: number, p: number, q: number, seed: number, jitter = 0.85): Cell {
  const x = u * p, y = v * q, ix = Math.floor(x), iy = Math.floor(y);
  let f1 = 9, f2 = 9, id = 0, bx = 0, by = 0;
  for (let j = -1; j <= 1; j++) for (let i = -1; i <= 1; i++) {
    const cx = wrap(ix + i, p), cy = wrap(iy + j, q);
    const px = ix + i + 0.5 + (hash(cx, cy, seed) - 0.5) * jitter, py = iy + j + 0.5 + (hash(cx, cy, seed + 1) - 0.5) * jitter;
    const dx = px - x, dy = py - y, d = Math.hypot(dx, dy);
    if (d < f1) { f2 = f1; f1 = d; id = hash(cx, cy, seed + 2); bx = dx; by = dy; } else if (d < f2) f2 = d;
  }
  cellOut.f1 = f1; cellOut.f2 = f2; cellOut.id = id; cellOut.dx = bx; cellOut.dy = by; return cellOut;
}

/** one texel as a material writes it: height (m), paint multiplier, roughness */
interface Tex { h: number; r: number; g: number; b: number; rough: number }
type Mat = (u: number, v: number, t: Tex) => void;
const tint = (t: Tex, k: number) => { t.r *= k; t.g *= k; t.b *= k; };
const hue = (t: Tex, r: number, g: number, b: number, k: number) => { t.r *= mix(1, r, k); t.g *= mix(1, g, k); t.b *= mix(1, b, k); };

// ---- a mud wall rendered by hand and left to the weather: the render went on in daubs a forearm across, each smoothed with the flat of a hand in its own
// direction (the fingers' grooves still in it, a ridge where one daub laps the next); straw in it; long shrinkage cracks that open into a net where it dried
// fastest, their edges curled; the thin last coat flaked off in plates here and there; pebbles in the mix, and the holes of the ones that fell out
const mud: Mat = (u, v, t) => {
  const lump = fbm(u, v, 5, 4, 11), mid = fbm(u, v, 22, 3, 12);
  const wu = u + 0.07 * (fbm(u, v, 3, 3, 13) - 0.5), wv = v + 0.07 * (fbm(u, v, 4, 3, 14) - 0.5);   // (a daub's outline is a hand's, not a ruler's)
  const d = cells(wu, wv, 5, 15, 0.9), did = d.id, dEdge = d.f2 - d.f1;           // the daubs
  const a = did * 40, ca = Math.cos(a), sa = Math.sin(a), across = (d.dx * sa - d.dy * ca) * 0.4 + 0.05 * (vn(u, v, 6, 6, 16) - 0.5) + 0.25 * (d.dx * ca + d.dy * sa) * (d.dx * ca + d.dy * sa) * 0.4;   // metres across this daub's stroke (the stroke is an arc)
  const finger = 0.5 + 0.5 * Math.cos(across * TAU / 0.021), swept = smooth(0.1, 0.45, dEdge) * smooth(0.45, 0.8, hash(Math.floor(did * 4096), 3, 16)) * (0.5 + 0.5 * vn(u, v, 8, 8, 30));   // four fingers, 2 cm apart; most daubs were wiped flat after, and no stroke runs its whole length
  const lap = smooth(0.12, 0.0, dEdge) * smooth(0.35, 0.7, vn(u, v, 10, 10, 22));       // the ridge where the next daub was pressed over this one
  t.h = 0.026 * (lump - 0.5) + 0.006 * (mid - 0.5) + 0.0016 * (finger - 0.5) * swept + 0.0035 * lap;
  tint(t, 1 + 0.2 * (lump - 0.5) + 0.1 * (mid - 0.5) + 0.07 * (did - 0.5) * smooth(0.0, 0.3, dEdge) + 0.04 * (finger - 0.5) * swept);
  // the last thin coat, gone in plates: a crisp ragged edge, a few millimetres down to a coarser, darker coat
  const fl = fbm(u, v, 4, 4, 23, 0.55) + 0.08 * (vn(u, v, 60, 60, 24) - 0.5), flake = smooth(0.635, 0.65, fl), rim = smooth(0.61, 0.635, fl) * (1 - flake);
  t.h += 0.0015 * rim - flake * (0.0045 + 0.0025 * (fbm(u, v, 48, 2, 25) - 0.5) * 2);
  tint(t, 1 + 0.06 * rim - flake * (0.16 + 0.12 * (vn(u, v, 80, 80, 26) - 0.5)));
  // cracks: one long net (cells a pace across) that shows only where the wall dried hard, and the fine crazing inside it
  const region = smooth(0.5, 0.68, fbm(u, v, 3, 3, 17));
  const big = cells(u + 0.03 * (mid - 0.5), v + 0.03 * (lump - 0.5), 3, 27, 1), bw = big.f2 - big.f1;
  const long = smooth(0.03, 0.006, bw) * smooth(0.25, 0.6, region + 0.3 * vn(u, v, 6, 6, 28)), curl = smooth(0.1, 0.03, bw) * (1 - long) * smooth(0.25, 0.6, region);
  const c = cells(u + 0.02 * (mid - 0.5), v + 0.02 * (lump - 0.5), 7, 18);
  const fine = smooth(0.045, 0.01, c.f2 - c.f1) * smooth(0.6, 0.75, region) * (0.35 + 0.65 * vn(u, v, 11, 11, 21));
  t.h += 0.003 * curl - 0.016 * long - 0.006 * fine; tint(t, 1 - 0.55 * long - 0.38 * fine + 0.05 * curl); t.rough = 0.95 + 0.03 * Math.max(long, fine);
  const s = cells(u, v, 44, 19, 1);                                               // straw: a short pale fibre in about a fifth of the cells, each at its own angle
  if (s.id < 0.2) {
    const b = s.id * 97, cb = Math.cos(b), sb = Math.sin(b), along = s.dx * cb + s.dy * sb, acr = Math.abs(-s.dx * sb + s.dy * cb);
    const f = smooth(0.07, 0.03, acr) * smooth(0.42, 0.3, Math.abs(along)) * (1 - flake);
    t.h += 0.0018 * f; hue(t, 1.32, 1.26, 1.02, f);
  }
  const p = cells(u, v, 19, 20);                                                  // pebbles in the mix: one cell in twelve holds one standing proud, one in twenty the hole of one that fell out
  if (p.id < 0.13) {
    const r = p.f1 + 0.2 * (vn(u, v, 90, 90, 29) - 0.5), f = smooth(0.26, 0.14, r) * smooth(0.0, 0.2, p.f2 - p.f1);
    if (p.id < 0.05) { t.h -= 0.007 * f; tint(t, 1 - 0.3 * f); }
    else { t.h += 0.005 * f * (0.5 + p.id * 2); hue(t, 0.92, 0.98, 1.1, f * 0.9); tint(t, 1 + (p.id - 0.1) * 3.5 * f); if (f > 0) t.rough = mix(t.rough, 0.7, f); }
  }
  hue(t, 0.96, 1.0, 1.06, smooth(0.55, 0.8, mid) * 0.6);                           // paler, dustier on the high spots
};
// ---- render over block: a trowelled coat, hairline crazing, and the places it has let go of the wall and shows what is under it
const plaster: Mat = (u, v, t) => {
  const low = fbm(u, v, 3, 3, 31), wu = u + 0.06 * (fbm(u, v, 4, 2, 32) - 0.5), wv = v + 0.06 * (fbm(u, v, 4, 2, 33) - 0.5);
  const trowel = vn(wu, wv, 5, 38, 34), speck = vn(u, v, 190, 190, 35);
  t.h = 0.005 * (low - 0.5) + 0.0014 * (trowel - 0.5) + 0.0005 * (speck - 0.5);
  tint(t, 1 + 0.1 * (low - 0.5) + 0.05 * (trowel - 0.5) + 0.03 * (speck - 0.5)); t.rough = 0.84 + 0.06 * trowel;
  const c = cells(u + 0.12 * (fbm(u, v, 6, 3, 41) - 0.5), v + 0.12 * (fbm(u, v, 6, 3, 42) - 0.5), 3, 36);
  const hair = smooth(0.02, 0.004, c.f2 - c.f1) * smooth(0.5, 0.7, fbm(u, v, 3, 2, 37));
  t.h -= 0.002 * hair; tint(t, 1 - 0.3 * hair);
  const gone = fbm(u, v, 4, 4, 38) + 0.08 * (vn(u, v, 40, 40, 39) - 0.5);
  const off = smooth(0.635, 0.655, gone), lip = smooth(0.6, 0.635, gone) * (1 - off);
  if (off > 0) {                                                                    // the coat is off: a step down to the rough block face
    const under = fbm(u, v, 60, 3, 40), course = smooth(0.42, 0.5, Math.abs(((v * 10) % 1) - 0.5));
    t.h = mix(t.h, -0.008 + 0.003 * (under - 0.5) - 0.004 * course, off);
    t.r = mix(t.r, 0.7 + 0.2 * under - 0.12 * course, off); t.g = mix(t.g, 0.68 + 0.2 * under - 0.12 * course, off); t.b = mix(t.b, 0.68 + 0.2 * under - 0.12 * course, off);
    t.rough = mix(t.rough, 0.97, off);
  }
  t.h += 0.0012 * lip; tint(t, 1 + 0.07 * lip);                                    // the edge of the coat curls up, and is the palest thing on the wall
};
// ---- concrete block in a running bond: 40 × 20 cm blocks, each its own shade, raked mortar joints, pitted faces, corners knocked off
const block: Mat = (u, v, t) => {
  const row = Math.floor(v * 10), x = u * 5 + 0.5 * (row & 1), col = wrap(Math.floor(x), 5), fx = x - Math.floor(x), fy = v * 10 - row;
  const d = Math.min(Math.min(fx, 1 - fx) * 0.4, Math.min(fy, 1 - fy) * 0.2);       // metres to the block's edge
  const chip = smooth(0.62, 0.9, vn(u, v, 36, 36, 51)) * smooth(0.05, 0.0, d);    // corners and arrises knocked off, here and there
  const joint = smooth(0.011 + 0.02 * chip, 0.005, d);
  const id = hash(col, row, 52), id2 = hash(col, row, 53), pit = vn(u, v, 150, 150, 54), pore = cells(u, v, 64, 55);
  const hole = pore.id < 0.14 ? smooth(0.14 + 0.2 * pore.id * 7, 0.06, pore.f1) : 0;
  t.h = (1 - joint) * (0.0016 * (id - 0.5) + 0.0014 * (pit - 0.5) - 0.0022 * hole + 0.0015 * (fbm(u, v, 12, 2, 56) - 0.5)) - 0.0065 * joint;
  tint(t, 1 + 0.26 * (id - 0.5) + 0.08 * (pit - 0.5) - 0.2 * hole); hue(t, 1.05, 1.0, 0.94, id2);
  if (joint > 0) { const m = vn(u, v, 90, 90, 57); t.r = mix(t.r, 1.12 + 0.14 * m, joint); t.g = mix(t.g, 1.12 + 0.14 * m, joint); t.b = mix(t.b, 1.14 + 0.14 * m, joint); }
  t.rough = mix(0.92 + 0.04 * pit, 0.98, joint);
};
// ---- a yard of beaten earth: hard-packed and smooth where it is walked, stones worked up to the surface in drifts, fine cracks where a puddle dried
const earth: Mat = (u, v, t) => {
  const low = fbm(u, v, 3, 4, 71), mid = fbm(u, v, 18, 3, 72), grit = vn(u, v, 170, 170, 73), pack = smooth(0.4, 0.62, fbm(u, v, 4, 3, 74));
  t.h = 0.012 * (low - 0.5) + 0.0035 * (mid - 0.5) * (1 - 0.6 * pack) + 0.0008 * (grit - 0.5);
  tint(t, 1 + 0.22 * (low - 0.5) + 0.1 * (mid - 0.5) + 0.07 * (grit - 0.5) + 0.07 * pack); t.rough = 0.97 - 0.1 * pack;
  const drift = smooth(0.5, 0.72, fbm(u, v, 5, 3, 75));                             // where the stones gather
  for (let k = 0; k < 2; k++) {
    const s = cells(u, v, k ? 46 : 19, 76 + k * 5), want = k ? 0.05 + 0.34 * drift : 0.02 + 0.2 * drift;
    if (s.id < want) {
      const r = 0.2 + 0.26 * hash(Math.floor(s.id * 9973), k, 77), rr = r * (0.8 + 0.4 * vn(u, v, 70, 70, 81)), dome = smooth(0, 1, 1 - (s.f1 / rr) * (s.f1 / rr)) * smooth(0.0, 0.14, s.f2 - s.f1);
      if (dome > 0) {
        const cellM = LOOK_TILE / (k ? 46 : 19), f = smooth(0.0, 0.35, dome), shade = 0.85 + 0.6 * hash(Math.floor(s.id * 7919), k, 78);
        t.h = Math.max(t.h, t.h * (1 - f) + (0.22 * r * cellM * dome));
        t.r = mix(t.r, 1.15 * shade, f); t.g = mix(t.g, 1.2 * shade, f); t.b = mix(t.b, 1.38 * shade, f); t.rough = mix(t.rough, 0.8, f);
        const sun = clamp01(0.5 - 0.4 * (s.dx + s.dy) / r); tint(t, mix(1, 0.9 + 0.2 * sun, f));   // a stone is lighter on one shoulder: it reads as round with no lamp on it
      }
    }
  }
  const c = cells(u, v, 11, 79), dried = smooth(0.58, 0.72, fbm(u, v, 3, 2, 80)) * pack;
  const crack = smooth(0.035, 0.008, c.f2 - c.f1) * dried * smooth(0.5, 0.75, vn(u, v, 5, 5, 82)); t.h -= 0.002 * crack; tint(t, 1 - 0.22 * crack);
};
// ---- gravel: stones shoulder to shoulder, two sizes, every one its own grey, dark between them
const gravel: Mat = (u, v, t) => {
  let h = -0.006, f = 0, shade = 1, hueK = 0, sh = 0;
  for (let k = 0; k < 2; k++) {
    const p = k ? 58 : 30, s = cells(u, v, p, 91 + k * 7, 0.95), cellM = LOOK_TILE / p;
    const r = 0.5 + 0.22 * s.id, dome = Math.sqrt(Math.max(0, 1 - (s.f1 / r) * (s.f1 / r))) * smooth(0.0, 0.16, s.f2 - s.f1);
    const hh = (k ? 0.36 : 0.5) * cellM * dome - (k ? 0.003 : 0);
    if (hh > h) { h = hh; f = smooth(0, 0.3, dome); shade = 0.68 + 0.75 * hash(Math.floor(s.id * 9973), k, 92); hueK = hash(Math.floor(s.id * 7919), k, 93); sh = clamp01(0.5 - 0.45 * (s.dx + s.dy) / r); }
  }
  t.h = h; const fine = vn(u, v, 200, 200, 94);
  tint(t, mix(0.5, shade * (0.82 + 0.36 * sh), f) * (0.96 + 0.08 * fine)); hue(t, 1.1, 1.0, 0.86, f * hueK * 0.8); hue(t, 0.94, 1.0, 1.12, f * (1 - hueK) * 0.6);
  t.rough = mix(0.97, 0.78, f);
};
// ---- a concrete slab, two metres square: floated and brushed, pores, a stain or two, one crack, a tooled joint at its edge with the arris broken here and there
const slab: Mat = (u, v, t) => {
  const low = fbm(u, v, 4, 4, 111), brush = vn(u + 0.01 * (low - 0.5), v, 6, 150, 112), fine = vn(u, v, 180, 180, 113);
  t.h = 0.002 * (low - 0.5) + 0.0006 * (brush - 0.5) + 0.0004 * (fine - 0.5);
  tint(t, 1 + 0.14 * (low - 0.5) + 0.05 * (brush - 0.5) + 0.04 * (fine - 0.5)); t.rough = 0.8 + 0.08 * brush;
  const p = cells(u, v, 76, 114); if (p.id < 0.22) { const f = smooth(0.28, 0.1, p.f1); t.h -= 0.0018 * f; tint(t, 1 - 0.28 * f); }
  const stain = smooth(0.62, 0.8, fbm(u, v, 3, 4, 115)); tint(t, 1 - 0.16 * stain); t.rough -= 0.22 * stain;   // something was spilt and soaked in: darker, a little glossier
  const c = cells(u, v, 2, 116, 0.6), crack = smooth(0.02, 0.004, (c.f2 - c.f1) + 0.012 * (vn(u, v, 30, 30, 117) - 0.5)) * smooth(0.6, 0.78, vn(u, v, 2, 2, 118));
  t.h -= 0.0025 * crack; tint(t, 1 - 0.45 * crack);
  const d = Math.min(Math.min(u, 1 - u), Math.min(v, 1 - v)) * LOOK_TILE, broke = smooth(0.6, 0.85, vn(u, v, 26, 26, 119));
  const joint = smooth(0.011 + 0.02 * broke, 0.004, d), round = smooth(0.03, 0.011, d) * (1 - joint);
  t.h -= 0.007 * joint + 0.0015 * round; tint(t, 1 - 0.5 * joint + 0.05 * round); t.rough = mix(t.rough, 0.97, joint);
};
// ---- rough-sawn planks left out for years, grain along u: no two one width, none straight-edged (the gaps open and close along a board), each from another
// tree and sitting at its own height, cupped, the soft grain weathered out so the hard grain stands in ridges, the saw's strokes still across some, long
// checks running in from the ends, knots that have shrunk in their holes, big nail heads with the rust run out of them, the butt joints staggered
const PLANK = [0.071, 0.052, 0.066, 0.047, 0.074, 0.058, 0.069, 0.063];            // the widths of eight boards, in tiles (they sum to ½: the set is laid twice, from other trees)
const wood: Mat = (u, v, t) => {
  let y = wrap(Math.floor(v * 2), 2), fv = v * 2 - Math.floor(v * 2), k = 0, y0 = 0;
  const wob = 0.004 * (vn(u, v, 5, 2, 141) - 0.5) + 0.0015 * (vn(u, v, 23, 2, 142) - 0.5);   // an edge is never straight (in tiles)
  fv = wrap(fv * 0.5 + wob, 1) * 2; if (fv >= 1) { fv -= 1; y = 1 - y; }
  for (; k < 7 && fv * 0.5 >= y0 + PLANK[k]; k++) y0 += PLANK[k];
  const p = y * 8 + k, wdt = PLANK[k], fy = (fv * 0.5 - y0) / wdt, wm = wdt * LOOK_TILE;   // which board, 0‥1 across it, its width in metres
  const id = hash(p, 0, 131), id2 = hash(p, 1, 131), id3 = hash(p, 2, 131);
  const off = id * 37, warp = 0.5 * (vn(u + off, v, 3, 16, 132) - 0.5);
  const kn = cells2(u + off, fy * 0.5, 3, 1, 136 + p, 0.9);                       // (knots first: the grain runs round them)
  const ke = kn.id < 0.42 ? Math.hypot(kn.dx * 0.667, kn.dy * 2 * wm) : 9, knot = smooth(0.035 + 0.02 * id3, 0.012, ke), swirl = smooth(0.16, 0.03, ke) * (1 - knot);   // (metres from the knot's heart)
  const ring = vn(u + off, (fy + warp) * (1 - 0.5 * swirl) + 0.5 * swirl * Math.tanh(kn.dy * 24), 2, 9, 133), line = vn(u + off, v, 10, 16 * 34, 134), fib = vn(u + off, v, 60, 16 * 90, 135);
  const hard = Math.pow(Math.abs(Math.sin(ring * 14)), 0.7);                      // the hard grain: what the weather leaves standing
  const grain = 0.5 * hard + 0.32 * line + 0.18 * fib;
  const edge = Math.min(fy, 1 - fy) * wm;                                         // metres to the board's edge
  const open = 0.0035 + 0.006 * vn(u + off, 0.5, 7, 1, 137) * (0.3 + id3);        // the gap's half-width wanders along the board
  const gap = smooth(open + 0.002, open, edge);
  let du = Math.abs(u - id2); du = Math.min(du, 1 - du); const butt = smooth(0.0026, 0.001, du), dm = du * LOOK_TILE;
  const seam = Math.max(gap, butt);
  // checks: splits along the grain, running in from the butt end and dying out
  const cy = 0.25 + 0.5 * id3 + 0.05 * (vn(u + off, 0.3, 9, 1, 138) - 0.5), reach = 0.18 + 0.55 * id, chk = id2 > 0.3 ? smooth(0.014, 0.003, Math.abs(fy - cy) + 0.02 * smooth(0, reach, dm)) * smooth(reach, reach * 0.4, dm) : 0;
  const saw = id3 > 0.55 ? 0.5 + 0.5 * Math.sin((u * LOOK_TILE * 1.0 + fy * wm * 0.35 + 0.01 * Math.sin(fy * 9)) * TAU / 0.03) : 0.5;   // the saw's strokes, three centimetres apart, slanted
  const cup = -Math.cos((fy - 0.5) * Math.PI) * (id - 0.35) * 0.0032;
  t.h = 0.0019 * (hard - 0.5) + 0.0007 * (line - 0.5) + cup + 0.0026 * (id2 - 0.5) + 0.0006 * (saw - 0.5) * (id3 > 0.55 ? 1 : 0) - 0.006 * chk - 0.0035 * knot + 0.0012 * swirl - 0.014 * seam;
  const grey = smooth(0.35, 0.8, id);                                             // how far this board has gone grey
  tint(t, (0.74 + 0.5 * id2) * (1 + 0.42 * (grain - 0.5)) * (1 - 0.1 * (saw - 0.5))); hue(t, 1.1, 1.0, 0.84, 1 - grey); hue(t, 0.86, 0.93, 1.08, grey);
  tint(t, 1 - 0.6 * knot - 0.5 * chk + 0.12 * swirl * Math.sin(ke * 260));
  tint(t, 1 - 0.22 * smooth(0.06, 0.0, dm));                                      // end grain drinks the wet: dark at the butts
  tint(t, 1 - 0.8 * seam); t.rough = mix(0.86 - 0.14 * hard + 0.06 * line, 0.98, Math.max(seam, chk));
  for (const e of [0.016, -0.016]) for (const q of [0.25, 0.75]) {               // two nails each side of a butt joint, the rust run along the grain from each
    let dn = Math.abs(u - id2 - e); dn = Math.min(dn, 1 - dn); const r = Math.hypot(dn * LOOK_TILE, (fy - q) * wm), n = smooth(0.0065, 0.0045, r), run = smooth(0.012, 0.004, Math.abs(fy - q) * wm) * smooth(0.11, 0.0, dn * LOOK_TILE) * (1 - n);
    hue(t, 1.12, 0.78, 0.55, 0.55 * run); tint(t, 1 - 0.2 * run);
    if (n > 0) { t.r = mix(t.r, 0.5, n); t.g = mix(t.g, 0.4, n); t.b = mix(t.b, 0.36, n); t.h += 0.0012 * n - 0.0016 * smooth(0.009, 0.0065, r) * (1 - n); t.rough = mix(t.rough, 0.45, n); }
  }
};
// ---- corrugated sheet, ribs up and down (v): zinc spangle where it is sound, rust that starts in the valleys and at the laps and runs down, dents, a row of screws
const sheet: Mat = (u, v, t) => {
  const ph = u * 27, rib = 0.5 + 0.5 * Math.cos(ph * TAU), crest = Math.pow(rib, 0.7);
  const dent = fbm(u, v, 3, 2, 151), spangle = cells(u, v, 70, 152), run = vn(u, v, 27, 3, 153), patch = fbm(u, v, 4, 4, 154);
  const lapD = Math.min(v, 1 - v), lap = smooth(0.012, 0.004, lapD);                 // sheets lap every two metres
  const rust = smooth(0.66, 0.86, patch + 0.16 * (run - 0.5) + 0.12 * (1 - rib) + 0.45 * smooth(0.14, 0.0, lapD) * run);
  t.h = 0.0095 * crest + 0.004 * (dent - 0.5) + 0.0005 * rust * (vn(u, v, 120, 120, 155) - 0.5) + 0.0012 * lap;
  tint(t, 0.95 + 0.12 * spangle.id + 0.06 * (dent - 0.5)); t.rough = 0.42 + 0.14 * spangle.id;
  const deep = vn(u, v, 60, 60, 156);
  t.r = mix(t.r, 0.5 + 0.2 * deep, rust); t.g = mix(t.g, 0.29 + 0.11 * deep, rust); t.b = mix(t.b, 0.19 + 0.06 * deep, rust); t.rough = mix(t.rough, 0.94, rust);
  tint(t, 1 - 0.25 * lap);
  const sv = Math.abs(v - 0.5), su = Math.abs(((ph / 3 + 0.5) % 1) - 0.5) * 3 / 27;      // a screw on every third crest (27 ribs: nine screws, whole at the seam), half way up the sheet
  const screw = smooth(0.006, 0.003, Math.hypot(su, sv)), washer = smooth(0.011, 0.008, Math.hypot(su, sv)) * (1 - screw);
  t.h += 0.002 * screw + 0.0008 * washer; tint(t, 1 - 0.5 * washer + 0.15 * screw); if (screw + washer > 0) t.rough = mix(t.rough, 0.7, washer);
};
// ---- sackcloth, canvas, a blanket: a plain weave drawn coarse enough to survive the play camera (a thread every 1.6 cm), no thread the weight of its neighbour
// and none of them straight; the cloth itself lies in folds, holds dust in them and is stained
const cloth: Mat = (u, v, t) => {
  const N = 128;
  const x = (u + 0.006 * (vn(u, v, 6, 6, 171) - 0.5)) * N, y = (v + 0.006 * (vn(u, v, 6, 6, 172) - 0.5)) * N;
  const ix = wrap(Math.floor(x), N), iy = wrap(Math.floor(y), N), fx = x - Math.floor(x), fy = y - Math.floor(y);
  const over = ((ix + iy) & 1) === 0;                                   // warp over weft at every other crossing
  const a = Math.sin(fx * Math.PI), b = Math.sin(fy * Math.PI);
  const thick = over ? hash(ix, 0, 173) : hash(0, iy, 174);             // a thread keeps its weight along its whole length: slubs run across the cloth
  const yarn = over ? a * (0.55 + 0.45 * b) : b * (0.55 + 0.45 * a);
  const fold = 0.6 * vn(u, v, 2, 7, 175) + 0.4 * vn(u, v, 5, 13, 176), stain = fbm(u, v, 3, 3, 177);   // it lies in folds that run one way more than the other
  t.h = 0.0015 * yarn * (0.6 + 0.8 * thick) + 0.011 * (fold - 0.5);
  tint(t, (0.82 + 0.3 * thick) * (0.78 + 0.22 * yarn) * (1 + 0.3 * (fold - 0.5)));
  hue(t, 0.88, 0.86, 0.82, smooth(0.5, 0.85, stain) * 0.6);
  t.rough = 0.93 + 0.06 * yarn;
};
// ---- bare iron left out: a skin of rust that blooms in patches and lifts in plates, pits, and bright scratches where the skin is still sound
const metal: Mat = (u, v, t) => {
  const bloom = fbm(u, v, 5, 4, 181), fine = fbm(u, v, 40, 3, 182), pit = cells(u, v, 46, 183), where = fbm(u, v, 6, 2, 187);
  const pits = pit.id < 0.5 * smooth(0.45, 0.7, where) ? smooth(0.3, 0.1, pit.f1) : 0;   // pits come in patches
  const rust = smooth(0.4, 0.64, bloom + 0.25 * (fine - 0.5));
  const plate = cells(u + 0.02 * (fine - 0.5), v + 0.02 * (bloom - 0.5), 11, 186), lift = rust * smooth(0.55, 0.8, where + 0.3 * (plate.id - 0.5)), edge = lift * smooth(0.07, 0.015, plate.f2 - plate.f1);   // where it is worst the skin lifts in plates
  const scratch = Math.max(smooth(0.88, 0.95, vn(u, v, 3, 150, 184)), smooth(0.91, 0.97, vn(u, v, 120, 4, 185))) * (1 - rust);
  t.h = 0.0014 * rust * (fine - 0.5) + 0.0011 * lift * (plate.id - 0.5) - 0.0011 * edge - 0.0013 * pits - 0.0003 * scratch;
  tint(t, 0.86 + 0.34 * (fine - 0.5) + 0.26 * (bloom - 0.5));
  hue(t, 1.28, 0.92, 0.68, rust * (0.5 + 0.5 * fine));
  tint(t, (1 - 0.3 * edge) * (1 - 0.4 * pits) * (1 + 0.5 * scratch));
  t.rough = mix(mix(0.5, 0.97, rust), 0.3, scratch);
};
// ---- old paint on steel or boards (a drum, a gate, a truck's door): dented, chalked by the sun, scuffed in long strokes, and CHIPPED: specks the size of a
// fingernail where it has been knocked, gathered in a few worn places, and here and there a flake a few fingers across with a lifted rim, down to what is
// under it. Never a bare island: at the game's two centimetres a pixel a patch a hand across is a blot on a crate's lid (the maintainer, 11-04, of patches up
// to 36 cm: "splotches on here look a bit weird, overdone").
/** how hard the place has been knocked about, 0‥1: the few places old paint has chipped in */
const paintWorn = (u: number, v: number) => smooth(0.5, 0.72, fbm(u, v, 5, 3, 192));
const paint: Mat = (u, v, t) => {
  const dent = fbm(u, v, 3, 3, 191), fine = vn(u, v, 90, 90, 193), brush = vn(u, v, 4, 150, 195);
  const rag = 0.14 * (fine - 0.5) + 0.12 * (vn(u, v, 37, 37, 199) - 0.5);                   // a broken edge, in cells
  // a chip is a shard: the middle of a cell of a scattered lattice, cut back from the cell's own straight sides (f2 − f1 = how far in from them), its edge broken.
  // Whether a cell has one is asked ONCE, where its own point is (asked at each texel, a chip that straddled the edge of a worn place was sliced along it)
  const fl = cells(u, v, 13, 194, 1.0), fk = fl.id < 0.4 * paintWorn(u + fl.dx / 13, v + fl.dy / 13) ? 0.82 - 0.3 * ((fl.id * 97.13) % 1) : 9, fe = fl.f2 - fl.f1 + rag;   // flakes: 3 to 9 cm across
  const ch = cells(u, v, 46, 200, 1.0), ck = ch.id < 0.03 + 0.45 * paintWorn(u + ch.dx / 46, v + ch.dy / 46) ? 0.8 - 0.45 * ((ch.id * 61.7) % 1) : 9, ce = ch.f2 - ch.f1 + rag;   // chips: 1 to 3 cm
  const gone = Math.max(smooth(fk, fk + 0.03, fe), smooth(ck, ck + 0.1, ce));
  const rim = smooth(fk - 0.07, fk, fe) * (1 - gone);
  const scuff = smooth(0.82, 0.96, vn(u, v, 2, 70, 196)) * (1 - gone);
  const under = vn(u, v, 50, 50, 198);
  t.h = 0.003 * (dent - 0.5) + 0.00025 * (brush - 0.5) - 0.0007 * gone + 0.0004 * rim + 0.0004 * gone * (under - 0.5);
  tint(t, 0.93 + 0.16 * (dent - 0.5) + 0.06 * (brush - 0.5));
  hue(t, 1.1, 1.08, 1.06, smooth(0.4, 0.8, fbm(u, v, 2, 2, 197)) * 0.6);   // chalked pale in patches
  tint(t, 1 + 0.25 * rim + 0.22 * scuff);
  t.r = mix(t.r, 0.62 + 0.2 * under, gone); t.g = mix(t.g, 0.4 + 0.12 * under, gone); t.b = mix(t.b, 0.3 + 0.08 * under, gone);   // (a multiplier of the paint: bare metal comes out dark and warm whatever the paint was)
  t.rough = mix(mix(0.46 + 0.08 * brush, 0.72, scuff), 0.94, gone);
};
// ---- a plinth of field stone, the foot of a mud wall (shaders/look.wgsl lays it by the wall's height, not by a paint): stones the size of a loaf, wider than
// tall, laid dry in courses that were never level and packed with mud that has washed back from the faces; no two one colour, each standing out of the wall
// by its own amount with its own lean, and the high faces worn smooth enough to catch a lamp
const stone: Mat = (u, v, t) => {
  const wu = u + 0.022 * (vn(u, v, 5, 5, 202) - 0.5), wv = v + 0.03 * (vn(u, v, 4, 4, 201) - 0.5);
  const c = cells2(wu, wv, 7, 11, 203, 0.78);
  const key = Math.floor(c.id * 65536), id = c.id, id2 = hash(key, 7, 204), id3 = hash(key, 9, 205);
  const edge = c.f2 - c.f1;                                                        // 0 in the gap between two stones
  const body = smooth(0.035, 0.2 + 0.32 * id2, edge);                              // the face rising out of the gap: some stones flat-faced, some round
  const lean = (c.dx * (id2 - 0.5) + c.dy * (id3 - 0.5)) * 0.03;
  const face = fbm(u, v, 26, 3, 206), chip = smooth(0.62, 0.8, vn(u, v, 34, 34, 207)) * smooth(0.5, 0.15, edge);   // spalls at a stone's edge
  const cleft = id3 < 0.18 ? smooth(0.05, 0.012, Math.abs(c.dx * Math.cos(id * 40) + c.dy * Math.sin(id * 40) + 0.12 * (vn(u, v, 20, 20, 208) - 0.5))) : 0;   // a few are split across
  t.h = body * (0.02 + 0.032 * id3 + lean + 0.0045 * (face - 0.5) - 0.007 * chip - 0.01 * cleft);
  tint(t, mix(0.42, (0.72 + 0.62 * id) * (1 + 0.16 * (face - 0.5)), smooth(0.0, 0.5, body)));   // packed mud in the gaps, in shadow
  hue(t, 0.8, 1.05, 1.55, smooth(0.25, 0.7, id2) * body);                         // grey ones among the brown
  if (id3 > 0.86) hue(t, 1.3, 1.28, 1.2, body); else if (id3 < 0.1) tint(t, mix(1, 0.62, body));   // a pale limestone, a dark slate
  tint(t, 1 - 0.35 * cleft - 0.12 * chip);
  const l = cells(u, v, 13, 209);                                                 // lichen: pale ragged blots on the stones that face the weather
  if (l.id < 0.16 && id2 > 0.35) { const f = smooth(0.34, 0.2, l.f1 + 0.3 * (fbm(u, v, 40, 3, 210) - 0.5)) * body * (0.5 + 0.5 * vn(u, v, 150, 150, 211)); hue(t, 1.22, 1.27, 1.1, f); }
  t.rough = mix(0.97, 0.58 + 0.3 * face + 0.12 * id, body) + 0.1 * chip;
};
// ---- sun-dried mud brick, where a wall's render has let go (shaders/look.wgsl puts it in patches the wall decides): 38 × 12 cm bricks in a bond that was
// never quite regular, the joints washed out a finger deep, every brick its own batch of mud, arrises gone, one in a dozen crumbled back or fallen out
const brick: Mat = (u, v, t) => {
  const rows = 16, cols = 5;
  const y = (v + 0.011 * (vn(u, v, 3, 2, 221) - 0.5)) * rows, row = wrap(Math.floor(y), rows), fy = y - Math.floor(y);
  const x = u * cols + hash(row, 0, 222) * 0.6 + 0.5 * (row & 1), col = wrap(Math.floor(x), cols), fx = x - Math.floor(x);
  const id = hash(col, row, 223), id2 = hash(col, row, 224), id3 = hash(col, row, 225);
  const dx = Math.min(fx, 1 - fx) * 0.4, dy = Math.min(fy, 1 - fy) * 0.125;        // metres to the brick's edges
  const rag = 0.007 * (vn(u, v, 70, 70, 226) - 0.5) + 0.006 * (vn(u, v, 23, 23, 231) - 0.5);
  const d = Math.min(dx, dy) - 0.007 * smooth(0.035, 0.0, Math.max(dx, dy)) + rag;  // (the corners are worn round)
  const joint = smooth(0.017, 0.007, d), arris = smooth(0.0, 0.03, d);
  const grit = fbm(u, v, 44, 3, 227), soft = id3 > 0.78 ? (id3 - 0.78) * 4.5 : 0, gone = id3 < 0.07;
  let h = arris * (0.013 + 0.006 * (id - 0.5)) + 0.003 * (grit - 0.5) - soft * 0.011 * fbm(u, v, 16, 3, 229);
  let shade = (0.8 + 0.4 * id) * (1 + 0.14 * (grit - 0.5));
  if (gone) { h = -0.014 + 0.008 * fbm(u, v, 18, 3, 230); shade = 0.5 + 0.2 * grit; }                // a hole, rubble at the back of it
  t.h = mix(h, -0.002 + 0.002 * (grit - 0.5), joint);
  tint(t, mix(shade, 0.68 + 0.12 * grit, joint)); hue(t, 1.07, 1.0, 0.9, id2); hue(t, 0.94, 0.98, 1.06, smooth(0.55, 0.9, id) * 0.7);
  const s = cells(u, v, 52, 232, 1);                                              // chopped straw in the mud, as in the render
  if (s.id < 0.16 && !gone) { const a = s.id * 97, ca = Math.cos(a), sa = Math.sin(a), f = smooth(0.07, 0.03, Math.abs(-s.dx * sa + s.dy * ca)) * smooth(0.4, 0.28, Math.abs(s.dx * ca + s.dy * sa)) * (1 - joint); t.h += 0.0015 * f; hue(t, 1.3, 1.24, 1.02, f); }
  t.rough = 0.94 + 0.04 * joint;
};
// ==== THE FOUR OFFICE STUFFS (11-04; the maintainer, of a desk and a swivel chair in a dressed office: "nicer materials on the chair and table"). Until them the
// look had one wood (weathered planks with gaps), one cloth (a sack's weave) and one metal (iron gone to rust): a desk drew as a pallet, a chair as a sandbag
// and steel shelving as a wreck. These four are things made in a factory and kept indoors: even, clean, and what little they show is how they were made.
/** a few thin scratches, each a short straight line through a cell's own point at its own angle; `p` cells a tile, `share` of the cells have one. 0‥1.
 *  `wide` = its half width in texels of the full map, never under one: a line thinner than a texel is drawn as a row of beads */
function scratches(u: number, v: number, p: number, seed: number, share: number, wide: number): number {
  const c = cells(u, v, p, seed, 1.0); if (c.id >= share) return 0;
  const half = Math.max(1, wide) * p / LOOK_N;   // (in cells)
  const a = TAU * ((c.id * 53.7) % 1), ca = Math.cos(a), sa = Math.sin(a), across = Math.abs(c.dx * sa - c.dy * ca), along = Math.abs(c.dx * ca + c.dy * sa), len = 0.12 + 0.3 * ((c.id * 17.3) % 1);
  return smooth(half, 0, across) * smooth(len, len * 0.6, along);
}
// ---- veneer: a desk's top, a cabinet's door. A sawn leaf of wood glued flat and lacquered: no boards, no gaps, no knots. Leaves 40 cm wide laid side by side,
// each its own tone and each turned end for end from the last, the grain running the long way in fine straight lines that swell into a slow arch here and
// there, the pores short dark ticks along it. Smooth enough to show a lamp.
const veneer: Mat = (u, v, t) => {
  const leaf = Math.floor(v * 5), fv = v * 5 - leaf, id = hash(leaf, 0, 211), flip = leaf & 1 ? -1 : 1;
  const uu = flip * u + id * 13, arch = 0.035 * (vn(uu, v, 2, 5, 212) - 0.5) + 0.01 * (vn(uu, v, 7, 5, 213) - 0.5);       // the grain's slow wander across the leaf (tiles)
  const ring = Math.sin((fv + arch * 5) * TAU * (9 + 5 * id)), band = 0.5 + 0.5 * ring;                                       // the growth rings, cut along: bands a few centimetres wide
  const line = vn(uu, v, 6, 260, 214), fib = vn(uu, v, 40, 512, 215), pore = smooth(0.78, 0.9, vn(uu, v, 90, 256, 216)) * (0.4 + 0.6 * band);
  const joint = smooth(0.012, 0.004, Math.min(fv, 1 - fv));                                                                   // where two leaves meet: a hair's line, flush
  t.h = 0.00012 * (band - 0.5) - 0.00018 * pore - 0.00025 * joint;
  tint(t, (0.94 + 0.12 * id) * (1 + 0.1 * (band - 0.5) + 0.1 * (line - 0.5) + 0.05 * (fib - 0.5)) * (1 - 0.14 * pore) * (1 - 0.12 * joint));
  hue(t, 1.04, 1.0, 0.93, band * 0.6);                                                                                       // the late wood is the warmer
  t.rough = 0.36 + 0.07 * (line - 0.5) + 0.12 * pore + 0.1 * joint;
};
// ---- moulded plastic: a chair's shell, a bin, a monitor's back. One colour through and through, a fine pebbled skin off the mould that kills a mirror but
// leaves a soft sheen, flow lines so faint they are only a change of sheen, and the marks of use: a few pale hairline scratches and the places hands and
// backs have rubbed smooth.
const plastic: Mat = (u, v, t) => {
  const peb = vn(u, v, 256, 256, 221), peb2 = vn(u, v, 128, 128, 222), flow = fbm(u, v, 2, 2, 223), rub = smooth(0.55, 0.8, fbm(u, v, 3, 3, 224));
  const scr = Math.max(scratches(u, v, 9, 225, 0.3, 1), scratches(u, v, 17, 227, 0.2, 1));
  t.h = 0.00016 * (peb - 0.5) * (1 - 0.7 * rub) + 0.00008 * (peb2 - 0.5) - 0.00012 * scr;
  tint(t, 0.985 + 0.03 * (flow - 0.5) + 0.02 * (peb - 0.5) + 0.1 * scr);
  t.rough = 0.5 + 0.06 * (peb - 0.5) - 0.14 * rub + 0.05 * (flow - 0.5) + 0.2 * scr;
};
// ---- office fabric: a chair's seat and back, a partition's face. A tight flat weave of two yarns a shade apart (a melange: no thread to count from across a
// room, a grain up close), a faint twill running up it, lint and pilling where it is sat on, and no sheen at all.
const fabric: Mat = (u, v, t) => {
  const N = 256, x = u * N, y = v * N, ix = wrap(Math.floor(x), N), iy = wrap(Math.floor(y), N);       // (a yarn is two texels of the full map: its relief is which yarn is on top, not a round back)
  const over = ((ix + iy) & 1) === 0, warp = hash(ix, 0, 231), weft = hash(0, iy, 232);                // a yarn keeps its shade along its whole length
  const shade = over ? warp : weft, up = over ? 1 : 0;
  const twill = 0.5 + 0.5 * Math.sin((x + y) * TAU / 4), heather = vn(u, v, 90, 90, 233), worn = smooth(0.5, 0.8, fbm(u, v, 3, 3, 234));
  const pc = cells(u, v, 60, 235, 1.0), pill = pc.id < 0.1 + 0.25 * worn ? smooth(0.3, 0.1, pc.f1) : 0;   // pills gather where it is sat on
  t.h = 0.00022 * (up - 0.5) + 0.00012 * (twill - 0.5) + 0.0005 * pill;
  tint(t, (0.9 + 0.2 * shade) * (1 + 0.04 * (up - 0.5)) * (1 + 0.05 * (twill - 0.5) + 0.08 * (heather - 0.5)) * (1 + 0.12 * pill) * (1 + 0.04 * worn));
  t.rough = 0.95 + 0.02 * (up - 0.5) - 0.03 * worn;
};
// ---- painted steel, kept: a filing cabinet, shelving, a window's frame, a locker. A baked coat, even to the eye, with the faint orange peel of the spray and
// a satin sheen; pressed flat, so the only relief is the peel. Use shows as a few fine scratches and scuffs paler than the coat and a soft dulling where
// hands go. No rust and no chips: old paint (above) is the coat that has been left out.
const steel: Mat = (u, v, t) => {
  const peel = fbm(u, v, 70, 2, 241), slow = fbm(u, v, 2, 3, 242), dull = smooth(0.55, 0.85, fbm(u, v, 3, 2, 243));
  const scr = Math.max(scratches(u, v, 7, 244, 0.35, 1), scratches(u, v, 13, 246, 0.2, 1)), scuff = smooth(0.86, 0.97, vn(u, v, 3, 60, 248)) * 0.5;
  t.h = 0.00014 * (peel - 0.5) - 0.0001 * scr;
  tint(t, 0.985 + 0.04 * (slow - 0.5) + 0.015 * (peel - 0.5) + 0.12 * scr + 0.06 * scuff);
  t.rough = 0.4 + 0.05 * (peel - 0.5) + 0.1 * dull + 0.2 * scr + 0.12 * scuff;
};
// ---- car paint: a body panel of a car that is driven and washed now and then (11-04; the maintainer, of the rounded cars: "maybe we want a car paint
// material too?" — a truck drew as old paint, chipped and rust-specked all over, a saloon as nothing). An enamel under a clear coat: one colour, the spray's
// orange peel only as a wobble of the shine, and what the road leaves on it: a film of dust in soft patches that dulls the coat where it lies, the faint
// runs a rain leaves down through that dust, hairline marks of the cloth, and a stone chip here and there no bigger than a lentil. THE SHINE IS THE BOX'S
// (its own finish: a roughness of about 0.2, not metal, said by whoever makes the car); this map only says where the coat is clean and where it is dulled.
const carpaint: Mat = (u, v, t) => {
  const peel = fbm(u, v, 64, 2, 251), dust = smooth(0.42, 0.78, fbm(u, v, 3, 4, 252)), run = smooth(0.7, 0.92, vn(u, v, 46, 3, 253)) * dust;   // (runs go up the map: a side panel's map lies along the car and up it)
  const scr = Math.max(scratches(u, v, 6, 254, 0.4, 1), scratches(u, v, 11, 256, 0.25, 1)) * 0.6;
  const ch = cells(u, v, 40, 258, 1.0), chip = ch.id < 0.025 ? smooth(0.16, 0.08, ch.f1) : 0;
  t.h = 0.00008 * (peel - 0.5) - 0.00015 * chip;
  tint(t, (0.99 + 0.012 * (peel - 0.5) + 0.05 * dust * (1 - 0.7 * run) + 0.04 * scr) * (1 - 0.45 * chip));
  t.rough = 0.17 + 0.04 * (peel - 0.5) + 0.26 * dust * (1 - 0.8 * run) + 0.12 * scr + 0.5 * chip;
};
// ---- THE KEPT CITY'S SURFACES (11-04; the maintainer, of a game whose walls he sent pictures of: "it has some very tastefully done high frequency detail but
// overall quite flat", "it's all quite intentionally dressed", "i'm referring specifically to the texture work"). Ten stuffs drawn the way those surfaces are
// made, each ONE colour (the box's) with next to nothing added to it, and all of what it is in its relief, its finish and its PIECES: a coat of gloss paint over
// something rough shows every lump as a glint; a thing cast or laid in pieces shows its seams, the holes its formwork left, and a little dirt where a seam
// holds it. Nothing here is weathered: these are places somebody keeps (no `fades`, no `soaks`). And NOTHING IN A MAP IS BETWEEN A HAND AND A DOOR WIDE BUT
// ITS OWN PIECES: soft tone or sheen inside the two-metre tile is seen to repeat at the game's size.
/** m from a point to the nearest edge of the piece of `name` it is in (the stuff's own `unit`); `pieceOut` holds which piece (its column and course, wrapped)
 *  and where in it (0‥1) */
const pieceOut = { col: 0, row: 0, fx: 0, fy: 0 };
function piece(name: LookName, u: number, v: number): number {
  const U = STUFFS[name].unit!, pu = Math.round(LOOK_TILE / U[0]), pv = Math.round(LOOK_TILE / U[1]);
  const y = v * pv, row = Math.floor(y), x = u * pu + (U[2] && (row & 1) ? 0.5 : 0), col = Math.floor(x);
  pieceOut.col = wrap(col, pu); pieceOut.row = wrap(row, pv); pieceOut.fx = x - col; pieceOut.fy = y - row;
  return Math.min(Math.min(pieceOut.fx, 1 - pieceOut.fx) * U[0], Math.min(pieceOut.fy, 1 - pieceOut.fy) * U[1]);
}
/** soft lumps on a jittered lattice of `p` × `q` cells, a share of the cells holding one: how high the highest lump that reaches this point stands here, 0‥1.
 *  Each lump is asked of its own cell AND the eight round it, so a lump is whole across its cell's edge and two that landed close run together */
function lumps(u: number, v: number, p: number, q: number, seed: number, share: number): number {
  const x = u * p, y = v * q, ix = Math.floor(x), iy = Math.floor(y); let best = 0;
  for (let j = -1; j <= 1; j++) for (let i = -1; i <= 1; i++) {
    const cx = wrap(ix + i, p), cy = wrap(iy + j, q); if (hash(cx, cy, seed + 2) > share) continue;
    const dx = x - (ix + i + hash(cx, cy, seed)), dy = y - (iy + j + hash(cx, cy, seed + 1)), a = TAU * hash(cx, cy, seed + 3), ca = Math.cos(a), sa = Math.sin(a);
    const size = 0.4 + 0.3 * hash(cx, cy, seed + 4), d = Math.hypot((dx * ca + dy * sa) * 0.72, (-dx * sa + dy * ca) * 1.25);   // (longer the way it was dragged; never past a cell and a tenth)
    const t = clamp01(1 - d / size), dome = Math.min(1, 1.5 * t * t * (3 - 2 * t));                                                // a rounded shoulder up to a top pressed nearly flat
    best = Math.max(best, dome * (0.65 + 0.35 * hash(cx, cy, seed + 5)));
  }
  return best;
}
// ---- stucco under gloss paint: a wall, a pillar, a stairwell in one strong colour. Thrown on and pressed back with the flat of a trowel: soft lumps three to
// nine centimetres across, each dragged a little one way, no two alike and none in rows, run together where they landed close; between them the float coat.
// The paint lies thick and smooth on the lumps and thin between them, so every lump carries a glint and the colour itself never changes.
const stucco: Mat = (u, v, t) => {
  const big = lumps(u, v, 22, 22, 301, 0.74), small = lumps(u + 0.013, v, 40, 46, 306, 0.8), top = Math.max(big, small);
  const grit = cells(u, v, 120, 311, 1.0), fine = vn(u, v, 210, 210, 321), drag = vn(u, v, 20, 160, 322);   // (the trowel's drag lines across the pressed tops)
  t.h = 0.0044 * big + 0.0021 * small + 0.0005 * smooth(0.4, 0.15, grit.f1) + 0.00012 * (fine - 0.5) + 0.0002 * top * (drag - 0.5);
  tint(t, 0.98 + 0.035 * top);                                    // (the hollows a breath darker: paint lies thin there. Their shadow is the cavity map's)
  t.rough = 0.6 - 0.26 * top + 0.04 * (fine - 0.5);
};
// ---- a skim of plaster, floated and rolled with white paint: a corridor's walls, its ceiling, a poured floor, a painted pad. Flat to the eye and alive under
// a lamp: the roller's stipple, a grain of soft peaks a few millimetres apart; a swell a hand wide, seen only when a light rakes it; pinholes where air came
// out of the skim. No strokes to count and nothing a tile could be seen to repeat.
const skim: Mat = (u, v, t) => {
  const stip = 0.6 * vn(u, v, 150, 150, 331) + 0.4 * vn(u, v, 290, 290, 332), nap = cells(u, v, 170, 333, 1.0), peak = smooth(0.5, 0.1, nap.f1);
  const swell = 0.6 * vn(u, v, 17, 17, 334) + 0.4 * vn(u, v, 31, 31, 335), pin = cells(u, v, 96, 336, 1.0), hole = pin.id < 0.035 ? smooth(0.22, 0.08, pin.f1) : 0;
  t.h = 0.0005 * (swell - 0.5) + 0.00042 * (stip - 0.5) + 0.00022 * peak - 0.0011 * hole;
  tint(t, (1 + 0.012 * (stip - 0.5)) * (1 - 0.3 * hole));
  t.rough = 0.55 + 0.08 * (stip - 0.5) - 0.05 * peak + 0.3 * hole;
};
// ---- concrete block under gloss paint, laid in courses (40 by 20 cm, every other course half a block along): a service corridor. The block's open face is
// still there under the coats, softened: pores the paint bridged and pores it fell into. The joints are struck back a finger's width and the paint has
// rounded their shoulders, so each course has a line of light along its lower edge and a line of shade above it. One colour, joints and all.
const glossBlock: Mat = (u, v, t) => {
  const d = piece('gloss block', u, v), col = pieceOut.col, row = pieceOut.row;
  const joint = smooth(0.006, 0.0035, d), shoulder = smooth(0.012, 0.006, d) * (1 - joint);
  const pore = cells(u, v, 120, 341, 1.0), open = pore.id < 0.45 ? smooth(0.45, 0.15, pore.f1) * (0.3 + 1.5 * pore.id) : 0, grit = vn(u, v, 200, 200, 342), lump = vn(u, v, 70, 70, 345), peel = fbm(u, v, 60, 2, 343);
  const sit = hash(col, row, 344) - 0.5;                                                     // no two blocks quite flush
  t.h = (1 - joint) * (0.0007 * sit + 0.0004 * (grit - 0.5) + 0.0005 * (lump - 0.5) + 0.00012 * (peel - 0.5) - 0.0008 * open - 0.0016 * shoulder) - 0.0055 * joint;
  tint(t, (1 - 0.16 * joint) * (1 - 0.06 * open) * (0.995 + 0.012 * (peel - 0.5)));
  t.rough = mix(0.36 + 0.08 * (grit - 0.5) + 0.14 * open, 0.6, joint);
};
// ---- concrete cast against boards, painted: a plant room's wall. Every board of the formwork is still there in the wall: twelve and a half centimetres
// high, a metre and more long, butted end to end at no two places alike; each left its grain, each sat a hair proud of or behind its neighbour, and the
// grout that ran between two boards stands as a fine fin. Air caught against the wood left holes, the size of a pea to the size of a thumbnail.
const boardConcrete: Mat = (u, v, t) => {
  const NB = 16, y = v * NB, row = wrap(Math.floor(y), NB), fy = y - Math.floor(y);
  const b0 = 0.15 + 0.7 * hash(row, 0, 351), b1 = (b0 + 0.36 + 0.3 * hash(row, 1, 351)) % 1;   // two butt joints a course, in tiles
  const lo = Math.min(b0, b1), hi = Math.max(b0, b1), seg = u >= lo && u < hi ? 1 : 0;
  let du = Math.min(Math.abs(u - b0), Math.abs(u - b1)); du = Math.min(du, 1 - du);
  const butt = smooth(0.0026, 0.001, du), dv = Math.min(fy, 1 - fy) * LOOK_TILE / NB, fin = smooth(0.004, 0.0012, dv) * (0.4 + 0.6 * vn(u, v, 24, NB, 360));
  const id = hash(row, seg, 352), id2 = hash(row, seg, 353), id3 = hash(row, seg, 349);
  const wav = 0.02 * Math.sin(u * TAU * 2 + id * 40) + 0.012 * Math.sin(u * TAU * 5 + id2 * 30), g = fy + wav * 5;         // the grain wanders along the board
  const ring = 0.5 + 0.5 * Math.cos((g + 0.3 * id3) * TAU * (2.5 + 2 * id3)), grain = 0.45 * Math.pow(ring, 2.5) + 0.35 * vn(u + id, g, 3, 190, 355) + 0.2 * vn(u + id2, g, 12, 380, 356);   // (whole periods along the tile: a board that crosses its edge has no seam there)
  const saw = id3 > 0.6 ? 0.5 + 0.5 * Math.sin(u * TAU * 91 + 14 * fy) : 0.5;                                              // some boards came off the saw rough: its strokes across them, a couple of centimetres apart
  const bug = cells(u, v, 52, 358, 1.0), near = smooth(0.35, 0.05, Math.min(fy, 1 - fy)), hole = bug.id < 0.03 + 0.05 * near ? smooth(0.07 + 1.6 * bug.id, 0.025, bug.f1) : 0, fine = vn(u, v, 230, 230, 359);
  t.h = 0.0018 * (id - 0.5) + 0.0007 * (grain - 0.5) + 0.00035 * (saw - 0.5) + 0.00009 * (fine - 0.5) + 0.0009 * fin * (1 - butt) - 0.0014 * butt - 0.0024 * hole;
  tint(t, (1 + 0.03 * (id2 - 0.5) + 0.03 * (grain - 0.5)) * (1 - 0.08 * butt) * (1 - 0.45 * hole));
  t.rough = 0.42 + 0.14 * (grain - 0.5) + 0.06 * (id - 0.5) + 0.35 * hole;
};
// ---- concrete cast in panels, a metre by half a metre, every other course half a panel along: the white walls of a big hall, the plant on a roof. Each panel
// was poured on another day and is a shade its own (the shader's: `unit`); its edges are a struck joint with a small chamfer; six tie holes, three over three, are
// where it was lifted and fixed. The face is fine and close, with a few air holes. What dirt there is has a reason: it sits in the joints, a little creeps
// onto the face from them, and the rain has drawn faint lines down from a joint's lower lip.
const castPanel: Mat = (u, v, t) => {
  const d = piece('cast panel', u, v), col = pieceOut.col, row = pieceOut.row, fx = pieceOut.fx, fy = pieceOut.fy;
  const joint = smooth(0.006, 0.002, d), cham = smooth(0.014, 0.006, d) * (1 - joint);
  // (six tie holes a panel, three over three, each 4 cm across with its plug set back and a raised rim: at the game's height a pixel is a centimetre and a
  // half, and the first pass's four 2 cm holes were specks. His pictures' cast walls are read by their holes before anything else)
  const hx = Math.min(Math.abs(fx - 0.09), Math.abs(fx - 0.5), Math.abs(fx - 0.91)) * 1.0, hy = Math.min(fy, 1 - fy) * 0.5 - 0.075, hd = Math.hypot(hx, hy), lift = smooth(0.021, 0.014, hd), lip = smooth(0.03, 0.022, hd) * (1 - lift);
  const sit = hash(col, row, 361), agg = vn(u, v, 200, 200, 363), agg2 = vn(u, v, 90, 90, 364);
  const bug = cells(u, v, 46, 365, 1.0), hole = bug.id < 0.05 ? smooth(0.08 + 2.4 * bug.id, 0.025, bug.f1) : 0;
  const creep = smooth(0.06, 0.0, d) * (0.5 + 0.5 * vn(u, v, 40, 40, 366)), streak = smooth(0.25, 0.0, 1 - fy) * smooth(0.55, 0.85, vn(u, v, 70, 4, 367)) * (1 - joint);   // under a joint's lower lip (fy near 1 = the top of a panel)
  t.h = (1 - joint) * (0.0005 * (sit - 0.5) + 0.00016 * (agg - 0.5) - 0.0012 * cham - 0.0016 * hole + 0.0004 * lip) - 0.004 * joint - 0.006 * lift;
  tint(t, (1 + 0.03 * (agg - 0.5) + 0.02 * (agg2 - 0.5)) * (1 - 0.34 * joint) * (1 - 0.62 * lift) * (1 + 0.05 * lip) * (1 - 0.3 * hole) * (1 - 0.07 * creep) * (1 - 0.045 * streak));
  t.rough = mix(0.68 + 0.08 * (agg - 0.5), 0.92, Math.max(joint, lift));
};
// ---- pavers on a roof, a floor of tiles: half a metre square, laid straight. Pale cast stone with its aggregate ground flat and showing as fine specks; a
// narrow dark joint; edges eased so each tile is outlined by a hairline of light; no two tiles quite level (their shade is the shader's: `unit`), and a
// little dust swept into the joints and onto the tile's rim.
const paver: Mat = (u, v, t) => {
  const d = piece('paver', u, v), col = pieceOut.col, row = pieceOut.row, fx = pieceOut.fx, fy = pieceOut.fy;
  const joint = smooth(0.0045, 0.0015, d), ease = smooth(0.012, 0.0045, d) * (1 - joint);
  const sit = hash(col, row, 371), tx = hash(col, row, 372) - 0.5, ty = hash(col, row, 373) - 0.5;
  const sp = cells(u, v, 128, 374, 1.0), speck = sp.id < 0.5 ? smooth(0.42, 0.2, sp.f1) * (sp.id < 0.22 ? -1 : 0.7) : 0, fine = vn(u, v, 256, 256, 375), dust = smooth(0.05, 0.0, d) * (0.4 + 0.6 * vn(u, v, 50, 50, 376));
  t.h = (1 - joint) * (0.0005 * (sit - 0.5) + 0.0009 * (tx * (fx - 0.5) + ty * (fy - 0.5)) + 0.00006 * (fine - 0.5) - 0.0011 * ease) - 0.0028 * joint;
  tint(t, (1 + 0.06 * speck + 0.015 * (fine - 0.5)) * (1 - 0.42 * joint) * (1 - 0.06 * dust));
  t.rough = mix(0.46 + 0.1 * Math.abs(speck) + 0.05 * (fine - 0.5) + 0.12 * dust, 0.9, joint);
};
// ---- brushed steel: a door's kick plate, a cabinet, a hatch. Bare, bright, and brushed one way (along u): a grain of lines too fine to count that turns a
// lamp into a streak across it. It is touched and kicked, so the brushing is crossed by a few longer scratches and dulled in smears a hand wide where hands
// and boots go. No rust, no paint.
const brushed: Mat = (u, v, t) => {
  const g = 0.55 * vn(u, v, 5, 256, 381) + 0.3 * vn(u + 0.3, v, 11, 512, 382) + 0.15 * vn(u, v, 40, 512, 383), smear = smooth(0.55, 0.85, vn(u, v, 23, 9, 384)), smear2 = smooth(0.6, 0.9, vn(u, v, 31, 14, 385));   // (smears a hand wide and no wider: anything broader would be seen to repeat)
  const scr = Math.max(scratches(u, v, 6, 386, 0.45, 1), scratches(u, v, 13, 388, 0.3, 1));
  t.h = 0.00011 * (g - 0.5) - 0.00016 * scr;
  tint(t, 0.99 + 0.05 * (g - 0.5) - 0.015 * smear + 0.08 * scr);
  t.rough = 0.3 + 0.12 * (g - 0.5) + 0.1 * smear + 0.06 * smear2 + 0.15 * scr;
};
// ---- a louvre: the grille of an air handler, the vent in a door. Pressed blades six centimetres apart running along u (thirty-two to the tile: sixteen
// texels each, so no blade's gap falls between texels), each sloping out and down to a rolled lip, the gap under it black.
const louvre: Mat = (u, v, t) => {
  const NS = 32, y = v * NS, fy = y - Math.floor(y), peel = fbm(u, v, 60, 2, 391);
  const blade = smooth(0.0, 0.2, fy) * (1 - smooth(0.2, 1.0, fy) * 0.9), gap = smooth(0.2, 0.1, fy), lip = smooth(0.14, 0.22, fy) * smooth(0.34, 0.24, fy);
  t.h = 0.012 * blade + 0.0015 * lip - 0.004 * gap + 0.0001 * (peel - 0.5);
  tint(t, (1 - 0.72 * gap) * (0.99 + 0.02 * (peel - 0.5)));
  t.rough = mix(0.42 + 0.05 * (peel - 0.5), 0.9, gap);
};
// ---- enamel: gloss paint on something smooth and kept. A pipe, a rail, a line painted on a floor, a machine's case, a door. One colour and the spray's or
// the brush's orange peel as a wobble of the shine. Nothing else: no dust, no chips, no rust (old `paint` is the coat left out; `carpaint` the one the road dusts).
// (His pictures' pipes and doors do carry a chip or two. Not here: a chip in this map would come round every two metres along a painted line.)
const enamel: Mat = (u, v, t) => {
  const peel = fbm(u, v, 70, 2, 395), fine = vn(u, v, 256, 256, 396);
  t.h = 0.00012 * (peel - 0.5) + 0.00003 * (fine - 0.5);
  tint(t, 0.995 + 0.01 * (peel - 0.5));
  t.rough = 0.3 + 0.05 * (peel - 0.5) + 0.02 * (fine - 0.5);
};
// ---- carpet: a kept floor's loop pile, one dye. Tufts a centimetre across in close rows, each a soft round back, a yarn or two a shade off the rest.
const carpet: Mat = (u, v, t) => {
  const c = cells2(u, v, 170, 170, 398, 0.5), tuft = smooth(0.62, 0.1, c.f1), fine = vn(u, v, 256, 256, 399);
  t.h = 0.0007 * tuft + 0.00008 * (fine - 0.5);
  tint(t, (0.93 + 0.14 * c.id) * (0.9 + 0.1 * tuft));
  t.rough = 0.95 + 0.03 * (fine - 0.5);
};
// ---- tarmac: rolled asphalt, years down. Black binder with stone chips a finger's width across rolled flat into it; the binder has worn off the tops of
// some, and each of those shows its own grey; the fines between are darker and a hair lower; a slow mottle where the roller's passes and the weather left it
// paler or darker; a chip plucked out here and there (a pit), and a hairline craze where the base under it moved. No wheel line and no patch: those are a
// level's own marks, laid where its traffic goes, and a two-metre map would print them every two metres.
const tarmac: Mat = (u, v, t) => {
  const low = fbm(u, v, 3, 4, 411), fine = vn(u, v, 230, 230, 413);
  const pit = cells(u, v, 40, 421), gone = pit.id < 0.07 ? smooth(0.2, 0.06, pit.f1) : 0;   // a chip plucked out: the hole has no chip in it
  let h = 0, f = 0, shade = 1, hueK = 0;
  for (let k = 0; k < 2; k++) {                                                     // the chips: two sizes, flat on top, a rounded shoulder
    const p = k ? 150 : 96, s = cells(u, v, p, 414 + k * 5, 0.9), cellM = LOOK_TILE / p, r = 0.36 + 0.2 * s.id;
    const top = smooth(r, r * 0.6, s.f1) * smooth(0.0, 0.12, s.f2 - s.f1), hk = 0.05 * cellM * top;
    if (hk <= h) continue;                                                          // the chip that stands highest here is the one that is seen, worn or not
    h = hk;
    // whether its top is worn is the CHIP's, asked where the chip lies (a drift of wear a stride across), so no chip is half worn
    const id = Math.floor(s.id * 5003), worn = 0.12 + 0.3 * smooth(0.3, 0.75, fbm(u + s.dx / p, v + s.dy / p, 14, 3, 412));
    if (hash(id, k, 417) < worn) { f = top; shade = 0.8 + 0.5 * hash(Math.floor(s.id * 9973), k, 415); hueK = hash(Math.floor(s.id * 7919), k, 416); } else f = 0;
  }
  h *= 1 - gone; f *= 1 - gone;
  t.h = h + 0.0015 * (low - 0.5) + 0.0002 * (fine - 0.5) - 0.003 * gone;
  tint(t, mix(0.9, 1.3 * shade, f) * (1 + 0.18 * (low - 0.5)) * (0.95 + 0.1 * fine) * (1 - 0.35 * gone));
  hue(t, 1.08, 1.0, 0.88, f * hueK * 0.6); hue(t, 0.94, 1.0, 1.1, f * (1 - hueK) * 0.4);
  t.rough = mix(mix(0.9, 0.74, f), 0.95, gone);                                     // a worn chip's top is the smoothest thing in it; the binder and the fines are matt
  const c = cells(u, v, 5, 422, 0.7), craze = smooth(0.02, 0.005, (c.f2 - c.f1) + 0.01 * (vn(u, v, 40, 40, 423) - 0.5)) * smooth(0.55, 0.75, fbm(u, v, 2, 2, 424));
  t.h -= 0.002 * craze; tint(t, 1 - 0.4 * craze);
};
// ---- grass: a lawn nobody waters, seen from a roof's height. Blades a finger long, four layers of them lying over one another, every blade its own green and
// its own way of leaning about the way its patch leans (the patches lean every way, and swirl where two meet); thicker and longer in drifts a stride across
// and thin between, where the thatch shows; a tenth of it dried to straw, more in drifts of their own; and places where none took and the earth shows, with
// ragged edges. A blade is never drawn thinner than a texel and a third of this map (it would come out a dotted line and its normals noise). Whether a
// blade is there, and whether it is straw, is asked of the drifts at the texel, not at the blade: a blade in fifty is cut short where a drift ends.
const grass: Mat = (u, v, t) => {
  const low = fbm(u, v, 3, 4, 431), lush = smooth(0.3, 0.7, fbm(u, v, 9, 3, 432)), dry = smooth(0.35, 0.85, fbm(u, v, 4, 3, 433)), fine = vn(u, v, 240, 240, 434);
  const bare = smooth(0.76, 0.86, fbm(u, v, 6, 4, 435) + 0.14 * (vn(u, v, 60, 60, 436) - 0.5)), lie = Math.atan2(vn(u, v, 3, 3, 438) - 0.5, vn(u, v, 3, 3, 437) - 0.5);
  let h = 0, f = 0, green = 1, tip = 0, straw = 0;
  for (let k = 0; k < 4; k++) {
    const p = 44 + 11 * k, s = cells(u, v, p, 440 + 7 * k, 1.0), id = Math.floor(s.id * 9973);
    if (hash(id, k, 441) < 0.75 * bare || hash(id, k, 447) < 0.4 * (1 - lush)) continue;   // none took here, or it is thin between the drifts
    const th = lie + 1.6 * (hash(id, k, 442) - 0.5), cx = Math.cos(th), sy = Math.sin(th);
    const across = -s.dx * sy + s.dy * cx, along = s.dx * cx + s.dy * sy, len = 0.38 + 0.26 * lush * hash(id, k, 443);   // from the blade's own middle, in cells: periodic because the cells are
    const wide = Math.max(0.11, 1.3 * p / LOOK_N), m = smooth(wide, 0.3 * wide, Math.abs(across)) * smooth(len, 0.5 * len, Math.abs(along));
    const hh = (0.012 + 0.02 * lush) * (0.5 + 0.5 * hash(id, k, 444)) * m * (0.6 + 0.4 * k / 3);   // the later layers lie over the earlier
    if (hh > h) { h = hh; f = m; green = 0.78 + 0.32 * hash(id, k, 445); tip = smooth(-len, len, along); straw = hash(id, k, 446) < 0.08 + 0.55 * dry ? 1 : 0; }
  }
  t.h = h + 0.006 * (low - 0.5) - 0.004 * bare;
  tint(t, mix(0.5, green * (0.88 + 0.16 * tip), f) * (1 + 0.26 * (low - 0.5)) * (0.96 + 0.08 * fine) * (0.8 + 0.3 * lush));
  hue(t, 1.22, 1.08, 0.82, 0.6 * straw * f);
  const soil = bare * (1 - f); t.r = mix(t.r, 0.74, 0.85 * soil); t.g = mix(t.g, 0.42, 0.85 * soil); t.b = mix(t.b, 0.4, 0.85 * soil);   // (on a green paint: a dull brown, darker than the grass round it)
  t.rough = 0.96 - 0.05 * f;
};
const MATS: Mat[] = [mud, plaster, block, earth, gravel, slab, wood, sheet, cloth, metal, paint, veneer, plastic, fabric, steel, carpaint, stucco, skim, glossBlock, boardConcrete, castPanel, paver, brushed, louvre, enamel, carpet, tarmac, grass, stone, brick];
const RELIEF: readonly number[] = LOOK_NAMES.map(n => STUFFS[n].relief);

/** the layers a box's own paint is drawn as (a class of look.wgsl); the rest are what a wall is made of UNDER its paint (the plinth's stone, the brick behind fallen render) */
export const LOOK_PAINTED = LOOK_NAMES.indexOf('field stone');
/** A STUFF'S NUMBER BY ITS NAME (look.wgsl's classes; the box's word carries it): 0 for a name that is no stuff (and for the two masonry layers, which no box is) */
export function lookStuffOf(name: string | undefined): number { const i = (LOOK_NAMES as readonly string[]).indexOf(name ?? ''); return i >= 0 && i < LOOK_PAINTED ? i + 1 : 0; }
/** how a colour's value is weighed (linear rgb) */
const lum = (r: number, g: number, b: number) => 0.2126 * r + 0.7152 * g + 0.0722 * b;
/** one layer's two maps at full size (N² × 4 bytes each).
 *  A MAP VARIES ITS PAINT AND NEVER MOVES ITS VALUE (`even`, the page's default; STYLE.md, Surfaces). Two things in a map multiply the paint, and each is brought to a
 *  mean of 1 over the tile, the second weighed by the first, so that the two together are the paint's value however far the panel's Cavities row is turned:
 *    the multiplier (A's rgb)  × one number a layer, so its mean VALUE is 1. One number and not one a channel: a layer keeps the hue its material gave it (the rust
 *                              cast over bare metal, the warmth of boards), which one number a channel would turn into its opposite on every clean texel. The two
 *                              masonry layers are another stuff than the paint over them and keep their own colour.
 *    the hollows (B's alpha)   the shader darkens a hollow by 0.5 + this. It was 0.5 + ½ of "how open" (never over 1: a hollow darkened and nothing lightened, so
 *                              gravel drew a tenth under its paint); now ½ of how open plus what puts the mean at ½, so the high places are that much paler.
 *                              The mean is weighed by the multiplier's value: a material whose high places are also its pale ones (worn boards) drew 3 % over
 *                              its paint with each evened apart.
 *  The level chose the paint's value, the rays bounce it and a man is seen against it; cloth drawn 15 % under its paint made the picture and the light disagree about
 *  the same sack. `even` false = the layer as its material wrote it, the picture as it was before 11-01 (`?look=even:0`, read when the page boots).
 *  `tone` = the multiplier's mean value before it was evened (1 when it was not), `hollow` = what was added to the hollows: who was tuned against the layer as it
 *  fell (the men's cloth, look.wgsl lookMan) takes both back out. */
export interface LookMaps { a: Uint8Array; b: Uint8Array; tone: number; hollow: number }
export function lookLayer(layer: number, N = LOOK_N, even = true): LookMaps { const it = lookLayerSteps(layer, N, even); for (;;) { const r = it.next(); if (r.done) return r.value; } }
/** the same maps, made `sliceMs` at a time with `breath` awaited between the pieces (a turn of the event loop: a frame is drawn, a pipeline lands). The arithmetic
 *  and its order are lookLayer's own (one body, below): the maps are the same to the byte however they were cut up. */
export async function lookLayerSliced(layer: number, N: number, even: boolean, sliceMs: number, breath: () => Promise<void>): Promise<LookMaps> {
  const it = lookLayerSteps(layer, N, even), now = () => typeof performance !== 'undefined' ? performance.now() : 0; let t0 = now();
  for (;;) { const r = it.next(); if (r.done) return r.value; if (now() - t0 >= sliceMs) { await breath(); t0 = now(); } }
}
/** lookLayer's body, stopping after every row of the material's own arithmetic (the long part) and between the passes after it */
function* lookLayerSteps(layer: number, N: number, even: boolean): Generator<void, LookMaps, void> {
  const mat = MATS[layer], H = new Float32Array(N * N), a = new Uint8Array(N * N * 4), b = new Uint8Array(N * N * 4), R = new Float32Array(N * N), P = new Float32Array(N * N * 3);
  const t: Tex = { h: 0, r: 1, g: 1, b: 1, rough: 1 };
  let lo = 1e9, hi = -1e9, sum = 0;
  for (let y = 0; y < N; y++) { if (y) yield; for (let x = 0; x < N; x++) {
    t.h = 0; t.r = t.g = t.b = 1; t.rough = LOOK_ROUGH[layer];
    mat((x + 0.5) / N, (y + 0.5) / N, t);
    const i = y * N + x; H[i] = t.h; R[i] = t.rough; if (t.h < lo) lo = t.h; if (t.h > hi) hi = t.h;
    const r = Math.min(2, Math.max(0, t.r)), g = Math.min(2, Math.max(0, t.g)), bl = Math.min(2, Math.max(0, t.b));   // (what the map can hold: paint × 0‥2)
    P[i * 3] = r; P[i * 3 + 1] = g; P[i * 3 + 2] = bl; sum += lum(r, g, bl);
  } }
  yield;
  const mean = sum / (N * N), tone = even && layer < LOOK_PAINTED && mean > 0 ? mean : 1, kv = 1 / tone;
  for (let i = 0; i < N * N * 3; i++) a[(i / 3 | 0) * 4 + i % 3] = Math.round(255 * clamp01(P[i] * kv * 0.5));
  // cavity: how far a texel sits under the mean of its neighbourhood (a box blur of ±R texels, periodic, two passes)
  const rad = Math.max(2, Math.round(N / 85)), tmp = new Float32Array(N * N), blur = new Float32Array(N * N), w = 1 / (2 * rad + 1);
  for (let y = 0; y < N; y++) { let s = 0; for (let k = -rad; k <= rad; k++) s += H[y * N + wrap(k, N)]; for (let x = 0; x < N; x++) { tmp[y * N + x] = s * w; s += H[y * N + wrap(x + rad + 1, N)] - H[y * N + wrap(x - rad, N)]; } }
  for (let x = 0; x < N; x++) { let s = 0; for (let k = -rad; k <= rad; k++) s += tmp[wrap(k, N) * N + x]; for (let y = 0; y < N; y++) { blur[y * N + x] = s * w; s += tmp[wrap(y + rad + 1, N) * N + x] - tmp[wrap(y - rad, N) * N + x]; } }
  const open = blur; let openSum = 0;   // (in place: how open each texel is, 0‥1, 1 = not in a hollow)
  for (let i = 0; i < N * N; i++) { open[i] = clamp01(1 - (blur[i] - H[i]) * 160); openSum += open[i] * lum(P[i * 3], P[i * 3 + 1], P[i * 3 + 2]); }
  const hollow = even && sum > 0 ? 0.5 - 0.5 * openSum / sum : 0;   // (weighed by the paint: where a material's high places are also its pale ones, as on worn wood, an even count of texels left the two together 3 % bright)
  yield;
  const texel = LOOK_TILE / N, k = RELIEF[layer] / (2 * texel), span = Math.max(hi - lo, 1e-6);
  for (let y = 0; y < N; y++) for (let x = 0; x < N; x++) {
    const i = y * N + x;
    const sx = (H[y * N + wrap(x + 1, N)] - H[y * N + wrap(x - 1, N)]) * k, sy = (H[wrap(y + 1, N) * N + x] - H[wrap(y - 1, N) * N + x]) * k, il = 1 / Math.hypot(sx, sy, 1);
    a[i * 4 + 3] = Math.round(255 * (H[i] - lo) / span);
    b[i * 4] = Math.round(255 * (0.5 - 0.5 * sx * il)); b[i * 4 + 1] = Math.round(255 * (0.5 - 0.5 * sy * il));
    b[i * 4 + 2] = Math.round(255 * clamp01(R[i])); b[i * 4 + 3] = Math.round(255 * clamp01(0.5 * open[i] + hollow));
  }
  return { a, b, tone, hollow };
}
/** the next mip of one map (a 2 × 2 mean; for B the normal is left SHORT where its four texels disagree, and that shortfall is put into the roughness: relief too fine
 *  to draw at this size turns into a duller surface instead of a shimmer) */
export function lookMip(src: Uint8Array, n: number, isB: boolean): Uint8Array {
  const m = n >> 1, out = new Uint8Array(m * m * 4);
  for (let y = 0; y < m; y++) for (let x = 0; x < m; x++) {
    const o = (y * m + x) * 4, i0 = ((2 * y) * n + 2 * x) * 4, i1 = i0 + 4, i2 = i0 + n * 4, i3 = i2 + 4;
    for (let c = 0; c < 4; c++) out[o + c] = (src[i0 + c] + src[i1 + c] + src[i2 + c] + src[i3 + c] + 2) >> 2;
    if (isB) {
      let len = 0;
      for (const i of [i0, i1, i2, i3]) { const nx = src[i] / 127.5 - 1, ny = src[i + 1] / 127.5 - 1; len += Math.sqrt(Math.max(0, 1 - nx * nx - ny * ny)); }
      const mx = out[o] / 127.5 - 1, my = out[o + 1] / 127.5 - 1, flat = Math.sqrt(Math.max(0, 1 - mx * mx - my * my)) - len / 4;   // > 0 where the four normals fan out
      out[o + 2] = Math.min(255, out[o + 2] + Math.round(255 * Math.min(0.35, 2.2 * Math.max(0, flat))));
    }
  }
  return out;
}
