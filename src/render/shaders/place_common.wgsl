// THE LOOK PROTOTYPE — A PLACE AND WHAT IT IS SHOWN BY, for every program that sends a place's light to the eye (the composite's surfaces and their
// mirrors, the volumetric march's air). One text, pasted in front of each kernel that binds the place field as `placeBuf` (render/passes.ts): the level of
// a place, the gain the one exposure model shows it by, and the one rule for light brighter than the place's own. Nothing here reads a texture or a light.
// The long account of the exposure is in composite.wgsl, over lookShadeGain.
const EXPO_BAND = 1.25;
const EXPO_KEY = 1.8;        // THE KEY where the field has no middle word to say it: with Sam in a dark place the whole picture is exposed up to this many times over the open air's low level, in full from a place 3 times under the open
const EXPO_MOST = 16384.0;   // the darkest place the exposure follows: 14 stops under the open air (a room with nothing in its probes holds a millionth, and what shows there at that gain is noise)
const EXPO_LAMP = 1.23;        // THE NIGHT'S WHITE: what a pale floor sends back under a room's own bulb as a night shows it (0.85 of 15 lux over π, the night's units; render/lookexpo.ts LOOK_EXPO.lamp)
const EXPO_READ = 0.000008;   // a place's level under this is the field's read floor and no reading (three stops over place_field.wgsl's 1e-6): the eyes' widest counts no place darker (render/lookexpo.ts LOOK_EXPO.read)
const EXPO_TOP = 4.0;        // what a place shows to eyes that are in it is rolled off above sunlit ground's brightness, toward this many times it (a sun patch and the yard in a wet floor, in a room shown hundreds of times brighter)
const PLACE_W: i32 = 100;
const PLACE_H: i32 = 70;
struct PlaceBuf { v: array<vec4f, 3501> };   // (PLACE_W × PLACE_H levels, the four words, then a second block of PLACE_W × PLACE_H: render/placeField.ts PLACE_BYTES)
/** THE PLACE FIELD (render/placeField.ts; the kernel's own `placeBuf` binding): log2 of the light that falls on the floor of the place over (x, z), the same from a room's window to its far
 *  end; under −500 in a wall's cell. Four words follow the cells: Sam's eye, 1 once the field is live, the open air's LOW level on this map (a tenth of the cells under the sky are darker) and its MIDDLE (half are). */
fn placeCell(c: vec2i) -> f32 { let q = clamp(c, vec2i(0), vec2i(PLACE_W - 1, PLACE_H - 1)); let i = q.y * PLACE_W + q.x; return placeBuf.v[i >> 2u][i & 3]; }
/** … and THE BRIGHTEST PLACE A STRIDE AWAY from that cell (the field's second block, place_field.wgsl hand): never under the cell's own level */
fn placeNear(c: vec2i) -> f32 { let q = clamp(c, vec2i(0), vec2i(PLACE_W - 1, PLACE_H - 1)); let i = PLACE_W * PLACE_H + 4 + q.y * PLACE_W + q.x; return placeBuf.v[i >> 2u][i & 3]; }
fn placeWord(k: i32) -> f32 { let i = PLACE_W * PLACE_H + k; return placeBuf.v[i >> 2u][i & 3]; }
/** THE SHOULDER: light of brightness `l` that a place's gain `g` has already multiplied is rolled off above sunlit ground's brightness toward EXPO_TOP
 *  times it; this is the factor. In full from a gain of 1.5, and 1 at a gain of 1 (the open air is sent on as it is). It is for light that is not the
 *  place's own: a sun patch through the door of a room shown hundreds of times brighter, the floor right under its bulb, the yard mirrored in its wet floor,
 *  which the tone map would take to white and the bloom would smear. (No Contrast row: about the sky's light on open ground.) */
