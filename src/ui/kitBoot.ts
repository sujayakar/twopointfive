// THE PAGE'S ONE BOOT CALL FOR THE KIT TABLE (game/kit.ts): read twopointsix.kit from this machine's store and put the table's hook on window. The page's tools
// and checks drive the kit through `__kit`.
import { KIT, KIT_STAGED, SLOTS, SUITS, TRIMS, kitCost, kitLegal, kitLoad, kitLoadout, kitTeam, kitWalls, loadoutMissing, wallHere, kitPool, kitPreset, kitPresetAll, kitReset, kitSet, kitStep, kitTry, kitWords, slotsSet, slotsUnknown, squadCost, type KitPreset } from '../game/kit';

/** Called by main.ts before anything listens for keys. (`__kit` is up from the first tick, before `__ready`: it is the table's, not the game's, and wants no engine.
 *  It sits UNDER the fatal card: a boot that died still shows.) */
export function kitBoot(): void {
  let store: Storage | null = null; try { store = window.localStorage; } catch { /* a sandboxed frame: the table lives for the visit */ }
  kitLoad(store, location.search);
  (window as unknown as { __kit: unknown }).__kit = {
    KIT, suits: SUITS, trims: TRIMS, staged: KIT_STAGED, reset: kitReset,
    set: (i: number, field: string, v: string | number | boolean) => kitSet(i, field as never, v),
    step: (i: number, field: string, dir: 1 | -1 = 1) => kitStep(i, field as never, dir),
    preset: (p: KitPreset, i?: number) => (i === undefined ? (kitPresetAll(p), { ok: true, why: '' }) : kitPreset(i, p)),
    words: () => KIT.members.map((_m, i) => kitWords(i)), pool: kitPool,
    // the pouches rule (kit.ts SLOTS; slots.json): each man's used / total and the squad's, a step that says why it was refused, the table swapped by hand, one of its named loadouts put on all five
    slots: SLOTS, cost: (i?: number) => (i === undefined ? KIT.members.map((_m, k) => kitCost(k)) : kitCost(i)), unknown: slotsUnknown,
    try: (i: number, field: string, dir: 1 | -1 = 1) => kitTry(i, field as never, dir),
    table: (t: unknown) => slotsSet(t) ?? 'taken',
    squad: () => squadCost(), legal: () => kitLegal(), team: (n?: number) => kitTeam(n), loadouts: () => SLOTS.table.loadouts.map(l => ({ id: l.id, name: l.name, missing: loadoutMissing(l) })), walls: (n = kitTeam()) => kitWalls(n), wallHere: () => wallHere(),
    loadout: (id: string) => kitLoadout(id) ?? 'done',
  };
}
