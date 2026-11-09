// THE MARKS. Each is what a good object reaches, measured on the six that are here, with room for its noise (see notes/objects.md); not a guess of what would be nice.
import type { Found } from './rules';
export interface Mark { /** at most, at least */ most?: number; least?: number; /** only said, not judged */ said?: true }
export const MARKS: Record<string, Mark> = {
  'rule.foot': { most: 4 }, 'rule.floats': { most: 0 }, 'rule.faces': { most: 0 }, 'rule.thing': { most: 0 }, 'rule.same': { most: 0 }, 'rule.size': { most: 8 }, 'rule.order': { least: 1.5 }, 'rule.stems': { most: 2 },
  'cost.parts': { most: 0.67 },
  'live.rests': { most: 1 }, 'live.upright': { most: 2 }, 'live.lands': { most: 0 }, 'live.sinks': { most: 1 }, 'live.repeats': { most: 0 }, 'live.pushed': { least: 2 }, 'live.weight': { said: true }, 'live.floats': { most: 0 }, 'live.walks': { least: 2 }, 'live.feet': { most: 1 }, 'live.together': { most: 6 },
};
export function judged(f: Found): { ok: boolean; mark: string } { const m = MARKS[f.id]; if (!m || m.said) return { ok: true, mark: 'said' }; const ok = Number.isFinite(f.value) && (m.most === undefined || f.value <= m.most) && (m.least === undefined || f.value >= m.least); return { ok, mark: [m.most === undefined ? '' : `at most ${m.most}`, m.least === undefined ? '' : `at least ${m.least}`].filter(Boolean).join(', ') }; }
