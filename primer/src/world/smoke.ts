// How much thinner a volume must be made when its finer waves are left out, so that as much gets through in the
// mean. Worked out once for each volume by sending rays through it: the noise here is the shader's, to the bit.

import type { Vec3 } from './vec';

export interface VolumeGiven {
  center: Vec3;
  half: Vec3;
  /** for each meter, of each color */
  absorb: Vec3;
  scatter: Vec3;
  emit?: Vec3;
  /** 1 forward, 0 every way alike */
  g?: number;
  /** how uneven: 0 is even */
  contrast?: number;
  /** the longest wave, meters */
  wave?: number;
  /** how far in from its sides it fades, of 1 */
  soft?: number;
  /** meters a second */
  drift?: Vec3;
}

const hash = (x: number): number => {
  const s = (Math.imul(x, 747796405) + 2891336453) >>> 0;
  const w = Math.imul(((s >>> ((s >>> 28) + 4)) ^ s) >>> 0, 277803737) >>> 0;
  return ((w >>> 22) ^ w) >>> 0;
};
const hash2 = (a: number, b: number): number => hash((a + hash(b)) >>> 0);
const lattice = (x: number, y: number, z: number): number =>
  ((hash2(x >>> 0, hash2(y >>> 0, z >>> 0)) >>> 8) / 16777216) * 2 - 1;

function noise(x: number, y: number, z: number): number {
  const X = Math.floor(x);
  const Y = Math.floor(y);
  const Z = Math.floor(z);
  const ease = (t: number) => t * t * (3 - 2 * t);
  const fx = ease(x - X);
  const fy = ease(y - Y);
  const fz = ease(z - Z);
  const mix = (a: number, b: number, t: number) => a + (b - a) * t;
  const row = (dy: number, dz: number) => mix(lattice(X, Y + dy, Z + dz), lattice(X + 1, Y + dy, Z + dz), fx);
  return mix(mix(row(0, 0), row(1, 0), fy), mix(row(0, 1), row(1, 1), fy), fz);
}

/** for 1, 2, 3 and 4 waves left out */
export function clumps(v: VolumeGiven): [number, number, number, number] {
  const contrast = v.contrast ?? 0;
  if (contrast <= 0) return [1, 1, 1, 1];
  const wave = v.wave ?? 0.5;
  const thick = 0.2126 * (v.absorb[0] + v.scatter[0]) + 0.7152 * (v.absorb[1] + v.scatter[1]) + 0.0722 * (v.absorb[2] + v.scatter[2]);
  const density = (p: Vec3, waves: number): number => {
    let n = 0;
    let w = wave;
    let strength = 0.6;
    for (let o = 0; o < waves; o++) {
      n += strength * noise(p[0] / w + o * 7.3, p[1] / w + o * 7.3, p[2] / w + o * 7.3);
      w *= 0.5;
      strength *= 0.6;
    }
    return Math.max(0, 1 + contrast * n);
  };
  let state = 17;
  const random = () => (state = hash(state + 0x9e3779b9) >>> 0) / 4294967296;
  const depth = Math.min(v.half[0], v.half[1], v.half[2]) * 2;
  const STEPS = 48;
  return [1, 2, 3, 4].map((gone) => {
    // rays as wide as the waves that are left out: all that is in one beam is one answer
    const width = wave / 2 ** (4 - gone);
    let through = 0;
    let coarse = 0;
    const BEAMS = 24;
    const RAYS = 12;
    for (let b = 0; b < BEAMS; b++) {
      const base: Vec3 = [random() * 20, random() * 20, random() * 20];
      for (let r = 0; r < RAYS; r++) {
        const x = base[0] + (random() - 0.5) * width;
        const y = base[1] + (random() - 0.5) * width;
        let fine = 0;
        let left = 0;
        for (let i = 0; i < STEPS; i++) {
          const p: Vec3 = [x, y, base[2] + ((i + 0.5) / STEPS) * depth];
          fine += density(p, 4) * (depth / STEPS);
          left += density(p, 4 - gone) * (depth / STEPS);
        }
        through += Math.exp(-thick * fine) / (BEAMS * RAYS);
        coarse += Math.exp(-thick * left) / (BEAMS * RAYS);
      }
    }
    // in the mean over all beams, as much must get through
    return Math.max(0.3, Math.min(1.2, Math.log(through) / Math.log(coarse)));
  }) as [number, number, number, number];
}
