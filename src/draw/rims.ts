// THE OUTLINE IN THE DARK (render/outline.ts), from its section: how wide and in what ink is host/findme.ts OUTLINE_LOOK's, how strong the picture says.
import { FINDME, OUTLINE_LOOK, outlineInk, type OutlineHue } from '../host/findme';
import { makeOutlineMan, type OutlineFrame, type OutlineMan } from '../render/outline';
import type { Drawn } from './drawn';
import { Rim, Section, View } from './sections';

const HUES: readonly OutlineHue[] = ['you', 'red', 'blue', 'purple'];

/** one man's part */
function man(M: OutlineMan, D: Drawn, o: number, times: number): void {
  const f = D.f, u = D.u, L = OUTLINE_LOOK;
  M.on = true;
  for (let k = 0; k < 3; k++) M.at[k] = f[o + Rim.at + k];
  M.reach = L.reachM; M.top = L.topM; M.times = times; outlineInk(M.ink, HUES[u[o + Rim.hue]]);
}
export function rimsFrom(F: OutlineFrame, D: Drawn): void {
  const n = D.n[Section.rims], first = D.at[Section.rims], pool = F.mates, night = D.f[D.at[Section.view] + View.night];
  if (!n || !FINDME.outline) { F.on = false; for (const m of pool) { m.on = false; m.owner = 0; } return; }
  F.whole = Math.max(0, Math.min(1, 1 - night));
  F.owner = D.u[first + Rim.id]; man(F, D, first, 1);
  for (const m of pool) m.on = false;
  for (let i = 1; i < n; i++) { const o = first + i * Rim.row, id = D.u[o + Rim.id];
    let m: OutlineMan | undefined, free: OutlineMan | undefined;
    for (const x of pool) { if (x.owner === id) { m = x; break; } if (!x.owner) free ??= x; }
    if (!m) { m = free; if (!m) pool.push(m = makeOutlineMan()); m.owner = id; }
    man(m, D, o, OUTLINE_LOOK.mates); }
  for (const m of pool) if (!m.on) m.owner = 0;
}
