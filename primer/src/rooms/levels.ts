// Objects with levels. Each level is pieces that are there for rays of a range of widths; the next takes over
// where the last one ends. What a level must keep: what it blocks, and what it sends back, in the mean.

import { turnAbout, type Vec3 } from '../world/vec';
import type { Builder, PieceGiven, RoomSpec } from '../world/room';

const grey = (v: number): Vec3 => [v, v, v];
const Y: Vec3 = [0, 1, 0];
const FOREVER = 1e9;

/** gives every piece made inside `make` a range of widths */
export function level(b: Builder, from: number, until: number, make: () => void): void {
  const first = b.pieces.length;
  make();
  for (const p of b.pieces.slice(first) as PieceGiven[]) {
    p.from = from;
    p.until = until;
  }
}
const mean = (colors: Vec3[]): Vec3 =>
  [0, 1, 2].map((c) => colors.reduce((sum, x) => sum + x[c]!, 0) / colors.length) as Vec3;

const BOOKS: Vec3[] = [[0.6, 0.12, 0.1], [0.12, 0.25, 0.5], [0.7, 0.62, 0.4], [0.1, 0.35, 0.2], [0.45, 0.3, 0.15], [0.75, 0.72, 0.68], [0.3, 0.1, 0.3]];

/** a shelf of books: 5 boards, about 70 books · 5 boards and 4 rows · one box */
function shelf(b: Builder, at: Vec3): void {
  const wood: Vec3 = [0.4, 0.25, 0.14];
  const board = b.material({ matte: wood, roughness: 0.6 });
  const covers = BOOKS.map((c) => b.material({ matte: c, roughness: 0.7 }));
  const row = b.material({ matte: mean(BOOKS).map((c) => c * 0.85) as Vec3, roughness: 1 });
  const all = b.material({ matte: mean([...BOOKS, wood]).map((c) => c * 0.75) as Vec3, roughness: 1 });
  const P = (x: number, y: number, z: number): Vec3 => [at[0] + x, at[1] + y, at[2] + z];
  const boards = () => {
    for (let i = 0; i <= 4; i++) b.box(P(0, 0.02 + i * 0.4, 0), [0.6, 0.02, 0.15], board);
    b.box(P(-0.6, 0.82, 0), [0.02, 0.82, 0.15], board);
    b.box(P(0.6, 0.82, 0), [0.02, 0.82, 0.15], board);
  };
  level(b, 0, 0.12, () => {
    boards();
    for (let r = 0; r < 4; r++) {
      let x = -0.55;
      for (let k = 0; x < 0.5; k++) {
        const pick = (k * 7 + r * 5) % 11;
        const thick = 0.012 + 0.004 * (pick % 5);
        const tall = 0.12 + 0.018 * (pick % 4);
        b.box(P(x + thick, 0.04 + r * 0.4 + tall, 0.01), [thick - 0.001, tall, 0.11 + 0.004 * (pick % 3)], covers[pick % BOOKS.length]!);
        x += thick * 2 + (pick === 3 ? 0.06 : 0.002);
      }
    }
  });
  level(b, 0.12, 0.5, () => {
    boards();
    for (let r = 0; r < 4; r++) b.box(P(0, 0.04 + r * 0.4 + 0.145, 0.0), [0.56, 0.145, 0.11], row, { cover: 0.96 });
  });
  level(b, 0.5, FOREVER, () => b.box(P(0, 0.82, 0), [0.62, 0.82, 0.14], all, { cover: 0.93 }));
}

