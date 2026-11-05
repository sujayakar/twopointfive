// THE PICTURE'S PLANNERS HAVE A PACE (render/steps.ts has why: they run cold, and all of them ran on the frame a wall was blown).
// One allowance of time a drawn frame, between them: the openings found again and the walls read again (engine.ts lookOpeningLights) and which side of
// a wall each cell is on (render/wallSides.ts) are jobs in steps, a step at least every frame; the place map (drawn at once) and the order the statics
// are drawn in (scene/boxes.ts: made whole, on a drawn frame once the statics have stood still) have their time taken off it.
// THE SWITCH: PLAN_PACE.on — ON in the page (`?planpace=old` = each planner whole, on the frame that asks for it, as it was, to the frame), OFF here,
// so a tool or a check under bun gets every plan at once unless it asks for a pace. No rule of the game reads it. What it moves is WHEN a plan lands,
// by a few frames, and the game's light meter reads the picture those plans shape (render/steps.ts).
export const PLAN_PACE = {
  on: false,
  /** milliseconds of planning a drawn frame, between all the planners. The jobs in steps stop often (most stops are a quarter to half a millisecond of
   *  cold code apart; the longest timed in a page was 3) and run over the allowance by a stop; run warm, each is whole inside it. The place map's draw
   *  (2 to 6 ms cold, on the frame a wall changes) and the order's making (2 to 7, on a calm frame after it) are whole whatever is left: their time
   *  is taken off, so the jobs then get their one step */
  ms: 2,
  /** drawn frames the statics must have stood still for before the order they are drawn in is made again; until then they are drawn in the list's own
   *  order. Ten: the order is made whole (2 to 7 ms cold), and for eight or so frames after a blast the game's own update is still long (the pieces
   *  falling), so three put it on a frame of 30 ms in Chromium on the office where ten puts it on one of 12 */
  calm: 10,
};
