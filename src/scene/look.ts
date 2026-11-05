// THE DEFAULT LOOK — 2010s GWOT (the levels goal: "lean into 2010s GWOT for the assets to just have something default"): mud-brick and render, dust and
// gravel, rugs and cots and oil drums for the places (scene/gwotKit.ts is the kit; the loud levels that already read as such places have a second file in
// it), helmets and plate carriers against local dress for the men (dress only: weapons, goggles and kit are the gear part's own switch). LOOK ONLY: nothing a man
// collides with, hides behind or sees through differs between a level's two files, no weapon number is touched, and no dice are drawn by any of it — headless no
// run differs by it. ON THE PAGE the repainted walls bounce other light, the GPU meter reads that bounce, and a sighting can so differ between a level's two files.
// ONE SWITCH (and under it `suit`, the sneaking suit: on whenever the look is, looks only). OFF in this module, so everything headless (every probe, frozen digest and level load) stands as it was; the PAGE turns it on at boot before the
// level and the cast are fetched (host/profile.ts pageSwitches: on unless the URL carries ?look=plain), a harness run under TPS_LOOK=1.
// No imports here: scene/, game/ and the tools all read it.
import type { BoxStuff } from './boxes';
export const LOOK = {
  on: false,
  /** THE SNEAKING SUIT (the maintainer, 10-10: "sam should also be in a black sneaking suit in stealth missions (probs not loud ones)"): with the look on, Sam —
   *  and the partner of a quiet mission as two — wears a black suit with no helmet and no carrier on every mission that is not a squad's loud one (game/cast.ts
   *  sneakSuit: the parts; render/charMaterials.ts SNEAK_SUIT: the cloth). False (`?suit=0`) = the operators' issue dress on every mission, as the look first shipped.
   *  Nothing with the look off. Looks only. */
  suit: true,
};

/** THE SUIT FOR A PICTURE: a host that stages men for a picture and not for play (the Team & kit picture stands a quiet mission's team at a way in staged as a room
 *  clear — host/kitView.ts — while the card says the suit as issued) says which mission the men are DRESSED for, whatever is staged: 'quiet' = Sam and the staged
 *  team in the suit, null = no mission (Sam alone in it, as on the plain night). Undefined — as it is everywhere unless a host says otherwise — = the scenario that
 *  is up decides, as ever (game/cast.ts sneakSuit reads this first). Not a knob and in no table: one window's host says it for its own picture. Under LOOK.on and
 *  LOOK.suit like the suit itself; the rifle's black furniture follows the suit (combat.ts), where the rifle hangs does not (kitanim.ts KIT.sling). Looks only. */
let suitAsked: 'quiet' | null | undefined;
export function setSuitAs(k: 'quiet' | null | undefined) { suitAsked = k; }
/** the mission a host asked the men to be dressed for (undefined: none asked — the game's own scenario decides) */
export function suitAs(): 'quiet' | null | undefined { return suitAsked; }

/** THE MEN'S DRESS FOLLOWS THE PLACE (the maintainer, 10-10: "middle eastern enemies also don't make sense for the office level"): the look's cast and colour
 *  tables — local dress for the defenders, tan issue for the operators — are for the look's places. On the OFFICE family (the bundled office, relief, the texel
 *  variants) the men are dressed as they were before the look: the first cast file and its tables, and of its defenders only the ones dressed for a building
 *  (game/cast.ts officeDress: no straw hat, no bandolier). The sneaking suit is the look's on every level (LOOK.suit). Not a knob: it is a fact of the level the
 *  PAGE booted on, said once at boot (main.ts, before the cast is fetched); everything headless leaves it true, so `LOOK.on` alone still means the look's dress there.
 *  `?look=plain` = no look at all, as ever. Looks only. */
let lookPlace = true;
/** the page says what its level is: true = one of the look's places (every level but the office family) */
export function setLookPlace(gwot: boolean) { lookPlace = gwot; }
/** do the men wear the look's dress here? (the look on AND a place of the look's) */
export function lookDress(): boolean { return LOOK.on && lookPlace; }
/** the look on, in a place that is not the look's (the office): the men of the first cast, dressed for a building */
export function lookOffice(): boolean { return LOOK.on && !lookPlace; }
/** is the level a page's query boots one of the look's places? No `levelFile` = the bundled office (or the editor's document under `?level=local`: asked by its
 *  name where the stash can be read); `office.json` / `office-*.json` = the office family */
