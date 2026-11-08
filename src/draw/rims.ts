// THE OUTLINE IN THE DARK (render/outline.ts), from its section: how wide and in what ink is the player's own (host/findme.ts), how strong his light says.
import { FINDME, OUTLINE_LOOK, outlineInk, outlineWant, type OutlineHue } from '../host/findme';
import { makeOutlineMan, type OutlineFrame, type OutlineMan } from '../render/outline';
import type { Drawn } from './drawn';
import { Rim, Section, View } from './sections';

const HUES: readonly OutlineHue[] = ['you', 'red', 'blue', 'purple'];

/** one man's part, his strength eased over `dt` (a frame with no time in it takes it whole) */
function man(M: OutlineMan, D: Drawn, o: number, night: number, dt: number, times: number): void {
  const f = D.f, u = D.u, L = OUTLINE_LOOK, want = u[o + Rim.up] ? outlineWant(f[o + Rim.light], night) : 0;
  M.on = true; M.strength = dt > 0 && u[o + Rim.up] ? M.strength + (want - M.strength) * Math.min(1, dt * L.ease) : want;
  for (let k = 0; k < 3; k++) M.at[k] = f[o + Rim.at + k];
  M.reach = L.reachM; M.top = L.topM; M.wide = FINDME.width; M.times = times; M.alpha = FINDME.opacity; outlineInk(M.ink, HUES[u[o + Rim.hue]]);
}
export function rimsFrom(F: OutlineFrame, D: Drawn, dt: number): void {
  const n = D.n[Section.rims], first = D.at[Section.rims], pool = F.mates, night = D.f[D.at[Section.view] + View.night];
  if (!n || !FINDME.outline || !(FINDME.width > 0) || !(FINDME.opacity > 0)) { F.on = false; F.strength = 0; for (const m of pool) { m.on = false; m.strength = 0; m.owner = 0; } return; }
  F.picture = FINDME.picture; F.lit = FINDME.picLit; F.dark = FINDME.picDark; F.whole = Math.max(0, Math.min(1, 1 - night));
  F.owner = D.u[first + Rim.id]; man(F, D, first, night, dt, 1);
  for (const m of pool) m.on = false;
  if (FINDME.mates > 0) for (let i = 1; i < n; i++) { const o = first + i * Rim.row, id = D.u[o + Rim.id];
    let m: OutlineMan | undefined, free: OutlineMan | undefined;
    for (const x of pool) { if (x.owner === id) { m = x; break; } if (!x.owner) free ??= x; }
    if (!m) { m = free; if (!m) pool.push(m = makeOutlineMan()); m.owner = id; m.strength = 0; }
    man(m, D, o, night, dt, FINDME.mates); }
  for (const m of pool) if (!m.on) { m.strength = 0; m.owner = 0; }
}
