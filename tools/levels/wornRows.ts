// A PALETTE'S ROWS ARE WRITTEN ONLY WHERE A FILE WEARS THEM (tools/levels/export.ts: a generator's `PALETTE` export). A row added to a palette tomorrow
// then moves no file that does not use it. WHO WEARS A PAINT is one list, tools/levels/paintUsers.ts (the boxes, the props' parts, a gear screen's own
// boxes); a box's shape pieces are asked here beside it, since that list does not visit them. A wearer that names alternatives ("first|second": a gear
// screen's box, read as the first the file holds) wears every one of them for this purpose: a row some reading of the file would ask for is kept.
import type { LevelData } from '../../src/scene/levelData';
import { paintUsers, type Data } from './paintUsers';

/** every paint name the file's things say they wear (the `materials` table itself is not a wearer) */
function wornIn(data: LevelData): Set<string> {
  const worn = new Set<string>(), say = (m: unknown) => { if (typeof m === 'string') for (const n of m.split('|')) worn.add(n); };
  for (const { o } of paintUsers(data as unknown as Data)) { say(o.material); for (const t of ((o as { shape?: { with?: { material?: string }[] } }).shape?.with ?? [])) say(t.material); }
  return worn;
}
/** the names of `palette`'s rows that nothing in `data` wears, taken out of its materials; the names are handed back. A ROW IS THE PALETTE'S BY ITS NAME
 *  AND ITS NUMBERS: a material of the file that only shares a name with a palette row (the office's `wall` beside a palette's) is not the palette's and is
 *  left alone */
export function dropUnworn(data: LevelData, palette: Readonly<Record<string, { albedo: readonly number[] }>>): string[] {
  const worn = wornIn(data), mats = data.materials as Record<string, { albedo: readonly number[] }>;
  const same = (m: string) => Object.hasOwn(mats, m) && mats[m].albedo.join(',') === palette[m].albedo.join(',');
  const out = Object.keys(palette).filter(m => !worn.has(m) && same(m));
  if (process.env.PALETTE_BREAK === '3') return [];   // (tools/qa/palette-check.ts's third red path: every row kept)
  for (const m of out) delete mats[m];
  return out;
}
