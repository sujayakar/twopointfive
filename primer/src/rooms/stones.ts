import { cutAt } from '../mesh/clusters';
import { column, normalsOf, pebble, random } from '../mesh/shapes';
import { meshAsset, type MeshAsset } from '../mesh/meshes';
import { turnAbout, turnVec, type Quat, type Vec3 } from '../world/vec';
import type { Builder, RoomSpec } from '../world/room';

const grey = (v: number): Vec3 => [v, v, v];
const Y: Vec3 = [0, 1, 0];

const stones = (): MeshAsset[] => [1, 2, 3].map((seed) => meshAsset(`pebble ${seed}`, 0.02, () => pebble(seed, 4)));
const brokenColumn = (): MeshAsset => meshAsset('column', 0.01, () => column(120, 150));

/** a stone, and the ball that stands in for it: its box's, a little smaller */
function stone(b: Builder, asset: MeshAsset, position: Vec3, scale: number, turn: Quat, material: number): void {
  b.mesh({ asset, position, scale, turn, material }, () => {
    const half = asset.low.map((l, k) => ((asset.high[k]! - l) / 2) * 0.95 * scale) as Vec3;
    const middle = asset.low.map((l, k) => ((asset.high[k]! + l) / 2) * scale) as Vec3;
    const off = turnVec(turn, middle);
    b.ball([position[0] + off[0], position[1] + off[1], position[2] + off[2]], half, material, { turn });
  });
}

/** Levels of meshes, equal brightness, the seam. */
export const stoneYard: RoomSpec = {
  id: 'stone-yard',
  name: 'Stone yard',
  question: 'How fine must a mesh be where you look, and how fine where only light does? Pebbles, a broken column, a ball.',
  camera: { target: [0, 0.45, 0], distance: 5.5, yaw: 0.4, pitch: 0.45, lens: 38 },
  exposure: 0.3,
  sky: { zenith: [0.25, 0.4, 0.75], horizon: [0.55, 0.6, 0.66], ground: grey(0.12) },
  sliders: [{ key: 'stones', label: 'Stones', min: 10, max: 400, step: 10, value: 160, about: 'each a mesh of 5,120 triangles' }],
  build(b, s) {
    const earth = b.material({ matte: [0.3, 0.25, 0.2], roughness: 1 });
    const pale = b.material({ matte: [0.62, 0.6, 0.55], roughness: 0.8 });
    const kinds = [[0.45, 0.43, 0.4], [0.36, 0.33, 0.31], [0.52, 0.47, 0.4]].map((c) => b.material({ matte: c as Vec3, roughness: 0.7 }));
    const red = b.material({ matte: [0.7, 0.12, 0.08], roughness: 0.35 });
    b.box([0, -0.1, 0], [6, 0.1, 6], earth);
    const next = random(11);
    const assets = stones();
    for (let i = 0; i < s.stones!; i++) {
      const a = next() * 6.28;
      const r = 0.35 + 2.6 * Math.sqrt(next());
      const size = 0.12 + 0.28 * next() ** 3;
      stone(b, assets[i % 3]!, [Math.cos(a) * r, size * 0.28, Math.sin(a) * r], size, turnAbout(Y, next() * 6.28), kinds[i % 3]!);
    }
    b.mesh({ asset: brokenColumn(), position: [-0.4, 0, -0.5], scale: 1.6, material: pale }, () => {
      b.cylinder([-0.4, 0.62, -0.5], 0.29, 0.62, pale);
      b.cylinder([-0.4, 0.09, -0.5], 0.36, 0.09, pale);
    });
    b.ball([0.75, 0.42, 0.6], 0.3, red);
    b.sun([0.6, 0.65, 0.45], [26000, 23500, 19500]);
  },
  subject: { center: [-0.4, 0.75, -0.5], radius: 0.85 },
};

/** What each form of one thing costs a ray. */
export const ladder: RoomSpec = {
  id: 'ladder',
  name: 'Ladder',
  question: 'What does each form of one thing cost a ray? The same stone as fine triangles, coarse triangles, one ball, and cells.',
  camera: { target: [0, 0.35, 0], distance: 7, yaw: 0.2, pitch: 0.3, lens: 38 },
  exposure: 0.3,
  sky: { zenith: [0.25, 0.4, 0.75], horizon: [0.55, 0.6, 0.66], ground: grey(0.12) },
  build(b) {
    const earth = b.material({ matte: [0.4, 0.36, 0.3], roughness: 1 });
    const gray = b.material({ matte: [0.45, 0.43, 0.4], roughness: 0.7 });
    b.box([0, -0.1, 0], [4, 0.1, 3], earth);
    const asset = stones()[0]!;
    const half = asset.low.map((l, k) => (asset.high[k]! - l) / 2) as Vec3;
    const size = 0.95;
    b.mesh({ asset, position: [-1.65, half[1] * size, 0], scale: size, material: gray });
    const coarse = meshAsset('pebble 1, coarse', 0.05, () => {
      const triangles = cutAt(asset.levels, 0.05);
      return { points: asset.levels.points, normals: normalsOf(asset.levels.points, triangles), triangles };
    });
    b.mesh({ asset: coarse, position: [-0.55, half[1] * size, 0], scale: size, material: gray });
    b.ball([0.55, half[1] * size, 0], half.map((h) => h * size * 0.95) as Vec3, gray);
    // cells: boxes that each stop the share of rays that the ball's part in them would
    const N = 6;
    for (let x = 0; x < N; x++) for (let y = 0; y < N; y++) for (let z = 0; z < N; z++) {
      const at = [x, y, z].map((i) => ((i + 0.5) / N) * 2 - 1) as Vec3;
      const cover = Math.min(1, Math.max(0, (1 - Math.hypot(...at)) * 2.5 + 0.5));
      if (cover <= 0) continue;
      b.box([1.65 + at[0] * half[0] * size, half[1] * size * (1 + at[1]), at[2] * half[2] * size],
        half.map((h) => (h * size) / N) as Vec3, gray, { cover });
    }
    b.sun([0.6, 0.65, 0.45], [26000, 23500, 19500]);
  },
  subject: { center: [0.55, 0.3, 0], radius: 0.5 },
};
