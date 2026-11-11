// THE CURSOR, AS THE VIEW IS ASKED ABOUT IT: where it is and through which camera, as the frame is drawn. Nothing under src/view imports the game
// (tools/qa/view-check.ts).
import { CursorNow, type Doubles } from '../draw/input';
export type { CursorView, Under, UnderKind, UnderWall } from './read';

/** the cursor, in the page's pixels */
export interface CursorAt { readonly mouseX: number; readonly mouseY: number }
/** the camera it is seen through */
export interface CursorLens { readonly viewProj: ArrayLike<number>; readonly invViewProj: ArrayLike<number> }
export interface CursorCanvas { readonly clientWidth: number; readonly clientHeight: number }
/** written to `N`; true = another cursor, picture or camera than `N` held */
export function sampleCursor(N: Doubles<'CursorNow'>, at: CursorAt, lens: CursorLens, canvas: CursorCanvas): boolean {
  let moved = false; const put = (i: number, v: number) => { if (N[i] !== v) { N[i] = v; moved = true; } };
  put(CursorNow.mouse, at.mouseX); put(CursorNow.mouse + 1, at.mouseY); put(CursorNow.client, canvas.clientWidth); put(CursorNow.client + 1, canvas.clientHeight);
  for (let i = 0; i < 16; i++) { put(CursorNow.viewProj + i, lens.viewProj[i]); put(CursorNow.invViewProj + i, lens.invViewProj[i]); }
  return moved;
}
