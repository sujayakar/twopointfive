// THE LOOK PROTOTYPE's WINDOW LIGHT (the look panel's "Window light" row, render/frame.ts lookConfig.windows; engine.ts lookWindowLights keeps the lights).
// The maintainer, 10-28, in a room with every door shut under an overcast sky: "it doesn't really _look_ like all the light is coming from the window? feels
// flat". It does come in at the window, but by the radiance cache alone: a grid of probes a metre apart that hold a few broad directions each, read per pixel
// by the gather. A pane a metre wide is smaller than one of those directions, so its light arrives spread over the room with no way it came from: nothing in
// the room throws a shadow away from the window and the floor under it is hardly brighter than the far corner.
// So by day every window is ALSO a light of its own, as a film crew would hang one: a one-sided disk on the inside face of the wall, as big as the glass, as
// bright as what the glass looks out on (the sky near the horizon, and the ground the sky and the sun light), shaded per pixel like any lamp. A man by the
// window throws a shadow into the room and the light falls off from the glass. What the cache already brought in is still there (a little is counted twice;
// it was a few hundredths of the open air). Picture and light meter alike: the meter reads the picture's light. Nothing here knows the level by name:
// a window is any upright Glass box with a roof over one side of it and none over the other. The check: tools/qa/lookwindow-check.ts.
import { Vec3 } from '../math/vec';

export const LOOK_WINDOW = {
  ground: 0.5,    // what the ground outside sends back of the light on it (snow more, a shaded yard less)
  up: 0.3,        // the sky a pane looks at: this much of the zenith's and the rest of the horizon's
  proud: 0.17,    // m from the glass into the room: just clear of the wall's inside face (a disk inside the reveal has the wall in front of its rim)
  gap: 0.3,       // m: panes this close along one wall, in one plane, are one window (a frame's width between them)
  thin: 0.05,     // m: a pane's half thickness is under this (a glass block is not a window)
  least: 0.004,   // a window whose light is under this is not lit at all (a night's sky: it would only take a lamp's place in the list)
};

export interface LookWindow { /** where the light hangs */ pos: Vec3; /** into the room */ dir: Vec3; /** the glass, m² */ area: number; /** the disk's radius, m */ radius: number }
interface Pane { c: ArrayLike<number>; h: ArrayLike<number>; yaw: number }

/** the windows among `panes` (the level's upright glass boxes): panes side by side in one wall are one; `roofed(p)` = is there a roof over p. A window is glass
 *  with a roof over exactly one side of it, and its light faces that side. */
export function lookWindows(panes: readonly Pane[], roofed: (p: Vec3) => boolean): LookWindow[] {
  const K = LOOK_WINDOW; type G = { ax: Vec3; long: Vec3; y: number; hy: number; plane: number; lo: number; hi: number; area: number; depth: number };
  const groups: G[] = [];
  for (const b of panes) {
    const thin = b.h[0] <= b.h[2] ? 0 : 2; if (b.h[thin] > K.thin || b.h[1] < 0.15 || b.h[2 - thin] < 0.1) continue;   // (a skylight, a shelf, a sliver: not a window)
    const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); const X: Vec3 = [cs, 0, -sn], Z: Vec3 = [sn, 0, cs]; const ax = thin === 0 ? X : Z, long = thin === 0 ? Z : X;
    const plane = b.c[0] * ax[0] + b.c[2] * ax[2], at = b.c[0] * long[0] + b.c[2] * long[2], hl = b.h[2 - thin], area = 4 * hl * b.h[1];
    const g = groups.find(g => Math.abs(g.ax[0] * ax[0] + g.ax[2] * ax[2]) > 0.99 && Math.abs(g.plane * Math.sign(g.ax[0] * ax[0] + g.ax[2] * ax[2]) - plane) < 0.05 && Math.abs(g.y - b.c[1]) < 0.05 && Math.abs(g.hy - b.h[1]) < 0.05
      && at * Math.sign(g.long[0] * long[0] + g.long[2] * long[2]) + hl > g.lo - K.gap && at * Math.sign(g.long[0] * long[0] + g.long[2] * long[2]) - hl < g.hi + K.gap);
    if (g) { const s = Math.sign(g.long[0] * long[0] + g.long[2] * long[2]); g.lo = Math.min(g.lo, at * s - hl); g.hi = Math.max(g.hi, at * s + hl); g.area += area; }
    else groups.push({ ax, long, y: b.c[1], hy: b.h[1], plane, lo: at - hl, hi: at + hl, area, depth: b.h[thin] });
  }
  const out: LookWindow[] = [];
  for (const g of groups) {
    const mid = (g.lo + g.hi) * 0.5, c: Vec3 = [g.ax[0] * g.plane + g.long[0] * mid, g.y, g.ax[2] * g.plane + g.long[2] * mid];
    const a = roofed([c[0] + g.ax[0] * 0.6, c[1], c[2] + g.ax[2] * 0.6]), b = roofed([c[0] - g.ax[0] * 0.6, c[1], c[2] - g.ax[2] * 0.6]);
    if (a === b) continue;   // (glass between two rooms, or a screen out of doors)
    const s = a ? 1 : -1, dir: Vec3 = [g.ax[0] * s, 0, g.ax[2] * s];
    out.push({ pos: [c[0] + dir[0] * K.proud, c[1], c[2] + dir[2] * K.proud], dir, area: g.area, radius: Math.max(0.1, Math.min(Math.sqrt(g.area / Math.PI), Math.min((g.hi - g.lo) * 0.5, g.hy) + 0.02)) });
  }
  return out;
}

/** the radiance a window looks out on (linear RGB, the picture's units): half of what it sees is the sky, low down (common.wgsl skyRadiance's ramp), and half
 *  the ground, lit by the whole sky (π × (0.947 zenith + 0.053 horizon), as engine.ts lookShade has it) and by the sun that is up. × `amount`. BY DAY ONLY: it
 *  fades out as the sky's own mean radiance falls from 0.12 to 0.04 (every hour of the day is over that, a night's sky is under it: a night's rooms are the
 *  game's, as dark as they were, and a window that would give next to nothing takes no place in the light list). */
export function windowRadiance(zen: ArrayLike<number>, hor: ArrayLike<number>, skyK: number, sun: { intensity: number; dir: ArrayLike<number>; color: ArrayLike<number> } | undefined, amount: number): Vec3 {
  const K = LOOK_WINDOW, r: Vec3 = [0, 0, 0], sunE = sun && sun.dir[1] < 0 ? sun.intensity * -sun.dir[1] : 0;
  const up = skyK * (0.2126 * (0.8 * zen[0] + 0.2 * hor[0]) + 0.7152 * (0.8 * zen[1] + 0.2 * hor[1]) + 0.0722 * (0.8 * zen[2] + 0.2 * hor[2])), t = Math.min(1, Math.max(0, (up - 0.04) / 0.08)), day = t * t * (3 - 2 * t);
  for (let i = 0; i < 3; i++) {
    const sky = skyK * (K.up * zen[i] + (1 - K.up) * hor[i]), groundE = Math.PI * skyK * (0.947 * zen[i] + 0.053 * hor[i]) + sunE * (sun ? sun.color[i] : 0);
    r[i] = Math.max(0, amount) * day * 0.5 * (sky + K.ground * groundE / Math.PI);
  }
  return r;
}
