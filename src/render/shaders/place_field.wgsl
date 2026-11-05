// THE LOOK PROTOTYPE — THE PLACE FIELD (render/placeField.ts has the why; composite.wgsl lookExpoGain is its reader). One number a cell of the level's floor
// plan: log2 of the light that falls on the floor of the PLACE, where a place is what walls, windows and shut doors enclose. Each free cell is pulled toward what the radiance
// cache holds over it (the probes it can see) and toward its free neighbours, hard: the answer is the cache's light smoothed over PLACE.reach metres but never through a
// wall or a doorway, so a room has one level from its window to its far end and the yard outside it has another.
// The solve is red-black successive over-relaxation in place: a sweep takes the cells of one colour, each reading its four neighbours of the other.
struct PlaceU {
  parity: u32, snap: u32, kappa: f32, omega: f32,
  sam: vec2f, eyeOpen: f32, eyeClose: f32,
  y: f32, dark: f32, spare: f32, skyEase: f32,
  skyShare: f32, near: f32, nearDoor: f32, pad2: f32,
};
const PLACE_W: i32 = 100;
const PLACE_H: i32 = 70;
const PLACE_SOLID: f32 = -1000.0;
const PLACE_NEAR: i32 = PLACE_W * PLACE_H + 4;   // where the second block begins: after the cells and their four words (render/placeField.ts PLACE_NEAR)
const DOOR_STEPS: f32 = 31.0;      // a doorway's openness as the map carries it (scene/placePlan.ts DOOR_STEPS, DOOR_PER_STOP: lookexpo-check compares)
const DOOR_PER_STOP: f32 = 6.0;
@group(0) @binding(0) var<uniform> U: PlaceU;
@group(0) @binding(1) var diceTex: texture_3d<f32>;
@group(0) @binding(3) var maskTex: texture_2d<u32>;
@group(0) @binding(4) var<storage, read_write> field: array<f32>;
@group(0) @binding(6) var<storage, read_write> reads: array<vec2f>;   // what the cache holds over each cell this frame (placeRead), which every sweep of the frame then takes

fn maskAt(c: vec2i) -> u32 { if (c.x < 0 || c.y < 0 || c.x >= PLACE_W || c.y >= PLACE_H) { return 255u; } return textureLoad(maskTex, c, 0).r; }
/** a cell's level, or PLACE_SOLID: off the map, in a wall, or free since a moment ago with no answer yet */
fn cellAt(c: vec2i) -> f32 { if (maskAt(c) == 255u) { return PLACE_SOLID; } return field[c.y * PLACE_W + c.x]; }
/** WHAT ONE NEIGHBOUR HANDS A CELL whose own map value is `m`, read once: xy = its pull (its level × the pull's weight, the weight), z = THE BRIGHTEST PLACE A STRIDE
 *  AWAY as it hands it on (under −500 = it has none to hand).
 *  THE PULL is between two cells of one place and never through a doorway: A DOORWAY'S CELL HOLDS NO LEVEL (placeSweep; the map's low bits 1,
 *  scene/placePlan.ts placeMask), so it hands none on. Two rooms through an open door are two places, each with the level its own floor holds, and what one
 *  takes from the other is the light that comes through the door, which is in its reading already. (Until 11-04 a doorway's cell held a level of its own
 *  and passed a hundredth of the pull: a threshold was then a third place with a number between its two rooms', and it drew as a dark band across the
 *  doorway from the dark side and a pale one from the bright side.)
 *  A DOOR OPENS BY DEGREES (doorShut: the map's bits above, scene/placePlan.ts placeMask `steps`): the brightest place a stride away is handed out of the
 *  doorway DIMMER BY THE SHARE OF THE DOORWAY THAT IS SHUT, ONCE, on the hand that leaves the
 *  door's line (two doors whose lines touch in a row are charged as the more shut of the two; a stop for each halving: a door a quarter open passes a quarter of the yard, so the room is held to the yard's level less two stops),
 *  on top of U.nearDoor. So the floor inside a door, which gets that share of the yard's light, is shown by the level of that share, and what a room
 *  takes from the yard comes in with the swing with no number of its own.
 *  THE BRIGHTEST PLACE A STRIDE AWAY (the field's second block; place_common.wgsl lookGainsOwn is its reader): the neighbour's own answer, U.near stops
 *  dimmer for the cell of floor between them and only U.nearDoor dimmer when either is a doorway's cell: so the first cell of a room inside its door holds
 *  the yard's level all but whole, and each cell further in a step less, never through a wall. Each cell then keeps the most of its own level and what
 *  its four neighbours hand it (placeSweep). EVERY STEP LOSES SOMETHING, a doorway's too: two cells that handed each other their answers whole would keep
 *  the brightest either ever held (a yard gone dim at dusk, a lamp put out), and the bound would stand open nine cells into the room for good. With a loss
 *  on every step a stale answer falls by at least U.nearDoor a sweep, and the block follows the levels both ways. */
