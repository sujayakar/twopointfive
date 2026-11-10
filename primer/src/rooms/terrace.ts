// The prototype's terrace corner: its boxes, tubes and balls as they stand there, its 22,000 leaves as cells. Carried
// over once, as numbers (terrace.json).

import type { Vec3 } from '../world/vec';
import type { RoomSpec } from '../world/room';
import data from './terrace.json';

const SHAPES = ['box', 'ball', 'cylinder'] as const;
/** what the numbers are measured from: the middle of the corner's floor */
const FROM: Vec3 = [6.6, 3.4, 1.1];
const moved = (x: number, y: number, z: number): Vec3 => [x - FROM[0], y - FROM[1], z - FROM[2]];

export const terrace: RoomSpec = {
  id: 'terrace',
  name: 'Terrace corner',
  question: 'The exam: a corner of the prototype’s rooftop, all of this renderer at once, beside the truth.',
  camera: { target: [0, 0.6, 0], distance: 10, yaw: 2.2, pitch: 0.72, lens: 38 },
  exposure: 0.3,
  sky: { zenith: [0.2, 0.38, 0.8], horizon: [0.55, 0.62, 0.7], ground: [0.15, 0.15, 0.15] },
  sliders: [
    { key: 'day', label: 'Sun', min: 0, max: 1, step: 0.02, value: 1, live: true, about: 'down to nothing: only the terrace’s lamps are left' },
    { key: 'lamps', label: 'Terrace lamps', min: 0, max: 1, step: 0.02, value: 0.3, live: true, about: 'two globes on the wall' },
  ],
  power: (s) => new Map([[0, s.day!], ...data.lamps.map((_, i): [number, number] => [i + 1, s.lamps!])]),
  build(b) {
    const materials = data.materials.map(([r, g, bl, roughness, metal]) =>
      b.material({ matte: [r!, g!, bl!], roughness: roughness!, metal: metal! }));
    b.sun([0.45, 0.75, 0.5], [26000, 23500, 19500]);
    for (const l of data.lamps) {
      const at = moved(l.at[0]!, l.at[1]!, l.at[2]!);
      if (l.kind === 'sphere') b.ballLamp(at, l.u[0]!, l.radiance as Vec3);
      else b.panelLamp(at, l.u as Vec3, (l.v as Vec3).map((v) => -v) as Vec3, l.radiance as Vec3);
    }
    for (const p of data.pieces) {
      const [shape, x, y, z, hx, hy, hz, qx, qy, qz, qw, material] = p as number[];
      b.pieces.push({
        shape: SHAPES[shape!]!, center: moved(x!, y!, z!), half: [hx!, hy!, hz!], turn: [qx!, qy!, qz!, qw!], material: materials[material!]!,
      });
    }
    const leaf = b.material({ matte: [0.09, 0.19, 0.05], through: [0.1, 0.17, 0.03], roughness: 0.6 });
    const s = data.cellSize;
    for (const c of data.cells) {
      const [i, j, k, cover] = c as number[];
      b.box(moved((i! + 0.5) * s, (j! + 0.5) * s, (k! + 0.5) * s), [s / 2, s / 2, s / 2], leaf, { cover: cover!, form: 'leaf' });
    }
  },
};
