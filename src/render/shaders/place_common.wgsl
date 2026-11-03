// THE LOOK PROTOTYPE — A PLACE AND WHAT IT IS SHOWN BY, for every program that sends a place's light to the eye (the composite's surfaces and their
// mirrors, the volumetric march's air). One text, pasted in front of each kernel that binds the place field as `placeBuf` (render/passes.ts): the level of
// a place, the gain the One exposure row shows it by, and the one rule for light brighter than the place's own. Nothing here reads a texture or a light.
// The long account of the exposure is in composite.wgsl, over lookShadeGain.
const EXPO_BAND = 1.25;
const EXPO_KEY = 1.8;        // THE KEY where the field has no middle word to say it: with Sam in a dark place the whole picture is exposed up to this many times over the open air's low level, in full from a place 3 times under the open
const EXPO_MOST = 16384.0;   // the darkest place the exposure follows: 14 stops under the open air (a room with nothing in its probes holds a millionth, and what shows there at that gain is noise)
const EXPO_TOP = 4.0;        // what a place shows to eyes that are in it is rolled off above sunlit ground's brightness, toward this many times it (a sun patch and the yard in a wet floor, in a room shown hundreds of times brighter)
const PLACE_W: i32 = 100;
const PLACE_H: i32 = 70;
struct PlaceBuf { v: array<vec4f, 1751> };   // (PLACE_W × PLACE_H levels, then the four words: render/placeField.ts PLACE_BYTES)
/** THE PLACE FIELD (render/placeField.ts; the kernel's own `placeBuf` binding): log2 of the light that falls on the floor of the place over (x, z), the same from a room's window to its far
 *  end; under −500 in a wall's cell. Four words follow the cells: Sam's eye, 1 once the field is live, the open air's LOW level on this map (a tenth of the cells under the sky are darker) and its MIDDLE (half are). */
fn placeCell(c: vec2i) -> f32 { let q = clamp(c, vec2i(0), vec2i(PLACE_W - 1, PLACE_H - 1)); let i = q.y * PLACE_W + q.x; return placeBuf.v[i >> 2u][i & 3]; }
fn placeWord(k: i32) -> f32 { let i = PLACE_W * PLACE_H + k; return placeBuf.v[i >> 2u][i & 3]; }
/** THE SHOULDER: light of brightness `l` that a place's gain `g` has already multiplied is rolled off above sunlit ground's brightness toward EXPO_TOP
 *  times it; this is the factor. In full from a gain of 1.5, and 1 at a gain of 1 (the open air is sent on as it is). It is for light that is not the
 *  place's own: a sun patch through the door of a room shown hundreds of times brighter, the floor right under its bulb, the yard mirrored in its wet floor,
 *  which the tone map would take to white and the bloom would smear. (No Contrast row: about the sky's light on open ground.) */
fn lookRoll(l: f32, g: f32) -> f32 {
  let P = select(frame.shadeCfg.w, frame.gradeCfg.y, frame.gradeCfg.y > 0.0);
  if (!(frame.eyeCfg.w < 0.0 && P > 0.0 && g > 1.0 && l > P)) { return 1.0; }   // (the One exposure row only: the rows before it are as they were)
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
  if (frame.eyeCfg.x > 1.0) { base = min(base, frame.eyeCfg.x); }   // (never brighter than the night would show the same lamps: render/lookexpo.ts)
  return base;
}
/** A PLACE'S THREE GAINS, from the light it holds: x = the gain it is shown by now, with the eyes where they are; y = the gain it is shown by to eyes
 *  that are IN it (never under x: a place brighter than the eyes' is shown over, and is its own picture already); z = the gain the open air is shown by
 *  now (never over x). */
fn lookGains(here: f32) -> vec3f {
  let oa = lookOpenAir(); var rs = 1.0;
  if (oa.z > 0.5) { let ue = placeWord(0); if (ue > -500.0) { rs = lookEyeDark(exp2(ue), oa.x); } }
  let g = lookExpoFor(here, rs, oa);
  return vec3f(g, max(lookExpoFor(here, lookEyeDark(here, oa.x), oa), g), min(lookExpoFor(oa.x, rs, oa), g));
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
/** the level of the place a point of the AIR is in (the volumetric march's steps): its cell's; in a wall's cell (the map's cell takes in the air beside
 *  the wall) the BRIGHTEST free cell within two, so the air at a wall's foot is never shown by the gain of a darker room behind the wall. Under −500 = none. */
fn placeAir(X: vec3f) -> f32 {
  let q = (X.xz - RC_MIN.xz) / RC_SIZE.xz * vec2f(f32(PLACE_W), f32(PLACE_H)); let c = vec2i(floor(q)); let own = placeCell(c);
  if (own > -500.0) { return own; }
  var hi = -1000.0;
  for (var r = 1; r <= 2 && hi < -500.0; r++) { for (var dy = -r; dy <= r; dy++) { for (var dx = -r; dx <= r; dx++) { hi = max(hi, placeCell(c + vec2i(dx, dy))); } } }
  return hi;
}
