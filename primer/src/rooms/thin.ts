import { add, normalize, scale, turnAbout, turnBoth, type Quat, type Vec3 } from '../world/vec';
import type { Builder, RoomSpec } from '../world/room';
import { random } from '../mesh/shapes';
import { level } from './levels';

const grey = (v: number): Vec3 => [v, v, v];
const Y: Vec3 = [0, 1, 0];
const LEAF: { matte: Vec3; through: Vec3; roughness: number } = { matte: [0.1, 0.22, 0.05], through: [0.2, 0.32, 0.05], roughness: 0.45 };

/** the turn that lays a leaf's flat side along a normal, and spins it */
function leafTurn(next: () => number): { turn: Quat; normal: Vec3 } {
  // leaves look up more often than not
  const normal = normalize([next() - 0.5, 0.35 + next() * 0.6, next() - 0.5]);
  const axis = normalize([normal[2], 0, -normal[0]]);
  const tilt = turnAbout(axis, Math.acos(normal[1]));
  return { turn: turnBoth(tilt, turnAbout(Y, next() * 6.28)), normal };
}

interface CrownLevels {
  /** widths from which the leaves are cells of this size */
  cells: { from: number; size: number }[];
}

/** a crown of leaves about a point. Past some width its leaves are cells that stop a share of rays */
function crown(b: Builder, center: Vec3, radius: number, count: number, seed: number, levels: CrownLevels, startAt = 0): void {
  const next = random(seed);
  const leaf = b.material(LEAF);
  const HALF: Vec3 = [0.055, 0.0015, 0.03];
  const spots: Vec3[] = [];
  const leaves: { at: Vec3; turn: Quat }[] = [];
  for (let i = 0; i < count; i++) {
    const way = normalize([next() - 0.5, (next() - 0.5) * 0.8, next() - 0.5]);
    const far = radius * (0.45 + 0.55 * Math.sqrt(next()));
    const at = add(center, scale([way[0], way[1] * 0.75, way[2]], far));
    spots.push(at);
    leaves.push({ at, turn: leafTurn(next).turn });
  }
  if (startAt === 0) {
    level(b, 0, levels.cells[0]!.from, () => {
      for (const l of leaves) b.ball(l.at, HALF, leaf, { turn: l.turn, form: 'leaf' });
    });
  }
  levels.cells.forEach(({ from, size }, i) => {
    if (i + 1 < startAt) return;
    const until = levels.cells[i + 1]?.from ?? 1e9;
    const cells = new Map<string, number>();
    for (const p of spots) {
      const key = p.map((v) => Math.floor(v / size)).join(',');
      cells.set(key, (cells.get(key) ?? 0) + 1);
    }
    const area = Math.PI * HALF[0] * HALF[2];
    level(b, startAt > 0 && i + 1 === startAt ? 0 : from, until, () => {
      for (const [key, many] of cells) {
        const c = key.split(',').map(Number);
        // leaves hide one another: independent, so what gets through is exp(−the area that faces a ray, over the cell's)
        const cover = 1 - Math.exp((-many * area * 0.5) / (size * size));
        b.box(c.map((v) => (v + 0.5) * size) as Vec3, [size / 2, size / 2, size / 2], leaf, { cover, form: 'leaf' });
      }
    });
  });
}

/** Thin things; leaves as cells. */
export const thinRoom: RoomSpec = {
  id: 'lantern-and-tree',
  name: 'Lantern and tree',
  question: 'What comes through a leaf, and what does a crown look like to a wide ray? One leaf, a paper lantern, the same crown three times.',
  camera: { target: [0.1, 0.95, 0], distance: 7, yaw: 0.2, pitch: 0.28, lens: 38 },
  exposure: 0.6,
  sky: { zenith: [0.18, 0.3, 0.6], horizon: [0.4, 0.44, 0.5], ground: grey(0.08) },
  sliders: [
    { key: 'leaves', label: 'Leaves a crown', min: 20, max: 400, step: 10, value: 200, about: 'each a thin ball' },
    { key: 'sun', label: 'Sun', min: 0, max: 1, step: 0.05, value: 1, live: true, about: 'dim it to see the lantern' },
  ],
  power: (s) => new Map([[0, s.sun!]]),
  build(b, s) {
    const ground = b.material({ matte: [0.4, 0.36, 0.3], roughness: 1 });
    const bark = b.material({ matte: [0.25, 0.17, 0.11], roughness: 0.9 });
    const leaf = b.material(LEAF);
    const paper = b.material({ matte: [0.55, 0.48, 0.38], through: [0.36, 0.28, 0.17], roughness: 0.9 });
    b.sun([0.35, 0.8, -0.5], [26000, 23500, 19000]);
    b.box([0, -0.1, 0], [4.5, 0.1, 3], ground);
    const levels: CrownLevels = { cells: [{ from: 0.12, size: 0.14 }, { from: 0.4, size: 0.34 }] };
    [-2.1, -0.2, 1.7].forEach((x, i) => {
      b.cylinder([x, 0.55, -0.7], 0.05, 0.55, bark);
      crown(b, [x, 1.45, -0.7], 0.6, s.leaves!, 5, levels, i);
    });
    // one leaf, against the light
    b.ball([-1.35, 0.55, 0.9], [0.32, 0.003, 0.19], leaf, { turn: turnAbout([1, 0, 0.35], 1.15), form: 'leaf' });
    b.cylinder([-1.35, 0.2, 0.95], 0.012, 0.22, bark);
    // a lantern: sixteen sheets of paper round a lamp
    const L: Vec3 = [1.25, 0.42, 0.9];
    for (let i = 0; i < 16; i++) {
      const a = (i / 16) * 2 * Math.PI;
      b.box([L[0] + Math.cos(a) * 0.22, L[1], L[2] + Math.sin(a) * 0.22], [0.0015, 0.3, 0.045], paper, { turn: turnAbout(Y, -a) });
    }
    b.ballLamp([L[0], L[1], L[2]], 0.05, [55, 38, 20]);
  },
  subject: { center: [-2.1, 1.45, -0.7], radius: 0.75 },
};
