// THE DEFAULT LOOK — 2010s GWOT (the levels goal: "lean into 2010s GWOT for the assets to just have something default"): mud-brick and render, dust and
// gravel, rugs and cots and oil drums for the places (tools/levels/lib/gwotKit.ts is the kit, tools/levels/gwotSkin.ts dresses the loud levels' files in it), helmets and plate carriers against local dress for the men (dress only: weapons, goggles and kit are the gear part's own switch). The men's dress is LOOK ONLY:
// no weapon number is touched and no dice are drawn by any of it. A level's paint is that level's file: a wall's colour is in the light that comes round a room, which
// a man's eye reads (sim/glow.rs), so paint plays.
// It says what the men wear and how a leaf is painted, never which file a level is.
// No imports here: scene/, game/ and the tools all read it.

/** THE MEN'S DRESS FOLLOWS THE PLACE (the maintainer, 10-10: "middle eastern enemies also don't make sense for the office level"): the look's cast and colour
 *  tables — local dress for the defenders, tan issue for the operators — are for the look's places. On the OFFICE family (the bundled office, relief, the texel
 *  variants) the men are dressed as they were before the look: the first cast file and its tables, and of its defenders only the ones dressed for a building
 *  (rust/game_rules/src/outfit/cast.rs office_dress: no straw hat, no bandolier). Not a knob: it is a fact of the level the
 *  PAGE booted on, said once at boot (host/drawpage.ts, before the cast is fetched); a tool leaves it true: the look's dress.
 *  Looks only. */
let lookPlace = true;
/** the page says what its level is: true = one of the look's places (every level but the office family) */
export function setLookPlace(gwot: boolean) { lookPlace = gwot; }
/** do the men wear the look's dress here? No: the office, whose men are the first cast's, dressed for a building */
export function lookDress(): boolean { return lookPlace; }
/** is the level a page's query boots one of the look's places? No `levelFile` = the bundled office; `office.json` / `office-*.json` = the office family */
export function placeOfQuery(q: URLSearchParams): boolean {
  const lf = q.get('levelFile'); if (lf && /^[\w.-]+\.json$/.test(lf)) return !/^office(-[\w.-]+)?\.json$/.test(lf);   // (a name host/drawpage.ts's boot will not fetch boots the bundled office)
  return false;
}

// THE LOOK (the material picture: render/frame.ts lookConfig, shaders/look.wgsl; not the dress above, though they share the word): what on an address is its.
// One reader, so that the page and a run of several pages that is to be ONE look from its first page to its last (ui/benchrun.ts) cannot come to read an
// address two ways. A `look` value is a ROW of the look.
/** a `look` value that is a row of the look's: bare, or `key:value`. Any other word is no row and asks for nothing (`look=old` drew
 *  the picture before this one, `look=plain` the level's other file: an address that still says one is an ordinary page, not the lab) */
const lookRow = (v: string): boolean => v === '' || v.includes(':');
/** its rows (`?look`, `?look=key:value`) */
export function lookPageRows(q: URLSearchParams): string[] { return q.getAll('look').filter(lookRow); }
/** what this machine keeps OFF of the look and the address does not name, as the address would say it */
export function lookTrialsKept(q: URLSearchParams): string[] {
  let cut: string | null = null; try { cut = localStorage.getItem('twopointsix.cutboxes'); } catch { /* no store: on */ }   // (the cut boxes' own switch: scene/cut.ts initCutSwitch)
  return cut === '0' && !q.has('cutboxes') && !q.has('nocutboxes') ? ['cutboxes=0'] : []; }
/** the other words the page reads once, at boot, into its shader text and its level (host/drawpage.ts, scene/cut.ts: the look's geometry is cut boxes) */
const LOOK_PAGE_WORDS: ReadonlySet<string> = new Set(['longfoot', 'walked', 'firesnow', 'firesoot', 'emberfog', 'cutboxes', 'nocutboxes', 'smokelook', 'nosunglint', 'glintpick', 'holedepth', 'kept', 'lookworkers']);
/** a word of an address, decoded, is the look page's own */
export function lookPageWord(k: string, v: string): boolean { return k === 'look' ? lookRow(v) : LOOK_PAGE_WORDS.has(k); }
/** HOW KEPT THE PLACE IS, for the frame (render/frame.ts lookConfig.kept → look.wgsl lookKept): the address's `&kept=<0‥1>` for the visit, else the level's
 *  own word (its file's meta.kept), else 0, which is every level as it was drawn before the word. 0 = left to the weather and the years: a stuff that fades
 *  blotches and drifts, one that soaks has its damp foot and its runs, old paint is rubbed thin and gone a shade into its wall. 1 = cleaned and painted last
 *  night: none of that, and a weathered stuff's surface is dressed by the kept rules (dust at a wall's foot, shoe marks, and the knocked foot and
 *  brought-in dirt its stuff's `worn` says). Where feet go does not ask this word: a lane is trodden and paint walked off a floor however kept. */
export function lookKeptOf(levelKept: number | undefined, word: string | null): number {
  const w = word === null || word.trim() === '' ? NaN : Number(word), v = Number.isFinite(w) ? w : levelKept ?? 0;   // (an empty or blank word is no word: Number(' ') is 0, which would have called every level left to the weather)
  return Number.isFinite(v) ? Math.min(1, Math.max(0, v)) : 0;
}