fn lookRoll(l: f32, g: f32) -> f32 {
  var P = select(frame.shadeCfg.w, frame.gradeCfg.y, frame.gradeCfg.y > 0.0);
  P = max(P, EXPO_LAMP);   // (ONE EXPOSURE AT EVERY HOUR: a room brought to the night's own picture has the night's white, where the hour's sunlit ground is dimmer than that: a night, dawn, dusk, a grey day)
  if (!(frame.eyeCfg.w < 0.0 && P > 0.0 && g > 1.0 && l > P)) { return 1.0; }   // (only for a place the model has brought up)
  let room = (EXPO_TOP - 1.0) * P;
  return mix(1.0, (P + room * (1.0 - exp(-(l - P) / room))) / l, clamp((g - 1.0) * 2.0, 0.0, 1.0));
}
/** the picture's own numbers, the same for every place of a frame: (the open air's level, the most the key is, 1 where the field is live). Until the
 *  field is live the open air is the frame's own word for it and Sam is taken to stand in the open. */
fn lookOpenAir() -> vec3f {
  var openL = frame.shadeCfg.y; var keyMost = EXPO_KEY; var live = 0.0;
  if (placeWord(1) > 0.5) {
    live = 1.0;
    let sky = placeWord(2); if (sky > -500.0) { openL = exp2(sky); let mid = placeWord(3); if (mid > -500.0) { keyMost = max(exp2(mid - sky) * EXPO_BAND, 1.0); } }   // (THE OPEN AIR is what the cells under the open sky hold on this map, render/placeField.ts: a walled yard is × 1, whatever open country would hold)
  }
  return vec3f(openL, keyMost, live);
}
/** how many times darker than the open air the eyes' place counts as, from the light it holds (`ue`), under the Eyes follow Sam row's power */
fn lookEyeDark(ue: f32, openL: f32) -> f32 { return pow(clamp(openL / max(ue, 1e-9) / EXPO_BAND, 1.0, EXPO_MOST), clamp(-frame.eyeCfg.w - 1.0, 0.0, 1.0)); }
/** ONE GAIN A PLACE: the place holds `here`, the eyes are in a place `rs` times darker than the open air (lookEyeDark), `oa` = lookOpenAir() */
fn lookExpoFor(here: f32, rs: f32, oa: vec3f) -> f32 {
  let rp = clamp(oa.x / max(here, 1e-9) / EXPO_BAND, 1.0, EXPO_MOST);
  let a = frame.eyeCfg.y; let outG = pow(rs, frame.eyeCfg.z);
  let key = pow(min(sqrt(rs), oa.y), a);   // (by where Sam stands alone: one number a picture, so the order of its places is as it was)
  var base = 1.0;
  if (rp >= rs) { base = key * pow(rs, a) * pow(rp / rs, frame.shadeCfg.x); }
  else {
    let q = rs / rp;
    var over = 1.0; if (outG > 1.001) { over = 1.0 + (outG - 1.0) * (1.0 - exp(-(q - 1.0) / (outG - 1.0))); }
    base = key * pow(rp * over, a);
  }
  if (frame.gradeCfg.x > 0.0) { base *= pow(max(rp / base, 1e-6), 1.0 - frame.gradeCfg.x); }
  if (frame.lampCfg.w > 0.0 && base > 1.0) { base = pow(base, frame.lampCfg.w); }   // (THE ROOM BRIGHTNESS ROW, the look panel's `roomb`: the maintainer, 11-01, "exposure does seem a bit too bright when indoors but we can tweak it". A place's gain to this power: 1 = as it was, under 1 = every room darker, a deep one by more; 0 in the frame = the row's light is not this frame's)
  // THE EYES' WIDEST (render/lookexpo.ts), AT EVERY HOUR. Never brighter than the night would show the same lamps (eyeCfg.x, the night's exposure over the
  // hour's; 1 at night), and a place whose floor holds less than the dim level (placeCfg.z, in the night's units) is brought up by the shade row's share of
  // its dark under that level, placeCfg.w at the most: the rule the model has for a place darker than the open air, about the level a night's exposure
  // leaves off at. Wherever the eyes are, so no two dark places are drawn at one level. Nothing here asks what hour it is.
  {
    let over = max(frame.eyeCfg.x, 1.0); var most = over;
    if (oa.z > 0.5) { most *= clamp(pow(frame.placeCfg.z / (over * max(here, EXPO_READ)), frame.shadeCfg.x), 1.0, max(frame.placeCfg.w, 1.0)); }   // (`here` is in the field's measure once the field is live, and the buffers hold exposed light: the dim level in this hour's units is z ∕ over. Before the field is live no place is past the night's exposure)
    base = min(base, most);
  }
  return base;
}
/** how many times darker than the open air the place the eyes are in counts as (1 until the field is live and has an eye for him) */
fn lookEyeRs(oa: vec3f) -> f32 { var rs = 1.0; if (oa.z > 0.5) { let ue = placeWord(0); if (ue > -500.0) { rs = lookEyeDark(exp2(ue), oa.x); } } return rs; }
/** A PLACE'S THREE GAINS, from the light it holds: x = the gain it is shown by now, with the eyes where they are; y = the gain it is shown by to eyes
 *  that are IN it (never under x: a place brighter than the eyes' is shown over, and is its own picture already); z = the gain the open air is shown by
 *  now (never over x). */
