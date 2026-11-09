// FINDING YOUR OWN MAN ON THE SCREEN, and how the camera answers the hand (Options > Game; ui/findmeviz.ts draws the ring, hands the renderer the outline's frame and
// ui/findme.ts builds the rows). With all of them as they start the game is as it stood before it had them on both of the harness's stands: the ring and the outline start
// off and the camera's two start as the camera was. (THE PAGE, since 2026-10-26, starts the outline ON, the aim pan at none and the turn the other way:
// FINDME_TUNED and findMeStarts, below; the module and the frozen stand are as this says.)
//   pan      how far the view leans from the man toward the aim point, as a share of the follow camera's own lead (FollowCamera.aimLead as it was built):
//            1 = as it was · 0 = he is the middle of the screen wherever the mouse goes
//   ring     the gold ring at his feet (the one an open order menu draws: ui/planfloor.ts) on at all times
//   outline  a rim round him and his teammates where they are dark as drawn, by the picture, part by part: his true silhouette, drawn by the renderer from the
//            pixels that are his (render/outline.ts). Its numbers are settled there and in OUTLINE_LOOK, below
//   turn     which way the view turns under Ctrl + the mouse: 1 = as it was · -1 = the other way
// pan and turn change the seat's VIEW, and the view is the player's input (rust/game_rules/src/cmd carries it as data, tick by tick): a recording replays the view
// it was made with whatever these say today, and no rule of the game reads this table. ring is the HUD's alone and outline the picture's: nobody's senses,
// no light meter and no digest can tell either on from off.
// KEPT PER MACHINE (localStorage `twopointsix.findme`), read as this module loads and again when the page says which starts are in force (findMeStarts);
// a harness has no store and gets the starts. Only the rows that
// are NOT at their starts are written (keep), so a start that is tuned later reaches every machine whose row stands at the old start — a row put back to
// its start by hand is one of those: these are rows on trial, and their starts are what the maintainer's own hand is settling. A line an older build kept
// whole is written again that way as the page reads it under its own starts, and a row that is no longer one (the outline's width, strength, fade, light rows,
// teammates' width and colour, settled 2026-11-08; the double tap's, gone with the auto camera) reads as if it were not named. No imports, on purpose:
// the loop (src/host/drawpage.ts) and the engine replay's own view (replay/engineplay.ts, its Ctrl turn) read the table and nothing of the game or the page comes with it.

export interface FindMe { pan: number; ring: boolean; outline: boolean; turn: 1 | -1 }
/** every row as it was BUILT: the module's own starts, and the frozen stand's (no rim) */
export const FINDME_BUILT: Readonly<FindMe> = { pan: 1, ring: false, outline: false, turn: 1 };
/** THE ROWS AS THE MAINTAINER'S HAND LEFT THEM, made the starts on 2026-10-26: he had watched a first-time player on a machine that had kept nothing and "i don't
 *  think the outlines showed up for him" — it was off — then "we should take the numbers i tuned ... and use those as the defaults". THE CAMERA'S TWO are his as
 *  well ("i think aim pan zero and turn direction reversed feel good"; "yeah let's do those as defaults to"): the view does not lean toward the aim point, and it
 *  turns the other way under the camera's key. THE PAGE'S STARTS, not the module's: FINDTUNED and findMeStarts, below. */
export const FINDME_TUNED: Readonly<Partial<FindMe>> = { pan: 0, outline: true, turn: -1 };
/** THE SWITCH: on = the starts in force are the tuned ones over the built. ON in the page (host/profile.ts pageSwitches; `?findtuned=0` = every row starts
 *  as built, no rim until Options > Game asks, as it was), OFF here and on the frozen stand (`TPS_FINDTUNED=1`). The page's alone, as host/savepic.ts's
 *  is (game/knobs.ts VISIT_OWN): the rim is a picture on this machine's screen, and the pan and the turn are how this machine's camera answers the hand —
 *  no rule of the game reads any of it and no recording sets it (a recording carries the view it was made with as data). ON A HARNESS the word reaches
 *  one thing: a scripted turn under the camera's key (Input.orbitDX through ctrlTurn, below) goes the other way round. No stand hands its camera over
 *  (bindFindMe is the page's), so a stand's camera keeps its lead as built whatever the pan row says. Whoever writes it calls findMeStarts() after: the
 *  table is not read off it frame by frame (game/knobs.ts knobsBack does, when a copy put back moves it). */
export const FINDTUNED = { on: false };
const START: FindMe = { ...FINDME_BUILT };
/** the starts in force: as built until the switch says otherwise (findMeStarts). What Reset puts back, and what a kept line is written against */
export const FINDME_START: Readonly<FindMe> = START;
/** the rims' inks as displayed, before the tone map: the pale one they were first drawn in, and at its full the player's yellow
 *  and the two pairs' red and blue (a teammate of a squad with no pairs: blue); `purple` is the second player's own man in a game of two (a player's man has his seat's tone: rust/game_rules/src/seats seatTone) */
export const OUTLINE_INK = { pale: [0.42, 0.5, 0.55], you: [0.95, 0.72, 0.12], red: [0.95, 0.22, 0.16], blue: [0.2, 0.45, 1], purple: [0.74, 0.32, 0.98] };
export type OutlineHue = 'you' | 'red' | 'blue' | 'purple';
/** `out` = the ink of a man of that hue, OUTLINE_LOOK.colour of the way from the pale one */
export function outlineInk(out: number[], hue: OutlineHue): void { const p = OUTLINE_INK.pale, c = OUTLINE_INK[hue], s = OUTLINE_LOOK.colour; for (let i = 0; i < 3; i++) out[i] = p[i] + (c[i] - p[i]) * s; }
export const FINDME: FindMe = { ...FINDME_START };
const LS = 'twopointsix.findme';

