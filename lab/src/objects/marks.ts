// THE MARKS. Each is what good objects reach, measured on the six that are here, with room over the worst of them (see notes/objects.md); not a guess of what would be nice. Leaves have marks of their own where
// no surface can be followed.
import type { Found } from './rules';
import type { Shape } from './object';
export interface Mark { /** at most, at least */ most?: number; least?: number; /** the same, where the finest level is leaves */ leaves?: number; /** only said, not judged */ said?: true }
export const MARKS: Record<string, Mark> = {
  'rule.foot': { most: 4 }, 'rule.floats': { most: 0 }, 'rule.faces': { most: 0 }, 'rule.thing': { most: 0 }, 'rule.same': { most: 0 }, 'rule.size': { most: 8 }, 'rule.order': { least: 1.5 }, 'rule.stems': { most: 2 },
  'look.noon': { most: 9, leaves: 13 }, 'look.dusk': { most: 14, leaves: 14 }, 'look.lamp': { most: 14, leaves: 17 }, 'look.torch': { most: 17, leaves: 17 }, 'look.stops': { most: 0.25 },
  'seen.edge': { least: 50 },
  'swap.outline': { most: 1.1 }, 'swap.cover': { most: 6 }, 'swap.holes': { said: true }, 'swap.bright': { most: 0.12 }, 'swap.color': { most: 1.6, leaves: 2.6 }, 'swap.shine': { most: 0.3 }, 'swap.places': { most: 34 }, 'swap.shadow': { most: 16 }, 'swap.pop': { most: 12 },
  'wide.shadow': { most: 12 }, 'wide.bounce': { most: 2 }, 'calm.sparkle': { most: 1.7 }, 'calm.level': { most: 1.3 }, 'print.kept': { least: 75 }, 'print.seen': { least: 20 },
  'cost.parts': { most: 0.67 }, 'cost.ms': { said: true }, 'cost.boxes': { said: true }, 'cost.gain': { most: 1.02 },
  'live.rests': { most: 1 }, 'live.upright': { most: 2 }, 'live.lands': { most: 0 }, 'live.sinks': { most: 1 }, 'live.repeats': { most: 0 }, 'live.pushed': { least: 2 }, 'live.weight': { said: true }, 'live.floats': { most: 0 }, 'live.walks': { least: 2 }, 'live.feet': { most: 1 }, 'live.together': { most: 6 }, 'live.sways': { least: 0.5, most: 8 },
};
export function judged(f: Found, shape: Shape = 'pieces'): { ok: boolean; mark: string } { const m = MARKS[f.id]; if (!m) throw new Error('no mark for ' + f.id); if (m.said) return { ok: true, mark: 'said' }; const most = shape === 'leaves' ? m.leaves ?? m.most : m.most, ok = Number.isFinite(f.value) && (most === undefined || f.value <= most) && (m.least === undefined || f.value >= m.least); return { ok, mark: [m.least === undefined ? '' : `at least ${m.least}`, most === undefined ? '' : `at most ${most}`].filter(Boolean).join(', ') }; }