fn lookGains(here: f32) -> vec3f { let oa = lookOpenAir(); return lookGainsAt(here, lookEyeRs(oa), oa); }
/** … the same, handed the open air's words and the eyes' darkness (lookOpenAir, lookEyeRs) by a caller that has them */
fn lookGainsAt(here: f32, rs: f32, oa: vec3f) -> vec3f {
  let g = lookExpoFor(here, rs, oa);
  return vec3f(g, max(lookExpoFor(here, lookEyeDark(here, oa.x), oa), g), min(lookExpoFor(oa.x, rs, oa), g));
}
/** LIGHT KEEPS ITS ORDER WHERE TWO PLACES MEET (`k` = frame.placeCfg.x, the look panel's Light in order row; 0 = lookGains(here), to the bit). The
 *  maintainer, 11-04, of an open door's orange leaf glowing in a shaded doorway: "is there some nonphysical stuff going on here with doors? door looks like
 *  it's lit even though the ground immediately in front of it is in shadow", and then "is there a way we can make the exposure stuff more general and not
 *  special case doors? this should hold for holes from wall charges too right?", "or even areas with very different brightnesses outdoors". A place has ONE
 *  gain, from the light on its floor, and a dark place's is large (9 stops under the yard is shown 3 under). What stands at its mouth in the light of the
 *  place beside it (a leaf in a doorway, the reveal of a hole blown in the wall, a man in an open side) got the dark place's whole gain and was drawn brighter than the
 *  lighter place it is lit by. Only sunlit ground's brightness stopped it (lookRoll). THE RULE NAMES NO OPENING: A PIXEL THAT HOLDS MORE LIGHT THAN ITS
 *  PLACE IS SHOWN AS A PLACE HOLDING ITS OWN LIGHT WOULD BE, AS FAR AS THE BRIGHTEST PLACE A STRIDE AWAY HOLDS (`near`: the field's second block, which
 *  dims a stop for every cell of floor between and never passes a wall), and as far as the light of the place the eyes are in (the model's own word for
 *  it: with Sam in the room nothing moves). Deep in a room there is no brighter place a stride away, so `near` is the room's own level and every pixel
 *  is to the bit as it was: A ROOM KEEPS ITS FALL-OFF FROM ITS WINDOW (the first shape of this rule had no `near`, and a whole daylit room seen from the
 *  yard went dark and flat: every pixel an opening lights holds more than the room's level). Where the field is smooth (the open air, one place over
 *  eight metres) `near` is hardly over the level and nothing moves either: the rule acts where the gain steps, which is where order was lost.
 *  WHAT IT DOES NOT REACH, on purpose: the bound does not pass a pane (glass is a wall to a place: a window's light is the room's own, and a sill in it
 *  keeps the room's gain), and out of doors a sunlit thing in a deep shade is as the field's own smoothing leaves it.
 *  `own` = the light on the pixel in the field's measure (the caller's: composite.wgsl lookExpoGain). */
