// THE PAGE'S ONE BOOT CALL FOR THE KIT TABLE (game/kit.ts): read twopointsix.kit from this machine's store and put the table's hook on window. The page's tools
// drive the table the page's own start wears through `__kit`; the cards ask the desk (view/desk.ts).
import { KIT, KIT_STAGED, kitCost, kitLoad, kitLoadout, kitPreset, kitPresetAll, kitReset, kitSet, kitWalls, kitTeam, type KitPreset } from '../game/kit';

/** Called by main.ts before anything listens for keys. (`__kit` is up from the first tick, before `__ready`: it is the table's, not the game's, and wants no engine.
 *  It sits UNDER the fatal card: a boot that died still shows.) */
export function kitBoot(): void {
  let store: Storage | null = null; try { store = window.localStorage; } catch { /* a sandboxed frame: the table lives for the visit */ }
  kitLoad(store, location.search);
  (window as unknown as { __kit: unknown }).__kit = {
    KIT, staged: KIT_STAGED, reset: kitReset,
    set: (i: number, field: string, v: string | number | boolean) => kitSet(i, field as never, v),
    preset: (p: KitPreset, i?: number) => (i === undefined ? (kitPresetAll(p), { ok: true, why: '' }) : kitPreset(i, p)),
    cost: (i?: number) => (i === undefined ? KIT.members.map((_m, k) => kitCost(k)) : kitCost(i)),
    walls: (n = kitTeam()) => kitWalls(n),
    loadout: (id: string) => kitLoadout(id) ?? 'done',
  };
}