fn hand(c: vec2i, m: u32) -> vec3f {
  let mn = maskAt(c); if (mn == 255u) { return vec3f(0.0, 0.0, PLACE_SOLID); }
  let i = c.y * PLACE_W + c.x; let door = (m & 3u) == 1u || (mn & 3u) == 1u;
  let v = field[i]; let nv = field[PLACE_NEAR + i];
  var o = vec3f(0.0, 0.0, PLACE_SOLID);
  let sm = doorShut(m); let sn = doorShut(mn);   // (stops under wide open; 0 = wide, or the cell is no doorway's)
  if (v > -500.0) { o = vec3f(v * U.kappa, U.kappa, PLACE_SOLID); }
  if (nv > -500.0) { o.z = nv - select(U.near, U.nearDoor + max(0.0, sn - sm), door); }   // (… + what the giver's door is shut by over the taker's: 0 inside one door's line and into it, the door's stops out of it)
  return o;
}
/** how far under wide open a doorway cell's door stands, in stops (0 = wide open, or the map does not say, or the cell is no doorway's) */
fn doorShut(m: u32) -> f32 { let s = (m >> 3u) & 31u; return select(0.0, (DOOR_STEPS - f32(s)) / DOOR_PER_STOP, m != 255u && (m & 3u) == 1u && s != 0u); }

/** what the cache holds over a cell: (log2 of it, how much the reading counts). THE LIGHT THAT FALLS ON THE FLOOR (the cache's +y slab: the irradiance on
 *  a face that looks up, lighting_common.wgsl diceIrradiance), as a light meter laid face up reads it, and the same kind of number as the game's own
 *  meter on Sam. Not the mean light in the air: that of a room is partly its door, its window and its bulb seen straight on. The four probes round the
 *  cell's centre, each by its share, but only those that stand in a free cell and that the cell sees along the map: a probe in a wall holds nothing and
 *  one behind it holds the next room. A probe that holds nothing though the map says it is free (it is inside a crate, or the place is black) counts for
 *  U.dark of its share. */
fn reading(c: vec2i) -> vec2f {
  let dims = vec3i(textureDimensions(diceTex)); let n = vec3i(dims.x, dims.y / 7, dims.z);
  let toCell = vec2f(f32(PLACE_W), f32(PLACE_H)) / vec2f(f32(n.x), f32(n.z)); let me = vec2f(c) + 0.5;
  let q = me / toCell - 0.5; let p0 = vec2i(floor(q)); let f = q - floor(q);
  let iy = clamp(i32(U.y * f32(n.y)), 0, n.y - 1) + 2 * n.y;   // (the cache's +y slab)
  var s = 0.0; var ws = 0.0;
  for (var k = 0u; k < 4u; k++) {
    let p = p0 + vec2i(i32(k & 1u), i32(k >> 1u));
    if (p.x < 0 || p.y < 0 || p.x >= n.x || p.y >= n.z) { continue; }
    let w = select(1.0 - f.x, f.x, (k & 1u) != 0u) * select(1.0 - f.y, f.y, (k & 2u) != 0u);
    if (w < 1e-3) { continue; }
    let pc = (vec2f(p) + 0.5) * toCell; let steps = i32(ceil(length(pc - me) * 2.0)); var seen = true;
    for (var j = 1; j <= steps; j++) { if (maskAt(vec2i(floor(mix(me, pc, f32(j) / f32(steps))))) == 255u) { seen = false; break; } }
    if (!seen) { continue; }
    let tap = textureLoad(diceTex, vec3i(p.x, iy, p.y), 0);
    let l = dot(tap.xyz, vec3f(0.2126, 0.7152, 0.0722)) + tap.w;   // (+ the sky a portal took out of this face to send itself: rc.wgsl rcDice. 0 with no portal lit)
    let wk = w * select(U.dark, 1.0, l > 1e-6);
    s += wk * log2(max(l, 1e-6)); ws += wk;
  }
  if (ws <= 0.0) { return vec2f(0.0); }
  return vec2f(s / ws, ws);
}

/** THE CACHE IS READ ONCE A FRAME, for every cell, into `reads`: a cell's reading is of the cache and the map alone, the same at every sweep of a frame.
 *  (Until 11-05 each sweep read it again for each of its cells, four probes and their sight lines along the map: that was the sweep's whole cost, and it
 *  held a frame to four sweeps. The field then closed a quarter of its distance to a new light a frame, and a room whose light changed at a stroke, a wall
 *  blown open to the day, was shown by the gain of the room it had been for a third of a second. render/placeField.ts PLACE_SWEEPS.) A wall's cell and a
 *  doorway's hold no reading: the sweep takes none there. */
