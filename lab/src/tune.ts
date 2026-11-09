// What can be turned while looking: a number a name, kept in the frame's own uniforms (tune), so any stage may read it. The defaults are what the lab is.
export interface Knob { i: number; name: string; min: number; max: number; def: number; step?: number }
const K = (i: number, name: string, min: number, max: number, def: number, step?: number): Knob => ({ i, name, min, max, def, ...(step ? { step } : {}) });
export const KNOBS = {
  visFrames: K(0, 'Frames of shadow history', 1, 64, 18, 1), bounceFrames: K(1, 'Frames of bounce history', 1, 128, 36, 1), mirrorFrames: K(2, 'Frames of mirror history', 1, 96, 40, 1), leash: K(3, 'How far history may stand off', 0.1, 6, 1),
  rays: K(4, 'Bounce rays a pixel', 1, 16, 2, 1), soft: K(5, 'How alike neighbors must be', 0.05, 6, 1), keep: K(6, 'Samples the kept light remembers', 4, 400, 96, 1), bounce: K(7, 'Bounce strength (1 is true)', 0, 3, 1),
  haze: K(8, 'Haze', 0, 8, 1), dust: K(9, 'Dust in the rooms', 0, 0.06, 0.009), beams: K(10, 'Beams', 0, 0.06, 0.014), taau: K(11, 'Frames in the picture', 1, 64, 28, 1),
  glow: K(12, 'Glow', 0, 0.3, 0.028), glowWide: K(13, 'Glow width', 0, 1, 0.5), key: K(14, 'Exposure, stops', -6, 2, -2.1), detail: K(15, 'Fine contrast', 0.4, 2, 1.12),
  high: K(16, 'How much the lights keep', 0.1, 1.2, 0.55), low: K(17, 'How much the shadows keep', 0.1, 1.2, 0.78), contrast: K(18, 'Curve contrast', 0.8, 2.4, 1.45), toe: K(19, 'Toe', 1, 2.6, 1.6),
  color: K(20, 'Color', -0.6, 1.4, 0.38), hue: K(21, 'Hue held', 0, 1, 0.65), vignette: K(22, 'Vignette', 0, 1, 0.3), mirror: K(23, 'Mirror strength (1 is true)', 0, 3, 1),
  clarity: K(30, 'Clarity', 0.4, 2.4, 1.15), wide: K(28, 'How fast a bounce ray widens (m a m)', 0.02, 100, 0.5), fine: K(29, 'Kept light is read when the ray is wider than (cells)', 0, 2, 0),
  dark: K(24, 'How dark the night may look', 0, 1, 0.6),
} satisfies Record<string, Knob>;
/** a level's look at an hour: what it adds to the knobs */
export interface Grade { hour: number; clarity?: number; detail?: number; contrast?: number; color?: number; key?: number; toe?: number; low?: number; high?: number }
export const graded = (g: Grade[] | undefined, hour: number, to: Float32Array) => { if (!g?.length) return; const t = ((hour % 24) + 24) % 24; let i = 0; while (i < g.length && g[i]!.hour <= t) i++; const b = g[i % g.length]!, a = g[(i + g.length - 1) % g.length]!, span = ((b.hour - a.hour + 24) % 24) || 24, x = ((t - a.hour + 24) % 24) / span, k = x * x * (3 - 2 * x); for (const n of ['clarity', 'detail', 'contrast', 'color', 'key', 'toe', 'low', 'high'] as const) to[KNOBS[n].i]! += (a[n] ?? 0) * (1 - k) + (b[n] ?? 0) * k; };
export const tuneOf = (k: Knob, base = 'frame.tune') => `${base}[${k.i >> 2}u].${'xyzw'[k.i & 3]}`;
export const defaults = () => { const f = new Float32Array(32); for (const k of Object.values(KNOBS)) f[k.i] = k.def; return f; };
/** `@name` in a shader's text is that number */
export const tuned = (code: string, base = 'frame.tune') => code.replace(/@tune\.(\w+)/g, (_, n: string) => { const k = (KNOBS as Record<string, Knob>)[n]; if (!k) throw new Error('no knob ' + n); return tuneOf(k, base); });