export function placeOfQuery(q: URLSearchParams): boolean {
  const lf = q.get('levelFile'); if (lf && /^[\w.-]+\.json$/.test(lf)) return !/^office(-[\w.-]+)?\.json$/.test(lf);   // (a name main.ts's boot will not fetch boots the bundled office)
  if (q.get('level') === 'local') { try { const n = JSON.parse((globalThis as any).localStorage?.getItem('twopointsix.editorLevel') ?? 'null')?.name; return typeof n === 'string' && n !== 'office' && !n.startsWith('office-'); } catch { return false; } }
  return false;
}

/** the levels that have a file in the look: today's file → the look's (same rooms, doors, marks and squads; tools/levels/<name>-gwot.ts builds it from the
 *  same generator) */
export const LOOK_LEVELS: Readonly<Record<string, string>> = {
  'compound.json': 'compound-gwot.json', 'barracks.json': 'barracks-gwot.json', 'warehouse.json': 'warehouse-gwot.json', 'depot.json': 'depot-gwot.json',
  'registry.json': 'registry-gwot.json', 'block.json': 'block-gwot.json', 'block-squads.json': 'block-squads-gwot.json', 'records.json': 'records-gwot.json',
  'consulate.json': 'consulate-gwot.json', 'defuse.json': 'defuse-gwot.json', 'floodlit.json': 'floodlit-gwot.json', 'mousehole.json': 'mousehole-gwot.json',
  'killhouse.json': 'killhouse-gwot.json', 'hall.json': 'hall-gwot.json',
};
/** DOOR LEAVES per level (picture only; scene/lookTidy.ts lookDoors): the paint of the outside leaves and, where a level has its own, of the inside ones — for
 *  the levels whose accent is not one of the three paints lookDoors counts off a level's boxes (the compound's blue, the barracks' green, the depot's oxide
 *  red). Without a row here every such level got the barracks' green doors. Keyed by the level's own file, colours as plain numbers (no imports here).
 *  `stuff` = WHAT THE LEAVES ARE MADE OF (Box.stuff): one word for every leaf, or one for the outside leaves and one for the inside ones. A row's colours are
 *  numbers and numbers say nothing, so a row that says no word has plain leaves. (Until 11-04 a leaf was whatever its colour was nearest: the records
 *  office's black lacquer was bare iron and the consulate's dark varnish a blanket, on the leaves whose own shade fell under their paint's and on no other.) */
export type LookDoorStuff = BoxStuff | { out?: BoxStuff; in?: BoxStuff };
export const LOOK_DOORS: Readonly<Record<string, { out: readonly [number, number, number]; in?: readonly [number, number, number]; stuff?: LookDoorStuff }>> = {
  'white-roof.json': { out: [0.88, 0.24, 0.03], in: [0.74, 0.74, 0.73], stuff: 'enamel' },   // the look study's roof: orange gloss outside, white inside, kept
  'block.json': { out: [0.22, 0.4, 0.1] }, 'block-squads.json': { out: [0.22, 0.4, 0.1] },   // the telephone exchange: pistachio gloss
  'warehouse.json': { out: [0.6, 0.42, 0.04], stuff: { out: 'paint' } },         // the customs shed: safety yellow
  'registry.json': { out: [0.1, 0.1, 0.105], stuff: { out: 'paint' } },          // the records office: black lacquer
  'records.json': { out: [0.2, 0.2, 0.22], in: [0.47, 0.36, 0.22], stuff: { in: 'wood' } },   // the land office: grey paint outside, pale poplar inside
  'consulate.json': { out: [0.21, 0.05, 0.105], in: [0.27, 0.18, 0.12], stuff: { in: 'wood' } },   // the consulate: plum outside, dark varnish inside
  'defuse.json': { out: [0.3, 0.14, 0.05] },                                     // the bomb maker's house: the firing wire's copper, darker
  'floodlit.json': { out: [0.62, 0.17, 0.03] },                                  // the airstrip's fuel post: aviation orange
  'mousehole.json': { out: [0.05, 0.2, 0.19] },                                 // the clinic: its queue line's teal, darker
  'killhouse.json': { out: [0.26, 0.12, 0.42], in: [0.55, 0.43, 0.27], stuff: { in: 'wood' } },   // the shoot house: its marking paint's violet, darker, outside; raw plywood inside
  'hall.json': { out: [0.16, 0.36, 0.47] },                                      // the scouts' barrack wing: its range card's pale sky blue, darker
};
/** the file to LOAD for a level the URL / a mission row names: the look's while the switch is on and the level has one, else the file itself */
export function lookLevelFile(file: string): string { return LOOK.on ? LOOK_LEVELS[file] ?? file : file; }
/** what a page's query says: on unless `?look=plain` (or `?look=0` / `?look=off` / `?nolook`) */
export function lookOfQuery(q: URLSearchParams): boolean { const v = q.get('look'); return !(q.has('nolook') || dressOff(v)); }
function dressOff(v: string | null): boolean { return v === 'plain' || v === '0' || v === 'off'; }
/** a value of `look` on an address is the DRESS's off word (the level a run plays), not a row of the look prototype's page: the two share the word */
export const lookDressWord = (v: string | null): boolean => dressOff(v);
/** …and of the sneaking suit: on unless `?suit=0` (or `?suit=off` / `?nosuit`) */
export function suitOfQuery(q: URLSearchParams): boolean { const v = q.get('suit'); return !(q.has('nosuit') || v === '0' || v === 'off'); }