/** what a kept line says, whatever it holds: a field that is missing or not of its kind is the start's */
export function findMeRead(text: string | null | undefined): FindMe {
  let v: Record<string, unknown> = {};
  try { const j = JSON.parse(text ?? 'null'); if (j && typeof j === 'object') v = j; } catch { /* not ours */ }
  const flag = (k: 'ring' | 'outline') => typeof v[k] === 'boolean' ? v[k] as boolean : FINDME_START[k];
  const pan = v.pan;
  return { pan: typeof pan === 'number' && isFinite(pan) ? Math.max(0, Math.min(1, pan)) : FINDME_START.pan, ring: flag('ring'), outline: flag('outline'), turn: v.turn === -1 ? -1 : v.turn === 1 ? 1 : FINDME_START.turn };
}
/** THE OUTLINE'S SETTLED NUMBERS (the maintainer's hand, 2026-10-22, on his GPU: "overall these feel a lot better already"): a teammate's rim covers `mates` of what the
 *  player's does (render/outline.ts outlineRadius `times`), and `colour` of the way from the pale ink to his own. The box the rim is looked for in reaches `reachM`
 *  round a man and `topM` up */
export const OUTLINE_LOOK = { mates: 0.8, colour: 0.5, reachM: 1.4, topM: 2.3 };

/** the camera whose lead `pan` scales, and that lead as it was built (bindFindMe) */
let seat: { aimLead: number } | null = null, lead = 0;
/** the page hands over its follow camera once (main.ts, after its starts are in force): a pan under 1 — the page's own start is none — is put on it; at 1
 *  the camera is not written at all. After that the camera's lead is written only when the pan row itself moves (to that share of the lead as built: the
 *  debugger's own aim lead slider writes the same number, and the later hand wins) */
export function bindFindMe(cam: { aimLead: number }) { seat = cam; lead = cam.aimLead; if (FINDME.pan !== 1) cam.aimLead = lead * FINDME.pan; }
/** one setting changed by the player: on the table, on the camera if it is the pan, and kept */
export function setFindMe<K extends keyof FindMe>(k: K, v: FindMe[K]) { if (put(k, v)) keep(); }
/** every row back to its start (Options' Reset): only what had been moved is written */
export function resetFindMe() { let any = false; for (const k of Object.keys(FINDME_START) as (keyof FindMe)[]) any = put(k, FINDME_START[k]) || any; if (any) keep(); }
function put<K extends keyof FindMe>(k: K, v: FindMe[K]): boolean { if (FINDME[k] === v) return false; FINDME[k] = v; if (k === 'pan' && seat) seat.aimLead = lead * FINDME.pan; return true; }
function movedRows(): string { const moved: Record<string, unknown> = {}; for (const k of Object.keys(START) as (keyof FindMe)[]) if (FINDME[k] !== START[k]) moved[k] = FINDME[k]; return JSON.stringify(moved); }
/** the store's line: a string, null = a store that has kept nothing, undefined = no store here (a harness) or one that may not be read (a frame whose
 *  storage is blocked throws at the name itself) */
function keptLine(): string | null | undefined { try { const st = globalThis.localStorage; return st ? st.getItem(LS) : undefined; } catch { return undefined; } }
/** the table kept: the line is the rows moved off the tuned starts and nothing else. */
function keep() {
  try {
    const st = globalThis.localStorage; if (!st) return;
    st.setItem(LS, movedRows());
  } catch { /* no store */ }
}
/** THE STARTS IN FORCE (host/profile.ts pageSwitches calls this once, before anything is drawn). A row standing at its old start moves to its new one; a row a hand has moved stays. Then, where there is a store, the kept line is
 *  read again over the new starts — a machine that has kept nothing gets the outline, one whose line names a row keeps that row, Off included — and a line
 *  that says more than the rows moved off them is written again as only those (an older build's whole line; a line that kept the tuned numbers by hand). The page calls this BEFORE it hands its camera over (bindFindMe puts
 *  the pan on it); no camera is written here. Called again with nothing changed it does nothing on a harness. */
export function findMeStarts(): void {
  const T: Partial<FindMe> = FINDME_TUNED, S = START as unknown as Record<string, unknown>, F = FINDME as unknown as Record<string, unknown>;
  for (const k of Object.keys(FINDME_BUILT) as (keyof FindMe)[]) { const next = k in T ? T[k] : FINDME_BUILT[k]; if (S[k] !== next) { if (F[k] === S[k]) F[k] = next; S[k] = next; } }
  const was = keptLine(); if (was === undefined) return;
  Object.assign(FINDME, findMeRead(was)); if (was != null && was !== movedRows()) keep();
}
{ const was = keptLine(); if (was !== undefined) Object.assign(FINDME, findMeRead(was)); }   // (as the module loads: the line over the built starts. The game page asks again once its switches are written — pageSwitches — and the engine replay's Ctrl turn runs on that page)

// ---- Ctrl + the mouse
/** the view's turn for `dx` pixels of the mouse under Ctrl (rad), the way round the `turn` row says: the one place the rate and the sign are written (the
 *  loop's orbit in play, the engine replay's own view) */
export function ctrlTurn(dx: number): number { return dx * 0.006 * FINDME.turn; }
