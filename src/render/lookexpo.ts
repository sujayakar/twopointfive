// THE LOOK PROTOTYPE — ONE EXPOSURE, the frame's side (it had a row of its own, "One exposure", until 11-04: it is now the only way; the shader's side is
// shaders/place_common.wgsl lookExpoFor, which says what the numbers do). The maintainer, 10-31: "i worry that we're finding these cases one by one and not fixing them
// systematically", and he chose one exposure model over mending the rows it replaced (Eyes adjust to shade's cap, Eyes follow Sam's four numbers by his
// meter, the roll-off by a pixel's own light, the day's contrast given back under a roof and Room contrast). THOSE ROWS ARE GONE (11-04, on his ruling that
// every special case of the light be fixed; git has them), and the row with them: the model is the only way a place is brought up, and with the Eyes adjust
// to shade row at 0 none is. The panel's rows
// that are left say how much: Eyes adjust to shade = the share of a darker place's own dark that is brought up, Eyes follow Sam = how far the exposure goes
// to where he stands, Glare outside = how far the open is overexposed from indoors, Contrast = the turn, about each place's own white.
// AT EVERY HOUR (11-04; it was by day only, as the shade row it rides on was: the maintainer ruled every special case of the light fixed, and this model's was
// that it asked the hour and stood down on a night; he saw the other answer on his GPU behind a row and kept it). The model asks no hour. The day's share is 1
// at every hour, and what holds a night's picture is not a test but THE EYES' WIDEST: no place is shown by more than the night's exposure over the hour's
// (lookExpoFrame's x, as before, now said at night too, where it is 1), except a place whose floor holds less than the DIM level (lookExpoBound: under a
// night's open yard and under a lit room's bounce), which is brought up by the shade row's share of its dark under that level, LOOK_EXPO.dark stops × the
// Eyes open in the dark row at the most (place_common.wgsl lookExpoFor). It is the rule the model has by day for a place darker than the open air, about
// the level a night's exposure leaves off at, and it is the same wherever the eyes are: no two dark places are ever drawn at one level. A night's yard and
// its lit rooms are to the number as the night drew them, and a room lit by its own lamps is one picture at dusk and at night. Under the goggles the tube
// is the eye, and nothing is brought up (game/nvg.ts models the tube on the light as it is). PICTURE ONLY: the guards and the light meter read the light as it is. The check: tools/qa/lookexpo-check.ts.
export const LOOK_EXPO = {
  out: 0.6,      // the open is overexposed by his place's darkness to this power × the Glare outside row (0.1 of a row is a seventh brighter from a window room)
  band: 1.25,    // a place IS the open air while its floor gets 1 ∕ this of the open ground's light or more, and a darker one is counted from there (place_common.wgsl EXPO_BAND, the same number)
  key: 1.8,      // with Sam in a dark place the whole picture is exposed up from the open air's LOW level to its MIDDLE (the field's two words, render/placeField.ts); this many times where the field has no middle to say (composite.wgsl EXPO_KEY)
  most: 16384,   // the darkest place the exposure follows, × darker than the open air (composite.wgsl EXPO_MOST)
  top: 4,        // what a place's gain shows is rolled off above sunlit ground's brightness toward this many times it (place_common.wgsl EXPO_TOP)
  dark: 2,       // THE EYES' WIDEST: a place under the dim level is shown by this many stops more than the night's exposure at the most, × the Eyes open in the dark row
  dim: 0.03,     // THE DIM LEVEL, in the place field's own measure (the cache's light on a face that looks up, render/placeField.ts: the sky's and the bounce's, not a lamp's or the moon's own beam) and the night's units: under a night's open yard and under a lit room's bounce, over an unlit room. Read off the field on the block's night: its yard holds 0.071, its lit rooms 0.14 to 0.38, the far end of its weakest lit hall 0.031, the unlit room its team starts in 0.004 to 0.009 …
  dimSky: 0.4,   // … and never over this share of what the level's own night sky puts on open ground (the block's 0.082; a level under a blacker sky has a darker yard, and its dim level goes down with it)
  read: 8e-6,    // a place's level under this is the field's read floor and no reading (three stops over place_field.wgsl's 1e-6; place_common.wgsl EXPO_READ, the same number): by day the dim level is under it and nothing is brought up past the night's exposure
  lamp: 1.23,    // THE NIGHT'S WHITE (place_common.wgsl EXPO_LAMP, the same number): what a pale floor (0.85) sends back under a room's own bulb (15 lux, render/photometry.ts) as a night shows it. The shoulder is never about less: a room brought to the night's own picture at dusk or on a grey day is rolled off where the night rolls it off
};
/** this frame's eyeCfg under the row: x = THE MOST a place is brought up, the night's exposure over the hour's (`hourExp`, render/photometry.ts hourExposure:
 *  with the Real light values row at 1 a room lit by its lamps alone is then shown as the night shows it, and a room with no light of its own is not shown by its noise; 1 = no bound said. The place Sam stands in is the place field's own
 *  eye, eased on the GPU, render/placeField.ts: there is no second eye by his meter), y = the day's share (the shade row's own fade into a night), z = the power of his darkness the open is overexposed by,
 *  w = −1 − the Eyes follow Sam row (under 0 = the shader's word for this model). Null = no shade row or a sky with no light in it: the frame's field stays zeros and no place
 *  takes a gain. `shade` = this frame's shadeCfg (engine.ts lookShade: x = the shade row × the day's share); `shadeRow` = the shade row's amount;
 *  `amount` = the Eyes follow Sam row; `glare` = the Glare outside row. The day's share is 1 at every hour (the caller's shade row carries no fade either,
 *  engine.ts lookShade). */
