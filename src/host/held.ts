// WHICH KNOB ROWS ARE HELD AT ONE VALUE. Import-free, so the page's hold (host/rustgame.ts rustSettledHold), the tools' fold (tools/qa/fold.ts) and whoever
// builds rows of a panel from data can all share it. The page holds every settled switch row at the value it plays: a read answers that value and a write of
// anything else is refused. A box or a slider over such a row could not be moved, so the builders ask `rowHeld` first and build none (ui/bindings.ts knobRows). The hold says what it holds; nothing is guessed from the row's shape (SLOTS.on and BIGSLOT.on answer through accessors of
// their own and are live switches).
const HELD = new WeakMap<object, Set<string>>();
/** the hold has laid this row: it answers one value from here on */
export function markHeld(table: object, key: string): void { let s = HELD.get(table); if (!s) HELD.set(table, s = new Set()); s.add(key); }
/** no row to build: the table has not got the key, or the hold has laid it */
export function rowHeld(table: object | null | undefined, key: string): boolean { return !table || !(key in table) || !!HELD.get(table)?.has(key); }
