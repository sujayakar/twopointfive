import { turnAbout, type Quat, type Vec3 } from '../world/vec';
import type { Place, RoomSpec } from '../world/room';

const grey = (v: number): Vec3 => [v, v, v];
const Y: Vec3 = [0, 1, 0];

/** Picking lamps; keeping still and changing light apart. */
export const lampHall: RoomSpec = {
  id: 'lamp-hall',
  name: 'Lamp hall',
  question: 'Which of 60 lamps should one ray ask? And what of the torch that swings?',
  camera: { target: [0, 0.7, 0], distance: 8, yaw: 0.35, pitch: 0.55, lens: 38 },
  exposure: 1.2,
  moves: true,
  sliders: [
    { key: 'lamps', label: 'Lamps', min: 4, max: 60, step: 1, value: 60, about: 'small colored balls behind slats' },
    { key: 'swing', label: 'Torch swings', min: 0, max: 1, step: 1, value: 1, about: 'a lamp that moves: its light gets a history of its own' },
  ],
  build(b, s) {
    const floor = b.material({ matte: grey(0.6), roughness: 0.6 });
    const wall = b.material({ matte: [0.55, 0.52, 0.48], roughness: 0.9 });
    const wood = b.material({ matte: [0.35, 0.22, 0.12], roughness: 0.5 });
    b.box([0, -0.1, 0], [4, 0.1, 3], floor);
    b.box([0, 1.2, -3.1], [4, 1.3, 0.1], wall);
    b.box([-4.1, 1.2, 0], [0.1, 1.3, 3.2], wall);
    for (let i = 0; i < 12; i++) b.box([-3.3 + i * 0.6, 0.9, -1.6], [0.05, 0.9, 0.12], wood);
    for (let i = 0; i < 8; i++) b.box([2.2, 0.9, -1.1 + i * 0.45], [0.12, 0.9, 0.04], wood);
    b.box([-1.2, 0.3, 0.8], [0.4, 0.3, 0.4], wood, { turn: turnAbout(Y, 0.5) });
    b.ball([0.6, 0.35, 0.7], 0.35, floor);
    b.cylinder([1.5, 0.5, 1.6], 0.2, 0.5, floor);
    const hue = (h: number): Vec3 =>
      [0, 1 / 3, 2 / 3].map((o) => 0.15 + 0.85 * Math.max(0, Math.cos(2 * Math.PI * (h + o))) ** 1.5) as Vec3;
    for (let i = 0; i < s.lamps!; i++) {
      const along = (i * 0.618034) % 1;
      const behind = i % 3 !== 2;
      const at: Vec3 = behind
        ? [-3.7 + 7 * along, 0.25 + ((i * 0.37) % 1) * 1.6, -2.3 - ((i * 0.23) % 1) * 0.5]
        : [2.8 + ((i * 0.41) % 1) * 0.9, 0.2 + ((i * 0.29) % 1) * 1.5, -1.4 + 3.6 * along];
      b.ballLamp(at, 0.04, hue(i * 0.137).map((c) => c * 55) as Vec3);
    }
    const torch = b.object();
    b.ballLamp([0, 0, 0], 0.07, [60, 52, 40], { object: torch, changing: true });
  },
  move(seconds, s) {
    const a = s.swing ? Math.sin(seconds * 1.6) * 1.1 : 0.4;
    return new Map<number, Place>([[1, { position: [Math.sin(a) * 1.8, 2.4 - Math.cos(a) * 0.9, 1.6], turn: [0, 0, 0, 1] }]]);
  },
};

/** Penumbras, and where still × moving goes wrong. */
export const fence: RoomSpec = {
  id: 'fence',
  name: 'Fence',
  question: 'How soft is a shadow? A lamp of any size behind a fence, and a ball that rolls past.',
  camera: { target: [0, 0.5, 0.4], distance: 6.5, yaw: 0.25, pitch: 0.65, lens: 38 },
  exposure: 0.2,
  moves: true,
  sky: { zenith: grey(0.01), horizon: grey(0.01), ground: grey(0.01) },
  sliders: [
    { key: 'size', label: 'Lamp, its radius (m)', min: 0.01, max: 0.6, step: 0.01, value: 0.15, about: 'the same power at any size: a larger one only makes softer shadows' },
    { key: 'roll', label: 'Ball rolls', min: 0, max: 1, step: 1, value: 1, about: 'something that moves in front of a lamp that does not' },
  ],
  build(b, s) {
    const floor = b.material({ matte: grey(0.7), roughness: 0.9 });
    const wood = b.material({ matte: [0.45, 0.28, 0.15], roughness: 0.7 });
    const rubber = b.material({ matte: [0.1, 0.3, 0.6], matte2: [0.8, 0.8, 0.75], pattern: 'checker', patternSize: 0.2, roughness: 0.6 });
    b.box([0, -0.1, 0], [5, 0.1, 4], floor);
    for (let i = 0; i < 11; i++) b.box([-2 + i * 0.4, 0.5, -0.8], [0.06, 0.5, 0.03], wood);
    b.box([0, 0.85, -0.85], [2.1, 0.05, 0.02], wood);
    b.box([0, 0.3, -0.85], [2.1, 0.05, 0.02], wood);
    const bright = 1.1 / (s.size! * s.size!);
    b.ballLamp([0, 1.5, -2.4], s.size!, [bright, bright * 0.9, bright * 0.75]);
    const ball = b.object();
    b.ball([0, 0, 0], 0.3, rubber, { object: ball });
  },
  move(seconds, s) {
    const x = s.roll ? Math.sin(seconds * 0.9) * 1.8 : 0.5;
    const turn: Quat = turnAbout([0, 0, 1], -x / 0.3);
    return new Map<number, Place>([[1, { position: [x, 0.3, -0.2], turn }]]);
  },
};

