// Every material, by name. `kind` picks the pattern in surface.wgsl; the rest are numbers.
import type { V3 } from '../math';
export interface Material { name: string; kind: number; base: V3; rough: number; metal?: number; f0?: number; glow?: number; b2?: V3; scale?: number }
export const MATERIALS: Material[] = [
  { name: 'plaster', kind: 11, base: [0.80, 0.80, 0.79], rough: 0.62 },
  { name: 'concrete', kind: 1, base: [0.60, 0.605, 0.60], rough: 0.72 },
  { name: 'paint', kind: 0, base: [0.86, 0.86, 0.86], rough: 0.30, f0: 0.045 },
  { name: 'epoxy', kind: 2, base: [0.78, 0.78, 0.78], rough: 0.10, f0: 0.045 },
  { name: 'tile', kind: 3, base: [0.80, 0.81, 0.81], rough: 0.14, b2: [0.42, 0.42, 0.41], scale: 0.3 },
  { name: 'steel', kind: 4, base: [0.62, 0.63, 0.65], rough: 0.26, metal: 1 },
  { name: 'roof', kind: 5, base: [0.74, 0.745, 0.74], rough: 0.78 },
  { name: 'wood', kind: 6, base: [0.46, 0.31, 0.19], rough: 0.42 },
  { name: 'tower', kind: 7, base: [0.66, 0.67, 0.68], rough: 0.8, b2: [0.05, 0.075, 0.10] },
  { name: 'water', kind: 8, base: [0.02, 0.07, 0.08], rough: 0.03, f0: 0.02 },
  { name: 'glass', kind: 9, base: [0.9, 0.95, 0.93], rough: 0.02, f0: 0.04 },
  { name: 'fabric', kind: 10, base: [0.80, 0.80, 0.80], rough: 0.92 },
  { name: 'glow', kind: 12, base: [1, 1, 1], rough: 0.5, glow: 1 },
  { name: 'soil', kind: 13, base: [0.10, 0.075, 0.05], rough: 0.95 },
  { name: 'bark', kind: 15, base: [0.22, 0.16, 0.11], rough: 0.9 },
  { name: 'gravel', kind: 16, base: [0.55, 0.54, 0.52], rough: 0.9 },
  { name: 'rubber', kind: 18, base: [0.045, 0.045, 0.05], rough: 0.55 },
  { name: 'hazard', kind: 19, base: [0.95, 0.72, 0.05], rough: 0.4, b2: [0.02, 0.02, 0.022] },
  { name: 'poolTile', kind: 3, base: [0.42, 0.72, 0.76], rough: 0.2, b2: [0.75, 0.8, 0.8], scale: 0.12 },
  { name: 'stone', kind: 3, base: [0.66, 0.64, 0.60], rough: 0.35, b2: [0.45, 0.44, 0.42], scale: 0.9 },
  { name: 'brass', kind: 4, base: [0.86, 0.62, 0.30], rough: 0.22, metal: 1 },
  { name: 'cloth', kind: 10, base: [0.62, 0.63, 0.66], rough: 0.85 },
  { name: 'skin', kind: 18, base: [0.55, 0.36, 0.27], rough: 0.5 },
  { name: 'plastic', kind: 0, base: [0.84, 0.84, 0.84], rough: 0.38 },
  { name: 'mirror', kind: 18, base: [0.95, 0.95, 0.95], rough: 0.015, metal: 1 },
];
export const M: Record<string, number> = Object.fromEntries(MATERIALS.map((m, i) => [m.name, i]));
/** the rooms' paints (as the eye gives them; the shader squares) */
export const C = { white: [1, 1, 1], orange: [1.0, 0.50, 0.11], red: [0.96, 0.16, 0.12], green: [0.33, 0.83, 0.42], blue: [0.10, 0.42, 0.93], cyan: [0.20, 0.80, 0.92], yellow: [1.0, 0.83, 0.12], black: [0.22, 0.22, 0.23], grey: [0.62, 0.63, 0.64], pink: [0.98, 0.45, 0.55], teal: [0.10, 0.62, 0.62] } satisfies Record<string, V3>;
