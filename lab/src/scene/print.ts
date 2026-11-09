// The colors of each invented package, shared by its label and by what is printed on.
export type RGB = [number, number, number];
const INK: RGB[] = [[0.86, 0.16, 0.13], [0.96, 0.78, 0.22], [0.12, 0.45, 0.33], [0.95, 0.93, 0.86], [0.15, 0.30, 0.55], [0.93, 0.50, 0.14], [0.45, 0.28, 0.17], [0.97, 0.80, 0.80], [0.55, 0.72, 0.35], [0.13, 0.14, 0.16], [0.60, 0.80, 0.88], [0.72, 0.13, 0.26], [0.98, 0.90, 0.66], [0.20, 0.52, 0.55], [0.83, 0.67, 0.44], [0.94, 0.94, 0.94], [0.38, 0.22, 0.42], [0.98, 0.65, 0.45], [0.28, 0.36, 0.20], [0.80, 0.85, 0.90]];
const PAIR = [3, 0, 12, 0, 1, 9, 12, 11, 15, 1, 4, 12, 2, 3, 6, 4, 12, 13, 1, 0], THIRD = [15, 6, 1, 13, 15, 15, 5, 15, 6, 15, 15, 1, 5, 1, 15, 5, 7, 15, 3, 11];
/** ground, accent, light */
export function inks(k: number): [RGB, RGB, RGB] { const i = (k * 7 + Math.floor(k / 20) * 3) % 20, swap = Math.floor(k / 20) % 3 === 1; const a = INK[i]!, b = INK[PAIR[i]!]!, c = INK[THIRD[i]!]!; return swap ? [b, a, c] : [a, b, c]; }
export const css = (c: RGB) => `rgb(${c.map(x => Math.round(x * 255)).join(' ')})`;