// THE LOOK PROTOTYPE's page (`?look`, main.ts; not the dress above, though they share the word): what on an address is that page's. One reader, so that the page
// and a run of several pages that is to be ONE look from its first page to its last (ui/benchrun.ts) cannot come to read an address two ways.
/** its rows (`?look`, `?look=key:value`): every `look` value that is not one of the dress's three words */
export function lookPageRows(q: URLSearchParams): string[] { return q.getAll('look').filter(v => !dressOff(v)); }
/** the page is the look's: a row on the address, or the tag a one-file build of it carries (where the address takes no flags) */
export function lookPageOn(q: URLSearchParams): boolean { return lookPageRows(q).length > 0 || (typeof document !== 'undefined' && !!document.querySelector?.('meta[name="tps-look"]')); }
/** ITS TRIALS: the address's word of each, and the key under which the look panel's button keeps it on this machine */
export const LOOK_TRIALS = { geo: 'twopointsix.geotry', hills: 'twopointsix.hillrays', lorry: 'twopointsix.lorrytry' } as const;
export type LookTrial = keyof typeof LOOK_TRIALS;
/** a trial's switch: the address's own word wins; else what the panel's button last kept on this machine (on, where it kept nothing) */
export function lookTrialOn(q: URLSearchParams, word: LookTrial): boolean { if (q.has(word)) return q.get(word) !== '0'; try { return localStorage.getItem(LOOK_TRIALS[word]) !== '0'; } catch { return true; } }
/** the trials this machine keeps OFF and the address does not name, as the address would say them: with these on it, another page is the look this one shows */
export function lookTrialsKept(q: URLSearchParams): string[] { const off = (Object.keys(LOOK_TRIALS) as LookTrial[]).filter(w => !q.has(w) && !lookTrialOn(q, w)).map(w => `${w}=0`);
  let cut: string | null = null; try { cut = localStorage.getItem('twopointsix.cutboxes'); } catch { /* no store: on */ }   // (…and the cut boxes' own switch, which four of the five stand on: scene/cut.ts initCutSwitch)
  return cut === '0' && !q.has('cutboxes') && !q.has('nocutboxes') ? [...off, 'cutboxes=0'] : off; }
/** the other words the page reads once, at boot, into its shader text and its level (main.ts, scene/cut.ts: the look's geometry is cut boxes) */
const LOOK_PAGE_WORDS: ReadonlySet<string> = new Set([...Object.keys(LOOK_TRIALS), 'roll', 'lorrykit', 'seat', 'firefit', 'longfoot', 'walked', 'firesnow', 'firesoot', 'emberfog', 'cutboxes', 'nocutboxes', 'smokelook', 'nosunglint', 'glintpick', 'holedepth', 'sheen']);
/** a word of an address, decoded, is the look page's own */
export function lookPageWord(k: string, v: string): boolean { return k === 'look' ? !dressOff(v) : k === 'round' ? v === '1' : LOOK_PAGE_WORDS.has(k); }   // (`round=1` is the page's: its round test compiled beside a trial's cut boxes; `round=0` is a rule's off switch, another fight)