fn lookGainsOwn(here: f32, own: f32, near: f32, k: f32) -> vec3f {
  var h = here; let oa = lookOpenAir(); let rs = lookEyeRs(oa);
  if (k > 0.0 && near > h && oa.z > 0.5 && placeWord(2) > -500.0) {   // (the field is live and says what the open air holds: `own` and the eyes' light are in its measure)
    let o = min(own, min(near, oa.x / (rs * EXPO_BAND)));
    if (o > h) { h = max(h, 1e-9) * pow(o / max(h, 1e-9), min(k, 1.0)); }
  }
  return lookGainsAt(h, rs, oa);
}
/** WHAT A PLACE SENDS THE EYE, ONE RULE FOR EVERY KIND OF LIGHT (11-02; the maintainer on his GPU, midday, his eyes in the yard and then in the room: the men
 *  in a bulb-lit room white at their edges on a near-black floor, "not able to see where the glow is coming from here by flipping through the debug views",
 *  "it subsides a lot once i shoot out the one light in the room"; and smoke in a room "black indoors but white outside"). A PLACE SEEN FROM ANOTHER IS THE
 *  PICTURE IT IS TO EYES THAT ARE IN IT, SHOWN DARKER: its light is exposed as for eyes in the place (gains.y), rolled off there above sunlit ground's
 *  brightness (lookRoll), and only then taken down to the gain the place is shown by now (gains.x). So the highlight of a bulb on a wet floor, on a steel
 *  drum or on a man's kit is as many times brighter than the room's floor from the yard as it is from inside the room, where before it kept the whole of its
 *  strength while the floor went down twelve or thirty times beside it. AND NEVER DARKER THAN THE OPEN AIR SHOWS THE SAME LIGHT (gains.z): a sun patch through
 *  a door is as bright in a room shown dark as the ground it came in over. `c` = the light before any gain; the diffuse, the mirrors, the gloss, the men's
 *  finish and the air's own light all pass through here, and nothing else rolls a place's light off. To eyes in the place itself this is the gain and the
 *  shoulder as they were. */
fn lookShowBy(c: vec3f, gains: vec3f) -> vec3f {
  let l = luminance(c);
  if (!(l > 0.0)) { return c * gains.x; }
  let full = l * gains.y * lookRoll(l * gains.y, gains.y);
  return c * (max(full * (gains.x / gains.y), l * gains.z) / l);
}
/** the level of the place a point of the AIR is in (the volumetric march's steps): its cell's; in a cell that holds none (a wall's: the map's cell takes
 *  in the air beside the wall; a doorway's) the BRIGHTEST free cell within two, so the air at a wall's foot and on a threshold is never shown by the gain
 *  of the darker room. ONE ANSWER A CELL, whatever the point in it: the march keeps a cell's answer for the steps it takes there (volumetrics.wgsl
 *  airShow), so the plan's lines, which part a cell in two, are not asked here. Under −500 = none. */
