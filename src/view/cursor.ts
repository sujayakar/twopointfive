// THE CURSOR, AS THE VIEW IS ASKED ABOUT IT: where it is and through which camera, as the frame is drawn (36 doubles, rust/game_face `face_cursor_view`). Nothing under
// src/view imports the game (tools/qa/view-check.ts).
export type { CursorView, Under, UnderKind, UnderWall } from './read';

/** the cursor, in the page's pixels */
export interface CursorAt { readonly mouseX: number; readonly mouseY: number }
/** the camera it is seen through */
export interface CursorLens { readonly viewProj: ArrayLike<number>; readonly invViewProj: ArrayLike<number> }
export interface CursorCanvas { readonly clientWidth: number; readonly clientHeight: number }
export const CURSOR_DOUBLES = 36;
/** written to `N`; true = another cursor, picture or camera than `N` held */
export function sampleCursor(N: Float64Array, at: CursorAt, lens: CursorLens, canvas: CursorCanvas): boolean {
  let moved = false; const put = (i: number, v: number) => { if (N[i] !== v) { N[i] = v; moved = true; } };
  put(0, at.mouseX); put(1, at.mouseY); put(2, canvas.clientWidth); put(3, canvas.clientHeight);
  for (let i = 0; i < 16; i++) { put(4 + i, lens.viewProj[i]); put(20 + i, lens.invViewProj[i]); }
  return moved;
}