/** Rare light, leaks, lag, aiming through openings. */
export const door: RoomSpec = {
  id: 'door',
  name: 'Door',
  question: 'A bright room, a dark room, a door. Where does the light that comes through get found, and where does it leak?',
  camera: { target: [0.6, 0.8, 0], distance: 7.5, yaw: 0.35, pitch: 1.0, lens: 40 },
  exposure: 2.2,
  moves: true,
  sliders: [
    { key: 'open', label: 'Door, degrees', min: 0, max: 110, step: 1, value: 35, about: 'the less it is open, the rarer the light behind it' },
    { key: 'sway', label: 'Door moves', min: 0, max: 1, step: 1, value: 0, about: 'it swings by itself' },
    { key: 'wall', label: 'Wall, half as thick (m)', min: 0.01, max: 0.15, step: 0.01, value: 0.05, about: 'thin walls are where kept light leaks' },
  ],
  build(b, s) {
    const floor = b.material({ matte: [0.6, 0.55, 0.48], roughness: 0.8 });
    const wall = b.material({ matte: grey(0.72), roughness: 0.95 });
    const paint = b.material({ matte: [0.75, 0.3, 0.1], roughness: 0.9 });
    const wood = b.material({ matte: [0.4, 0.25, 0.14], roughness: 0.6 });
    const t = s.wall!;
    const H = 1.2;
    b.box([0, -0.1, 0], [3.6, 0.1, 1.9], floor);
    b.box([-3.5 - 0.1, H, 0], [0.1, H, 1.9], wall);
    b.box([3.5 + 0.1, H, 0], [0.1, H, 1.9], wall);
    b.box([0, H, -1.8], [3.6, H, 0.1], wall);
    b.box([0, H * 0.35, 1.8], [3.6, H * 0.35, 0.1], wall);
    // the wall between, with an opening from z = −0.1 to 0.8
    b.box([0, H, -0.9], [t, H, 0.8], wall);
    b.box([0, H, 1.25], [t, H, 0.45], wall);
    b.box([0, H * 2 - 0.15, 0.35], [t, 0.15, 0.45], wall);
    b.box([-2.2, 0.35, -1.1], [0.5, 0.35, 0.4], paint);
    b.ball([1.6, 0.4, -0.6], 0.4, wall);
    b.box([2.4, 0.5, 0.9], [0.3, 0.5, 0.3], paint, { turn: turnAbout(Y, 0.5) });
    b.panelLamp([-2, 2.3, 0.2], [0.5, 0, 0], [0, 0, 0.5], [16, 13, 9]);
    b.opening([0, 1.05, 0.35], [0, 1.05, 0], [0, 0, 0.45]);
    const leaf = b.object();
    b.box([0, H - 0.15, 0.45], [0.025, H - 0.16, 0.44], wood, { object: leaf });
  },
  move(seconds, s) {
    const degrees = s.sway ? 45 + 45 * Math.sin(seconds * 0.8) : s.open!;
    return new Map<number, Place>([[1, { position: [0, 0, -0.1], turn: turnAbout(Y, (degrees * Math.PI) / 180) }]]);
  },
};

