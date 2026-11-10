// AN OBJECT WITH OTHER LEVELS, to set them side by side: name~0.08,0.16 its finest and blocks of cells of those sizes; name@0.08,0.16 its finest and its coarsest hand made level, taking over where those would.
import { type V3, add, sub } from '../math';
import { F } from '../scene/world';
import { M } from '../scene/materials';
import { build, bounds } from './rules';
import { traced } from './trace';
import { Block, type Kind } from './voxels';
import { calibrate } from './looks';
import type { Level, Made, ObjectKind, Put } from './object';
const kept = new Map<string, { words: Uint32Array; at: V3; half: V3; many: number }>();
export function blockOf(m: Made, h: number, kind: Kind = {}, gain = true) { const id = [m.name, h, JSON.stringify(kind), gain].join(); let k = kept.get(id); if (!k) { const b0 = bounds(build(m, 0)), e = h * 0.02, t = traced(m, 0), b = new Block(t, sub(b0.lo, [e, Math.min(e, Math.max(0, b0.lo[1])), e]), add(b0.hi, [e, e, e]), h, kind); if (gain) b.gain(calibrate(b, t)); const half = b.n.map(v => v * h / 2) as V3; kept.set(id, k = { words: b.pack(/grass/.test(m.name)), at: add(b.lo, half), half, many: b.many }); } return k; }
export const voxLevel = (m: Made, h: number, error: number, kind: Kind = {}): Level => ({ shape: 'voxels', error, less: `cells of ${(h * 100).toFixed(0)} cm`, put(p: Put) { const k = blockOf(m, h, kind); p.add({ at: k.at, half: [...k.half] as V3, mat: M['lamp shade']!, vox: k.words, flags: F.VOX }); } });
export function variant(kinds: Record<string, ObjectKind>, name: string): Made | null { const q = /^(\w+)([~@])([\d.,]+)(?:\/(\w+))?$/.exec(name); if (!q) return kinds[name]?.make() ?? null; const m = kinds[q[1]!]?.make(); if (!m) return null; const at = q[3]!.split(',').map(Number), last = m.levels[m.levels.length - 1]!, kind: Kind = q[4] === 'open' ? { open: true } : {}; return { ...m, title: m.title + (q[2] === '~' ? ', cells' : ', hand made'), levels: [m.levels[0]!, ...at.map((h): Level => q[2] === '~' ? voxLevel(m, h, h, kind) : { ...last, error: h })] }; }
