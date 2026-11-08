// HOW MANY COME: the squad's size as an address says it and as a row words it. A leaf, for whoever reads a mission's link.
/** how many teammates stand beside Sam */
export type SquadSize = 0 | 1 | 2 | 3 | 4;
export const SQUAD_SIZES: SquadSize[] = [0, 1, 2, 3, 4];

/** a number as a squad size, or null (not an integer 0‥4) */
export function asSquadSize(v: unknown): SquadSize | null { const n = typeof v === 'number' ? v : typeof v === 'string' && /^\d$/.test(v.trim()) ? Number(v) : NaN; return n === 0 || n === 1 || n === 2 || n === 3 || n === 4 ? n : null; }
/** The squad size a query string asks for: `?squad=N` (0‥4), else `?squad4` = 4 (unless `flag4` is false: the rescue never read that flag), else null — the
 *  scenario's default. A `squad=` that is no size (`squad=9`, `squad=many`) asks for nothing. */
export function squadOfQuery(search: string | URLSearchParams, flag4 = true): SquadSize | null {
  const q = typeof search === 'string' ? new URLSearchParams(search) : search;
  return asSquadSize(q.get('squad')) ?? (flag4 && q.has('squad4') ? 4 : null);
}

/** 'solo' / 'you + one' / 'you + the pair' / 'you + three' / 'you + four' */
export function sizeWords(n: number): string { return n >= 4 ? 'you + four' : n === 3 ? 'you + three' : n === 2 ? 'you + the pair' : n === 1 ? 'you + one' : 'solo'; }
/** the short tag: 'solo' / 'one' / 'pair' / 'three' / 'four' */
export function sizeTag(n: number): string { return n >= 4 ? 'four' : n === 3 ? 'three' : n === 2 ? 'pair' : n === 1 ? 'one' : 'solo'; }

/** THE QUERY STRING DEPLOY LOADS: the row's own (`pick` — scenarioList.ts pickSearch, with whatever else rides on it) with the size on it. At the scenario's
 *  own size (`own`) it is `pick` untouched (but for an older `squad=` that is not the row's own); otherwise `squad4` and any older `squad=` are dropped and `squad=N` added. `loadout` never rides along. */
export function squadSearch(pick: string, n: SquadSize, own: number): string {
  const q = new URLSearchParams(pick.replace(/^[^?]*\?/, '')); q.delete('loadout');
  const kept = n === own && own !== 2 && q.get('squad') === String(own) && !q.has('squad4');   // (a row whose own size IS its `squad=N` — the four's rescue: dropped, the page would stage the pair)
  if (!kept) q.delete('squad');
  if (n !== own) { q.delete('squad4'); q.set('squad', String(n)); }
  const parts: string[] = []; q.forEach((v, k) => parts.push(v === '' ? encodeURIComponent(k) : `${encodeURIComponent(k)}=${encodeURIComponent(v)}`));   // (bare flags left bare: `?roomclear`, not `?roomclear=`)
  return parts.length ? '?' + parts.join('&') : '';
}