fn placeAir(X: vec3f) -> f32 {
  let q = (X.xz - RC_MIN.xz) / RC_SIZE.xz * vec2f(f32(PLACE_W), f32(PLACE_H)); let c = vec2i(floor(q)); let own = placeCell(c);
  if (own > -500.0) { return own; }
  var hi = -1000.0;
  for (var r = 1; r <= 2 && hi < -500.0; r++) { for (var dy = -r; dy <= r; dy++) { for (var dx = -r; dx <= r; dx++) { hi = max(hi, placeCell(c + vec2i(dx, dy))); } } }
  return hi;
}
/** THE PIXEL'S PLACE IS READ OFF THE FLOOR PLAN, NEVER GUESSED (the maintainer, 11-04, of a door on his GPU: "weird flash on the door when opening", "there's
 *  a dark band right by the entry of the door threshold and door", and of a depot at midday: "lots of bleeding in that level"; before those, 10-31: "i worry
 *  that we're finding these cases one by one and not fixing them systematically"). Each was a pixel shown by the gain of a place it is not in, and each came
 *  from the same step: where the field had no plain answer (a wall's cell, a doorway's, a door mid swing) the place was GUESSED from what the radiance cache
 *  held near the pixel, five reads of it with a box ray each. That guess is gone, with the kernel's second box walk. What answers now:
 *    A FREE CELL: its own level, blended with the free cells beside it (placeBlend).
 *    A WALL'S CELL, the floor at its foot and its faces: the wall's own line says which side the point is on (common.wgsl sideLine, to 2.5 cm), and the
 *      answer is the first cell out on that side (placeSideCell), blended as any cell of that room is, so the strip at a wall's foot is
 *      one with the floor beside it.
 *    A DOORWAY'S CELL: the same, by its door's line, shut or open and at every angle of its leaf. It holds no level of its own (place_field.wgsl
 *      placeSweep), so a threshold is never a third place with a number between its two rooms'. Each side of the line is its own room to the line, and what
 *      joins them in the picture is the light itself: the floor a bright room lights inside a dark one's door is shown by the level of the light that falls on
 *      it (the Light in order row, place_common.wgsl lookGainsOwn), so the edge a doorway shows is the light's own and never the plan's.
 *    A WALL'S TOP (a face that looks up at a wall's height, within PLACE_WALL of a wall's line): of neither room, so THE BRIGHTER OF THE TWO its line parts. A sunlit top
 *      between a yard and a dark room is shown as the yard is and never by the gain of the room under it, which was the white lines along the depot's walls.
 *      The part of a top that overhangs into the free cell beside the wall's is found by the wall's line too (PLACE_WALL).
 *    A CELL WITH NO LINE (a turned wall, two walls in one cell), or with nothing its walk could reach: the brightest of the nearest cells that hold a level
 *      (on the point's own side of its cell's lines, where it has any), by the same rule: a point that is of no one room is never shown by the gain of the
 *      darker.
 *    NO PLACE WITHIN FOUR CELLS (the middle of a wide mass's top): the light that falls on the pixel is its place's level. The one use of the pixel's own
 *      light to say a place.
 *    BEFORE THE FIELD IS LIVE (a device that cannot build it; the engine builds it with the boot's own programs, so the page's first frame has it): every
 *      place is the open air, and no gain is taken. */
const PLACE_TOP: f32 = 1.88;     // a face that looks up above this height on a wall's line is the wall's top (scene/placePlan.ts: a wall fills the height to 1.9 m; a man's head is under it)
const PLACE_WALL: f32 = 0.14;    // m: a top this near a wall's centre line is that wall's, though the plan's cell under it is the room's (a shell wall's half thickness, 0.12, and the 1.25 cm a line's code may sit off the line, with a little over)
/** ONE PLACE'S ANSWER AT A POINT OF THE PLAN, as (its level, THE BRIGHTEST PLACE A STRIDE AWAY: the field's second block, place_common.wgsl placeNear), both
 *  log2: cell `c`, which holds `own`, blended with the cells beside it by where `q` (in cells) is in it; a `q` outside the cell is taken at the cell's nearest
 *  edge. Never read through a wall or across a doorway's line: the neighbours weighed in are the two beside the cell (two cells with a level side by side are
 *  one side of any wall, and a doorway's cells hold none) and the diagonal one only past one of those. */
