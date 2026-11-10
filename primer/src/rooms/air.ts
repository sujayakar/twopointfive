import { turnAbout, type Vec3 } from '../world/vec';
import type { RoomSpec } from '../world/room';

const grey = (v: number): Vec3 => [v, v, v];

/** The three numbers of a volume, and its levels. */
export const smokeWindow: RoomSpec = {
  id: 'smoke-window',
  name: 'Smoke window',
  question: 'What does smoke take, turn and give? A sunbeam through a window, and four boxes: black, white, colored, glowing.',
  camera: { target: [0, 0.9, 0], distance: 7, yaw: 0.25, pitch: 0.28, lens: 40 },
  exposure: 1.4,
  moves: true,
  sky: { zenith: [0.15, 0.25, 0.5], horizon: [0.3, 0.33, 0.38], ground: grey(0.05) },
  sliders: [
    { key: 'haze', label: 'Haze in the room', min: 0, max: 0.4, step: 0.01, value: 0.1, about: 'a little scattering everywhere: the beam shows' },
    { key: 'thick', label: 'The boxes, how thick', min: 0, max: 4, step: 0.05, value: 1, about: 'times what they absorb, scatter and send' },
    { key: 'contrast', label: 'How uneven', min: 0, max: 3, step: 0.05, value: 1.4, about: '0: even. The more uneven, the more wrong an averaged level is' },
  ],
  build(b, s) {
    const floor = b.material({ matte: [0.5, 0.47, 0.43], matte2: [0.42, 0.39, 0.36], pattern: 'checker', patternSize: 0.5, roughness: 0.6 });
    const wall = b.material({ matte: [0.6, 0.58, 0.55], roughness: 0.95 });
    const mirror = b.material({ matte: grey(0.92), metal: 1, roughness: 0.02 });
    b.box([0, -0.1, 0], [3.4, 0.1, 2], floor);
    b.box([0, 1.2, -2.1], [3.4, 1.3, 0.1], wall);
    // the left wall has a window in it
    b.box([-3.3, 0.4, 0], [0.1, 0.5, 2], wall);
    b.box([-3.3, 2.2, 0], [0.1, 0.3, 2], wall);
    b.box([-3.3, 1.4, -1.35], [0.1, 0.5, 0.65], wall);
    b.box([-3.3, 1.4, 1.35], [0.1, 0.5, 0.65], wall);
    b.box([0, 2.55, 0], [3.4, 0.05, 2], wall);
    b.box([3.3, 1.2, 0], [0.1, 1.3, 2], wall);
    b.box([2.9, 0.9, -1.2], [0.5, 0.75, 0.02], mirror, { turn: turnAbout([0, 1, 0], -0.5) });
    b.opening([-3.3, 1.4, 0], [0, 0.5, 0], [0, 0, 0.7]);
    b.sun([-0.9, 0.3, 0.1], [70000, 63000, 52000]);
    const k = s.thick!;
    const box = { half: [0.42, 0.55, 0.42] as Vec3, contrast: s.contrast!, wave: 0.5, soft: 0.25 };
    const each = (x: number, z: number, absorb: Vec3, scatter: Vec3, emit: Vec3 = [0, 0, 0], g = 0.3) =>
      b.smoke({
        ...box, center: [x, 0.7, z], g,
        absorb: absorb.map((v) => v * k) as Vec3, scatter: scatter.map((v) => v * k) as Vec3, emit: emit.map((v) => v * k) as Vec3,
        drift: [0.05, 0.12, 0],
      });
    each(-2.0, -0.2, [0.05, 0.05, 0.05], [4, 4, 4], [0, 0, 0], 0.4);
    each(-0.8, -0.35, [0.1, 1.6, 2.6], [2.2, 0.7, 0.3]);
    each(0.45, -0.2, [3.2, 3.2, 3.2], [0.3, 0.3, 0.3]);
    each(1.8, -0.2, [1.5, 1.2, 0.8], [0.2, 0.2, 0.2], [2.2, 0.8, 0.2]);
    if (s.haze! > 0) b.smoke({ center: [0, 1.25, 0], half: [3.2, 1.25, 2], absorb: grey(0.002), scatter: grey(s.haze!), g: 0.2, soft: 0.02 });
  },
};