export function lookExpoFrame(shade: readonly [number, number, number, number] | undefined, shadeRow: number, amount: number, glare = 0.1, hourExp = 1): [number, number, number, number] | null {
  if (!shade || !(shade[0] > 0) || !(shade[3] > 0) || !(shadeRow > 0)) return null;
  return [hourExp > 0 && hourExp < 1 ? 1 / hourExp : 1, 1, LOOK_EXPO.out * Math.max(0, glare), -1 - Math.min(1, Math.max(0, amount))];
}
/** THE EYES' WIDEST, for the frame (frame.placeCfg's z and w; place_common.wgsl lookExpoFor): [the dim level in the night's units (never 0: over 0 is the
 *  shader's word for the row), the most a place under it is shown by over the night's exposure]. `row` = the Eyes open in the dark row; `nightSkyE` = what
 *  the night's sky puts on open ground (lookNightSkyE; null = there is none to go by); `nv` = how far the goggles are down (0‥1): the tube is the eye
 *  from halfway (post.wgsl takes the tube's picture there), and by then nothing is brought up. */
export function lookExpoBound(row: number, nightSkyE: number | null = null, nv = 0): [number, number] {
  const K = LOOK_EXPO, dim = nightSkyE !== null && nightSkyE >= 0 ? Math.min(K.dim, K.dimSky * nightSkyE) : K.dim;
  return [Math.max(dim, 1e-9), Math.pow(2, K.dark * Math.max(0, row) * (1 - Math.min(1, 2 * Math.max(0, nv))))];
}
/** what the night's sky puts on open ground, for lookExpoBound. On a night it is the sky this frame draws (`frameSkyE`: the frame's shadeCfg.w, with
 *  whatever a weather or a slider has done to it). With an hour up the frame's sky is the hour's, and it is the level file's own sky by the page's word
 *  (`levelSkyE`; null = nobody says, or the level is its own day). `hourExp` = the hour's exposure against the night's (1 = a night). */
export function lookNightSkyE(hourExp: number, frameSkyE: number | undefined, levelSkyE: number | null): number | null {
  return hourExp >= 1 ? frameSkyE ?? null : levelSkyE;
}
