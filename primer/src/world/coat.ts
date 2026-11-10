// HOW MUCH A COAT SENDS BACK in all, for light from everywhere alike: for each roughness and each slant
// of looking, the two numbers A and B of  reflected = f0 · A + B.  Worked out once, by adding up draws of
// the same lobe the shaders use (GGX, visible normals, Smith shadowing both ways).

export const COAT_SIZE = 16;

export function coatTable(): Float32Array {
  const out = new Float32Array(COAT_SIZE * COAT_SIZE * 4);
  const DRAWS = 32;
  for (let r = 0; r < COAT_SIZE; r++) {
    const roughness = r / (COAT_SIZE - 1);
    const alpha = Math.max(roughness * roughness, 1e-3);
    for (let m = 0; m < COAT_SIZE; m++) {
      const cosView = (m + 0.5) / COAT_SIZE;
      const v = [Math.sqrt(1 - cosView * cosView), 0, cosView] as const;
      let A = 0;
      let B = 0;
      for (let i = 0; i < DRAWS; i++) {
        for (let j = 0; j < DRAWS; j++) {
          const h = drawFacet(v, alpha, (i + 0.5) / DRAWS, (j + 0.5) / DRAWS);
          const vh = v[0] * h[0] + v[1] * h[1] + v[2] * h[2];
          const cosLight = 2 * vh * h[2] - v[2];
          if (cosLight <= 0) continue;
          const weight = shadowingRatio(cosView, cosLight, alpha);
          const f = Math.pow(1 - vh, 5);
          A += weight * (1 - f);
          B += weight * f;
        }
      }
      const at = (r * COAT_SIZE + m) * 4;
      out[at] = A / (DRAWS * DRAWS);
      out[at + 1] = B / (DRAWS * DRAWS);
    }
  }
  return out;
}

const lambda = (cos: number, a: number) => {
  const c2 = cos * cos;
  return (Math.sqrt(1 + (a * a * (1 - c2)) / c2) - 1) / 2;
};
/** both ways shadowed, over only the view's */
const shadowingRatio = (view: number, light: number, a: number) =>
  (1 + lambda(view, a)) / (1 + lambda(view, a) + lambda(light, a));

/** a facet among those the view can see (Heitz 2018) */
function drawFacet(v: readonly [number, number, number], a: number, u1: number, u2: number): [number, number, number] {
  const n = (x: number, y: number, z: number): [number, number, number] => {
    const l = Math.hypot(x, y, z);
    return [x / l, y / l, z / l];
  };
  const vh = n(a * v[0], a * v[1], v[2]);
  const flat = vh[0] * vh[0] + vh[1] * vh[1];
  const t1: [number, number, number] = flat > 0 ? [-vh[1] / Math.sqrt(flat), vh[0] / Math.sqrt(flat), 0] : [1, 0, 0];
  const t2 = [vh[1] * t1[2] - vh[2] * t1[1], vh[2] * t1[0] - vh[0] * t1[2], vh[0] * t1[1] - vh[1] * t1[0]] as const;
  const r = Math.sqrt(u1);
  const phi = 2 * Math.PI * u2;
  const p1 = r * Math.cos(phi);
  const s = 0.5 * (1 + vh[2]);
  const p2 = (1 - s) * Math.sqrt(1 - p1 * p1) + s * r * Math.sin(phi);
  const p3 = Math.sqrt(Math.max(0, 1 - p1 * p1 - p2 * p2));
  return n(
    a * (p1 * t1[0] + p2 * t2[0] + p3 * vh[0]),
    a * (p1 * t1[1] + p2 * t2[1] + p3 * vh[1]),
    Math.max(1e-6, p1 * t1[2] + p2 * t2[2] + p3 * vh[2]),
  );
}
