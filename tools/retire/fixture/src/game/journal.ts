// A recording from before a key replays with it off, as src/game/journal.ts applyKnobs has it.
import { K } from './k';
export function applyKnobs(t: { K?: Record<string, unknown> }): void {
  for (const key of ['viaList', 'on'] as const) if (typeof t.K?.[key] !== 'boolean') K[key] = false;   // (from before both)
  for (const key of ['extra', 'stay'] as const) if (typeof t.K?.[key] !== 'boolean') K[key] = false;   // (one of these stays)
  if (!('panelOnly' in (t.K ?? {}))) K.panelOnly = false;
}