/** Narrow light, the image's motion, and what is lost on purpose. */
export const stripFloor: RoomSpec = {
  id: 'strip-floor',
  name: 'Strip floor',
  question: 'What does a mirror show, the rougher it gets? And who finds the sun that a mirror throws on a wall?',
  camera: { target: [0, 0.6, 0], distance: 6.5, yaw: 0.35, pitch: 0.4, lens: 38 },
  exposure: 0.2,
  moves: true,
  sky: { zenith: [0.12, 0.2, 0.4], horizon: [0.35, 0.38, 0.42], ground: grey(0.1) },
  sliders: [{ key: 'turn', label: 'Things turn', min: 0, max: 1, step: 1, value: 1, about: 'the image in a mirror moves otherwise than the mirror' }],
  build(b) {
    const strips = b.material({ matte: grey(0.5), matte2: grey(0.5), pattern: 'strips', patternSize: 0.6, roughness: 0.02, roughness2: 0.7, metal: 1 });
    const wall = b.material({ matte: [0.6, 0.58, 0.55], roughness: 0.9 });
    const mirror = b.material({ matte: grey(0.95), metal: 1, roughness: 0.01 });
    const red = b.material({ matte: [0.75, 0.15, 0.1], roughness: 0.5 });
    const blue = b.material({ matte: [0.12, 0.3, 0.7], matte2: grey(0.8), pattern: 'checker', patternSize: 0.2, roughness: 0.4 });
    b.box([0, -0.1, 0], [2.4, 0.1, 2], strips);
    b.box([0, 1, -2.1], [2.6, 1.1, 0.1], wall);
    b.box([2.5, 0.8, -0.6], [0.04, 0.8, 0.9], mirror, { turn: turnAbout(Y, -0.35) });
    b.sun([-0.6, 0.8, 0.5], [26000, 23000, 19000]);
    const spinner = b.object();
    b.box([0.7, 0.45, 0], [0.18, 0.45, 0.18], red, { object: spinner });
    b.ball([-0.8, 0.5, 0], 0.32, blue, { object: spinner });
    b.ballLamp([0, 1.3, 0], 0.08, [14, 11, 7], { object: spinner, changing: true });
  },
  move(seconds, s) {
    return new Map<number, Place>([[1, { position: [0, 0, 0], turn: turnAbout(Y, s.turn ? seconds * 0.7 : 0.6) }]]);
  },
};

/** Lists, motion, histories. */
export const turntable: RoomSpec = {
  id: 'turntable',
  name: 'Turntable',
  question: 'What moved, and how does a history know? Things that stand, turn slowly, turn fast, and bend.',
  camera: { target: [0, 0.5, 0], distance: 6.5, yaw: 0.2, pitch: 0.5, lens: 38 },
  exposure: 0.4,
  moves: true,
  sky: { zenith: [0.1, 0.14, 0.2], horizon: [0.15, 0.16, 0.17], ground: grey(0.05) },
  sliders: [{ key: 'speed', label: 'Speed', min: 0, max: 3, step: 0.05, value: 1, about: 'of everything that moves' }],
  build(b) {
    const floor = b.material({ matte: grey(0.55), matte2: grey(0.42), pattern: 'checker', patternSize: 0.5, roughness: 0.7 });
    const still = b.material({ matte: [0.6, 0.55, 0.45], roughness: 0.6 });
    const slow = b.material({ matte: [0.2, 0.5, 0.25], matte2: grey(0.7), pattern: 'checker', patternSize: 0.25, roughness: 0.5 });
    const fast = b.material({ matte: [0.75, 0.2, 0.12], matte2: grey(0.8), pattern: 'checker', patternSize: 0.2, roughness: 0.4 });
    const bends = b.material({ matte: [0.2, 0.3, 0.7], roughness: 0.5 });
    b.box([0, -0.1, 0], [4, 0.1, 3], floor);
    b.box([-2.4, 0.5, 0], [0.3, 0.5, 0.3], still);
    const a = b.object();
    b.box([0, 0.5, 0], [0.35, 0.5, 0.35], slow, { object: a });
    const c = b.object();
    b.box([0, 0.35, 0], [0.5, 0.35, 0.08], fast, { object: c });
    for (let i = 0; i < 7; i++) {
      const bone = b.object();
      b.cylinder([0, 0.1, 0], 0.08, 0.11, bends, { object: bone });
    }
    b.panelLamp([0, 2.8, 1.2], [0.6, 0, 0], [0, 0.1, 0.58], [9, 8.2, 7]);
  },
  move(seconds, s) {
    const t = seconds * s.speed!;
    const places = new Map<number, Place>();
    places.set(1, { position: [-1, 0, 0], turn: turnAbout(Y, t * 0.5) });
    places.set(2, { position: [0.5, 0.02, 0], turn: turnAbout(Y, t * 7) });
    // a stalk that bends: each bone stands on the one below
    let at: Vec3 = [2.1, 0, 0];
    let lean = 0;
    for (let i = 0; i < 7; i++) {
      lean += 0.16 * Math.sin(t * 2 + i * 0.35);
      places.set(3 + i, { position: at, turn: turnAbout([0, 0, 1], lean) });
      at = [at[0] - Math.sin(lean) * 0.2, at[1] + Math.cos(lean) * 0.2, at[2]];
    }
    return places;
  },
};
