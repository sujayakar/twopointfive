// THE LOOK PROTOTYPE — ONE EXPOSURE, the frame's side (the look panel's "One exposure" row, render/frame.ts lookConfig.expo; the shader's side is
// shaders/place_common.wgsl lookExpoFor, which says what the numbers do). The maintainer, 10-31: "i worry that we're finding these cases one by one and not fixing them
// systematically", and he chose one exposure model over mending the rows it replaces (Eyes adjust to shade's cap, Eyes follow Sam's four numbers, the
// roll-off by a pixel's own light, the day's contrast given back under a roof and Room contrast). With the row up those rows' AMOUNTS still say how much:
// Eyes adjust to shade = the share of a darker place's own dark that is brought up, Eyes follow Sam = how far the exposure goes to where he stands, Glare
// outside = how far the open is overexposed from indoors, Contrast = the turn, now about each place's own white. Room contrast has nothing left to do.
// BY DAY ONLY, as the shade row it rides on is. PICTURE ONLY: the guards and the light meter read the light as it is. The check: tools/qa/lookexpo-check.ts.
export const LOOK_EXPO = {
  out: 0.6,      // the open is overexposed by his place's darkness to this power × the Glare outside row (LOOK_EYE.out: 0.1 of a row is a seventh brighter from a window room)
  band: 1.25,    // a place IS the open air while its floor gets 1 ∕ this of the open ground's light or more, and a darker one is counted from there (place_common.wgsl EXPO_BAND, the same number)
  key: 1.8,      // with Sam in a dark place the whole picture is exposed up from the open air's LOW level to its MIDDLE (the field's two words, render/placeField.ts); this many times where the field has no middle to say (composite.wgsl EXPO_KEY)
  most: 16384,   // the darkest place the exposure follows, × darker than the open air (composite.wgsl EXPO_MOST)
  top: 4,        // what a place's gain shows is rolled off above sunlit ground's brightness toward this many times it (place_common.wgsl EXPO_TOP)
};
/** this frame's eyeCfg under the row: x = THE MOST a place is brought up, the night's exposure over the hour's (`hourExp`, render/photometry.ts hourExposure:
 *  with the Real light values row at 1 a room lit by its lamps alone is then shown as the night shows it, and a room with no light of its own is not shown by its noise; 1 = no bound said. The place Sam stands in is the place field's own
 *  eye, eased on the GPU, render/placeField.ts: there is no second eye by his meter), y = the day's share (the shade row's own fade into a night), z = the power of his darkness the open is overexposed by,
 *  w = −1 − the Eyes follow Sam row (under 0 = the shader's word for this model). Null = no shade row or a night: the frame's field stays zeros and the rows
 *  as they were run. `shade` = this frame's shadeCfg (engine.ts lookShade: x = the shade row × the day's share); `shadeRow` = the shade row's amount;
 *  `amount` = the Eyes follow Sam row; `glare` = the Glare outside row. */
export function lookExpoFrame(shade: readonly [number, number, number, number] | undefined, shadeRow: number, amount: number, glare = 0.1, hourExp = 1): [number, number, number, number] | null {
  if (!shade || !(shade[0] > 0) || !(shade[3] > 0) || !(shadeRow > 0)) return null;
  const day = Math.min(1, shade[0] / Math.min(1, shadeRow));
  return [hourExp > 0 && hourExp < 1 ? 1 / hourExp : 1, day, day * LOOK_EXPO.out * Math.max(0, glare), -1 - Math.min(1, Math.max(0, amount))];
}
