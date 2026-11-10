import { add, normalize, sub, turnAbout, type Vec3 } from '../world/vec';
import type { RoomSpec } from '../world/room';

const grey = (v: number): Vec3 => [v, v, v];
const Y: Vec3 = [0, 1, 0];

/** Does any light get lost or made? */
export const furnace: RoomSpec = {
  id: 'furnace',
  name: 'Grey furnace',
  question: 'Does any light get lost or made? A white ball under an even sky must vanish, however rough its coat is.',
  camera: { target: [0, 0, 0], distance: 6, yaw: 0.3, pitch: 0.2, lens: 35 },
  exposure: -1.3,
  sky: { zenith: grey(1), horizon: grey(1), ground: grey(1) },
  sliders: [
    { key: 'paint', label: 'Paint', min: 0, max: 1, step: 0.01, value: 1, about: '1: the balls send back everything and cannot be seen' },
    { key: 'metal', label: 'Metal', min: 0, max: 1, step: 1, value: 0, about: 'a metal has no matte lobe: only its coat' },
  ],
  build(b, s) {
    for (let i = 0; i < 5; i++) {
      const m = b.material({ matte: grey(s.paint!), roughness: i / 4, metal: s.metal! });
      b.ball([(i - 2) * 1.05, 0, 0], 0.48, m);
    }
  },
};

/** Ten exact shapes: the bench's own room. */
export const shapes: RoomSpec = {
  id: 'shapes',
  name: 'Ten shapes',
  question: 'What does each pixel look at? Boxes, balls and cylinders, cut by planes: every hit is exact.',
  camera: { target: [0, 0.45, 0], distance: 6.5, yaw: 0.55, pitch: 0.5, lens: 35 },
  exposure: 0.5,
  sky: { zenith: [0.25, 0.4, 0.8], horizon: [0.6, 0.65, 0.7], ground: [0.1, 0.1, 0.1] },
  build(b) {
    const floor = b.material({ matte: grey(0.55), matte2: grey(0.4), pattern: 'checker', patternSize: 0.5, roughness: 0.7 });
    const red = b.material({ matte: [0.7, 0.12, 0.08], roughness: 0.5 });
    const blue = b.material({ matte: [0.1, 0.25, 0.6], roughness: 0.25 });
    const cream = b.material({ matte: [0.8, 0.75, 0.6], roughness: 0.8 });
    const green = b.material({ matte: [0.15, 0.45, 0.2], roughness: 0.6 });
    const steel = b.material({ matte: [0.9, 0.88, 0.85], roughness: 0.15, metal: 1 });
    b.box([0, -0.1, 0], [4, 0.1, 4], floor);
    b.box([-1.5, 0.4, 0.2], [0.4, 0.4, 0.4], red, { turn: turnAbout(Y, 0.5) });
    b.ball([0, 0.5, 0.6], 0.5, cream);
    b.ball([1.5, 0.3, 0.9], [0.5, 0.3, 0.25], blue, { turn: turnAbout(Y, 0.7) });
    b.cylinder([1.1, 0.6, -0.7], 0.35, 0.6, steel);
    b.cylinder([-0.6, 0.2, 1.6], 0.2, 0.6, green, { turn: turnAbout([0, 0, 1], Math.PI / 2) });
    // a hexagon: a box cut by six planes
    const six: [Vec3, number][] = [];
    for (let i = 0; i < 6; i++) six.push([[Math.cos((i * Math.PI) / 3), 0, Math.sin((i * Math.PI) / 3)], 0.4]);
    b.box([-0.6, 0.35, -1.1], [0.5, 0.35, 0.5], cream, { planes: six });
    // a wedge, and a ball with its top cut off
    b.box([2, 0.3, -0.2], [0.35, 0.3, 0.35], green, { planes: [[[1, 1, 0], 0.15]] });
    b.ball([-2.1, 0.35, 1.2], 0.45, steel, { planes: [[[0.3, 1, 0.2], 0.15]] });
    b.box([0.35, 0.06, 1.6], [0.3, 0.06, 0.2], red, { turn: turnAbout(Y, -0.4) });
    b.panelLamp([-1.2, 2.6, 1.6], [0.5, 0, 0], [0, 0.15, 0.48], [9, 8, 6.5]);
  },
};