fn placeBlend(q: vec2f, c: vec2i, own: f32) -> vec2f {
  let f = clamp(q - vec2f(c), vec2f(0.0), vec2f(1.0)) - 0.5; let s = vec2i(select(-1, 1, f.x >= 0.0), select(-1, 1, f.y >= 0.0)); let a = abs(f);
  let vx = placeCell(c + vec2i(s.x, 0)); let vz = placeCell(c + vec2i(0, s.y)); let vd = placeCell(c + s);
  var ws = (1.0 - a.x) * (1.0 - a.y); var sum = own * ws; var sn = max(placeNear(c), own) * ws;
  if (vx > -500.0) { let w = a.x * (1.0 - a.y); sum += w * vx; sn += w * max(placeNear(c + vec2i(s.x, 0)), vx); ws += w; }
  if (vz > -500.0) { let w = (1.0 - a.x) * a.y; sum += w * vz; sn += w * max(placeNear(c + vec2i(0, s.y)), vz); ws += w; }
  if (vd > -500.0 && (vx > -500.0 || vz > -500.0)) { let w = a.x * a.y; sum += w * vd; sn += w * max(placeNear(c + s), vd); ws += w; }
  return vec2f(sum / ws, sn / ws);
}
/** THE CELL THAT ANSWERS FOR A POINT IN A CELL WITH NO LEVEL (a wall's, a doorway's), by the plan's own lines (common.wgsl sideLine): from cell `c0` a cell
 *  at a time toward the point's side `o0` of that cell's line, to the first cell with a level. A DOORWAY'S CELL MET FROM ITS SIDE TURNS THE WALK to the
 *  point's side of ITS line: the floor at the foot of a wall that ends in a doorway is of the room on its side of the door. A walk along a doorway's own
 *  way through goes on as it was (the side was said where it began; each cell of a run holds the run's line to its own 2.5 cm, and asking it again cell
 *  by cell could turn a walk back). ANOTHER WALL'S LINE AHEAD STOPS IT: the pocket between a thing and the wall behind it has no cell of its own, and
 *  the room past that wall is not its place (the caller's rule for a point with no line answers). The line of the cell it set out from is not another's:
 *  a wall on a cell's edge is two cells with one line, and a wall's top asks for the far side of its own. `m` = the point, metres from the map's corner.
 *  → the cell; x under −9000 = none. */
const PLACE_PAST: f32 = 0.02;    // m: a wall's line this far ahead of the point or more stops a walk …
const PLACE_SAME: f32 = 0.03;    // … unless it lies within this of the line the walk set out from (the cells of one wall hold its line to 2.5 cm)
fn placeSideCell(m: vec2f, c0: vec2i, o0: vec2i) -> vec2i {
  var c = c0; var o = o0; let d0 = sideLine(c0, m);
  for (var r = 0; r < 6; r++) {
    c += o; if (placeCell(c) > -500.0) { return c; }
    let d = sideLine(c, m);
    if (sideIsDoor(c)) { if (d.x == 0.0 && d.y != 0.0 && o.x != 0) { o = vec2i(0, i32(sign(d.y))); } else if (d.y == 0.0 && d.x != 0.0 && o.y != 0) { o = vec2i(i32(sign(d.x)), 0); } }
    else if ((d.x * f32(o.x) < -PLACE_PAST && (d0.x == 0.0 || abs(d.x - d0.x) > PLACE_SAME)) || (d.y * f32(o.y) < -PLACE_PAST && (d0.y == 0.0 || abs(d.y - d0.y) > PLACE_SAME))) { return vec2i(-9999); }
  }
  return vec2i(-9999);
}
/** the place of the pixel at P with normal N, as (the light its place holds, the brightest place a stride away), both as light and in the field's measure;
 *  `Eown` = the light that falls on the pixel, in that measure. The rule is the note above. ONE CALL of the walk and one of the blend, in a loop over
 *  the cells that may answer (a wall's top asks two sides or four, everything else one cell): the kernel's size is counted by its call sites. */