@compute @workgroup_size(8, 8)
fn placeRead(@builtin(global_invocation_id) gid: vec3u) {
  let c = vec2i(gid.xy); if (c.x >= PLACE_W || c.y >= PLACE_H) { return; }
  let m = textureLoad(maskTex, c, 0).r;
  var t = vec2f(0.0); if (m != 255u && (m & 3u) != 1u) { t = reading(c); }
  reads[c.y * PLACE_W + c.x] = t;
}

@compute @workgroup_size(8, 8)
fn placeSweep(@builtin(global_invocation_id) gid: vec3u) {
  let c = vec2i(gid.xy);
  if (c.x >= PLACE_W || c.y >= PLACE_H || ((gid.x + gid.y) & 1u) != U.parity) { return; }
  let i = c.y * PLACE_W + c.x; let m = textureLoad(maskTex, c, 0).r;
  if (m == 255u) { field[i] = PLACE_SOLID; field[PLACE_NEAR + i] = PLACE_SOLID; return; }
  let h0 = hand(c + vec2i(1, 0), m); let h1 = hand(c - vec2i(1, 0), m); let h2 = hand(c + vec2i(0, 1), m); let h3 = hand(c - vec2i(0, 1), m);
  if ((m & 3u) == 1u) { field[i] = PLACE_SOLID; field[PLACE_NEAR + i] = max(max(h0.z, h1.z), max(h2.z, h3.z)); return; }   // (a doorway's cell: no level, hand above; the brightest place a stride away goes through it)
  let t = reads[i];
  let s = h0.xy + h1.xy + h2.xy + h3.xy;
  let w = t.y;
  let den = w + s.y; if (den <= 0.0) { return; }   // (no reading and no neighbour with an answer yet: it waits)
  let gs = (w * t.x + s.x) / den;
  let old = field[i];
  let lv = select(old + U.omega * (gs - old), gs, old < -500.0);
  field[i] = lv;
  field[PLACE_NEAR + i] = max(lv, max(max(h0.z, h1.z), max(h2.z, h3.z)));
}

/** THE OPEN AIR is what the cells with no roof over them hold (the map's bit 4): the level a tenth of them are darker than (PLACE.skyShare, U.skyShare), so it is
 *  the walled yard's and not the snow field's past the wall, and one dark nook does not move it. A map with no cell under the sky (a level all indoors)
 *  takes its brightest place.
 *  COUNTED BY EVERY CELL AT ONCE (placeSky, a dispatch over the map after the sweeps), into a table the eye then reads: a count a bin, the sum of the levels in
 *  it, and the brightest place. (Until 11-05 the eye's one invocation walked every cell of the map twice a frame, a texture read a cell: 1.7 ms of a GPU's
 *  frame on one of its threads, where a sweep of the same cells by all of them is a hundredth of that.) The sums are whole numbers so that the order the
 *  cells add themselves in changes nothing: a level is kept as its height over SKY_BASE in steps of 1 ∕ SKY_FIX of a stop, and a bin's mean comes back
 *  within half a step of the true one (the walk's own sum, a float's, strayed further). A level outside SKY_BASE to SKY_BASE + SKY_SPAN counts as the
 *  end it is past: no reading is under 2^-20, and only a cell the sweep's over-relaxation has thrown for a frame can be out there. */
const SKY_BINS: i32 = 64;        // a quarter of a stop each, from 12 stops under 1 to 4 over it (open ground at noon holds 1 to 8; a level off either end counts in the end bin)
const SKY_TOP: i32 = 2 * SKY_BINS;   // the table: SKY_BINS counts, SKY_BINS sums, then the brightest place (render/placeField.ts SKY_WORDS)
const SKY_BASE: f32 = -24.0;     // under any level a reading can give (log2 of 1e-6 at the least)
const SKY_SPAN: f32 = 48.0;
const SKY_FIX: f32 = 8192.0;
const_assert f32(PLACE_W * PLACE_H) * SKY_SPAN * SKY_FIX < 4294967296.0;   // (every cell in one bin at the top of the span still fits a sum's word)
const_assert SKY_TOP + 1 == 129;   // (the table's words: render/placeField.ts SKY_WORDS, which lookexpo-check holds to this text)
@group(0) @binding(5) var<storage, read_write> sky: array<atomic<u32>, 129>;
fn skyBin(l: f32) -> i32 { return clamp(i32(floor((l + 12.0) * 4.0)), 0, SKY_BINS - 1); }
/** a level as a word that orders as the level does (atomicMax has no float): 0 is under every level */
fn skyWord(l: f32) -> u32 { let b = bitcast<u32>(l); return select(b | 0x80000000u, ~b, (b & 0x80000000u) != 0u); }
fn skyLevel(w: u32) -> f32 { return bitcast<f32>(select(~w, w & 0x7fffffffu, (w & 0x80000000u) != 0u)); }
@compute @workgroup_size(8, 8)
fn placeSky(@builtin(global_invocation_id) gid: vec3u) {
  let c = vec2i(gid.xy); if (c.x >= PLACE_W || c.y >= PLACE_H) { return; }
  let m = textureLoad(maskTex, c, 0).r; if (m == 255u) { return; }
  let l = field[c.y * PLACE_W + c.x]; if (!(l > -500.0)) { return; }   // (no answer yet; or no number at all, which as a word would order over every level)
  if ((m & 3u) == 2u) { atomicMax(&sky[SKY_TOP], skyWord(l)); }
  if ((m & 4u) != 0u) { let b = skyBin(l); atomicAdd(&sky[b], 1u); atomicAdd(&sky[SKY_BINS + b], u32(clamp(l - SKY_BASE, 0.0, SKY_SPAN) * SKY_FIX + 0.5)); }
}

