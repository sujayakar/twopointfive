// Written by the program `half` (rust/game_start/src/bin/half) from rust/game_types/src/handed.rs. To change what is handed, change it there.

/** a row's doubles, by its name */
export type Doubles<N extends string> = Float64Array & { readonly row: N };
/** what a player's hand is kept as (the input, then the lens) has this stamp: a tape of another is refused */
export const HAND = 3121145077;

/** WHAT A SEAT'S HANDS DID IN A TICK, as a player's own machine hands it and as it travels to another's: his keys go beside it, as their codes */
export const enum SeatInput {
  /** f64: the mouse's buttons held, a bit each */
  buttons = 0,
  /** f64: those that went down in the tick */
  clicked = 1,
  /** f64: what the wheel turned */
  wheel = 2,
  /** f64: the cursor, in the page's pixels */
  mx = 3,
  /** f64 */
  my = 4,
  /** f64: the view it is in: its width and height in the page's pixels */
  w = 5,
  /** f64 */
  h = 6,
  /** f64: … and in the picture's own */
  pw = 7,
  /** f64 */
  ph = 8,
  /** f64 × 3: what his camera looks at */
  target = 9,
  /** f64: from where about it: the turn, the tilt, how far */
  yaw = 12,
  /** f64 */
  pitch = 13,
  /** f64 */
  distance = 14,
  /** f64: its lens */
  fov = 15,
  /** f64 */
  aspect = 16,
  /** f64: how far it leads his aim */
  aimLead = 17,
  /** doubles a row */
  row = 18,
}
export const seatInput = (): Doubles<'SeatInput'> => new Float64Array(SeatInput.row) as unknown as Doubles<'SeatInput'>;

/** THE VIEW A PLAYER'S OWN MACHINE HAS beside that: what the game works out again for a seat that is another machine's */
export const enum SeatLens {
  /** f64 × 16 */
  vp = 0,
  /** f64 × 16 */
  ivp = 16,
  /** f64 × 3 */
  pos = 32,
  /** f64 × 3 */
  forward = 35,
  /** f64: the distance the camera is on its way to */
  desired = 38,
  /** doubles a row */
  row = 39,
}
export const seatLens = (): Doubles<'SeatLens'> => new Float64Array(SeatLens.row) as unknown as Doubles<'SeatLens'>;

/** THE CURSOR A QUESTION IS ASKED OF, as the frame is drawn: where it is and in what picture (the page's pixels), through which camera */
export const enum CursorNow {
  /** f64 × 2 */
  mouse = 0,
  /** f64 × 2 */
  client = 2,
  /** f64 × 16 */
  viewProj = 4,
  /** f64 × 16 */
  invViewProj = 20,
  /** doubles a row */
  row = 36,
}
export const cursorNow = (): Doubles<'CursorNow'> => new Float64Array(CursorNow.row) as unknown as Doubles<'CursorNow'>;

/** A NETWORKED HOST'S OWN MAN, AHEAD OF THE TICK: where the tick has him (his feet, the way he aims), and where he is drawn */
export const enum LeadNow {
  /** f64 */
  man = 0,
  /** f64 × 3 */
  at = 1,
  /** f64 */
  yaw = 4,
  /** f64 × 3 */
  to = 5,
  /** f64 */
  toYaw = 8,
  /** doubles a row */
  row = 9,
}
export const leadNow = (): Doubles<'LeadNow'> => new Float64Array(LeadNow.row) as unknown as Doubles<'LeadNow'>;

/** WHAT THE PAGE SAYS OF ITS LOOK: a yes is 1 */
export const enum PageLook {
  /** f64 */
  on = 0,
  /** f64 */
  marks = 1,
  /** f64 */
  cut = 2,
  /** f64 */
  round = 3,
  /** f64 */
  motion = 4,
  /** f64 */
  panes = 5,
  /** f64: lines of light: mirrors show them, and the air shows a weapon's */
  mirrors = 6,
  /** f64 */
  air = 7,
  /** f64: solid glass: the darkest it is sent and its roughness (under 0: as any box) */
  glassLum = 8,
  /** f64 */
  glassRough = 9,
  /** f64 */
  texel = 10,
  /** f64 × 32: by stuff: its number in the look's maps, and the roughness its coat lends (under 0: none) */
  stuff = 11,
  /** f64 × 32 */
  coat = 43,
  /** f64 × 4: by side of men: the colours a cast man's own are put for */
  swaps = 75,
  /** f64: the men's cloth and skin show wet */
  wet = 79,
  /** doubles a row */
  row = 80,
}
export const pageLook = (): Doubles<'PageLook'> => new Float64Array(PageLook.row) as unknown as Doubles<'PageLook'>;
