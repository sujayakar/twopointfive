// A recording from before a key replays with it off, as src/game/journal.ts applyKnobs has it.
import { K } from './k';
export function applyKnobs(t: { K?: Record<string, unknown> }): void {
  for (const key of ['viaList', 'on'] as const) if (typeof t.K?.[key] !== 'boolean') K[key] = false;   // (from before both)
  for (const key of ['extra', 'stay'] as const) if (typeof t.K?.[key] !== 'boolean') K[key] = false;   // (one of these stays)
  if (!('panelOnly' in (t.K ?? {}))) K.panelOnly = false;
  if ((t.K as Partial<typeof K> | undefined)?.snap === undefined) K.snap = false;   // (a recording's own word for the row, read off the recording)
  for (const key of SHARED) if (typeof t.K?.[key] !== 'boolean') K[key] = false;
}
/** a list of keys two things read */
const SHARED = ['shared', 'stay'] as const;
export const SHARED_COUNT = SHARED.length;
