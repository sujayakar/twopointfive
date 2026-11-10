// THE RAYS OF THE PINNED PIXEL, drawn over the panels.

import { makeBuffer, readBack, type Gpu } from '../gpu/device';
import type { Graph } from '../gpu/graph';
import { struct, u32, unpack, vec3f } from '../gpu/layout';
import type { Eye } from '../world/camera';
import { dot, sub, type Vec3 } from '../world/vec';
import drawCode from '../wgsl/draw.wgsl';

/** what a line stands for: its name in shaders, its color, and what the page calls it */
export const RAY_KINDS = [
  ['seen', '#ffe08a', 'a lamp, seen'],
  ['blocked', '#e2725b', 'a lamp, hidden'],
  ['wide', '#6bb8ff', 'a wide ray'],
  ['narrow', '#8ee59a', 'a narrow ray'],
  ['onward', '#8d93a8', 'a later bounce'],
  ['candidate', '#6b6250', 'a lamp that was looked at'],
  ['reused', '#d79bff', 'a neighbor that was asked'],
  ['cache', '#5fd4c4', 'a cache entry and its rays'],
] as const;
const MOST_LINES = 256;

const Line = struct('Line', { start: vec3f(''), kind: u32(''), end: vec3f(''), spare: u32('') });
const Drawn = struct('Drawn', {});
Drawn.wgsl = `struct Drawn { count: atomic<u32>, spare0: u32, spare1: u32, spare2: u32, lines: array<Line, ${MOST_LINES}> }`;

export interface DrawnLine {
  start: Vec3;
  end: Vec3;
  kind: number;
}

export class Rays {
  lines: DrawnLine[] = [];
  private buffer: GPUBuffer;

  constructor(private gpu: Gpu, graph: Graph) {
    this.buffer = makeBuffer(gpu, 'drawn', 16 + MOST_LINES * Line.stride, GPUBufferUsage.STORAGE);
    graph.addShared('drawn', { type: 'Drawn', structs: [Line, Drawn], access: 'read_write', buffer: () => this.buffer });
    const names = RAY_KINDS.map(([name], i) => `const RAY_${name.toUpperCase()} = ${i}u;`).join('\n');
    graph.addModule('draw', { code: `const MOST_LINES = ${MOST_LINES}u;\n${names}\n${drawCode}`, needs: [], shared: ['drawn'] });
  }

  start(encoder: GPUCommandEncoder): void {
    encoder.clearBuffer(this.buffer, 0, 16);
  }

  private reading = false;
  async read(): Promise<void> {
    if (this.reading) return;
    this.reading = true;
    try {
      const data = await readBack(this.gpu, this.buffer, 0, 16 + MOST_LINES * Line.stride);
      const count = Math.min(MOST_LINES, data.getUint32(0, true));
      this.lines = Array.from({ length: count }, (_, i) => {
        const v = unpack(Line, data, 16 + i * Line.stride);
        return { start: v.start as Vec3, end: v.end as Vec3, kind: v.kind as number };
      });
    } finally {
      this.reading = false;
    }
  }
}

/** where a point is in the picture, 0 to 1; null behind the eye */
export function project(e: Eye, p: Vec3): [number, number] | null {
  const v = sub(p, e.eye);
  const z = dot(v, e.forward);
  if (z < 1e-3) return null;
  const x = dot(v, e.right) / (dot(e.right, e.right) * z);
  const y = dot(v, e.up) / (dot(e.up, e.up) * z);
  return [x / 2 + 0.5, 0.5 - y / 2];
}

/** a line in space as a line in the picture, cut off at the eye's plane */
export function projectLine(e: Eye, a: Vec3, b: Vec3): [[number, number], [number, number]] | null {
  const NEAR = 0.02;
  const za = dot(sub(a, e.eye), e.forward);
  const zb = dot(sub(b, e.eye), e.forward);
  if (za < NEAR && zb < NEAR) return null;
  const cut = (p: Vec3, q: Vec3, zp: number, zq: number): Vec3 => {
    const t = (NEAR * 1.01 - zp) / (zq - zp);
    return [p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t, p[2] + (q[2] - p[2]) * t];
  };
  const p = project(e, za < NEAR ? cut(a, b, za, zb) : a);
  const q = project(e, zb < NEAR ? cut(b, a, zb, za) : b);
  return p && q ? [p, q] : null;
}