/** THE EYE: the level of the place Sam stands in, eased as an eye is (opening slower than it closes), kept in the word after the last cell; the word after
 *  that says the field is live, and the two after that are THE OPEN AIR's levels, read off placeSky's table (which it leaves empty for the next frame's count)
 *  and eased, so the picture is never handed a step. One invocation a frame. */
@compute @workgroup_size(1)
fn placeEye() {
  let c = clamp(vec2i(floor(U.sam)), vec2i(0), vec2i(PLACE_W - 1, PLACE_H - 1));
  var v = cellAt(c);
  // (his cell holds no level: he stands on a threshold, in a short passage, or against a wall. The eye takes THE NEAREST CELL THAT HOLDS ONE, by where he
  //  stands in his cell: through a doorway that changes about its middle, as the picture's own line does (composite.wgsl lookPlaceLevels), and against a
  //  wall it is his own side's. The brightest beside him, as it was, shut the eye on a dark room he had not yet left)
  if (v < -500.0) { var best = 1e9; for (var dy = -3; dy <= 3; dy++) { for (var dx = -3; dx <= 3; dx++) { let cc = c + vec2i(dx, dy); let l = cellAt(cc); if (l > -500.0) { let dd = distance(vec2f(cc) + 0.5, U.sam); if (dd < best) { best = dd; v = l; } } } } }
  let k = PLACE_W * PLACE_H; field[k + 1] = 1.0;
  var hist: array<u32, 64>; var nSky = 0u;
  for (var b = 0; b < SKY_BINS; b++) { hist[b] = atomicLoad(&sky[b]); nSky += hist[b]; }
  let tw = atomicLoad(&sky[SKY_TOP]); let top = select(-1000.0, skyLevel(tw), tw != 0u);
  // the open air's two words: its LOW level (the level a `skyShare` of the cells under the sky are darker than: where "as bright as the open air" starts) and
  // its MIDDLE (the level half of them are darker than: how bright the open air is, which a room Sam stands in is shown as)
  var open = top; var mid = top;
  if (nSky > 0u) {
    let want = max(u32(ceil(f32(nSky) * U.skyShare)), 1u); let half = max(u32(ceil(f32(nSky) * 0.5)), 1u); var acc = 0u; var bin = -1; var binM = -1;
    for (var b = 0; b < SKY_BINS; b++) { acc += hist[b]; if (bin < 0 && acc >= want) { bin = b; } if (binM < 0 && acc >= half) { binM = b; } }
    // (the mean of a bin's own cells: a yard at one level answers with it, not with a bin's edge)
    if (bin >= 0) { open = SKY_BASE + f32(atomicLoad(&sky[SKY_BINS + bin])) / (SKY_FIX * f32(hist[bin])); }
    if (binM >= 0) { mid = SKY_BASE + f32(atomicLoad(&sky[SKY_BINS + binM])) / (SKY_FIX * f32(hist[binM])); if (bin >= 0) { mid = max(mid, open); } }   // (the middle is never under the low word: two means a step apart at a bin's edge are held in order)
  }
  for (var i = 0; i <= SKY_TOP; i++) { atomicStore(&sky[i], 0u); }
  let was = field[k + 2]; let wasM = field[k + 3];
  field[k + 2] = select(was + (open - was) * U.skyEase, open, U.snap != 0u || was < -500.0 || open < -500.0);
  field[k + 3] = select(wasM + (mid - wasM) * U.skyEase, mid, U.snap != 0u || wasM < -500.0 || mid < -500.0);
  if (U.sam.x < 0.0 || v < -500.0) { return; }   // (nobody to follow this frame: the eye keeps what it has)
  let old = field[k];
  if (U.snap != 0u || old < -500.0) { field[k] = v; return; }
  field[k] = old + (v - old) * select(U.eyeClose, U.eyeOpen, v < old);
}
