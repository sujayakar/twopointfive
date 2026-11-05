// A STAND-IN FOR A MAN'S CHARACTER, for a check that builds one by hand. A real Character keeps his place and his velocity as columns (src/sim/men), and the game reads them as `posX` and
// `velZ`: on a plain `{ pos: [...] }` those are undefined and every sum made of them NaN, with nothing to say so. This gives a hand-built one the same six words, over its own arrays.
const AXES = ['X', 'Y', 'Z'] as const;
export function standInChar<C extends { pos: number[]; vel?: number[] }>(c: C): C {
  AXES.forEach((axis, i) => {
    Object.defineProperty(c, `pos${axis}`, { get: () => c.pos[i], set: (v: number) => { c.pos[i] = v; }, configurable: true });
    Object.defineProperty(c, `vel${axis}`, { get: () => c.vel?.[i], set: (v: number) => { if (c.vel) c.vel[i] = v; }, configurable: true });
  });
  return c;
}