/** a chair: 4 legs, seat, 5 slats, rail · 2 pairs of legs, seat, back · two boxes */
function chair(b: Builder, at: Vec3, yaw: number): void {
  const paint: Vec3 = [0.12, 0.35, 0.4];
  const fine = b.material({ matte: paint, roughness: 0.4 });
  const dull = b.material({ matte: paint, roughness: 0.8 });
  const turn = turnAbout(Y, yaw);
  const c = Math.cos(yaw);
  const s = Math.sin(yaw);
  const P = (x: number, y: number, z: number): Vec3 => [at[0] + x * c + z * s, at[1] + y, at[2] - x * s + z * c];
  level(b, 0, 0.06, () => {
    for (const [x, z] of [[-1, -1], [1, -1], [-1, 1], [1, 1]] as const) b.box(P(x * 0.19, 0.22, z * 0.19), [0.018, 0.22, 0.018], fine, { turn });
    b.box(P(0, 0.455, 0), [0.22, 0.018, 0.22], fine, { turn });
    for (let i = 0; i < 5; i++) b.box(P(-0.16 + i * 0.08, 0.7, -0.2), [0.012, 0.23, 0.01], fine, { turn });
    b.box(P(0, 0.94, -0.2), [0.21, 0.03, 0.014], fine, { turn });
  });
  level(b, 0.06, 0.3, () => {
    b.box(P(0, 0.22, -0.19), [0.21, 0.22, 0.018], fine, { turn, cover: 0.17 });
    b.box(P(0, 0.22, 0.19), [0.21, 0.22, 0.018], fine, { turn, cover: 0.17 });
    b.box(P(0, 0.455, 0), [0.22, 0.018, 0.22], fine, { turn });
    b.box(P(0, 0.71, -0.2), [0.21, 0.26, 0.012], fine, { turn, cover: 0.4 });
  });
  level(b, 0.3, FOREVER, () => {
    b.box(P(0, 0.24, 0), [0.22, 0.24, 0.22], dull, { turn, cover: 0.5 });
    b.box(P(0, 0.72, -0.2), [0.21, 0.24, 0.04], dull, { turn, cover: 0.4 });
  });
}

/** a railing: posts and bars · a panel that covers as much as they do · the same, thicker */
function railing(b: Builder, at: Vec3, half: number): void {
  const iron: Vec3 = [0.1, 0.1, 0.11];
  const m = b.material({ matte: iron, roughness: 0.5 });
  const P = (x: number, y: number, z: number): Vec3 => [at[0] + x, at[1] + y, at[2] + z];
  const bars = Math.round(half * 20);
  level(b, 0, 0.06, () => {
    for (let i = 0; i <= bars; i++) b.cylinder(P(-half + (2 * half * i) / bars, 0.45, 0), 0.012, 0.45, m);
    b.box(P(0, 0.92, 0), [half, 0.02, 0.025], m);
    b.box(P(0, 0.06, 0), [half, 0.015, 0.015], m);
  });
  const cover = Math.min(1, (bars * 0.024) / (2 * half));
  level(b, 0.06, 0.25, () => {
    b.box(P(0, 0.45, 0), [half, 0.45, 0.012], m, { cover });
    b.box(P(0, 0.92, 0), [half, 0.02, 0.025], m);
  });
  level(b, 0.25, FOREVER, () => b.box(P(0, 0.47, 0), [half, 0.47, 0.06], m, { cover: cover * 1.02 }));
}

/** One object's detail, as a ray of any width sees it. */
export const levelsRoom: RoomSpec = {
  id: 'levels',
  name: 'Levels',
  question: 'What does a ray so wide see? A shelf, chairs and railings that declare their own simplifications.',
  camera: { target: [0, 0.6, 0.2], distance: 6, yaw: 0.35, pitch: 0.4, lens: 38 },
  exposure: 0.4,
  sky: { zenith: [0.2, 0.28, 0.45], horizon: [0.35, 0.36, 0.38], ground: grey(0.1) },
  sliders: [{ key: 'many', label: 'Chairs', min: 1, max: 12, step: 1, value: 4, about: 'more of the same' }],
  build(b, s) {
    const floor = b.material({ matte: [0.55, 0.5, 0.44], matte2: [0.4, 0.36, 0.3], pattern: 'checker', patternSize: 0.12, roughness: 0.6 });
    const wall = b.material({ matte: [0.66, 0.64, 0.6], roughness: 0.95 });
    b.box([0, -0.1, 0], [3.2, 0.1, 2.4], floor);
    b.box([0, 1.1, -2.1], [3.2, 1.1, 0.1], wall);
    shelf(b, [-1.7, 0, -1.8]);
    shelf(b, [-0.35, 0, -1.8]);
    for (let i = 0; i < s.many!; i++) {
      chair(b, [0.5 + (i % 4) * 0.62 - 0.4 * Math.floor(i / 4), 0, -0.9 + Math.floor(i / 4) * 0.8 + (i % 2) * 0.12], 0.5 + i * 1.1);
    }
    railing(b, [-1.2, 0, 0.6], 1.4);
    railing(b, [1.7, 0, 1.3], 1.0);
    b.ballLamp([0.4, 2.1, 0.9], 0.12, [34, 29, 22]);
    b.sun([-0.5, 0.75, 0.55], [17000, 15500, 13000]);
  },
  subject: { center: [0.72, 0.48, -0.9], radius: 0.6 },
};
