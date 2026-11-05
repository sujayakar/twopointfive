// The page's switches from its address, and the list of what the frozen stand holds off, as src/host/profile.ts has them.
import { K, N, LOOSE } from '../game/k';
import { M } from '../game/m';
export function pageSwitches(q: URLSearchParams): void {
  K.on = !q.has('noon');   // the rule: ON in the page; ?noon = as it was
  K.extra = q.get('extra') !== '0';
  K.old = q.has('old');
  K.viaList = K.snap = !q.has('novialist');   // (two rows given one value)
  K.panelOnly = q.get('panel') !== '0';
  K.listRead = !q.has('other');
  K.early = q.get('early') !== '0';
  const m = q.get('mode'); K.mode = m ?? 'fast';
  K.half1 = !q.has('nohalf');
  K.deep.on = !q.has('nodeep');
  K.rides = K.stay && q.get('rides') !== '0';   // (rides a row that stays: not a constant of the page)
  if (q.has('noforce')) K.forced = false; else K.forced = true;
  K.shared = !q.has('noshared');
  K.wrong = q.has('wrong');
  LOOSE.a = true;
  K.deep2.on = !q.has('nodeep2');
  K.iife = !q.has('noiife');
  for (const key of ['kl1'] as const) K[key] = true;
  K.selfref = K.selfref && !q.has('noself');
  K.hl1 = !q.has('nohl');
  N.wide = 6;
  N.w2 = 6;
  N.w3 = 6;
  K.sw = q.get('sw') ?? 'b';
  K.e2 = !q.has('noe2');
  K.e3 = !q.has('noe3');
  M.on = !q.has('nom');
  K.rec = !q.has('norec');
  K.j1 = !q.has('noj1');
  K.j2 = !q.has('noj2');
  K.deepn.on = !q.has('nodeepn');
  K.e4 = !q.has('noe4');
  K.e5 = !q.has('noe5');
  K.e6 = !q.has('noe6');
  K.sw2 = q.get('sw2') ?? 'b';
  N.h1 = 6;
  N.reach = 5;   // the page's reach
}
export const FROZEN_HOLDS: Record<string, string> = {
  'K.on': 'TPS_ON=1',
  'K.extra': 'TPS_EXTRA=1',
  'K.viaList': 'none: the journal sets it',
  'K.panelOnly': 'none',
  'K.stay': 'TPS_KEEP=1',
  'N.reach': 'none: the stand keeps the old reach',
  'N.wide': 'none',
  'K.snap': 'none',
  'K.early': 'none',
  'K.mode': 'none',
};
