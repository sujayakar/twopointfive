// WHERE A TEAMMATE'S LINE FROM OFF THE SCREEN IS PUT (game/squadnet.ts says what such a line is, the view says it is one and how it is laid: `view.look.net`). Pure, and nothing of the game under it.
import { clamp } from '../math/vec';
import type { NetLook } from '../view/read';

/** the pitch of a far line's row, in the view's pixels: two lines nearer than this in y are on the same row (ui/bubbles.ts tests by it too) */
export const NET_ROW = 26;

/** how far in from the border the way to a man is laid from, where you stand nearer the border than that: a share of the view's shorter side */
const NET_INSET = 0.15;

/** WHERE A FAR LINE IS DRAWN (ui/bubbles.ts asks nothing else), in the view's own pixels: from (px, py) — you, or the middle of the view — toward
 *  (mx, my), where the man it names lies on the screen's plane; (bw, bh) the line's box, drawn up from the point and centred on it; `hits(x, y)` = a
 *  box put there would lie on something to keep clear of (a line already drawn, a HUD card). Null: he is where it would be drawn from (no side).
 *  `edge` ON: where the way to him leaves the view, `edgeIn` inside the border; at him where he is inside that. The way is laid from you, or, where
 *  you stand out by the border yourself, from a point a little way in (or a man past the border you stand at would put the line on you). A spot
 *  that hits something moves ALONG THE BORDER it is at, the nearest free place first and toward the middle of the view before away from it: up or
 *  down a row at a side border; at the top or the bottom a row or two in first (two men off the same way read as a list), then sideways. Never off
 *  the view, and the spot itself where nothing is free.
 *  OFF: `near` out from you, and up a row at a time while that hits (down where up leaves the view), as it was. */
export function netPlace(K: NetLook, w: number, h: number, px: number, py: number, mx: number, my: number, bw: number, bh: number, hits: (x: number, y: number) => boolean): [number, number] | null {
  const L = Math.hypot(mx - px, my - py); if (!(L > 1)) return null;
  const x0 = bw / 2 + 6, x1 = Math.max(x0, w - bw / 2 - 6), y0 = bh + 12, y1 = Math.max(y0, h - 12), m = Math.min(w, h);   // (the box kept whole inside the view)
  if (!K.edge) { const R = Math.min(L, K.near * m), x = clamp(px + (mx - px) / L * R, x0, x1), ya = clamp(py + (my - py) / L * R, y0, y1); let y = ya;
    for (let k = 1; k < 12 && hits(x, y); k++) { const up = ya - NET_ROW * k; y = up >= y0 && !hits(x, up) ? up : ya + NET_ROW * k; }
    return [x, y]; }
  const pad = Math.min(K.edgeIn * m, (x1 - x0) / 2, (y1 - y0) / 2), bx0 = x0 + pad, bx1 = x1 - pad, by0 = y0 + pad, by1 = y1 - pad, cx = (bx0 + bx1) / 2, cy = (by0 + by1) / 2;
  let x = clamp(mx, bx0, bx1), y = clamp(my, by0, by1), side = true;   // (at him, where he is inside the border; `side`: the spot slides in y)
  if (x !== mx || y !== my) { const ins = NET_INSET * m, ax = clamp(px, Math.min(bx0 + ins, cx), Math.max(bx1 - ins, cx)), ay = clamp(py, Math.min(by0 + ins, cy), Math.max(by1 - ins, cy)), dx = mx - ax, dy = my - ay;
    const tx = dx > 0 ? (bx1 - ax) / dx : dx < 0 ? (bx0 - ax) / dx : Infinity, ty = dy > 0 ? (by1 - ay) / dy : dy < 0 ? (by0 - ay) / dy : Infinity, t = Math.min(tx, ty);
    x = clamp(ax + dx * t, bx0, bx1); y = clamp(ay + dy * t, by0, by1); side = tx <= ty; }
  if (!hits(x, y)) return [x, y];
  if (!side) for (let k = 1; k <= 2; k++) { const yi = y + (y > cy ? -1 : 1) * NET_ROW * k; if (yi >= by0 && yi <= by1 && !hits(x, yi)) return [x, yi]; }
  const v = side ? y : x, lo = side ? by0 : bx0, hi = side ? by1 : bx1, first = v > (lo + hi) / 2 ? -1 : 1;
  for (let k = 1; k * NET_ROW <= hi - lo; k++) for (const s of [first, -first]) { const u = v + s * NET_ROW * k; if (u >= lo && u <= hi && !hits(side ? x : u, side ? u : y)) return side ? [x, u] : [u, y]; }
  return [x, y];
}
