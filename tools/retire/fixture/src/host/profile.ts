// The page's switches from its address, and the list of what the frozen stand holds off, as src/host/profile.ts has them.
import { K, N } from '../game/k';
export function pageSwitches(q: URLSearchParams): void {
  K.on = !q.has('noon');   // the rule: ON in the page; ?noon = as it was
  K.extra = q.get('extra') !== '0';
  K.old = q.has('old');
  K.viaList = !q.has('novialist');
  K.panelOnly = q.get('panel') !== '0';
  K.listRead = !q.has('other');
  N.reach = 5;   // the page's reach
}
export const FROZEN_HOLDS: Record<string, string> = {
  'K.on': 'TPS_ON=1',
  'K.extra': 'TPS_EXTRA=1',
  'K.viaList': 'none: the journal sets it',
  'K.panelOnly': 'none',
  'K.stay': 'TPS_KEEP=1',
  'N.reach': 'none: the stand keeps the old reach',
};