fn lookPlaceLevels(P: vec3f, N: vec3f, Eown: f32) -> vec2f {
  if (placeWord(1) < 0.5) { return vec2f(lookOpenAir().x); }
  let X = P + N * 0.3; let m = X.xz - RC_MIN.xz;
  let q = m / RC_SIZE.xz * vec2f(f32(PLACE_W), f32(PLACE_H)); let c = vec2i(floor(q)); let own = placeCell(c);
  let d = sideLine(c, m); let o = vec2i(sign(d));
  for (var ask = select(1, 0, N.y > 0.7 && P.y > PLACE_TOP); ask < 2; ask++) {
    var cw = c; var n = 0; var dirs: array<vec2i, 4>;   // (the cells that may answer: `cw` itself for a step of (0, 0), else placeSideCell's from `cw` along the step)
    if (ask == 0) {
      // A WALL'S TOP: within PLACE_WALL of a wall's line. The wall is this cell's, or the cell's beside it where the top overhangs its side or its end (the
      // nearer cell on each axis, by its own lines). A tall thing's top further from any line than that is its room's, by the second asking
      var dt = d;
      if (own > -500.0) {
        let s = vec2i(select(-1, 1, q.x - floor(q.x) >= 0.5), select(-1, 1, q.y - floor(q.y) >= 0.5));
        let dx = sideLine(c + vec2i(s.x, 0), m); let dz = sideLine(c + vec2i(0, s.y), m);
        if ((dx.x != 0.0 || dx.y != 0.0) && max(abs(dx.x), abs(dx.y)) <= PLACE_WALL) { cw = c + vec2i(s.x, 0); dt = dx; }
        else if ((dz.x != 0.0 || dz.y != 0.0) && max(abs(dz.x), abs(dz.y)) <= PLACE_WALL) { cw = c + vec2i(0, s.y); dt = dz; }
      }
      // (the side of each line the point is within a wall's half thickness of. Where the cell has two, a corner or a T, and the point is over both, the
      //  top is of the four places round the corner; over one of them, of the two that line parts, on the point's side of the other: a walk straight
      //  along a wall's own cells would come out at its far end)
      let t = vec2i(select(0, i32(sign(dt.x)), dt.x != 0.0 && abs(dt.x) <= PLACE_WALL), select(0, i32(sign(dt.y)), dt.y != 0.0 && abs(dt.y) <= PLACE_WALL));
      if (t.x != 0 && t.y != 0) { n = 4; dirs[0] = t; dirs[1] = -t; dirs[2] = vec2i(t.x, -t.y); dirs[3] = vec2i(-t.x, t.y); }
      else if (t.x != 0 || t.y != 0) { let p = vec2i(select(0, i32(sign(dt.x)), t.x == 0), select(0, i32(sign(dt.y)), t.y == 0)); n = 2; dirs[0] = t + p; dirs[1] = -t + p; }
    }
    else if (own > -500.0) { n = 1; dirs[0] = vec2i(0); }
    else if (o.x != 0 || o.y != 0) { n = 1; dirs[0] = o; }
    var best = vec2f(-1000.0);
    for (var k = 0; k < n; k++) {
      var cc = cw; if (dirs[k].x != 0 || dirs[k].y != 0) { cc = placeSideCell(m, cw, dirs[k]); }
      if (cc.x > -9000) { best = max(best, placeBlend(q, cc, placeCell(cc))); }
    }
    if (best.x > -500.0) { return exp2(best); }
  }
  // (the nearest cells that hold a level, the brightest of them. A cell with lines whose walk found nothing asks first for cells on the point's own side
  //  of those lines: the floor in a corner behind a panel is not the room across the corner's own wall)
  var hi = -1000.0; var nh = -1000.0;
  for (var ask = 0; ask < 2 && hi < -500.0; ask++) {
    if (ask == 0 && o.x == 0 && o.y == 0) { continue; }
    for (var r = 1; r <= 4 && hi < -500.0; r++) { for (var dy = -r; dy <= r; dy++) { for (var dx = -r; dx <= r; dx++) {
      if (ask == 0 && ((o.x != 0 && dx * o.x < 0) || (o.y != 0 && dy * o.y < 0))) { continue; }
      hi = max(hi, placeCell(c + vec2i(dx, dy))); nh = max(nh, placeNear(c + vec2i(dx, dy)));
    } } }
  }
  if (hi < -500.0) { return vec2f(max(Eown, 1e-9)); }
  return exp2(vec2f(hi, max(nh, hi)));
}
/** THE THREE GAINS OF THE PLACE A POINT OF THE AIR IS IN (lookGains of placeAir's level), for a step of the volumetric march. Over the roofs, and
 *  where no cell within two holds a level, the air is the open air's. All 1 until the field is live. (A particle asks what a surface asks,
 *  lookPlaceLevels: particles_draw.wgsl partShow.) */
fn lookAirGains(X: vec3f) -> vec3f {
  let oa = lookOpenAir();
  if (!(oa.z > 0.5)) { return vec3f(1.0); }
  var here = oa.x; if (X.y < CEIL_Y) { let lv = placeAir(X); if (lv > -500.0) { here = exp2(lv); } }
  return lookGainsAt(here, lookEyeRs(oa), oa);
}