/** How does light build up, bounce by bounce? */
export const colorBox: RoomSpec = {
  id: 'color-box',
  name: 'Color box',
  question: 'How does light build up, bounce by bounce? A red wall, a green wall, one lamp.',
  camera: { target: [0, 1.25, 0], distance: 5.6, yaw: 0, pitch: 0, lens: 35 },
  exposure: 0.3,
  sliders: [
    { key: 'lamp', label: 'Lamp, half its width (m)', min: 0.02, max: 1.1, step: 0.01, value: 0.35, about: 'the same power at any size' },
    { key: 'paint', label: 'Walls send back', min: 0, max: 0.98, step: 0.01, value: 0.75, about: 'near 1, light goes round and round' },
  ],
  build(b, s) {
    const p = s.paint!;
    const white = b.material({ matte: grey(p), roughness: 0.9 });
    const red = b.material({ matte: [p * 0.85, p * 0.08, p * 0.06], roughness: 0.9 });
    const green = b.material({ matte: [p * 0.15, p * 0.6, p * 0.12], roughness: 0.9 });
    const T = 0.1;
    b.box([0, -T, 0], [1.25 + T, T, 1.25], white);
    b.box([0, 2.5 + T, 0], [1.25 + T, T, 1.25], white);
    b.box([0, 1.25, -1.25 - T], [1.25 + T, 1.25 + 2 * T, T], white);
    b.box([-1.25 - T, 1.25, 0], [T, 1.25, 1.25], red);
    b.box([1.25 + T, 1.25, 0], [T, 1.25, 1.25], green);
    b.box([-0.45, 0.75, -0.4], [0.37, 0.75, 0.37], white, { turn: turnAbout(Y, 0.3) });
    b.box([0.45, 0.37, 0.3], [0.37, 0.37, 0.37], white, { turn: turnAbout(Y, -0.3) });
    const h = s.lamp!;
    const bright = 1.9 / (h * h * 4);
    b.panelLamp([0, 2.48, 0], [h, 0, 0], [0, 0, h], [bright * 10, bright * 8, bright * 5.5]);
  },
};

/** When to aim at lamps, when to follow the lobe. */
export const plates: RoomSpec = {
  id: 'plates',
  name: 'Plates',
  question: 'When to aim at lamps, and when to follow the lobe? Four plates from mirror to rough, four lamps from small to large.',
  camera: { target: [0, 0.9, 0], distance: 6, yaw: 0, pitch: 0.28, lens: 38 },
  exposure: 1.5,
  sky: { zenith: grey(0.004), horizon: grey(0.004), ground: grey(0.004) },
  build(b) {
    const dark = b.material({ matte: grey(0.25), roughness: 1 });
    b.box([0, -0.1, 0], [4, 0.1, 4], dark);
    b.box([0, 2, -2.4], [4, 2, 0.1], dark);
    const EYE: Vec3 = [0, 2.56, 5.77];
    const LAMPS_AT: Vec3 = [0, 2.25, -1.4];
    const rough = [0.06, 0.18, 0.32, 0.55];
    rough.forEach((r, i) => {
      const steel = b.material({ matte: grey(0.75), metal: 1, roughness: r });
      // tilted so that the middle of each mirrors the lamps to the eye
      const at: Vec3 = [0, 1.15 - i * 0.28, -1 + i * 0.72];
      const half = add(normalize(sub(EYE, at)), normalize(sub(LAMPS_AT, at)));
      b.box(at, [1.6, 0.02, 0.3], steel, { turn: turnAbout([1, 0, 0], Math.atan2(half[2], half[1])) });
    });
    const sizes = [0.03, 0.1, 0.22, 0.42];
    const colors: Vec3[] = [[1, 0.75, 0.5], [0.6, 1, 0.6], [0.5, 0.75, 1], [1, 0.6, 0.85]];
    sizes.forEach((radius, i) => {
      const bright = 0.09 / (radius * radius);
      b.ballLamp([-1.35 + i * 0.9, 2.25, -1.4], radius, colors[i]!.map((c) => c * bright) as Vec3);
    });
  },
};
